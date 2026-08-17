extern crate x11_clipboard;

use x11_clipboard::Clipboard;

fn main() {
    let clipboard = Clipboard::new().unwrap();

    loop {
        let val = clipboard
            .load_wait(
                clipboard.setter.atoms.clipboard,
                clipboard.setter.atoms.string,
                clipboard.setter.atoms.property,
            )
            .unwrap();

        let val = String::from_utf8(val).unwrap();

        println!("{}", val);
    }
}
