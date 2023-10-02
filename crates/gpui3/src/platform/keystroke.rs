use anyhow::anyhow;
use serde::Deserialize;
use std::fmt::Write;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default, Deserialize, Hash)]
pub struct Modifiers {
    pub control: bool,
    pub alt: bool,
    pub shift: bool,
    pub command: bool,
    pub function: bool,
}

impl Modifiers {
    pub fn modified(&self) -> bool {
        self.control || self.alt || self.shift || self.command || self.function
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Hash)]
pub struct Keystroke {
    pub key: String,
    pub modifiers: Modifiers,
}

impl Keystroke {
    pub fn parse(source: &str) -> anyhow::Result<Self> {
        let mut control = false;
        let mut alt = false;
        let mut shift = false;
        let mut command = false;
        let mut function = false;
        let mut key = None;

        let mut components = source.split('-').peekable();
        while let Some(component) = components.next() {
            match component {
                "ctrl" => control = true,
                "alt" => alt = true,
                "shift" => shift = true,
                "cmd" => command = true,
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
            modifiers: Modifiers {
                control,
                alt,
                shift,
                command,
                function,
            },
            key,
        })
    }

    pub fn modified(&self) -> bool {
        self.modifiers.modified()
    }
}

impl std::fmt::Display for Keystroke {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Modifiers {
            control,
            alt,
            shift,
            command,
            function,
        } = self.modifiers;

        if control {
            f.write_char('^')?;
        }
        if alt {
            f.write_char('⎇')?;
        }
        if command {
            f.write_char('⌘')?;
        }
        if shift {
            f.write_char('⇧')?;
        }
        if function {
            f.write_char('𝙛')?;
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
