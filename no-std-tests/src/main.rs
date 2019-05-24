#![feature(lang_items, start, panic_implementation)]
#![no_std]

extern crate libc;
extern crate sophon_types;
extern crate sofbloom;
extern crate fixed_hash;

use sophon_types::{Address, Public, Secret, Signature};

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
   0
}

#[cfg(not(test))]
#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn rust_eh_personality() {}

#[cfg(not(test))]
#[panic_implementation]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        libc::abort();
    }
}
