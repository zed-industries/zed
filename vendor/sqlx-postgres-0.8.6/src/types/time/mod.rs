mod date;
mod datetime;

// Parent module is named after the `time` crate, this module is named after the `TIME` SQL type.
#[allow(clippy::module_inception)]
mod time;

#[rustfmt::skip]
const PG_EPOCH: ::time::Date = ::time::macros::date!(2000-1-1);
