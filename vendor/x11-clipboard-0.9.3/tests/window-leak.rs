extern crate x11_clipboard;

use x11_clipboard::error::Error;
use x11_clipboard::Clipboard;

pub fn paste_to_clipboard(content: &str) -> Result<(), Error> {
    let clipboard = Clipboard::new()?;

    clipboard.store(
        clipboard.setter.atoms.primary,
        clipboard.setter.atoms.utf8_string,
        content,
    )?;

    clipboard.store(
        clipboard.setter.atoms.clipboard,
        clipboard.setter.atoms.utf8_string,
        content,
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work_but_does_not() -> Result<(), Error> {
        for i in 0..1000 {
            paste_to_clipboard(&format!("I have told you {} times!", i))?;
        }

        Ok(())
    }
}
