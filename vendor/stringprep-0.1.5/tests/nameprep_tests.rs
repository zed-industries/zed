// Integration tests from https://tools.ietf.org/html/draft-josefsson-idn-test-vectors-00
extern crate stringprep;

use stringprep::{nameprep, Error};

fn assert_prohibited_character<T>(result: Result<T, Error>) {
    assert!(result.is_err());
}

fn assert_prohibited_bidirectional_text<T>(result: Result<T, Error>) {
    assert!(result.is_err());
}

// Additional examples from http://josefsson.org/idn.php
#[test]
fn test_nameprep() {
    assert_eq!(
        "安室奈美恵-with-super-monkeys",
        nameprep("安室奈美恵-with-SUPER-MONKEYS").unwrap()
    );
    assert_eq!("미술", nameprep("미술").unwrap()); // Korean
    assert_eq!("ليهمابتكلموشعربي؟", nameprep("ليهمابتكلموشعربي؟").unwrap()); // Egyptian
    assert_eq!(
        "他们为什么不说中文",
        nameprep("他们为什么不说中文").unwrap()
    ); // Chinese
    assert_eq!(
        "למההםפשוטלאמדבריםעברית",
        nameprep("למההםפשוטלאמדבריםעברית").unwrap()
    ); // Hebrew
    assert_eq!(
        "почемужеонинеговорятпорусски",
        nameprep("почемужеонинеговорятпорусски").unwrap()
    ); // Russian
    assert_eq!(
        "tạisaohọkhôngthểchỉnóitiếngviệt",
        nameprep("TạisaohọkhôngthểchỉnóitiếngViệt").unwrap()
    ); // Vietnamese
    assert_eq!("ひとつ屋根の下2", nameprep("ひとつ屋根の下2").unwrap()); // Japanese
    assert_eq!(
        "pročprostěnemluvíčesky",
        nameprep("Pročprostěnemluvíčesky").unwrap()
    ); // Czech
    assert_eq!(
        "यहलोगहिन्दीक्योंनहींबोलसकतेहैं",
        nameprep("यहलोगहिन्दीक्योंनहींबोलसकतेहैं").unwrap()
    ); // Hindi
    assert_eq!("ພາສາລາວ", nameprep("ພາສາລາວ").unwrap()); // Lao
    assert_eq!("bonġusaħħa", nameprep("bonġusaħħa").unwrap()); // Maltese
    assert_eq!("ελληνικά", nameprep("ελληνικά").unwrap()); // Greek
}

// 4.1 Map to nothing
#[test]
fn should_map_to_nothing() {
    let input =
        "foo\u{00ad}\u{034f}\u{1806}\u{180b}bar\u{200b}\u{2060}baz\u{fe00}\u{fe08}\u{fe0f}\u{feff}";
    assert_eq!("foobarbaz", nameprep(input).unwrap());
}

// 4.2 Case folding ASCII U+0043 U+0041 U+0046 U+0045
#[test]
fn should_case_fold_ascii() {
    assert_eq!("cafe", nameprep("CAFE").unwrap());
}

// 4.3 Case folding 8bit U+00DF (german sharp s)
#[test]
fn should_case_fold_8bit() {
    assert_eq!("ss", nameprep("\u{00df}").unwrap());
}

// 4.4 Case folding U+0130 (turkish capital I with dot)
#[test]
fn should_case_fold_16bit() {
    assert_eq!("\u{0069}\u{0307}", nameprep("\u{0130}").unwrap());
}

// 4.5 Case folding multibyte U+0143 U+037A
#[test]
fn should_case_fold_multibyte() {
    let input = "\u{0143}\u{037a}";
    let output = "\u{0144} \u{03b9}";
    assert_eq!(output, nameprep(input).unwrap());
}

// 4.6 Case folding U+2121 U+33C6 U+1D7BB
#[test]
fn should_case_fold() {
    let input = "\u{2121}\u{33c6}\u{1d7bb}";
    let output = "telc\u{2215}\u{006b}\u{0067}\u{03c3}";
    assert_eq!(output, nameprep(input).unwrap());
}

