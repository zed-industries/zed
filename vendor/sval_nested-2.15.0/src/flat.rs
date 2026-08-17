use core::{fmt, marker::PhantomData, mem};

use sval_buffer::{BinaryBuf, TextBuf, ValueBuf};

use crate::{Error, Result, Stream, StreamEnum, StreamMap, StreamRecord, StreamSeq, StreamTuple};

use super::{flat_enum::FlatStreamEnum, owned_label_ref};

pub(super) struct FlatStream<'sval, S: Stream<'sval>> {
    buffered: Option<Buffered<'sval>>,
    state: State<'sval, S>,
}

impl<'sval, S: Stream<'sval>> fmt::Debug for FlatStream<'sval, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FlatStream")
            .field("buffered", &self.buffered)
            .field("state", &self.state)
            .finish()
    }
}

enum State<'sval, S: Stream<'sval>> {
    Any(Option<Any<'sval, S>>),
    Seq(Option<Seq<S::Seq>>),
    Map(Option<Map<S::Map>>),
    Tagged(Option<Tagged<S>>),
    Tuple(Option<Tuple<S::Tuple>>),
    Record(Option<Record<S::Record>>),
    Enum(Option<Enum<FlatStreamEnum<S::Enum>>>),
    EnumVariant(Option<EnumVariant<'sval, S>>),
    Done(Option<Result<S::Ok>>),
}

impl<'sval, S: Stream<'sval>> fmt::Debug for State<'sval, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Any(state) => fmt::Debug::fmt(state, f),
            State::Seq(state) => fmt::Debug::fmt(state, f),
            State::Map(state) => fmt::Debug::fmt(state, f),
            State::Tuple(state) => fmt::Debug::fmt(state, f),
            State::Record(state) => fmt::Debug::fmt(state, f),
            State::Tagged(state) => fmt::Debug::fmt(state, f),
            State::Enum(state) => fmt::Debug::fmt(state, f),
            State::EnumVariant(state) => fmt::Debug::fmt(state, f),
            State::Done(_) => f.debug_struct("Done").finish_non_exhaustive(),
        }
    }
}

#[derive(Debug)]
enum Buffered<'sval> {
    Text(TextBuf<'sval>),
    Binary(BinaryBuf<'sval>),
    Value(ValueBuf<'sval>),
}

struct Any<'sval, S: Stream<'sval>> {
    stream: S,
    _marker: PhantomData<&'sval ()>,
}

impl<'sval, S: Stream<'sval>> fmt::Debug for Any<'sval, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Any").finish_non_exhaustive()
    }
}

struct Seq<S> {
    stream: S,
}

impl<S> fmt::Debug for Seq<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Seq").finish_non_exhaustive()
    }
}

struct Map<S> {
    stream: S,
    is_key: bool,
}

impl<S> fmt::Debug for Map<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Map")
            .field("is_key", &self.is_key)
            .finish_non_exhaustive()
    }
}

struct Record<S> {
    stream: S,
    field: Option<(Option<sval::Tag>, sval::Label<'static>)>,
}

impl<S> fmt::Debug for Record<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Record").finish_non_exhaustive()
    }
}

struct Tuple<S> {
    stream: S,
    field: Option<(Option<sval::Tag>, sval::Index)>,
}

impl<S> fmt::Debug for Tuple<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tuple").finish_non_exhaustive()
    }
}

struct Tagged<S> {
    stream: S,
    tag: Option<sval::Tag>,
    label: Option<sval::Label<'static>>,
    index: Option<sval::Index>,
}

impl<S> fmt::Debug for Tagged<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Tagged")
            .field("tag", &self.tag)
            .field("label", &self.label)
            .field("index", &self.index)
            .finish_non_exhaustive()
    }
}

struct Enum<S> {
    stream: S,
}

impl<S> fmt::Debug for Enum<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Enum").finish_non_exhaustive()
    }
}

enum EnumVariant<'sval, S: Stream<'sval>> {
    Tagged(Tagged<FlatStreamEnum<S::Enum>>),
    Tuple(Tuple<<S::Enum as StreamEnum<'sval>>::Tuple>),
    Record(Record<<S::Enum as StreamEnum<'sval>>::Record>),
}

impl<'sval, S: Stream<'sval>> fmt::Debug for EnumVariant<'sval, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnumVariant::Tagged(stream) => fmt::Debug::fmt(stream, f),
            EnumVariant::Tuple(stream) => fmt::Debug::fmt(stream, f),
            EnumVariant::Record(stream) => fmt::Debug::fmt(stream, f),
        }
    }
}

