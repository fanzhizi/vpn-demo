package com.vpndemo.app;

import android.content.Intent;
import android.net.VpnService;
import android.os.ParcelFileDescriptor;
import android.util.Log;

import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.io.IOException;
import java.net.InetSocketAddress;
import java.net.Socket;

/**
 * Android VPN 服务核心实现类。
 *
 * 在整体架构中的角色：
 * - 继承 Android 系统的 VpnService，负责创建和管理 TUN 虚拟网卡设备
 * - 作为 Java 层与 Rust 层之间的数据桥梁：从 TUN 读取 IP 数据包送入 Rust，
 *   同时从 Rust 读取处理后的数据包写回 TUN
 * - 提供 protectSocket() 方法给 Rust 层通过 JNI 回调，使 SOCKS5 连接绕过 VPN 路由
 *
 * 工作流程：
 * 1. MainActivity 请求 VPN 权限后启动此 Service
 * 2. 注册 socket 保护回调（防止 SOCKS5 流量被 VPN 捕获形成死循环）
 * 3. 启动 Rust tunnel-core（内含 netstack + SOCKS5 代理逻辑）
 * 4. 通过 Builder 配置并建立 TUN 设备
 * 5. 启动双向数据转发线程：TUN → Rust、Rust → TUN
 */
public class TunnelVpnService extends VpnService {
    private static final String TAG = "TunnelVpnService";

    // TUN 设备文件描述符，通过 Builder.establish() 获得
    // ParcelFileDescriptor 封装了底层的 Linux fd，支持跨进程传递
    private ParcelFileDescriptor tunFd;

    // 读线程：从 TUN 设备读取 IP 数据包，送入 Rust 的 netstack 进行协议解析
    private Thread readThread;

    // 写线程：从 Rust 的 netstack 读取处理后的 IP 数据包，写回 TUN 设备送往应用
    private Thread writeThread;

    // volatile 保证多线程可见性，用于优雅停止读写线程
    private volatile boolean running = false;

    // SOCKS5 代理服务器地址，由 MainActivity 在启动前设置
    private static String socks5Address = "192.168.31.209:1080";

    // 静态实例引用，供 Rust JNI 回调 protectSocket 时使用
    private static TunnelVpnService instance;

    /**
     * 设置 SOCKS5 代理地址。
     * 在启动 VPN Service 之前由 MainActivity 调用。
     *
     * @param addr 格式为 "ip:port"，例如 "192.168.31.209:1080"
     */
    public static void setSocks5Address(String addr) {
        socks5Address = addr;
    }

    /**
     * Socket 保护方法 - 被 Rust 通过 JNI 回调调用。
     *
     * 为什么需要这个方法：
     * - VPN 配置了 addRoute("0.0.0.0", 0)，即所有流量都走 TUN
     * - 但 SOCKS5 代理本身的 TCP/UDP socket 不能走 TUN，否则会形成死循环：
     *   SOCKS5 连接 → TUN → netstack → SOCKS5 连接 → TUN → ... (无限循环)
     * - protect(fd) 调用 Linux 内核的 SO_MARK，标记该 socket 绕过 VPN 路由规则
     *
     * 调用链：Rust connect() 前 → JNI call_method → protectSocket(fd) → VpnService.protect(fd)
     *
     * @param fd 需要保护的 socket 文件描述符
     * @return true 表示保护成功，该 socket 的流量将绕过 VPN
     */
    public boolean protectSocket(int fd) {
        return protect(fd);
    }

    /**
     * Service 启动入口，由系统在 startService() 时调用。
     *
     * @param intent 包含 action 信息："STOP" 表示停止，null/其他表示启动
     * @param flags 系统传入的启动标志
     * @param startId 本次启动的唯一 ID
     * @return START_STICKY 表示系统杀死后自动重启，START_NOT_STICKY 表示不重启
     */
    @Override
    public int onStartCommand(Intent intent, int flags, int startId) {
        if (intent != null && "STOP".equals(intent.getAction())) {
            stopVpn();
            return START_NOT_STICKY;
        }
        startVpn();
        return START_STICKY;
    }

