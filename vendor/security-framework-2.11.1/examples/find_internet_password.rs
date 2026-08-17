#[cfg(target_os = "macos")]
use security_framework::os::macos::keychain::SecKeychain;
#[cfg(target_os = "macos")]
use security_framework::os::macos::passwords::{SecAuthenticationType, SecProtocolType};

fn main() {
    #[cfg(target_os = "macos")] {
    let hostname = "example.com";
    let username = "rusty";
    let res = SecKeychain::default().unwrap().find_internet_password(
        hostname,
        None,
        username,
        "",
        None,
        SecProtocolType::Any,
        SecAuthenticationType::Any,
    );
    match res {
        Ok((password, _)) => {
            println!(
                "Password for {}@{} is {} bytes long",
                username,
                hostname,
                password.len()
            );
        }
        Err(err) if err.code() == -128 => {
            eprintln!("Account was found in the Keychain, but user denied access");
        }
        Err(err) => {
            eprintln!("Password not found. Open Keychain Access.app and add internet password for '{username}' at 'https://{hostname}': {err:?}");
        }
    }
}}
