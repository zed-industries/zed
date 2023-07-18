use std::fmt::Write;

use anyhow::anyhow;
use serde::Deserialize;

#[derive(Clone, Debug, Eq, PartialEq, Default, Deserialize, Hash)]
pub struct Keystroke {
    pub ctrl: bool,
    pub alt: bool,
    pub shift: bool,
    pub cmd: bool,
    pub function: bool,
    pub key: String,
}

impl Keystroke {
    pub fn parse(source: &str) -> anyhow::Result<Self> {
        let mut ctrl = false;
        let mut alt = false;
        let mut shift = false;
        let mut cmd = false;
        let mut function = false;
        let mut key = None;

        let mut components = source.split('-').peekable();
        while let Some(component) = components.next() {
            match component {
                "ctrl" => ctrl = true,
                "alt" => alt = true,
                "shift" => shift = true,
                "cmd" => cmd = true,
                "fn" => function = true,
                _ => {
                    if let Some(component) = components.peek() {
                        if component.is_empty() && source.ends_with('-') {
                            key = Some(String::from("-"));
                            break;
                        } else {
                            return Err(anyhow!("Invalid keystroke `{}`", source));
                        }
                    } else {
                        key = Some(String::from(component));
                    }
                }
            }
        }

        let key = key.ok_or_else(|| anyhow!("Invalid keystroke `{}`", source))?;

        Ok(Keystroke {
            ctrl,
            alt,
            shift,
            cmd,
            function,
            key,
        })
    }

    pub fn modified(&self) -> bool {
        self.ctrl || self.alt || self.shift || self.cmd
    }
}

impl std::fmt::Display for Keystroke {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.ctrl {
            f.write_char('^')?;
        }
        if self.alt {
            f.write_char('⎇')?;
        }
        if self.cmd {
            f.write_char('⌘')?;
        }
        if self.shift {
            f.write_char('⇧')?;
        }
        let key = match self.key.as_str() {
            "backspace" => '⌫',
            "up" => '↑',
            "down" => '↓',
            "left" => '←',
            "right" => '→',
            "tab" => '⇥',
            "escape" => '⎋',
            key => {
                if key.len() == 1 {
                    key.chars().next().unwrap().to_ascii_uppercase()
                } else {
                    return f.write_str(key);
                }
            }
        };
        f.write_char(key)
    }
}
