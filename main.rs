#![feature(lang_items)]
#![feature(no_core)]

#![feature(libc)]
#![no_core]

extern crate libc;
extern crate core;

/// A fast compressor/decompressor
///
/// Snappy is a compression/decompression library. It does not aim for maximum compression, or compatibility with any other compression library; instead, it aims for very high speeds and reasonable compression.
mod snappy;

use libc::{printf, fprintf, open};
use libc::{strcmp, strcat};
use libc::exit;
use libc::mmap;

use core::panic::PanicInfo;

#[no_mangle]
/// Exported main function
///
/// * snappy \[files\]
/// compress each file with suffix .snappy, - is stdin
///
/// * unsnappy \[file\] \[output\]
/// uncompress file to output, which default is stdout
pub unsafe extern "C" fn _main(argc: i32, argv: *const [*const i8]) -> i32 {
  if argc == 1 {
    print("snappy [files]\ncompress each file with suffix .snappy, `-' means stdin\n\nunsnappy [file] [output]\nuncompress file to output, which default is stdout\n\00");
    exit(1);
  }

  if strcmp((*argv)[0], "snappy\00".as_ptr() as *const i8) == 0
  || strcmp((*argv)[0], "./snappy\00".as_ptr() as *const i8) == 0
  {
    print("Snappy running in zip mode\n\00");

    snappy_zip(argc - 1, argv);
  } else {
    print("Snappy running in unzip mode\n\00");

    if argc > 3 { print("Too much arguments given\n\00"); exit(1); }
    let o = if argc == 2 { 1 } else { open((*argv)[2], libc::O_WRONLY) };

    snappy_unzip((*argv)[1], o);
  }

  return 0;
}

/// Zip a set of file
pub unsafe extern "C" fn snappy_zip(num: i32, pathes: *const [*const i8]) {
}


/// Unzip a snappy zipped file
pub unsafe extern "C" fn snappy_unzip(input: *const i8, output: i32) {
}

/// Display rust &str to stdout
pub unsafe fn print(str: &str) {
  printf(str.as_ptr() as *const i8);
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

