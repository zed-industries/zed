use crate::RawPasswordInput;
use crate::config::{Config, InputTarget};
use rtoolbox::fix_line_issues::fix_line_issues;
use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Read};

pub(crate) const DEFAULT_INPUT_PATH: &str = "/dev/stdin";
pub(crate) const DEFAULT_OUTPUT_PATH: &str = "/dev/stdout";

pub(crate) struct RawModeInput {
    config: Config,
}

impl RawPasswordInput for RawModeInput {
    fn new(config: Config) -> io::Result<impl RawPasswordInput> {
        Ok(RawModeInput { config })
    }

    fn needs_terminal_configuration(&self) -> bool {
        false
    }

    fn apply_terminal_configuration(&mut self) -> io::Result<()> {
        Ok(())
    }

    #[allow(unused)]
    fn read_char(&mut self) -> std::io::Result<char> {
        unimplemented!()
    }

    fn read_password(
        &mut self,
        _password_feedback: crate::PasswordFeedback,
    ) -> std::io::Result<String> {
        let input: Box<dyn Read> = match &mut self.config.input {
            InputTarget::FilePath(path) => Box::new(OpenOptions::new().read(true).open(path)?),
            InputTarget::Reader(reader) => Box::new(reader),
        };
        let mut reader = BufReader::new(input);
        let mut line = String::new();
        reader.read_line(&mut line)?;
        fix_line_issues(line)
    }

    fn write_output(&mut self, _output: &str) -> std::io::Result<()> {
        Ok(())
    }

    fn send_signal_sigint(&mut self) -> io::Result<()> {
        // Not sure what to do with signals on WASM, so just ignore it for now
        Ok(())
    }
}
