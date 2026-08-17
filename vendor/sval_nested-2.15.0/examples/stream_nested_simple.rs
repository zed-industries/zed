/*!
This example demonstrates implementing a nested `Stream`, similar to the flat example from `sval`.
*/

pub struct Stream;

impl<'sval> sval_nested::Stream<'sval> for Stream {
    type Ok = ();

    type Seq = Self;

    type Map = Self;

    type Tuple = Self;

    type Record = Self;

    type Enum = Self;

    fn null(self) -> sval_nested::Result<Self::Ok> {
        print!("null");
        Ok(())
    }

    fn bool(self, value: bool) -> sval_nested::Result<Self::Ok> {
        print!("{}", value);
        Ok(())
    }

    fn i64(self, value: i64) -> sval_nested::Result<Self::Ok> {
        print!("{}", value);
        Ok(())
    }

    fn f64(self, value: f64) -> sval_nested::Result<Self::Ok> {
        print!("{}", value);
        Ok(())
    }

    fn text_computed(self, text: &str) -> sval_nested::Result<Self::Ok> {
        print!("{:?}", text);
        Ok(())
    }

    fn seq_begin(self, _: Option<usize>) -> sval_nested::Result<Self::Seq> {
        print!("[ ");
        Ok(self)
    }

    fn map_begin(self, num_entries: Option<usize>) -> sval_nested::Result<Self::Map> {
        self.seq_begin(num_entries)
    }

    fn tuple_begin(
        self,
        _: Option<sval::Tag>,
        _: Option<sval::Label>,
        _: Option<sval::Index>,
        num_entries: Option<usize>,
    ) -> sval_nested::Result<Self::Tuple> {
        self.seq_begin(num_entries)
    }

    fn record_begin(
        self,
        _: Option<sval::Tag>,
        _: Option<sval::Label>,
        _: Option<sval::Index>,
        num_entries: Option<usize>,
    ) -> sval_nested::Result<Self::Record> {
        self.map_begin(num_entries)
    }

    fn enum_begin(
        self,
        _: Option<sval::Tag>,
        _: Option<sval::Label>,
        _: Option<sval::Index>,
    ) -> sval_nested::Result<Self::Enum> {
        Ok(self)
    }
}

impl<'sval> sval_nested::StreamSeq<'sval> for Stream {
    type Ok = ();

    fn value_computed<V: sval::Value>(&mut self, value: V) -> sval_nested::Result {
        sval_nested::Stream::value_computed(Stream, value)?;
        print!(", ");

        Ok(())
    }

    fn end(self) -> sval_nested::Result<Self::Ok> {
        print!("]");
        Ok(())
    }
}

impl<'sval> sval_nested::StreamMap<'sval> for Stream {
    type Ok = ();

    fn key_computed<V: sval::Value>(&mut self, key: V) -> sval_nested::Result {
        print!("[ ");
        sval_nested::Stream::value_computed(Stream, key)?;
        print!(", ");

        Ok(())
    }

    fn value_computed<V: sval::Value>(&mut self, value: V) -> sval_nested::Result {
        sval_nested::Stream::value_computed(Stream, value)?;
        print!(", ], ");

        Ok(())
    }

    fn end(self) -> sval_nested::Result<Self::Ok> {
        sval_nested::StreamSeq::end(self)
    }
}

impl<'sval> sval_nested::StreamTuple<'sval> for Stream {
    type Ok = ();

    fn value_computed<V: sval::Value>(
        &mut self,
        _: Option<sval::Tag>,
        _: sval::Index,
        value: V,
    ) -> sval_nested::Result {
        sval_nested::StreamSeq::value_computed(self, value)
    }

    fn end(self) -> sval_nested::Result<Self::Ok> {
        sval_nested::StreamSeq::end(self)
    }
}

impl<'sval> sval_nested::StreamRecord<'sval> for Stream {
    type Ok = ();

    fn value_computed<V: sval::Value>(
        &mut self,
        _: Option<sval::Tag>,
        label: sval::Label,
        value: V,
    ) -> sval_nested::Result {
        sval_nested::StreamMap::key_computed(self, label.as_str())?;
        sval_nested::StreamMap::value_computed(self, value)
    }

    fn end(self) -> sval_nested::Result<Self::Ok> {
        sval_nested::StreamSeq::end(self)
    }
}

impl<'sval> sval_nested::StreamEnum<'sval> for Stream {
    type Ok = ();

    type Tuple = Self;

    type Record = Self;

    type Nested = Self;

    fn tag(
        self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
    ) -> sval_nested::Result<Self::Ok> {
        sval_nested::Stream::tag(self, tag, label, index)
    }

    fn tagged_computed<V: sval::Value>(
        self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
        value: V,
    ) -> sval_nested::Result<Self::Ok> {
        sval_nested::Stream::tagged_computed(self, tag, label, index, value)
    }

    fn tuple_begin(
        self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
        num_entries: Option<usize>,
    ) -> sval_nested::Result<Self::Tuple> {
        sval_nested::Stream::tuple_begin(self, tag, label, index, num_entries)
    }

    fn record_begin(
        self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
        num_entries: Option<usize>,
    ) -> sval_nested::Result<Self::Record> {
        sval_nested::Stream::record_begin(self, tag, label, index, num_entries)
    }

    fn nested<
        F: FnOnce(
            Self::Nested,
        )
            -> sval_nested::Result<<Self::Nested as sval_nested::StreamEnum<'sval>>::Ok>,
    >(
        self,
        _: Option<sval::Tag>,
        _: Option<sval::Label>,
        _: Option<sval::Index>,
        variant: F,
    ) -> sval_nested::Result<Self::Ok> {
        variant(self)
    }

    fn empty(self) -> sval_nested::Result<Self::Ok> {
        sval_nested::Stream::null(self)
    }
}

fn main() -> sval_nested::Result {
    stream(42)?;
    stream(true)?;

    stream(Some(42))?;
    stream(None::<i32>)?;

    stream(sval::MapSlice::new(&[("a", 1), ("b", 2), ("c", 3)]))?;

    stream(&[&["Hello", "world"], &["Hello", "world"]])?;

    Ok(())
}

fn stream(v: impl sval::Value) -> sval_nested::Result {
    sval_nested::stream_computed(Stream, v)?;
    println!();

    Ok(())
}
