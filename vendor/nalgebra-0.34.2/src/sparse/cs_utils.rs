use crate::allocator::Allocator;
use crate::{DefaultAllocator, Dim, OVector};

pub fn cumsum<D: Dim>(a: &mut OVector<usize, D>, b: &mut OVector<usize, D>) -> usize
where
    DefaultAllocator: Allocator<D>,
{
    assert!(a.len() == b.len());
    let mut sum = 0;

    for i in 0..a.len() {
        b[i] = sum;
        sum += a[i];
        a[i] = b[i];
    }

    sum
}
