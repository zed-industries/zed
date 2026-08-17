// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::{Meaning, Byte, classify, decode, all_cont};

use std::borrow::ToOwned;
use std::io::Write;
use std_test::Bencher;

#[test]
fn classify_all_bytes() {
    for n in 0x00..0x80 { assert_eq!(Byte::classify(n), Some(Byte::Ascii)); }
    for n in 0x80..0xC0 { assert_eq!(Byte::classify(n), Some(Byte::Cont)); }
    for n in 0xC0..0xE0 { assert_eq!(Byte::classify(n), Some(Byte::Start(2))); }
    for n in 0xE0..0xF0 { assert_eq!(Byte::classify(n), Some(Byte::Start(3))); }
    for n in 0xF0..0xF8 { assert_eq!(Byte::classify(n), Some(Byte::Start(4))); }
    for n in 0xF8..0xFF { assert_eq!(Byte::classify(n), None); }
    assert_eq!(Byte::classify(0xFF), None);
}

#[test]
fn test_all_cont() {
    assert!(all_cont(b""));
    assert!(all_cont(b"\x80"));
    assert!(all_cont(b"\xBF"));
    assert!(all_cont(b"\x80\xBF\x80\xBF"));

    assert!(!all_cont(b"z"));
    assert!(!all_cont(b"\xC0\xBF"));
    assert!(!all_cont(b"\xFF"));
    assert!(!all_cont(b"\x80\xBFz\x80\xBF"));
    assert!(!all_cont(b"\x80\xBF\xC0\x80\xBF"));
    assert!(!all_cont(b"\x80\xBF\xFF\x80\xBF"));
    assert!(!all_cont(b"\x80\xBF\x80\xBFz"));
    assert!(!all_cont(b"\x80\xBF\x80\xBF\xC0"));
    assert!(!all_cont(b"z\x80\xBF\x80\xBF"));
    assert!(!all_cont(b"\xC0\x80\xBF\x80\xBF"));
}

#[test]
fn test_decode() {
    unsafe {
        assert_eq!(Some(Meaning::Whole('ő')), decode(b"\xC5\x91"));
        assert_eq!(Some(Meaning::Whole('\u{a66e}')), decode(b"\xEA\x99\xAE"));
        assert_eq!(Some(Meaning::Whole('\u{1f4a9}')), decode(b"\xF0\x9F\x92\xA9"));
        assert_eq!(Some(Meaning::Whole('\u{10ffff}')), decode(b"\xF4\x8F\xBF\xBF"));

        assert_eq!(Some(Meaning::LeadSurrogate(0x0000)), decode(b"\xED\xA0\x80"));
        assert_eq!(Some(Meaning::LeadSurrogate(0x0001)), decode(b"\xED\xA0\x81"));
        assert_eq!(Some(Meaning::LeadSurrogate(0x03FE)), decode(b"\xED\xAF\xBE"));
        assert_eq!(Some(Meaning::LeadSurrogate(0x03FF)), decode(b"\xED\xAF\xBF"));

        assert_eq!(Some(Meaning::TrailSurrogate(0x0000)), decode(b"\xED\xB0\x80"));
        assert_eq!(Some(Meaning::TrailSurrogate(0x0001)), decode(b"\xED\xB0\x81"));
        assert_eq!(Some(Meaning::TrailSurrogate(0x03FE)), decode(b"\xED\xBF\xBE"));
        assert_eq!(Some(Meaning::TrailSurrogate(0x03FF)), decode(b"\xED\xBF\xBF"));

        // The last 4-byte UTF-8 sequence. This would be U+1FFFFF, which is out of
        // range.
        assert_eq!(None, decode(b"\xF7\xBF\xBF\xBF"));

        // First otherwise-valid sequence (would be U+110000) that is out of range
        assert_eq!(None, decode(b"\xF4\x90\x80\x80"));

        // Overlong sequences
        assert_eq!(None, decode(b"\xC0\x80"));
        assert_eq!(None, decode(b"\xC1\xBF"));
        assert_eq!(None, decode(b"\xE0\x80\x80"));
        assert_eq!(None, decode(b"\xE0\x9F\xBF"));
        assert_eq!(None, decode(b"\xF0\x80\x80\x80"));
        assert_eq!(None, decode(b"\xF0\x8F\xBF\xBF"));

        // For not-overlong sequence for each sequence length
        assert_eq!(Some(Meaning::Whole('\u{80}')), decode(b"\xC2\x80"));
        assert_eq!(Some(Meaning::Whole('\u{800}')), decode(b"\xE0\xA0\x80"));
        assert_eq!(Some(Meaning::Whole('\u{10000}')), decode(b"\xF0\x90\x80\x80"));
   }
}

