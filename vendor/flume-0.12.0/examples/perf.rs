fn main() {
    let thread_num = 32;
    let msg_num = 16;

    let (mut main_tx, main_rx) = flume::bounded::<()>(1);

    for _ in 0..thread_num {
        let (mut tx, rx) = flume::bounded(1);
        std::mem::swap(&mut tx, &mut main_tx);

        std::thread::spawn(move || {
            for msg in rx.iter() {
                tx.send(msg).unwrap();
            }
        });
    }

    for _ in 0..1000 {
        let main_tx = main_tx.clone();
        std::thread::spawn(move || {
            for _ in 0..msg_num {
                main_tx.send(Default::default()).unwrap();
            }
        });

        for _ in 0..msg_num {
            main_rx.recv().unwrap();
        }
    }
}
