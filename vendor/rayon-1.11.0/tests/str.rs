use rand::distr::StandardUniform;
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use rayon::prelude::*;

fn seeded_rng() -> XorShiftRng {
    let mut seed = <XorShiftRng as SeedableRng>::Seed::default();
    (0..).zip(seed.as_mut()).for_each(|(i, x)| *x = i);
    XorShiftRng::from_seed(seed)
}

#[test]
pub fn execute_strings() {
    let rng = seeded_rng();
    let s: String = rng
        .sample_iter::<char, _>(&StandardUniform)
        .take(1024)
        .collect();

    let par_chars: String = s.par_chars().collect();
    assert_eq!(s, par_chars);

    let par_even: String = s.par_chars().filter(|&c| (c as u32) & 1 == 0).collect();
    let ser_even: String = s.chars().filter(|&c| (c as u32) & 1 == 0).collect();
    assert_eq!(par_even, ser_even);

    // test `FromParallelIterator<&char> for String`
    let vchars: Vec<char> = s.par_chars().collect();
    let par_chars: String = vchars.par_iter().collect();
    assert_eq!(s, par_chars);

    let par_bytes: Vec<u8> = s.par_bytes().collect();
    assert_eq!(s.as_bytes(), &*par_bytes);

    let par_utf16: Vec<u16> = s.par_encode_utf16().collect();
    let ser_utf16: Vec<u16> = s.encode_utf16().collect();
    assert_eq!(par_utf16, ser_utf16);

    let par_charind: Vec<_> = s.par_char_indices().collect();
    let ser_charind: Vec<_> = s.char_indices().collect();
    assert_eq!(par_charind, ser_charind);
}

#[test]
pub fn execute_strings_split() {
    // char testcases from examples in `str::split` etc.,
    // plus a large self-test for good measure.
    let tests = vec![
        ("Mary had a little lamb", ' '),
        ("", 'X'),
        ("lionXXtigerXleopard", 'X'),
        ("||||a||b|c", '|'),
        ("(///)", '/'),
        ("010", '0'),
        ("    a  b c", ' '),
        ("A.B.", '.'),
        ("A..B..", '.'),
        ("foo\r\nbar\n\nbaz\n", '\n'),
        ("foo\nbar\n\r\nbaz", '\n'),
        ("A few words", ' '),
        (" Mary   had\ta\u{2009}little  \n\t lamb", ' '),
        ("Mary had a little lamb\nlittle lamb\nlittle lamb.", '\n'),
        ("Mary had a little lamb\nlittle lamb\nlittle lamb.\n", '\n'),
        (include_str!("str.rs"), ' '),
    ];

    macro_rules! check_separators {
        ($split:ident, $par_split:ident) => {
            for &(string, separator) in &tests {
                let serial: Vec<_> = string.$split(separator).collect();
                let parallel: Vec<_> = string.$par_split(separator).collect();
                assert_eq!(serial, parallel);

                let array = ['\u{0}', separator, '\u{1F980}'];
                let array_ref = &array;
                let slice: &[char] = array_ref;

                let serial: Vec<_> = string.$split(slice).collect();
                let parallel: Vec<_> = string.$par_split(slice).collect();
                assert_eq!(serial, parallel);

                let serial: Vec<_> = string.$split(array).collect();
                let parallel: Vec<_> = string.$par_split(array).collect();
                assert_eq!(serial, parallel);

                let serial: Vec<_> = string.$split(array_ref).collect();
                let parallel: Vec<_> = string.$par_split(array_ref).collect();
                assert_eq!(serial, parallel);

                let serial_fn: Vec<_> = string.$split(|c| c == separator).collect();
                let parallel_fn: Vec<_> = string.$par_split(|c| c == separator).collect();
                assert_eq!(serial_fn, parallel_fn);
            }
        };
    }

    check_separators!(split, par_split);
    check_separators!(split_inclusive, par_split_inclusive);
    check_separators!(split_terminator, par_split_terminator);

    for &(string, _) in &tests {
        let serial: Vec<_> = string.lines().collect();
        let parallel: Vec<_> = string.par_lines().collect();
        assert_eq!(serial, parallel);
    }

    for &(string, _) in &tests {
        let serial: Vec<_> = string.split_whitespace().collect();
        let parallel: Vec<_> = string.par_split_whitespace().collect();
        assert_eq!(serial, parallel);
    }

    for &(string, _) in &tests {
        let serial: Vec<_> = string.split_ascii_whitespace().collect();
        let parallel: Vec<_> = string.par_split_ascii_whitespace().collect();
        assert_eq!(serial, parallel);
    }

    // try matching separators too!
    check_separators!(matches, par_matches);
    check_separators!(match_indices, par_match_indices);
}
