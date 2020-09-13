use std::ptr;
use winapi::um::memoryapi::{VirtualProtect};
use winapi::ctypes::c_void;
pub type HaloAddr = u32;

pub fn read_addr<T: Copy>(addr: HaloAddr) -> T {
    unsafe {
        ptr::read_unaligned(addr as *const T)
    }
}

//sets PAGE_EXECUTE_READWRITE
pub fn deprotect(addr: HaloAddr, size: usize) {
    let mut out: u32 = 0;
    unsafe {
        VirtualProtect(addr as *mut c_void, size, 0x40u32, &mut out);
    }
}

pub fn write_addr<T: Copy>(addr: HaloAddr, data: &T) {
    unsafe {
        ptr::write_unaligned(addr as *mut T, *data);
    }
}

pub fn find_signature(match_bytes: &[Option<u8>], start_addr: HaloAddr, end_addr: HaloAddr) -> Option<HaloAddr> {
    let mut curr_addr = start_addr;
    'outer: while curr_addr <= end_addr {
        for (offset, &match_byte) in match_bytes.iter().enumerate() {
            if let Some(match_value) = match_byte {
                let scan_addr = curr_addr + offset as HaloAddr;
                if read_addr::<u8>(scan_addr) != match_value {
                    curr_addr += 1;
                    continue 'outer;
                }
            }
        }
        return Some(curr_addr);
    }
    None
}
