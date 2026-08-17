use std::error::Error;

use clap::Args;

#[derive(Args, Debug)]
pub(crate) struct FnParser {
    /// Hand-written parser for tuples
    #[arg(short = 'D', value_name = "KEY=VALUE", value_parser = parse_key_val::<String, i32>)]
    defines: Vec<(String, i32)>,
}

/// Parse a single key-value pair
fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: Error + Send + Sync + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}
