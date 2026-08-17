use nom::{
  bytes::complete::tag,
  multi::{many, many0_count},
  Parser,
};

#[test]
fn parse() {
  let mut counter = 0;

  let res = {
    let mut parser = many::<_, (), Vec<&str>, _, _>(0.., |i| {
      counter += 1;
      tag("abc")(i)
    });

    parser.parse("abcabcabcabc").unwrap()
  };

  println!("res: {:?}", res);
  assert_eq!(counter, 5);
}

#[test]
fn accumulate() {
  let mut v = Vec::new();

  let (_, count) = {
    let mut parser = many0_count::<_, (), _>(|i| {
      let (i, o) = tag("abc")(i)?;
      v.push(o);
      Ok((i, ()))
    });
    parser.parse("abcabcabcabc").unwrap()
  };

  println!("v: {:?}", v);
  assert_eq!(count, 4);
  assert_eq!(v.len(), 4);
}
