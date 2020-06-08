#![cfg(windows)]

mod mem;

use winapi::shared::minwindef::{HINSTANCE, DWORD, LPVOID, BOOL, TRUE};
use winapi::um::consoleapi;

use mem::{find_signature, read_addr, HaloAddr};
use std::thread::{JoinHandle, spawn, sleep};
use std::time::Duration;
use std::ffi::{CStr};

#[no_mangle]
extern "system" fn DllMain(_dll_module: HINSTANCE, call_reason: DWORD, _reserved: LPVOID) -> BOOL {
    match call_reason {
        1 => init(),
        0 => revert(),
        _ => ()
    }
    TRUE
}

static mut HURAGOK_RUNNING: bool = true;
static mut HURAGOK_HANDLE: Option<JoinHandle<()>> = None;

const START_ADDR: HaloAddr = 0x401000;
const END_ADDR: HaloAddr = 0x5DF000;
const MAP_NAME_SIGNATURE: &[Option<u8>] = &[
    None,
    None,
    None,
    None,
    Some(0xE8),
    None,
    None,
    None,
    None,
    Some(0x32),
    Some(0xC9),
    Some(0x83),
    Some(0xF8),
    Some(0x13),
    Some(0x7D),
];

fn huragok_thread_main() {
    unsafe {
        while HURAGOK_RUNNING {
            sleep(Duration::from_secs(5));
            if let Some(map_name_ptr_ptr) = find_signature(MAP_NAME_SIGNATURE, START_ADDR, END_ADDR) {
                let map_name_ptr: HaloAddr = read_addr(map_name_ptr_ptr);
                let map_name = CStr::from_ptr(map_name_ptr as *const i8)
                    .to_str()
                    .expect("Failed to interpret map name as valid UTF8");
                println!("Current map name: {}", map_name);
            }
        }
    }
}

fn init() {
    unsafe {
        consoleapi::AllocConsole();
        println!("Initializing Huragok; starting worker thread");
        HURAGOK_HANDLE = Some(spawn(|| {
            huragok_thread_main();
        }));
    }
}

fn revert() {
    unsafe {
        println!("Reverting Huragok");
        HURAGOK_RUNNING = false;
        let opt_handle = HURAGOK_HANDLE.take();
        if let Some(handle) = opt_handle {
            println!("Waiting for worker thread to shut down");
            match handle.join() {
                Err(_) => println!("Worker thread panicked!"),
                _ => ()
            }
        }
    }
}
