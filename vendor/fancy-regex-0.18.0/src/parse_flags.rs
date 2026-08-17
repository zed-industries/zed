/// Flag bit for case insensitive matching
pub const FLAG_CASEI: u32 = 1;
/// Flag bit for multi-line mode
pub const FLAG_MULTI: u32 = 1 << 1;
/// Flag bit for dot matches newline
pub const FLAG_DOTNL: u32 = 1 << 2;
/// Flag bit for swapping greed
pub const FLAG_SWAP_GREED: u32 = 1 << 3;
/// Flag bit for ignoring whitespace  
pub const FLAG_IGNORE_SPACE: u32 = 1 << 4;
/// Flag bit for unicode mode
pub const FLAG_UNICODE: u32 = 1 << 5;
/// Flag bit for parsing in Oniguruma compatibility mode
pub const FLAG_ONIGURUMA_MODE: u32 = 1 << 6;
/// Flag bit for CRLF mode (treat `\r\n` as a single line ending)
pub const FLAG_CRLF: u32 = 1 << 7;
/// Flag bit for treating numbered groups as non-capturing when named groups exist
pub const FLAG_IGNORE_NUMBERED_GROUPS_WHEN_NAMED_GROUPS_EXIST: u32 = 1 << 8;
