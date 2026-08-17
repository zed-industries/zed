use crate::*;

use core::fmt;
use core::iter::FusedIterator;

macro_rules! slice {
    (
        $(
            $(#[$attrs:meta])*
            $variant:ident($ty:ty),
        )*
    ) => {
        /// A slice containing primitive values.
        ///
        /// The `Slice` enum is used to pass multiple primitive-values to the
        /// [visitor][`Visit`]. This is used as an optimization when visiting
        /// [`Listable`] types to avoid a dynamic dispatch call to [`Visit`] for
        /// each element in the collection.
        ///
        /// `Slice` instances are usually not created explicitly. Instead, they
        /// are created when calling [`Valuable::visit_slice()`].
        #[non_exhaustive]
        pub enum Slice<'a> {
            $(
                $(#[$attrs])*
                $variant(&'a [$ty]),
            )*
        }

        /// [`Slice`] iterator
        ///
        /// Instances are created by the [`iter()`][Slice::iter] method on
        /// [`Slice`]. See its documentation for more.
        ///
        /// # Examples
        ///
        /// ```
        /// use valuable::Slice;
        ///
        /// let slice = Slice::U32(&[1, 1, 2, 3, 5]);
        ///
        /// for value in slice.iter() {
        ///     println!("{:?}", value);
        /// }
        /// ```
        #[derive(Debug)]
        pub struct Iter<'a>(IterKind<'a>);

        #[derive(Debug)]
        enum IterKind<'a> {
            $(
                $(#[$attrs])*
                $variant(core::slice::Iter<'a, $ty>),
            )*
        }

        impl<'a> Slice<'a> {
            /// Returns the number of elements in the slice
            ///
            /// # Examples
            ///
            /// ```
            /// use valuable::Slice;
            ///
            /// let slice = Slice::U32(&[1, 1, 2, 3, 5]);
            /// assert_eq!(5, slice.len());
            /// ```
            pub fn len(&self) -> usize {
                #[allow(unused_doc_comments)]
                match self {
                    $(
                        $(#[$attrs])*
                        Slice::$variant(s) => s.len(),
                    )*
                }
            }


            /// Returns `true` if the slice is not empty.
            ///
            /// # Examples
            ///
            /// ```
            /// use valuable::Slice;
            ///
            /// let slice = Slice::U32(&[1, 1, 2, 3, 5]);
            /// assert!(!slice.is_empty());
            /// ```
            /// ```
            /// # use valuable::Slice;
            /// let slice = Slice::U32(&[]);
            /// assert!(slice.is_empty());
            /// ```
            pub fn is_empty(&self) -> bool {
                self.len() == 0
            }

            /// Returns an iterator over the slice.
            ///
            /// # Examples
            ///
            /// ```
            /// use valuable::Slice;
            ///
            /// let slice = Slice::U32(&[1, 1, 2, 3, 5]);
            ///
            /// for value in slice.iter() {
            ///     println!("{:?}", value);
            /// }
            /// ```
            pub fn iter(&self) -> Iter<'a> {
                self.into_iter()
            }
        }

        impl<'a> IntoIterator for Slice<'a> {
            type Item = Value<'a>;
            type IntoIter = Iter<'a>;

            fn into_iter(self) -> Self::IntoIter {
                (&self).into_iter()
            }
        }

        impl<'a> IntoIterator for &'_ Slice<'a> {
            type Item = Value<'a>;
            type IntoIter = Iter<'a>;

            fn into_iter(self) -> Self::IntoIter {
                #[allow(unused_doc_comments)]
                Iter(match self {
                    $(
                        $(#[$attrs])*
                        Slice::$variant(s) => IterKind::$variant(s.iter()),
                    )*
                })
            }
        }

        impl fmt::Debug for Slice<'_> {
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                use Slice::*;

                let mut d = fmt.debug_list();

                #[allow(unused_doc_comments)]
                match *self {
                    $(
                        $(#[$attrs])*
                        $variant(v) => d.entries(v),
                    )*
                };

                d.finish()
            }
        }

        impl<'a> Iterator for Iter<'a> {
            type Item = Value<'a>;

            fn size_hint(&self) -> (usize, Option<usize>) {
                use IterKind::*;

                #[allow(unused_doc_comments)]
                match &self.0 {
                    $(
                        $(#[$attrs])*
                        $variant(v) => v.size_hint(),
                    )*
                }
            }

            fn next(&mut self) -> Option<Value<'a>> {
                use IterKind::*;

                #[allow(unused_doc_comments)]
                match &mut self.0 {
                    $(
                        $(#[$attrs])*
                        $variant(v) => v.next().map(Valuable::as_value),
                    )*
                }
            }
        }

        impl DoubleEndedIterator for Iter<'_> {
            fn next_back(&mut self) -> Option<Self::Item> {
                use IterKind::*;

                #[allow(unused_doc_comments)]
                match &mut self.0 {
                    $(
                        $(#[$attrs])*
                        $variant(v) => v.next_back().map(Valuable::as_value),
                    )*
                }
            }
        }

        impl ExactSizeIterator for Iter<'_> {
            fn len(&self) -> usize {
                use IterKind::*;

                #[allow(unused_doc_comments)]
                match &self.0 {
                    $(
                        $(#[$attrs])*
                        $variant(v) => v.len(),
                    )*
                }
            }
        }

        impl FusedIterator for Iter<'_> {}
    }
}

slice! {
    /// A slice containing `bool` values.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Slice;
    ///
    /// let v = Slice::Bool(&[true, true, false]);
    /// ```
    Bool(bool),

    /// A slice containing `char` values.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Slice;
    ///
    /// let v = Slice::Char(&['a', 'b', 'c']);
    /// ```
    Char(char),

    /// A slice containing `f32` values.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Slice;
    ///
    /// let v = Slice::F32(&[3.1415, 2.71828]);
    /// ```
    F32(f32),

    /// A slice containing `f64` values.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Slice;
    ///
    /// let v = Slice::F64(&[3.1415, 2.71828]);
    /// ```
    F64(f64),

    /// A slice containing `i8` values.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Slice;
    ///
    /// let v = Slice::I8(&[1, 1, 2, 3, 5]);
    /// ```
    I8(i8),

    /// A slice containing `i16` values.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Slice;
    ///
    /// let v = Slice::I16(&[1, 1, 2, 3, 5]);
    /// ```
    I16(i16),

    /// A slice containing `I32` values.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Slice;
    ///
    /// let v = Slice::I32(&[1, 1, 2, 3, 5]);
    /// ```
    I32(i32),

    /// A slice containing `I64` values.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Slice;
    ///
    /// let v = Slice::I64(&[1, 1, 2, 3, 5]);
    /// ```
    I64(i64),

    /// A slice containing `I128` values.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Slice;
    ///
    /// let v = Slice::I128(&[1, 1, 2, 3, 5]);
    /// ```
    I128(i128),

    /// A slice containing `isize` values.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Slice;
    ///
    /// let v = Slice::Isize(&[1, 1, 2, 3, 5]);
    /// ```
    Isize(isize),

    /// A slice containing `str` values.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Slice;
    ///
    /// let v = Slice::Str(&["foo", "bar", "baz"]);
    /// ```
    Str(&'a str),

    /// A slice containing `String` values.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Slice;
    ///
    /// let v = Slice::String(&["foo".to_string(), "bar".to_string()]);
    /// ```
    #[cfg(feature = "alloc")]
    String(alloc::string::String),

    /// A slice containing `u8` values.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Slice;
    ///
    /// let v = Slice::U8(&[1, 1, 2, 3, 5]);
    /// ```
    U8(u8),

    /// A slice containing `u16` values.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Slice;
    ///
    /// let v = Slice::U16(&[1, 1, 2, 3, 5]);
    /// ```
    U16(u16),

    /// A slice containing `u32` values.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Slice;
    ///
    /// let v = Slice::U32(&[1, 1, 2, 3, 5]);
    /// ```
    U32(u32),

    /// A slice containing `u64` values.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Slice;
    ///
    /// let v = Slice::U64(&[1, 1, 2, 3, 5]);
    /// ```
    U64(u64),

    /// A slice containing `u128` values.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Slice;
    ///
    /// let v = Slice::U128(&[1, 1, 2, 3, 5]);
    /// ```
    U128(u128),

    /// A slice containing `usize` values.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Slice;
    ///
    /// let v = Slice::Usize(&[1, 1, 2, 3, 5]);
    /// ```
    Usize(usize),

    /// A slice containing `()` values.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::Slice;
    ///
    /// let v = Slice::Unit(&[(), (), ()]);
    /// ```
    Unit(()),
}
