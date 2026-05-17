package com.vpndemo.app;

import android.app.Activity;
import android.content.Intent;
import android.net.VpnService;
import android.os.Bundle;
import android.util.Log;
import android.view.View;
import android.widget.Button;
import android.widget.EditText;
import android.widget.TextView;

/**
 * VPN Demo 主界面 Activity。
 *
 * 架构角色：
 * - 提供用户交互界面：输入 SOCKS5 地址、连接/断开 VPN、测试 SOCKS5 连通性
 * - 管理 VPN 权限请求流程（VpnService.prepare() → 系统弹窗 → onActivityResult）
 * - 通过 Intent 启动/停止 TunnelVpnService
 *
 * 注意：VPN 实际工作在 TunnelVpnService 中，MainActivity 只负责 UI 和权限管理。
 * 即使 MainActivity 被销毁（按返回键），VPN Service 仍然在后台运行。
 */
public class MainActivity extends Activity {
    private static final String TAG = "VPNDemo";

    // VPN 权限请求的 requestCode，用于在 onActivityResult 中识别回调
    private static final int VPN_REQUEST_CODE = 1;

    private EditText addressInput;   // SOCKS5 地址输入框
    private Button connectButton;    // 连接/断开按钮
    private Button testButton;       // 测试 SOCKS5 连通性按钮
    private TextView statusText;     // 连接状态显示（绿色/红色圆点 + 文字）
    private TextView logText;        // 操作日志区域（保留最近 6 行）
    private boolean connected = false; // 当前 VPN 连接状态

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        // 绑定 UI 控件
        addressInput = findViewById(R.id.address_input);
        connectButton = findViewById(R.id.connect_button);
        testButton = findViewById(R.id.test_button);
        statusText = findViewById(R.id.status_text);
        logText = findViewById(R.id.log_text);

        // 注册点击事件
        connectButton.setOnClickListener(v -> toggleVpn());
        testButton.setOnClickListener(v -> testSocks5());
    }

    /**
     * 追加日志到界面日志区域，同时输出到 logcat。
     * 保留最近 6 行，防止日志区域无限增长。
     *
     * @param msg 日志消息
     */
    private void appendLog(String msg) {
        Log.i(TAG, msg);
        runOnUiThread(() -> {
            String current = logText.getText().toString();
            String[] lines = current.split("\n");
            StringBuilder sb = new StringBuilder();
            // 只保留最近 5 行 + 新增 1 行 = 最多 6 行
            int start = Math.max(0, lines.length - 5);
            for (int i = start; i < lines.length; i++) {
                if (!lines[i].isEmpty()) {
                    sb.append(lines[i]).append("\n");
                }
            }
            sb.append(msg);
            logText.setText(sb.toString());
        });
    }

    /**
     * 切换 VPN 连接状态。
     * 已连接 → 发送 STOP Intent 停止 Service
     * 未连接 → 检查权限 → 启动 Service
     */
    private void toggleVpn() {
        if (connected) {
            // 断开 VPN：发送带 "STOP" action 的 Intent
            appendLog("Stopping VPN...");
            Intent intent = new Intent(this, TunnelVpnService.class);
            intent.setAction("STOP");
            startService(intent);
            connected = false;
            runOnUiThread(() -> {
                statusText.setText("● Disconnected");
                statusText.setTextColor(0xFFF44336); // Material Red
                connectButton.setText("Connect VPN");
                addressInput.setEnabled(true);
            });
        } else {
            // 连接 VPN
            String address = addressInput.getText().toString().trim();
            if (address.isEmpty()) {
                appendLog("Please enter SOCKS5 address");
                return;
            }

            appendLog("Requesting VPN permission...");
            // 在启动 Service 前设置 SOCKS5 地址（通过静态方法传递配置）
            TunnelVpnService.setSocks5Address(address);

            // VpnService.prepare() 检查是否已授权 VPN 权限
            // 返回 null 表示已授权，返回 Intent 表示需要弹窗请求用户确认
            // 这是 Android VPN 的安全机制：同一时间只能有一个 VPN 活跃
            Intent vpnIntent = VpnService.prepare(this);
            if (vpnIntent != null) {
                // 需要用户确认，弹出系统 VPN 授权对话框
                startActivityForResult(vpnIntent, VPN_REQUEST_CODE);
            } else {
                // 已有权限，直接启动
                startVpnService();
            }
        }
    }

    /**
     * VPN 权限请求的回调。
     * 用户在系统 VPN 授权对话框中点击"确定"或"取消"后触发。
     */
    @Override
    protected void onActivityResult(int requestCode, int resultCode, Intent data) {
        if (requestCode == VPN_REQUEST_CODE) {
            if (resultCode == RESULT_OK) {
                // 用户授权成功，启动 VPN Service
                startVpnService();
            } else {
                // 用户拒绝了 VPN 权限
                appendLog("VPN permission denied");
            }
        }
        super.onActivityResult(requestCode, resultCode, data);
    }

    /**
     * 启动 VPN Service 并更新 UI 状态。
     * 通过 startService(Intent) 以 Started Service 模式启动 TunnelVpnService。
     */
    private void startVpnService() {
        appendLog("Starting VPN service...");
        Intent intent = new Intent(this, TunnelVpnService.class);
        startService(intent); // 触发 TunnelVpnService.onStartCommand()
        connected = true;
        runOnUiThread(() -> {
            statusText.setText("● Connected");
            statusText.setTextColor(0xFF4CAF50); // Material Green
            connectButton.setText("Disconnect");
            addressInput.setEnabled(false); // 连接期间禁止修改地址
        });
        appendLog("VPN started");
    }

    /**
     * 测试 SOCKS5 代理连通性。
     * 在后台线程中直接连接 SOCKS5 代理（不经过 TUN），验证代理是否可用。
     * 这对于调试非常有用：如果测试失败，说明代理本身有问题，而非 VPN 隧道的问题。
     */
    private void testSocks5() {
        String address = addressInput.getText().toString().trim();
        if (address.isEmpty()) {
            appendLog("Please enter SOCKS5 address");
            return;
        }
        appendLog("Testing SOCKS5: " + address + "...");

        // 在新线程中执行网络操作，避免阻塞 UI 线程（Android 主线程不允许网络操作）
        new Thread(() -> {
            byte[] outBuf = new byte[2048];
            // 调用 Rust 侧的 tunnel_test_socks5，会尝试通过 SOCKS5 访问 httpbin.org
            int result = TunnelCore.tunnelTestSocks5(address, outBuf, outBuf.length);
            // 查找 C 风格字符串的 null 结尾位置
            int len = 0;
            for (int i = 0; i < outBuf.length; i++) {
                if (outBuf[i] == 0) break;
                len = i + 1;
            }
            String msg = new String(outBuf, 0, len);
            appendLog("Test result(" + result + "): " + msg);
        }).start();
    }
}
