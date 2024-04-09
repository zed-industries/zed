use std::num::Saturating;
use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::{MultiBuffer, MultiBufferSnapshot};

#[derive(Hash)]
pub struct Offset<S>(pub usize, core::marker::PhantomData<S>);

impl<S> Copy for Offset<S> {}

impl<S> Default for Offset<S> {
    fn default() -> Self {
        Offset::new(0)
    }
}

impl Sub for Offset<MultiBuffer> {
    type Output = usize;
    fn sub(self, rhs: Offset<MultiBuffer>) -> Self::Output {
        self.0 - rhs.0
    }
}

use rand::distributions::uniform::{SampleBorrow, SampleUniform, UniformInt, UniformSampler};
use rand::Rng;
use text::TextDimension;

struct OffsetUniform<S>(UniformInt<usize>, core::marker::PhantomData<S>);

impl<S> UniformSampler for OffsetUniform<S> {
    type X = Offset<S>;

    fn new<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        OffsetUniform(
            UniformInt::new(low.borrow().0, high.borrow().0),
            Default::default(),
        )
    }

    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        OffsetUniform(
            UniformInt::new_inclusive(low.borrow().0, high.borrow().0),
            Default::default(),
        )
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        Offset::new(self.0.sample(rng))
    }
}

impl<S> SampleUniform for Offset<S> {
    type Sampler = OffsetUniform<S>;
}

impl Sub<usize> for Offset<MultiBuffer> {
    type Output = Offset<MultiBuffer>;
    fn sub(self, rhs: usize) -> Self::Output {
        Offset(self.0 - rhs, Default::default())
    }
}

impl SubAssign<usize> for Offset<MultiBuffer> {
    fn sub_assign(&mut self, rhs: usize) {
        self.0 -= rhs;
    }
}

impl Add<usize> for Offset<MultiBuffer> {
    type Output = Offset<MultiBuffer>;
    fn add(self, rhs: usize) -> Self::Output {
        Offset(self.0 + rhs, Default::default())
    }
}

impl AddAssign<usize> for Offset<MultiBuffer> {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
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

    pub fn zero() -> Self {
        Self::new(0)
    }

    pub fn saturating_sub(self, rhs: Self) -> usize {
        self.0.saturating_sub(rhs.0)
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

pub trait WithMapping {
    type Mapping;
}

impl WithMapping for MultiBuffer {
    type Mapping = MultiBufferSnapshot;
}

impl ToOffset<MultiBuffer> for Offset<MultiBuffer> {
    fn to_offset(&self, cx: &MultiBufferSnapshot) -> Offset<MultiBuffer> {
        *self
    }
}
