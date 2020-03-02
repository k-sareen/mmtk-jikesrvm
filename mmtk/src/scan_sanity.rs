use libc::c_void;
use mmtk::{TraceLocal};
use mmtk::util::OpaquePointer;
use entrypoint::*;
use JTOC_BASE;

pub fn scan_boot_image_sanity<T: TraceLocal>(trace: &mut T, tls: OpaquePointer) {
    trace!("scan_boot_image_sanity");
    let mut boot_image_roots: [usize; 10000] = [0; 10000];
    let addr = &boot_image_roots as *const usize;

    unsafe {
        jtoc_call!(SCAN_BOOT_IMAGE_METHOD_OFFSET, tls, addr);
    }

    for i in 0..10000 {
        let slot = boot_image_roots[i];
        if slot == 0 {
            break;
        }
        print!("0x{:X} ", boot_image_roots[i]);
    }
    println!();
}