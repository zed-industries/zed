wasip3::cli::command::export!(Example);

struct Example;

impl wasip3::exports::cli::run::Guest for Example {
    async fn run() -> Result<(), ()> {
        let (mut tx, rx) = wasip3::wit_stream::new();

        futures::join!(
            async { wasip3::cli::stdout::write_via_stream(rx).await.unwrap() },
            async {
                let remaining = tx.write_all(b"Hello, WASI!".to_vec()).await;
                assert!(remaining.is_empty());
                drop(tx);
            }
        );
        Ok(())
    }
}
