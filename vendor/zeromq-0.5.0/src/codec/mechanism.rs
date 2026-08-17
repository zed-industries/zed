use super::error::CodecError;

use std::convert::TryFrom;
use std::fmt::Display;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, Default)]
pub enum ZmqMechanism {
    #[default]
    NULL,
    PLAIN,
    CURVE,
}

impl ZmqMechanism {
    pub const fn as_str(&self) -> &'static str {
        match self {
            ZmqMechanism::NULL => "NULL",
            ZmqMechanism::PLAIN => "PLAIN",
            ZmqMechanism::CURVE => "CURVE",
        }
    }
}

impl Display for ZmqMechanism {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl TryFrom<&[u8]> for ZmqMechanism {
    type Error = CodecError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mech = value.split(|x| *x == 0x0).next().unwrap_or_default();
        // mechanism-char = "A"-"Z" | DIGIT
        //                  | "-" | "_" | "." | "+" | %x0
        // according to https://rfc.zeromq.org/spec:23/ZMTP/
        match mech {
            b"NULL" => Ok(ZmqMechanism::NULL),
            b"PLAIN" => Ok(ZmqMechanism::PLAIN),
            b"CURVE" => Ok(ZmqMechanism::CURVE),
            _ => Err(CodecError::Mechanism("Failed to parse ZmqMechanism")),
        }
    }
}
