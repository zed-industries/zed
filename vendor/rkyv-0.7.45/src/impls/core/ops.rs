use crate::{
    ops::{
        ArchivedRange, ArchivedRangeFrom, ArchivedRangeInclusive, ArchivedRangeTo,
        ArchivedRangeToInclusive,
    },
    Archive, Archived, Deserialize, Fallible, Serialize,
};
use core::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

// RangeFull

impl Archive for RangeFull {
    type Archived = Self;
    type Resolver = ();

    #[inline]
    unsafe fn resolve(&self, _: usize, _: Self::Resolver, _: *mut Self::Archived) {}
}

impl<S: Fallible + ?Sized> Serialize<S> for RangeFull {
    #[inline]
    fn serialize(&self, _: &mut S) -> Result<Self::Resolver, S::Error> {
        Ok(())
    }
}

impl<D: Fallible + ?Sized> Deserialize<RangeFull, D> for RangeFull {
    #[inline]
    fn deserialize(&self, _: &mut D) -> Result<Self, D::Error> {
        Ok(RangeFull)
    }
}

// Range

impl<T: Archive> Archive for Range<T> {
    type Archived = ArchivedRange<T::Archived>;
    type Resolver = Range<T::Resolver>;

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        let (fp, fo) = out_field!(out.start);
        self.start.resolve(pos + fp, resolver.start, fo);
        let (fp, fo) = out_field!(out.end);
        self.end.resolve(pos + fp, resolver.end, fo);
    }
}

impl<T: Serialize<S>, S: Fallible + ?Sized> Serialize<S> for Range<T> {
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        Ok(Range {
            start: self.start.serialize(serializer)?,
            end: self.end.serialize(serializer)?,
        })
    }
}

impl<T: Archive, D: Fallible + ?Sized> Deserialize<Range<T>, D> for Archived<Range<T>>
where
    T::Archived: Deserialize<T, D>,
{
    #[inline]
    fn deserialize(&self, deserializer: &mut D) -> Result<Range<T>, D::Error> {
        Ok(Range {
            start: self.start.deserialize(deserializer)?,
            end: self.end.deserialize(deserializer)?,
        })
    }
}

impl<T, U: PartialEq<T>> PartialEq<Range<T>> for ArchivedRange<U> {
    #[inline]
    fn eq(&self, other: &Range<T>) -> bool {
        self.start.eq(&other.start) && self.end.eq(&other.end)
    }
}

// RangeInclusive

impl<T: Archive> Archive for RangeInclusive<T> {
    type Archived = ArchivedRangeInclusive<T::Archived>;
    type Resolver = Range<T::Resolver>;

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        let (fp, fo) = out_field!(out.start);
        self.start().resolve(pos + fp, resolver.start, fo);
        let (fp, fo) = out_field!(out.end);
        self.end().resolve(pos + fp, resolver.end, fo);
    }
}

impl<T: Serialize<S>, S: Fallible + ?Sized> Serialize<S> for RangeInclusive<T> {
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        Ok(Range {
            start: self.start().serialize(serializer)?,
            end: self.end().serialize(serializer)?,
        })
    }
}

impl<T, D> Deserialize<RangeInclusive<T>, D> for Archived<RangeInclusive<T>>
where
    T: Archive,
    T::Archived: Deserialize<T, D>,
    D: Fallible + ?Sized,
{
    #[inline]
    fn deserialize(&self, deserializer: &mut D) -> Result<RangeInclusive<T>, D::Error> {
        Ok(RangeInclusive::new(
            self.start.deserialize(deserializer)?,
            self.end.deserialize(deserializer)?,
        ))
    }
}

impl<T, U: PartialEq<T>> PartialEq<RangeInclusive<T>> for ArchivedRangeInclusive<U> {
    #[inline]
    fn eq(&self, other: &RangeInclusive<T>) -> bool {
        self.start.eq(other.start()) && self.end.eq(other.end())
    }
}

// RangeFrom

impl<T: Archive> Archive for RangeFrom<T> {
    type Archived = ArchivedRangeFrom<T::Archived>;
    type Resolver = RangeFrom<T::Resolver>;

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        let (fp, fo) = out_field!(out.start);
        self.start.resolve(pos + fp, resolver.start, fo);
    }
}

impl<T: Serialize<S>, S: Fallible + ?Sized> Serialize<S> for RangeFrom<T> {
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        Ok(RangeFrom {
            start: self.start.serialize(serializer)?,
        })
    }
}

impl<T: Archive, D: Fallible + ?Sized> Deserialize<RangeFrom<T>, D> for Archived<RangeFrom<T>>
where
    T::Archived: Deserialize<T, D>,
{
    #[inline]
    fn deserialize(&self, deserializer: &mut D) -> Result<RangeFrom<T>, D::Error> {
        Ok(RangeFrom {
            start: self.start.deserialize(deserializer)?,
        })
    }
}

impl<T, U: PartialEq<T>> PartialEq<RangeFrom<T>> for ArchivedRangeFrom<U> {
    #[inline]
    fn eq(&self, other: &RangeFrom<T>) -> bool {
        self.start.eq(&other.start)
    }
}

// RangeTo

impl<T: Archive> Archive for RangeTo<T> {
    type Archived = ArchivedRangeTo<T::Archived>;
    type Resolver = RangeTo<T::Resolver>;

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        let (fp, fo) = out_field!(out.end);
        self.end.resolve(pos + fp, resolver.end, fo);
    }
}

impl<T: Serialize<S>, S: Fallible + ?Sized> Serialize<S> for RangeTo<T> {
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        Ok(RangeTo {
            end: self.end.serialize(serializer)?,
        })
    }
}

impl<T: Archive, D: Fallible + ?Sized> Deserialize<RangeTo<T>, D> for Archived<RangeTo<T>>
where
    T::Archived: Deserialize<T, D>,
{
    #[inline]
    fn deserialize(&self, deserializer: &mut D) -> Result<RangeTo<T>, D::Error> {
        Ok(RangeTo {
            end: self.end.deserialize(deserializer)?,
        })
    }
}

impl<T, U: PartialEq<T>> PartialEq<RangeTo<T>> for ArchivedRangeTo<U> {
    #[inline]
    fn eq(&self, other: &RangeTo<T>) -> bool {
        self.end.eq(&other.end)
    }
}

// RangeToInclusive

impl<T: Archive> Archive for RangeToInclusive<T> {
    type Archived = ArchivedRangeToInclusive<T::Archived>;
    type Resolver = RangeToInclusive<T::Resolver>;

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        let (fp, fo) = out_field!(out.end);
        self.end.resolve(pos + fp, resolver.end, fo);
    }
}

impl<T: Serialize<S>, S: Fallible + ?Sized> Serialize<S> for RangeToInclusive<T> {
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        Ok(RangeToInclusive {
            end: self.end.serialize(serializer)?,
        })
    }
}

impl<T, D> Deserialize<RangeToInclusive<T>, D> for Archived<RangeToInclusive<T>>
where
    T: Archive,
    T::Archived: Deserialize<T, D>,
    D: Fallible + ?Sized,
{
    #[inline]
    fn deserialize(&self, deserializer: &mut D) -> Result<RangeToInclusive<T>, D::Error> {
        Ok(RangeToInclusive {
            end: self.end.deserialize(deserializer)?,
        })
    }
}

impl<T, U: PartialEq<T>> PartialEq<RangeToInclusive<T>> for ArchivedRangeToInclusive<U> {
    #[inline]
    fn eq(&self, other: &RangeToInclusive<T>) -> bool {
        self.end.eq(&other.end)
    }
}