static JUNK: &'static [u8] = b"\
    \xf8\x0d\x07\x25\xa6\x7b\x95\xeb\x47\x01\x7f\xee\
    \x3b\x00\x60\x57\x1d\x9e\x5d\x0a\x0b\x0a\x7c\x75\
    \x13\xa1\x82\x46\x27\x34\xe9\x52\x61\x0d\xec\x10\
    \x54\x49\x6e\x54\xdf\x7b\xe1\x31\x8c\x06\x21\x83\
    \x0f\xb5\x1f\x4c\x6a\x71\x52\x42\x74\xe7\x7b\x50\
    \x59\x1f\x6a\xd4\xff\x06\x92\x33\xc4\x34\x97\xff\
    \xcc\xb5\xc4\x00\x7b\xc3\x4a\x7f\x7e\x63\x96\x58\
    \x51\x63\x21\x54\x53\x2f\x03\x8a\x7d\x41\x79\x98\
    \x5b\xcb\xb8\x94\x6b\x73\xf3\x0c\x5a\xd7\xc4\x12\
    \x7a\x2b\x9a\x2e\x67\x62\x2a\x00\x45\x2c\xfe\x7d\
    \x8d\xd6\x51\x4e\x59\x36\x72\x1b\xae\xaa\x06\xe8\
    \x71\x1b\x85\xd3\x35\xb5\xbe\x9e\x16\x96\x72\xd8\
    \x1a\x48\xba\x4d\x55\x4f\x1b\xa2\x77\xfa\x8f\x71\
    \x58\x7d\x03\x93\xa2\x3a\x76\x51\xda\x48\xe2\x3f\
    \xeb\x8d\xda\x89\xae\xf7\xbd\x3d\xb6\x37\x97\xca\
    \x99\xcc\x4a\x8d\x62\x89\x97\xe3\xc0\xd1\x8d\xc1\
    \x26\x11\xbb\x8d\x53\x61\x4f\x76\x03\x00\x30\xd3\
    \x5f\x86\x19\x52\x9c\x3e\x99\x8c\xb7\x21\x48\x1c\
    \x85\xae\xad\xd5\x74\x00\x6c\x3e\xd0\x17\xff\x76\
    \x5c\x32\xc3\xfb\x24\x99\xd4\x4c\xa4\x1f\x66\x46\
    \xe7\x2d\x44\x56\x7d\x14\xd9\x76\x91\x37\x2f\xb7\
    \xcc\x1b\xd3\xc2";

#[test]
fn classify_whole() {
    assert_eq!(JUNK.len(), 256);

    for &c in &['\0', '\x01', 'o', 'z', 'ő', '\u{2764}',
                '\u{a66e}', '\u{1f4a9}', '\u{1f685}'] {
        for idx in 0 .. JUNK.len() - 3 {
            let mut buf = JUNK.to_owned();
            let ch = format!("{}", c).into_bytes();
            (&mut buf[idx..]).write_all(&ch).unwrap();

            for j in 0 .. ch.len() {
                let class = classify(&buf, idx+j).unwrap();
                assert_eq!(class.bytes, &*ch);
                assert_eq!(class.rewind, j);
                assert_eq!(class.meaning, Meaning::Whole(c));
            }
        }
    }
}

#[test]
fn classify_surrogates() {
    for &(s, b) in &[
        (Meaning::LeadSurrogate(0x0000), b"\xED\xA0\x80"),
        (Meaning::LeadSurrogate(0x0001), b"\xED\xA0\x81"),
        (Meaning::LeadSurrogate(0x03FE), b"\xED\xAF\xBE"),
        (Meaning::LeadSurrogate(0x03FF), b"\xED\xAF\xBF"),

        (Meaning::TrailSurrogate(0x0000), b"\xED\xB0\x80"),
        (Meaning::TrailSurrogate(0x0001), b"\xED\xB0\x81"),
        (Meaning::TrailSurrogate(0x03FE), b"\xED\xBF\xBE"),
        (Meaning::TrailSurrogate(0x03FF), b"\xED\xBF\xBF"),
    ] {
        for idx in 0 .. JUNK.len() - 2 {
            let mut buf = JUNK.to_owned();
            (&mut buf[idx..]).write_all(b).unwrap();

            let class = classify(&buf, idx).unwrap();
            assert_eq!(class.bytes, b);
            assert_eq!(class.rewind, 0);
            assert_eq!(class.meaning, s);
        }
    }
}

#[test]
fn classify_prefix_suffix() {
    for &c in &['ő', '\u{a66e}', '\u{1f4a9}'] {
        let ch = format!("{}", c).into_bytes();
        for pfx in 1 .. ch.len() - 1 {
            let mut buf = JUNK.to_owned();
            let buflen = buf.len();
            (&mut buf[buflen - pfx .. buflen]).write_all(&ch[..pfx]).unwrap();
            for j in 0 .. pfx {
                let idx = buflen - 1 - j;
                let class = classify(&buf, idx).unwrap();
                assert_eq!(class.bytes, &ch[..pfx]);
                assert_eq!(class.rewind, pfx - 1 - j);
                assert_eq!(class.meaning, Meaning::Prefix(ch.len() - pfx));
            }
        }
        for sfx in 1 .. ch.len() - 1 {
            let ch_bytes = &ch[ch.len() - sfx ..];
            let mut buf = JUNK.to_owned();
            (&mut *buf).write_all(ch_bytes).unwrap();
            for j in 0 .. sfx {
                let class = classify(&buf, j).unwrap();
                assert!(ch_bytes.starts_with(class.bytes));
                assert_eq!(class.rewind, j);
                assert_eq!(class.meaning, Meaning::Suffix);
            }
        }
    }
}

