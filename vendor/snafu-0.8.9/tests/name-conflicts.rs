use ::snafu as real_snafu;
use real_snafu::{ensure, Snafu};

// Likely candidates to clash with generated code
mod core {}
mod std {}
mod snafu {}

#[derive(Debug, Snafu)]
enum _VariantNamedNone {
    #[snafu(context(suffix(false)))]
    None,
}

#[derive(Debug, Snafu)]
enum _VariantNamedSome<T> {
    Some { value: T },
}

#[derive(Debug, Snafu)]
enum _VariantNamedOk<T> {
    Ok { value: T },
}

#[derive(Debug, Snafu)]
enum _VariantNamedErr<T> {
    Err { value: T },
}

fn _using_ensure() -> Result<u8, _VariantNamedNone> {
    ensure!(false, None);
    Ok(0)
}
