
extern crate peg;

peg::parser!( grammar arithmetic() for str {
    rule number() -> i64
        = n:$(['0'..='9']+) { n.parse().unwrap() }

    pub(crate) rule calculate() -> i64 = precedence!{
        x:(@) "+" y:@ { x + y }
        x:(@) "-" y:@ { x - y }
              "-" v:@ { - v }
        --
        x:(@) "*" y:@ { x * y }
        x:(@) "/" y:@ { x / y }
        --
        x:@   "^" y:(@) { x.pow(y as u32) }
        v:@   "!"       { (1..v+1).product() }
        --
        "(" v:calculate() ")" { v }
        n:number() {n}
    }
});

fn main() {
    assert_eq!(arithmetic::calculate("3+3*3+3"), Ok(15));
    assert_eq!(arithmetic::calculate("2+2^2^2^2/2+2"), Ok(32772));
    assert_eq!(arithmetic::calculate("1024/2/2/2+1"), Ok(129));
    assert_eq!(arithmetic::calculate("1024/(1+1)/2/2+1"), Ok(129));
    assert_eq!(arithmetic::calculate("-1-2*-2"), Ok(3));
    assert_eq!(arithmetic::calculate("1+3!+1"), Ok(8));
}