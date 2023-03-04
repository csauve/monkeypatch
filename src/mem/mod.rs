use std::ptr;
use winapi::um::memoryapi::VirtualProtect;
use winapi::ctypes::c_void;


const START_ADDR: Addr = 0x400000;
const END_ADDR:   Addr = 0xD00000;

pub type Addr = u32;

pub struct Signature {
    pattern: Vec<Option<u8>>,
    offset: Addr,
}

pub fn read_addr<T: Copy>(addr: Addr) -> T {
    unsafe {
        ptr::read_unaligned(addr as *const T)
    }
}

//sets PAGE_EXECUTE_READWRITE
pub fn deprotect(addr: Addr, size: usize) {
    let mut out: u32 = 0;
    unsafe {
        VirtualProtect(addr as *mut c_void, size, 0x40u32, &mut out);
    }
}

pub fn write_addr<T: Copy>(addr: Addr, data: &T) {
    unsafe {
        ptr::write_unaligned(addr as *mut T, *data);
    }
}

pub fn find_byte_pattern(pattern: &[Option<u8>]) -> Option<Addr> {
    let mut curr_addr = START_ADDR;
    'outer: while curr_addr <= END_ADDR {
        for (offset, &match_byte) in pattern.iter().enumerate() {
            if let Some(match_value) = match_byte {
                let scan_addr = curr_addr + offset as Addr;
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

impl Signature {
    pub fn from_str(offset: Addr, pattern: &str) -> Signature {
        Signature {
            offset,
            pattern: pattern.split_whitespace()
                .map(|byte_str|
                    match byte_str {
                        "??" => None,
                        _ => Some(u8::from_str_radix(byte_str, 16).unwrap())
                    }
                )
                .collect()
        }
    }

    pub fn find(&self) -> Option<Addr> {
        find_byte_pattern(&self.pattern).map(|addr| addr + self.offset)
    }
}
