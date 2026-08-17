use crate::parser::{CommentParser, ElementParser, Parser, PiParser};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DtdParser {
    /// If inside a PubidLiteral or SystemLiteral, it holds the quote type (either `'` or `"`).
    /// Otherwise, it holds `0` (this is an initial state).
    ///
    /// ```text
    /// [28]    doctypedecl     ::=   '<!DOCTYPE' S Name (S ExternalID)? S? ('[' intSubset ']' S?)? '>'
    /// ```
    BeforeInternalSubset(u8),
    /// Inside of the `intSubset` rule.
    ///
    /// ```text
    /// [28a]   DeclSep         ::=   PEReference | S
    /// [28b]   intSubset       ::=   (markupdecl | DeclSep)*
    /// [29]    markupdecl      ::=   elementdecl | AttlistDecl | EntityDecl | NotationDecl | PI | Comment
    /// ```
    InsideOfInternalSubset,
    /// After `]` but before `>`.
    AfterInternalSubset,
    InComment(CommentParser),
    InPi(PiParser),
    /// ```text
    /// [45]    elementdecl     ::=   '<!ELEMENT' S Name S contentspec S? '>'
    /// ```
    InElementDecl,
    /// This state handles ATTLIST, ENTITY and NOTATION elements, i.e. all elements that can have
    /// quotes strings (`'...'` or `"..."`) inside their markup, in which `>` should not be threated
    /// as the end of the markup.
    ///
    /// This state handles the following productions from XML grammar:
    ///
    /// ### ATTLIST
    ///
    /// ```text
    /// [52]    AttlistDecl     ::=   '<!ATTLIST' S Name AttDef* S? '>'
    /// [53]    AttDef          ::=   S Name S AttType S DefaultDecl
    /// [60]    DefaultDecl     ::=   '#REQUIRED' | '#IMPLIED' | (('#FIXED' S)? AttValue)
    /// ```
    ///
    /// ### ENTITY
    ///
    /// ```text
    /// [70]    EntityDecl      ::=   GEDecl | PEDecl
    /// [71]    GEDecl          ::=   '<!ENTITY' S Name S EntityDef S? '>'
    /// [72]    PEDecl          ::=   '<!ENTITY' S '%' S Name S PEDef S? '>'
    /// [73]    EntityDef       ::=   EntityValue | (ExternalID NDataDecl?)
    /// [74]    PEDef           ::=   EntityValue | ExternalID
    /// [75]    ExternalID      ::=   'SYSTEM' S SystemLiteral | 'PUBLIC' S PubidLiteral S SystemLiteral
    /// [76]    NDataDecl       ::=   S 'NDATA' S Name
    /// ```
    ///
    /// ### NOTATION
    ///
    /// ```text
    /// [82]    NotationDecl    ::=   '<!NOTATION' S Name S (ExternalID | PublicID) S? '>'
    /// ```
    InQuoteSensitive(ElementParser),
    /// The state where it was not possible to determine which markup it was during the previous iteration.  \
    /// It holds the number of bytes read since the start of the markup.
    UndecidedMarkup(usize),
    Finished,
}

