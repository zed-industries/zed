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


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_resolve_source_mapped_path_ts_in_node_modules() {
        // Creamos un directorio temporal
        let tmp_dir = tempdir().unwrap();
        let js_file = tmp_dir.path().join("example.js");
        fs::write(&js_file, "// dummy js").unwrap();

        // Creamos un source map que apunta a un .ts en node_modules
        let map_file = js_file.with_extension("js.map");
        let map_content = r#"
        {
            "version":3,
            "file":"example.js",
            "sources":["lib/example.ts"],
            "sourceRoot":"node_modules/remeda",
            "mappings":"AAAA"
        }
        "#;
        fs::write(&map_file, map_content).unwrap();

        // Simulamos que el TS existe
        let ts_path = PathBuf::from("node_modules/remeda/lib/example.ts");
        fs::create_dir_all(ts_path.parent().unwrap()).unwrap();
        fs::write(&ts_path, "// dummy ts").unwrap();

        let resolved = resolve_source_mapped_path(&js_file);
        assert_eq!(resolved, ts_path, "Should resolve to the TS file in node_modules");
    }

    #[test]
    fn test_client_source_uses_resolved_path() {
        let tmp_dir = tempdir().unwrap();
        let js_file = tmp_dir.path().join("example.js");
        fs::write(&js_file, "// dummy js").unwrap();

        // Sin map, debe devolver la misma ruta
        let source = client_source(&js_file);
        assert_eq!(
            source.path.unwrap(),
            js_file.to_string_lossy(),
            "client_source should use resolved path"
        );
    }
}