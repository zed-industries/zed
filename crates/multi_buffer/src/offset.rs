mod private {
    pub trait Sealed {}
}
pub trait Space: private::Sealed + 'static {}

pub struct Buffer;

impl private::Sealed for Buffer {}
impl Space for Buffer {}

pub struct MultiBuffer;

impl private::Sealed for MultiBuffer {}
impl Space for MultiBuffer {}

#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub struct Offset<S: 'static>(usize, core::marker::PhantomData<&'static S>);

impl<S: 'static> Offset<S> {
    pub fn new(val: usize) -> Self {
        Self(val, Default::default())
    }
}

pub trait ToOffset<Target: Space> {
    type Context;
    fn to_offset(&self, cx: &Self::Context) -> Offset<Target>;
}
