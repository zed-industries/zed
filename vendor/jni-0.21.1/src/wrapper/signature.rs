use std::{fmt, str::FromStr};

use combine::{
    between, many, many1, parser, satisfy, token, ParseError, Parser, StdParseResult, Stream,
};

use crate::errors::*;

/// A primitive java type. These are the things that can be represented without
/// an object.
#[allow(missing_docs)]
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum Primitive {
    Boolean, // Z
    Byte,    // B
    Char,    // C
    Double,  // D
    Float,   // F
    Int,     // I
    Long,    // J
    Short,   // S
    Void,    // V
}

impl fmt::Display for Primitive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Primitive::Boolean => write!(f, "Z"),
            Primitive::Byte => write!(f, "B"),
            Primitive::Char => write!(f, "C"),
            Primitive::Double => write!(f, "D"),
            Primitive::Float => write!(f, "F"),
            Primitive::Int => write!(f, "I"),
            Primitive::Long => write!(f, "J"),
            Primitive::Short => write!(f, "S"),
            Primitive::Void => write!(f, "V"),
        }
    }
}

/// Enum representing any java type in addition to method signatures.
#[allow(missing_docs)]
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum JavaType {
    Primitive(Primitive),
    Object(String),
    Array(Box<JavaType>),
    Method(Box<TypeSignature>),
}

impl FromStr for JavaType {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        parser(parse_type)
            .parse(s)
            .map(|res| res.0)
            .map_err(|e| Error::ParseFailed(e, s.to_owned()))
    }
}

impl fmt::Display for JavaType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            JavaType::Primitive(ref ty) => ty.fmt(f),
            JavaType::Object(ref name) => write!(f, "L{name};"),
            JavaType::Array(ref ty) => write!(f, "[{ty}"),
            JavaType::Method(ref m) => m.fmt(f),
        }
    }
}

/// Enum representing any java type that may be used as a return value
///
/// This type intentionally avoids capturing any heap allocated types (to avoid
/// allocations while making JNI method calls) and so it doesn't fully qualify
/// the object or array types with a String like `JavaType::Object` does.
#[allow(missing_docs)]
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum ReturnType {
    Primitive(Primitive),
    Object,
    Array,
}

impl FromStr for ReturnType {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        parser(parse_return)
            .parse(s)
            .map(|res| res.0)
            .map_err(|e| Error::ParseFailed(e, s.to_owned()))
    }
}

impl fmt::Display for ReturnType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReturnType::Primitive(ref ty) => ty.fmt(f),
            ReturnType::Object => write!(f, "L;"),
            ReturnType::Array => write!(f, "["),
        }
    }
}

/// A method type signature. This is the structure representation of something
/// like `(Ljava/lang/String;)Z`. Used by the `call_(object|static)_method`
/// functions on jnienv to ensure safety.
#[allow(missing_docs)]
#[derive(Eq, PartialEq, Debug, Clone)]
pub struct TypeSignature {
    pub args: Vec<JavaType>,
    pub ret: ReturnType,
}

impl TypeSignature {
    /// Parse a signature string into a TypeSignature enum.
    // Clippy suggests implementing `FromStr` or renaming it which is not possible in our case.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str<S: AsRef<str>>(s: S) -> Result<TypeSignature> {
        Ok(match parser(parse_sig).parse(s.as_ref()).map(|res| res.0) {
            Ok(JavaType::Method(sig)) => *sig,
            Err(e) => return Err(Error::ParseFailed(e, s.as_ref().to_owned())),
            _ => unreachable!(),
        })
    }
}

impl fmt::Display for TypeSignature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(")?;
        for a in &self.args {
            write!(f, "{a}")?;
        }
        write!(f, ")")?;
        write!(f, "{}", self.ret)?;
        Ok(())
    }
}

