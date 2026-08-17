//! Message ordering, concurrency, and the dispatch loop.
//!
//! Understanding how agent-client-protocol processes messages is key to writing correct code.
//! This chapter explains the dispatch loop and the ordering guarantees you
//! can rely on.
//!
//! # The Dispatch Loop
//!
//! Each connection has a central **dispatch loop** that processes incoming
//! messages one at a time. When a message arrives, it is passed to your
//! handlers in order until one claims it.
//!
//! The key property: **the dispatch loop waits for each handler to complete
//! before processing the next message.** This gives you sequential ordering
//! guarantees within a single connection.
//!
//! # Ordered Callbacks Hold the Loop
//!
//! Request and notification callbacks registered with [`on_receive_request`]
//! and [`on_receive_notification`] run inside the dispatch loop. The loop is
//! blocked until the callback completes.
//!
//! Registering [`on_receiving_result`] or [`on_receiving_ok_result`] returns
//! immediately. If registration happens before a peer response is routed
//! during its original dispatch, response handling then holds an ordering
//! barrier until the callback completes. A pending-request failure delivered
//! without an incoming response, or a response routed later, does not carry
//! that barrier.
//!
//! Session-start helpers are two-phase: [`on_session_start`] and
//! [`on_proxy_session_start`] perform framework-owned session setup under this
//! ordering guarantee, then invoke the user callback in a spawned task so it
//! can consume later session traffic. No user callback code runs under the
//! session-setup ordering guarantee.
//!
//! While a callback holds the dispatch loop, this means:
//! - No other messages are processed while your callback runs
//! - You can safely do setup before "releasing" control back to the loop
//! - Messages are processed in the order they arrive
//!
//! # Deadlock Risk
//!
//! Because callbacks can hold the dispatch loop, it's easy to create deadlocks.
//! The most common pattern:
//!
//! ```ignore
//! // DEADLOCK: This blocks the loop waiting for a response,
//! // but the response can't arrive because the loop is blocked!
//! builder.on_receive_request(async |request: MyRequest, responder, cx| {
//!     let response = cx.send_request(SomeRequest { ... })
//!         .block_task()  // <-- Waits for response
//!         .await?;       // <-- But response can never arrive!
//!     responder.respond(response)
//! }, on_receive_request!());
//! ```
//!
//! The response can never arrive because the dispatch loop is blocked waiting
//! for your callback to complete.
//!
//! # `block_task` vs `on_receiving_result`
//!
//! When you send a request, you get a [`SentRequest`] with two ways to handle it:
//!
//! ## `block_task()` - Does not hold dispatch while you process
//!
//! Use this from a task that already runs outside the dispatch loop, such as a
//! foreground `connect_with` future or a spawned task:
//!
//! ```
//! # use agent_client_protocol::{Client, Agent, ConnectTo};
//! # use agent_client_protocol_test::MyRequest;
//! # async fn example(transport: impl ConnectTo<Client>) -> Result<(), agent_client_protocol::Error> {
//! # Client.builder().connect_with(transport, async |cx| {
//! cx.spawn({
//!     let cx = cx.clone();
//!     async move {
//!         // Safe: we're in a spawned task, not blocking the dispatch loop
//!         let response = cx.send_request(MyRequest {})
//!             .block_task()
//!             .await?;
//!         // Process response...
//!         Ok(())
//!     }
//! })?;
//! # Ok(())
//! # }).await?;
//! # Ok(())
//! # }
//! ```
//!
//! The dispatch loop continues immediately after delivering the response.
//! Your code receives the response and can take as long as it wants.
//!
//! ## `on_receiving_result()` - A peer response callback can hold the loop
//!
//! Use this when you need ordering guarantees:
//!
//! ```
//! # use agent_client_protocol::{Client, Agent, ConnectTo};
//! # use agent_client_protocol_test::MyRequest;
//! # async fn example(transport: impl ConnectTo<Client>) -> Result<(), agent_client_protocol::Error> {
//! # Client.builder().connect_with(transport, async |cx| {
//! cx.send_request(MyRequest {})
//!     .on_receiving_result(async |result| {
//!         // A timely peer response holds dispatch until this completes
//!         let response = result?;
//!         // Do something with response...
//!         Ok(())
//!     })?;
//! # Ok(())
//! # }).await?;
//! # Ok(())
//! # }
//! ```
//!
//! Register the callback before a peer response is routed to select this
//! ordered mode. The dispatch loop then waits for your callback before
//! processing the next message. A pending-request failure delivered without
//! an incoming response (such as EOF), a response that was already routed, or
//! one that an interceptor retains and routes after its original dispatch does
//! not carry the barrier.
//!
//! An ordered callback must not wait for later inbound traffic on the same
//! connection. Spawn that follow-up work and return from the callback so the
//! loop can continue.
//!
//! # Escaping the Loop: `spawn`
//!
//! Use [`spawn`] to run work outside the dispatch loop:
//!
//! ```ignore
//! builder.on_receive_request(async |request: MyRequest, responder, cx| {
//!     cx.spawn(async move {
//!         // This runs outside the loop - other messages may be processed
//!         let response = cx.send_request(SomeRequest { ... })
//!             .block_task()
//!             .await?;
//!         // ...
//!         Ok(())
//!     })?;
//!     responder.respond(MyResponse { ... })  // Return immediately
//! }, on_receive_request!());
//! ```
//!
//! # Blocking Session Methods
//!
//! `SessionBuilder::run_until` does not spawn its caller. It waits for the
//! session response on the current task, so call it only when that task already
//! runs outside the dispatch loop—for example, from the foreground future
//! passed to `connect_with` or from a task created with [`spawn`]. Awaiting it
//! from a message handler deadlocks just like awaiting `block_task()` there.
//!
//! ```
//! # use agent_client_protocol::{Client, Agent, ConnectTo};
//! # async fn example(transport: impl ConnectTo<Client>) -> Result<(), agent_client_protocol::Error> {
//! # Client.builder().connect_with(transport, async |cx| {
//! cx.build_session_cwd()?
//!     .block_task()
//!     .run_until(async |mut session| {
//!         // Safe: connect_with's foreground future runs outside the dispatch loop
//!         session.send_prompt("Hello")?;
//!         let response = session.read_to_string().await?;
//!         Ok(())
//!     })
//!     .await?;
//! # Ok(())
//! # }).await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Summary
//!
//! | Pattern | Blocks Loop? | Use When |
//! |---------|--------------|----------|
//! | `on_receive_*` callback | Yes | Handle one incoming message |
//! | `on_receiving_*` callback | For a timely peer response | Bounded response work that needs ordering |
//! | `on_session_start` / `on_proxy_session_start` | Setup only | Install session routing, then run session work concurrently |
//! | `block_task()` | If awaited in a handler | Wait for a response from outside the dispatch loop |
//! | `spawn(...)` | No | Long-running work, don't need ordering |
//! | `block_task().run_until(...)` | If called in a handler | Session-scoped work from outside the dispatch loop |
//!
//! # Next Steps
//!
//! - [Proxies and Conductors](super::proxies) - Building message interceptors
//!
//! [`on_receive_request`]: crate::Builder::on_receive_request
//! [`on_receive_notification`]: crate::Builder::on_receive_notification
//! [`on_receiving_result`]: crate::SentRequest::on_receiving_result
//! [`on_receiving_ok_result`]: crate::SentRequest::on_receiving_ok_result
//! [`on_session_start`]: crate::SessionBuilder::on_session_start
//! [`on_proxy_session_start`]: crate::SessionBuilder::on_proxy_session_start
//! [`SentRequest`]: crate::SentRequest
//! [`spawn`]: crate::ConnectionTo::spawn
