// Our foundation: deterministic tests with controlled randomness
#[gpui::test(iterations = 10)]
async fn test_collaborative_editing(executor: BackgroundExecutor) {
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client("user_a").await;
    let client_b = server.create_client("user_b").await;

    // Create shared project
    let project_a = client_a.build_local_project("/code").await;
    let project_id = project_a.borrow_mut().share().await.unwrap();

    // Client B joins
    let project_b = client_b.join_remote_project(project_id).await;

    // Open same buffer
    let buffer_a = project_a.borrow_mut()
        .open_local_buffer("/code/main.rs").await.unwrap();
    let buffer_b = project_b.borrow_mut()
        .open_buffer("main.rs").await.unwrap();

    // Concurrent edits
    buffer_a.borrow_mut().edit([(0..0, "// A's edit\n")]);
    buffer_b.borrow_mut().edit([(0..0, "// B's edit\n")]);

    // Controlled network failures
    server.disconnect_client(client_a.peer_id().unwrap());
    executor.advance_clock(RECEIVE_TIMEOUT);

    // B continues editing while A is disconnected
    buffer_b.borrow_mut().edit([(24..24, "// B alone\n")]);

    // A reconnects
    executor.advance_clock(RECONNECT_TIMEOUT);
    executor.run_until_parked();

    // Clear pass/fail - reproducible every time
    assert_eq!(buffer_a.borrow().text(), buffer_b.borrow().text());
}
