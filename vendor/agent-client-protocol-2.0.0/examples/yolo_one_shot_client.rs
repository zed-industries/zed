//! YOLO one-shot client: A simple ACP client that runs a single prompt against an agent.
//!
//! This is a simplified example showing basic ACP client usage. It only supports
//! simple command strings (not JSON configs or environment variables).
//!
//! For a more full-featured client with JSON config support, see the `yopo` binary crate.
//!
//! # Usage
//!
//! ```bash
//! cargo run --example yolo_one_shot_client -- --command "python my_agent.py" "What is 2+2?"
//! ```

use agent_client_protocol::schema::ProtocolVersion;
use agent_client_protocol::schema::v1::{
    ContentBlock, InitializeRequest, NewSessionRequest, PromptRequest, RequestPermissionOutcome,
    RequestPermissionRequest, RequestPermissionResponse, SelectedPermissionOutcome,
    SessionNotification, TextContent,
};
use agent_client_protocol::{AcpAgent, Agent, ConnectionTo};
use clap::Parser;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Parser)]
#[command(name = "yolo-one-shot-client")]
#[command(about = "A simple ACP client for one-shot prompts", long_about = None)]
struct Cli {
    /// The command to run the agent (e.g., "python my_agent.py")
    #[arg(short, long)]
    command: String,

    /// The prompt to send to the agent
    prompt: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    eprintln!("🚀 Spawning agent: {}", cli.command);

    let agent = AcpAgent::from_str(&cli.command)?;

    // Run the client — AcpAgent implements ConnectTo, so it serves as the transport
    agent_client_protocol::Client
        .builder()
        .on_receive_notification(
            async move |notification: SessionNotification, _cx| {
                println!("{:?}", notification.update);
                Ok(())
            },
            agent_client_protocol::on_receive_notification!(),
        )
        .on_receive_request(
            async move |request: RequestPermissionRequest, responder, _connection| {
                // YOLO: Auto-approve all permission requests by selecting the first option
                eprintln!("✅ Auto-approving permission request: {request:?}");
                let option_id = request.options.first().map(|opt| opt.option_id.clone());
                if let Some(id) = option_id {
                    responder.respond(RequestPermissionResponse::new(
                        RequestPermissionOutcome::Selected(SelectedPermissionOutcome::new(id)),
                    ))
                } else {
                    eprintln!("⚠️ No options provided in permission request, cancelling");
                    responder.respond(RequestPermissionResponse::new(
                        RequestPermissionOutcome::Cancelled,
                    ))
                }
            },
            agent_client_protocol::on_receive_request!(),
        )
        .connect_with(agent, |connection: ConnectionTo<Agent>| async move {
            // Initialize the agent
            eprintln!("🤝 Initializing agent...");
            let init_response = connection
                .send_request(InitializeRequest::new(ProtocolVersion::V1))
                .block_task()
                .await?;

            eprintln!("✓ Agent initialized: {:?}", init_response.agent_info);

            // Create a new session
            eprintln!("📝 Creating new session...");
            let new_session_response = connection
                .send_request(NewSessionRequest::new(
                    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/")),
                ))
                .block_task()
                .await?;

            let session_id = new_session_response.session_id;
            eprintln!("✓ Session created");

            // Send the prompt
            eprintln!("💬 Sending prompt: \"{}\"", cli.prompt);
            let prompt_response = connection
                .send_request(PromptRequest::new(
                    session_id.clone(),
                    vec![ContentBlock::Text(TextContent::new(cli.prompt.clone()))],
                ))
                .block_task()
                .await?;

            eprintln!("✅ Agent completed!");
            eprintln!("Stop reason: {:?}", prompt_response.stop_reason);

            Ok(())
        })
        .await?;

    Ok(())
}
