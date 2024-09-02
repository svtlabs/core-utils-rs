use std::{ffi::c_void, ptr};

#[repr(transparent)]
pub struct TrampolineRefcon(*mut c_void);

#[repr(transparent)]
pub struct TrampolineCallback<Ret = (), Param = TrampolineRefcon>(
    extern "C" fn(Param, TrampolineRefcon) -> Ret,
);

pub fn create_trampoline<Ret, Param, F: FnOnce(Param) -> Ret + 'static>(
    wrapped_closure: F,
) -> (TrampolineCallback<Ret, Param>, TrampolineRefcon) {
    pub extern "C" fn caller<Ret, Param, F>(param: Param, closure_ptr: TrampolineRefcon) -> Ret
    where
        F: FnOnce(Param) -> Ret,
    {
        println!("caller");
        unsafe {
            let closure: F = ptr::read(closure_ptr.0.cast());
            closure(param)
        }
    }
    (
        TrampolineCallback(caller::<Ret, Param, F>),
        TrampolineRefcon(Box::into_raw(Box::new(wrapped_closure)).cast()),
    )
}
