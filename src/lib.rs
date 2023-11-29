mod class_loader;
mod jvm;
mod environment;

use std::os::raw::c_void;
use jni::{JavaVM, JNIEnv};
use jni::sys::{jint, JNI_VERSION_1_6};
use class_loader::ClassLoader;

#[no_mangle]
pub unsafe extern "system" fn JNI_OnLoad(vm: JavaVM<>, _reserved: *mut c_void) -> jint {
    let env = vm.get_env().unwrap();
    ClassLoader::setup_hook(JNIEnv::from_raw(env.get_native_interface()).unwrap());

    return JNI_VERSION_1_6;
}
