mod to_token;

pub(crate) use self::to_token::{ToToken, ToTokenResult};

use bitflags::bitflags;
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct TokenCaptureFlags: u8 {
        const TEXT = 0b0000_0001;
        const COMMENTS = 0b0000_0010;
        const NEXT_START_TAG = 0b0000_0100;
        const NEXT_END_TAG = 0b0000_1000;
        const DOCTYPES = 0b0001_0000;
    }
}
