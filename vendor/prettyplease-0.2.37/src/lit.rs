use crate::algorithm::Printer;
use proc_macro2::Literal;
use syn::{Lit, LitBool, LitByte, LitByteStr, LitCStr, LitChar, LitFloat, LitInt, LitStr};

impl Printer {
    pub fn lit(&mut self, lit: &Lit) {
        match lit {
            #![cfg_attr(all(test, exhaustive), deny(non_exhaustive_omitted_patterns))]
            Lit::Str(lit) => self.lit_str(lit),
            Lit::ByteStr(lit) => self.lit_byte_str(lit),
            Lit::CStr(lit) => self.lit_c_str(lit),
            Lit::Byte(lit) => self.lit_byte(lit),
            Lit::Char(lit) => self.lit_char(lit),
            Lit::Int(lit) => self.lit_int(lit),
            Lit::Float(lit) => self.lit_float(lit),
            Lit::Bool(lit) => self.lit_bool(lit),
            Lit::Verbatim(lit) => self.lit_verbatim(lit),
            _ => unimplemented!("unknown Lit"),
        }
    }

    pub fn lit_str(&mut self, lit: &LitStr) {
        self.word(lit.token().to_string());
    }

    fn lit_byte_str(&mut self, lit: &LitByteStr) {
        self.word(lit.token().to_string());
    }

    fn lit_c_str(&mut self, lit: &LitCStr) {
        self.word(lit.token().to_string());
    }

    fn lit_byte(&mut self, lit: &LitByte) {
        self.word(lit.token().to_string());
    }

    fn lit_char(&mut self, lit: &LitChar) {
        self.word(lit.token().to_string());
    }

    fn lit_int(&mut self, lit: &LitInt) {
        self.word(lit.token().to_string());
    }

    fn lit_float(&mut self, lit: &LitFloat) {
        self.word(lit.token().to_string());
    }

    fn lit_bool(&mut self, lit: &LitBool) {
        self.word(if lit.value { "true" } else { "false" });
    }

    fn lit_verbatim(&mut self, token: &Literal) {
        self.word(token.to_string());
    }
}
