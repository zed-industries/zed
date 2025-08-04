use std::fs;
use std::sync::OnceLock;

use rustls::ClientConfig;
use rustls_platform_verifier::{ConfigVerifierExt, Verifier};

static TLS_CONFIG: OnceLock<rustls::ClientConfig> = OnceLock::new();

pub fn tls_config() -> ClientConfig {
    TLS_CONFIG
        .get_or_init(|| {
            // rustls uses the `aws_lc_rs` provider by default
            // This only errors if the default provider has already
            // been installed. We can ignore this `Result`.
            rustls::crypto::aws_lc_rs::default_provider()
                .install_default()
                .ok();

            // Check for custom certificate environment variables
            let custom_certs = load_custom_certificates();
            
            if custom_certs.is_empty() {
                // No custom certificates, use platform verifier
                ClientConfig::with_platform_verifier()
            } else {
                // Create config with custom verifier that includes both platform and custom certs
                let verifier = Verifier::new_with_extra_roots(custom_certs)
                    .expect("Failed to create verifier with extra roots");
                
                ClientConfig::builder()
                    .dangerous()
                    .with_custom_certificate_verifier(std::sync::Arc::new(verifier))
                    .with_no_client_auth()
            }
        })
        .clone()
}

fn load_custom_certificates() -> Vec<rustls::pki_types::CertificateDer<'static>> {
    let mut certs = Vec::new();
    
    // Check SSL_CERT_FILE environment variable
    if let Ok(cert_file) = std::env::var("SSL_CERT_FILE") {
        if let Ok(file_certs) = load_certs_from_file(&cert_file) {
            certs.extend(file_certs);
        } else {
            log::warn!("Failed to load certificates from SSL_CERT_FILE: {}", cert_file);
        }
    }
    
    // Check SSL_CERT_DIR environment variable
    if let Ok(cert_dir) = std::env::var("SSL_CERT_DIR") {
        if let Ok(dir_certs) = load_certs_from_directory(&cert_dir) {
            certs.extend(dir_certs);
        } else {
            log::warn!("Failed to load certificates from SSL_CERT_DIR: {}", cert_dir);
        }
    }
    
    certs
}

fn load_certs_from_file(path: &str) -> Result<Vec<rustls::pki_types::CertificateDer<'static>>, Box<dyn std::error::Error>> {
    let cert_file = fs::read(path)?;
    let certs = rustls_pemfile::certs(&mut &cert_file[..])
        .collect::<Result<Vec<_>, _>>()?;
    Ok(certs)
}

fn load_certs_from_directory(dir: &str) -> Result<Vec<rustls::pki_types::CertificateDer<'static>>, Box<dyn std::error::Error>> {
    let mut certs = Vec::new();
    let entries = fs::read_dir(dir)?;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        // Only process files with common certificate extensions
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "pem" || ext == "crt" || ext == "cert" || ext == "cer" {
                    if let Ok(file_certs) = load_certs_from_file(&path.to_string_lossy()) {
                        certs.extend(file_certs);
                    }
                }
            }
        }
    }
    
    Ok(certs)
}
