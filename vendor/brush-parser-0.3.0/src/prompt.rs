//! Parser for shell prompt syntax (e.g., `PS1`).

use crate::error;

/// A piece of a prompt string.
#[derive(Clone)]
#[cfg_attr(test, derive(PartialEq, Eq, serde::Serialize))]
pub enum PromptPiece {
    /// An ASCII character.
    AsciiCharacter(u32),
    /// A backslash character.
    Backslash,
    /// The bell character.
    BellCharacter,
    /// A carriage return character.
    CarriageReturn,
    /// The current command number.
    CurrentCommandNumber,
    /// The current history number.
    CurrentHistoryNumber,
    /// The name of the current user.
    CurrentUser,
    /// Path to the current working directory.
    CurrentWorkingDirectory {
        /// Whether or not to apply tilde-replacement before expanding.
        tilde_replaced: bool,
        /// Whether or not to only expand to the basename of the directory.
        basename: bool,
    },
    /// The current date, using the given format.
    Date(PromptDateFormat),
    /// The dollar or pound character.
    DollarOrPound,
    /// Special marker indicating the end of a non-printing sequence of characters.
    EndNonPrintingSequence,
    /// The escape character.
    EscapeCharacter,
    /// An escaped sequence not otherwise recognized.
    EscapedSequence(String),
    /// The hostname of the system.
    Hostname {
        /// Whether or not to include only up to the first dot of the name.
        only_up_to_first_dot: bool,
    },
    /// A literal string.
    Literal(String),
    /// A newline character.
    Newline,
    /// The number of actively managed jobs.
    NumberOfManagedJobs,
    /// The base name of the shell.
    ShellBaseName,
    /// The release of the shell.
    ShellRelease,
    /// The version of the shell.
    ShellVersion,
    /// Special marker indicating the start of a non-printing sequence of characters.
    StartNonPrintingSequence,
    /// The base name of the terminal device.
    TerminalDeviceBaseName,
    /// The current time, using the given format.
    Time(PromptTimeFormat),
}

/// Format for a date in a prompt.
#[derive(Clone)]
#[cfg_attr(test, derive(PartialEq, Eq, serde::Serialize))]
pub enum PromptDateFormat {
    /// A format including weekday, month, and date.
    WeekdayMonthDate,
    /// A customer string format.
    Custom(String),
}

/// Format for a time in a prompt.
#[derive(Clone)]
#[cfg_attr(test, derive(PartialEq, Eq, serde::Serialize))]
pub enum PromptTimeFormat {
    /// A twelve-hour time format with AM/PM.
    TwelveHourAM,
    /// A twelve-hour time format (HHMMSS).
    TwelveHourHHMMSS,
    /// A twenty-four-hour time format (HHMM).
    TwentyFourHourHHMM,
    /// A twenty-four-hour time format (HHMMSS).
    TwentyFourHourHHMMSS,
}

peg::parser! {
    grammar prompt_parser() for str {
        pub(crate) rule prompt() -> Vec<PromptPiece> =
            pieces:prompt_piece()*

        rule prompt_piece() -> PromptPiece =
            special_sequence() /
            literal_sequence()

        //
        // Reference: https://www.gnu.org/software/bash/manual/bash.html#Controlling-the-Prompt
        //
        rule special_sequence() -> PromptPiece =
            "\\a" { PromptPiece::BellCharacter } /
            "\\A" { PromptPiece::Time(PromptTimeFormat::TwentyFourHourHHMM) } /
            "\\d" { PromptPiece::Date(PromptDateFormat::WeekdayMonthDate) } /
            "\\D{" f:date_format() "}" { PromptPiece::Date(PromptDateFormat::Custom(f)) } /
            "\\e" { PromptPiece::EscapeCharacter } /
            "\\h" { PromptPiece::Hostname { only_up_to_first_dot: true } } /
            "\\H" { PromptPiece::Hostname { only_up_to_first_dot: false } } /
            "\\j" { PromptPiece::NumberOfManagedJobs } /
            "\\l" { PromptPiece::TerminalDeviceBaseName } /
            "\\n" { PromptPiece::Newline } /
            "\\r" { PromptPiece::CarriageReturn } /
            "\\s" { PromptPiece::ShellBaseName } /
            "\\t" { PromptPiece::Time(PromptTimeFormat::TwentyFourHourHHMMSS ) } /
            "\\T" { PromptPiece::Time(PromptTimeFormat::TwelveHourHHMMSS ) } /
            "\\@" { PromptPiece::Time(PromptTimeFormat::TwelveHourAM ) } /
            "\\u" { PromptPiece::CurrentUser } /
            "\\v" { PromptPiece::ShellVersion } /
            "\\V" { PromptPiece::ShellRelease } /
            "\\w" { PromptPiece::CurrentWorkingDirectory { tilde_replaced: true, basename: false, } } /
            "\\W" { PromptPiece::CurrentWorkingDirectory { tilde_replaced: true, basename: true, } } /
            "\\!" { PromptPiece::CurrentHistoryNumber } /
            "\\#" { PromptPiece::CurrentCommandNumber } /
            "\\$" { PromptPiece::DollarOrPound } /
            "\\" n:octal_number() { PromptPiece::AsciiCharacter(n) } /
            "\\\\" { PromptPiece::Backslash } /
            "\\[" { PromptPiece::StartNonPrintingSequence } /
            "\\]" { PromptPiece::EndNonPrintingSequence } /
            s:$("\\" [_]) { PromptPiece::EscapedSequence(s.to_owned()) }

        rule literal_sequence() -> PromptPiece =
            s:$((!special_sequence() [c])+) { PromptPiece::Literal(s.to_owned()) }

        rule date_format() -> String =
            s:$([c if c != '}']*) { s.to_owned() }

        rule octal_number() -> u32 =
            s:$(['0'..='7']*<1,3>) {? u32::from_str_radix(s, 8).or(Err("invalid octal number")) }
    }
}

/// Parses a shell prompt string.
///
/// # Arguments
///
/// * `s` - The prompt string to parse.
pub fn parse(s: &str) -> Result<Vec<PromptPiece>, error::WordParseError> {
    let result = prompt_parser::prompt(s).map_err(|e| error::WordParseError::Prompt(e.into()))?;
    Ok(result)
}
