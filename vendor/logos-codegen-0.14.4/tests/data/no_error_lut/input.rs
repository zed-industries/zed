#[derive(Logos)]
#[logos(source = [u8])]
enum Token {
    #[token("\n")]
    Newline,
    #[regex(".")]
    AnyUnicode,
    #[regex(b".", priority = 0)]
    Any,
}
