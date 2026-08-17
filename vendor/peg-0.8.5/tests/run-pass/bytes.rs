extern crate peg;
use peg::parser;

parser!{
    grammar byteparser() for [u8] {
        pub rule commands() -> Vec<&'input[u8]> = command()*
        rule command() -> &'input [u8] = ">" val:$([b' ' ..= b'~']+) [0] { val }
    }
}

fn main() {
    assert_eq!(byteparser::commands(b">asdf\0>xyz\0"), Ok(vec![&b"asdf"[..], &b"xyz"[..]]));
}
