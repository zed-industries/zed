use combine::{
    many1,
    parser::char::{letter, space},
    sep_by, Parser,
};

#[test]
fn readme() {
    main();
}

fn main() {
    let word = many1(letter());

    let mut parser = sep_by(word, space()).map(|mut words: Vec<String>| words.pop());
    let result = parser.parse("Pick up that word!");
    assert_eq!(result, Ok((Some("word".to_string()), "!")));
}
