/*!
The data processing module, which implements algorithms related to visualization of data.
Such as, down-sampling, etc.
*/

mod data_range;
pub use data_range::fitting_range;

mod quartiles;
pub use quartiles::Quartiles;

/// Handles the printing of floating-point numbers.
pub mod float;
