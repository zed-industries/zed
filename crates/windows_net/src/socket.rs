use windows::Win32::Networking::WinSock::{
    WSASocketW, AF_UNIX, SOCKET, SOCK_STREAM, WSA_FLAG_OVERLAPPED,
};

pub(crate) struct RawSocket(SOCKET);

impl RawSocket {
    pub(crate) fn new() -> std::io::Result<Self> {
        unsafe {
            let raw = WSASocketW(AF_UNIX as _, SOCK_STREAM.0, 0, None, 0, WSA_FLAG_OVERLAPPED)?;
            Ok(Self(raw))
        }
    }

    pub(crate) fn as_raw(&self) -> &SOCKET {
        &self.0
    }
}
