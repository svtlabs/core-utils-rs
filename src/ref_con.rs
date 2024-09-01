pub type ClosurePointer = *mut Box<dyn FnOnce()>;
pub type ClosureCaller = extern "C" fn(ClosurePointer);

pub fn trampoline<F: FnOnce() + 'static>(closure: F) -> (ClosureCaller, ClosurePointer) {
    pub extern "C" fn caller(closure: ClosurePointer) {
        unsafe {
            let closure = Box::from_raw(closure);
            closure();
        };
    }
    (caller, Box::into_raw(Box::new(Box::new(closure))))
}
