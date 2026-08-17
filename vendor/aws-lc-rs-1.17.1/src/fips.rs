// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

/// Retrieve the FIPS module service status.
#[allow(dead_code)] // appease clippy
#[cfg(all(feature = "fips", debug_assertions))]
pub(crate) fn get_fips_service_status() -> FipsServiceStatus<()> {
    if let Some(status) = indicator::get_status() {
        if status {
            FipsServiceStatus::Approved(())
        } else {
            FipsServiceStatus::NonApproved(())
        }
    } else {
        FipsServiceStatus::Unset(())
    }
}

#[inline]
pub(crate) fn set_fips_service_status_unapproved() {
    #[cfg(all(feature = "fips", debug_assertions))]
    indicator::set_unapproved();
}

#[allow(dead_code)]
#[cfg(all(feature = "fips", debug_assertions))]
#[inline]
pub(crate) fn clear_fips_service_status() {
    indicator::clear();
}

#[cfg(all(feature = "fips", debug_assertions))]
pub(crate) mod indicator {
    use core::cell::Cell;

    thread_local! {
        static STATUS_INDICATOR: Cell<Option<bool>> = const { Cell::new(None) };
    }

    // Retrieves and returns the current indicator status while resetting the indicator
    // for future calls.
    pub fn get_status() -> Option<bool> {
        STATUS_INDICATOR.with(|v| {
            let swap = Cell::new(None);
            v.swap(&swap);
            swap.take()
        })
    }

    pub fn set_approved() {
        STATUS_INDICATOR.with(|v| v.set(Some(true)));
    }

    pub fn set_unapproved() {
        STATUS_INDICATOR.with(|v| v.set(Some(false)));
    }

    pub fn clear() {
        STATUS_INDICATOR.with(|v| v.set(None));
    }
}

#[cfg(all(feature = "fips", debug_assertions))]
#[inline]
pub(crate) fn service_indicator_before_call() -> u64 {
    unsafe { aws_lc::FIPS_service_indicator_before_call() }
}

#[cfg(all(feature = "fips", debug_assertions))]
#[inline]
pub(crate) fn service_indicator_after_call() -> u64 {
    unsafe { aws_lc::FIPS_service_indicator_after_call() }
}

/// The FIPS Module Service Status
#[allow(dead_code)] // appease clippy
#[cfg(all(feature = "fips", debug_assertions))]
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FipsServiceStatus<R> {
    /// Indicates that the current thread is using approved FIPS cryptographic services.
    Approved(R),

    /// Indicates that the current thread has used non-approved FIPS cryptographic services.
    /// The service indicator status can be reset using `reset_fips_service_status`.
    /// `reset_fips_service_status` will return `NonApprovedMode` if the service used a non-approved
    /// service, and automatically resets the service status for you.
    NonApproved(R),

    /// Indicates that the service indicator is not set
    Unset(R),
}

#[cfg(all(feature = "fips", debug_assertions))]
impl<R> FipsServiceStatus<R> {
    /// Maps a `ServiceStatus<R>` to a `ServiceStatus<S>` by applying a function to a contained value.
    #[allow(dead_code)]
    pub fn map<S, F>(self, op: F) -> FipsServiceStatus<S>
    where
        F: FnOnce(R) -> S,
    {
        match self {
            FipsServiceStatus::Approved(v) => FipsServiceStatus::Approved(op(v)),
            FipsServiceStatus::NonApproved(v) => FipsServiceStatus::NonApproved(op(v)),
            FipsServiceStatus::Unset(v) => FipsServiceStatus::Unset(op(v)),
        }
    }
}

macro_rules! indicator_check {
    ($function:expr) => {{
        #[cfg(all(feature = "fips", debug_assertions))]
        {
            use crate::fips::{service_indicator_after_call, service_indicator_before_call};
            let before = service_indicator_before_call();
            let result = $function;
            let after = service_indicator_after_call();
            if before == after {
                crate::fips::indicator::set_unapproved();
                result
            } else {
                crate::fips::indicator::set_approved();
                result
            }
        }
        #[cfg(any(not(feature = "fips"), not(debug_assertions)))]
        {
            $function
        }
    }};
}

pub(crate) use indicator_check;

#[allow(unused_macros)]
#[cfg(all(feature = "fips", debug_assertions))]
macro_rules! check_fips_service_status {
    ($function:expr) => {{
        // Clear the current indicator status first by retrieving it
        use $crate::fips::{clear_fips_service_status, get_fips_service_status};
        clear_fips_service_status();
        // do the expression
        let result = $function;
        // Check indicator after expression
        get_fips_service_status().map(|()| result)
    }};
}

#[allow(unused_imports)]
#[cfg(all(feature = "fips", debug_assertions))]
pub(crate) use check_fips_service_status;

#[allow(unused_macros)]
#[cfg(all(feature = "fips", debug_assertions))]
macro_rules! assert_fips_status_indicator {
    ($function:expr, $expect:path) => {
        assert_fips_status_indicator!($function, $expect, "unexpected service indicator")
    };
    ($function:expr, $expect:path, $message:literal) => {{
        match crate::fips::check_fips_service_status!($function) {
            $expect(v) => v,
            _ => panic!($message),
        }
    }};
}

#[allow(unused_imports)]
#[cfg(all(feature = "fips", debug_assertions))]
pub(crate) use assert_fips_status_indicator;

#[cfg(test)]
mod tests {

    #[cfg(all(feature = "fips", debug_assertions))]
    #[test]
    fn test_service_status() {
        use crate::fips::FipsServiceStatus;

        assert_eq!(
            FipsServiceStatus::Approved(true),
            FipsServiceStatus::Approved(()).map(|()| true)
        );
        assert_eq!(
            FipsServiceStatus::NonApproved(true),
            FipsServiceStatus::NonApproved(()).map(|()| true)
        );
        assert_eq!(
            FipsServiceStatus::Unset(true),
            FipsServiceStatus::Unset(()).map(|()| true)
        );
    }
}
