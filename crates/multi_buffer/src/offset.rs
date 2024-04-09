use std::ops::Sub;

use crate::{MultiBuffer, MultiBufferSnapshot};

#[derive(Hash)]
pub struct Offset<S>(pub usize, core::marker::PhantomData<S>);

impl<S> Copy for Offset<S> {}

impl Sub for Offset<MultiBuffer> {
    type Output = usize;
    fn sub(self, rhs: Offset<MultiBuffer>) -> Self::Output {
        self.0 - rhs.0
    }
}

impl<S> PartialEq for Offset<S> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<S> Eq for Offset<S> {}

impl<S> PartialOrd for Offset<S> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<S> Ord for Offset<S> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<S> Clone for Offset<S> {
    fn clone(&self) -> Self {
        Self(self.0, Default::default())
    }
}

impl<S: 'static> std::fmt::Debug for Offset<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Offset<{}>({})", std::any::type_name::<S>(), self.0)
    }
}

impl<S: 'static> Offset<S> {
    pub fn new(val: usize) -> Self {
        Self(val, Default::default())
    }
}

pub trait ToOffset<S: WithMapping> {
    fn to_offset(&self, cx: &S::Mapping) -> Offset<S>;
}

// impl ToOffset<Buffer> for Offset<MultiBuffer> {
//     fn to_offset(&self, cx: Self::Mapping) -> Offset<Buffer> {
//         self
//     }
// }

trait WithMapping {
    type Mapping;
}

impl WithMapping for MultiBuffer {
    type Mapping = MultiBufferSnapshot;
}

impl ToOffset<MultiBuffer> for Offset<MultiBuffer> {
    fn to_offset(&self, cx: &MultiBufferSnapshot) -> Offset<MultiBuffer> {
        self
    }
}
