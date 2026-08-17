#[cfg(feature = "approx")]
pub mod impl_approx;

#[cfg(feature = "bytemuck")]
pub mod impl_bytemuck;

#[cfg(feature = "mint")]
pub mod impl_mint;

#[cfg(feature = "rand")]
pub mod impl_rand;

#[cfg(feature = "serde")]
pub mod impl_serde;

#[cfg(feature = "speedy")]
pub mod impl_speedy;

#[cfg(feature = "rkyv")]
pub mod impl_rkyv;

#[cfg(feature = "encase")]
pub mod impl_encase;

#[cfg(feature = "zerocopy")]
pub mod impl_zerocopy;

#[cfg(feature = "arbitrary")]
pub mod impl_arbitrary;
