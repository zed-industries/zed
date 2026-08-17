#[macro_export]
macro_rules! test_sequence {
    ($bytes:expr, $seq:expr) => {
        let mut parser = ::anes::parser::Parser::default();
        parser.advance($bytes, false);
        assert_eq!(parser.next(), Some($seq));
    };
}

#[macro_export]
macro_rules! test_sequences {
    (
        $(
            $bytes:expr, $seq:expr,
        )*
    ) => {
        $(
            test_sequence!($bytes, $seq);
        )*
    };
}

mod cursor;
mod key;
mod mouse;
