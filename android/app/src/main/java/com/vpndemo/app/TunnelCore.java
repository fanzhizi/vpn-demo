package com.vpndemo.app;

/**
 * JNI bridge to Rust tunnel-core library.
 * These native methods map to the C FFI functions exported by Rust.
 */
public class TunnelCore {
    static {
        System.loadLibrary("tunnel_core");
    }

    public static native boolean tunnelStart(String socks5Addr);
    public static native void tunnelStop();
    public static native boolean tunnelFeedPacket(byte[] data, int len);
    public static native int tunnelReadPacket(byte[] buf, int bufLen);
    public static native boolean tunnelIsRunning();
    public static native int tunnelTestSocks5(String socks5Addr, byte[] outBuf, int outLen);

    /** Set the VpnService instance for socket protection callback */
    public static native void setProtectSocketCallback(Object vpnService, String methodName);
}
