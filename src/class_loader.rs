use std::ffi::CString;
use std::mem;
use std::slice::from_raw_parts_mut;

use detour::static_detour;
use jni::sys::{jbyte, jclass, JNIEnv, jobject, jsize};
use winapi::ctypes::c_char;
use winapi::um::libloaderapi::{GetModuleHandleA, GetProcAddress};

use crate::jvm::DefineClassCommon;

static_detour! {
    static DefineClassCommonHook: fn(JNIEnv, *const c_char, jobject, *const jbyte, jsize, jobject, *const c_char) -> jclass;
}

const fn convert_magic_number(integer: u32) -> [jbyte; 4] {
    let bytes = integer.to_be_bytes();
    return [bytes[0] as jbyte, bytes[1] as jbyte, bytes[2] as jbyte, bytes[3] as jbyte];
}

pub struct ClassLoader {}

impl ClassLoader {
    const CUSTOM_MAGIC_VALUE: [jbyte; 4] = convert_magic_number(0xDEADC0DE);
    const JAVA_MAGIC_VALUE: [jbyte; 4] = convert_magic_number(0xCAFEBABE);

    pub unsafe fn setup_hook() {
        let module = CString::new("jvm.dll").unwrap();
        let handle = GetModuleHandleA(module.as_ptr());

        let method_name = CString::new("JVM_DefineClassWithSource").unwrap();
        let method = GetProcAddress(handle, method_name.as_ptr());
        let target: DefineClassCommon = mem::transmute(method);

        DefineClassCommonHook.initialize(target, ClassLoader::hooked_define_class_common)
            .unwrap()
            .enable()
            .expect("Unable to hook into class loading.");
    }

    fn hooked_define_class_common(env: JNIEnv, name: *const c_char, loader: jobject, buf: *const jbyte, len: jsize, pd: jobject, source: *const c_char) -> jclass {
        unsafe {
            let bytes = from_raw_parts_mut(buf as *mut jbyte, len as usize);

            if ClassLoader::is_custom_payload(bytes) {
                ClassLoader::decrypt_custom_payload(bytes);
            }

            return DefineClassCommonHook.call(env, name, loader, bytes.as_ptr(), len, pd, source);
        }
    }

    unsafe fn is_custom_payload(bytes: &mut [jbyte]) -> bool {
        for i in 0..ClassLoader::CUSTOM_MAGIC_VALUE.len() {
            if bytes[i] != ClassLoader::CUSTOM_MAGIC_VALUE[i] {
                return false;
            }
        }
        return true;
    }

    unsafe fn decrypt_custom_payload(bytes: &mut [jbyte]) {
        for i in 0..bytes.len() {
            bytes[i] ^= 42;
        }

        for i in 0..ClassLoader::JAVA_MAGIC_VALUE.len() {
            bytes[i] = ClassLoader::JAVA_MAGIC_VALUE[i];
        }
    }
}