#[test]
fn out_of_bounds() {
    assert!(classify(b"", 0).is_none());
    assert!(classify(b"", 7).is_none());
    assert!(classify(b"aaaaaaa", 7).is_none());
}

#[test]
fn malformed() {
    assert_eq!(None, classify(b"\xFF", 0));
    assert_eq!(None, classify(b"\xC5\xC5", 0));
    assert_eq!(None, classify(b"x\x91", 1));
    assert_eq!(None, classify(b"\x91\x91\x91\x91", 3));
    assert_eq!(None, classify(b"\x91\x91\x91\x91\x91", 4));
    assert_eq!(None, classify(b"\xEA\x91\xFF", 1));
    assert_eq!(None, classify(b"\xF0\x90\x90\xF0", 0));
    assert_eq!(None, classify(b"\xF0\x90\x90\xF0", 1));
    assert_eq!(None, classify(b"\xF0\x90\x90\xF0", 2));

    for i in 0..4 {
        // out of range: U+110000
        assert_eq!(None, classify(b"\xF4\x90\x80\x80", i));

        // out of range: U+1FFFFF
        assert_eq!(None, classify(b"\xF7\xBF\xBF\xBF", i));

        // Overlong sequences
        assert_eq!(None, classify(b"\xC0\x80", i));
        assert_eq!(None, classify(b"\xC1\xBF", i));
        assert_eq!(None, classify(b"\xE0\x80\x80", i));
        assert_eq!(None, classify(b"\xE0\x9F\xBF", i));
        assert_eq!(None, classify(b"\xF0\x80\x80\x80", i));
        assert_eq!(None, classify(b"\xF0\x8F\xBF\xBF", i));
    }
}

static TEXT: &'static str = "
    All human beings are born free and equal in dignity and rights.
    They are endowed with reason and conscience and should act
    towards one another in a spirit of brotherhood.

    Minden emberi lény szabadon születik és egyenlő méltósága és
    joga van. Az emberek, ésszel és lelkiismerettel bírván,
    egymással szemben testvéri szellemben kell hogy viseltessenek.

    เราทุกคนเกิดมาอย่างอิสระ เราทุกคนมีความคิดและความเข้าใจเป็นของเราเอง
    เราทุกคนควรได้รับการปฏิบัติในทางเดียวกัน.

    모든 인간은 태어날 때부터 자유로우며 그 존엄과 권리에 있어
    동등하다. 인간은 천부적으로 이성과 양심을 부여받았으며 서로
    형제애의 정신으로 행동하여야 한다.

    ro remna cu se jinzi co zifre je simdu'i be le ry. nilselsi'a
    .e lei ry. selcru .i ry. se menli gi'e se sezmarde .i .ei
    jeseki'ubo ry. simyzu'e ta'i le tunba

    ᏂᎦᏓ ᎠᏂᏴᏫ ᏂᎨᎫᏓᎸᎾ ᎠᎴ ᎤᏂᏠᏱ ᎤᎾᏕᎿ ᏚᏳᎧᏛ ᎨᏒᎢ. ᎨᏥᏁᎳ ᎤᎾᏓᏅᏖᏗ ᎠᎴ ᎤᏃᏟᏍᏗ
    ᎠᎴ ᏌᏊ ᎨᏒ ᏧᏂᎸᏫᏍᏓᏁᏗ ᎠᎾᏟᏅᏢ ᎠᏓᏅᏙ ᎬᏗ.";

// random
static IXES: &'static [usize]
    = &[778, 156, 87, 604, 1216, 365, 884, 311,
        469, 515, 709, 162, 871, 206, 634, 442];

static BOUNDARY: &'static [bool]
    = &[false, true, true, false, false, true, true, true,
        true, false, false, true, true, true, false, false];

#[bench]
fn std_utf8_check(b: &mut Bencher) {
    b.iter(|| {
        assert!(IXES.iter().zip(BOUNDARY.iter()).all(|(&ix, &expect)| {
            expect == TEXT.is_char_boundary(ix)
        }));
    });
}

// We don't expect to be as fast as is_char_boundary, because we provide more
// information. But we shouldn't be tremendously slower, either. A factor of
// 5-10 is expected on this text.
#[bench]
fn futf_check(b: &mut Bencher) {
    b.iter(|| {
        assert!(IXES.iter().zip(BOUNDARY.iter()).all(|(&ix, &expect)| {
            expect == (::classify(TEXT.as_bytes(), ix).unwrap().rewind == 0)
        }));
    });
}
