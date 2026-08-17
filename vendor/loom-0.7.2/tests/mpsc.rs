use loom::sync::mpsc::channel;
use loom::thread;

#[test]
fn basic_sequential_usage() {
    loom::model(|| {
        let (s, r) = channel();
        s.send(5).unwrap();
        let val = r.recv().unwrap();
        assert_eq!(val, 5);
    });
}

#[test]
fn basic_parallel_usage() {
    loom::model(|| {
        let (s, r) = channel();
        thread::spawn(move || {
            s.send(5).unwrap();
        });
        let val = r.recv().unwrap();
        assert_eq!(val, 5);
    });
}

#[test]
fn commutative_senders() {
    loom::model(|| {
        let (s, r) = channel();
        let s2 = s.clone();
        thread::spawn(move || {
            s.send(5).unwrap();
        });
        thread::spawn(move || {
            s2.send(6).unwrap();
        });
        let mut val = r.recv().unwrap();
        val += r.recv().unwrap();
        assert_eq!(val, 11);
    });
}

fn ignore_result<A, B>(_: Result<A, B>) {}

#[test]
#[should_panic]
fn non_commutative_senders1() {
    loom::model(|| {
        let (s, r) = channel();
        let s2 = s.clone();
        thread::spawn(move || {
            ignore_result(s.send(5));
        });
        thread::spawn(move || {
            ignore_result(s2.send(6));
        });
        let val = r.recv().unwrap();
        assert_eq!(val, 5);
        ignore_result(r.recv());
    });
}

#[test]
#[should_panic]
fn non_commutative_senders2() {
    loom::model(|| {
        let (s, r) = channel();
        let s2 = s.clone();
        thread::spawn(move || {
            ignore_result(s.send(5));
        });
        thread::spawn(move || {
            ignore_result(s2.send(6));
        });
        let val = r.recv().unwrap();
        assert_eq!(val, 6);
        ignore_result(r.recv());
    });
}

#[test]
fn drop_receiver() {
    loom::model(|| {
        let (s, r) = channel();
        s.send(1).unwrap();
        s.send(2).unwrap();
        assert_eq!(r.recv().unwrap(), 1);
    });
}
