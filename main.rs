#![feature(lang_items)]
#![feature(no_core)]

#![feature(libc)]
#![no_core]

#![feature(start)]

extern crate libc;
extern crate core;

use libc::printf;
use core::panic::PanicInfo;

#[no_mangle]
pub unsafe extern "C" fn _main(argc: i32, argv: *const *const u8) -> i32 {
  printf("Hello, world!\n".as_ptr() as *const i8);
  return 0;
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[panic_handler]
#[no_mangle]
pub extern "C" fn panic_handler(p: &PanicInfo) -> ! {
  loop {} // never return
}

