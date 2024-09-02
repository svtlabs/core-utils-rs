use std::{ffi::c_void, ptr};

#[repr(transparent)]
pub struct TrampolineRefcon(*mut c_void);

#[repr(transparent)]
pub struct TrampolineCallback(extern "C" fn(TrampolineRefcon));

pub fn create_trampoline<F: FnOnce() + 'static>(
    wrapped_closure: F,
) -> (TrampolineCallback, TrampolineRefcon) {
    pub extern "C" fn caller<F>(closure_ptr: TrampolineRefcon)
    where
        F: FnOnce(),
    {
        unsafe {
            let closure: F = ptr::read(closure_ptr.0.cast());
            closure();
        };
    }
    (
        TrampolineCallback(caller::<F>),
        TrampolineRefcon(Box::into_raw(Box::new(wrapped_closure)) as *mut c_void),
    )
}
