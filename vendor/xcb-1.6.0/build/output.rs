use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

#[derive(Debug)]
pub enum Output {
    Fmt(Child),
    Direct(File),
}

impl Output {
    pub fn new(rustfmt: &Option<PathBuf>, out: &Path) -> io::Result<Output> {
        fs::create_dir_all(out.parent().unwrap())?;
        let output = File::create(&out)?;

        Ok(match rustfmt {
            Some(rustfmt) => Output::Fmt(
                Command::new(rustfmt)
                    .arg("--emit=stdout")
                    .arg("--edition=2018")
                    .stdin(Stdio::piped())
                    .stdout(output)
                    .spawn()?,
            ),

            None => Output::Direct(output),
        })
    }
}

impl Write for Output {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Output::Fmt(child) => child.stdin.as_ref().unwrap().write(buf),
            Output::Direct(file) => file.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            Output::Fmt(child) => child.stdin.as_ref().unwrap().flush(),
            Output::Direct(file) => file.flush(),
        }
    }
}
