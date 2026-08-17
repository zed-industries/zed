extern crate x11_clipboard;

use std::time::{ Instant, Duration };
use x11_clipboard::Clipboard;


#[test]
fn it_work() {
    let data = format!("{:?}", Instant::now());
    let clipboard = Clipboard::new().unwrap();

    let atom_clipboard = clipboard.setter.atoms.clipboard;
    let atom_utf8string = clipboard.setter.atoms.utf8_string;
    let atom_property = clipboard.setter.atoms.property;

    clipboard.store(atom_clipboard, atom_utf8string, data.as_bytes()).unwrap();

    let output = clipboard.load(atom_clipboard, atom_utf8string, atom_property, None).unwrap();
    assert_eq!(output, data.as_bytes());

    let data = format!("{:?}", Instant::now());
    clipboard.store(atom_clipboard, atom_utf8string, data.as_bytes()).unwrap();

    let output = clipboard.load(atom_clipboard, atom_utf8string, atom_property, None).unwrap();
    assert_eq!(output, data.as_bytes());

    let output = clipboard.load(atom_clipboard, atom_utf8string, atom_property, None).unwrap();
    assert_eq!(output, data.as_bytes());

    let dur = Duration::from_secs(3);
    let output = clipboard.load(atom_clipboard, atom_utf8string, atom_property, dur).unwrap();
    assert_eq!(output, data.as_bytes());
}
