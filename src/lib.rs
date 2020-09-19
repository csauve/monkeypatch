#![cfg(windows)]

mod mem;

use winapi::shared::minwindef::{HINSTANCE, DWORD, LPVOID, BOOL, TRUE};
use winapi::um::consoleapi;

use mem::{find_signature, write_addr, read_addr, deprotect, HaloAddr};
use std::thread::{JoinHandle, spawn, sleep};
use std::time::Duration;

#[no_mangle]
extern "system" fn DllMain(_dll_module: HINSTANCE, call_reason: DWORD, _reserved: LPVOID) -> BOOL {
    match call_reason {
        1 => init(),
        0 => revert(),
        _ => ()
    }
    TRUE
}

static mut MONKEYWRENCH_RUNNING: bool = true;
static mut MONKEYWRENCH_HANDLE: Option<JoinHandle<()>> = None;

const START_ADDR: HaloAddr = 0x400000;
const END_ADDR: HaloAddr = 0x5DF000;

const LIMIT_REPLACEMENT: f32 = 600_000.0;
const LIMIT_SIG_VALUE: f32 = 4950.0;
// const LIMIT_SIGNATURE_ALT: &[Option<u8>] = &[
//     //5000 be: 45 9c 40 00
//     Some(0x00),
//     Some(0x40),
//     Some(0x9c),
//     Some(0x45),
//     //-5000 be: c5 9c 40 00
//     Some(0x00),
//     Some(0x40),
//     Some(0x9c),
//     Some(0xc5),
// ];

fn monkeywrench_thread_main() {
    let limit_signature: Vec<Option<u8>> = LIMIT_SIG_VALUE.to_le_bytes().iter().map(|b| Some(*b)).collect();
    unsafe {
        while MONKEYWRENCH_RUNNING {
            sleep(Duration::from_secs(5));
            if let Some(addr) = find_signature(&limit_signature, START_ADDR, END_ADDR) {
                deprotect(addr, std::mem::size_of::<f64>());
                println!("Limit ptr: {}", addr);
                write_addr(addr, &LIMIT_REPLACEMENT.to_le_bytes());
                let limit: [u8; std::mem::size_of::<f64>()] = read_addr(addr);
                println!("Written value: {}", f64::from_le_bytes(limit));
            } else {
                println!("Signature not found");
            }
        }
    }
}

fn init() {
    unsafe {
        consoleapi::AllocConsole();
        println!("Initializing monkeywrench; starting worker thread");
        MONKEYWRENCH_HANDLE = Some(spawn(|| {
            monkeywrench_thread_main();
        }));
    }
}

fn revert() {
    unsafe {
        println!("Reverting monkeywrench");
        MONKEYWRENCH_RUNNING = false;
        let opt_handle = MONKEYWRENCH_HANDLE.take();
        if let Some(handle) = opt_handle {
            println!("Waiting for worker thread to shut down");
            match handle.join() {
                Err(_) => println!("Worker thread panicked!"),
                _ => ()
            }
        }
    }
}
