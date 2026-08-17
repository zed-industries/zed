mod private {
    pub trait DispatchStream<'sval> {
        fn dispatch_value_computed(&mut self, value: &dyn crate::Value) -> sval::Result;

        fn dispatch_null(&mut self) -> sval::Result;

        fn dispatch_u8(&mut self, value: u8) -> sval::Result;

        fn dispatch_u16(&mut self, value: u16) -> sval::Result;

        fn dispatch_u32(&mut self, value: u32) -> sval::Result;

        fn dispatch_u64(&mut self, value: u64) -> sval::Result;

        fn dispatch_u128(&mut self, value: u128) -> sval::Result;

        fn dispatch_i8(&mut self, value: i8) -> sval::Result;

        fn dispatch_i16(&mut self, value: i16) -> sval::Result;

        fn dispatch_i32(&mut self, value: i32) -> sval::Result;

        fn dispatch_i64(&mut self, value: i64) -> sval::Result;

        fn dispatch_i128(&mut self, value: i128) -> sval::Result;

        fn dispatch_f32(&mut self, value: f32) -> sval::Result;

        fn dispatch_f64(&mut self, value: f64) -> sval::Result;

        fn dispatch_bool(&mut self, value: bool) -> sval::Result;

        fn dispatch_text_begin(&mut self, num_bytes: Option<usize>) -> sval::Result;

        fn dispatch_text_end(&mut self) -> sval::Result;

        fn dispatch_text_fragment(&mut self, fragment: &'sval str) -> sval::Result;

        fn dispatch_text_fragment_computed(&mut self, fragment: &str) -> sval::Result;

        fn dispatch_binary_begin(&mut self, num_bytes: Option<usize>) -> sval::Result;

        fn dispatch_binary_end(&mut self) -> sval::Result;

        fn dispatch_binary_fragment(&mut self, fragment: &'sval [u8]) -> sval::Result;

        fn dispatch_binary_fragment_computed(&mut self, fragment: &[u8]) -> sval::Result;

        fn dispatch_map_begin(&mut self, num_entries_hint: Option<usize>) -> sval::Result;

        fn dispatch_map_end(&mut self) -> sval::Result;

        fn dispatch_map_key_begin(&mut self) -> sval::Result;

        fn dispatch_map_key_end(&mut self) -> sval::Result;

        fn dispatch_map_value_begin(&mut self) -> sval::Result;

        fn dispatch_map_value_end(&mut self) -> sval::Result;

        fn dispatch_seq_begin(&mut self, num_elems_hint: Option<usize>) -> sval::Result;

        fn dispatch_seq_end(&mut self) -> sval::Result;

        fn dispatch_seq_value_begin(&mut self) -> sval::Result;

        fn dispatch_seq_value_end(&mut self) -> sval::Result;

        fn dispatch_tagged_begin(
            &mut self,
            tag: Option<&sval::Tag>,
            label: Option<&sval::Label>,
            index: Option<&sval::Index>,
        ) -> sval::Result;

        fn dispatch_tagged_end(
            &mut self,
            tag: Option<&sval::Tag>,
            label: Option<&sval::Label>,
            index: Option<&sval::Index>,
        ) -> sval::Result;

        fn dispatch_tag(
            &mut self,
            tag: Option<&sval::Tag>,
            label: Option<&sval::Label>,
            index: Option<&sval::Index>,
        ) -> sval::Result;

        fn dispatch_tag_hint(&mut self, tag: &sval::Tag) -> sval::Result;

        fn dispatch_record_begin(
            &mut self,
            tag: Option<&sval::Tag>,
            label: Option<&sval::Label>,
            index: Option<&sval::Index>,
            num_entries: Option<usize>,
        ) -> sval::Result;

        fn dispatch_record_value_begin(
            &mut self,
            tag: Option<&sval::Tag>,
            label: &sval::Label,
        ) -> sval::Result;

        fn dispatch_record_value_end(
            &mut self,
            tag: Option<&sval::Tag>,
            label: &sval::Label,
        ) -> sval::Result;

        fn dispatch_record_end(
            &mut self,
            tag: Option<&sval::Tag>,
            label: Option<&sval::Label>,
            index: Option<&sval::Index>,
        ) -> sval::Result;

        fn dispatch_tuple_begin(
            &mut self,
            tag: Option<&sval::Tag>,
            label: Option<&sval::Label>,
            index: Option<&sval::Index>,
            num_entries: Option<usize>,
        ) -> sval::Result;

        fn dispatch_tuple_value_begin(
            &mut self,
            tag: Option<&sval::Tag>,
            index: &sval::Index,
        ) -> sval::Result;

        fn dispatch_tuple_value_end(
            &mut self,
            tag: Option<&sval::Tag>,
            index: &sval::Index,
        ) -> sval::Result;

        fn dispatch_tuple_end(
            &mut self,
            tag: Option<&sval::Tag>,
            label: Option<&sval::Label>,
            index: Option<&sval::Index>,
        ) -> sval::Result;

        fn dispatch_record_tuple_begin(
            &mut self,
            tag: Option<&sval::Tag>,
            label: Option<&sval::Label>,
            index: Option<&sval::Index>,
            num_entries: Option<usize>,
        ) -> sval::Result;

        fn dispatch_record_tuple_value_begin(
            &mut self,
            tag: Option<&sval::Tag>,
            label: &sval::Label,
            index: &sval::Index,
        ) -> sval::Result;

        fn dispatch_record_tuple_value_end(
            &mut self,
            tag: Option<&sval::Tag>,
            label: &sval::Label,
            index: &sval::Index,
        ) -> sval::Result;

        fn dispatch_record_tuple_end(
            &mut self,
            tag: Option<&sval::Tag>,
            label: Option<&sval::Label>,
            index: Option<&sval::Index>,
        ) -> sval::Result;

        fn dispatch_enum_begin(
            &mut self,
            tag: Option<&sval::Tag>,
            label: Option<&sval::Label>,
            index: Option<&sval::Index>,
        ) -> sval::Result;

        fn dispatch_enum_end(
            &mut self,
            tag: Option<&sval::Tag>,
            label: Option<&sval::Label>,
            index: Option<&sval::Index>,
        ) -> sval::Result;
    }

    pub trait EraseStream<'sval> {
        fn erase_stream_ref(&self) -> crate::private::Erased<&dyn DispatchStream<'sval>>;
        fn erase_stream(&mut self) -> crate::private::Erased<&mut dyn DispatchStream<'sval>>;
    }
}

