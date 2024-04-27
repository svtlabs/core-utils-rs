use std::ffi::c_void;

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

use libffi::high::ClosureOnce0;
use libffi::high::FnPtr0;

pub type ClosurePointer<'a> = *const FnPtr0<'a, ()>;
pub type ClosureCaller = extern "C" fn(ClosurePointer, *const c_void);
pub struct VoidTrampoline {
    closure: ClosureOnce0<()>,
    pub caller: ClosureCaller,
}

pub trait Trampoline {}

impl Trampoline for VoidTrampoline {}

impl VoidTrampoline {
    pub fn new<F: FnOnce() + 'static>(outter: F) -> Self {
        extern "C" fn caller(closure: ClosurePointer, _: *const c_void) {
            let callable = unsafe { closure.as_ref() }.unwrap();
            callable.call();
        }
        Self {
            closure: ClosureOnce0::new(outter),
            caller,
        }
    }
    pub fn as_code_ptr(&self) -> ClosurePointer {
        self.closure.code_ptr()
    }
}
