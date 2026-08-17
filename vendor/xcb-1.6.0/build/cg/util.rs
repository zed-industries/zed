// capitalize each substring beginning by uppercase
// said otherwise: every upper preceded by another upper and followed by a upper is turned to lower
pub(super) fn tit_cap(name: &str) -> String {
    if name.len() <= 1 {
        return name.into();
    }

    let is_high = |c: char| c.is_ascii_uppercase() | c.is_ascii_digit();

    let mut res = String::new();
    let mut ch = name.chars();
    let mut prev = ch.next().unwrap();
    res.push(prev.to_ascii_uppercase());
    let mut c = ch.next().unwrap();

    for next in ch {
        if c != '_' {
            if is_high(c) && is_high(prev) && (is_high(next) || next == '_') {
                res.push(c.to_ascii_lowercase())
            } else if prev == '_' && c != '_' {
                res.push(c.to_ascii_uppercase())
            } else {
                res.push(c)
            }
        }
        prev = c;
        c = next;
    }
    if is_high(c) && is_high(prev) {
        res.push(c.to_ascii_lowercase());
    } else {
        res.push(c);
    }

    res
}

#[test]
fn test_tit_cap() {
    assert!(tit_cap("SomeString") == "SomeString");
    assert!(tit_cap("WINDOW") == "Window");
    assert!(tit_cap("CONTEXT_TAG") == "ContextTag");
    assert!(tit_cap("value_list") == "ValueList");
    assert!(tit_cap("GContext") == "GContext");
    assert!(tit_cap("IDChoice") == "IdChoice");
}

// insert a underscore before each uppercase/digit preceded or follwed by lowercase
// do not apply to the first char
pub(super) fn tit_split(name: &str) -> String {
    if name.len() <= 1 {
        return name.into();
    }

    let is_high = |c: char| c.is_ascii_uppercase();
    let is_low = |c: char| c.is_ascii_lowercase();

    let mut res = String::new();
    let mut ch = name.chars();
    let mut prev = ch.next().unwrap();
    res.push(prev);
    let mut c = ch.next().unwrap();

    for next in ch {
        if (is_low(prev) && is_high(c) || is_high(c) && is_low(next)) && prev != '_' {
            res.push('_');
        }
        res.push(c);
        prev = c;
        c = next;
    }
    if is_low(prev) && is_high(c) && prev != '_' {
        res.push('_');
    }
    res.push(c);

    res
}

#[test]
fn test_tit_split() {
    assert_eq!(tit_dig("SomeString"), "Some_String");
    assert_eq!(tit_dig("WINDOW"), "WINDOW");
}

pub(super) fn to_snake_case(name: &str) -> String {
    tit_split(name).to_ascii_lowercase()
}

pub(super) fn extract_module(typ: &str) -> (Option<&str>, &str) {
    let len = typ.len();
    let colon = typ.as_bytes().iter().position(|b| *b == b':');
    if let Some(colon) = colon {
        (Some(&typ[0..colon]), &typ[colon + 1..len])
    } else {
        (None, typ)
    }
}
