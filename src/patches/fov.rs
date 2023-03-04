use crate::mem::{Signature, Addr, write_addr, deprotect, read_addr};

const FOV_SIGNATURES: &[(Addr, &'static str)] = &[
    (0, "C3 F5 48 3F"), //0.785000026
];

// static FOV_REPLACEMENT: f32 = 90.0 * 0.85 / 70.0;
static FOV_REPLACEMENT: f32 = 1.2;

pub fn init_fov() {
    println!("Initializing FOV patch");
    for (i, &(offset, pattern)) in FOV_SIGNATURES.iter().enumerate() {
        let sig = Signature::from_str(offset, pattern);
        if let Some(addr) = sig.find() {
            // println!("Found FOV signature {} at {:X}", i, addr);
            deprotect(addr, std::mem::size_of::<f32>());
            write_addr(addr, &FOV_REPLACEMENT.to_le_bytes());
        } else {
            // println!("Failed to find FOV signature {}", i);
        }
    }
}
