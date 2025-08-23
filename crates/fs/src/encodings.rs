use encoding::Encoding;

pub enum CharacterEncoding {
    Utf8,
    Iso8859_1,
    Cp865,
}

pub fn to_utf8<'a>(input: Vec<u8>, encoding: &'a impl encoding::Encoding) -> String {
    match encoding.decode(&input, encoding::DecoderTrap::Strict) {
        Ok(v) => return v,
        Err(_) => panic!(),
    }
}

pub fn to<'a>(input: String, target: &'a impl encoding::Encoding) -> Vec<u8> {
    match target.encode(&input, encoding::EncoderTrap::Strict) {
        Ok(v) => v,
        Err(_) => panic!(),
    }
}
