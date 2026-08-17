use sval::{tags, Index, Result, Stream};

use crate::ValueRef;

impl<'sval, T: ValueRef<'sval>> ValueRef<'sval> for [T] {
    fn stream_ref<S: Stream<'sval> + ?Sized>(&self, stream: &mut S) -> Result {
        stream.seq_begin(Some(self.len()))?;

        for elem in self {
            stream.seq_value_begin()?;
            crate::stream_ref(stream, elem)?;
            stream.seq_value_end()?;
        }

        stream.seq_end()
    }
}

impl<'sval, T: ValueRef<'sval>, const N: usize> ValueRef<'sval> for [T; N] {
    fn stream_ref<S: Stream<'sval> + ?Sized>(&self, stream: &mut S) -> Result {
        stream.tagged_begin(Some(&tags::CONSTANT_SIZE), None, None)?;
        stream.seq_begin(Some(self.len()))?;

        for elem in self {
            stream.seq_value_begin()?;
            crate::stream_ref(stream, elem)?;
            stream.seq_value_end()?;
        }

        stream.seq_end()?;
        stream.tagged_end(Some(&tags::CONSTANT_SIZE), None, None)
    }
}

macro_rules! tuple {
    ($(
        $len:expr => ( $(self.$i:tt: $ty:ident,)+ ),
    )+) => {
        $(
            impl<'sval, $($ty: ValueRef<'sval>),+> ValueRef<'sval> for ($($ty,)+) {
                fn stream_ref<S: Stream<'sval> + ?Sized>(&self, stream: &mut S) -> Result {
                    stream.tuple_begin(None, None, None, Some($len))?;

                    $(
                        stream.tuple_value_begin(None, &Index::new($i).with_tag(&sval::tags::VALUE_OFFSET))?;
                        crate::stream_ref(stream, &self.$i)?;
                        stream.tuple_value_end(None, &Index::new($i).with_tag(&sval::tags::VALUE_OFFSET))?;
                    )+

                    stream.tuple_end(None, None, None)
                }
            }
        )+
    }
}

tuple! {
    1 => (
        self.0: T0,
    ),
    2 => (
        self.0: T0,
        self.1: T1,
    ),
    3 => (
        self.0: T0,
        self.1: T1,
        self.2: T2,
    ),
    4 => (
        self.0: T0,
        self.1: T1,
        self.2: T2,
        self.3: T3,
    ),
    5 => (
        self.0: T0,
        self.1: T1,
        self.2: T2,
        self.3: T3,
        self.4: T4,
    ),
    6 => (
        self.0: T0,
        self.1: T1,
        self.2: T2,
        self.3: T3,
        self.4: T4,
        self.5: T5,
    ),
    7 => (
        self.0: T0,
        self.1: T1,
        self.2: T2,
        self.3: T3,
        self.4: T4,
        self.5: T5,
        self.6: T6,
    ),
    8 => (
        self.0: T0,
        self.1: T1,
        self.2: T2,
        self.3: T3,
        self.4: T4,
        self.5: T5,
        self.6: T6,
        self.7: T7,
    ),
    9 => (
        self.0: T0,
        self.1: T1,
        self.2: T2,
        self.3: T3,
        self.4: T4,
        self.5: T5,
        self.6: T6,
        self.7: T7,
        self.8: T8,
    ),
    10 => (
        self.0: T0,
        self.1: T1,
        self.2: T2,
        self.3: T3,
        self.4: T4,
        self.5: T5,
        self.6: T6,
        self.7: T7,
        self.8: T8,
        self.9: T9,
    ),
    11 => (
        self.0: T0,
        self.1: T1,
        self.2: T2,
        self.3: T3,
        self.4: T4,
        self.5: T5,
        self.6: T6,
        self.7: T7,
        self.8: T8,
        self.9: T9,
        self.10: T10,
    ),
    12 => (
        self.0: T0,
        self.1: T1,
        self.2: T2,
        self.3: T3,
        self.4: T4,
        self.5: T5,
        self.6: T6,
        self.7: T7,
        self.8: T8,
        self.9: T9,
        self.10: T10,
        self.11: T11,
    ),
    13 => (
        self.0: T0,
        self.1: T1,
        self.2: T2,
        self.3: T3,
        self.4: T4,
        self.5: T5,
        self.6: T6,
        self.7: T7,
        self.8: T8,
        self.9: T9,
        self.10: T10,
        self.11: T11,
        self.12: T12,
    ),
    14 => (
        self.0: T0,
        self.1: T1,
        self.2: T2,
        self.3: T3,
        self.4: T4,
        self.5: T5,
        self.6: T6,
        self.7: T7,
        self.8: T8,
        self.9: T9,
        self.10: T10,
        self.11: T11,
        self.12: T12,
        self.13: T13,
    ),
    15 => (
        self.0: T0,
        self.1: T1,
        self.2: T2,
        self.3: T3,
        self.4: T4,
        self.5: T5,
        self.6: T6,
        self.7: T7,
        self.8: T8,
        self.9: T9,
        self.10: T10,
        self.11: T11,
        self.12: T12,
        self.13: T13,
        self.14: T14,
    ),
    16 => (
        self.0: T0,
        self.1: T1,
        self.2: T2,
        self.3: T3,
        self.4: T4,
        self.5: T5,
        self.6: T6,
        self.7: T7,
        self.8: T8,
        self.9: T9,
        self.10: T10,
        self.11: T11,
        self.12: T12,
        self.13: T13,
        self.14: T14,
        self.15: T15,
    ),
}

