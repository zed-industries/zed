mod attr_bon;
mod attr_builder;
mod attr_const;
mod attr_crate;
mod attr_default;
mod attr_derive;
mod attr_field;
mod attr_getter;
mod attr_into;
mod attr_into_future;
mod attr_on;
#[cfg(feature = "experimental-overwritable")]
mod attr_overwritable;
mod attr_required;
mod attr_setters;
mod attr_skip;
mod attr_top_level_start_fn;
mod attr_with;
mod cfgs;
mod generics;
mod init_order;
mod lints;
mod many_params;
mod name_conflicts;
mod native_fields;
mod orig_fn_naming;
mod positional_members;
mod raw_idents;
mod smoke;
mod target_feature;
mod track_caller;

use crate::prelude::*;

#[test]
fn leading_underscore_is_stripped() {
    #[builder]
    fn sut(#[builder(default)] _arg1: bool, _arg2: Option<()>) {}

    sut().arg1(true).call();
    sut().arg2(()).call();
    sut().maybe_arg2(Some(())).call();
}

#[test]
fn lifetime_elision() {
    #[builder]
    fn sut(arg: &str, _arg2: usize) -> (&str, &str, [&str; 1]) {
        (arg, arg, [arg])
    }

    let actual = sut().arg("blackjack").arg2(32).call();
    assert_eq!(actual, ("blackjack", "blackjack", ["blackjack"]));
}

#[cfg(feature = "std")]
#[tokio::test]
async fn async_fn() {
    #[builder]
    async fn sut(arg: u32) -> u32 {
        std::future::ready(arg).await
    }

    let actual = sut().arg(42).call().await;
    assert_eq!(actual, 42);
}

#[cfg(feature = "std")]
#[tokio::test]
async fn async_fn_with_future_arg() {
    #[builder]
    async fn sut<Fut: std::future::Future + Send>(fut: Fut) -> Fut::Output {
        fut.await
    }

    fn is_send(_val: impl Send + Sync) {}

    let fut = sut().fut(std::future::ready(42)).call();

    is_send(fut);

    let actual = sut().fut(async { 42 }).call().await;
    assert_eq!(actual, 42);
}

#[test]
#[allow(unsafe_code)]
fn unsafe_fn() {
    #[builder]
    unsafe fn sut(arg: bool) {
        let _ = arg;
    }

    let builder = sut().arg(true);

    // Only the call method should be unsafe
    unsafe { builder.call() };
}

#[test]
fn impl_traits() {
    #[builder]
    fn sut(
        /// Some documentation
        iterable: impl IntoIterator<Item = impl Into<u32>>,
        multi_bounds: impl Send + Copy,
    ) {
        drop(iterable.into_iter().map(Into::into));
        let _ = multi_bounds;
        let _ = multi_bounds;
    }

    sut().iterable([1_u16, 2, 3]).multi_bounds("multi").call();
}

#[test]
fn const_function() {
    #[builder]
    const fn foo(_arg: u32) {}

    foo().arg(42).call();
}

#[test]
fn mut_fn_params() {
    #[builder]
    fn sut(mut arg1: u32, mut arg2: u32) -> (u32, u32) {
        arg1 += 1;
        arg2 += 2;

        (arg1, arg2)
    }

    let actual = sut().arg1(1).arg2(2).call();
    assert_eq!(actual, (2, 4));
}

// This is based on the issue https://github.com/elastio/bon/issues/12
#[test]
fn types_not_implementing_default() {
    struct DoesNotImplementDefault;

    #[builder]
    fn test(_value: Option<DoesNotImplementDefault>) {}

    test().call();
}
