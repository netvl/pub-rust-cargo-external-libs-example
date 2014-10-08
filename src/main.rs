extern crate libc;

use std::c_str::CString;

use libc::c_char;

#[link(name = "foo")]
extern {
    fn hello_msg() -> *const c_char;
}

fn main() {
    let cbuf = unsafe { CString::new(hello_msg(), false) };
    println!("Hello, {}!", cbuf.as_str().unwrap())
}
