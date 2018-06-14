#![feature(repr_transparent)]
mod js_string_utils;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use serde_json::{Value, Error};
use std::os::raw::c_char;
use std::ffi::CString;
use std::str;

#[derive(Serialize, Deserialize)]
struct strList{
    list: Vec<String>
}

#[no_mangle]
pub unsafe extern "C" fn get_hello(s: js_string_utils::JsInteropString) -> *mut c_char {
    let s = s.into_boxed_string();
    sget_hello(&s)
}

#[no_mangle]
pub unsafe extern "C" fn get_hello_from_json(s: js_string_utils::JsInteropString) -> *mut c_char {
    let s = s.into_boxed_string();

    let strList: strList = serde_json::from_str(&s).unwrap(); 
    let mut rets: &str = "";
    for s0 in &strList.list {
        rets = s0.as_str();
        break;
    }

    sget_hello(rets)
}

// #[no_mangle]
// pub unsafe extern "C" fn get_hello_from_list(strList: Vec<js_string_utils::JsInteropString>) -> *mut c_char {
//     let s = "";
//     for s0 in strList {
//         s = &s0.into_boxed_string();
//         break;
//     }
// 
//     sget_hello(&s)
// }

// #[no_mangle]
// pub unsafe extern "C" fn get_hello_from_list(strList: Vec<js_string_utils::JsInteropString>) -> *mut c_char {
//     let s = "";
//     for s0 in strList {
//         s = s0.into_boxed_string();
//         break;
//     }
// 
//     sget_hello(&s)
// }

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

// // for stringList
// #[no_mangle]
// pub unsafe extern "C" fn stringListPrepare(cap: usize) -> js_string_utils::JsInteropStringList {
//     js_string_utils::JsInteropStringList::with_capacity(cap)
// }

