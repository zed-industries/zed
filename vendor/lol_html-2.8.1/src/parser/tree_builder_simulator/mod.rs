//! HTML parser has 6 different state machines for text parsing
//! purposes in different contexts. Switch between these state machines
//! usually performed by the tree construction stage depending on the
//! state of the stack of open elements (HTML is a context-sensitive grammar).
//!
//! Luckily, in the majority of cases this tree construction stage feedback
//! can be simulated without the stack of open elements and comlicated rules
//! required to maintain its state.
//!
//! This module implements such feedback simulation. However, there are few
//! cases where we can't unambiguously determine parsing context and prefer
//! to bail out from the tokenization in such a case
//! (see `AmbiguityGuard` for the details).
mod ambiguity_guard;

use self::ambiguity_guard::AmbiguityGuard;
use crate::base::eq_case_insensitive;
use crate::html::{LocalNameHash, Namespace, Tag, TextType};
use crate::parser::{TagLexeme, TagTokenOutline};
use TagTokenOutline::{EndTag, StartTag};

pub use self::ambiguity_guard::ParsingAmbiguityError;

const DEFAULT_NS_STACK_CAPACITY: usize = 256;

#[must_use]
pub(crate) enum TreeBuilderFeedback {
    SwitchTextType(TextType),
    SetAllowCdata(bool),
    #[allow(clippy::type_complexity)]
    RequestLexeme(Box<dyn FnMut(&mut TreeBuilderSimulator, &TagLexeme<'_>) -> Self + Send>),
    None,
}

impl From<TextType> for TreeBuilderFeedback {
    #[inline]
    fn from(text_type: TextType) -> Self {
        Self::SwitchTextType(text_type)
    }
}

#[inline]
fn request_lexeme(
    callback: impl FnMut(&mut TreeBuilderSimulator, &TagLexeme<'_>) -> TreeBuilderFeedback
    + 'static
    + Send,
) -> TreeBuilderFeedback {
    TreeBuilderFeedback::RequestLexeme(Box::new(callback))
}

macro_rules! expect_tag {
    ($lexeme:expr, $tag_pat:pat => $action:expr) => {
        match *$lexeme.token_outline() {
            $tag_pat => $action,
            _ => {
                debug_assert!(false, "Got unexpected tag type");
                return TreeBuilderFeedback::None;
            }
        }
    };
}

#[inline]
fn get_text_type_adjustment(tag_name: LocalNameHash) -> TreeBuilderFeedback {
    use TextType::*;

    if tag_is_one_of!(tag_name, [Textarea, Title]) {
        RCData.into()
    } else if tag_name == Tag::Plaintext {
        PlainText.into()
    } else if tag_name == Tag::Script {
        ScriptData.into()
    } else if tag_is_one_of!(tag_name, [Style, Iframe, Xmp, Noembed, Noframes, Noscript]) {
        RawText.into()
    } else {
        TreeBuilderFeedback::None
    }
}

#[inline]
fn causes_foreign_content_exit(tag_name: LocalNameHash) -> bool {
    tag_is_one_of!(
        tag_name,
        [
            B, Big, Blockquote, Body, Br, Center, Code, Dd, Div, Dl, Dt, Em, Embed, H1, H2, H3, H4,
            H5, H6, Head, Hr, I, Img, Li, Listing, Menu, Meta, Nobr, Ol, P, Pre, Ruby, S, Small,
            Span, Strong, Strike, Sub, Sup, Table, Tt, U, Ul, Var
        ]
    )
}

#[inline]
fn is_text_integration_point_in_math_ml(tag_name: LocalNameHash) -> bool {
    tag_is_one_of!(tag_name, [Mi, Mo, Mn, Ms, Mtext])
}

#[inline]
fn is_html_integration_point_in_svg(tag_name: LocalNameHash) -> bool {
    tag_is_one_of!(tag_name, [Desc, Title, ForeignObject])
}

// TODO limit ns stack
pub(crate) struct TreeBuilderSimulator {
    ns_stack: Vec<Namespace>,
    current_ns: Namespace,
    ambiguity_guard: AmbiguityGuard,
    strict: bool,
}

impl TreeBuilderSimulator {
    #[inline]
    #[must_use]
    pub fn new(strict: bool) -> Self {
        let mut simulator = Self {
            ns_stack: Vec::with_capacity(DEFAULT_NS_STACK_CAPACITY),
            current_ns: Namespace::Html,
            ambiguity_guard: AmbiguityGuard::default(),
            strict,
        };

        simulator.ns_stack.push(Namespace::Html);

        simulator
    }

    pub fn get_feedback_for_start_tag(
        &mut self,
        tag_name: LocalNameHash,
    ) -> Result<TreeBuilderFeedback, ParsingAmbiguityError> {
        if self.strict {
            self.ambiguity_guard.track_start_tag(tag_name)?;
        }

        Ok(if tag_name == Tag::Svg {
            self.enter_ns(Namespace::Svg)
        } else if tag_name == Tag::Math {
            self.enter_ns(Namespace::MathML)
        } else if self.current_ns != Namespace::Html {
            self.get_feedback_for_start_tag_in_foreign_content(tag_name)
        } else {
            get_text_type_adjustment(tag_name)
        })
    }

    pub fn get_feedback_for_end_tag(&mut self, tag_name: LocalNameHash) -> TreeBuilderFeedback {
        if self.strict {
            self.ambiguity_guard.track_end_tag(tag_name);
        }

        if self.current_ns == Namespace::Html {
            self.check_integration_point_exit(tag_name)
        } else if self.should_leave_ns(tag_name) {
            self.leave_ns()
        } else {
            TreeBuilderFeedback::None
        }
    }

    fn should_leave_ns(&self, tag_name: LocalNameHash) -> bool {
        if self.current_ns == Namespace::Svg && tag_name == Tag::Svg
            || self.current_ns == Namespace::MathML && tag_name == Tag::Math
        {
            return true;
        }

        if (self.current_ns == Namespace::Svg || self.current_ns == Namespace::MathML)
            && tag_is_one_of!(tag_name, [P, Br])
        {
            // 13.2.6.5
            return true;
        }
        false
    }

    #[inline]
    pub const fn current_ns(&self) -> Namespace {
        self.current_ns
    }

    #[inline]
    fn enter_ns(&mut self, ns: Namespace) -> TreeBuilderFeedback {
        self.ns_stack.push(ns);
        self.current_ns = ns;
        TreeBuilderFeedback::SetAllowCdata(ns != Namespace::Html)
    }

    #[inline]
    fn leave_ns(&mut self) -> TreeBuilderFeedback {
        self.ns_stack.pop();

        let Some(item) = self.ns_stack.last() else {
            debug_assert!(
                false,
                "Namespace stack should always have at least one item"
            );
            return TreeBuilderFeedback::None;
        };
        self.current_ns = *item;

        TreeBuilderFeedback::SetAllowCdata(self.current_ns != Namespace::Html)
    }

    fn is_integration_point_enter(&self, tag_name: LocalNameHash) -> bool {
        self.current_ns == Namespace::Svg && is_html_integration_point_in_svg(tag_name)
            || self.current_ns == Namespace::MathML
                && is_text_integration_point_in_math_ml(tag_name)
    }

    fn check_integration_point_exit(&mut self, tag_name: LocalNameHash) -> TreeBuilderFeedback {
        let ns_stack_len = self.ns_stack.len();

        if ns_stack_len < 2 {
            return TreeBuilderFeedback::None;
        }

        let prev_ns = self.ns_stack[ns_stack_len - 2];

        if prev_ns == Namespace::MathML && is_text_integration_point_in_math_ml(tag_name)
            || prev_ns == Namespace::Svg && is_html_integration_point_in_svg(tag_name)
        {
            self.leave_ns()
        } else if tag_name.is_empty() && prev_ns == Namespace::MathML {
            // NOTE: empty tag name hash - possibly <annotation-xml> case
            request_lexeme(|this, lexeme| {
                expect_tag!(lexeme, EndTag { name, .. } => {
                    if eq_case_insensitive(&lexeme.part(name), b"annotation-xml") {
                        this.leave_ns()
                    } else {
                        TreeBuilderFeedback::None
                    }
                })
            })
        } else {
            TreeBuilderFeedback::None
        }
    }

    fn get_feedback_for_start_tag_in_foreign_content(
        &mut self,
        tag_name: LocalNameHash,
    ) -> TreeBuilderFeedback {
        if causes_foreign_content_exit(tag_name) {
            return self.leave_ns();
        }

        if self.is_integration_point_enter(tag_name) {
            return request_lexeme(|this, lexeme| {
                expect_tag!(lexeme, StartTag { self_closing, .. } => {
                    if self_closing {
                        TreeBuilderFeedback::None
                    } else {
                        this.enter_ns(Namespace::Html)
                    }
                })
            });
        }

        if tag_name == Tag::Font {
            // NOTE: <font> tag special case requires attributes
            // to decide on foreign context exit
            return request_lexeme(|this, lexeme| {
                expect_tag!(lexeme, StartTag { ref attributes, .. } => {
                    for attr in attributes {
                        let name = lexeme.part(attr.name);

                        if eq_case_insensitive(&name, b"color")
                            || eq_case_insensitive(&name, b"size")
                            || eq_case_insensitive(&name, b"face")
                        {
                            return this.leave_ns();
                        }
                    }
                });

                TreeBuilderFeedback::None
            });
        }

        if tag_name.is_empty() && self.current_ns == Namespace::MathML {
            // NOTE: tag name hash is empty - we need integration point check
            // for the possible <annotation-xml> case
            return request_lexeme(|this, lexeme| {
                expect_tag!(lexeme, StartTag {
                    name,
                    ref attributes,
                    self_closing,
                    ..
                } => {
                    let name = lexeme.part(name);

                    if !self_closing && eq_case_insensitive(&name, b"annotation-xml") {
                        for attr in attributes {
                            let name = lexeme.part(attr.name);
                            let value = lexeme.part(attr.value);

                            if eq_case_insensitive(&name, b"encoding")
                                && (eq_case_insensitive(&value, b"text/html")
                                    || eq_case_insensitive(&value, b"application/xhtml+xml"))
                            {
                                return this.enter_ns(Namespace::Html);
                            }
                        }
                    }
                });

                TreeBuilderFeedback::None
            });
        }

        TreeBuilderFeedback::None
    }
}
