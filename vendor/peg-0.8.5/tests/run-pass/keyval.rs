extern crate peg;
use std::collections::HashMap;

peg::parser!( grammar keyval() for str {
    rule number() -> i64
        = n:$(['0'..='9']+) { n.parse().unwrap() }

    pub rule keyvals() -> HashMap<i64, i64>
        = kvs:keyval() ++ "\n" {
            kvs.iter().cloned().collect::<HashMap<i64, i64>>()
        }

    rule keyval() -> (i64, i64)
        = k:number() ":" + v:number() { (k, v) }
});

fn main() {
    let mut expected = HashMap::new();
    expected.insert(1, 3);
    expected.insert(2, 4);
    assert_eq!(keyval::keyvals("1:3\n2:4"), Ok(expected));
}
