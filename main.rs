#![feature(lang_items)]
#![feature(no_core)]

#![feature(libc)]
#![no_core]

extern crate libc;
extern crate core;

mod snappy;

use libc::printf;
use core::panic::PanicInfo;

#[no_mangle]
/// Exported main function
///
/// * snappy \[files\]
/// compress each file with suffix .snappy, - is stdin
///
/// * unsnappy \[file\] \[output\]
/// uncompress file to output, which default is stdout
pub unsafe extern "C" fn _main(argc: i32, argv: *const *const u8) -> i32 {
  printf("Hello, world!\n\00".as_ptr() as *const i8);
  return 0;
}

#[lang = "eh_personality"]
#[no_mangle]
/// Rust error handling personality function
pub extern "C" fn eh_personality() {}

#[panic_handler]
#[no_mangle]
/// Rust panic handler
pub extern "C" fn panic_handler(p: &PanicInfo) -> ! {
  loop {} // never return
}

