peg::parser!( grammar test() for str {
    rule number<T: std::str::FromStr>() -> T = s:$(['0'..='9']+) {? s.parse().or(Err("number")) }

    pub rule numbers() -> (u8, i32)
        = n1:number::<u8>() "," n2:number::<i32>() { (n1, n2) }
});

fn main() {
    assert_eq!(test::numbers("42,1234"), Ok((42, 1234)));
}
