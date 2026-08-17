mod socket;

pub fn available() -> bool {
    tokio::runtime::Handle::try_current().is_ok()
}
