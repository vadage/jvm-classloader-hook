use std::os::raw::c_char;
use jni::sys::{JNIEnv, jbyte, jsize, jobject, jclass};

pub type DefineClassCommon = fn(JNIEnv, *const c_char, jobject, *const jbyte, jsize, jobject, *const c_char) -> jclass;
