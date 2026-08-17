use super::AsciiByte;

pub(crate) struct PatternCtor<T>(pub(crate) T);

impl PatternCtor<u8> {
    pub(crate) const fn conv(self) -> Pattern {
        Pattern::AsciiByte(AsciiByte::new(self.0))
    }
}

impl PatternCtor<&'static str> {
    pub(crate) const fn conv(self) -> Pattern {
        if let [b @ 0..=127] = *self.0.as_bytes() {
            Pattern::AsciiByte(AsciiByte::new(b))
        } else {
            Pattern::Str(self.0)
        }
    }
}

impl PatternCtor<char> {
    pub(crate) const fn conv(self) -> Pattern {
        let code = self.0 as u32;
        if let c @ 0..=127 = code {
            Pattern::AsciiByte(AsciiByte::new(c as u8))
        } else {
            Pattern::Char(crate::char_encoding::char_to_display(self.0))
        }
    }
}

#[derive(Copy, Clone)]
pub(crate) enum Pattern {
    AsciiByte(AsciiByte),
    Str(&'static str),
    Char(crate::char_encoding::FmtChar),
}

pub(crate) enum PatternNorm<'a> {
    AsciiByte(AsciiByte),
    Str(&'a [u8]),
}

impl Pattern {
    pub(crate) const fn normalize(&self) -> PatternNorm<'_> {
        match self {
            Pattern::AsciiByte(ab) => PatternNorm::AsciiByte(*ab),
            Pattern::Str(str) => PatternNorm::Str(str.as_bytes()),
            Pattern::Char(char) => PatternNorm::Str(char.as_bytes()),
        }
    }
}
