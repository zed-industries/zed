/// For implementing debug or display formatting "manually".
///
/// # Generated code
///
/// This macro generates:
///
/// - An implementation of the [`FormatMarker`] trait for all the `impl`d types,
///
/// - All the listed impls, by repeating the methods (and other associated items)
/// passed to this macro in each of the impls.
///
/// # Example
///
/// ### Generic type
///
/// This demonstrates how you can implement debug formatting for a generic struct.
///
/// ```rust
///
/// use const_format::{Error, Formatter, PWrapper, StrWriter};
/// use const_format::{formatc, impl_fmt, try_};
///
/// use std::marker::PhantomData;
///
/// pub struct Tupled<T>(u32, T);
///
/// // Implements debug formatting for:
/// // - Tupled<PhantomData<T>>
/// // - Tupled<bool>
/// // - Tupled<Option<bool>>
/// // Repeating the `const_debug_fmt` function definition in each of those 3 impls.
/// impl_fmt!{
///     // The trailing comma is required
///     impl[T,] Tupled<PhantomData<T>>
///     where[ T: 'static ];
///
///     impl[] Tupled<bool>;
///     impl Tupled<Option<bool>>;
///     
///     pub const fn const_debug_fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
///         let mut fmt = fmt.debug_tuple("Tupled");
///
///         // PWrapper implements const_debug_fmt methods for many std types.
///         //
///         // You can use `call_debug_fmt` for formatting generic std types
///         // if this doesn't work
///         try_!(PWrapper(self.0).const_debug_fmt(fmt.field()));
///         try_!(PWrapper(self.1).const_debug_fmt(fmt.field()));
///
///         fmt.finish()
///     }
/// }
///
/// const S_PHANTOM: &str = formatc!("{:?}", Tupled(3, PhantomData::<u32>));
/// const S_BOOL: &str = formatc!("{:?}", Tupled(5, false));
/// const S_OPTION: &str = formatc!("{:?}", Tupled(8, Some(true)));
///
/// assert_eq!(S_PHANTOM, "Tupled(3, PhantomData)");
/// assert_eq!(S_BOOL, "Tupled(5, false)");
/// assert_eq!(S_OPTION, "Tupled(8, Some(true))");
///
///
/// ```
///
/// ### Enum
///
/// This demonstrates how you can implement debug formatting for an enum,
/// using this macro purely for implementing the [`FormatMarker`] trait.
///
/// ```rust
///
/// use const_format::{Error, Formatter, PWrapper, StrWriter};
/// use const_format::{formatc, impl_fmt, try_};
///
/// use std::cmp::Ordering;
///
/// pub enum Enum {
///     Braced{ord: Ordering},
///     Tupled(u32, u32),
///     Unit,
/// }
///
/// impl_fmt!{
///     impl Enum;
///     
///     pub const fn const_debug_fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
///         match self {
///             Self::Braced{ord} => {
///                 let mut fmt = fmt.debug_struct("Braced");
///
///                 // PWrapper implements const_debug_fmt methods for many std types.
///                 //
///                 // You can use `call_debug_fmt` for formatting generic std types
///                 // if this doesn't work
///                 try_!(PWrapper(*ord).const_debug_fmt(fmt.field("ord")));
///
///                 fmt.finish()
///             }
///             Self::Tupled(f0,f1) => {
///                 let mut fmt = fmt.debug_tuple("Tupled");
///
///                 try_!(PWrapper(*f0).const_debug_fmt(fmt.field()));
///                 try_!(PWrapper(*f1).const_debug_fmt(fmt.field()));
///
///                 fmt.finish()
///             }
///             Self::Unit => {
///                 fmt.debug_tuple("Unit").finish()
///             }
///         }
///     }
/// }
///
/// const S_BRACED: &str = formatc!("{:?}", Enum::Braced{ord: Ordering::Greater});
/// const S_TUPLED: &str = formatc!("{:?}", Enum::Tupled(5, 8));
/// const S_UNIT: &str = formatc!("{:?}", Enum::Unit);
///
/// assert_eq!(S_BRACED, "Braced { ord: Greater }");
/// assert_eq!(S_TUPLED, "Tupled(5, 8)");
/// assert_eq!(S_UNIT, "Unit");
///
/// ```
///
/// [`FormatMarker`]: ./marker_traits/trait.FormatMarker.html
///
#[cfg_attr(feature = "__docsrs", doc(cfg(feature = "fmt")))]
#[macro_export]
macro_rules! impl_fmt {
    (
        is_std_type;
        $($rem:tt)*
    ) => (
        $crate::__impl_fmt_recursive!{
            impls[
                is_std_type = true;
            ]
            tokens[$($rem)*]
        }
    );
    (
        $($rem:tt)*
    ) => (
        $crate::__impl_fmt_recursive!{
            impls[
                is_std_type = false;
            ]
            tokens[$($rem)*]
        }
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_fmt_recursive{
    (
        impls[$($impls:tt)*]

        tokens[
            $(#[$impl_attr:meta])*
            impl[$($impl_:tt)*] $type:ty
            $(where[ $($where:tt)* ])?;

            $($rem:tt)*
        ]
    ) => (
        $crate::__impl_fmt_recursive!{

            impls[
                $($impls)*
                (
                    $(#[$impl_attr])*
                    #[allow(unused_mut)]
                    impl[$($impl_)*] $type
                    where[ $($($where)*)? ];
                )
            ]
            tokens[
                $($rem)*
            ]
        }
    );
    // The same as the above macro branch, but it doesn't require the `[]` in `impl[]`
    (
        impls[$($impls:tt)*]

        tokens[
            $(#[$impl_attr:meta])*
            impl $type:ty
            $(where[ $($where:tt)* ])?;

            $($rem:tt)*
        ]
    ) => (
        $crate::__impl_fmt_recursive!{

            impls[
                $($impls)*
                (
                    $(#[$impl_attr])*
                    #[allow(unused_mut)]
                    impl[] $type
                    where[ $($($where)*)? ];
                )
            ]
            tokens[
                $($rem)*
            ]
        }
    );
    (
        impls $impls:tt
        tokens[
            $($rem:tt)*
        ]
    ) => (
        $crate::__impl_fmt_inner!{
            @all_impls
            impls $impls
            ($($rem)*)
        }
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_fmt_inner {
    (@all_impls
        impls [
            is_std_type = $is_std_type:ident;
            $( $an_impl:tt )+
        ]

        $stuff:tt
    )=>{
        $(
            $crate::__impl_fmt_inner!{
                @impl_get_type_kind
                is_std_type = $is_std_type;
                $an_impl
            }

            $crate::__impl_fmt_inner!{
                @an_impl
                is_std_type = $is_std_type;
                $an_impl
                $stuff
            }
        )+
    };
    (@impl_get_type_kind
        is_std_type = true;
        (
            $(#[$impl_attr:meta])*
            impl[$($impl_:tt)*] $type:ty
            where[ $($where:tt)* ];
        )
    )=>{
        $(#[$impl_attr])*
        impl<$($impl_)*> $crate::pmr::FormatMarker for $type
        where
            $($where)*
        {
            type Kind = $crate::pmr::IsStdKind;
            type This = Self;
        }

        $(#[$impl_attr])*
        impl<$($impl_)* __T> $crate::pmr::IsAFormatMarker<IsStdKind, $type, __T>
        where
            $($where)*
        {
            #[inline(always)]
            pub const fn coerce(self, reference: &$type) -> PWrapper<$type> {
                PWrapper(*reference)
            }
        }
    };
    (@impl_get_type_kind
        is_std_type = false;
        (
            $(#[$impl_attr:meta])*
            impl[$($impl_:tt)*] $type:ty
            where[ $($where:tt)* ];
        )
    )=>{
        $(#[$impl_attr])*
        impl<$($impl_)*> $crate::pmr::FormatMarker for $type
        where
            $($where)*
        {
            type Kind = $crate::pmr::IsNotStdKind;
            type This = Self;
        }
    };
    (@an_impl
        is_std_type = $is_std_type:ident;
        (
            $(#[$impl_attr:meta])*
            impl[$($impl_:tt)*] $type:ty
            where[ $($where:tt)* ];
        )
        (
            $($everything:tt)*
        )
    )=>{
        $(#[$impl_attr])*
        impl<$($impl_)*> $crate::__impl_fmt_inner!(@self_ty $type, $is_std_type )
        where
            $($where)*
        {
            $($everything)*
        }
    };

    (@self_ty $self:ty, /*is_std_type*/ true )=>{
        $crate::pmr::PWrapper<$self>
    };
    (@self_ty $self:ty, /*is_std_type*/ false )=>{
        $self
    };

}
