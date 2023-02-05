use jni::sys::{JNIEnv, jbyte, jsize, jobject, jclass};
use winapi::ctypes::c_char;

pub type DefineClassCommon = fn(JNIEnv, *const c_char, jobject, *const jbyte, jsize, jobject, *const c_char) -> jclass;
