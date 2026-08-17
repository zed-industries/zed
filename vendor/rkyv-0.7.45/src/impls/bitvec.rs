#[cfg(feature = "bitvec_alloc")]
use crate::vec::{ArchivedVec, VecResolver};
use crate::{
    bitvec::ArchivedBitVec,
    out_field,
    ser::{ScratchSpace, Serializer},
    Archive, Archived, Deserialize, Fallible, Serialize,
};
use core::convert::{TryFrom, TryInto};
use core::{marker::PhantomData, ops::Deref};

use bitvec::{prelude::*, view::BitViewSized};

impl<T: BitStore + Archive, O: BitOrder> ArchivedBitVec<T, O> {
    /// Gets the elements of the archived `BitVec` as a `BitSlice`.
    pub fn as_bitslice(&self) -> &BitSlice<T, O> {
        self.deref()
    }
}

#[cfg(feature = "bitvec_alloc")]
impl<T: BitStore + Archive, O: BitOrder> Archive for BitVec<T, O>
where
    Archived<T>: BitStore,
{
    type Archived = ArchivedBitVec<Archived<T>, O>;
    type Resolver = VecResolver;

    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        let (fp, fo) = out_field!(out.inner);
        ArchivedVec::resolve_from_slice(self.as_raw_slice(), pos + fp, resolver, fo);
        let (fp, fo) = out_field!(out.bit_len);
        usize::resolve(&self.len(), pos + fp, (), fo);
    }
}

#[cfg(feature = "bitvec_alloc")]
impl<T, O, S> Serialize<S> for BitVec<T, O>
where
    T: BitStore + Archive + Serialize<S>,
    O: BitOrder,
    S: Fallible + ?Sized + ScratchSpace + Serializer,
    Archived<T>: BitStore,
{
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, <S as Fallible>::Error> {
        let resolver = ArchivedVec::serialize_from_slice(self.as_raw_slice(), serializer)?;
        usize::serialize(&self.len(), serializer)?;

        Ok(resolver)
    }
}

#[cfg(feature = "bitvec_alloc")]
impl<T, O, D> Deserialize<BitVec<T, O>, D> for ArchivedBitVec<Archived<T>, O>
where
    T: BitStore + Archive,
    O: BitOrder,
    D: Fallible + ?Sized,
    Archived<T>: Deserialize<T, D> + BitStore,
{
    fn deserialize(&self, deserializer: &mut D) -> Result<BitVec<T, O>, <D as Fallible>::Error> {
        let vec = ArchivedVec::deserialize(&self.inner, deserializer)?;
        let bit_len = Archived::<usize>::deserialize(&self.bit_len, deserializer)?;

        let mut bitvec = BitVec::<T, O>::from_vec(vec);
        bitvec.truncate(bit_len);
        Ok(bitvec)
    }
}

#[cfg_attr(feature = "validation", derive(bytecheck::CheckBytes))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ArchivedBitArray<A = [Archived<usize>; 1], O = Lsb0>
where
    A: BitViewSized,
    O: BitOrder,
{
    inner: A,
    _or: PhantomData<O>,
}

impl<A: BitViewSized + Archive, O: BitOrder> ArchivedBitArray<A, O> {
    /// Gets the elements of the archived `BitArray` as a `BitSlice`.
    pub fn as_bitslice(&self) -> &BitSlice<A::Store, O> {
        self.deref()
    }
}

impl<A: BitViewSized + Archive, O: BitOrder> Deref for ArchivedBitArray<A, O> {
    type Target = BitSlice<A::Store, O>;

    fn deref(&self) -> &Self::Target {
        self.inner.view_bits::<O>()
    }
}

impl<A: BitViewSized + Archive, O: BitOrder> Archive for BitArray<A, O>
where
    Archived<A>: BitViewSized,
    for<'a> &'a A: TryFrom<&'a [A::Store]>,
{
    type Archived = ArchivedBitArray<Archived<A>, O>;
    type Resolver = A::Resolver;

    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        let arr_ref = self.as_raw_slice().try_into().ok().unwrap();

        let (fp, fo) = out_field!(out.inner);
        A::resolve(arr_ref, pos + fp, resolver, fo);
    }
}

impl<A, O, S> Serialize<S> for BitArray<A, O>
where
    A: BitViewSized + Archive + Serialize<S>,
    O: BitOrder,
    S: Fallible + ?Sized + ScratchSpace + Serializer,
    Archived<A>: BitViewSized,
    for<'a> &'a A: TryFrom<&'a [A::Store]>,
{
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, <S as Fallible>::Error> {
        let arr_ref = self.as_raw_slice().try_into().ok().unwrap();
        let resolver = A::serialize(arr_ref, serializer)?;

        Ok(resolver)
    }
}

impl<A: BitViewSized + Archive, O: BitOrder, D: Fallible + ?Sized> Deserialize<BitArray<A, O>, D>
    for ArchivedBitArray<Archived<A>, O>
where
    Archived<A>: Deserialize<A, D> + BitViewSized,
{
    fn deserialize(&self, deserializer: &mut D) -> Result<BitArray<A, O>, <D as Fallible>::Error> {
        let arr = Archived::<A>::deserialize(&self.inner, deserializer)?;
        Ok(arr.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        archived_root,
        ser::{serializers::CoreSerializer, Serializer},
        Deserialize, Infallible,
    };
    use bitvec::prelude::*;

    #[test]
    #[cfg(feature = "bitvec_alloc")]
    fn bitvec() {
        use crate::ser::serializers::CoreSerializer;

        let mut serializer = CoreSerializer::<256, 256>::default();
        let original = bitvec![1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1];

        serializer.serialize_value(&original).unwrap();
        let end = serializer.pos();
        let buffer = serializer.into_serializer().into_inner();

        let output = unsafe { archived_root::<BitVec>(&buffer[0..end]) };
        assert_eq!(&original, output.as_bitslice());

        let deserialized: BitVec = output.deserialize(&mut Infallible).unwrap();
        assert_eq!(deserialized, original);
    }

    #[test]
    fn bitarr() {
        let mut serializer = CoreSerializer::<256, 256>::default();
        let original = bitarr![1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1];

        serializer.serialize_value(&original).unwrap();
        let end = serializer.pos();
        let buffer = serializer.into_serializer().into_inner();

        let output = unsafe { archived_root::<BitArray>(&buffer[0..end]) };
        assert_eq!(&original[..11], &output[..11]);

        let deserialized: BitArray = output.deserialize(&mut Infallible).unwrap();
        assert_eq!(&deserialized[..11], &original[..11]);
    }
}
