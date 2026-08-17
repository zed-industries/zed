#![macro_use]

//! Contains several macros used in this crate.

macro_rules! gen_setter {
    ($(#[$comments:meta])* $field:ident : into $t:ty) => {

            $(#[$comments])*
            ///
            /// <small>See [`ParserConfig`][crate::ParserConfig] fields docs for details</small>
            #[inline]
            #[must_use]
            pub fn $field<T: Into<$t>>(mut self, value: T) -> Self {
                self.$field = value.into();
                self
            }
    };
    ($(#[$comments:meta])* $field:ident : val $t:ty) => {
            $(#[$comments])*
            ///
            /// <small>See [`ParserConfig`][crate::ParserConfig] fields docs for details</small>
            #[inline]
            #[must_use]
            pub const fn $field(mut self, value: $t) -> Self {
                self.$field = value;
                self
            }
    };
    ($(#[$comments:meta])* $field:ident : delegate $t:ty) => {
            $(#[$comments])*
            ///
            /// <small>See [`ParserConfig`][crate::ParserConfig] fields docs for details</small>
            #[inline]
            #[must_use]
            pub const fn $field(mut self, value: $t) -> Self {
                self.c.$field = value;
                self
            }
    };
    ($(#[$comments:meta])* $field:ident : c2 $t:ty) => {
            $(#[$comments])*
            ///
            /// <small>See [`ParserConfig2`][crate::reader::ParserConfig2] fields docs for details</small>
            #[inline]
            #[must_use]
            pub fn $field(self, value: $t) -> ParserConfig2 {
                ParserConfig2 {
                    c: self,
                    ..Default::default()
                }
                .$field(value)
            }
    };
}

macro_rules! gen_setters {
    ($target:ident, $($(#[$comments:meta])* $field:ident : $k:tt $tpe:ty),+) => (
        impl $target {$(

            gen_setter! { $(#[$comments])* $field : $k $tpe }
        )+
    })
}