/**
An object-safe version of [`sval::Stream`].
*/
pub trait Stream<'sval>: private::EraseStream<'sval> {}

impl<'sval, R: sval::Stream<'sval>> Stream<'sval> for R {}

impl<'sval, R: sval::Stream<'sval>> private::EraseStream<'sval> for R {
    fn erase_stream_ref(&self) -> crate::private::Erased<&dyn private::DispatchStream<'sval>> {
        crate::private::Erased(self)
    }

    fn erase_stream(&mut self) -> crate::private::Erased<&mut dyn private::DispatchStream<'sval>> {
        crate::private::Erased(self)
    }
}

impl<'sval, R: sval::Stream<'sval>> private::DispatchStream<'sval> for R {
    fn dispatch_value_computed(&mut self, value: &dyn crate::Value) -> sval::Result {
        self.value_computed(value)
    }

    fn dispatch_null(&mut self) -> sval::Result {
        self.null()
    }

    fn dispatch_u8(&mut self, value: u8) -> sval::Result {
        self.u8(value)
    }

    fn dispatch_u16(&mut self, value: u16) -> sval::Result {
        self.u16(value)
    }

    fn dispatch_u32(&mut self, value: u32) -> sval::Result {
        self.u32(value)
    }

    fn dispatch_u64(&mut self, value: u64) -> sval::Result {
        self.u64(value)
    }

    fn dispatch_u128(&mut self, value: u128) -> sval::Result {
        self.u128(value)
    }

    fn dispatch_i8(&mut self, value: i8) -> sval::Result {
        self.i8(value)
    }

    fn dispatch_i16(&mut self, value: i16) -> sval::Result {
        self.i16(value)
    }

    fn dispatch_i32(&mut self, value: i32) -> sval::Result {
        self.i32(value)
    }

    fn dispatch_i64(&mut self, value: i64) -> sval::Result {
        self.i64(value)
    }

    fn dispatch_i128(&mut self, value: i128) -> sval::Result {
        self.i128(value)
    }

    fn dispatch_f32(&mut self, value: f32) -> sval::Result {
        self.f32(value)
    }

    fn dispatch_f64(&mut self, value: f64) -> sval::Result {
        self.f64(value)
    }

    fn dispatch_bool(&mut self, value: bool) -> sval::Result {
        self.bool(value)
    }

    fn dispatch_text_begin(&mut self, num_bytes: Option<usize>) -> sval::Result {
        self.text_begin(num_bytes)
    }

    fn dispatch_text_end(&mut self) -> sval::Result {
        self.text_end()
    }

    fn dispatch_text_fragment(&mut self, fragment: &'sval str) -> sval::Result {
        self.text_fragment(fragment)
    }

    fn dispatch_text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
        self.text_fragment_computed(fragment)
    }

    fn dispatch_binary_begin(&mut self, num_bytes: Option<usize>) -> sval::Result {
        self.binary_begin(num_bytes)
    }

    fn dispatch_binary_end(&mut self) -> sval::Result {
        self.binary_end()
    }

    fn dispatch_binary_fragment(&mut self, fragment: &'sval [u8]) -> sval::Result {
        self.binary_fragment(fragment)
    }

    fn dispatch_binary_fragment_computed(&mut self, fragment: &[u8]) -> sval::Result {
        self.binary_fragment_computed(fragment)
    }

    fn dispatch_map_begin(&mut self, num_entries_hint: Option<usize>) -> sval::Result {
        self.map_begin(num_entries_hint)
    }

    fn dispatch_map_end(&mut self) -> sval::Result {
        self.map_end()
    }

    fn dispatch_map_key_begin(&mut self) -> sval::Result {
        self.map_key_begin()
    }

    fn dispatch_map_key_end(&mut self) -> sval::Result {
        self.map_key_end()
    }

    fn dispatch_map_value_begin(&mut self) -> sval::Result {
        self.map_value_begin()
    }

    fn dispatch_map_value_end(&mut self) -> sval::Result {
        self.map_value_end()
    }

    fn dispatch_seq_begin(&mut self, num_elems_hint: Option<usize>) -> sval::Result {
        self.seq_begin(num_elems_hint)
    }

    fn dispatch_seq_end(&mut self) -> sval::Result {
        self.seq_end()
    }

    fn dispatch_seq_value_begin(&mut self) -> sval::Result {
        self.seq_value_begin()
    }

    fn dispatch_seq_value_end(&mut self) -> sval::Result {
        self.seq_value_end()
    }

    fn dispatch_tagged_begin(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        self.tagged_begin(tag, label, index)
    }

    fn dispatch_tagged_end(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        self.tagged_end(tag, label, index)
    }

    fn dispatch_tag(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        self.tag(tag, label, index)
    }

    fn dispatch_tag_hint(&mut self, tag: &sval::Tag) -> sval::Result {
        self.tag_hint(tag)
    }

    fn dispatch_record_begin(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
        num_entries: Option<usize>,
    ) -> sval::Result {
        self.record_begin(tag, label, index, num_entries)
    }

    fn dispatch_record_value_begin(
        &mut self,
        tag: Option<&sval::Tag>,
        label: &sval::Label,
    ) -> sval::Result {
        self.record_value_begin(tag, label)
    }

    fn dispatch_record_value_end(
        &mut self,
        tag: Option<&sval::Tag>,
        label: &sval::Label,
    ) -> sval::Result {
        self.record_value_end(tag, label)
    }

    fn dispatch_record_end(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        self.record_end(tag, label, index)
    }

    fn dispatch_tuple_begin(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
        num_entries: Option<usize>,
    ) -> sval::Result {
        self.tuple_begin(tag, label, index, num_entries)
    }

    fn dispatch_tuple_value_begin(
        &mut self,
        tag: Option<&sval::Tag>,
        index: &sval::Index,
    ) -> sval::Result {
        self.tuple_value_begin(tag, index)
    }

    fn dispatch_tuple_value_end(
        &mut self,
        tag: Option<&sval::Tag>,
        index: &sval::Index,
    ) -> sval::Result {
        self.tuple_value_end(tag, index)
    }

    fn dispatch_tuple_end(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        self.tuple_end(tag, label, index)
    }

    fn dispatch_record_tuple_begin(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
        num_entries: Option<usize>,
    ) -> sval::Result {
        self.record_tuple_begin(tag, label, index, num_entries)
    }

    fn dispatch_record_tuple_value_begin(
        &mut self,
        tag: Option<&sval::Tag>,
        label: &sval::Label,
        index: &sval::Index,
    ) -> sval::Result {
        self.record_tuple_value_begin(tag, label, index)
    }

    fn dispatch_record_tuple_value_end(
        &mut self,
        tag: Option<&sval::Tag>,
        label: &sval::Label,
        index: &sval::Index,
    ) -> sval::Result {
        self.record_tuple_value_end(tag, label, index)
    }

    fn dispatch_record_tuple_end(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        self.record_tuple_end(tag, label, index)
    }

    fn dispatch_enum_begin(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        self.enum_begin(tag, label, index)
    }

    fn dispatch_enum_end(
        &mut self,
        tag: Option<&sval::Tag>,
        label: Option<&sval::Label>,
        index: Option<&sval::Index>,
    ) -> sval::Result {
        self.enum_end(tag, label, index)
    }
}

