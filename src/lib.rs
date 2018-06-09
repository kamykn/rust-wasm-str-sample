#![feature(repr_transparent)]

use std::os::raw::c_char;

#[no_mangle]
pub unsafe extern "C" fn get_hello(s: JsInteropString) -> *mut c_char {
    let s = s.into_boxed_string();
    real_code::get_hello(&s)
}

mod real_code {
    use std::ffi::CString;
    use std::os::raw::c_char;
    use std::str;

    pub fn get_hello(somestring: &str ) -> *mut c_char {
        let s = CString::new(somestring).unwrap();
        s.into_raw()
    }
}


#[no_mangle]
pub fn get_hello_len() -> usize {
    // HELLO.len()
    return 4;
}

// Very important to use `transparent` to prevent ABI issues
#[repr(transparent)]
pub struct JsInteropString(*mut String);

impl JsInteropString {
    // Unsafe because we create a string and say it's full of valid
    // UTF-8 data, but it isn't!
    unsafe fn with_capacity(cap: usize) -> Self {
        let mut d = Vec::with_capacity(cap);
        d.set_len(cap);
        let s = Box::new(String::from_utf8_unchecked(d));
        JsInteropString(Box::into_raw(s))
    }

    unsafe fn as_string(&self) -> &String {
        &*self.0
    }

    unsafe fn as_mut_string(&mut self) -> &mut String {
        &mut *self.0
    }

    unsafe fn into_boxed_string(self) -> Box<String> {
        Box::from_raw(self.0)
    }

    unsafe fn as_mut_ptr(&mut self) -> *mut u8 {
        self.as_mut_string().as_mut_vec().as_mut_ptr()
    }
}


#[no_mangle]
pub unsafe extern "C" fn stringPrepare(cap: usize) -> JsInteropString {
    JsInteropString::with_capacity(cap)
}

#[no_mangle]
pub unsafe extern "C" fn stringData(mut s: JsInteropString) -> *mut u8 {
    s.as_mut_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn stringLen(s: JsInteropString) -> usize {
    s.as_string().len()
}

