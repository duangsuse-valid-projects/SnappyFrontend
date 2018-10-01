#![feature(lang_items)]
#![feature(libc)]
#![no_core]

use libc::printf;

#[no_mangle]
pub extern "C" fn main() -> i32 {
  return 0;
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
fn panic_fmt() -> ! {
  loop {}
}
