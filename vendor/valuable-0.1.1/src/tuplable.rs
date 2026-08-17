use crate::{Valuable, Value, Visit};

use core::fmt;

/// A tuple-like [`Valuable`] sub-type.
///
/// Implemented by [`Valuable`] types that have a tuple-like shape. Fields are
/// always unnamed. Values that implement `Tuplable` must return
/// [`Value::Tuplable`] from their [`Valuable::as_value`] implementation.
///
/// It is uncommon for users to implement this type as the crate provides
/// implementations of `Tuplable` for Rust tuples.
///
/// # Inspecting
///
/// Inspecting fields contained by a `Tuplable` instance is done by visiting the
/// tuple. When visiting a `Tuple`, the `visit_unnamed_fields()` method is
/// called. When the tuple is statically defined, `visit_unnamed_fields()` is
/// called once with the values of all the fields. A dynamic tuple
/// implementation may call `visit_unnamed_fields()` multiple times.
pub trait Tuplable: Valuable {
    /// Returns the tuple's definition.
    ///
    /// See [`TupleDef`] documentation for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::{Tuplable, TupleDef};
    ///
    /// let tuple = (123, "hello");
    ///
    /// if let TupleDef::Static { fields, .. } = tuple.definition() {
    ///     assert_eq!(2, fields);
    /// }
    /// ```
    fn definition(&self) -> TupleDef;
}

/// The number of fields and other tuple-level information.
///
/// Returned by [`Tuplable::definition()`], `TupleDef` provides the caller with
/// information about the tuple's definition.
///
/// This includes the number of fields contained by the tuple.
#[derive(Debug)]
#[non_exhaustive]
pub enum TupleDef {
    /// The tuple is statically-defined, all fields are known ahead of time.
    ///
    /// Static tuple implementations are provided by the crate.
    ///
    /// # Examples
    ///
    /// A statically defined tuple.
    ///
    /// ```
    /// use valuable::{Tuplable, TupleDef};
    ///
    /// let tuple = (123, "hello");
    ///
    /// match tuple.definition() {
    ///     TupleDef::Static { fields, .. } => {
    ///         assert_eq!(2, fields);
    ///     }
    ///     _ => unreachable!(),
    /// };
    /// ```
    #[non_exhaustive]
    Static {
        /// The number of fields contained by the tuple.
        fields: usize,
    },
    /// The tuple is dynamically-defined, not all fields are known ahead of
    /// time.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::{Tuplable, TupleDef, Valuable, Value, Visit};
    ///
    /// struct MyTuple;
    ///
    /// impl Valuable for MyTuple {
    ///     fn as_value(&self) -> Value<'_> {
    ///         Value::Tuplable(self)
    ///     }
    ///
    ///     fn visit(&self, visit: &mut dyn Visit) {
    ///         visit.visit_unnamed_fields(&[Value::I32(123)]);
    ///         visit.visit_unnamed_fields(&[Value::String("hello world")]);
    ///     }
    /// }
    ///
    /// impl Tuplable for MyTuple {
    ///     fn definition(&self) -> TupleDef {
    ///         TupleDef::new_dynamic((1, Some(3)))
    ///     }
    /// }
    /// ```
    #[non_exhaustive]
    Dynamic {
        /// Returns the bounds on the number of tuple fields.
        ///
        /// Specifically, the first element is the lower bound, and the second
        /// element is the upper bound.
        fields: (usize, Option<usize>),
    },
}