// 4.7 Normalization of U+006a U+030c U+00A0 U+00AA
#[test]
fn should_normalize() {
    let input = "j\u{030c}\u{00a0}\u{00aa}";
    let output = "\u{01f0} a";
    assert_eq!(output, nameprep(input).unwrap());
}

// 4.8 Case folding U+1FB7 and normalization
#[test]
fn should_case_fold_and_normalize() {
    let input = "\u{1fb7}";
    let output = "\u{1fb6}\u{03b9}";
    assert_eq!(output, nameprep(input).unwrap());
}

// 4.9  Self-reverting case folding U+01F0 and normalization
// 4.10 Self-reverting case folding U+0390 and normalization
// 4.11 Self-reverting case folding U+03B0 and normalization
// 4.12 Self-reverting case folding U+1E96 and normalization
// 4.13 Self-reverting case folding U+1F56 and normalization
#[test]
fn should_revert_case_fold_and_normalization() {
    let inputs = ["\u{01f0}", "\u{0390}", "\u{03b0}", "\u{1e96}", "\u{1f56}"];
    for input in inputs.iter() {
        assert_eq!(*input, nameprep(input).unwrap());
    }
}

// 4.14 ASCII space character U+0020
#[test]
fn should_permit_ascii_space() {
    assert_eq!(" ", nameprep(" ").unwrap());
}

// 4.15 Non-ASCII 8bit space character U+00A0
#[test]
fn should_map_8bit_space() {
    assert_eq!(" ", nameprep("\u{00a0}").unwrap());
}

// 4.16 Non-ASCII multibyte space character U+1680
#[test]
fn should_prohibit_multibyte_space() {
    assert_prohibited_character(nameprep("\u{1680}"));
}

// 4.17 Non-ASCII multibyte space character U+2000
#[test]
fn should_map_multibyte_space1() {
    assert_eq!(" ", nameprep("\u{2000}").unwrap());
}

// 4.18 Zero Width Space U+200b
#[test]
fn should_drop_zero_width_space() {
    assert_eq!("", nameprep("\u{200b}").unwrap());
}

// 4.19 Non-ASCII multibyte space character U+3000
#[test]
fn should_map_multibyte_space2() {
    assert_eq!(" ", nameprep("\u{3000}").unwrap());
}

// 4.20 ASCII control characters U+0010 U+007F
#[test]
fn should_permit_ascii_control() {
    assert_eq!("\u{0010}\u{007f}", nameprep("\u{0010}\u{007f}").unwrap());
}

// 4.21 Non-ASCII 8bit control character U+0085
#[test]
fn should_prohibit_8bit_control() {
    assert_prohibited_character(nameprep("\u{0085}"));
}

// 4.22 Non-ASCII multibyte control character U+180E
#[test]
fn should_prohibit_multibyte_control() {
    assert_prohibited_character(nameprep("\u{180e}"));
}

// 4.23 Zero Width No-Break Space U+FEFF
#[test]
fn should_drop_zero_width_no_break_space() {
    assert_eq!("", nameprep("\u{feff}").unwrap());
}

// 4.24 Non-ASCII control character U+1D175
#[test]
fn should_prohibit_non_ascii_control() {
    assert_prohibited_character(nameprep("\u{1d175}"));
}

// 4.25 Plane 0 private use character U+F123
#[test]
fn should_prohibit_plane0_private_use() {
    assert_prohibited_character(nameprep("\u{f123}"));
}

// 4.26 Plane 15 private use character U+F1234
#[test]
fn should_prohibit_plane15_private_use() {
    assert_prohibited_character(nameprep("\u{f1234}"));
}

// 4.27 Plane 16 private use character U+10F234
#[test]
fn should_prohibit_plane16_private_use() {
    assert_prohibited_character(nameprep("\u{10f234}"));
}

