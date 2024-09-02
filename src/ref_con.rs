pub type TrampolineRefcon = *mut Box<dyn FnOnce()>;
pub type TrampolineCallback = extern "C" fn(TrampolineRefcon);

pub fn create_trampoline<F: FnOnce() + 'static>(
    wrapped_closure: F,
) -> (TrampolineCallback, TrampolineRefcon) {
    pub extern "C" fn caller(closure_ptr: TrampolineRefcon) {
        unsafe {
            let closure = Box::from_raw(closure_ptr);
            closure();
        };
    }
    (caller, Box::into_raw(Box::new(Box::new(wrapped_closure))))
}
