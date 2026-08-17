//! This test checks whether or not we can read from a reader when
//! stdin is not a terminal.

use std::io::Cursor;

#[allow(deprecated)]
use rpassword::read_password_from_bufread;

#[cfg(all(target_family = "unix", not(target_family = "wasm")))]
fn close_stdin() {
    unsafe {
        libc::close(libc::STDIN_FILENO);
    }
}

#[cfg(target_family = "windows")]
fn close_stdin() {
    use windows_sys::Win32::Foundation::CloseHandle;
    use windows_sys::Win32::System::Console::{GetStdHandle, STD_INPUT_HANDLE};

    unsafe {
        CloseHandle(GetStdHandle(STD_INPUT_HANDLE));
    }
}

#[cfg(target_family = "wasm")]
fn close_stdin() {
    unimplemented!()
}

fn mock_input_crlf() -> Cursor<&'static [u8]> {
    Cursor::new(&b"A mocked response.\r\nAnother mocked response.\r\n"[..])
}

fn mock_input_lf() -> Cursor<&'static [u8]> {
    Cursor::new(&b"A mocked response.\nAnother mocked response.\n"[..])
}

#[test]
#[allow(deprecated)]
fn can_read_from_redirected_input_many_times() {
    close_stdin();

    let mut reader_crlf = mock_input_crlf();

    let response = crate::read_password_from_bufread(&mut reader_crlf).unwrap();
    assert_eq!(response, "A mocked response.");
    let response = crate::read_password_from_bufread(&mut reader_crlf).unwrap();
    assert_eq!(response, "Another mocked response.");

    let mut reader_lf = mock_input_lf();
    let response = crate::read_password_from_bufread(&mut reader_lf).unwrap();
    assert_eq!(response, "A mocked response.");
    let response = crate::read_password_from_bufread(&mut reader_lf).unwrap();
    assert_eq!(response, "Another mocked response.");
}

#[test]
#[allow(deprecated)]
fn can_read_from_input_ctrl_u() {
    close_stdin();

    let mut reader_ctrl_u = Cursor::new(&b"A mocked response.Another mocked response.\n"[..]);
    let response = crate::read_password_from_bufread(&mut reader_ctrl_u).unwrap();
    assert_eq!(response, "Another mocked response.");

    let mut reader_ctrl_u_at_end = Cursor::new(&b"A mocked response.\n"[..]);
    let response = crate::read_password_from_bufread(&mut reader_ctrl_u_at_end).unwrap();
    assert_eq!(response, "");
}
