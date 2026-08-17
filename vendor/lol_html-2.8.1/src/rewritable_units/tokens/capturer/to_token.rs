use super::TokenCaptureFlags;
use crate::html::TextType;
use crate::parser::{NonTagContentLexeme, NonTagContentTokenOutline, TagLexeme, TagTokenOutline};
use crate::rewritable_units::{Attributes, Comment, Doctype, EndTag, StartTag, Token};
use encoding_rs::Encoding;

pub(crate) enum ToTokenResult<'i> {
    Token(Token<'i>),
    Text(TextType),
    None,
}

pub(crate) trait ToToken {
    fn to_token(
        &self,
        capture_flags: &mut TokenCaptureFlags,
        encoding: &'static Encoding,
    ) -> ToTokenResult<'_>;
}

impl ToToken for TagLexeme<'_> {
    #[inline]
    fn to_token(
        &self,
        capture_flags: &mut TokenCaptureFlags,
        encoding: &'static Encoding,
    ) -> ToTokenResult<'_> {
        match *self.token_outline() {
            TagTokenOutline::StartTag {
                name,
                ref attributes,
                ns,
                self_closing,
                ..
            } if capture_flags.contains(TokenCaptureFlags::NEXT_START_TAG) => {
                // NOTE: clear the flag once we've seen required start tag.
                capture_flags.remove(TokenCaptureFlags::NEXT_START_TAG);
                ToTokenResult::Token(StartTag::new_token(
                    self.part(name),
                    Attributes::new(self.input(), attributes, encoding),
                    ns,
                    self_closing,
                    self.spanned().into(),
                ))
            }

            TagTokenOutline::EndTag { name, .. }
                if capture_flags.contains(TokenCaptureFlags::NEXT_END_TAG) =>
            {
                // NOTE: clear the flag once we've seen required end tag.
                capture_flags.remove(TokenCaptureFlags::NEXT_END_TAG);
                ToTokenResult::Token(EndTag::new_token(
                    self.part(name),
                    self.spanned().into(),
                    encoding,
                ))
            }
            _ => ToTokenResult::None,
        }
    }
}

impl ToToken for NonTagContentLexeme<'_> {
    #[inline]
    fn to_token(
        &self,
        capture_flags: &mut TokenCaptureFlags,
        encoding: &'static Encoding,
    ) -> ToTokenResult<'_> {
        match self.token_outline() {
            Some(NonTagContentTokenOutline::Text(text_type))
                if capture_flags.contains(TokenCaptureFlags::TEXT) =>
            {
                ToTokenResult::Text(*text_type)
            }

            Some(NonTagContentTokenOutline::Comment(text))
                if capture_flags.contains(TokenCaptureFlags::COMMENTS) =>
            {
                ToTokenResult::Token(Comment::new_token(
                    self.part(*text),
                    self.spanned().into(),
                    encoding,
                ))
            }

            Some(NonTagContentTokenOutline::Doctype(doctype))
                if capture_flags.contains(TokenCaptureFlags::DOCTYPES) =>
            {
                ToTokenResult::Token(Doctype::new_token(
                    self.opt_part(doctype.name),
                    self.opt_part(doctype.public_id),
                    self.opt_part(doctype.system_id),
                    doctype.force_quirks,
                    false, // removed
                    self.spanned(),
                    encoding,
                ))
            }
            _ => ToTokenResult::None,
        }
    }
}
