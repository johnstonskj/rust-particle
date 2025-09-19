// ------------------------------------------------------------------------------------------------
// Implementations ❱ AsInner
// ------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! impl_as_ref_for_as_inner {
    ($wrapper:ty => $inner:ty) => {
        impl core::convert::AsRef<$inner> for $wrapper {
            fn as_ref(&self) -> &$inner {
                self.as_inner()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_borrow_for_as_inner {
    ($wrapper:ty => $inner:ty) => {
        impl core::borrow::Borrow<$inner> for $wrapper {
            fn borrow(&self) -> &$inner {
                self.as_inner()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_deref_for_as_inner {
    ($wrapper:ty => $inner:ty) => {
        impl core::ops::Deref for $wrapper {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                self.as_inner()
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ AsInnerMut
// ------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! impl_as_ref_mut_for_as_inner_mut {
    ($wrapper:ty => $inner:ty) => {
        impl core::convert::AsRefMut<$inner> for $wrapper {
            fn as_ref_mut(&mut self) -> &mut $inner {
                <Self as $crate::wrappers::AsInnerMut>::as_inner_mut(self)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_borrow_mut_for_as_inner_mut {
    ($wrapper:ty => $inner:ty) => {
        impl core::borrow::BorrowMut<$inner> for $wrapper {
            fn borrow_mut(&mut self) -> &mut $inner {
                <Self as $crate::wrappers::AsInnerMut>::as_inner_mut(self)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_deref_mut_for_as_inner_mut {
    ($wrapper:ty => $inner:ty) => {
        impl core::ops::DerefMut for $wrapper {
            type Target = $inner;

            fn deref_mut(&mut self) -> &mut Self::Target {
                self.as_inner_mut()
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ IntoInner
// ------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! impl_from_wrapper_for_into_inner {
    ($wrapper:ty => $inner:ty) => {
        impl core::convert::From<$wrapper> for $inner {
            fn from(wrapper: $wrapper) -> $inner {
                wrapper.into_inner()
            }
        }
    };
}

// ------------------------------------------------------------------------------------------------
// Implementations ❱ FromInner
// ------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! impl_from_inner_for_from_inner {
    ($wrapper:ty => $inner:ty) => {
        impl core::convert::From<$inner> for $wrapper {
            fn from(inner: $inner) -> $wrapper {
                Self::from_inner(inner)
            }
        }
    };
}
