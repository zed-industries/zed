use crate::common::{is_name_char, is_name_start_char, is_whitespace_char};
use crate::reader::error::SyntaxError;

use crate::reader::events::XmlEvent;
use crate::reader::lexer::Token;

use super::{DeclarationSubstate, Encountered, ProcessingInstructionSubstate, PullParser, Result, State};

impl PullParser {
    pub fn inside_processing_instruction(&mut self, t: Token, s: ProcessingInstructionSubstate) -> Option<Result> {
        match s {
            ProcessingInstructionSubstate::PIInsideName => match t {
                Token::Character(c) if self.buf.is_empty() && is_name_start_char(c) ||
                                 self.buf_has_data() && is_name_char(c) => {
                    if self.buf.len() > self.config.max_name_length {
                        return Some(self.error(SyntaxError::ExceededConfiguredLimit));
                    }
                    self.buf.push(c);
                    None
                },

                Token::ProcessingInstructionEnd => {
                    // self.buf contains PI name
                    let name = self.take_buf();

                    // Don't need to check for declaration because it has mandatory attributes
                    // but there is none
                    match &*name {
                        // Name is empty, it is an error
                        "" => Some(self.error(SyntaxError::ProcessingInstructionWithoutName)),

                        // Found <?xml-like PI not at the beginning of a document,
                        // it is an error - see section 2.6 of XML 1.1 spec
                        n if "xml".eq_ignore_ascii_case(n) =>
                            Some(self.error(SyntaxError::InvalidXmlProcessingInstruction(name.into()))),

                        // All is ok, emitting event
                        _ => {
                            debug_assert!(self.next_event.is_none(), "{:?}", self.next_event);
                            // can't have a PI before `<?xml`
                            let event1 = self.set_encountered(Encountered::Declaration);
                            let event2 = Some(Ok(XmlEvent::ProcessingInstruction {
                                name,
                                data: None
                            }));
                            // emitting two events at once is cumbersome
                            let event1 = if event1.is_some() {
                                self.next_event = event2;
                                event1
                            } else {
                                event2
                            };
                            self.into_state(State::OutsideTag, event1)
                        },
                    }
                },

                Token::Character(c) if is_whitespace_char(c) => {
                    // self.buf contains PI name
                    let name = self.take_buf();

                    match &*name {
                        // We have not ever encountered an element and have not parsed XML declaration
                        "xml" if self.encountered == Encountered::None =>
                            self.into_state_continue(State::InsideDeclaration(DeclarationSubstate::BeforeVersion)),

                        // Found <?xml-like PI after the beginning of a document,
                        // it is an error - see section 2.6 of XML 1.1 spec
                        n if "xml".eq_ignore_ascii_case(n) =>
                            Some(self.error(SyntaxError::InvalidXmlProcessingInstruction(name.into()))),

                        // All is ok, starting parsing PI data
                        _ => {
                            self.data.name = name;
                            // can't have a PI before `<?xml`
                            let next_event = self.set_encountered(Encountered::Declaration);
                            self.into_state(State::InsideProcessingInstruction(ProcessingInstructionSubstate::PIInsideData), next_event)
                        },
                    }
                },

                _ => {
                    let buf = self.take_buf();
                    Some(self.error(SyntaxError::UnexpectedProcessingInstruction(buf.into(), t)))
                },
            },

            ProcessingInstructionSubstate::PIInsideData => match t {
                Token::ProcessingInstructionEnd => {
                    let name = self.data.take_name();
                    let data = self.take_buf();
                    self.into_state_emit(
                        State::OutsideTag,
                        Ok(XmlEvent::ProcessingInstruction { name, data: Some(data) }),
                    )
                },

                Token::Character(c) if !self.is_valid_xml_char(c) => {
                    Some(self.error(SyntaxError::InvalidCharacterEntity(c as u32)))
                },

                // Any other token should be treated as plain characters
                _ => {
                    if self.buf.len() > self.config.max_data_length {
                        return Some(self.error(SyntaxError::ExceededConfiguredLimit));
                    }
                    t.push_to_string(&mut self.buf);
                    None
                },
            },
        }
    }
}
