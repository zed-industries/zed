use crate::*;

/// Traverse a value's fields and variants.
///
/// Each method of the `Visit` trait is a hook that enables the implementor to
/// observe value fields. By default, most methods are implemented as a no-op.
/// The `visit_primitive_slice` default implementation will iterate the slice,
/// calling `visit_value` with each item.
///
/// To recurse, the implementor must implement methods to visit the arguments.
///
/// # Examples
///
/// Recursively printing a Rust value.
///
/// ```
/// use valuable::{NamedValues, Valuable, Value, Visit};
///
/// struct Print(String);
///
/// impl Print {
///     fn indent(&self) -> Print {
///        Print(format!("{}    ", self.0))
///     }
/// }
///
/// impl Visit for Print {
///     fn visit_value(&mut self, value: Value<'_>) {
///         match value {
///             Value::Structable(v) => {
///                 let def = v.definition();
///                 // Print the struct name
///                 println!("{}{}:", self.0, def.name());
///
///                 // Visit fields
///                 let mut visit = self.indent();
///                 v.visit(&mut visit);
///             }
///             Value::Enumerable(v) => {
///                 let def = v.definition();
///                 let variant = v.variant();
///                 // Print the enum name
///                 println!("{}{}::{}:", self.0, def.name(), variant.name());
///
///                 // Visit fields
///                 let mut visit = self.indent();
///                 v.visit(&mut visit);
///             }
///             Value::Listable(v) => {
///                 println!("{}", self.0);
///
///                 // Visit fields
///                 let mut visit = self.indent();
///                 v.visit(&mut visit);
///             }
///             Value::Mappable(v) => {
///                 println!("{}", self.0);
///
///                 // Visit fields
///                 let mut visit = self.indent();
///                 v.visit(&mut visit);
///             }
///             // Primitive or unknown type, just render Debug
///             v => println!("{:?}", v),
///         }
///     }
///
///     fn visit_named_fields(&mut self, named_values: &NamedValues<'_>) {
///         for (field, value) in named_values {
///             print!("{}- {}: ", self.0, field.name());
///             value.visit(self);
///         }
///     }
///
///     fn visit_unnamed_fields(&mut self, values: &[Value<'_>]) {
///         for value in values {
///             print!("{}- ", self.0);
///             value.visit(self);
///         }
///     }
///
///     fn visit_entry(&mut self, key: Value<'_>, value: Value<'_>) {
///         print!("{}- {:?}: ", self.0, key);
///         value.visit(self);
///     }
/// }
///
/// #[derive(Valuable)]
/// struct Person {
///     name: String,
///     age: u32,
///     addresses: Vec<Address>,
/// }
///
/// #[derive(Valuable)]
/// struct Address {
///     street: String,
///     city: String,
///     zip: String,
/// }
///
/// let person = Person {
///     name: "Angela Ashton".to_string(),
///     age: 31,
///     addresses: vec![
///         Address {
///             street: "123 1st Ave".to_string(),
///             city: "Townsville".to_string(),
///             zip: "12345".to_string(),
///         },
///         Address {
///             street: "555 Main St.".to_string(),
///             city: "New Old Town".to_string(),
///             zip: "55555".to_string(),
///         },
///     ],
/// };
///
/// let mut print = Print("".to_string());
/// valuable::visit(&person, &mut print);
/// ```
pub trait Visit {
    /// Visit a single value.
    ///
    /// The `visit_value` method is called once when visiting single primitive
    /// values. When visiting `Listable` types, the `visit_value` method is
    /// called once per item in the listable type.
    ///
    /// Note, in the case of Listable types containing primitive types,
    /// `visit_primitive_slice` can be implemented instead for less overhead.
    ///
    /// # Examples
    ///
    /// Visiting a single value.
    ///
    /// ```
    /// use valuable::{Valuable, Visit, Value};
    ///
    /// struct Print;
    ///
    /// impl Visit for Print {
    ///     fn visit_value(&mut self, value: Value<'_>) {
    ///         println!("{:?}", value);
    ///     }
    /// }
    ///
    /// let my_val = 123;
    /// my_val.visit(&mut Print);
    /// ```
    ///
    /// Visiting multiple values in a list.
    ///
    /// ```
    /// use valuable::{Valuable, Value, Visit};
    ///
    /// struct PrintList { comma: bool };
    ///
    /// impl Visit for PrintList {
    ///     fn visit_value(&mut self, value: Value<'_>) {
    ///         match value {
    ///             Value::Listable(v) => v.visit(self),
    ///             value => {
    ///                 if self.comma {
    ///                     println!(", {:?}", value);
    ///                 } else {
    ///                     print!("{:?}", value);
    ///                     self.comma = true;
    ///                 }
    ///             }
    ///         }
    ///     }
    /// }
    ///
    /// let my_list = vec![1, 2, 3, 4, 5];
    /// valuable::visit(&my_list, &mut PrintList { comma: false });
    /// ```
    fn visit_value(&mut self, value: Value<'_>);

    /// Visit a struct or enum's named fields.
    ///
    /// When the struct/enum is statically defined, all fields are known ahead
    /// of time and `visit_named_fields` is called once with all field values.
    /// When the struct/enum is dynamic, then the `visit_named_fields` method
    /// may be called multiple times.
    ///
    /// See [`Structable`] and [`Enumerable`] for static vs. dynamic details.
    ///
    /// # Examples
    ///
    /// Visiting all fields in a struct.
    ///
    /// ```
    /// use valuable::{NamedValues, Valuable, Value, Visit};
    ///
    /// #[derive(Valuable)]
    /// struct MyStruct {
    ///     hello: String,
    ///     world: u32,
    /// }
    ///
    /// struct Print;
    ///
    /// impl Visit for Print {
    ///     fn visit_named_fields(&mut self, named_values: &NamedValues<'_>) {
    ///         for (field, value) in named_values {
    ///             println!("{:?}: {:?}", field, value);
    ///         }
    ///     }
    ///
    ///     fn visit_value(&mut self, value: Value<'_>) {
    ///         match value {
    ///             Value::Structable(v) => v.visit(self),
    ///             _ => {} // do nothing for other types
    ///         }
    ///     }
    /// }
    ///
    /// let my_struct = MyStruct {
    ///     hello: "Hello world".to_string(),
    ///     world: 42,
    /// };
    ///
    /// valuable::visit(&my_struct, &mut Print);
    /// ```
    fn visit_named_fields(&mut self, named_values: &NamedValues<'_>) {
        let _ = named_values;
    }

    /// Visit a struct or enum's unnamed fields.
    ///
    /// When the struct/enum is statically defined, all fields are known ahead
    /// of time and `visit_unnamed_fields` is called once with all field values.
    /// When the struct/enum is dynamic, then the `visit_unnamed_fields` method
    /// may be called multiple times.
    ///
    /// See [`Structable`] and [`Enumerable`] for static vs. dynamic details.
    ///
    /// # Examples
    ///
    /// Visiting all fields in a struct.
    ///
    /// ```
    /// use valuable::{Valuable, Value, Visit};
    ///
    /// #[derive(Valuable)]
    /// struct MyStruct(String, u32);
    ///
    /// struct Print;
    ///
    /// impl Visit for Print {
    ///     fn visit_unnamed_fields(&mut self, values: &[Value<'_>]) {
    ///         for value in values {
    ///             println!("{:?}", value);
    ///         }
    ///     }
    ///
    ///     fn visit_value(&mut self, value: Value<'_>) {
    ///         match value {
    ///             Value::Structable(v) => v.visit(self),
    ///             _ => {} // do nothing for other types
    ///         }
    ///     }
    /// }
    ///
    /// let my_struct = MyStruct("Hello world".to_string(), 42);
    ///
    /// valuable::visit(&my_struct, &mut Print);
    /// ```
    fn visit_unnamed_fields(&mut self, values: &[Value<'_>]) {
        let _ = values;
    }

    /// Visit a primitive slice.
    ///
    /// This method exists as an optimization when visiting [`Listable`] types.
    /// By default, `Listable` types are visited by passing each item to
    /// `visit_value`. However, if the listable stores a **primitive** type
    /// within contiguous memory, then `visit_primitive_slice` is called
    /// instead.
    ///
    /// When implementing `visit_primitive_slice`, be aware that the method may
    /// be called multiple times for a single `Listable` type.
    ///
    /// # Examples
    ///
    /// A vec calls `visit_primitive_slice` one time, but a `VecDeque` will call
    /// `visit_primitive_slice` twice.
    ///
    /// ```
    /// use valuable::{Valuable, Value, Visit, Slice};
    /// use std::collections::VecDeque;
    ///
    /// struct Count(u32);
    ///
    /// impl Visit for Count {
    ///     fn visit_primitive_slice(&mut self, slice: Slice<'_>) {
    ///         self.0 += 1;
    ///     }
    ///
    ///     fn visit_value(&mut self, value: Value<'_>) {
    ///         match value {
    ///             Value::Listable(v) => v.visit(self),
    ///             _ => {} // do nothing for other types
    ///         }
    ///     }
    /// }
    ///
    /// let vec = vec![1, 2, 3, 4, 5];
    ///
    /// let mut count = Count(0);
    /// valuable::visit(&vec, &mut count);
    /// assert_eq!(1, count.0);
    ///
    /// let mut vec_deque = VecDeque::from(vec);
    ///
    /// let mut count = Count(0);
    /// valuable::visit(&vec_deque, &mut count);
    ///
    /// assert_eq!(2, count.0);
    /// ```
    fn visit_primitive_slice(&mut self, slice: Slice<'_>) {
        for value in slice {
            self.visit_value(value);
        }
    }

    /// Visit a `Mappable`'s entries.
    ///
    /// The `visit_entry` method is called once for each entry contained by a
    /// `Mappable.`
    ///
    /// # Examples
    ///
    /// Visit a map's entries
    ///
    /// ```
    /// use valuable::{Valuable, Value, Visit};
    /// use std::collections::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("hello", 123);
    /// map.insert("world", 456);
    ///
    /// struct Print;
    ///
    /// impl Visit for Print {
    ///     fn visit_entry(&mut self, key: Value<'_>, value: Value<'_>) {
    ///         println!("{:?} => {:?}", key, value);
    ///     }
    ///
    ///     fn visit_value(&mut self, value: Value<'_>) {
    ///         match value {
    ///             Value::Mappable(v) => v.visit(self),
    ///             _ => {} // do nothing for other types
    ///         }
    ///     }
    /// }
    ///
    /// valuable::visit(&map, &mut Print);
    /// ```
    fn visit_entry(&mut self, key: Value<'_>, value: Value<'_>) {
        let _ = (key, value);
    }
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
            impl<T: ?Sized + Visit> Visit for $ty {
                fn visit_value(&mut self, value: Value<'_>) {
                    T::visit_value(&mut **self, value)
                }

                fn visit_named_fields(&mut self, named_values: &NamedValues<'_>) {
                    T::visit_named_fields(&mut **self, named_values)
                }

                fn visit_unnamed_fields(&mut self, values: &[Value<'_>]) {
                    T::visit_unnamed_fields(&mut **self, values)
                }

                fn visit_primitive_slice(&mut self, slice: Slice<'_>) {
                    T::visit_primitive_slice(&mut **self, slice)
                }

                fn visit_entry(&mut self, key: Value<'_>, value: Value<'_>) {
                    T::visit_entry(&mut **self, key, value)
                }
            }
        )*
    };
}

deref! {
    &mut T,
    #[cfg(feature = "alloc")]
    alloc::boxed::Box<T>,
}

/// Inspects a value by calling the relevant [`Visit`] methods with `value`'s
/// data.
///
/// This method calls [`Visit::visit_value()`] with the provided [`Valuable`]
/// instance. See [`Visit`] documentation for more details.
///
/// # Examples
///
/// Extract a single field from a struct. Note: if the same field is repeatedly
/// extracted from a struct, it is preferable to obtain the associated
/// [`NamedField`] once and use it repeatedly.
///
/// ```
/// use valuable::{NamedValues, Valuable, Value, Visit};
///
/// #[derive(Valuable)]
/// struct MyStruct {
///     foo: usize,
///     bar: usize,
/// }
///
/// struct GetFoo(usize);
///
/// impl Visit for GetFoo {
///     fn visit_named_fields(&mut self, named_values: &NamedValues<'_>) {
///         if let Some(foo) = named_values.get_by_name("foo") {
///             if let Some(val) = foo.as_usize() {
///                 self.0 = val;
///             }
///         }
///     }
///
///     fn visit_value(&mut self, value: Value<'_>) {
///         if let Value::Structable(v) = value {
///             v.visit(self);
///         }
///     }
/// }
///
/// let my_struct = MyStruct {
///     foo: 123,
///     bar: 456,
/// };
///
/// let mut get_foo = GetFoo(0);
/// valuable::visit(&my_struct, &mut get_foo);
///
/// assert_eq!(123, get_foo.0);
/// ```
///
/// [`Visit`]: Visit [`NamedField`]: crate::NamedField
pub fn visit(value: &impl Valuable, visit: &mut dyn Visit) {
    visit.visit_value(value.as_value());
}
