//! Proves the out-of-tree Parley text system can be injected into a GPUI app
//! through the public facade.

use gpui::{TestAppContext, TextSystem};
use gpui_parley::{ParleyTextSystem, ParleyTextSystemExt};
use std::sync::Arc;

#[gpui::test]
async fn parley_text_system_can_be_injected(cx: &mut TestAppContext) {
    let injected: Arc<dyn TextSystem> = ParleyTextSystem::new();
    cx.update(|cx| cx.set_text_system(injected.clone()));

    let current = cx.update(|cx| cx.text_system().clone());
    assert!(Arc::ptr_eq(&current, &injected));
}

/// The accessor recognises the injected system, and reports the default engine
/// as not being Parley.
#[gpui::test]
async fn the_accessor_finds_the_injected_system(cx: &mut TestAppContext) {
    cx.update(|cx| assert!(cx.text_system().as_parley().is_none()));

    let injected: Arc<dyn TextSystem> = ParleyTextSystem::new();
    cx.update(|cx| cx.set_text_system(injected.clone()));

    cx.update(|cx| {
        let expected = Arc::as_ptr(&injected) as *const ParleyTextSystem;
        let found = cx
            .text_system()
            .as_parley()
            .map(|parley| parley as *const ParleyTextSystem);
        assert_eq!(found, Some(expected));
    });
}
