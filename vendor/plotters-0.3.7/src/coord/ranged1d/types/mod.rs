#[cfg(feature = "chrono")]
mod datetime;
#[cfg(feature = "chrono")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "chrono")))]
pub use datetime::{
    IntoMonthly, IntoYearly, Monthly, RangedDate, RangedDateTime, RangedDuration, Yearly,
};

mod numeric;
pub use numeric::{
    RangedCoordf32, RangedCoordf64, RangedCoordi128, RangedCoordi32, RangedCoordi64,
    RangedCoordisize, RangedCoordu128, RangedCoordu32, RangedCoordu64, RangedCoordusize,
};

mod slice;
pub use slice::RangedSlice;
