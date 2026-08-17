#[cfg(feature = "runtime-async-std")]
#[macro_export]
macro_rules! block_on {
    ($($expr:tt)*) => {
        ::async_std::task::block_on( $($expr)* )
    };
}

#[cfg(feature = "runtime-actix")]
#[macro_export]
macro_rules! block_on {
    ($($expr:tt)*) => {
        ::actix_rt::System::new()
            .block_on( $($expr)* )
    };
}

#[cfg(feature = "runtime-tokio")]
#[macro_export]
macro_rules! block_on {
    ($($expr:tt)*) => {
        ::tokio::runtime::Runtime::new()
            .unwrap()
            .block_on( $($expr)* )
    };
}
