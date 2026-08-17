use anyhow::Result;
use std::env;

fn main() -> Result<()> {
    // Sanitize environment
    zed_daemon::sanitization::sanitize_env_for_daemon();

    let args: Vec<String> = env::args().collect();
    let is_stdio = args.iter().any(|a| a == "--stdio");
    let auth_token = args
        .iter()
        .position(|a| a == "--daemon-auth-token" || a == "--auth-token")
        .and_then(|idx| args.get(idx + 1).cloned())
        .or_else(|| env::var("ZED_DAEMON_TOKEN").ok());

    let auth_token = match auth_token {
        Some(tok) => tok,
        None => {
            eprintln!("Error: Standalone zed-daemon binary requires --daemon-auth-token or ZED_DAEMON_TOKEN environment variable");
            std::process::exit(1);
        }
    };

    let listen_addr = args
        .iter()
        .position(|a| a == "--daemon-listen-addr" || a == "--listen-addr")
        .and_then(|idx| args.get(idx + 1).cloned())
        .unwrap_or_else(|| "127.0.0.1:9257".to_string());

    let config = zed_daemon::DaemonConfig {
        listen_addr,
        max_connections: 64,
        auth_token: Some(auth_token),
    };

    let registry = zed_daemon::default_registry();
    let server = zed_daemon::DaemonServer::new(config, registry);

    if is_stdio {
        server.run_stdio()?;
    } else {
        smol::block_on(server.run())?;
    }

    Ok(())
}