impl DtdParser {
    /// Skip DTD contents.
    ///
    /// # Parameters (as same as `reader::BangType::parse`)
    /// - `buf`: buffer with data consumed on previous iterations
    /// - `chunk`: data read on current iteration and not yet consumed from reader
    pub fn feed(&mut self, buf: &[u8], chunk: &[u8]) -> Option<usize> {
        // This method assumes the DTD is well-formed.
        // Since this crate does not support parsing DTDs, the inability to read non-well-formed DTDs
        // is not particularly problematic; the only point of interest is reporting well-formed DTDs
        // to the user without errors.

        let mut cur = chunk;
        while !cur.is_empty() {
            match *self {
                Self::BeforeInternalSubset(0) => {
                    // Find the
                    // - start of quoted string ('...' or "...")
                    // - start of internal subset ([...])
                    // - end of DOCTYPE declaration (>)
                    if let Some(i) = cur
                        .iter()
                        .position(|&b| matches!(b, b'\'' | b'"' | b'[' | b'>'))
                    {
                        let b = cur[i];
                        match b {
                            b'\'' | b'"' => {
                                // SystemLiteral or PubidLiteral
                                *self = Self::BeforeInternalSubset(b);
                                cur = &cur[i + 1..]; // +1 to skip `'` or `"`
                                continue;
                            }
                            b'[' => {
                                *self = Self::InsideOfInternalSubset;
                                cur = &cur[i + 1..]; // +1 to skip `[`
                                continue;
                            }
                            b'>' => {
                                *self = Self::Finished;
                                return Some(chunk.len() - cur.len() + i);
                            }
                            _ => {}
                        }
                        continue;
                    }
                    break;
                }
                // Inside the quoted string (this is PubidLiteral or SystemLiteral) we do not want to
                // recognize other special characters (namely [ and >). Find only the closing quote
                Self::BeforeInternalSubset(quote) => {
                    // ExternalID handling
                    if let Some(i) = memchr::memchr(quote, cur) {
                        *self = Self::BeforeInternalSubset(0);
                        cur = &cur[i + 1..];
                        continue;
                    }
                    break;
                }
                Self::InsideOfInternalSubset => {
                    // Find the end of internal subset (]) or the start of the markup inside (<)
                    if let Some(i) = memchr::memchr2(b']', b'<', cur) {
                        if cur[i] == b']' {
                            *self = Self::AfterInternalSubset;
                            cur = &cur[i + 1..]; // +1 to skip `]`
                            continue;
                        }
                        // +1 to start after `<`
                        if let Some(skip) = self.switch(&cur[i + 1..]) {
                            cur = &cur[i + 1 + skip..]; // +1 to skip `<`
                            continue;
                        }
                        // Keep the number of already looked bytes (started from byte after `<`, so -1),
                        // try to decide after feeding the new chunk
                        *self = Self::UndecidedMarkup(cur.len() - i - 1);
                    }
                    break;
                }
                Self::AfterInternalSubset => {
                    if let Some(i) = memchr::memchr(b'>', cur) {
                        *self = Self::Finished;
                        return Some(chunk.len() - cur.len() + i);
                    }
                    break;
                }
                Self::InComment(ref mut parser) => {
                    // If comment is ended, return to the main state, otherwise keep in the current state
                    if let Some(i) = parser.feed(cur) {
                        *self = Self::InsideOfInternalSubset;
                        cur = &cur[i..];
                        continue;
                    }
                    break;
                }
                Self::InPi(ref mut parser) => {
                    // If processing instruction is ended, return to the main state,
                    // otherwise keep in the current state
                    if let Some(i) = parser.feed(cur) {
                        *self = Self::InsideOfInternalSubset;
                        cur = &cur[i..];
                        continue;
                    }
                    break;
                }
                Self::InElementDecl => {
                    // `<!ELEMENT >` does not have places where `>` could be escaped
                    // so the first occurrence ends that state
                    if let Some(i) = memchr::memchr(b'>', cur) {
                        *self = Self::InsideOfInternalSubset;
                        cur = &cur[i + 1..]; // +1 for `>`
                        continue;
                    }
                    break;
                }
                Self::InQuoteSensitive(ref mut parser) => {
                    // If ATTLIST, ENTITY or NOTATION is ended, return to the main state,
                    // otherwise keep in the current state
                    if let Some(i) = parser.feed(cur) {
                        *self = Self::InsideOfInternalSubset;
                        cur = &cur[i..];
                        continue;
                    }
                    break;
                }
                Self::UndecidedMarkup(skipped) => {
                    // Buffer is long enough to store the longest possible keyword `!NOTATION`
                    let mut bytes = [0u8; 9];

                    // Copy the last `skipped` bytes from the previous iteration into buffer,
                    // for example, "!NOT" (skipped = 4 in that case)...
                    bytes[..skipped].copy_from_slice(&buf[buf.len() - skipped..]);

                    // ...add new bytes to the buffer from current iteration,
                    // for example, "ATION"...
                    let end = bytes.len().min(skipped + cur.len());
                    bytes[skipped..end].copy_from_slice(&cur[..end - skipped]);

                    // ...and try to match over it.
                    // For example, "!NOTATION" will return 9, and we skip 9-4=5 bytes of "ATION"
                    if let Some(skip) = self.switch(&bytes[..end]) {
                        cur = &cur[skip - skipped..];
                        continue;
                    }
                    *self = Self::UndecidedMarkup(skipped + cur.len());
                    break;
                }
                Self::Finished => break,
            }
        }

        None
    }

    #[inline]
    fn switch(&mut self, markup: &[u8]) -> Option<usize> {
        match markup {
            [b'?', ..] => {
                // <?
                *self = Self::InPi(PiParser(false));
                Some(1)
            }
            [b'!', b'-', b'-', ..] => {
                // <!--
                *self = Self::InComment(CommentParser::Seen0);
                Some(3)
            }
            [b'!', b'E', b'L', b'E', b'M', b'E', b'N', b'T', ..] => {
                // <!ELEMENT
                *self = Self::InElementDecl;
                Some(8)
            }
            [b'!', b'E', b'N', b'T', b'I', b'T', b'Y', ..] => {
                // <!ENTITY
                *self = Self::InQuoteSensitive(ElementParser::Outside);
                Some(7)
            }
            [b'!', b'A', b'T', b'T', b'L', b'I', b'S', b'T', ..] => {
                // <!ATTLIST
                *self = Self::InQuoteSensitive(ElementParser::Outside);
                Some(8)
            }
            [b'!', b'N', b'O', b'T', b'A', b'T', b'I', b'O', b'N', ..] => {
                // <!NOTATION
                *self = Self::InQuoteSensitive(ElementParser::Outside);
                Some(9)
            }
            // <... - `markup` does not have enough data to determine markup
            // or markup is not known.
            // Undecided markup bytes will be written to `buf` to be available on
            // next iteration.
            _ => {
                // FIXME: to correctly report error position in DTD we need to provide
                // DTD events. For now our task just to skip (correct) DTD, so we postpone
                // error reporting and go with ending the unknown markup with `>`.
                if let Some(i) = memchr::memchr(b'>', markup) {
                    *self = Self::InsideOfInternalSubset;
                    Some(i + 1) // +1 to skip `>`
                } else {
                    None
                }
            }
        }
    }
}
