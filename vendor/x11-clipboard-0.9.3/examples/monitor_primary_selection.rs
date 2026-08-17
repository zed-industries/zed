extern crate x11_clipboard;

use x11_clipboard::Clipboard;


fn main() {
    let clipboard = Clipboard::new().unwrap();
    let mut last = String::new();

    println!("Waiting for selection...");

    loop {
        if let Ok(curr) = clipboard.load_wait(
            clipboard.getter.atoms.primary,
            clipboard.getter.atoms.utf8_string,
            clipboard.getter.atoms.property
        ) {
            let curr = String::from_utf8_lossy(&curr);
            let curr = curr
                .trim_matches('\u{0}')
                .trim();
            if !curr.is_empty() && last != curr {
                last = curr.to_owned();
                println!("Contents of primary selection: {}", last);
                println!("Waiting for selection...");
            }
        }
    }
}
