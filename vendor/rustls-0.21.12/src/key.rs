use std::fmt;

use crate::Error;

/// This type contains a private key by value.
///
/// The private key must be DER-encoded ASN.1 in either
/// PKCS#8, PKCS#1, or Sec1 format.
///
/// A common format for storing private keys is
/// [PEM](https://en.wikipedia.org/wiki/Privacy-Enhanced_Mail).
/// PEM private keys are commonly stored in files with a `.pem` or `.key` suffix, and look like this:
///
/// ```txt
/// -----BEGIN PRIVATE KEY-----
/// <base64-encoded private key content>
/// -----END PRIVATE KEY-----
/// ```
///
/// The [`rustls-pemfile`](https://docs.rs/rustls-pemfile/latest/rustls_pemfile/) crate can be used
/// to parse PEM files. The [`rcgen`](https://docs.rs/rcgen/latest/rcgen/) can be used to generate
/// certificates and private keys.
///
/// ## Examples
///
/// Creating a `PrivateKey` from a PEM file containing a PKCS8-encoded private key using the `rustls_pemfile` crate:
///
/// ```rust
/// use std::fs::File;
/// use std::io::BufReader;
/// use rustls::PrivateKey;
///
/// fn load_private_key_from_file(path: &str) -> Result<PrivateKey, Box<dyn std::error::Error>> {
///     let file = File::open(&path)?;
///     let mut reader = BufReader::new(file);
///     let mut keys = rustls_pemfile::pkcs8_private_keys(&mut reader)?;
///
///     match keys.len() {
///         0 => Err(format!("No PKCS8-encoded private key found in {path}").into()),
///         1 => Ok(PrivateKey(keys.remove(0))),
///         _ => Err(format!("More than one PKCS8-encoded private key found in {path}").into()),
///     }
/// }
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PrivateKey(pub Vec<u8>);

/// This type contains a single certificate by value.
///
/// The certificate must be in DER-encoded X.509 format.
///
/// A common format for storing certificates is
/// [PEM](https://en.wikipedia.org/wiki/Privacy-Enhanced_Mail).
/// PEM certificates are commonly stored in files with a `.pem`, `.cer` or `.crt` suffix, and look
/// like this:
///
/// ```txt
/// -----BEGIN CERTIFICATE-----
/// <base64-encoded certificate content>
/// -----END CERTIFICATE-----
/// ```
///
/// The [`rustls-pemfile`](https://docs.rs/rustls-pemfile/latest/rustls_pemfile/) crate can be used
/// to parse PEM files. The [`rcgen`](https://docs.rs/rcgen/latest/rcgen/) crate can be used to
/// generate certificates and private keys.
///
/// ## Examples
///
/// Parsing a PEM file to extract DER-encoded certificates:
///
/// ```rust
/// use std::fs::File;
/// use std::io::BufReader;
/// use rustls::Certificate;
///
/// fn load_certificates_from_pem(path: &str) -> std::io::Result<Vec<Certificate>> {
///     let file = File::open(path)?;
///     let mut reader = BufReader::new(file);
///     let certs = rustls_pemfile::certs(&mut reader)?;
///
///     Ok(certs.into_iter().map(Certificate).collect())
/// }
/// ```
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Certificate(pub Vec<u8>);

impl AsRef<[u8]> for Certificate {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Debug for Certificate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use super::bs_debug::BsDebug;
        f.debug_tuple("Certificate")
            .field(&BsDebug(&self.0))
            .finish()
    }
}

/// wrapper around internal representation of a parsed certificate. This is used in order to avoid parsing twice when specifying custom verification
#[cfg_attr(not(feature = "dangerous_configuration"), allow(unreachable_pub))]
#[cfg_attr(docsrs, doc(cfg(feature = "dangerous_configuration")))]
pub struct ParsedCertificate<'a>(pub(crate) webpki::EndEntityCert<'a>);

impl<'a> TryFrom<&'a Certificate> for ParsedCertificate<'a> {
    type Error = Error;
    fn try_from(value: &'a Certificate) -> Result<Self, Self::Error> {
        webpki::EndEntityCert::try_from(value.0.as_ref())
            .map_err(crate::verify::pki_error)
            .map(ParsedCertificate)
    }
}

#[cfg(test)]
mod test {
    use super::Certificate;

    #[test]
    fn certificate_debug() {
        assert_eq!(
            "Certificate(b\"ab\")",
            format!("{:?}", Certificate(b"ab".to_vec()))
        );
    }
}
