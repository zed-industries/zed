use generator::*;

fn main() {
    // fn square<'a, T: Iterator<Item = u32> + 'a>(input: T) -> impl Iterator<Item = u32> + 'a {
    fn square<'a, T: Iterator<Item = u32> + Send + 'a>(input: T) -> Generator<'a, (), u32> {
        Gn::new_scoped(|mut s| {
            for i in input {
                s.yield_with(i * i);
            }
            done!();
        })
    }

    // fn sum<'a, T: Iterator<Item = u32> + 'a>(input: T) -> impl Iterator<Item = u32> + 'a {
    fn sum<'a, T: Iterator<Item = u32> + Send + 'a>(input: T) -> Generator<'a, (), u32> {
        Gn::new_scoped(|mut s| {
            let mut acc = 0;
            for i in input {
                acc += i;
                s.yield_with(acc);
            }
            done!();
        })
    }

    for (i, sum) in sum(square(0..20)).enumerate() {
        println!("square_sum_{i:<2} = {sum:^4}");
    }
}
