use crate::{default_stream, Error, Result, Stream, StreamEnum, Unsupported};

use super::owned_label;

pub(super) struct FlatStreamEnum<S> {
    stream: S,
    queue: Queue,
}

#[derive(Debug)]
struct NestedVariant {
    tag: Option<sval::Tag>,
    label: Option<sval::Label<'static>>,
    index: Option<sval::Index>,
}

impl<'sval, S: StreamEnum<'sval>> FlatStreamEnum<S> {
    pub fn new(stream: S) -> Self {
        FlatStreamEnum {
            stream,
            queue: Default::default(),
        }
    }

    pub fn push(
        &mut self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
    ) -> Result {
        self.queue.push_back(NestedVariant {
            tag,
            label: if let Some(label) = label {
                Some(owned_label(label)?)
            } else {
                None
            },
            index,
        })
    }

    pub fn end(self) -> Result<S::Ok> {
        self.value_or_recurse(|stream, _| stream.end(), |stream, _| stream.end(), ())
    }

    fn value_or_recurse<I>(
        mut self,
        value: impl FnOnce(Self, I) -> Result<S::Ok>,
        nested: impl FnOnce(
            FlatStreamEnum<S::Nested>,
            I,
        ) -> Result<<S::Nested as StreamEnum<'sval>>::Ok>,
        input: I,
    ) -> Result<S::Ok> {
        if let Some(variant) = self.queue.pop_front() {
            self.stream
                .nested(variant.tag, variant.label, variant.index, |variant| {
                    nested(
                        FlatStreamEnum {
                            stream: variant,
                            queue: self.queue,
                        },
                        input,
                    )
                })
        } else {
            value(self, input)
        }
    }
}

impl<'sval, S: StreamEnum<'sval>> Stream<'sval> for FlatStreamEnum<S> {
    type Ok = S::Ok;

    type Seq = Unsupported<S::Ok>;
    type Map = Unsupported<S::Ok>;

    type Tuple = S::Tuple;
    type Record = S::Record;

    type Enum = Unsupported<S::Ok>;

    fn value<V: sval_ref::ValueRef<'sval>>(self, value: V) -> Result<Self::Ok> {
        self.value_or_recurse(
            |stream, value| default_stream::value(stream, value),
            |stream, value| stream.value(value),
            value,
        )
    }

    fn value_computed<V: sval::Value>(self, value: V) -> Result<Self::Ok> {
        self.value_or_recurse(
            |stream, value| default_stream::value_computed(stream, value),
            |stream, value| stream.value_computed(value),
            value,
        )
    }

    fn null(self) -> Result<Self::Ok> {
        Err(Error::invalid_value(
            "enum variants must be wrapped in a tag-carrying value",
        ))
    }

    fn bool(self, _: bool) -> Result<Self::Ok> {
        Err(Error::invalid_value(
            "enum variants must be wrapped in a tag-carrying value",
        ))
    }

    fn i64(self, _: i64) -> Result<Self::Ok> {
        Err(Error::invalid_value(
            "enum variants must be wrapped in a tag-carrying value",
        ))
    }

    fn f64(self, _: f64) -> Result<Self::Ok> {
        Err(Error::invalid_value(
            "enum variants must be wrapped in a tag-carrying value",
        ))
    }

    fn text_computed(self, _: &str) -> Result<Self::Ok> {
        Err(Error::invalid_value(
            "enum variants must be wrapped in a tag-carrying value",
        ))
    }

    fn tag(
        self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
    ) -> Result<Self::Ok> {
        self.value_or_recurse(
            |stream, (tag, label, index)| stream.stream.tag(tag, label, index),
            |stream, (tag, label, index)| Stream::tag(stream, tag, label, index),
            (tag, label, index),
        )
    }

    fn tagged<V: sval_ref::ValueRef<'sval>>(
        self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
        value: V,
    ) -> Result<Self::Ok> {
        self.value_or_recurse(
            |stream, (value, tag, label, index)| stream.stream.tagged(tag, label, index, value),
            |stream, (value, tag, label, index)| Stream::tagged(stream, tag, label, index, value),
            (value, tag, label, index),
        )
    }

    fn tagged_computed<V: sval::Value>(
        self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
        value: V,
    ) -> Result<Self::Ok> {
        self.value_or_recurse(
            |stream, (value, tag, label, index)| {
                stream.stream.tagged_computed(tag, label, index, value)
            },
            |stream, (value, tag, label, index)| {
                Stream::tagged_computed(stream, tag, label, index, value)
            },
            (value, tag, label, index),
        )
    }

    fn seq_begin(self, _: Option<usize>) -> Result<Self::Seq> {
        Err(Error::invalid_value("sequences are unsupported"))
    }

    fn map_begin(self, _: Option<usize>) -> Result<Self::Map> {
        Err(Error::invalid_value("maps are unsupported"))
    }

    fn tuple_begin(
        self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
        num_entries: Option<usize>,
    ) -> Result<Self::Tuple> {
        assert!(self.queue.is_empty());

        self.stream.tuple_begin(tag, label, index, num_entries)
    }

    fn record_begin(
        self,
        tag: Option<sval::Tag>,
        label: Option<sval::Label>,
        index: Option<sval::Index>,
        num_entries: Option<usize>,
    ) -> Result<Self::Record> {
        assert!(self.queue.is_empty());

        self.stream.record_begin(tag, label, index, num_entries)
    }

    fn enum_begin(
        self,
        _: Option<sval::Tag>,
        _: Option<sval::Label>,
        _: Option<sval::Index>,
    ) -> Result<Self::Enum> {
        unreachable!()
    }
}

#[derive(Default)]
struct Queue {
    #[cfg(feature = "alloc")]
    inner: alloc::collections::VecDeque<NestedVariant>,
}

impl Queue {
    fn push_back(&mut self, variant: NestedVariant) -> Result {
        #[cfg(feature = "alloc")]
        {
            self.inner.push_back(variant);
            Ok(())
        }
        #[cfg(not(feature = "alloc"))]
        {
            let _ = variant;
            Err(Error::no_alloc("nested enum variant"))
        }
    }

    fn pop_front(&mut self) -> Option<NestedVariant> {
        #[cfg(feature = "alloc")]
        {
            self.inner.pop_front()
        }
        #[cfg(not(feature = "alloc"))]
        {
            None
        }
    }

    fn is_empty(&self) -> bool {
        #[cfg(feature = "alloc")]
        {
            self.inner.is_empty()
        }
        #[cfg(not(feature = "alloc"))]
        {
            true
        }
    }
}