impl<'sval, S: Stream<'sval>> FlatStream<'sval, S> {
    pub fn new(stream: S) -> Self {
        FlatStream {
            buffered: None,
            state: State::Any(Some(Any {
                stream,
                _marker: PhantomData,
            })),
        }
    }

    pub fn finish(&mut self) -> Result<S::Ok> {
        if let State::Done(ref mut r) = self.state {
            r.take()
                .unwrap_or_else(|| Err(Error::invalid_value("incomplete stream")))
        } else {
            Err(Error::invalid_value("incomplete stream"))
        }
    }
}

impl<'sval, S: Stream<'sval>> sval::Stream<'sval> for FlatStream<'sval, S> {
    fn value<V: sval::Value + ?Sized>(&mut self, v: &'sval V) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| sval::Stream::value(buf, v),
            |stream| match stream.state {
                State::Enum(_) => {
                    sval::default_stream::value(stream, v)
                        .map_err(|_| Error::invalid_value("failed to stream value"))?;
                    Ok(None)
                }
                ref mut state => state.value(&sval_ref::to_ref(v), |stream, v| stream.value(v)),
            },
        )
    }

    fn value_computed<V: sval::Value + ?Sized>(&mut self, v: &V) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| sval::Stream::value_computed(buf, v),
            |stream| match stream.state {
                State::Enum(_) => {
                    sval::default_stream::value_computed(stream, v)
                        .map_err(|_| Error::invalid_value("failed to stream value"))?;
                    Ok(None)
                }
                ref mut state => state.value_computed(v, |stream, v| stream.value_computed(v)),
            },
        )
    }

    fn tag(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.tag(tag, label, index),
            |stream| match stream.state {
                State::Enum(ref mut stream) => {
                    let stream = stream.take().ok_or_else(|| {
                        Error::invalid_value(
                            "failed to stream an enum; the stream is already completed",
                        )
                    })?;

                    Ok(Some(stream.stream.tag(
                        tag.cloned(),
                        label.cloned(),
                        index.cloned(),
                    )?))
                }
                ref mut state => state.value_computed(&Tag(tag, label, index), |stream, _| {
                    stream.tag(tag.cloned(), label.cloned(), index.cloned())
                }),
            },
        )
    }

    fn seq_begin(&mut self, num_entries: Option<usize>) -> sval::Result {
        self.buffer_or_begin_with(
            |buf| buf.seq_begin(num_entries),
            |stream| {
                Ok(State::Seq(Some(Seq {
                    stream: stream.stream.seq_begin(num_entries)?,
                })))
            },
            |_| {
                Err(Error::invalid_value(
                    "sequences cannot be used as enum variants",
                ))
            },
        )
    }

    fn seq_value_begin(&mut self) -> sval::Result {
        Ok(())
    }

    fn seq_value_end(&mut self) -> sval::Result {
        Ok(())
    }

    fn seq_end(&mut self) -> sval::Result {
        self.buffer_or_end_with(
            |buf| buf.seq_end(),
            |stream| stream.take_seq()?.stream.end(),
        )
    }

    fn map_begin(&mut self, num_entries: Option<usize>) -> sval::Result {
        self.buffer_or_begin_with(
            |buf| buf.map_begin(num_entries),
            |stream| {
                Ok(State::Map(Some(Map {
                    stream: stream.stream.map_begin(num_entries)?,
                    is_key: true,
                })))
            },
            |_| Err(Error::invalid_value("maps cannot be used as enum variants")),
        )
    }

    fn map_key_begin(&mut self) -> sval::Result {
        self.buffer_or_with(
            |buf| buf.map_key_begin(),
            |stream| {
                stream.with_map(|stream| {
                    stream.is_key = true;

                    Ok(())
                })
            },
        )
    }

    fn map_key_end(&mut self) -> sval::Result {
        self.buffer_or_with(|buf| buf.map_key_end(), |_| Ok(()))
    }

    fn map_value_begin(&mut self) -> sval::Result {
        self.buffer_or_with(
            |buf| buf.map_value_begin(),
            |stream| {
                stream.with_map(|stream| {
                    stream.is_key = false;

                    Ok(())
                })
            },
        )
    }

    fn map_value_end(&mut self) -> sval::Result {
        self.buffer_or_with(|buf| buf.map_value_end(), |_| Ok(()))
    }

    fn map_end(&mut self) -> sval::Result {
        self.buffer_or_end_with(
            |buf| buf.map_end(),
            |stream| stream.take_map()?.stream.end(),
        )
    }

    fn enum_begin(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        self.buffer_or_begin_with(
            |buf| buf.enum_begin(tag, label, index),
            |stream| {
                Ok(State::Enum(Some(Enum {
                    stream: FlatStreamEnum::new(stream.stream.enum_begin(
                        tag.cloned(),
                        label.cloned(),
                        index.cloned(),
                    )?),
                })))
            },
            |mut stream| {
                stream
                    .stream
                    .push(tag.cloned(), label.cloned(), index.cloned())?;

                Ok(State::Enum(Some(stream)))
            },
        )
    }

    fn enum_end(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        self.buffer_or_end_with(
            |buf| buf.enum_end(tag, label, index),
            |stream| {
                if let Some(stream) = stream.take_enum()? {
                    stream.stream.end()
                } else {
                    stream.finish()
                }
            },
        )
    }

    fn tagged_begin(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        self.buffer_or_begin_with(
            |buf| buf.tagged_begin(tag, label, index),
            |stream| {
                Ok(State::Tagged(Some(Tagged {
                    stream: stream.stream,
                    tag: tag.cloned(),
                    label: if let Some(label) = label {
                        Some(owned_label_ref(label)?)
                    } else {
                        None
                    },
                    index: index.cloned(),
                })))
            },
            |stream| {
                Ok(State::EnumVariant(Some(EnumVariant::Tagged(Tagged {
                    stream: stream.stream,
                    tag: tag.cloned(),
                    label: if let Some(label) = label {
                        Some(owned_label_ref(label)?)
                    } else {
                        None
                    },
                    index: index.cloned(),
                }))))
            },
        )
    }

    fn tagged_end(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        self.buffer_or_end_with(
            |buf| buf.tagged_end(tag, label, index),
            |stream| stream.finish(),
        )
    }

    fn record_begin(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
        num_entries: Option<usize>,
    ) -> sval::Result {
        self.buffer_or_begin_with(
            |buf| buf.record_begin(tag, label, index, num_entries),
            |stream| {
                Ok(State::Record(Some(Record {
                    stream: stream.stream.record_begin(
                        tag.cloned(),
                        label.cloned(),
                        index.cloned(),
                        num_entries,
                    )?,
                    field: None,
                })))
            },
            |stream| {
                Ok(State::EnumVariant(Some(EnumVariant::Record(Record {
                    stream: stream.stream.record_begin(
                        tag.cloned(),
                        label.cloned(),
                        index.cloned(),
                        num_entries,
                    )?,
                    field: None,
                }))))
            },
        )
    }

    fn record_value_begin(&mut self, tag: Option<&sval::Tag>, label: &sval::Label) -> sval::Result {
        self.buffer_or_with(
            |buf| buf.record_value_begin(tag, label),
            |stream| {
                stream.with_record(
                    |record| {
                        record.field = Some((tag.cloned(), owned_label_ref(label)?));

                        Ok(())
                    },
                    |record_variant| {
                        record_variant.field = Some((tag.cloned(), owned_label_ref(label)?));

                        Ok(())
                    },
                )
            },
        )
    }

    fn record_value_end(&mut self, tag: Option<&sval::Tag>, label: &sval::Label) -> sval::Result {
        self.buffer_or_with(|buf| buf.record_value_end(tag, label), |_| Ok(()))
    }

    fn record_end(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        self.buffer_or_end_with(
            |buf| buf.record_end(tag, label, index),
            |stream| {
                stream.take_with_record(
                    |record| record.stream.end(),
                    |record_variant| record_variant.stream.end(),
                )
            },
        )
    }

    fn tuple_begin(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
        num_entries: Option<usize>,
    ) -> sval::Result {
        self.buffer_or_begin_with(
            |buf| buf.tuple_begin(tag, label, index, num_entries),
            |stream| {
                Ok(State::Tuple(Some(Tuple {
                    stream: stream.stream.tuple_begin(
                        tag.cloned(),
                        label.cloned(),
                        index.cloned(),
                        num_entries,
                    )?,
                    field: None,
                })))
            },
            |stream| {
                Ok(State::EnumVariant(Some(EnumVariant::Tuple(Tuple {
                    stream: stream.stream.tuple_begin(
                        tag.cloned(),
                        label.cloned(),
                        index.cloned(),
                        num_entries,
                    )?,
                    field: None,
                }))))
            },
        )
    }

    fn tuple_value_begin(&mut self, tag: Option<&sval::Tag>, index: &sval::Index) -> sval::Result {
        self.buffer_or_with(
            |buf| buf.tuple_value_begin(tag, index),
            |stream| {
                stream.with_tuple(
                    |tuple| {
                        tuple.field = Some((tag.cloned(), index.clone()));

                        Ok(())
                    },
                    |tuple_variant| {
                        tuple_variant.field = Some((tag.cloned(), index.clone()));

                        Ok(())
                    },
                )
            },
        )
    }

    fn tuple_value_end(&mut self, tag: Option<&sval::Tag>, index: &sval::Index) -> sval::Result {
        self.buffer_or_with(|buf| buf.tuple_value_end(tag, index), |_| Ok(()))
    }

    fn tuple_end(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        self.buffer_or_end_with(
            |buf| buf.tuple_end(tag, label, index),
            |stream| {
                stream.take_with_tuple(
                    |tuple| tuple.stream.end(),
                    |tuple_variant| tuple_variant.stream.end(),
                )
            },
        )
    }

    fn null(&mut self) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.null(),
            |stream| {
                stream
                    .state
                    .value(&sval_ref::to_ref(&sval::Null), |stream, _| stream.null())
            },
        )
    }

    fn bool(&mut self, value: bool) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.bool(value),
            |stream| {
                stream
                    .state
                    .value_computed(&value, |stream, value| stream.bool(*value))
            },
        )
    }

    fn u8(&mut self, value: u8) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.u8(value),
            |stream| {
                stream
                    .state
                    .value_computed(&value, |stream, value| stream.u8(*value))
            },
        )
    }

    fn u16(&mut self, value: u16) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.u16(value),
            |stream| {
                stream
                    .state
                    .value_computed(&value, |stream, value| stream.u16(*value))
            },
        )
    }

    fn u32(&mut self, value: u32) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.u32(value),
            |stream| {
                stream
                    .state
                    .value_computed(&value, |stream, value| stream.u32(*value))
            },
        )
    }

    fn u64(&mut self, value: u64) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.u64(value),
            |stream| {
                stream
                    .state
                    .value_computed(&value, |stream, value| stream.u64(*value))
            },
        )
    }

    fn u128(&mut self, value: u128) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.u128(value),
            |stream| {
                stream
                    .state
                    .value_computed(&value, |stream, value| stream.u128(*value))
            },
        )
    }

    fn i8(&mut self, value: i8) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.i8(value),
            |stream| {
                stream
                    .state
                    .value_computed(&value, |stream, value| stream.i8(*value))
            },
        )
    }

    fn i16(&mut self, value: i16) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.i16(value),
            |stream| {
                stream
                    .state
                    .value_computed(&value, |stream, value| stream.i16(*value))
            },
        )
    }

    fn i32(&mut self, value: i32) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.i32(value),
            |stream| {
                stream
                    .state
                    .value_computed(&value, |stream, value| stream.i32(*value))
            },
        )
    }

    fn i64(&mut self, value: i64) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.i64(value),
            |stream| {
                stream
                    .state
                    .value_computed(&value, |stream, value| stream.i64(*value))
            },
        )
    }

    fn i128(&mut self, value: i128) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.i128(value),
            |stream| {
                stream
                    .state
                    .value_computed(&value, |stream, value| stream.i128(*value))
            },
        )
    }

    fn f32(&mut self, value: f32) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.f32(value),
            |stream| {
                stream
                    .state
                    .value_computed(&value, |stream, value| stream.f32(*value))
            },
        )
    }

    fn f64(&mut self, value: f64) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.f64(value),
            |stream| {
                stream
                    .state
                    .value_computed(&value, |stream, value| stream.f64(*value))
            },
        )
    }

    fn text_begin(&mut self, size_hint: Option<usize>) -> sval::Result {
        self.buffer_or_with(
            |buf| buf.text_begin(size_hint),
            |stream| stream.put_buffer(Buffered::Text(TextBuf::new())),
        )
    }

    fn text_fragment(&mut self, fragment: &'sval str) -> sval::Result {
        self.buffer_or_with(
            |buf| buf.text_fragment(fragment),
            |stream| stream.with_text(|text| text.push_fragment(fragment).map_err(Error::buffer)),
        )
    }

    fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        self.buffer_or_with(
            |buf| buf.text_fragment_computed(fragment),
            |stream| {
                stream
                    .with_text(|text| text.push_fragment_computed(fragment).map_err(Error::buffer))
            },
        )
    }

    fn text_end(&mut self) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.text_end(),
            |stream| {
                let buf = stream.take_text()?;

                if let Some(text) = buf.as_borrowed_str() {
                    stream.state.value(&sval_ref::to_ref(text), |stream, text| {
                        stream.text(text.into_inner())
                    })
                } else {
                    stream
                        .state
                        .value_computed(buf.as_str(), |stream, text| stream.text_computed(text))
                }
            },
        )
    }

    fn binary_begin(&mut self, size_hint: Option<usize>) -> sval::Result {
        self.buffer_or_with(
            |buf| buf.binary_begin(size_hint),
            |stream| stream.put_buffer(Buffered::Binary(BinaryBuf::new())),
        )
    }

    fn binary_fragment(&mut self, fragment: &'sval [u8]) -> sval::Result {
        self.buffer_or_with(
            |buf| buf.binary_fragment(fragment),
            |stream| {
                stream.with_binary(|binary| binary.push_fragment(fragment).map_err(Error::buffer))
            },
        )
    }

    fn binary_fragment_computed(&mut self, fragment: &[u8]) -> sval::Result {
        self.buffer_or_with(
            |buf| buf.binary_fragment_computed(fragment),
            |stream| {
                stream.with_binary(|binary| {
                    binary
                        .push_fragment_computed(fragment)
                        .map_err(Error::buffer)
                })
            },
        )
    }

    fn binary_end(&mut self) -> sval::Result {
        self.buffer_or_stream_with(
            |buf| buf.binary_end(),
            |stream| {
                let buf = stream.take_binary()?;

                if let Some(binary) = buf.as_borrowed_slice() {
                    stream.state.value(
                        &sval_ref::to_ref(sval::BinarySlice::new(binary)),
                        |stream, binary| stream.binary(binary.into_inner().as_slice()),
                    )
                } else {
                    stream
                        .state
                        .value_computed(sval::BinarySlice::new(buf.as_slice()), |stream, binary| {
                            stream.binary_computed(binary.as_slice())
                        })
                }
            },
        )
    }
}

