
#[macro_export]
macro_rules! declare_TCFType {
    (
        $(#[$doc:meta])*
        $ty:ident<$lifetime:tt>, $raw:ident
    ) => {
        $(#[$doc])*
        pub struct $ty<$lifetime>($raw, ::std::marker::PhantomData<&$lifetime ()>);


        impl <$lifetime> Drop for $ty<$lifetime> {
            fn drop(&mut self) {
                unsafe { core_foundation::base::CFRelease(self.as_CFTypeRef()) }
            }
        }
    };
    (
        $(#[$doc:meta])*
        $ty:ident, $raw:ident
    ) => {
        core_foundation::declare_TCFType! {
            $(#[$doc])*
            $ty, $raw
        }
    };
}

#[macro_export]
macro_rules! impl_TCFType {
    ($ty:ident<$lifetime:tt>, $ty_ref:ident, $ty_id:ident) => {
        impl <$lifetime> core_foundation::base::TCFType for $ty<$lifetime> {
            type Ref = $ty_ref;

            #[inline]
            fn as_concrete_TypeRef(&self) -> $ty_ref {
                self.0
            }

            #[inline]
            unsafe fn wrap_under_get_rule(reference: $ty_ref) -> Self {
                assert!(!reference.is_null(), "Attempted to create a NULL object.");
                let reference = core_foundation::base::CFRetain(reference as *const ::std::os::raw::c_void) as $ty_ref;
                core_foundation::base::TCFType::wrap_under_create_rule(reference)
            }

            #[inline]
            fn as_CFTypeRef(&self) -> core_foundation::base::CFTypeRef {
                self.as_concrete_TypeRef() as core_foundation::base::CFTypeRef
            }

            #[inline]
            unsafe fn wrap_under_create_rule(reference: $ty_ref) -> Self {
                assert!(!reference.is_null(), "Attempted to create a NULL object.");
                // we need one PhantomData for each type parameter so call ourselves
                // again with @Phantom $p to produce that
                $ty(reference, ::std::marker::PhantomData)
            }

            #[inline]
            fn type_id() -> core_foundation::base::CFTypeID {
                unsafe {
                    $ty_id()
                }
            }
        }

        impl <$lifetime> Clone for $ty<$lifetime> {
            #[inline]
            fn clone(&self) -> $ty<$lifetime> {
                unsafe {
                    $ty::wrap_under_get_rule(self.0)
                }
            }
        }

        impl <$lifetime> PartialEq for $ty<$lifetime> {
            #[inline]
            fn eq(&self, other: &$ty) -> bool {
                self.as_CFType().eq(&other.as_CFType())
            }
        }

        impl <$lifetime> Eq for $ty<$lifetime> { }

        unsafe impl<$lifetime, 'b> core_foundation::base::ToVoid<$ty<$lifetime>> for &'b $ty<$lifetime> {
            fn to_void(&self) -> *const ::std::os::raw::c_void {
                use core_foundation::base::TCFTypeRef;
                self.as_concrete_TypeRef().as_void_ptr()
            }
        }

        unsafe impl <$lifetime> core_foundation::base::ToVoid<$ty<$lifetime>> for $ty<$lifetime> {
            fn to_void(&self) -> *const ::std::os::raw::c_void {
                use core_foundation::base::TCFTypeRef;
                self.as_concrete_TypeRef().as_void_ptr()
            }
        }

        unsafe impl <$lifetime> core_foundation::base::ToVoid<$ty<$lifetime>> for $ty_ref {
            fn to_void(&self) -> *const ::std::os::raw::c_void {
                use core_foundation::base::TCFTypeRef;
                self.as_void_ptr()
            }
        }

    };

    ($ty:ident, $ty_ref:ident, $ty_id:ident) => {
        core_foundation::impl_TCFType!($ty, $ty_ref, $ty_id);
    };

    ($ty:ident<$($p:ident $(: $bound:path)*),*>, $ty_ref:ident, $ty_id:ident) => {
       core_foundation::impl_TCFType!($ty<$($p $(: $bound)*),*>, $ty_ref, $ty_id);
    };

    (@Phantom $x:ident) => {
       core_foundation::impl_TCFType!(@Phantom $x);
    };
}