// 4.28 Non-character code point U+8FFFE
#[test]
fn should_prohibit_non_character1() {
    assert_prohibited_character(nameprep("\u{8fffe}"));
}

// 4.29 Non-character code point U+10FFFF
#[test]
fn should_prohibit_non_character2() {
    assert_prohibited_character(nameprep("\u{10ffff}"));
}

// 4.31 Non-plain text character U+FFFD
#[test]
fn should_prohibit_non_plain_text() {
    assert_prohibited_character(nameprep("\u{fffd}"));
}

// 4.32 Ideographic description character U+2FF5
#[test]
fn should_prohibit_ideographic_description() {
    assert_prohibited_character(nameprep("\u{2ff5}"));
}

// 4.33 Display property character U+0341
#[test]
fn should_normalize_display_property() {
    assert_eq!("\u{0301}", nameprep("\u{0341}").unwrap());
}

// 4.34 Left-to-right mark U+200E
#[test]
fn should_prohibit_left_to_right_mark() {
    assert_prohibited_character(nameprep("\u{200e}"));
}

// 4.35 Deprecated U+202A
#[test]
fn should_prohibit_deprecated() {
    assert_prohibited_character(nameprep("\u{202a}"));
}

// 4.36 Language tagging character U+E0001
#[test]
fn should_prohibit_language_tagging1() {
    assert_prohibited_character(nameprep("\u{e0001}"));
}

// 4.37 Language tagging character U+E0042
#[test]
fn should_prohibit_language_tagging2() {
    assert_prohibited_character(nameprep("\u{e0042}"));
}

// 4.38 Bidi: RandALCat character U+05BE and LCat characters
#[test]
fn should_prohibit_randalcat_with_lcat1() {
    assert_prohibited_bidirectional_text(nameprep("foo\u{05be}bar"));
}

// 4.39 Bidi: RandALCat character U+FD50 and LCat characters
#[test]
fn should_prohibit_randalcat_with_lcat2() {
    assert_prohibited_bidirectional_text(nameprep("foo\u{fd50}bar"));
}

// 4.40 Bidi: RandALCat character U+FB38 and LCat characters
#[test]
fn should_permit_randalcat1() {
    assert_eq!("foo \u{064e}bar", nameprep("foo\u{fe76}bar").unwrap());
}

// 4.41 Bidi: RandALCat without trailing RandALCat U+0627 U+0031
#[test]
fn should_prohibit_mixed_randalcat() {
    assert_prohibited_bidirectional_text(nameprep("\u{0672}\u{0031}"));
}

// 4.42 Bidi: RandALCat character U+0627 U+0031 U+0628
#[test]
fn should_permit_randalcat2() {
    assert_eq!(
        "\u{0627}\u{0031}\u{0628}",
        nameprep("\u{0627}\u{0031}\u{0628}").unwrap()
    );
}

// 4.43 Unassigned code point U+E0002
#[test]
fn should_prohibit_unassigned_code_point() {
    assert_prohibited_character(nameprep("\u{e0002}"));
}

// 4.44 Larger test (shrinking)
#[test]
fn should_shrink() {
    let input = "X\u{00ad}\u{00df}\u{0130}\u{2121}j\u{030c}\u{00a0}\u{00aa}\u{03b0}\u{2000}";
    let output = "xssi\u{0307}tel\u{01f0} a\u{03b0}\u{0020}";
    assert_eq!(output, nameprep(input).unwrap());
}

// 4.45 Larger test (expanding)
#[test]
fn should_expand() {
    let input = "X\u{00df}\u{3316}\u{0130}\u{2121}\u{249f}\u{3300}";
    let output = "xss\u{30ad}\u{30ed}\u{30e1}\u{30fc}\u{30c8}\u{30eb}\u{0069}\u{0307}\u{0074}\u{0065}\u{006c}\u{0028}\u{0064}\u{0029}\u{30a2}\u{30d1}\u{30fc}\u{30c8}";
    assert_eq!(output, nameprep(input).unwrap());
}
