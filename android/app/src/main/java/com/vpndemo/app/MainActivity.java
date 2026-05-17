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

public class MainActivity extends Activity {
    private static final String TAG = "VPNDemo";
    private static final int VPN_REQUEST_CODE = 1;

    private EditText addressInput;
    private Button connectButton;
    private Button testButton;
    private TextView statusText;
    private TextView logText;
    private boolean connected = false;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        addressInput = findViewById(R.id.address_input);
        connectButton = findViewById(R.id.connect_button);
        testButton = findViewById(R.id.test_button);
        statusText = findViewById(R.id.status_text);
        logText = findViewById(R.id.log_text);

        connectButton.setOnClickListener(v -> toggleVpn());
        testButton.setOnClickListener(v -> testSocks5());
    }

    private void appendLog(String msg) {
        Log.i(TAG, msg);
        runOnUiThread(() -> {
            String current = logText.getText().toString();
            String[] lines = current.split("\n");
            StringBuilder sb = new StringBuilder();
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

    private void toggleVpn() {
        if (connected) {
            appendLog("Stopping VPN...");
            Intent intent = new Intent(this, TunnelVpnService.class);
            intent.setAction("STOP");
            startService(intent);
            connected = false;
            runOnUiThread(() -> {
                statusText.setText("● Disconnected");
                statusText.setTextColor(0xFFF44336);
                connectButton.setText("Connect VPN");
                addressInput.setEnabled(true);
            });
        } else {
            String address = addressInput.getText().toString().trim();
            if (address.isEmpty()) {
                appendLog("Please enter SOCKS5 address");
                return;
            }

            appendLog("Requesting VPN permission...");
            TunnelVpnService.setSocks5Address(address);

            Intent vpnIntent = VpnService.prepare(this);
            if (vpnIntent != null) {
                startActivityForResult(vpnIntent, VPN_REQUEST_CODE);
            } else {
                startVpnService();
            }
        }
    }

    @Override
    protected void onActivityResult(int requestCode, int resultCode, Intent data) {
        if (requestCode == VPN_REQUEST_CODE) {
            if (resultCode == RESULT_OK) {
                startVpnService();
            } else {
                appendLog("VPN permission denied");
            }
        }
        super.onActivityResult(requestCode, resultCode, data);
    }

    private void startVpnService() {
        appendLog("Starting VPN service...");
        Intent intent = new Intent(this, TunnelVpnService.class);
        startService(intent);
        connected = true;
        runOnUiThread(() -> {
            statusText.setText("● Connected");
            statusText.setTextColor(0xFF4CAF50);
            connectButton.setText("Disconnect");
            addressInput.setEnabled(false);
        });
        appendLog("VPN started");
    }

    private void testSocks5() {
        String address = addressInput.getText().toString().trim();
        if (address.isEmpty()) {
            appendLog("Please enter SOCKS5 address");
            return;
        }
        appendLog("Testing SOCKS5: " + address + "...");

        new Thread(() -> {
            byte[] outBuf = new byte[2048];
            int result = TunnelCore.tunnelTestSocks5(address, outBuf, outBuf.length);
            // Find null terminator
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
