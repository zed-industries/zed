#[derive(Copy, Clone)]
pub enum Token {
    Open,
    Number(i32),
    Comma,
    Close,
}

peg::parser!{
    grammar tokenparser() for [Token] {
        pub rule list() -> (i32, i32) = [Token::Open] [Token::Number(a)] [Token::Comma] [Token::Number(b)] [Token::Close] { (a, b) }
    }
}

fn main() {
    assert_eq!(tokenparser::list(&[Token::Open, Token::Number(5), Token::Comma, Token::Number(7), Token::Close]), Ok((5, 7)));
}
