use warp::Filter;

#[tokio::test]
async fn uses_tracing() {
    // Setup a log subscriber (responsible to print to output)
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter("trace")
        .without_time()
        .finish();

    // Set the previously created subscriber as the global subscriber
    tracing::subscriber::set_global_default(subscriber).unwrap();
    // Redirect normal log messages to the tracing subscriber
    tracing_log::LogTracer::init().unwrap();

    // Start a span with some metadata (fields)
    let span = tracing::info_span!("app", domain = "www.example.org");
    let _guard = span.enter();

    log::info!("logged using log macro");

    let ok = warp::any()
        .map(|| {
            tracing::info!("printed for every request");
        })
        .untuple_one()
        .and(warp::path("aa"))
        .map(|| {
            tracing::info!("only printed when path '/aa' matches");
        })
        .untuple_one()
        .map(warp::reply)
        // Here we add the tracing logger which will ensure that all requests has a span with
        // useful information about the request (method, url, version, remote_addr, etc.)
        .with(warp::trace::request());

    tracing::info!("logged using tracing macro");

    // Send a request for /
    let req = warp::test::request();
    let resp = req.reply(&ok);
    assert_eq!(resp.await.status(), 404);

    // Send a request for /aa
    let req = warp::test::request().path("/aa");
    let resp = req.reply(&ok);
    assert_eq!(resp.await.status(), 200);
}
