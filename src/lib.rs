use winapi::um::winnt::DLL_PROCESS_ATTACH;

mod class_loader;
mod jvm;

use class_loader::ClassLoader;

#[no_mangle]
pub unsafe extern "stdcall" fn DllMain(_instance: u32, reason: u32, _reserved: *mut u8) -> bool {

    if reason == DLL_PROCESS_ATTACH {
        ClassLoader::setup_hook();
    }

    return true;
}
