use generator::*;

fn factors(n: u32) -> Generator<'static, (), u32> {
    Gn::new_scoped(move |mut s| {
        if n == 0 {
            return 0;
        }

        s.yield_with(1);

        for i in 2..n {
            if n % i == 0 {
                s.yield_with(i);
            }
        }
        done!();
    })
}

fn main() {
    for i in factors(28) {
        println!("{i}");
    }

    (0..10000)
        .filter(|n| factors(*n).sum::<u32>() == *n)
        .fold((), |_, n| {
            println!("n = {n}");
        })
}
