//! A crate for creating safe self-referencing structs.
//!
//! See the documentation of [`ouroboros_examples`](https://docs.rs/ouroboros_examples) for
//! sample documentation of structs which have had the macro applied to them.

#![no_std]
#![allow(clippy::needless_doctest_main)]

/// This macro is used to turn a regular struct into a self-referencing one. An example:
/// ```rust
/// use ouroboros::self_referencing;
///
/// #[self_referencing]
/// struct MyStruct {
///     int_data: i32,
///     float_data: f32,
///     #[borrows(int_data)]
///     // the 'this lifetime is created by the #[self_referencing] macro
///     // and should be used on all references marked by the #[borrows] macro
///     int_reference: &'this i32,
///     #[borrows(mut float_data)]
///     float_reference: &'this mut f32,
/// }
///
/// fn main() {
///     // The builder is created by the #[self_referencing] macro
///     // and is used to create the struct
///     let mut my_value = MyStructBuilder {
///         int_data: 42,
///         float_data: 3.14,
///
///         // Note that the name of the field in the builder
///         // is the name of the field in the struct + `_builder`
///         // ie: {field_name}_builder
///         // the closure that assigns the value for the field will be passed
///         // a reference to the field(s) defined in the #[borrows] macro
///
///         int_reference_builder: |int_data: &i32| int_data,
///         float_reference_builder: |float_data: &mut f32| float_data,
///     }.build();
///
///     // The fields in the original struct can not be accessed directly
///     // The builder creates accessor methods which are called borrow_{field_name}()
///
///     // Prints 42
///     println!("{:?}", my_value.borrow_int_data());
///     // Prints 3.14
///     println!("{:?}", my_value.borrow_float_reference());
///     // Sets the value of float_data to 84.0
///     my_value.with_mut(|fields| {
///         **fields.float_reference = (**fields.int_reference as f32) * 2.0;
///     });
///
///     // We can hold on to this reference...
///     let int_ref = *my_value.borrow_int_reference();
///     println!("{:?}", *int_ref);
///     // As long as the struct is still alive.
///     drop(my_value);
///     // This will cause an error!
///     // println!("{:?}", *int_ref);
/// }
/// ```
/// To explain the features and limitations of this crate, some definitions are necessary:
/// # Definitions
/// - **immutably borrowed field**: a field which is immutably borrowed by at least one other field.
/// - **mutably borrowed field**: a field which is mutably borrowed by exactly one other field.
/// - **self-referencing field**: a field which borrows at least one other field.
/// - **head field**: a field which does not borrow any other fields, I.E. not self-referencing.
///   This does not include fields with empty borrows annotations (`#[borrows()]`.)
/// - **tail field**: a field which is not borrowed by any other fields.
///
/// # Usage
/// To make a self-referencing struct, you must write a struct definition and place
/// `#[self_referencing]` on top. For every field that borrows other fields, you must place
/// `#[borrows()]` on top and place inside the parenthesis a list of fields that it borrows. Mut can
/// be prefixed to indicate that a mutable borrow is required. For example,
/// `#[borrows(a, b, mut c)]` indicates that the first two fields need to be borrowed immutably and
/// the third needs to be borrowed mutably. You can also use `#[borrows()]` without any arguments to
/// indicate a field that will eventually borrow from the struct, but does not borrow anything when
/// first created. For example, you could use this on a field like `error: Option<&'this str>`.
///
/// # You must comply with these limitations
/// - Fields must be declared before the first time they are borrowed.
/// - Normal borrowing rules apply, E.G. a field cannot be borrowed mutably twice.
/// - Fields that use the `'this` lifetime must have a corresponding `#[borrows()]` annotation.
///   The error for this needs some work, currently you will get an error saying that `'this` is
///   undefined at the location it was illegally used in.
///
/// Violating them will result in an error message directly pointing out the violated rule.
///
/// # Flexibility of this crate
/// The example above uses plain references as the self-referencing part of the struct, but you can
/// use anything that is dependent on lifetimes of objects inside the struct. For example, you could
/// do something like this:
/// ```rust
/// use ouroboros::self_referencing;
///
/// pub struct ComplexData<'a, 'b> {
///     aref: &'a i32,
///     bref: &'b mut i32,
///     number: i32,
/// }
///
/// impl<'a, 'b> ComplexData<'a, 'b> {
///     fn new(aref: &'a i32, bref: &'b mut i32, number: i32) -> Self {
///         Self { aref, bref, number }
///     }
///
///     /// Copies the value aref points to into what bref points to.
///     fn transfer(&mut self) {
///         *self.bref = *self.aref;
///     }
///
///     /// Prints the value bref points to.
///     fn print_bref(&self) {
///         println!("{}", *self.bref);
///     }
/// }
///
/// fn main() {
///     #[self_referencing]
///     struct DataStorage {
///         immutable: i32,
///         mutable: i32,
///         #[borrows(immutable, mut mutable)]
///         #[not_covariant]
///         complex_data: ComplexData<'this, 'this>,
///     }
///
///     let mut data_storage = DataStorageBuilder {
///         immutable: 10,
///         mutable: 20,
///         complex_data_builder: |i: &i32, m: &mut i32| ComplexData::new(i, m, 12345),
///     }.build();
///     data_storage.with_complex_data_mut(|data| {
///         // Copies the value in immutable into mutable.
///         data.transfer();
///         // Prints 10
///         data.print_bref();
///     });
/// }
/// ```
///
/// # Covariance
/// Many types in Rust have a property called "covariance". In practical tearms, this means that a
/// covariant type like `Box<&'this i32>` can be used as a `Box<&'a i32>` as long as `'a` is
/// smaller than `'this`. Since the lifetime is smaller, it does not violate the lifetime specified
/// by the original type. Contrast this to `Fn(&'this i32)`, which is not covariant. You cannot give
/// this function a reference with a lifetime shorter than `'this` as the function needs something
/// that lives at *least* as long as `'this`. Unfortunately, there is no easy way to determine
/// whether or not a type is covariant from inside the macro. As such, you may
/// receive a compiler error letting you know that the macro is uncertain if a particular field
/// uses a covariant type. Adding `#[covariant]` or `#[not_covariant]` will resolve this issue.
///
/// These annotations control whether or not a `borrow_*` method is generated for that field.
/// Incorrectly using one of these tags will result in a compilation error. It is impossible to
/// use them unsoundly.
///
/// # Async usage
/// All self-referencing structs can be initialized asynchronously by using either the
/// `MyStruct::new_async()` function or the `MyStructAsyncBuilder` builder. Due to limitations of
/// the rust compiler you closures must return a Future trait object wrapped in a `Pin<Box<_>>`.
///
/// Here is the same example as above in its async version:
///
/// ```ignore
/// use ouroboros::self_referencing;
///
/// #[self_referencing]
/// struct MyStruct {
///     int_data: i32,
///     float_data: f32,
///     #[borrows(int_data)]
///     int_reference: &'this i32,
///     #[borrows(mut float_data)]
///     float_reference: &'this mut f32,
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let mut my_value = MyStructAsyncBuilder {
///         int_data: 42,
///         float_data: 3.14,
///         int_reference_builder: |int_data: &i32| Box::pin(async move { int_data }),
///         float_reference_builder: |float_data: &mut f32| Box::pin(async move { float_data }),
///     }.build().await;
///
///     // Prints 42
///     println!("{:?}", my_value.borrow_int_data());
///     // Prints 3.14
///     println!("{:?}", my_value.borrow_float_reference());
///     // Sets the value of float_data to 84.0
///     my_value.with_mut(|fields| {
///         **fields.float_reference = (**fields.int_reference as f32) * 2.0;
///     });
///
///     // We can hold on to this reference...
///     let int_ref = *my_value.borrow_int_reference();
///     println!("{:?}", *int_ref);
///     // As long as the struct is still alive.
///     drop(my_value);
///     // This will cause an error!
///     // println!("{:?}", *int_ref);
/// }
/// ```
///
/// # Async Send
/// When Send trait is needed, the Send variant of async methods and builders is available.
///
/// Here is the same example as above in its async send version:
///
/// ```ignore
/// use ouroboros::self_referencing;
///
/// #[self_referencing]
/// struct MyStruct {
///     int_data: i32,
///     float_data: f32,
///     #[borrows(int_data)]
///     int_reference: &'this i32,
///     #[borrows(mut float_data)]
///     float_reference: &'this mut f32,
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let mut my_value = MyStructAsyncSendBuilder {
///         int_data: 42,
///         float_data: 3.14,
///         int_reference_builder: |int_data: &i32| Box::pin(async move { int_data }),
///         float_reference_builder: |float_data: &mut f32| Box::pin(async move { float_data }),
///     }.build().await;
///
///     // Prints 42
///     println!("{:?}", my_value.borrow_int_data());
///     // Prints 3.14
///     println!("{:?}", my_value.borrow_float_reference());
///     // Sets the value of float_data to 84.0
///     my_value.with_mut(|fields| {
///         **fields.float_reference = (**fields.int_reference as f32) * 2.0;
///     });
///
///     // We can hold on to this reference...
///     let int_ref = *my_value.borrow_int_reference();
///     println!("{:?}", *int_ref);
///     // As long as the struct is still alive.
///     drop(my_value);
///     // This will cause an error!
///     // println!("{:?}", *int_ref);
/// }
/// ```
///
/// # What does the macro generate?
/// The `#[self_referencing]` struct will replace your definition with an unsafe self-referencing
/// struct with a safe public interface. Many functions will be generated depending on your original
/// struct definition. Documentation is generated for all items, so building documentation for
/// your project allows accessing detailed information about available functions. Using
/// `#[self_referencing(no_doc)]` will hide the generated items from documentation if it is becoming
/// too cluttered.
///
/// ### A quick note on visibility
/// The visibility of generated items is dependent on one of two things. If the
/// generated item is related to a specific field of the struct, it uses the visibility of the
/// original field. (The actual field in the struct will be made private since accessing it could cause
/// undefined behavior.) If the generated item is not related to any particular field, it will by
/// default only be visible to the module the struct is declared in. (This includes things like
/// `new()` and `with()`.) You can use `#[self_referencing(pub_extras)]` to make these items have the
/// same visibility as the struct itself.
///
/// # List of generated items
/// ### `MyStruct::new(fields...) -> MyStruct`
/// A basic constructor. It accepts values for each field in the order you declared them in. For
/// **head fields**, you only need to pass in what value it should have and it will be moved in
/// to the output. For **self-referencing fields**, you must provide a function or closure which creates
/// the value based on the values it borrows. A field using the earlier example of
/// `#[borrow(a, b, mut c)]` would require a function typed as
/// `FnOnce(a: &_, b: &_, c: &mut _) -> _`. Fields which have an empty borrows annotation
/// (`#[borrows()]`) should have their value directly passed in. A field using the earlier example
/// of `Option<&'this str>` would require an input of `None`. Do not pass a function. Do not collect
/// 200 dollars.
/// ### `MyStruct::new_async(fields...) -> MyStruct`
/// A basic async constructor. It works identically to the sync constructor differing only in the
/// type of closures it expects. Whenever a closure is required it is expected to return a Pinned
/// and Boxed Future that Outputs the same type as the synchronous version.
/// ### `MyStruct::new_async_send(fields...) -> MyStruct`
/// An async send constructor. It works identically to the sync constructor differing only in the
/// Send trait being specified in the return type.
/// ### `MyStructBuilder`
/// This is the preferred way to create a new instance of your struct. It is similar to using the
/// `MyStruct { a, b, c, d }` syntax instead of `MyStruct::new(a, b, c, d)`. It contains one field
/// for every argument in the actual constructor. **Head fields** have the same name that you
/// originally defined them with. **self-referencing fields** are suffixed with `_builder` since you need
/// to provide a function instead of a value. Fields with an empty borrows annotation are not
/// initialized using builders. Calling `.build()` on an instance of `MyStructBuilder`
/// will convert it to an instance of `MyStruct` by calling all `_builder` functions in the order that
/// they were declared and storing their results.
/// ### `MyStructAsyncBuilder`
/// This is the preferred way to asynchronously create a new instance of your struct. It works
/// identically to the synchronous builder differing only in the type of closures it expects. In
/// particular, all builder functions are called serially in the order that they were declared.
/// Whenever a closure is required it is expected to return a Pinned and Boxed Future that Outputs
/// the same type as the synchronous version.
/// ### `MyStructAsyncSendBuilder`
/// Same as MyStructAsyncBuilder, but with Send trait specified in the return type.
/// ### `MyStruct::try_new<E>(fields...) -> Result<MyStruct, E>`
/// Similar to the regular `new()` function, except the functions which create values for all
/// **self-referencing fields** can return `Result<>`s. If any of those are `Err`s, that error will be
/// returned instead of an instance of `MyStruct`. The preferred way to use this function is through
/// `MyStructTryBuilder` and its `try_build()` function.
/// ### `MyStruct::try_new_async<E>(fields...) -> Result<MyStruct, E>`
/// Similar to the regular `new_async()` function, except the functions which create values for all
/// **self-referencing fields** can return `Result<>`s. If any of those are `Err`s, that error will be
/// returned instead of an instance of `MyStruct`. The preferred way to use this function is through
/// `MyStructAsyncTryBuilder` and its `try_build()` function.
/// ### `MyStruct::try_new_async_send<E>(fields...) -> Result<MyStruct, E>`
/// Same as `new_async()` function, but with Send trait specified in the return type.
/// ### `MyStruct::try_new_or_recover_async<E>(fields...) -> Result<MyStruct, (E, Heads)>`
/// Similar to the `try_new_async()` function, except that all the **head fields** are returned along side
/// the original error in case of an error. The preferred way to use this function is through
/// `MyStructAsyncTryBuilder` and its `try_build_or_recover()` function.
/// ### `MyStruct::try_new_or_recover_async_send<E>(fields...) -> Result<MyStruct, (E, Heads)>`
/// Same as `try_new_or_recover_async()` function, but with Send trait specified in the return type.
/// ### `MyStruct::with_FIELD<R>(&self, user: FnOnce(field: &FieldType) -> R) -> R`
/// This function is generated for every **tail and immutably-borrowed field** in your struct. It
/// allows safely accessing
/// a reference to that value. The function generates the reference and passes it to `user`. You
/// can do anything you want with the reference, it is constructed to not outlive the struct.
/// ### `MyStruct::borrow_FIELD(&self) -> &FieldType`
/// This function is generated for every **tail and immutably-borrowed field** in your struct. It
/// is equivalent to calling `my_struct.with_FIELD(|field| field)`. It is only generated for types
/// which are known to be covariant, either through the macro being able to detect it or through the
/// programmer adding the `#[covariant]` annotation to the field.
/// There is no `borrow_FIELD_mut`, unfortunately, as Rust's
/// borrow checker is currently not capable of ensuring that such a method would be used safely.
/// ### `MyStruct::with_FIELD_mut<R>(&mut self, user: FnOnce(field: &mut FieldType) -> R) -> R`
/// This function is generated for every **tail field** in your struct. It is the mutable version
/// of `with_FIELD`.
/// ### `MyStruct::with<R>(&self, user: FnOnce(fields: AllFields) -> R) -> R`
/// Allows borrowing all **tail and immutably-borrowed fields** at once. Functions similarly to
/// `with_FIELD`.
/// ### `MyStruct::with_mut<R>(&self, user: FnOnce(fields: AllFields) -> R) -> R`
/// Allows mutably borrowing all **tail fields** and immutably borrowing all **immutably-borrowed**
/// fields at once. Functions similarly to `with_FIELD_mut`, except that you can borrow multiple
/// fields as mutable at the same time and also have immutable access to any remaining fields.
/// ### `MyStruct::into_heads(self) -> Heads`
/// Drops all self-referencing fields and returns a struct containing all **head fields**.
pub use ouroboros_macro::self_referencing;

#[doc(hidden)]
pub mod macro_help {
    pub extern crate alloc;

    pub use aliasable::boxed::AliasableBox;
    pub use static_assertions::assert_impl_all;
    use aliasable::boxed::UniqueBox;

    pub struct CheckIfTypeIsStd<T>(core::marker::PhantomData<T>);

    macro_rules! std_type_check {
        ($fn_name:ident $T:ident $check_for:ty) => {
            impl<$T: ?Sized> CheckIfTypeIsStd<$check_for> {
                pub fn $fn_name() {}
            }
        };
    }

    std_type_check!(is_std_box_type T alloc::boxed::Box<T>);
    #[cfg(target_has_atomic = "ptr")]
    std_type_check!(is_std_arc_type T alloc::sync::Arc<T>);
    std_type_check!(is_std_rc_type T alloc::rc::Rc<T>);

    pub fn aliasable_boxed<T>(data: T) -> AliasableBox<T> {
        AliasableBox::from_unique(UniqueBox::new(data))
    }

    pub fn unbox<T>(boxed: AliasableBox<T>) -> T {
        *AliasableBox::into_unique(boxed)
    }

    /// Converts a reference to an object to a static reference This is
    /// obviously unsafe because the compiler can no longer guarantee that the
    /// data outlives the reference.  It is up to the consumer to get rid of the
    /// reference before the container is dropped. The + 'static ensures that
    /// whatever we are referring to will remain valid indefinitely, that there
    /// are no limitations on how long the pointer itself can live.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the returned reference is not used after the originally passed
    /// reference would become invalid.
    pub unsafe fn change_lifetime<'old, 'new: 'old, T: 'new>(data: &'old T) -> &'new T {
        &*(data as *const _)
    }

    /// Like change_lifetime, but for mutable references.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the returned reference is not used after the originally passed
    /// reference would become invalid.
    pub unsafe fn change_lifetime_mut<'old, 'new: 'old, T: 'new>(data: &'old mut T) -> &'new mut T {
        &mut *(data as *mut _)
    }
}
