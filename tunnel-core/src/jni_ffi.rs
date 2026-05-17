//! JNI FFI 模块 - Android 平台的 Java/Kotlin 调用入口。
//!
//! 本模块将 Rust 的 C FFI 函数包装为 JNI 兼容的函数签名，
//! 使得 Java 层的 TunnelCore 类可以通过 native method 直接调用。
//!
//! 命名规则：Java_包名_类名_方法名（下划线分隔）
//! 例如：Java_com_vpndemo_app_TunnelCore_tunnelStart
//!
//! JNI 函数签名规则：
//! - 第一个参数：JNIEnv（JNI 环境指针，用于 Java/Rust 类型转换）
//! - 第二个参数：JClass（调用者的 Class 对象，静态方法时使用）
//! - 后续参数：对应 Java native method 声明的参数
//! - 调用约定：extern "system"（Windows 为 stdcall，其他平台为 C 调用约定）

#[cfg(target_os = "android")]
pub mod android {
    use std::ffi::CString;
    use jni::JNIEnv;
    use jni::objects::{JByteArray, JClass, JString};
    use jni::sys::{jboolean, jint, JNI_FALSE, JNI_TRUE};

    use jni::objects::JObject;
    use crate::ffi;

    /// 对应 Java: TunnelCore.tunnelStart(String socks5Addr)
    /// 将 Java String 转为 Rust CString，然后调用 C FFI 的 tunnel_start
    #[no_mangle]
    pub extern "system" fn Java_com_vpndemo_app_TunnelCore_tunnelStart(
        mut env: JNIEnv,
        _class: JClass,
        socks5_addr: JString,
    ) -> jboolean {
        // JString → Rust String：JNI 字符串是 Modified UTF-8 编码，需要转换
        let addr: String = match env.get_string(&socks5_addr) {
            Ok(s) => s.into(),
            Err(_) => return JNI_FALSE,
        };
        // Rust String → CString：C FFI 需要 null 结尾的字符串
        // 如果字符串中包含 \0 会失败（CString 不允许内部 null）
        let c_addr = match CString::new(addr) {
            Ok(c) => c,
            Err(_) => return JNI_FALSE,
        };
        // 调用 ffi.rs 中的 tunnel_start（C 导出函数）
        let result = unsafe { ffi::tunnel_start(c_addr.as_ptr()) };
        if result { JNI_TRUE } else { JNI_FALSE }
    }

    /// 对应 Java: TunnelCore.tunnelStop()
    /// 直接调用 C FFI，无参数无返回值
    #[no_mangle]
    pub extern "system" fn Java_com_vpndemo_app_TunnelCore_tunnelStop(
        _env: JNIEnv,
        _class: JClass,
    ) {
        ffi::tunnel_stop();
    }

    /// 对应 Java: TunnelCore.tunnelFeedPacket(byte[] data, int len)
    /// 将 Java byte[] 转为 Rust Vec<u8>，送入隧道的 inbound channel
    #[no_mangle]
    pub extern "system" fn Java_com_vpndemo_app_TunnelCore_tunnelFeedPacket(
        env: JNIEnv,
        _class: JClass,
        data: JByteArray,
        len: jint,
    ) -> jboolean {
        // JByteArray → Vec<u8>：将 Java 堆上的字节数组复制到 Rust 堆
        // 这里会发生一次内存拷贝（Java GC 堆 → Rust 堆）
        let bytes = match env.convert_byte_array(&data) {
            Ok(b) => b,
            Err(_) => return JNI_FALSE,
        };
        let result = unsafe {
            ffi::tunnel_feed_packet(bytes.as_ptr(), len as usize)
        };
        if result { JNI_TRUE } else { JNI_FALSE }
    }