fn try_catch<'sval, T, S: Stream<'sval>>(
    stream: &mut FlatStream<'sval, S>,
    f: impl FnOnce(&mut FlatStream<'sval, S>) -> Result<T>,
) -> sval::Result<T> {
    match f(stream) {
        Ok(v) => Ok(v),
        Err(e) => {
            stream.state = State::Done(Some(Err(e)));

            sval::error()
        }
    }
}

impl<'sval, S: Stream<'sval>> State<'sval, S> {
    fn value<V: sval_ref::ValueRef<'sval> + ?Sized>(
        &mut self,
        value: &V,
        any: impl FnOnce(S, &V) -> Result<S::Ok>,
    ) -> Result<Option<S::Ok>> {
        self.value_with(
            |stream| any(stream, value),
            |stream, tag, label, index| stream.tagged(tag, label, index, value),
            |stream| StreamSeq::value(stream, value),
            |stream| StreamMap::key(stream, value),
            |stream| StreamMap::value(stream, value),
            |stream, tag, index| stream.value(tag, index, value),
            |stream, tag, label| stream.value(tag, label, value),
            |stream, tag, label, index| stream.tagged(tag, label, index, value),
            |stream, tag, index| stream.value(tag, index, value),
            |stream, tag, label| stream.value(tag, label, value),
        )
    }

    fn value_computed<V: sval::Value + ?Sized>(
        &mut self,
        value: &V,
        any: impl FnOnce(S, &V) -> Result<S::Ok>,
    ) -> Result<Option<S::Ok>> {
        self.value_with(
            |stream| any(stream, value),
            |stream, tag, label, index| stream.tagged_computed(tag, label, index, value),
            |stream| StreamSeq::value_computed(stream, value),
            |stream| StreamMap::key_computed(stream, value),
            |stream| StreamMap::value_computed(stream, value),
            |stream, tag, index| stream.value_computed(tag, index, value),
            |stream, tag, label| stream.value_computed(tag, label, value),
            |stream, tag, label, index| stream.tagged_computed(tag, label, index, value),
            |stream, tag, index| stream.value_computed(tag, index, value),
            |stream, tag, label| stream.value_computed(tag, label, value),
        )
    }

    fn value_with(
        &mut self,
        any: impl FnOnce(S) -> Result<S::Ok>,
        tagged: impl FnOnce(
            S,
            Option<sval::Tag>,
            Option<sval::Label>,
            Option<sval::Index>,
        ) -> Result<S::Ok>,
        seq: impl FnOnce(&mut S::Seq) -> Result,
        map_key: impl FnOnce(&mut S::Map) -> Result,
        map_value: impl FnOnce(&mut S::Map) -> Result,
        tuple: impl FnOnce(&mut S::Tuple, Option<sval::Tag>, sval::Index) -> Result,
        record: impl FnOnce(&mut S::Record, Option<sval::Tag>, sval::Label) -> Result,
        tagged_variant: impl FnOnce(
            FlatStreamEnum<S::Enum>,
            Option<sval::Tag>,
            Option<sval::Label>,
            Option<sval::Index>,
        ) -> Result<S::Ok>,
        tuple_variant: impl FnOnce(
            &mut <S::Enum as StreamEnum<'sval>>::Tuple,
            Option<sval::Tag>,
            sval::Index,
        ) -> Result,
        record_variant: impl FnOnce(
            &mut <S::Enum as StreamEnum<'sval>>::Record,
            Option<sval::Tag>,
            sval::Label,
        ) -> Result,
    ) -> Result<Option<S::Ok>> {
        match self {
            State::Any(ref mut stream) => {
                let stream = stream.take().ok_or_else(|| {
                    Error::invalid_value("cannot stream value; the stream is already completed")
                })?;

                Ok(Some(any(stream.stream)?))
            }
            State::Tagged(ref mut stream) => {
                let stream = stream.take().ok_or_else(|| {
                    Error::invalid_value(
                        "cannot stream tagged value; the stream is already completed",
                    )
                })?;

                Ok(Some(tagged(
                    stream.stream,
                    stream.tag,
                    stream.label,
                    stream.index,
                )?))
            }
            State::Seq(stream) => {
                let stream = stream.as_mut().ok_or_else(|| {
                    Error::invalid_value(
                        "cannot stream a sequence; the stream is already completed",
                    )
                })?;

                seq(&mut stream.stream)?;

                Ok(None)
            }
            State::Map(stream) => {
                let stream = stream.as_mut().ok_or_else(|| {
                    Error::invalid_value("cannot stream a map; the stream is already completed")
                })?;

                if stream.is_key {
                    map_key(&mut stream.stream)?;
                } else {
                    map_value(&mut stream.stream)?;
                }

                Ok(None)
            }
            State::Tuple(stream) => {
                let stream = stream.as_mut().ok_or_else(|| {
                    Error::invalid_value("cannot stream a tuple; the stream is already completed")
                })?;

                let (tag, index) = stream.field.take().ok_or_else(|| {
                    Error::invalid_value("cannot stream a tuple; the field index is missing")
                })?;

                tuple(&mut stream.stream, tag, index)?;

                Ok(None)
            }
            State::Record(stream) => {
                let stream = stream.as_mut().ok_or_else(|| {
                    Error::invalid_value("cannot stream a record; the stream is already completed")
                })?;

                let (tag, label) = stream.field.take().ok_or_else(|| {
                    Error::invalid_value("cannot stream a record; the field label is missing")
                })?;

                record(&mut stream.stream, tag, label)?;

                Ok(None)
            }
            State::Enum(_) => Err(Error::invalid_value(
                "cannot stream an enum; the stream is in an invalid state",
            )),
            State::EnumVariant(stream) => match stream {
                Some(EnumVariant::Tuple(Tuple {
                    ref mut stream,
                    ref mut field,
                })) => {
                    let (tag, index) = field.take().ok_or_else(|| {
                        Error::invalid_value(
                            "cannot stream a tuple variant; the field index is missing",
                        )
                    })?;

                    tuple_variant(stream, tag, index)?;

                    Ok(None)
                }
                Some(EnumVariant::Record(Record {
                    ref mut stream,
                    ref mut field,
                })) => {
                    let (tag, label) = field.take().ok_or_else(|| {
                        Error::invalid_value(
                            "cannot stream a record variant; the field label is missing",
                        )
                    })?;

                    record_variant(stream, tag, label)?;

                    Ok(None)
                }
                stream => {
                    match stream.take().ok_or_else(|| {
                        Error::invalid_value(
                            "cannot stream an enum variant; the stream is already completed",
                        )
                    })? {
                        EnumVariant::Tagged(Tagged {
                            stream,
                            tag,
                            label,
                            index,
                        }) => Ok(Some(tagged_variant(stream, tag, label, index)?)),
                        _ => unreachable!(),
                    }
                }
            },
            State::Done(_) => Err(Error::invalid_value(
                "cannot stream a value; the stream is already completed",
            )),
        }
    }
}

