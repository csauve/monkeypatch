use crate::mem::{Signature, Addr, write_addr, deprotect};

const FOV_SIGNATURES: &[(Addr, &'static str)] = &[
    (11, "C7 45 FC CC CC CC CC F3 0F 10 05 ?? ?? ?? ?? F3 0F 11 45 FC F3 0F 10 45 08 F3 0F 59 05 50 DD 93 00"),
    (9, "F3 0F 10 45 F8 F3 0F 59 05 ?? ?? ?? ?? F3 0F 11 45 F4 D9 45 F4"),
];

static FOV_REPLACEMENT: f32 = 90.0 * 0.85 / 70.0;

pub fn init_fov() {
    println!("Initializing FOV patch");
    for (i, &(offset, pattern)) in FOV_SIGNATURES.iter().enumerate() {
        let sig = Signature::from_str(offset, pattern);
        if let Some(addr) = sig.find() {
            println!("Found FOV signature {} at {:X}", i, addr);
            deprotect(addr, std::mem::size_of::<f32>());
            let fov_ptr = &FOV_REPLACEMENT as *const f32;
            write_addr(addr, &(fov_ptr as Addr).to_le_bytes());
        } else {
            println!("Failed to find FOV signature {}", i);
        }
    }
}
