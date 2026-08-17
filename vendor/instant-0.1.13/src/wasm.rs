use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::time::Duration;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Hash)]
pub struct Instant(Duration);

impl Ord for Instant {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other)
            .expect("an instant should never be NaN or Inf.")
    }
}
impl Eq for Instant {}

impl Instant {
    #[inline]
    pub fn now() -> Self {
        Instant(duration_from_f64(now()))
    }

    #[inline]
    pub fn duration_since(&self, earlier: Instant) -> Duration {
        assert!(
            earlier.0 <= self.0,
            "`earlier` cannot be later than `self`."
        );
        self.0 - earlier.0
    }

    #[inline]
    pub fn elapsed(&self) -> Duration {
        Self::now().duration_since(*self)
    }

    /// Returns `Some(t)` where `t` is the time `self + duration` if `t` can be represented as
    /// `Instant` (which means it's inside the bounds of the underlying data structure), `None`
    /// otherwise.
    #[inline]
    pub fn checked_add(&self, duration: Duration) -> Option<Instant> {
        self.0.checked_add(duration).map(Instant)
    }

    /// Returns `Some(t)` where `t` is the time `self - duration` if `t` can be represented as
    /// `Instant` (which means it's inside the bounds of the underlying data structure), `None`
    /// otherwise.
    #[inline]
    pub fn checked_sub(&self, duration: Duration) -> Option<Instant> {
        self.0.checked_sub(duration).map(Instant)
    }

    /// Returns the amount of time elapsed from another instant to this one, or None if that
    /// instant is later than this one.
    #[inline]
    pub fn checked_duration_since(&self, earlier: Instant) -> Option<Duration> {
        if earlier.0 > self.0 {
            None
        } else {
            Some(self.0 - earlier.0)
        }
    }

    /// Returns the amount of time elapsed from another instant to this one, or zero duration if
    /// that instant is later than this one.
    #[inline]
    pub fn saturating_duration_since(&self, earlier: Instant) -> Duration {
        self.checked_duration_since(earlier).unwrap_or_default()
    }
}

impl Add<Duration> for Instant {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Duration) -> Self {
        Instant(self.0 + rhs)
    }
}

impl AddAssign<Duration> for Instant {
    #[inline]
    fn add_assign(&mut self, rhs: Duration) {
        self.0 += rhs
    }
}

impl Sub<Duration> for Instant {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Duration) -> Self {
        Instant(self.0 - rhs)
    }
}

impl Sub<Instant> for Instant {
    type Output = Duration;

    #[inline]
    fn sub(self, rhs: Instant) -> Duration {
        self.duration_since(rhs)
    }
}

impl SubAssign<Duration> for Instant {
    #[inline]
    fn sub_assign(&mut self, rhs: Duration) {
        self.0 -= rhs
    }
}

fn duration_from_f64(millis: f64) -> Duration {
    Duration::from_millis(millis.trunc() as u64)
        + Duration::from_nanos((millis.fract() * 1.0e6) as u64)
}

#[cfg(all(feature = "stdweb", not(feature = "wasm-bindgen")))]
#[allow(unused_results)] // Needed because the js macro triggers it.
pub fn now() -> f64 {
    use stdweb::unstable::TryInto;

    // https://developer.mozilla.org/en-US/docs/Web/API/Performance/now
    #[cfg(not(feature = "inaccurate"))]
    let v = js! { return performance.now(); };
    #[cfg(feature = "inaccurate")]
    let v = js! { return Date.now(); };
    v.try_into().unwrap()
}

#[cfg(feature = "wasm-bindgen")]
pub fn now() -> f64 {
    #[cfg(not(feature = "inaccurate"))]
    let now = {
        use wasm_bindgen_rs::prelude::*;
        use wasm_bindgen_rs::JsCast;
        js_sys::Reflect::get(&js_sys::global(), &JsValue::from_str("performance"))
            .expect("failed to get performance from global object")
            .unchecked_into::<web_sys::Performance>()
            .now()
    };
    #[cfg(feature = "inaccurate")]
    let now = js_sys::Date::now();
    now
}

// The JS now function is in a module so it won't have to be renamed
#[cfg(not(any(feature = "wasm-bindgen", feature = "stdweb")))]
mod js {
    extern "C" {
        #[cfg(not(target_os = "emscripten"))]
        pub fn now() -> f64;
        #[cfg(target_os = "emscripten")]
        pub fn _emscripten_get_now() -> f64;
    }
}
// Make the unsafe extern function "safe" so it can be called like the other 'now' functions
#[cfg(not(any(feature = "wasm-bindgen", feature = "stdweb")))]
pub fn now() -> f64 {
    #[cfg(not(target_os = "emscripten"))]
    return unsafe { js::now() };
    #[cfg(target_os = "emscripten")]
    return unsafe { js::_emscripten_get_now() };
}

/// Returns the number of millisecods elapsed since January 1, 1970 00:00:00 UTC.
#[cfg(any(feature = "wasm-bindgen", feature = "stdweb"))]
fn get_time() -> f64 {
    #[cfg(feature = "wasm-bindgen")]
    return js_sys::Date::now();
    #[cfg(all(feature = "stdweb", not(feature = "wasm-bindgen")))]
    {
        let v = js! { return Date.now(); };
        return v.try_into().unwrap();
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct SystemTime(f64);

impl SystemTime {
    pub const UNIX_EPOCH: SystemTime = SystemTime(0.0);

    pub fn now() -> SystemTime {
        cfg_if::cfg_if! {
            if #[cfg(any(feature = "wasm-bindgen", feature = "stdweb"))] {
                SystemTime(get_time())
            } else {
                SystemTime(now())
            }
        }
    }

    pub fn duration_since(&self, earlier: SystemTime) -> Result<Duration, ()> {
        let dur_ms = self.0 - earlier.0;
        if dur_ms < 0.0 {
            return Err(());
        }
        Ok(Duration::from_millis(dur_ms as u64))
    }

    pub fn elapsed(&self) -> Result<Duration, ()> {
        Self::now().duration_since(*self)
    }

    pub fn checked_add(&self, duration: Duration) -> Option<SystemTime> {
        Some(*self + duration)
    }

    pub fn checked_sub(&self, duration: Duration) -> Option<SystemTime> {
        Some(*self - duration)
    }
}

impl Add<Duration> for SystemTime {
    type Output = SystemTime;

    fn add(self, other: Duration) -> SystemTime {
        SystemTime(self.0 + other.as_millis() as f64)
    }
}

impl Sub<Duration> for SystemTime {
    type Output = SystemTime;

    fn sub(self, other: Duration) -> SystemTime {
        SystemTime(self.0 - other.as_millis() as f64)
    }
}

impl AddAssign<Duration> for SystemTime {
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs;
    }
}

impl SubAssign<Duration> for SystemTime {
    fn sub_assign(&mut self, rhs: Duration) {
        *self = *self - rhs;
    }
}
