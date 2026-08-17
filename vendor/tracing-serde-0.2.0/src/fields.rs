//! Support for serializing fields as `serde` structs or maps.
use super::*;

#[derive(Debug)]
pub struct SerializeFieldMap<'a, T>(&'a T);

pub trait AsMap: Sized + sealed::Sealed {
    fn field_map(&self) -> SerializeFieldMap<'_, Self> {
        SerializeFieldMap(self)
    }
}

impl<'a> AsMap for Event<'a> {}
impl<'a> AsMap for Attributes<'a> {}
impl<'a> AsMap for Record<'a> {}

// === impl SerializeFieldMap ===

impl<'a> Serialize for SerializeFieldMap<'a, Event<'_>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = self.0.fields().count();
        let serializer = serializer.serialize_map(Some(len))?;
        let mut visitor = SerdeMapVisitor::new(serializer);
        self.0.record(&mut visitor);
        visitor.finish()
    }
}

impl<'a> Serialize for SerializeFieldMap<'a, Attributes<'_>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = self.0.metadata().fields().len();
        let serializer = serializer.serialize_map(Some(len))?;
        let mut visitor = SerdeMapVisitor::new(serializer);
        self.0.record(&mut visitor);
        visitor.finish()
    }
}

impl<'a> Serialize for SerializeFieldMap<'a, Record<'_>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let serializer = serializer.serialize_map(None)?;
        let mut visitor = SerdeMapVisitor::new(serializer);
        self.0.record(&mut visitor);
        visitor.finish()
    }
}
