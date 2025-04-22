use std::{path::Path, sync::Arc};

use anyhow::Result;
use dap::{DebugRequest, client::DebugAdapterClient};
use gpui::{App, AppContext, Entity, Subscription, Task};
use task::DebugTaskDefinition;

use super::session::{Session, SessionStateEvent};

pub fn intercept_debug_sessions<T: Fn(&Arc<DebugAdapterClient>) + 'static>(
    cx: &mut gpui::TestAppContext,
    configure: T,
) -> Subscription {
    cx.update(|cx| {
        dbg!("new session");
        let configure = Arc::new(configure);
        cx.observe_new::<Session>(move |_, _, cx| {
            let configure = configure.clone();
            cx.subscribe_self(move |session, event, cx| {
                let configure = configure.clone();
                dbg!("new session running");
                dbg!(&event);
                if matches!(event, SessionStateEvent::Running) {
                    let client = session.adapter_client().unwrap();
                    register_default_handlers(session, &client, cx);
                    configure(&client);
                    cx.background_spawn(async move {
                        client
                            .fake_event(dap::messages::Events::Initialized(
                                Some(Default::default()),
                            ))
                            .await
                    })
                    .detach();
                }
            })
            .detach();
        })
    })
}

pub fn start_debug_session_with<T: Fn(&Arc<DebugAdapterClient>) + 'static>(
    project: &Entity<Project>,
    cx: &mut gpui::TestAppContext,
    config: DebugTaskDefinition,
    configure: T,
) -> Task<Result<Entity<Session>>> {
    let subscription = intercept_debug_sessions(cx, configure);
    let task = project.update(cx, |project, cx| project.start_debug_session(config, cx));
    cx.spawn(async move |_| {
        let result = task.await;
        drop(subscription);
        result
    })
}

pub fn start_debug_session<T: Fn(&Arc<DebugAdapterClient>) + 'static>(
    project: &Entity<Project>,
    cx: &mut gpui::TestAppContext,
    configure: T,
) -> Task<Result<Entity<Session>>> {
    start_debug_session_with(
        project,
        cx,
        DebugTaskDefinition {
            adapter: "fake-adapter".to_string(),
            request: DebugRequest::Launch(Default::default()),
            label: "test".to_string(),
            initialize_args: None,
            tcp_connection: None,
            stop_on_entry: None,
        },
        configure,
    )
}

fn register_default_handlers(session: &Session, client: &Arc<DebugAdapterClient>, cx: &mut App) {
    client.on_request::<dap::requests::Initialize, _>(move |_, _| Ok(Default::default()));
    let paths = session
        .as_local()
        .unwrap()
        .breakpoint_store
        .read(cx)
        .breakpoint_paths();

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
