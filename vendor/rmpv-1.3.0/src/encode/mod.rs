pub use rmp::encode::ValueWriteError as Error;

mod value;
mod value_ref;

pub use self::value::write_value;
pub use self::value_ref::write_value_ref;
