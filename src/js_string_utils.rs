#![feature(repr_transparent)]

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

