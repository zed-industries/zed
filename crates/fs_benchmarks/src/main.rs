use fs::Fs;
use gpui::{AppContext, Application};
fn main() {
    let Some(path_to_read) = std::env::args().nth(1) else {
        println!("Expected path to read as 1st argument.");
        return;
    };

    let _ = Application::headless().run(|cx| {
        let fs = fs::RealFs::new(None, cx.background_executor().clone());
        cx.background_spawn(async move {
            let timer = std::time::Instant::now();
            let result = fs.load_bytes(path_to_read.as_ref()).await;
            let elapsed = timer.elapsed();
            if let Err(e) = result {
                println!("Failed `load_bytes` after {elapsed:?} with error `{e}`");
            } else {
                println!("Took {elapsed:?} to read {} bytes", result.unwrap().len());
            };
            let timer = std::time::Instant::now();
            let result = fs.metadata(path_to_read.as_ref()).await;
            let elapsed = timer.elapsed();
            if let Err(e) = result {
                println!("Failed `metadata` after {elapsed:?} with error `{e}`");
            } else {
                println!("Took {elapsed:?} to query metadata");
            };
            std::process::exit(0);
        })
        .detach();
    });
}
