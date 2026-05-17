package com.vpndemo.app;

/**
 * JNI 桥接类 - 连接 Java 层与 Rust tunnel-core 动态库。
 *
 * 架构角色：
 * - 声明所有 native 方法，对应 Rust 侧 jni_ffi.rs 中的 extern "system" fn
 * - 命名规则：Java_com_vpndemo_app_TunnelCore_方法名（JNI 标准命名约定）
 * - 类加载时自动加载 libtunnel_core.so 动态库
 *
 * 数据流：
 * Java 调用 → JNI → Rust jni_ffi.rs → Rust ffi.rs → tunnel-core 核心逻辑
 */
public class TunnelCore {

    // static 初始化块：类首次被引用时执行
    // 加载 libtunnel_core.so（系统会自动在 jniLibs/arm64-v8a/ 目录搜索）
    // 加载成功后触发 Rust 侧的 JNI_OnLoad 函数，缓存 JVM 引用
    static {
        System.loadLibrary("tunnel_core");
    }

    /**
     * 启动隧道核心。
     * Rust 侧会创建 tokio runtime、初始化 netstack-smoltcp 协议栈、启动 TCP/UDP 转发任务。
     *
     * @param socks5Addr SOCKS5 代理地址，格式 "ip:port"
     * @return true 启动成功，false 失败（地址无效、runtime 创建失败等）
     */
    public static native boolean tunnelStart(String socks5Addr);

    /**
     * 停止隧道核心。
     * 设置 running = false（AtomicBool），所有异步任务检测到后自行退出。
     */
    public static native void tunnelStop();

    /**
     * 将从 TUN 设备读取的 IP 数据包送入 Rust 的 netstack。
     * 内部通过 mpsc::channel.try_send() 发送，channel 满时会丢包返回 false。
     *
     * @param data IP 数据包的原始字节
     * @param len  有效数据长度
     * @return true 成功送入 channel，false channel 已满或隧道未运行
     */
    public static native boolean tunnelFeedPacket(byte[] data, int len);

    /**
     * 从 Rust 的 netstack 读取一个返回的 IP 数据包。
     * 内部通过 mpsc::channel.try_recv() 非阻塞获取。
     *
     * @param buf    接收缓冲区
     * @param bufLen 缓冲区大小
     * @return 实际读取的字节数，0 表示暂无数据
     */
    public static native int tunnelReadPacket(byte[] buf, int bufLen);

    /**
     * 查询隧道是否正在运行。
     *
     * @return true 运行中，false 已停止
     */
    public static native boolean tunnelIsRunning();

    /**
     * 测试 SOCKS5 代理连通性（不经过 TUN，直接连接）。
     * 会尝试通过 SOCKS5 访问 httpbin.org/ip 验证端到端连通。
     *
     * @param socks5Addr SOCKS5 代理地址
     * @param outBuf     输出缓冲区，存放测试结果字符串（C 风格 null 结尾）
     * @param outLen     缓冲区大小
     * @return 1 成功，0 失败
     */
    public static native int tunnelTestSocks5(String socks5Addr, byte[] outBuf, int outLen);

    /**
     * 注册 socket 保护回调。
     * Rust 侧用 GlobalRef 持有 vpnService 对象（防止 GC 回收），
     * 在需要 protect 时通过 JNI 反射调用 methodName 指定的方法。
     *
     * @param vpnService VpnService 实例（TunnelVpnService.this）
     * @param methodName 要回调的方法名（"protectSocket"）
     */
    public static native void setProtectSocketCallback(Object vpnService, String methodName);
}
