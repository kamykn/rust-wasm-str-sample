#![feature(repr_transparent)]

mod js_string_utils;
use std::os::raw::c_char;
use std::ffi::CString;
use std::str;

#[no_mangle]
pub unsafe extern "C" fn get_hello(s: js_string_utils::JsInteropString) -> *mut c_char {
    let s = s.into_boxed_string();
    sget_hello(&s)
}

pub fn sget_hello(somestring: &str ) -> *mut c_char {
    let s = CString::new(somestring).unwrap();
    s.into_raw()
}

#[no_mangle]
pub fn get_hello_len() -> usize {
    // HELLO.len()
    return 4;
}

#[no_mangle]
pub unsafe extern "C" fn stringPrepare(cap: usize) -> js_string_utils::JsInteropString {
    js_string_utils::JsInteropString::with_capacity(cap)
}

#[no_mangle]
pub unsafe extern "C" fn stringData(mut s: js_string_utils::JsInteropString) -> *mut u8 {
    s.as_mut_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn stringLen(s: js_string_utils::JsInteropString) -> usize {
    s.as_string().len()
}