macro_rules! deref {
    (
        $(
            $(#[$attrs:meta])*
            $ty:ty,
        )*
    ) => {
        $(
            $(#[$attrs])*
            impl<T: ?Sized + Tuplable> Tuplable for $ty {
                fn definition(&self) -> TupleDef {
                    T::definition(&**self)
                }
            }
        )*
    };
}

deref! {
    &T,
    &mut T,
    #[cfg(feature = "alloc")]
    alloc::boxed::Box<T>,
    #[cfg(feature = "alloc")]
    alloc::rc::Rc<T>,
    #[cfg(not(valuable_no_atomic_cas))]
    #[cfg(feature = "alloc")]
    alloc::sync::Arc<T>,
}

impl Tuplable for () {
    fn definition(&self) -> TupleDef {
        TupleDef::Static { fields: 0 }
    }
}

macro_rules! tuple_impls {
    (
        $( $len:expr => ( $($n:tt $name:ident)+ ) )+
    ) => {
        $(
            impl<$($name),+> Valuable for ($($name,)+)
            where
                $($name: Valuable,)+
            {
                fn as_value(&self) -> Value<'_> {
                    Value::Tuplable(self)
                }

                fn visit(&self, visit: &mut dyn Visit) {
                    visit.visit_unnamed_fields(&[
                        $(
                            self.$n.as_value(),
                        )+
                    ]);
                }
            }

            impl<$($name),+> Tuplable for ($($name,)+)
            where
                $($name: Valuable,)+
            {
                fn definition(&self) -> TupleDef {
                    TupleDef::Static { fields: $len }
                }
            }
        )+
    }
}

tuple_impls! {
    1 => (0 T0)
    2 => (0 T0 1 T1)
    3 => (0 T0 1 T1 2 T2)
    4 => (0 T0 1 T1 2 T2 3 T3)
    5 => (0 T0 1 T1 2 T2 3 T3 4 T4)
    6 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
    7 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
    8 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
    9 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
    10 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
    11 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
    12 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
    13 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
    14 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
    15 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
    16 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}

impl fmt::Debug for dyn Tuplable + '_ {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.definition().is_unit() {
            ().fmt(fmt)
        } else {
            struct DebugTuple<'a, 'b> {
                fmt: fmt::DebugTuple<'a, 'b>,
            }

            impl Visit for DebugTuple<'_, '_> {
                fn visit_unnamed_fields(&mut self, values: &[Value<'_>]) {
                    for value in values {
                        self.fmt.field(value);
                    }
                }

                fn visit_value(&mut self, _: Value<'_>) {
                    unimplemented!()
                }
            }

            let mut debug = DebugTuple {
                fmt: fmt.debug_tuple(""),
            };

            self.visit(&mut debug);
            debug.fmt.finish()
        }
    }
}

impl TupleDef {
    /// Create a new [`TupleDef::Static`] instance
    ///
    /// This should be used when the tuple's fields are fixed and known ahead of time.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::TupleDef;
    ///
    /// let def = TupleDef::new_static(2);
    /// ```
    pub const fn new_static(fields: usize) -> TupleDef {
        TupleDef::Static { fields }
    }

    /// Create a new [`TupleDef::Dynamic`] instance.
    ///
    /// This is used when the tuple's fields may vary at runtime.
    ///
    /// # Examples
    ///
    /// ```
    /// use valuable::TupleDef;
    ///
    /// let def = TupleDef::new_dynamic((2, Some(10)));
    /// ```
    pub const fn new_dynamic(fields: (usize, Option<usize>)) -> TupleDef {
        TupleDef::Dynamic { fields }
    }

    /// Returns `true` if `self` represents the [unit][primitive@unit] tuple.
    ///
    /// # Examples
    ///
    /// With the unit tuple
    ///
    /// ```
    /// use valuable::Tuplable;
    ///
    /// let tuple: &dyn Tuplable = &();
    /// assert!(tuple.definition().is_unit());
    /// ```
    ///
    /// When not the unit tuple.
    ///
    /// ```
    /// use valuable::Tuplable;
    ///
    /// let tuple: &dyn Tuplable = &(123,456);
    /// assert!(!tuple.definition().is_unit());
    /// ```
    pub fn is_unit(&self) -> bool {
        match *self {
            TupleDef::Static { fields } => fields == 0,
            TupleDef::Dynamic { fields } => fields == (0, Some(0)),
        }
    }

    /// Returns `true` if the tuple is [statically defined](TupleDef::Static).
    ///
    /// # Examples
    ///
    /// With a static tuple
    ///
    /// ```
    /// use valuable::TupleDef;
    ///
    /// let def = TupleDef::new_static(2);
    /// assert!(def.is_static());
    /// ```
    ///
    /// With a dynamic tuple
    ///
    /// ```
    /// use valuable::TupleDef;
    ///
    /// let def = TupleDef::new_dynamic((2, None));
    /// assert!(!def.is_static());
    /// ```
    pub fn is_static(&self) -> bool {
        matches!(self, TupleDef::Static { .. })
    }

    /// Returns `true` if the tuple is [dynamically defined](TupleDef::Dynamic).
    ///
    /// # Examples
    ///
    /// With a static tuple
    ///
    /// ```
    /// use valuable::TupleDef;
    ///
    /// let def = TupleDef::new_static(2);
    /// assert!(!def.is_dynamic());
    /// ```
    ///
    /// With a dynamic tuple
    ///
    /// ```
    /// use valuable::TupleDef;
    ///
    /// let def = TupleDef::new_dynamic((2, None));
    /// assert!(def.is_dynamic());
    /// ```
    pub fn is_dynamic(&self) -> bool {
        matches!(self, TupleDef::Dynamic { .. })
    }
}