#[cfg(feature = "alloc")]
mod alloc_support {
    use super::*;

    use crate::std::vec::Vec;

    impl<'sval, T: ValueRef<'sval>> ValueRef<'sval> for Vec<T> {
        fn stream_ref<S: Stream<'sval> + ?Sized>(&self, stream: &mut S) -> Result {
            (&**self).stream_ref(stream)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test::{compat_case, Ref, Token};

    #[test]
    fn seq_compat() {
        compat_case(
            &[Ref(&1)] as &[Ref<&i32>],
            &[
                Token::SeqBegin(Some(1)),
                Token::SeqValueBegin,
                Token::I32(1),
                Token::SeqValueEnd,
                Token::SeqEnd,
            ],
        );

        compat_case(
            &[Ref(&1)],
            &[
                Token::TaggedBegin(Some(sval::tags::CONSTANT_SIZE), None, None),
                Token::SeqBegin(Some(1)),
                Token::SeqValueBegin,
                Token::I32(1),
                Token::SeqValueEnd,
                Token::SeqEnd,
                Token::TaggedEnd(Some(sval::tags::CONSTANT_SIZE), None, None),
            ],
        );

        compat_case(
            &(Ref(&1), Ref(&2), Ref(&3)),
            &[
                Token::TupleBegin(None, None, None, Some(3)),
                Token::TupleValueBegin(None, sval::Index::new(0)),
                Token::I32(1),
                Token::TupleValueEnd(None, sval::Index::new(0)),
                Token::TupleValueBegin(None, sval::Index::new(1)),
                Token::I32(2),
                Token::TupleValueEnd(None, sval::Index::new(1)),
                Token::TupleValueBegin(None, sval::Index::new(2)),
                Token::I32(3),
                Token::TupleValueEnd(None, sval::Index::new(2)),
                Token::TupleEnd(None, None, None),
            ],
        );

        #[cfg(feature = "std")]
        {
            compat_case(
                &vec![Ref(&1)],
                &[
                    Token::SeqBegin(Some(1)),
                    Token::SeqValueBegin,
                    Token::I32(1),
                    Token::SeqValueEnd,
                    Token::SeqEnd,
                ],
            );
        }
    }
}
