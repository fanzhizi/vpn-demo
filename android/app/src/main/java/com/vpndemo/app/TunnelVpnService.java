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
 * Android VpnService implementation.
 * Creates a TUN device, reads IP packets, feeds them into Rust tunnel-core.
 */
public class TunnelVpnService extends VpnService {
    private static final String TAG = "TunnelVpnService";

    private ParcelFileDescriptor tunFd;
    private Thread readThread;
    private Thread writeThread;
    private volatile boolean running = false;

    private static String socks5Address = "192.168.31.209:1080";
    private static TunnelVpnService instance;

    public static void setSocks5Address(String addr) {
        socks5Address = addr;
    }

    /** Called from Rust (via JNI call_method on this instance) to protect a socket fd */
    public boolean protectSocket(int fd) {
        return protect(fd);
    }

    @Override
    public int onStartCommand(Intent intent, int flags, int startId) {
        if (intent != null && "STOP".equals(intent.getAction())) {
            stopVpn();
            return START_NOT_STICKY;
        }
        startVpn();
        return START_STICKY;
    }

    private void startVpn() {
        instance = this;
        Log.i(TAG, "Starting VPN, SOCKS5: " + socks5Address);

        // Register this VpnService instance for socket protection (like leaf)
        TunnelCore.setProtectSocketCallback(this, "protectSocket");
        Log.i(TAG, "Protect callback registered");

        // Start Rust tunnel core
        boolean started = TunnelCore.tunnelStart(socks5Address);
        if (!started) {
            Log.e(TAG, "Failed to start Rust tunnel");
            stopSelf();
            return;
        }
        Log.i(TAG, "Rust tunnel started");

        // Configure TUN device
        Builder builder = new Builder();
        builder.setSession("VPN Demo")
               .addAddress("10.0.0.2", 24)
               .addDnsServer("8.8.8.8")
               .addDnsServer("8.8.4.4")
               .setMtu(1500)
               .setBlocking(true);

        // Route all traffic through TUN. SOCKS5 sockets are protected via
        // VpnService.protect(fd) called from Rust through JNI, bypassing the tunnel.
        builder.addRoute("0.0.0.0", 0);

        tunFd = builder.establish();
        if (tunFd == null) {
            Log.e(TAG, "Failed to establish TUN");
            TunnelCore.tunnelStop();
            stopSelf();
            return;
        }

        Log.i(TAG, "TUN device established, fd=" + tunFd.getFd());
        running = true;

        // Thread: Read from TUN -> feed to Rust
        readThread = new Thread(() -> {
            FileInputStream in = new FileInputStream(tunFd.getFileDescriptor());
            byte[] buf = new byte[65535];
            long count = 0;
            while (running) {
                try {
                    int n = in.read(buf);
                    if (n > 0) {
                        count++;
                        if (count <= 5 || count % 500 == 0) {
                            Log.d(TAG, "TUN read #" + count + ": " + n + " bytes");
                        }
                        byte[] packet = new byte[n];
                        System.arraycopy(buf, 0, packet, 0, n);
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

        // Thread: Read from Rust -> write to TUN
        writeThread = new Thread(() -> {
            FileOutputStream out = new FileOutputStream(tunFd.getFileDescriptor());
            byte[] buf = new byte[65535];
            long count = 0;
            while (running) {
                int n = TunnelCore.tunnelReadPacket(buf, buf.length);
                if (n > 0) {
                    count++;
                    if (count <= 5 || count % 500 == 0) {
                        Log.d(TAG, "TUN write #" + count + ": " + n + " bytes");
                    }
                    try {
                        out.write(buf, 0, n);
                    } catch (IOException e) {
                        if (running) Log.e(TAG, "TUN write error", e);
                        break;
                    }
                } else {
                    try { Thread.sleep(1); } catch (InterruptedException e) { break; }
                }
            }
            Log.i(TAG, "TUN write thread ended, packets=" + count);
        }, "tun-write");
        writeThread.start();

        Log.i(TAG, "VPN started successfully");
    }

    private void stopVpn() {
        Log.i(TAG, "Stopping VPN");
        running = false;
        instance = null;

        TunnelCore.tunnelStop();

        if (readThread != null) readThread.interrupt();
        if (writeThread != null) writeThread.interrupt();

        if (tunFd != null) {
            try { tunFd.close(); } catch (IOException e) { Log.e(TAG, "Error closing TUN", e); }
            tunFd = null;
        }

        stopSelf();
        Log.i(TAG, "VPN stopped");
    }

    @Override
    public void onDestroy() {
        stopVpn();
        super.onDestroy();
    }
}
