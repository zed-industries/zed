use std::thread;

fn main() {
    let (tx, rx) = flume::unbounded();

    let t = thread::spawn(move || {
        for msg in rx.iter() {
            println!("Received: {}", msg);
        }
    });

    tx.send("Hello, world!").unwrap();
    tx.send("How are you today?").unwrap();

    drop(tx);

    t.join().unwrap();
}
