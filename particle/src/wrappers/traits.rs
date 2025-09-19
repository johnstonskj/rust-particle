use core::{error::Error, result::Result};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait Opaque {}

pub trait AsInner<Inner: ?Sized> {
    fn as_inner(&self) -> &Inner;
}

pub trait AsInnerMut<Inner: ?Sized> {
    fn as_inner_mut(&mut self) -> &mut Inner;
}

pub trait IntoInner<Inner> {
    fn into_inner(self) -> Inner;
}

pub trait FromInner<Inner> {
    fn from_inner(inner: Inner) -> Self;
}

pub trait Untrusted<Inner>: FromInner<Inner> {
    type Error: Error;
    type Trusted: Trusted<Inner>;

    fn trust(self) -> Result<Self::Trusted, Self::Error>;
}

pub trait Trusted<Inner>: AsInner<Inner> + AsInnerMut<Inner> + IntoInner<Inner> {
    type Untrusted: Untrusted<Inner>;

    fn untrust(self) -> Self::Untrusted;
}
