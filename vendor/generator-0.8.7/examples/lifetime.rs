fn main() {
    let str = "foo".to_string();

    let mut gen = generator::Gn::new_scoped(|mut s| {
        std::thread::scope(|s2| {
            s2.spawn(|| {
                std::thread::sleep(std::time::Duration::from_millis(500));
                println!("{str}");
            });
            // here we can't use `yield_` because it still ref to `str`
            // `yield_` only impl for static captured lifetime
            // s.yield_(());
            unsafe { s.yield_unsafe(()) };
        });
        generator::done!();
    });

    gen.next();
    // std::mem::forget(gen);
    // drop(gen);
    // drop(str);
    std::thread::sleep(std::time::Duration::from_millis(1000));
}
