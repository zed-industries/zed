//! Proves the out-of-tree Parley text system can be injected into a GPUI app
//! through the public facade.

use gpui::{TestAppContext, TextSystem};
use gpui_parley::ParleyTextSystem;
use std::sync::Arc;

#[gpui::test]
async fn parley_text_system_can_be_injected(cx: &mut TestAppContext) {
    let injected: Arc<dyn TextSystem> = ParleyTextSystem::new();
    cx.update(|cx| cx.set_text_system(injected.clone()));

    let current = cx.update(|cx| cx.text_system().clone());
    assert!(Arc::ptr_eq(&current, &injected));
}
