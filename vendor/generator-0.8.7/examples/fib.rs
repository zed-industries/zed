use generator::{done, Gn};

fn main() {
    let g = Gn::new_scoped(|mut s| {
        let (mut a, mut b) = (0, 1);
        while b < 200 {
            std::mem::swap(&mut a, &mut b);
            b += a;
            s.yield_(b);
        }
        done!();
    });

    for i in g {
        println!("{i}");
    }
}
