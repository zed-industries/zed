use std::{path::Path, sync::Arc};

use dap::client::DebugAdapterClient;
use gpui::{App, Subscription};

use super::session::{Session, SessionStateEvent};

pub fn intercept_debug_sessions<T: Fn(&Arc<DebugAdapterClient>) + 'static>(
    cx: &mut gpui::TestAppContext,
    configure: T,
) -> Subscription {
    cx.update(|cx| {
        let configure = Arc::new(configure);
        cx.observe_new::<Session>(move |_, _, cx| {
            let configure = configure.clone();
            cx.subscribe_self(move |session, event, cx| {
                let configure = configure.clone();
                if matches!(event, SessionStateEvent::Running) {
                    let client = session.adapter_client().unwrap();
                    register_default_handlers(session, &client, cx);
                    configure(&client);
                }
            })
            .detach();
        })
    })
}

fn register_default_handlers(session: &Session, client: &Arc<DebugAdapterClient>, cx: &mut App) {
    client.on_request::<dap::requests::Initialize, _>(move |_, _| Ok(Default::default()));
    let paths = session.breakpoint_store.read(cx).breakpoint_paths();

    client.on_request::<dap::requests::SetBreakpoints, _>(move |_, args| {
        let p = Arc::from(Path::new(&args.source.path.unwrap()));
        if !paths.contains(&p) {
            panic!("Sent breakpoints for path without any")
        }

        Ok(dap::SetBreakpointsResponse {
            breakpoints: Vec::default(),
        })
    });

    client.on_request::<dap::requests::Launch, _>(move |_, _| Ok(()));

    client.on_request::<dap::requests::SetExceptionBreakpoints, _>(move |_, _| {
        Ok(dap::SetExceptionBreakpointsResponse { breakpoints: None })
    });

    client.on_request::<dap::requests::Disconnect, _>(move |_, _| Ok(()));

    client.on_request::<dap::requests::Threads, _>(move |_, _| {
        Ok(dap::ThreadsResponse { threads: vec![] })
    });
}
