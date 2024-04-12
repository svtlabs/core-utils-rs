use std::{
    ffi::c_void,
    ptr::{self, drop_in_place},
};

use core_foundation::base::{OSStatus, TCFType};

// In Apple's frameworks, a refcon parameter is often used as a way to pass user-defined data into a callback function. This is a common pattern in C and Objective-C APIs, especially those that deal with low-level system or hardware interactions.
//
// The term refcon stands for "reference constant". It's typically a void * pointer, which means it can point to any type of data. You're responsible for casting it to the correct type within your callback function.
//
// Here's a simple example:
//
// ;
// In this example, SomeAppleAPIFunction is a hypothetical function in an Apple framework that takes a callback function and a refcon as parameters. We pass a pointer to data as the refcon, and then in MyCallbackFunction we cast the refcon back to a MyDataStruct * so we can access its fields.
//
// Remember that you need to ensure the data you're pointing to stays valid until the callback is called. If the data is a local variable in a function, and that function returns before the callback is called, then the refcon will be pointing to invalid memory. In such cases, you might need to dynamically allocate memory for the data (using malloc, for example) and then free it in the callback.
pub type RefCon = *mut c_void;

trait LeakedRefCon {
    fn into_leaked_mut_ptr(self) -> *mut c_void;
}

impl<T> LeakedRefCon for Option<T> {
    fn into_leaked_mut_ptr(self) -> *mut c_void {
        self.map_or(ptr::null_mut(), |r| {
            Box::into_raw(Box::new(r)) as *mut c_void
        })
    }
}

#[repr(C)]
pub struct TrampolineRefCon(pub RefCon, pub *mut c_void);
impl TrampolineRefCon {
    pub fn new<T, F>(refcon: Option<T>, callback: F) -> Self {
        Self(
            refcon.into_leaked_mut_ptr(),
            Box::leak(Box::new(callback)) as *mut F as *mut c_void,
        )
    }
    pub fn into_leaked_mut_ptr(self) -> *mut Self {
        let cb = Box::new(self);
        Box::into_raw(cb)
    }
}

impl Drop for TrampolineRefCon {
    fn drop(&mut self) {
        unsafe { drop_in_place(self.1) };
    }
}

/// .
///
/// # Safety
///
/// .
pub unsafe extern "C" fn trampoline<
    Param: TCFType,
    T,
    Error: Into<OSStatus>,
    F: FnMut(Param, T) -> Result<(), Error> + Send + 'static,
>(
    param: Param::Ref,
    refcon: *mut TrampolineRefCon,
) -> OSStatus {
    let refcon_data = &*(refcon);
    let mut user_data: F = ptr::read(refcon_data.0.cast());
    let ret = user_data(
        Param::wrap_under_get_rule(param),
        ptr::read(refcon_data.0.cast()),
    )
    .map_or_else(|err| err.into(), |_| 0);
    ptr::drop_in_place(refcon);
    ret
}
