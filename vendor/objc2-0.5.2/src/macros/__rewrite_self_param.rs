/// Detect instance vs. class method.
///
/// Will add:
/// ```ignore
/// (builder_method:ident)
/// (receiver:expr)
/// (receiver_ty:ty)
/// (params_prefix*)
/// (params_rest*)
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! __rewrite_self_param {
    {
        ($($params:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $crate::__rewrite_self_param_inner! {
            // Duplicate params out so that we can match on `self`, while still
            // using it as a function parameter
            ($($params)*)
            ($($params)*)

            ($out_macro)
            $($macro_args)*
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __rewrite_self_param_inner {
    // Instance method
    {
        (&self $($__params_rest:tt)*)
        (&$self:ident $(, $($params_rest:tt)*)?)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $out_macro! {
            $($macro_args)*

            (add_method)
            ($self)
            (&Self)
            (
                &$self,
                _: $crate::runtime::Sel,
            )
            ($($($params_rest)*)?)
        }
    };
    {
        (&mut self $($__params_rest:tt)*)
        (&mut $self:ident $(, $($params_rest:tt)*)?)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $out_macro! {
            $($macro_args)*

            (add_method)
            ($self)
            (&mut Self)
            (
                &mut $self,
                _: $crate::runtime::Sel,
            )
            ($($($params_rest)*)?)
        }
    };
    {
        (self: $__self_ty:ty $(, $($__params_rest:tt)*)?)
        ($self:ident: $self_ty:ty $(, $($params_rest:tt)*)?)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $out_macro! {
            $($macro_args)*

            (add_method)
            ($self)
            ($self_ty)
            (
                $self: $self_ty,
                _: $crate::runtime::Sel,
            )
            ($($($params_rest)*)?)
        }
    };
    {
        (mut self: $__self_ty:ty $(, $($__params_rest:tt)*)?)
        ($mut:ident $self:ident: $self_ty:ty $(, $($params_rest:tt)*)?)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $out_macro! {
            $($macro_args)*

            (add_method)
            ($self)
            ($self_ty)
            (
                $mut $self: $self_ty,
                _: $crate::runtime::Sel,
            )
            ($($($params_rest)*)?)
        }
    };

    // `this: Type` or `_this: Type` instance method
    // Workaround for arbitary self types being unstable
    // https://doc.rust-lang.org/nightly/unstable-book/language-features/arbitrary-self-types.html
    {
        (mut this: $__self_ty:ty $(, $($__params_rest:tt)*)?)
        ($mut:ident $this:ident: $this_ty:ty $(, $($params_rest:tt)*)?)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $out_macro! {
            $($macro_args)*

            (add_method)
            ($this)
            ($this_ty)
            (
                $mut $this: $this_ty,
                _: $crate::runtime::Sel,
            )
            ($($($params_rest)*)?)
        }
    };
    {
        (this: $__self_ty:ty $(, $($__params_rest:tt)*)?)
        ($this:ident: $this_ty:ty $(, $($params_rest:tt)*)?)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $out_macro! {
            $($macro_args)*

            (add_method)
            ($this)
            ($this_ty)
            (
                $this: $this_ty,
                _: $crate::runtime::Sel,
            )
            ($($($params_rest)*)?)
        }
    };
    {
        (mut _this: $__self_ty:ty $(, $($__params_rest:tt)*)?)
        ($mut:ident $this:ident: $this_ty:ty $(, $($params_rest:tt)*)?)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $out_macro! {
            $($macro_args)*

            (add_method)
            ($this)
            ($this_ty)
            (
                $mut $this: $this_ty,
                _: $crate::runtime::Sel,
            )
            ($($($params_rest)*)?)
        }
    };
    {
        (_this: $__self_ty:ty $(, $($__params_rest:tt)*)?)
        ($this:ident: $this_ty:ty $(, $($params_rest:tt)*)?)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $out_macro! {
            $($macro_args)*

            (add_method)
            ($this)
            ($this_ty)
            (
                $this: $this_ty,
                _: $crate::runtime::Sel,
            )
            ($($($params_rest)*)?)
        }
    };

    // Class method
    {
        ($($__params:tt)*)
        ($($params_rest:tt)*)

        ($out_macro:path)
        $($macro_args:tt)*
    } => {
        $out_macro! {
            $($macro_args)*

            (add_class_method)
            (<Self as $crate::ClassType>::class())
            (&$crate::runtime::AnyClass)
            (
                _: &$crate::runtime::AnyClass,
                _: $crate::runtime::Sel,
            )
            ($($params_rest)*)
        }
    };
}
