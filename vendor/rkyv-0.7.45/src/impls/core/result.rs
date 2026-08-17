use crate::{result::ArchivedResult, Archive, Deserialize, Fallible, Serialize};
use core::{hint::unreachable_unchecked, ptr};

#[allow(dead_code)]
#[repr(u8)]
enum ArchivedResultTag {
    Ok,
    Err,
}

#[repr(C)]
struct ArchivedResultVariantOk<T>(ArchivedResultTag, T);

#[repr(C)]
struct ArchivedResultVariantErr<E>(ArchivedResultTag, E);

impl<T: Archive, E: Archive> Archive for Result<T, E> {
    type Archived = ArchivedResult<T::Archived, E::Archived>;
    type Resolver = Result<T::Resolver, E::Resolver>;

    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        match resolver {
            Ok(resolver) => {
                let out = out.cast::<ArchivedResultVariantOk<T::Archived>>();
                ptr::addr_of_mut!((*out).0).write(ArchivedResultTag::Ok);

                let (fp, fo) = out_field!(out.1);
                match self.as_ref() {
                    Ok(value) => value.resolve(pos + fp, resolver, fo),
                    Err(_) => unreachable_unchecked(),
                }
            }
            Err(resolver) => {
                let out = out.cast::<ArchivedResultVariantErr<E::Archived>>();
                ptr::addr_of_mut!((*out).0).write(ArchivedResultTag::Err);

                let (fp, fo) = out_field!(out.1);
                match self.as_ref() {
                    Ok(_) => unreachable_unchecked(),
                    Err(err) => err.resolve(pos + fp, resolver, fo),
                }
            }
        }
    }
}

impl<T: Serialize<S>, E: Serialize<S>, S: Fallible + ?Sized> Serialize<S> for Result<T, E> {
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        Ok(match self.as_ref() {
            Ok(value) => Ok(value.serialize(serializer)?),
            Err(value) => Err(value.serialize(serializer)?),
        })
    }
}

impl<T, E, D> Deserialize<Result<T, E>, D> for ArchivedResult<T::Archived, E::Archived>
where
    T: Archive,
    E: Archive,
    D: Fallible + ?Sized,
    T::Archived: Deserialize<T, D>,
    E::Archived: Deserialize<E, D>,
{
    #[inline]
    fn deserialize(&self, deserializer: &mut D) -> Result<Result<T, E>, D::Error> {
        match self {
            ArchivedResult::Ok(value) => Ok(Ok(value.deserialize(deserializer)?)),
            ArchivedResult::Err(err) => Ok(Err(err.deserialize(deserializer)?)),
        }
    }
}
