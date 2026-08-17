#[cfg(feature = "async-dispatcher-runtime")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // todo: Show example using async-std-runtime
    // For now, check out for something similar https://github.com/runtimed/smoke/blob/main/src/main.rs
    Ok(())
}

#[cfg(feature = "tokio-runtime")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use jupyter_protocol::{
        ConnectionInfo, ExecuteRequest, ExecutionState, JupyterMessage, JupyterMessageContent,
    };
    use uuid::Uuid;

    let kernel_name = "python";
    let kernelspecs = runtimelib::list_kernelspecs().await;
    let kernel_specification = kernelspecs
        .iter()
        .find(|k| k.kernel_name.eq(kernel_name))
        .expect("Python kernel not found");

    let ip = std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1));
    let ports = runtimelib::peek_ports(ip, 5).await?;
    assert_eq!(ports.len(), 5);

    let connection_info = ConnectionInfo {
        transport: jupyter_protocol::connection_info::Transport::TCP,
        ip: ip.to_string(),
        stdin_port: ports[0],
        control_port: ports[1],
        hb_port: ports[2],
        shell_port: ports[3],
        iopub_port: ports[4],
        signature_scheme: "hmac-sha256".to_string(),
        key: uuid::Uuid::new_v4().to_string(),
        kernel_name: Some(kernel_name.to_string()),
    };

    let runtime_dir = runtimelib::dirs::runtime_dir();
    tokio::fs::create_dir_all(&runtime_dir).await.map_err(|e| {
        format!(
            "Failed to create jupyter runtime dir {}: {}",
            runtime_dir.display(),
            e
        )
    })?;

    let connection_path = runtime_dir.join("kernel-example.json");
    let content = serde_json::to_string(&connection_info)?;
    tokio::fs::write(connection_path.clone(), content).await?;

    let working_directory = "/tmp";

    let mut process = kernel_specification
        .clone()
        .command(&connection_path, None, None)?
        .current_dir(working_directory)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .stdin(std::process::Stdio::piped())
        .kill_on_drop(true)
        .spawn()?;

    let session_id = Uuid::new_v4().to_string();

    // Listen for display data, execute result, stdout messages, etc.
    let mut iopub_socket =
        runtimelib::create_client_iopub_connection(&connection_info, "", &session_id).await?;
    let identity = runtimelib::peer_identity_for_session(&session_id)?;
    let mut shell_socket = runtimelib::create_client_shell_connection_with_identity(
        &connection_info,
        &session_id,
        identity,
    )
    .await?;
    // Control socket is for kernel management, not used here
    // let mut control_socket =
    //     runtimelib::create_client_control_connection(&connection_info, &session_id).await?;

    let execute_request = ExecuteRequest::new("print('Hello, World!')".to_string());
    let execute_request: JupyterMessage = execute_request.into();

    let execute_request_id = execute_request.header.msg_id.clone();

    let iopub_handle = tokio::spawn({
        async move {
            loop {
                match iopub_socket.read().await {
                    Ok(message) => match message.content {
                        JupyterMessageContent::Status(status) => {
                            //
                            if status.execution_state == ExecutionState::Idle
                                && message.parent_header.as_ref().map(|h| h.msg_id.as_str())
                                    == Some(execute_request_id.as_str())
                            {
                                println!("Execution finalized, exiting...");
                                break;
                            }
                        }
                        _ => {
                            println!("{:?}", message.content);
                        }
                    },
                    Err(e) => {
                        eprintln!("Error receiving iopub message: {}", e);
                        break;
                    }
                }
            }
        }
    });

    shell_socket.send(execute_request).await?;

    iopub_handle.await?;

    process.start_kill()?;

    Ok(())
}