impl<'sval, S: Stream<'sval>> FlatStream<'sval, S> {
    fn buffer_or_stream_with(
        &mut self,
        buffer: impl FnOnce(&mut ValueBuf<'sval>) -> sval::Result,
        stream: impl FnOnce(&mut Self) -> Result<Option<S::Ok>>,
    ) -> sval::Result {
        let mut r = None;
        self.buffer_or_with(buffer, |s| match stream(s) {
            Ok(ok) => {
                r = ok;
                Ok(())
            }
            Err(e) => Err(e),
        })?;

        if let Some(ok) = r {
            self.state = State::Done(Some(Ok(ok)));
        }

        Ok(())
    }

    fn buffer_or_with(
        &mut self,
        buffer: impl FnOnce(&mut ValueBuf<'sval>) -> sval::Result,
        stream: impl FnOnce(&mut Self) -> Result,
    ) -> sval::Result {
        let r = try_catch(self, |s: &mut FlatStream<'_, S>| match s {
            FlatStream {
                buffered: Some(Buffered::Value(ref mut buf)),
                ..
            } => {
                if buffer(buf).is_err() {
                    let buf = mem::take(buf);

                    Err(buf.into_err().map(Error::buffer).unwrap_or_else(|| {
                        Error::invalid_value("the value itself failed to stream")
                    }))
                } else {
                    Ok(())
                }
            }
            s => stream(s),
        });

        r
    }

    fn buffer_or_begin_with(
        &mut self,
        mut buffer: impl FnMut(&mut ValueBuf<'sval>) -> sval::Result,
        transition_any: impl FnOnce(Any<'sval, S>) -> Result<State<'sval, S>>,
        transition_enum: impl FnOnce(Enum<FlatStreamEnum<S::Enum>>) -> Result<State<'sval, S>>,
    ) -> sval::Result {
        let new_buf = try_catch(self, |stream| match stream {
            FlatStream {
                buffered: Some(Buffered::Value(ref mut buf)),
                state: _,
            } => {
                if buffer(buf).is_err() {
                    let buf = mem::take(buf);

                    return Err(buf.into_err().map(Error::buffer).unwrap_or_else(|| {
                        Error::invalid_value("the value itself failed to stream")
                    }));
                }

                Ok(None)
            }
            FlatStream {
                buffered: None,
                state: State::Any(state),
            } => {
                stream.state = transition_any(state.take().ok_or_else(|| {
                    Error::invalid_value("cannot stream value; the stream is already completed")
                })?)?;

                Ok(None)
            }
            FlatStream {
                buffered: None,
                state: State::Enum(state),
            } => {
                stream.state = transition_enum(state.take().ok_or_else(|| {
                    Error::invalid_value(
                        "cannot stream enum value; the stream is already completed",
                    )
                })?)?;

                Ok(None)
            }
            FlatStream {
                buffered: None,
                state: _,
            } => {
                let mut buf = ValueBuf::new();
                if buffer(&mut buf).is_err() {
                    return Err(buf.into_err().map(Error::buffer).unwrap_or_else(|| {
                        Error::invalid_value("the value itself failed to stream")
                    }));
                }

                Ok(Some(Buffered::Value(buf)))
            }
            _ => Err(Error::invalid_value(
                "cannot begin buffering; the stream is in an invalid state",
            )),
        })?;

        if let Some(new_buf) = new_buf {
            self.buffered = Some(new_buf);
        }

        Ok(())
    }

    fn buffer_or_end_with(
        &mut self,
        buffer: impl FnOnce(&mut ValueBuf<'sval>) -> sval::Result,
        transition: impl FnOnce(&mut Self) -> Result<S::Ok>,
    ) -> sval::Result {
        let r = try_catch(self, |stream| match stream {
            FlatStream { buffered: None, .. } => Ok(Some(transition(stream)?)),
            FlatStream { buffered, .. } => {
                let buf = if let Some(Buffered::Value(ref mut buf)) = buffered {
                    buf
                } else {
                    return Err(Error::invalid_value(
                        "cannot end buffering value; the stream is in an invalid state",
                    ));
                };

                if buffer(buf).is_err() {
                    let buf = mem::take(buf);

                    return Err(buf.into_err().map(Error::buffer).unwrap_or_else(|| {
                        Error::invalid_value("the value itself failed to stream")
                    }));
                }

                if buf.is_complete() {
                    let buf = mem::take(buf);
                    *buffered = None;

                    return stream
                        .state
                        .value_computed(&buf, |stream, value| stream.value_computed(value));
                }

                return Ok(None);
            }
        })?;

        if let Some(r) = r {
            self.state = State::Done(Some(Ok(r)));
        }

        Ok(())
    }

    fn put_buffer(&mut self, buf: Buffered<'sval>) -> Result {
        match self.buffered {
            None => {
                self.buffered = Some(buf);

                Ok(())
            }
            Some(_) => Err(Error::invalid_value(
                "cannot begin buffering; a buffer is already active",
            )),
        }
    }

    fn with_text(&mut self, buffer: impl FnOnce(&mut TextBuf<'sval>) -> Result) -> Result {
        match self.buffered {
            Some(Buffered::Text(ref mut buf)) => buffer(buf),
            _ => Err(Error::invalid_value(
                "cannot buffer text; no active text buffer",
            )),
        }
    }

    fn take_text(&mut self) -> Result<TextBuf<'sval>> {
        match self.buffered {
            Some(Buffered::Text(ref mut buf)) => {
                let buf = mem::take(buf);
                self.buffered = None;

                Ok(buf)
            }
            _ => Err(Error::invalid_value(
                "cannot end buffering text; no active text buffer",
            )),
        }
    }

    fn with_binary(&mut self, buffer: impl FnOnce(&mut BinaryBuf<'sval>) -> Result) -> Result {
        match self.buffered {
            Some(Buffered::Binary(ref mut buf)) => buffer(buf),
            _ => Err(Error::invalid_value(
                "cannot buffer binary; no active binary buffer",
            )),
        }
    }

    fn take_binary(&mut self) -> Result<BinaryBuf<'sval>> {
        match self.buffered {
            Some(Buffered::Binary(ref mut buf)) => {
                let buf = mem::take(buf);
                self.buffered = None;

                Ok(buf)
            }
            _ => Err(Error::invalid_value(
                "cannot end buffering binary; no active binary buffer",
            )),
        }
    }

    fn take_seq(&mut self) -> Result<Seq<S::Seq>> {
        match self {
            FlatStream {
                buffered: None,
                state: State::Seq(seq),
            } => seq.take().ok_or_else(|| {
                Error::invalid_value("cannot end a sequence; the stream is already completed")
            }),
            _ => Err(Error::invalid_value(
                "cannot end a sequence; invalid stream state",
            )),
        }
    }

    fn with_map(&mut self, f: impl FnOnce(&mut Map<S::Map>) -> Result) -> Result {
        match self {
            FlatStream {
                buffered: None,
                state: State::Map(Some(map)),
            } => f(map),
            _ => Err(Error::invalid_value(
                "cannot stream a map; invalid stream state",
            )),
        }
    }

    fn take_map(&mut self) -> Result<Map<S::Map>> {
        match self {
            FlatStream {
                buffered: None,
                state: State::Map(map),
            } => map.take().ok_or_else(|| {
                Error::invalid_value("cannot end a map; the stream is already completed")
            }),
            _ => Err(Error::invalid_value(
                "cannot end a map; invalid stream state",
            )),
        }
    }

    fn with_tuple(
        &mut self,
        tuple: impl FnOnce(&mut Tuple<S::Tuple>) -> Result,
        tuple_variant: impl FnOnce(&mut Tuple<<S::Enum as StreamEnum<'sval>>::Tuple>) -> Result,
    ) -> Result {
        match self {
            FlatStream {
                buffered: None,
                state: State::Tuple(Some(stream)),
            } => tuple(stream),
            FlatStream {
                buffered: None,
                state: State::EnumVariant(Some(EnumVariant::Tuple(stream))),
            } => tuple_variant(stream),
            _ => Err(Error::invalid_value(
                "cannot stream a tuple; invalid stream state",
            )),
        }
    }

    fn take_with_tuple<T>(
        &mut self,
        tuple: impl FnOnce(Tuple<S::Tuple>) -> Result<T>,
        tuple_variant: impl FnOnce(Tuple<<S::Enum as StreamEnum<'sval>>::Tuple>) -> Result<T>,
    ) -> Result<T> {
        match self {
            FlatStream {
                buffered: None,
                state: State::Tuple(stream),
            } => tuple(stream.take().ok_or_else(|| {
                Error::invalid_value("cannot end a tuple; the stream is already completed")
            })?),
            FlatStream {
                buffered: None,
                state: State::EnumVariant(stream),
            } => {
                let stream = stream.take().ok_or_else(|| {
                    Error::invalid_value("cannot end a tuple; the stream is already completed")
                })?;

                match stream {
                    EnumVariant::Tuple(stream) => tuple_variant(stream),
                    _ => Err(Error::invalid_value(
                        "cannot end a tuple; invalid stream state",
                    )),
                }
            }
            _ => Err(Error::invalid_value(
                "cannot end a tuple; invalid stream state",
            )),
        }
    }

    fn with_record(
        &mut self,
        record: impl FnOnce(&mut Record<S::Record>) -> Result,
        record_variant: impl FnOnce(&mut Record<<S::Enum as StreamEnum<'sval>>::Record>) -> Result,
    ) -> Result {
        match self {
            FlatStream {
                buffered: None,
                state: State::Record(Some(stream)),
            } => record(stream),
            FlatStream {
                buffered: None,
                state: State::EnumVariant(Some(EnumVariant::Record(stream))),
            } => record_variant(stream),
            _ => Err(Error::invalid_value(
                "cannot stream a record; invalid stream state",
            )),
        }
    }

    fn take_with_record<T>(
        &mut self,
        record: impl FnOnce(Record<S::Record>) -> Result<T>,
        record_variant: impl FnOnce(Record<<S::Enum as StreamEnum<'sval>>::Record>) -> Result<T>,
    ) -> Result<T> {
        match self {
            FlatStream {
                buffered: None,
                state: State::Record(stream),
            } => record(stream.take().ok_or_else(|| {
                Error::invalid_value("cannot end a record; the stream is already completed")
            })?),
            FlatStream {
                buffered: None,
                state: State::EnumVariant(stream),
            } => {
                let stream = stream.take().ok_or_else(|| {
                    Error::invalid_value("cannot end a record; the stream is already completed")
                })?;

                match stream {
                    EnumVariant::Record(stream) => record_variant(stream),
                    _ => Err(Error::invalid_value(
                        "cannot end a record; invalid stream state",
                    )),
                }
            }
            _ => Err(Error::invalid_value(
                "cannot end a record; invalid stream state",
            )),
        }
    }

    fn take_enum(&mut self) -> Result<Option<Enum<FlatStreamEnum<S::Enum>>>> {
        match self {
            FlatStream {
                buffered: None,
                state: State::Enum(variant),
            } => Ok(variant.take()),
            FlatStream {
                buffered: None,
                state: State::Done(_),
            } => Ok(None),
            _ => Err(Error::invalid_value(
                "cannot end an enum; invalid stream state",
            )),
        }
    }
}

struct Tag<'a>(
    Option<&'a sval::Tag>,
    Option<&'a sval::Label<'a>>,
    Option<&'a sval::Index>,
);

impl<'a> sval::Value for Tag<'a> {
    fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
        stream.tag(self.0, self.1, self.2)
    }
}
