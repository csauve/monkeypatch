#![cfg(windows)]

mod mem;
mod patches;

use winapi::shared::minwindef::{HINSTANCE, DWORD, LPVOID, BOOL, TRUE};
// use winapi::um::consoleapi;

#[no_mangle]
extern "system" fn DllMain(_dll_module: HINSTANCE, call_reason: DWORD, _reserved: LPVOID) -> BOOL {
    match call_reason {
        1 => init(),
        _ => ()
    }
    TRUE
}

fn init() {
    // unsafe {
    //     consoleapi::AllocConsole();
    // }
    patches::init_fov();
}
