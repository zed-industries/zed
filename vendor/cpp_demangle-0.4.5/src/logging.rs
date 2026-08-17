#[cfg(feature = "logging")]
macro_rules! log {
    ( $fmt:expr ) => {
        println!($fmt);
    };
    ( $fmt:expr, $($x:tt)* ) => {
        println!($fmt, $($x)*);
    }
}

#[cfg(not(feature = "logging"))]
macro_rules! log {
    ( $fmt:expr ) => {};
    ( $fmt:expr, $($x:tt)* ) => {
        if false { let _ = format!($fmt, $($x)*); }
    };
}
