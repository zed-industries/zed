#!/usr/bin/env -S cargo -Z script
---cargo
[package]
edition = "2021"
[dependencies]
anyhow = "1"
reqwest = { version = "0.12", default-features = false, features = ["blocking", "rustls-tls-webpki-roots"] }
---

use std::{fs, path::Path};

fn main() -> anyhow::Result<()> {
    for (domain, output_path) in [
        ("my.1password.com", "1password_com_valid_1.crt"),
        ("agilebits.com", "agilebits_com_valid_1.crt"),
        ("lencr.org", "letsencrypt_org_valid_1.crt"),
    ] {
        query(domain, output_path)?;
    }
    Ok(())
}

fn query(domain: &str, path: &str) -> anyhow::Result<()> {
    let url = format!("https://{domain}");
    let response = reqwest::blocking::Client::builder()
        .tls_info(true)
        // avoids agilebits.com redirect, which will result in the wrong cert...
        // we want the cert of agilebits.com, not of 1password.com
        .redirect(reqwest::redirect::Policy::none())
        .build()?
        .get(url)
        .send()?;
    let Some(tls_info): Option<&reqwest::tls::TlsInfo> = response.extensions().get() else {
        anyhow::bail!("no TLS info found");
    };
    let Some(der) = tls_info.peer_certificate() else {
        anyhow::bail!("no TLS certificate found");
    };
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(path);
    eprintln!("writing DER of {domain} to {}", path.display());
    fs::write(path, der)?;
    Ok(())
}
