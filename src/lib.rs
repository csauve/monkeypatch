#![cfg(windows)]

use winapi::shared::minwindef::{HINSTANCE, DWORD, LPVOID, BOOL, TRUE};
use winapi::um::consoleapi;

#[no_mangle]
extern "system" fn DllMain(dll_module: HINSTANCE, call_reason: DWORD, reserved: LPVOID) -> BOOL {
    match call_reason {
        1 => init(),
        0 => revert(),
        _ => ()
    }
    TRUE
}

fn init() {
    unsafe {
        consoleapi::AllocConsole();
    }
    println!("Initializing Huragok");
}

fn revert() {
    println!("Reverting Huragok");
}
