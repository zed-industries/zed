use super::Chars;

fn parse_int(chars: &mut Chars<'_>, breaker: char) -> Option<String> {
    let mut num = String::with_capacity(2);
    num.push(chars.next().filter(|c| c.is_numeric() || *c == '-')?);
    for c in chars {
        if c.is_numeric() {
            num.push(c)
        } else if c == breaker {
            return Some(num);
        } else {
            break;
        }
    }
    None
}

pub fn parse(mut chars: Chars<'_>) -> Option<(isize, isize, Chars<'_>)> {
    let num_a = parse_int(&mut chars, '.')?;
    if !matches!(chars.next(), Some('.')) {
        return None;
    }
    let num_b: String = parse_int(&mut chars, '}')?;
    Some((num_a.parse().ok()?, num_b.parse().ok()?, chars))
}
