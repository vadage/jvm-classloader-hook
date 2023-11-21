use std::ops::Deref;
use std::os::raw::c_char;
use std::slice::from_raw_parts_mut;

use retour::static_detour;
use jni::sys::{jbyte, jclass, JNIEnv, jobject, jsize};
use libloading::Library;
use crate::environment;

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
        let handle = Library::new("jvm").expect("Could not find jvm library.");
        let method = handle.get::<DefineClassCommon>(b"JVM_DefineClassWithSource").expect("Could not find exported function.");

        let hook = DefineClassCommonHook.initialize(*method.deref(), ClassLoader::hooked_define_class_common).expect("Could not initialize hook for class loading.");
        hook.enable().expect("Could not to hook into class loading.");
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
        let key_bytes = environment::ENCRYPTION_KEY.as_bytes();
        for i in 0..bytes.len() {
            bytes[i] ^= key_bytes[i % key_bytes.len()] as i8;
        }

        for i in 0..ClassLoader::JAVA_MAGIC_VALUE.len() {
            bytes[i] = ClassLoader::JAVA_MAGIC_VALUE[i];
        }
    }
}
