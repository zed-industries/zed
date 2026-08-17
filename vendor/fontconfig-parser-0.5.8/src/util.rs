macro_rules! try_next {
    ($iter:expr, $($tt:tt)*) => {
        match $iter.next() {
            Some(e) => e,
            None => return Err(crate::Error::InvalidFormat(format!($($tt)*))),
        }
    }
}

macro_rules! try_text {
    ($node:expr) => {
        match $node.text() {
            Some(t) => t,
            None => return Err(crate::Error::InvalidFormat("Can't get text".into())),
        }
    };
}

macro_rules! parse_attrs_opt {
    ($node:expr, { $($key:expr => $lvalue:expr,)+ } $(, { $($str_key:expr => $str_lvalue:expr,)+ } )?) => {
        for attr in $node.attributes() {
            match attr.name() {
                $(
                    $key => $lvalue = attr.value().parse().ok()?,
                )+
                $(
                    $(
                        $str_key => $str_lvalue = attr.value().into(),
                    )+
                )?
                _ => {}
            }
        }
    };
}

macro_rules! parse_attrs {
    ($node:expr, { $($key:expr => $lvalue:expr,)+ } $(, { $($str_key:expr => $str_lvalue:expr,)+ } )?) => {
        for attr in $node.attributes() {
            match attr.name() {
                $(
                    $key => $lvalue = attr.value().parse()?,
                )+
                $(
                    $(
                        $str_key => $str_lvalue = attr.value().into(),
                    )+
                )?
                _ => {}
            }
        }
    };
}

macro_rules! parse_enum {
    (
        $ty:ty,
        $(
            ($variant:ident, $text:expr),
        )+
        |$arg:ident| $fallback:expr,
    ) => {
        impl core::str::FromStr for $ty {
            type Err = crate::Error;

            fn from_str($arg: &str) -> crate::Result<$ty> {
                match $arg {
                    $(
                        $text => Ok(<$ty>::$variant),
                    )+
                    _ => {
                        $fallback
                    }
                }
            }
        }
    };
    (
        $ty:ty,
        $(
            ($variant:ident, $text:expr),
        )+
    ) => {
        parse_enum! {
            $ty,
            $(
                ($variant, $text),
            )+
            |s| Err(crate::Error::ParseEnumError(core::any::type_name::<$ty>(), s.into())),
        }
    };
}
