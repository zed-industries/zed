extern crate xmlparser;

use xmlparser::*;


#[test]
fn text_pos_1() {
    let mut s = Stream::from("text");
    s.advance(2);
    assert_eq!(s.gen_text_pos(), TextPos::new(1, 3));
}

#[test]
fn text_pos_2() {
    let mut s = Stream::from("text\ntext");
    s.advance(6);
    assert_eq!(s.gen_text_pos(), TextPos::new(2, 2));
}

#[test]
fn text_pos_3() {
    let mut s = Stream::from("текст\nтекст");
    s.advance(15);
    assert_eq!(s.gen_text_pos(), TextPos::new(2, 3));
}


#[test]
fn token_size() {
    assert!(::std::mem::size_of::<Token>() <= 196);
}

#[test]
fn span_size() {
    assert!(::std::mem::size_of::<StrSpan>() <= 48);
}

#[test]
fn err_size_1() {
    assert!(::std::mem::size_of::<Error>() <= 64);
}

#[test]
fn err_size_2() {
    assert!(::std::mem::size_of::<StreamError>() <= 64);
}
