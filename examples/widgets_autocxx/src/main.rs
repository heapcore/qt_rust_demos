use autocxx::prelude::*;
use chrono::Local;
use std::ffi::CString;

include_cpp! {
    #include "qt_bridge.h"
    generate!("qt_bridge::run_qt_app_with_label")
    safety!(unsafe)
}

fn build_greeting() -> CString {
    let now = Local::now();
    let msg = format!("Hello from Rust at {}", now.format("%H:%M:%S"));
    CString::new(msg).unwrap()
}

fn main() {
    // All meaningful calculation happens in Rust; we only borrow Qt for the GUI shell.
    let greeting = build_greeting();
    unsafe {
        ffi::qt_bridge::run_qt_app_with_label(greeting.as_ptr());
    }
}
