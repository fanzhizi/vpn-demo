//! JNI FFI for Android. Wraps the C FFI functions for Java/Kotlin calls.

#[cfg(target_os = "android")]
pub mod android {
    use std::ffi::CString;
    use jni::JNIEnv;
    use jni::objects::{JByteArray, JClass, JString};
    use jni::sys::{jboolean, jint, JNI_FALSE, JNI_TRUE};

    use jni::objects::JObject;
    use crate::ffi;

    #[no_mangle]
    pub extern "system" fn Java_com_vpndemo_app_TunnelCore_tunnelStart(
        mut env: JNIEnv,
        _class: JClass,
        socks5_addr: JString,
    ) -> jboolean {
        let addr: String = match env.get_string(&socks5_addr) {
            Ok(s) => s.into(),
            Err(_) => return JNI_FALSE,
        };
        let c_addr = match CString::new(addr) {
            Ok(c) => c,
            Err(_) => return JNI_FALSE,
        };
        let result = unsafe { ffi::tunnel_start(c_addr.as_ptr()) };
        if result { JNI_TRUE } else { JNI_FALSE }
    }

    #[no_mangle]
    pub extern "system" fn Java_com_vpndemo_app_TunnelCore_tunnelStop(
        _env: JNIEnv,
        _class: JClass,
    ) {
        ffi::tunnel_stop();
    }

    #[no_mangle]
    pub extern "system" fn Java_com_vpndemo_app_TunnelCore_tunnelFeedPacket(
        env: JNIEnv,
        _class: JClass,
        data: JByteArray,
        len: jint,
    ) -> jboolean {
        let bytes = match env.convert_byte_array(&data) {
            Ok(b) => b,
            Err(_) => return JNI_FALSE,
        };
        let result = unsafe {
            ffi::tunnel_feed_packet(bytes.as_ptr(), len as usize)
        };
        if result { JNI_TRUE } else { JNI_FALSE }
    }

    #[no_mangle]
    pub extern "system" fn Java_com_vpndemo_app_TunnelCore_tunnelReadPacket(
        mut env: JNIEnv,
        _class: JClass,
        buf: JByteArray,
        buf_len: jint,
    ) -> jint {
        let mut native_buf = vec![0u8; buf_len as usize];
        let n = unsafe {
            ffi::tunnel_read_packet(native_buf.as_mut_ptr(), buf_len as usize)
        };
        if n > 0 {
            let _ = env.set_byte_array_region(&buf, 0, unsafe {
                std::slice::from_raw_parts(native_buf.as_ptr() as *const i8, n)
            });
        }
        n as jint
    }

    #[no_mangle]
    pub extern "system" fn Java_com_vpndemo_app_TunnelCore_tunnelIsRunning(
        _env: JNIEnv,
        _class: JClass,
    ) -> jboolean {
        if ffi::tunnel_is_running() { JNI_TRUE } else { JNI_FALSE }
    }

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
        let str_len = native_buf.iter().position(|&b| b == 0).unwrap_or(0);
        if str_len > 0 {
            let _ = env.set_byte_array_region(&out_buf, 0, unsafe {
                std::slice::from_raw_parts(native_buf.as_ptr() as *const i8, str_len)
            });
        }
        result
    }

    #[no_mangle]
    pub extern "system" fn Java_com_vpndemo_app_TunnelCore_setProtectSocketCallback(
        mut env: JNIEnv,
        _class: JClass,
        vpn_service: JObject,
        method_name: JString,
    ) {
        let method: String = match env.get_string(&method_name) {
            Ok(s) => s.into(),
            Err(_) => return,
        };
        let global_ref = match env.new_global_ref(vpn_service) {
            Ok(g) => g,
            Err(_) => return,
        };
        ffi::android_protect::set_protect_callback(global_ref, method);
    }
}