macro_rules! impl_stream {
    ($($impl:tt)*) => {
        $($impl)* {
            fn value_computed<V: sval::Value + ?Sized>(&mut self, v: &V) -> sval::Result {
                self.erase_stream().0.dispatch_value_computed(&v)
            }

            fn null(&mut self) -> sval::Result {
                self.erase_stream().0.dispatch_null()
            }

            fn u8(&mut self, value: u8) -> sval::Result {
                self.erase_stream().0.dispatch_u8(value)
            }

            fn u16(&mut self, value: u16) -> sval::Result {
                self.erase_stream().0.dispatch_u16(value)
            }

            fn u32(&mut self, value: u32) -> sval::Result {
                self.erase_stream().0.dispatch_u32(value)
            }

            fn u64(&mut self, value: u64) -> sval::Result {
                self.erase_stream().0.dispatch_u64(value)
            }

            fn u128(&mut self, value: u128) -> sval::Result {
                self.erase_stream().0.dispatch_u128(value)
            }

            fn i8(&mut self, value: i8) -> sval::Result {
                self.erase_stream().0.dispatch_i8(value)
            }

            fn i16(&mut self, value: i16) -> sval::Result {
                self.erase_stream().0.dispatch_i16(value)
            }

            fn i32(&mut self, value: i32) -> sval::Result {
                self.erase_stream().0.dispatch_i32(value)
            }

            fn i64(&mut self, value: i64) -> sval::Result {
                self.erase_stream().0.dispatch_i64(value)
            }

            fn i128(&mut self, value: i128) -> sval::Result {
                self.erase_stream().0.dispatch_i128(value)
            }

            fn f32(&mut self, value: f32) -> sval::Result {
                self.erase_stream().0.dispatch_f32(value)
            }

            fn f64(&mut self, value: f64) -> sval::Result {
                self.erase_stream().0.dispatch_f64(value)
            }

            fn bool(&mut self, value: bool) -> sval::Result {
                self.erase_stream().0.dispatch_bool(value)
            }

            fn text_begin(&mut self, num_bytes: Option<usize>) -> sval::Result {
                self.erase_stream().0.dispatch_text_begin(num_bytes)
            }

            fn text_end(&mut self) -> sval::Result {
                self.erase_stream().0.dispatch_text_end()
            }

            fn text_fragment(&mut self, fragment: &'sval str) -> sval::Result {
                self.erase_stream().0.dispatch_text_fragment(fragment)
            }

            fn text_fragment_computed(&mut self, fragment: &str) -> sval::Result {
                self.erase_stream().0.dispatch_text_fragment_computed(fragment)
            }

            fn binary_begin(&mut self, num_bytes: Option<usize>) -> sval::Result {
                self.erase_stream().0.dispatch_binary_begin(num_bytes)
            }

            fn binary_end(&mut self) -> sval::Result {
                self.erase_stream().0.dispatch_binary_end()
            }

            fn binary_fragment(&mut self, fragment: &'sval [u8]) -> sval::Result {
                self.erase_stream().0.dispatch_binary_fragment(fragment)
            }

            fn binary_fragment_computed(&mut self, fragment: &[u8]) -> sval::Result {
                self.erase_stream().0.dispatch_binary_fragment_computed(fragment)
            }

            fn map_begin(&mut self, num_entries_hint: Option<usize>) -> sval::Result {
                self.erase_stream().0.dispatch_map_begin(num_entries_hint)
            }

            fn map_end(&mut self) -> sval::Result {
                self.erase_stream().0.dispatch_map_end()
            }

            fn map_key_begin(&mut self) -> sval::Result {
                self.erase_stream().0.dispatch_map_key_begin()
            }

            fn map_key_end(&mut self) -> sval::Result {
                self.erase_stream().0.dispatch_map_key_end()
            }

            fn map_value_begin(&mut self) -> sval::Result {
                self.erase_stream().0.dispatch_map_value_begin()
            }

            fn map_value_end(&mut self) -> sval::Result {
                self.erase_stream().0.dispatch_map_value_end()
            }

            fn seq_begin(&mut self, num_elems_hint: Option<usize>) -> sval::Result {
                self.erase_stream().0.dispatch_seq_begin(num_elems_hint)
            }

            fn seq_end(&mut self) -> sval::Result {
                self.erase_stream().0.dispatch_seq_end()
            }

            fn seq_value_begin(&mut self) -> sval::Result {
                self.erase_stream().0.dispatch_seq_value_begin()
            }

            fn seq_value_end(&mut self) -> sval::Result {
                self.erase_stream().0.dispatch_seq_value_end()
            }

            fn tagged_begin(&mut self, tag: Option<&sval::Tag>, label: Option<&sval::Label>, index: Option<&sval::Index>) -> sval::Result {
                self.erase_stream().0.dispatch_tagged_begin(tag, label, index)
            }

            fn tagged_end(&mut self, tag: Option<&sval::Tag>, label: Option<&sval::Label>, index: Option<&sval::Index>) -> sval::Result {
                self.erase_stream().0.dispatch_tagged_end(tag, label, index)
            }

            fn tag(
                &mut self,
                tag: Option<&sval::Tag>,
                label: Option<&sval::Label>,
                index: Option<&sval::Index>,
            ) -> sval::Result {
                self.erase_stream().0.dispatch_tag(tag, label, index)
            }

            fn tag_hint(&mut self, tag: &sval::Tag) -> sval::Result {
                self.erase_stream().0.dispatch_tag_hint(tag)
            }

            fn record_begin(&mut self, tag: Option<&sval::Tag>, label: Option<&sval::Label>, index: Option<&sval::Index>, num_entries_hint: Option<usize>) -> sval::Result {
                self.erase_stream().0.dispatch_record_begin(tag, label, index, num_entries_hint)
            }

            fn record_value_begin(&mut self, tag: Option<&sval::Tag>, label: &sval::Label) -> sval::Result {
                self.erase_stream().0.dispatch_record_value_begin(tag, label)
            }

            fn record_value_end(&mut self, tag: Option<&sval::Tag>, label: &sval::Label) -> sval::Result {
                self.erase_stream().0.dispatch_record_value_end(tag, label)
            }

            fn record_end(&mut self, tag: Option<&sval::Tag>, label: Option<&sval::Label>, index: Option<&sval::Index>) -> sval::Result {
                self.erase_stream().0.dispatch_record_end(tag, label, index)
            }

            fn tuple_begin(&mut self, tag: Option<&sval::Tag>, label: Option<&sval::Label>, index: Option<&sval::Index>, num_entries_hint: Option<usize>) -> sval::Result {
                self.erase_stream().0.dispatch_tuple_begin(tag, label, index, num_entries_hint)
            }

            fn tuple_value_begin(&mut self, tag: Option<&sval::Tag>, index: &sval::Index) -> sval::Result {
                self.erase_stream().0.dispatch_tuple_value_begin(tag, index)
            }

            fn tuple_value_end(&mut self, tag: Option<&sval::Tag>, index: &sval::Index) -> sval::Result {
                self.erase_stream().0.dispatch_tuple_value_end(tag, index)
            }

            fn tuple_end(&mut self, tag: Option<&sval::Tag>, label: Option<&sval::Label>, index: Option<&sval::Index>) -> sval::Result {
                self.erase_stream().0.dispatch_tuple_end(tag, label, index)
            }

            fn record_tuple_begin(
                &mut self,
                tag: Option<&sval::Tag>,
                label: Option<&sval::Label>,
                index: Option<&sval::Index>,
                num_entries: Option<usize>,
            ) -> sval::Result {
                self.erase_stream().0.dispatch_record_tuple_begin(tag, label, index, num_entries)
            }

            fn record_tuple_value_begin(
                &mut self,
                tag: Option<&sval::Tag>,
                label: &sval::Label,
                index: &sval::Index,
            ) -> sval::Result {
                self.erase_stream().0.dispatch_record_tuple_value_begin(tag, label, index)
            }

            fn record_tuple_value_end(
                &mut self,
                tag: Option<&sval::Tag>,
                label: &sval::Label,
                index: &sval::Index,
            ) -> sval::Result {
                self.erase_stream().0.dispatch_record_tuple_value_end(tag, label, index)
            }

            fn record_tuple_end(
                &mut self,
                tag: Option<&sval::Tag>,
                label: Option<&sval::Label>,
                index: Option<&sval::Index>,
            ) -> sval::Result {
                self.erase_stream().0.dispatch_record_tuple_end(tag, label, index)
            }

            fn enum_begin(&mut self, tag: Option<&sval::Tag>, label: Option<&sval::Label>, index: Option<&sval::Index>) -> sval::Result {
                self.erase_stream().0.dispatch_enum_begin(tag, label, index)
            }

            fn enum_end(&mut self, tag: Option<&sval::Tag>, label: Option<&sval::Label>, index: Option<&sval::Index>) -> sval::Result {
                self.erase_stream().0.dispatch_enum_end(tag, label, index)
            }
        }
    }
}

impl_stream!(impl<'sval, 'd> sval::Stream<'sval> for dyn Stream<'sval> + 'd);
impl_stream!(impl<'sval, 'd> sval::Stream<'sval> for dyn Stream<'sval> + Send + 'd);
impl_stream!(impl<'sval, 'd> sval::Stream<'sval> for dyn Stream<'sval> + Send + Sync + 'd);
