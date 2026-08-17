#![cfg(feature = "parser")]

use crate::error::ErrMode;
use crate::error::ErrMode::Backtrack;
use crate::error::InputError;
use crate::token::literal;
use crate::{
    combinator::{separated, separated_pair},
    ModalResult, Parser,
};

use super::*;

#[cfg(feature = "std")]
#[test]
fn test_fxhashmap_compiles() {
    let input = "a=b";
    fn pair(i: &mut &str) -> ModalResult<(char, char)> {
        let out = separated_pair('a', '=', 'b').parse_next(i)?;
        Ok(out)
    }

    let _: rustc_hash::FxHashMap<char, char> = separated(0.., pair, ',').parse(input).unwrap();
}

#[test]
fn test_offset_u8() {
    let s = b"abcd123";
    let a = &s[..];
    let b = &a[2..];
    let c = &a[..4];
    let d = &a[3..5];
    assert_eq!(b.offset_from(&a), 2);
    assert_eq!(c.offset_from(&a), 0);
    assert_eq!(d.offset_from(&a), 3);
}

#[test]
fn test_offset_str() {
    let a = "abcřèÂßÇd123";
    let b = &a[7..];
    let c = &a[..5];
    let d = &a[5..9];
    assert_eq!(b.offset_from(&a), 7);
    assert_eq!(c.offset_from(&a), 0);
    assert_eq!(d.offset_from(&a), 5);
}

#[test]
fn test_partial_complete() {
    let mut i = Partial::new(&b""[..]);
    assert!(Partial::<&[u8]>::is_partial_supported());

    assert!(i.is_partial(), "incomplete by default");
    let incomplete_state = i.complete();
    assert!(!i.is_partial(), "the stream should be marked as complete");

    i.restore_partial(incomplete_state);
    assert!(i.is_partial(), "incomplete stream state should be restored");
}

#[test]
fn test_custom_slice() {
    type Token = usize;
    type TokenSlice<'i> = &'i [Token];

    let mut tokens: TokenSlice<'_> = &[1, 2, 3, 4];

    let input = &mut tokens;
    let start = input.checkpoint();
    let _ = input.next_token();
    let _ = input.next_token();
    let offset = input.offset_from(&start);
    assert_eq!(offset, 2);
}

#[test]
fn test_literal_support_char() {
    assert_eq!(
        literal::<_, _, ErrMode<InputError<_>>>('π').parse_peek("π"),
        Ok(("", "π"))
    );
    assert_eq!(
        literal::<_, _, ErrMode<InputError<_>>>('π').parse_peek("π3.14"),
        Ok(("3.14", "π"))
    );

    assert_eq!(
        literal::<_, _, ErrMode<InputError<_>>>("π").parse_peek("π3.14"),
        Ok(("3.14", "π"))
    );

    assert_eq!(
        literal::<_, _, ErrMode<InputError<_>>>('-').parse_peek("π"),
        Err(Backtrack(InputError::at("π")))
    );

    assert_eq!(
        literal::<_, Partial<&[u8]>, ErrMode<InputError<_>>>('π')
            .parse_peek(Partial::new(b"\xCF\x80")),
        Ok((Partial::new(Default::default()), "π".as_bytes()))
    );
    assert_eq!(
        literal::<_, &[u8], ErrMode<InputError<_>>>('π').parse_peek(b"\xCF\x80"),
        Ok((Default::default(), "π".as_bytes()))
    );

    assert_eq!(
        literal::<_, Partial<&[u8]>, ErrMode<InputError<_>>>('π')
            .parse_peek(Partial::new(b"\xCF\x803.14")),
        Ok((Partial::new(&b"3.14"[..]), "π".as_bytes()))
    );
    assert_eq!(
        literal::<_, &[u8], ErrMode<InputError<_>>>('π').parse_peek(b"\xCF\x80"),
        Ok((Default::default(), "π".as_bytes()))
    );

    assert_eq!(
        literal::<_, &[u8], ErrMode<InputError<_>>>('π').parse_peek(b"\xCF\x803.14"),
        Ok((&b"3.14"[..], "π".as_bytes()))
    );

    assert_eq!(
        literal::<_, &[u8], ErrMode<InputError<_>>>('-').parse_peek(b"\xCF\x80"),
        Err(Backtrack(InputError::at(&b"\xCF\x80"[..],)))
    );
}

#[test]
fn tokenslice_location() {
    #[derive(Clone, Debug)]
    struct Token {
        span: core::ops::Range<usize>,
    }

    impl Location for Token {
        #[inline(always)]
        fn previous_token_end(&self) -> usize {
            self.span.end
        }
        #[inline(always)]
        fn current_token_start(&self) -> usize {
            self.span.start
        }
    }

    let input = [
        Token { span: 1..9 },
        Token { span: 11..19 },
        Token { span: 21..29 },
    ];
    let mut input = TokenSlice::new(&input);
    assert_eq!(input.previous_token_end(), 1);

    // Parse operation
    assert_eq!(input.current_token_start(), 1);
    let _ = input.next_token();
    assert_eq!(input.previous_token_end(), 9);

    // Parse operation
    assert_eq!(input.current_token_start(), 11);
    let _ = input.next_token();
    assert_eq!(input.previous_token_end(), 19);

    // Parse operation
    assert_eq!(input.current_token_start(), 21);
    let _ = input.next_token();
    assert_eq!(input.previous_token_end(), 29);

    assert_eq!(input.current_token_start(), 29);
}
