use super::{char_debug_len, char_display_len, char_to_debug, char_to_display};

#[test]
fn char_to_utf8_encoding_test() {
    for c in '\0'..=core::char::MAX {
        let mut utf8_std = [0u8; 4];
        let utf8_std = c.encode_utf8(&mut utf8_std);

        let utf8_here = char_to_display(c);
        assert_eq!(utf8_here.len(), char_display_len(c));

        assert_eq!(utf8_std.as_bytes(), utf8_here.as_bytes());
    }
}

#[test]
fn char_to_utf8_display_test() {
    for c in '\0'..=core::char::MAX {
        let mut utf8_std = [0u8; 4];
        let utf8_std = c.encode_utf8(&mut utf8_std);

        let utf8_here = char_to_display(c);
        assert_eq!(utf8_here.len(), char_display_len(c));

        assert_eq!(utf8_std.as_bytes(), utf8_here.as_bytes());
    }
}

#[test]
fn char_to_utf8_debug_test() {
    let first_escapes = [
        ('\x00', r#"'\x00'"#),
        ('\x01', r#"'\x01'"#),
        ('\x02', r#"'\x02'"#),
        ('\x03', r#"'\x03'"#),
        ('\x04', r#"'\x04'"#),
        ('\x05', r#"'\x05'"#),
        ('\x06', r#"'\x06'"#),
        ('\x07', r#"'\x07'"#),
        ('\x08', r#"'\x08'"#),
        ('\t', r#"'\t'"#),
        ('\n', r#"'\n'"#),
        ('\x0B', r#"'\x0B'"#),
        ('\x0C', r#"'\x0C'"#),
        ('\r', r#"'\r'"#),
        ('\x0E', r#"'\x0E'"#),
        ('\x0F', r#"'\x0F'"#),
        ('\x10', r#"'\x10'"#),
        ('\x11', r#"'\x11'"#),
        ('\x12', r#"'\x12'"#),
        ('\x13', r#"'\x13'"#),
        ('\x14', r#"'\x14'"#),
        ('\x15', r#"'\x15'"#),
        ('\x16', r#"'\x16'"#),
        ('\x17', r#"'\x17'"#),
        ('\x18', r#"'\x18'"#),
        ('\x19', r#"'\x19'"#),
        ('\x1A', r#"'\x1A'"#),
        ('\x1B', r#"'\x1B'"#),
        ('\x1C', r#"'\x1C'"#),
        ('\x1D', r#"'\x1D'"#),
        ('\x1E', r#"'\x1E'"#),
        ('\x1F', r#"'\x1F'"#),
    ];

    for (c, expected) in first_escapes.iter().copied() {
        let utf8_here = char_to_debug(c);
        assert_eq!(expected.as_bytes(), utf8_here.as_bytes(), "{:?}", c);
        assert_eq!(expected.len(), char_debug_len(c), "{:?}", c);
    }

    let other_escapes = [('\'', r#"'\''"#), ('\"', r#"'\"'"#), ('\\', r#"'\\'"#)];

    let mut buffer = arrayvec::ArrayString::<12>::new();
    for c in '\x20'..=core::char::MAX {
        let utf8_here = char_to_debug(c);

        if let Some((_, expected)) = Some(c)
            .filter(|c| *c <= '\x7F')
            .and_then(|c| other_escapes.iter().copied().find(|x| x.0 == c))
        {
            assert_eq!(expected.as_bytes(), utf8_here.as_bytes(), "{:?}", c);
            assert_eq!(expected.len(), char_debug_len(c), "{:?}", c);
        } else {
            buffer.clear();
            buffer.push('\'');
            buffer.push(c);
            buffer.push('\'');
            assert_eq!(buffer.as_bytes(), utf8_here.as_bytes(), "{:?}", c);
            assert_eq!(buffer.len(), char_debug_len(c), "{:?}", c);
        }
    }
}
