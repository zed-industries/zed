//! Execute code on a Jupyter kernel via WebSocket.
//!
//! This example demonstrates the full lifecycle:
//! 1. Connect to a Jupyter server
//! 2. Launch a kernel
//! 3. Execute code and display output
//! 4. Shut down the kernel
//!
//! Usage:
//!   cargo run -p jupyter-websocket-client --example execute -- <server-url> <token> [code]
//!
//! Example:
//!   cargo run -p jupyter-websocket-client --example execute -- http://localhost:8888 mytoken "print('hello')"

use futures::{SinkExt, StreamExt};
use jupyter_protocol::{ExecuteRequest, ExecutionState, JupyterMessage, JupyterMessageContent};
use jupyter_protocol::media::MediaType;
use jupyter_websocket_client::RemoteServer;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <server-url> <token> [code]", args[0]);
        eprintln!();
        eprintln!("Example:");
        eprintln!("  {} http://localhost:8888 mytoken \"print('hello')\"", args[0]);
        std::process::exit(1);
    }

    let server_url = &args[1];
    let token = &args[2];
    let code = args.get(3).map(|s| s.as_str()).unwrap_or("print('Hello from jupyter-websocket-client!')");

    let server = RemoteServer {
        base_url: server_url.clone(),
        token: token.clone(),
    };

    let client = reqwest::Client::new();

    // List available kernelspecs
    println!("Fetching available kernels...");
    let kernelspecs_url = format!("{}/api/kernelspecs?token={}", server_url, token);
    let kernelspecs: jupyter_websocket_client::KernelSpecsResponse = client
        .get(&kernelspecs_url)
        .send()
        .await?
        .json()
        .await?;

    println!("Available kernels:");
    for (name, spec) in &kernelspecs.kernelspecs {
        println!("  - {} ({})", name, spec.spec.display_name);
    }
    println!("Default: {}", kernelspecs.default);
    println!();

    // Launch a kernel using the default kernelspec
    println!("Launching kernel ({})...", kernelspecs.default);
    let launch_url = format!("{}/api/kernels?token={}", server_url, token);
    let launch_request = jupyter_websocket_client::KernelLaunchRequest {
        name: kernelspecs.default.clone(),
        path: None,
    };

    let kernel: jupyter_websocket_client::Kernel = client
        .post(&launch_url)
        .json(&launch_request)
        .send()
        .await?
        .json()
        .await?;

    println!("Kernel launched: {}", kernel.id);
    println!();

    // Connect to the kernel WebSocket
    println!("Connecting to kernel WebSocket...");
    let (kernel_socket, _response) = server.connect_to_kernel(&kernel.id).await?;
    println!("Protocol mode: {:?}", kernel_socket.protocol_mode);

    let (mut writer, mut reader) = kernel_socket.split();

    // Wait for iopub_welcome message indicating the iopub channel is ready
    println!("Waiting for iopub channel...");
    while let Some(response) = reader.next().await {
        let msg = response?;
        if let JupyterMessageContent::IoPubWelcome(_) = &msg.content {
            println!("IOPub channel ready.");
            break;
        }
    }

    // Execute the code
    println!("Executing: {}", code);
    println!("---");

    let execute_request = ExecuteRequest {
        code: code.to_string(),
        silent: false,
        store_history: true,
        user_expressions: Default::default(),
        allow_stdin: false,
        stop_on_error: true,
    };

    let msg = JupyterMessage::new(execute_request, None);
    let our_msg_id = msg.header.msg_id.clone();
    writer.send(msg).await?;

    // Track state
    let mut saw_busy = false;
    let mut execution_status = None;

    // Read responses until kernel goes idle
    while let Some(response) = reader.next().await {
        let msg = response?;

        // Only process messages related to our request
        let is_ours = msg
            .parent_header
            .as_ref()
            .map(|h| h.msg_id == our_msg_id)
            .unwrap_or(false);

        if !is_ours {
            continue;
        }

        match &msg.content {
            JupyterMessageContent::StreamContent(stream) => {
                print!("{}", stream.text);
            }
            JupyterMessageContent::ExecuteResult(result) => {
                for media in &result.data.content {
                    if let MediaType::Plain(text) = media {
                        println!("{}", text);
                    }
                }
            }
            JupyterMessageContent::DisplayData(data) => {
                for media in &data.data.content {
                    if let MediaType::Plain(text) = media {
                        println!("{}", text);
                    }
                }
            }
            JupyterMessageContent::ErrorOutput(error) => {
                eprintln!("Error: {}", error.evalue);
                for line in &error.traceback {
                    eprintln!("{}", line);
                }
            }
            JupyterMessageContent::ExecuteReply(reply) => {
                execution_status = Some(reply.status.clone());
            }
            JupyterMessageContent::Status(status) => {
                match status.execution_state {
                    ExecutionState::Busy => {
                        saw_busy = true;
                    }
                    ExecutionState::Idle if saw_busy => {
                        break;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    println!("---");
    if let Some(status) = execution_status {
        println!("Execution status: {:?}", status);
    }
    println!();

    // Shut down the kernel
    println!("Shutting down kernel {}...", kernel.id);
    let delete_url = format!("{}/api/kernels/{}?token={}", server_url, kernel.id, token);
    client.delete(&delete_url).send().await?;
    println!("Kernel shut down.");

    Ok(())
}
