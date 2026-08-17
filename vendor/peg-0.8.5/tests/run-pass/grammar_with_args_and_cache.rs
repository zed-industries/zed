extern crate peg;

peg::parser! {
    grammar lol(config: bool) for str {
        #[cache_left_rec]
        rule one() -> ()
            = one() / "foo"
    }
}

fn main() {}
