#[cfg(target_os = "macos")]
fn main() {
    use fsevent::EventStream;
    use std::{env::args, path::Path, time::Duration};

    let paths = args().skip(1).collect::<Vec<_>>();
    let paths = paths.iter().map(Path::new).collect::<Vec<_>>();
    assert!(!paths.is_empty(), "Must pass 1 or more paths as arguments");

    let (stream, _handle) = EventStream::new(&paths, Duration::from_millis(100));
    stream.run(|events| {
        eprintln!("event batch");
        for event in events {
            eprintln!("  {:?}", event);
        }
        true
    });
}

#[cfg(not(target_os = "macos"))]
fn main() {
    eprintln!("This example only works on macOS");
}