fn parse_primitive<S: Stream<Token = char>>(input: &mut S) -> StdParseResult<Primitive, S>
where
    S::Error: ParseError<char, S::Range, S::Position>,
{
    let boolean = token('Z').map(|_| Primitive::Boolean);
    let byte = token('B').map(|_| Primitive::Byte);
    let char_type = token('C').map(|_| Primitive::Char);
    let double = token('D').map(|_| Primitive::Double);
    let float = token('F').map(|_| Primitive::Float);
    let int = token('I').map(|_| Primitive::Int);
    let long = token('J').map(|_| Primitive::Long);
    let short = token('S').map(|_| Primitive::Short);
    let void = token('V').map(|_| Primitive::Void);

    (boolean
        .or(byte)
        .or(char_type)
        .or(double)
        .or(float)
        .or(int)
        .or(long)
        .or(short)
        .or(void))
    .parse_stream(input)
    .into()
}

fn parse_array<S: Stream<Token = char>>(input: &mut S) -> StdParseResult<JavaType, S>
where
    S::Error: ParseError<char, S::Range, S::Position>,
{
    let marker = token('[');
    (marker, parser(parse_type))
        .map(|(_, ty)| JavaType::Array(Box::new(ty)))
        .parse_stream(input)
        .into()
}

fn parse_object<S: Stream<Token = char>>(input: &mut S) -> StdParseResult<JavaType, S>
where
    S::Error: ParseError<char, S::Range, S::Position>,
{
    let marker = token('L');
    let end = token(';');
    let obj = between(marker, end, many1(satisfy(|c| c != ';')));

    obj.map(JavaType::Object).parse_stream(input).into()
}

fn parse_type<S: Stream<Token = char>>(input: &mut S) -> StdParseResult<JavaType, S>
where
    S::Error: ParseError<char, S::Range, S::Position>,
{
    parser(parse_primitive)
        .map(JavaType::Primitive)
        .or(parser(parse_array))
        .or(parser(parse_object))
        .or(parser(parse_sig))
        .parse_stream(input)
        .into()
}

fn parse_return<S: Stream<Token = char>>(input: &mut S) -> StdParseResult<ReturnType, S>
where
    S::Error: ParseError<char, S::Range, S::Position>,
{
    parser(parse_primitive)
        .map(ReturnType::Primitive)
        .or(parser(parse_array).map(|_| ReturnType::Array))
        .or(parser(parse_object).map(|_| ReturnType::Object))
        .parse_stream(input)
        .into()
}

fn parse_args<S: Stream<Token = char>>(input: &mut S) -> StdParseResult<Vec<JavaType>, S>
where
    S::Error: ParseError<char, S::Range, S::Position>,
{
    between(token('('), token(')'), many(parser(parse_type)))
        .parse_stream(input)
        .into()
}

fn parse_sig<S: Stream<Token = char>>(input: &mut S) -> StdParseResult<JavaType, S>
where
    S::Error: ParseError<char, S::Range, S::Position>,
{
    (parser(parse_args), parser(parse_return))
        .map(|(a, r)| TypeSignature { args: a, ret: r })
        .map(|sig| JavaType::Method(Box::new(sig)))
        .parse_stream(input)
        .into()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser() {
        let inputs = [
            "(Ljava/lang/String;I)V",
            "[Lherp;",
            // fails because the return type does not contain the class name: "(IBVZ)L;"
            // "(IBVZ)Ljava/lang/String;",
        ];

        for each in inputs.iter() {
            let res = JavaType::from_str(each).unwrap();
            println!("{res:#?}");
            let s = format!("{res}");
            assert_eq!(s, *each);
            let res2 = JavaType::from_str(each).unwrap();
            println!("{res2:#?}");
            assert_eq!(res2, res);
        }
    }

    #[test]
    fn test_parser_invalid_signature() {
        let signature = "()Ljava/lang/List"; // no semicolon
        let res = JavaType::from_str(signature);

        match res {
            Ok(any) => {
                panic!("Unexpected result: {}", any);
            }
            Err(err) => {
                assert!(err.to_string().contains("input: ()Ljava/lang/List"));
            }
        }
    }
}
