//! Creating and managing sessions for multi-turn conversations.
//!
//! A **session** represents a multi-turn conversation with an agent. Within a
//! session, you can send prompts, receive responses, and the agent maintains
//! context across turns.
//!
//! # Creating a Session
//!
//! Use the session builder to create a new session:
//!
//! ```
//! # use agent_client_protocol::{Client, Agent, ConnectTo};
//! # async fn example(transport: impl ConnectTo<Client>) -> Result<(), agent_client_protocol::Error> {
//! # Client.builder().connect_with(transport, async |cx| {
//! cx.build_session_cwd()?          // Use current working directory
//!     .block_task()                // Mark as blocking
//!     .run_until(async |session| {
//!         // Use the session here
//!         Ok(())
//!     })
//!     .await?;
//! # Ok(())
//! # }).await?;
//! # Ok(())
//! # }
//! ```
//!
//! Or specify a custom working directory:
//!
//! ```
//! # use agent_client_protocol::{Client, Agent, ConnectTo};
//! # async fn example(transport: impl ConnectTo<Client>) -> Result<(), agent_client_protocol::Error> {
//! # Client.builder().connect_with(transport, async |cx| {
//! cx.build_session("/path/to/project")
//!     .block_task()
//!     .run_until(async |session| { Ok(()) })
//!     .await?;
//! # Ok(())
//! # }).await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Sending Prompts
//!
//! Inside `run_until`, you get an [`ActiveSession`] that lets you interact
//! with the agent:
//!
//! ```
//! # use agent_client_protocol::{Client, Agent, ConnectTo};
//! # async fn example(transport: impl ConnectTo<Client>) -> Result<(), agent_client_protocol::Error> {
//! # Client.builder().connect_with(transport, async |cx| {
//! # cx.build_session_cwd()?.block_task()
//! .run_until(async |mut session| {
//!     // Send a prompt
//!     session.send_prompt("What is 2 + 2?")?;
//!
//!     // Read the complete response as a string
//!     let response = session.read_to_string().await?;
//!     println!("{}", response);
//!
//!     // Send another prompt in the same session
//!     session.send_prompt("And what is 3 + 3?")?;
//!     let response = session.read_to_string().await?;
//!
//!     Ok(())
//! })
//! # .await?;
//! # Ok(())
//! # }).await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Adding MCP Servers
//!
//! You can attach MCP (Model Context Protocol) servers to a session to provide
//! tools to the agent:
//!
//! MCP attachment requires the `unstable_mcp_over_acp` feature. Standalone MCP
//! servers remain available without it.
//!
//! ```ignore
//! # use agent_client_protocol::{Client, Agent, ConnectTo};
//! # use agent_client_protocol::mcp_server::McpServer;
//! # use agent_client_protocol_rmcp::McpServerExt;
//! # async fn example(transport: impl ConnectTo<Client>) -> Result<(), agent_client_protocol::Error> {
//! # let my_mcp_server = McpServer::<Agent, _>::builder("tools").build();
//! # Client.builder().connect_with(transport, async |cx| {
//! cx.build_session_cwd()?
//!     .with_mcp_server(my_mcp_server)?
//!     .block_task()
//!     .run_until(async |session| { Ok(()) })
//!     .await?;
//! # Ok(())
//! # }).await?;
//! # Ok(())
//! # }
//! ```
//!
//! See the cookbook for detailed MCP server examples.
//!
//! # Non-Blocking Session Start
//!
//! If you're inside an `on_receive_*` callback and need to start a session,
//! use `on_session_start` instead of `block_task().run_until()`:
//!
//! ```
//! # use agent_client_protocol::{Client, Agent, ConnectTo};
//! # use agent_client_protocol::schema::v1::NewSessionRequest;
//! # async fn example(transport: impl ConnectTo<Client>) -> Result<(), agent_client_protocol::Error> {
//! Client.builder()
//!     .on_receive_request(async |req: NewSessionRequest, responder, cx| {
//!         cx.build_session_from(req)
//!             .on_session_start(async |session| {
//!                 // Handle the session
//!                 Ok(())
//!             })?;
//!         Ok(())
//!     }, agent_client_protocol::on_receive_request!())
//! #   .connect_with(transport, async |_| Ok(())).await?;
//! # Ok(())
//! # }
//! ```
//!
//! When the session response is routed during its original dispatch, session
//! routing is installed before later messages are dispatched. The callback is
//! invoked in a spawned task, so no user callback code has that ordering
//! guarantee and the callback can wait for session traffic. A response
//! interceptor that retains and routes the response later cannot retroactively
//! order setup before messages already processed. See [Ordering](super::ordering)
//! for details.
//!
//! # Next Steps
//!
//! - [Callbacks](super::callbacks) - Handle incoming requests
//! - [Ordering](super::ordering) - Understand when to use `block_task` vs `on_*`
//!
//! [`ActiveSession`]: crate::ActiveSession
