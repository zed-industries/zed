use crate::{Error, Stream};

/// Representation of the [`<IRI>`] type.
///
/// [`<IRI>`]: https://www.w3.org/TR/SVG11/types.html#DataTypeIRI
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct IRI<'a>(pub &'a str);

impl<'a> IRI<'a> {
    /// Parsers a `IRI` from a string.
    ///
    /// By the SVG spec, the ID must contain only [Name] characters,
    /// but since no one fallows this it will parse any characters.
    ///
    /// We can't use the `FromStr` trait because it requires
    /// an owned value as a return type.
    ///
    /// [Name]: https://www.w3.org/TR/xml/#NT-Name
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(text: &'a str) -> Result<Self, Error> {
        let mut s = Stream::from(text);
        let link = s.parse_iri()?;
        s.skip_spaces();
        if !s.at_end() {
            return Err(Error::UnexpectedData(s.calc_char_pos()));
        }

        Ok(Self(link))
    }
}

/// Representation of the [`<FuncIRI>`] type.
///
/// [`<FuncIRI>`]: https://www.w3.org/TR/SVG11/types.html#DataTypeFuncIRI
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct FuncIRI<'a>(pub &'a str);

impl<'a> FuncIRI<'a> {
    /// Parsers a `FuncIRI` from a string.
    ///
    /// By the SVG spec, the ID must contain only [Name] characters,
    /// but since no one fallows this it will parse any characters.
    ///
    /// We can't use the `FromStr` trait because it requires
    /// an owned value as a return type.
    ///
    /// [Name]: https://www.w3.org/TR/xml/#NT-Name
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(text: &'a str) -> Result<Self, Error> {
        let mut s = Stream::from(text);
        let link = s.parse_func_iri()?;
        s.skip_spaces();
        if !s.at_end() {
            return Err(Error::UnexpectedData(s.calc_char_pos()));
        }

        Ok(Self(link))
    }
}

impl<'a> Stream<'a> {
    pub fn parse_iri(&mut self) -> Result<&'a str, Error> {
        self.skip_spaces();
        self.consume_byte(b'#')?;
        let link = self.consume_bytes(|_, c| c != b' ');
        if link.is_empty() {
            return Err(Error::InvalidValue);
        }
        Ok(link)
    }

    pub fn parse_func_iri(&mut self) -> Result<&'a str, Error> {
        self.skip_spaces();
        self.consume_string(b"url(")?;
        self.skip_spaces();
        self.consume_byte(b'#')?;
        let link = self.consume_bytes(|_, c| c != b' ' && c != b')');
        if link.is_empty() {
            return Err(Error::InvalidValue);
        }
        self.skip_spaces();
        self.consume_byte(b')')?;
        Ok(link)
    }
}

#[rustfmt::skip]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_iri_1() {
        assert_eq!(IRI::from_str("#id").unwrap(), IRI("id"));
    }

    #[test]
    fn parse_iri_2() {
        assert_eq!(IRI::from_str("   #id   ").unwrap(), IRI("id"));
    }

    #[test]
    fn parse_iri_3() {
        // Trailing data is ok for the Stream, by not for IRI.
        assert_eq!(Stream::from("   #id   text").parse_iri().unwrap(), "id");
        assert_eq!(IRI::from_str("   #id   text").unwrap_err().to_string(),
                   "unexpected data at position 10");
    }

    #[test]
    fn parse_iri_4() {
        assert_eq!(IRI::from_str("#1").unwrap(), IRI("1"));
    }

    #[test]
    fn parse_err_iri_1() {
        assert_eq!(IRI::from_str("# id").unwrap_err().to_string(), "invalid value");
    }

    #[test]
    fn parse_func_iri_1() {
        assert_eq!(FuncIRI::from_str("url(#id)").unwrap(), FuncIRI("id"));
    }

    #[test]
    fn parse_func_iri_2() {
        assert_eq!(FuncIRI::from_str("url(#1)").unwrap(), FuncIRI("1"));
    }

    #[test]
    fn parse_func_iri_3() {
        assert_eq!(FuncIRI::from_str("    url(    #id    )   ").unwrap(), FuncIRI("id"));
    }

    #[test]
    fn parse_func_iri_4() {
        // Trailing data is ok for the Stream, by not for FuncIRI.
        assert_eq!(Stream::from("url(#id) qwe").parse_func_iri().unwrap(), "id");
        assert_eq!(FuncIRI::from_str("url(#id) qwe").unwrap_err().to_string(),
                   "unexpected data at position 10");
    }

    #[test]
    fn parse_err_func_iri_1() {
        assert_eq!(FuncIRI::from_str("url ( #1 )").unwrap_err().to_string(),
                   "expected 'url(' not 'url ' at position 1");
    }

    #[test]
    fn parse_err_func_iri_2() {
        assert_eq!(FuncIRI::from_str("url(#)").unwrap_err().to_string(), "invalid value");
    }

    #[test]
    fn parse_err_func_iri_3() {
        assert_eq!(FuncIRI::from_str("url(# id)").unwrap_err().to_string(),
                   "invalid value");
    }
}
