extern crate peg;

peg::parser!(grammar borrows() for str {
    use std::borrow::{ToOwned, Cow};
   
    pub rule borrowed() -> &'input str
        = $(['a'..='z']+)

    pub rule lifetime_parameter() -> Cow<'input, str>
        = x:$(['a'..='z']+) { x.into() }
        / "COW"  { "cow".to_owned().into() }
});

use self::borrows::*;

fn main() {
    assert_eq!(borrowed("abcd"), Ok("abcd"));
    assert_eq!(&*lifetime_parameter("abcd").unwrap(), "abcd");
    assert_eq!(&*lifetime_parameter("COW").unwrap(), "cow");
}