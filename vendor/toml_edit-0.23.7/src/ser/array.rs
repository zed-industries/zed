use super::Error;

#[doc(hidden)]
pub struct SerializeValueArray {
    values: Vec<crate::Item>,
}

impl SerializeValueArray {
    pub(crate) fn seq(len: Option<usize>) -> Self {
        let mut values = Vec::new();
        if let Some(len) = len {
            values.reserve(len);
        }
        Self { values }
    }
}

impl serde_core::ser::SerializeSeq for SerializeValueArray {
    type Ok = crate::Value;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: serde_core::ser::Serialize + ?Sized,
    {
        let value = value.serialize(super::ValueSerializer {})?;
        self.values.push(crate::Item::Value(value));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(crate::Value::Array(crate::Array::with_vec(self.values)))
    }
}

impl serde_core::ser::SerializeTuple for SerializeValueArray {
    type Ok = crate::Value;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: serde_core::ser::Serialize + ?Sized,
    {
        serde_core::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde_core::ser::SerializeSeq::end(self)
    }
}

impl serde_core::ser::SerializeTupleStruct for SerializeValueArray {
    type Ok = crate::Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: serde_core::ser::Serialize + ?Sized,
    {
        serde_core::ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde_core::ser::SerializeSeq::end(self)
    }
}

pub struct SerializeTupleVariant {
    variant: &'static str,
    inner: SerializeValueArray,
}

impl SerializeTupleVariant {
    pub(crate) fn tuple(variant: &'static str, len: usize) -> Self {
        Self {
            variant,
            inner: SerializeValueArray::seq(Some(len)),
        }
    }
}

impl serde_core::ser::SerializeTupleVariant for SerializeTupleVariant {
    type Ok = crate::Value;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: serde_core::ser::Serialize + ?Sized,
    {
        serde_core::ser::SerializeSeq::serialize_element(&mut self.inner, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        let inner = serde_core::ser::SerializeSeq::end(self.inner)?;
        let mut items = crate::table::KeyValuePairs::new();
        let value = crate::Item::Value(inner);
        items.insert(crate::Key::new(self.variant), value);
        Ok(crate::Value::InlineTable(crate::InlineTable::with_pairs(
            items,
        )))
    }
}
