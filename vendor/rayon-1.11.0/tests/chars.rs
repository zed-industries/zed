use rayon::prelude::*;

#[test]
fn half_open_correctness() {
    let low = char::from_u32(0xD800 - 0x7).unwrap();
    let high = char::from_u32(0xE000 + 0x7).unwrap();

    let range = low..high;
    let mut chars: Vec<char> = range.into_par_iter().collect();
    chars.sort();

    assert_eq!(
        chars,
        vec![
            '\u{D7F9}', '\u{D7FA}', '\u{D7FB}', '\u{D7FC}', '\u{D7FD}', '\u{D7FE}', '\u{D7FF}',
            '\u{E000}', '\u{E001}', '\u{E002}', '\u{E003}', '\u{E004}', '\u{E005}', '\u{E006}',
        ]
    );
}

#[test]
fn closed_correctness() {
    let low = char::from_u32(0xD800 - 0x7).unwrap();
    let high = char::from_u32(0xE000 + 0x7).unwrap();

    let range = low..=high;
    let mut chars: Vec<char> = range.into_par_iter().collect();
    chars.sort();

    assert_eq!(
        chars,
        vec![
            '\u{D7F9}', '\u{D7FA}', '\u{D7FB}', '\u{D7FC}', '\u{D7FD}', '\u{D7FE}', '\u{D7FF}',
            '\u{E000}', '\u{E001}', '\u{E002}', '\u{E003}', '\u{E004}', '\u{E005}', '\u{E006}',
            '\u{E007}',
        ]
    );
}