    /**
     * VPN 启动核心逻辑。
     * 执行顺序严格不可调换（特别是 protect 回调必须在 tunnel_start 之前注册）。
     */
    private void startVpn() {
        instance = this;
        Log.i(TAG, "Starting VPN, SOCKS5: " + socks5Address);

        // 【第一步】注册 socket 保护回调
        // 必须在 tunnelStart 之前完成，否则 Rust 建立 SOCKS5 连接时无法 protect
        // 传入 this (VpnService 实例) 和方法名 "protectSocket"
        // Rust 侧会用 GlobalRef 持有该对象防止 GC，用方法名反射调用
        TunnelCore.setProtectSocketCallback(this, "protectSocket");
        Log.i(TAG, "Protect callback registered");

        // 【第二步】启动 Rust tunnel-core
        // 内部会创建 tokio runtime、初始化 netstack-smoltcp 协议栈
        // 返回 false 表示启动失败（通常是地址解析错误或 runtime 创建失败）
        boolean started = TunnelCore.tunnelStart(socks5Address);
        if (!started) {
            Log.e(TAG, "Failed to start Rust tunnel");
            stopSelf();
            return;
        }
        Log.i(TAG, "Rust tunnel started");

        // 【第三步】配置并建立 TUN 虚拟网卡设备
        // Builder 是 VpnService 提供的内部类，用于声明式配置 TUN 设备参数
        Builder builder = new Builder();
        builder.setSession("VPN Demo")          // 系统 VPN 设置界面显示的会话名
               .addAddress("10.0.0.2", 24)      // TUN 设备的 IP 地址和子网掩码（/24 = 255.255.255.0）
               .addDnsServer("8.8.8.8")         // DNS 服务器，所有 DNS 查询都会走 TUN
               .addDnsServer("8.8.4.4")         // 备用 DNS 服务器
               .setMtu(1500)                    // 最大传输单元，标准以太网 MTU
               .setBlocking(true);              // 阻塞模式：read() 会阻塞直到有数据，适合线程模型

        // 路由所有流量到 TUN 设备（0.0.0.0/0 = 全部 IPv4 流量）
        // 被 protect() 标记的 socket 不受此路由规则影响
        builder.addRoute("0.0.0.0", 0);

        // establish() 实际创建 TUN 设备，返回文件描述符
        // 如果用户拒绝了 VPN 权限，这里会返回 null
        tunFd = builder.establish();
        if (tunFd == null) {
            Log.e(TAG, "Failed to establish TUN");
            TunnelCore.tunnelStop();
            stopSelf();
            return;
        }

        Log.i(TAG, "TUN device established, fd=" + tunFd.getFd());
        running = true;

        // 【第四步】启动 TUN 读线程：TUN → Rust
        // 从 TUN 设备读取应用发出的 IP 数据包，送入 Rust 的 netstack 协议栈解析
        // netstack 会解析 TCP/UDP 头部，将 TCP 连接通过 SOCKS5 代理转发
        readThread = new Thread(() -> {
            // FileInputStream 包装 TUN 设备的 fd，每次 read() 返回一个完整的 IP 数据包
            FileInputStream in = new FileInputStream(tunFd.getFileDescriptor());
            byte[] buf = new byte[65535]; // 最大 IP 包大小（理论上 MTU 1500 就够了，但保险起见用 64K）
            long count = 0;
            while (running) {
                try {
                    // 阻塞读取一个 IP 数据包（因为 setBlocking(true)）
                    int n = in.read(buf);
                    if (n > 0) {
                        count++;
                        // 前 5 个包和每 500 个包打印一次日志，避免日志刷屏
                        if (count <= 5 || count % 500 == 0) {
                            Log.d(TAG, "TUN read #" + count + ": " + n + " bytes");
                        }
                        // 拷贝有效数据（buf 是复用的大缓冲区，需截取实际长度）
                        byte[] packet = new byte[n];
                        System.arraycopy(buf, 0, packet, 0, n);
                        // 送入 Rust 的 mpsc channel，由 netstack 消费
                        TunnelCore.tunnelFeedPacket(packet, n);
                    }
                } catch (IOException e) {
                    if (running) Log.e(TAG, "TUN read error", e);
                    break;
                }
            }
            Log.i(TAG, "TUN read thread ended, packets=" + count);
        }, "tun-read");
        readThread.start();

        // 【第五步】启动 TUN 写线程：Rust → TUN
        // 从 Rust 的 netstack 读取返回的 IP 数据包（如 HTTP 响应），写入 TUN 送给应用
        writeThread = new Thread(() -> {
            // FileOutputStream 包装 TUN 的 fd，每次 write() 注入一个 IP 包到系统网络栈
            FileOutputStream out = new FileOutputStream(tunFd.getFileDescriptor());
            byte[] buf = new byte[65535];
            long count = 0;
            while (running) {
                // 非阻塞地从 Rust 的 outbound channel 尝试取一个包
                // 返回 0 表示暂无数据
                int n = TunnelCore.tunnelReadPacket(buf, buf.length);
                if (n > 0) {
                    count++;
                    if (count <= 5 || count % 500 == 0) {
                        Log.d(TAG, "TUN write #" + count + ": " + n + " bytes");
                    }
                    try {
                        // 将 IP 包写入 TUN，系统会将其交付给对应的应用 socket
                        out.write(buf, 0, n);
                    } catch (IOException e) {
                        if (running) Log.e(TAG, "TUN write error", e);
                        break;
                    }
                } else {
                    // 无数据时短暂休眠，避免空转浪费 CPU
                    // 这是轮询模型的妥协，因为 JNI 不方便使用 async/回调
                    try { Thread.sleep(1); } catch (InterruptedException e) { break; }
                }
            }
            Log.i(TAG, "TUN write thread ended, packets=" + count);
        }, "tun-write");
        writeThread.start();

        Log.i(TAG, "VPN started successfully");
    }

    /**
     * 停止 VPN，清理所有资源。
     * 停止顺序：标记停止 → 通知 Rust 停止 → 中断线程 → 关闭 TUN fd → 停止 Service
     */
    private void stopVpn() {
        Log.i(TAG, "Stopping VPN");
        running = false;        // 让读写线程退出循环
        instance = null;

        TunnelCore.tunnelStop(); // 通知 Rust 侧停止（设置 AtomicBool = false）

        // 中断读写线程（read 阻塞时需要 interrupt 来唤醒）
        if (readThread != null) readThread.interrupt();
        if (writeThread != null) writeThread.interrupt();

        // 关闭 TUN 设备文件描述符
        // 关闭后阻塞在 read() 的线程会收到 IOException 并退出
        if (tunFd != null) {
            try { tunFd.close(); } catch (IOException e) { Log.e(TAG, "Error closing TUN", e); }
            tunFd = null;
        }

        stopSelf(); // 停止 Service 自身
        Log.i(TAG, "VPN stopped");
    }

    /**
     * Service 被销毁时确保清理资源。
     * 可能由系统内存不足时触发，也可能由 stopSelf() 后触发。
     */
    @Override
    public void onDestroy() {
        stopVpn();
        super.onDestroy();
    }
}