    /// 对应 Java: TunnelCore.tunnelReadPacket(byte[] buf, int bufLen)
    /// 从隧道的 outbound channel 取一个包，写入 Java byte[] 缓冲区
    #[no_mangle]
    pub extern "system" fn Java_com_vpndemo_app_TunnelCore_tunnelReadPacket(
        mut env: JNIEnv,
        _class: JClass,
        buf: JByteArray,
        buf_len: jint,
    ) -> jint {
        // 先在 Rust 侧分配临时缓冲区接收数据
        let mut native_buf = vec![0u8; buf_len as usize];
        let n = unsafe {
            ffi::tunnel_read_packet(native_buf.as_mut_ptr(), buf_len as usize)
        };
        if n > 0 {
            // 将 Rust 缓冲区数据写入 Java byte[]
            // set_byte_array_region 直接操作 Java 堆内存，比创建新数组更高效
            // 需要 unsafe 将 u8 指针转为 i8 指针（Java byte 是 signed）
            let _ = env.set_byte_array_region(&buf, 0, unsafe {
                std::slice::from_raw_parts(native_buf.as_ptr() as *const i8, n)
            });
        }
        n as jint
    }

    /// 对应 Java: TunnelCore.tunnelIsRunning()
    /// 查询隧道运行状态
    #[no_mangle]
    pub extern "system" fn Java_com_vpndemo_app_TunnelCore_tunnelIsRunning(
        _env: JNIEnv,
        _class: JClass,
    ) -> jboolean {
        if ffi::tunnel_is_running() { JNI_TRUE } else { JNI_FALSE }
    }

    /// 对应 Java: TunnelCore.tunnelTestSocks5(String socks5Addr, byte[] outBuf, int outLen)
    /// 测试 SOCKS5 代理连通性，将结果写入 outBuf
    #[no_mangle]
    pub extern "system" fn Java_com_vpndemo_app_TunnelCore_tunnelTestSocks5(
        mut env: JNIEnv,
        _class: JClass,
        socks5_addr: JString,
        out_buf: JByteArray,
        out_len: jint,
    ) -> jint {
        let addr: String = match env.get_string(&socks5_addr) {
            Ok(s) => s.into(),
            Err(_) => return 0,
        };
        let c_addr = match CString::new(addr) {
            Ok(c) => c,
            Err(_) => return 0,
        };
        let mut native_buf = vec![0u8; out_len as usize];
        let result = unsafe {
            ffi::tunnel_test_socks5(c_addr.as_ptr(), native_buf.as_mut_ptr(), out_len as usize)
        };
        // 查找 C 字符串的 null 结尾位置
        let str_len = native_buf.iter().position(|&b| b == 0).unwrap_or(0);
        if str_len > 0 {
            // 将结果字符串写入 Java byte[] 缓冲区
            let _ = env.set_byte_array_region(&out_buf, 0, unsafe {
                std::slice::from_raw_parts(native_buf.as_ptr() as *const i8, str_len)
            });
        }
        result
    }

    /// 对应 Java: TunnelCore.setProtectSocketCallback(Object vpnService, String methodName)
    ///
    /// 注册 socket 保护回调。这是 Android VPN 的关键机制：
    /// 1. Java 传入 VpnService 实例和方法名
    /// 2. Rust 用 GlobalRef 持有该实例（防止 Java GC 回收）
    /// 3. 之后当 Rust 需要创建绕过 VPN 的 socket 时，通过 JNI 回调该方法
    /// 4. 方法内部调用 VpnService.protect(fd) 标记该 socket
    #[no_mangle]
    pub extern "system" fn Java_com_vpndemo_app_TunnelCore_setProtectSocketCallback(
        mut env: JNIEnv,
        _class: JClass,
        vpn_service: JObject,
        method_name: JString,
    ) {
        // 获取方法名字符串
        let method: String = match env.get_string(&method_name) {
            Ok(s) => s.into(),
            Err(_) => return,
        };
        // 创建 GlobalRef：将局部引用升级为全局引用
        // 局部引用在 JNI 函数返回后就会失效（被 GC 回收）
        // GlobalRef 会阻止 GC 回收该对象，直到显式 delete
        let global_ref = match env.new_global_ref(vpn_service) {
            Ok(g) => g,
            Err(_) => return,
        };
        // 将 GlobalRef 和方法名存入 ffi.rs 的静态变量中
        // 后续 protect_socket() 调用时使用
        ffi::android_protect::set_protect_callback(global_ref, method);
    }
}
