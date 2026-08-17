use generator::{done, Gn};

fn main() {
    let n = 100000;
    let range = Gn::new_scoped(move |mut s| {
        let mut num = 0;
        while num < n {
            s.yield_(num);
            num += 1;
        }
        done!();
    });

    let sum: usize = range.sum();
    println!("sum ={sum}");
}
