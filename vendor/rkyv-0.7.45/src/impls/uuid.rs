use crate::{Archive, Deserialize, Fallible, Serialize};
use uuid::Uuid;

impl Archive for Uuid {
    type Archived = Uuid;
    type Resolver = ();

    unsafe fn resolve(&self, _: usize, _: Self::Resolver, out: *mut Self::Archived) {
        // Safety: Uuid is portable and has no padding
        out.write(*self);
    }
}

// Safety: Uuid is portable and has no padding
#[cfg(feature = "copy")]
unsafe impl crate::copy::ArchiveCopySafe for Uuid {}

impl<S: Fallible + ?Sized> Serialize<S> for Uuid {
    fn serialize(&self, _: &mut S) -> Result<Self::Resolver, S::Error> {
        Ok(())
    }
}

impl<D: Fallible + ?Sized> Deserialize<Uuid, D> for Uuid {
    fn deserialize(&self, _: &mut D) -> Result<Uuid, D::Error> {
        Ok(*self)
    }
}

#[cfg(test)]
mod rkyv_tests {
    use crate::{
        archived_root,
        ser::{serializers::AlignedSerializer, Serializer},
        util::AlignedVec,
        Deserialize, Infallible,
    };
    use uuid::Uuid;

    #[test]
    fn test_serialize_deserialize() {
        let uuid_str = "f9168c5e-ceb2-4faa-b6bf-329bf39fa1e4";
        let u = Uuid::parse_str(uuid_str).unwrap();

        let mut serializer = AlignedSerializer::new(AlignedVec::new());
        serializer
            .serialize_value(&u)
            .expect("failed to archive uuid");
        let buf = serializer.into_inner();
        let archived = unsafe { archived_root::<Uuid>(buf.as_ref()) };

        assert_eq!(&u, archived);

        let deserialized = archived
            .deserialize(&mut Infallible)
            .expect("failed to deserialize uuid");

        assert_eq!(u, deserialized);
    }
}
