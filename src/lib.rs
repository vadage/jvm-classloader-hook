mod class_loader;
mod jvm;

use std::os::raw::c_void;
use jni::JavaVM;
use jni::sys::{jint, JNI_VERSION_1_6};
use class_loader::ClassLoader;

#[no_mangle]
pub unsafe extern "system" fn JNI_OnLoad(_: JavaVM, _: *mut c_void) -> jint {
    ClassLoader::setup_hook();
    return JNI_VERSION_1_6;
}
