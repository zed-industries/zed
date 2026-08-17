use bytes::buf::Chain;
use bytes::Bytes;
use digest::{Digest, OutputSizeUser};
use generic_array::GenericArray;
use rand::thread_rng;
use rsa::{pkcs8::DecodePublicKey, Oaep, RsaPublicKey};
use sha1::Sha1;
use sha2::Sha256;

use crate::connection::stream::MySqlStream;
use crate::error::Error;
use crate::protocol::auth::AuthPlugin;
use crate::protocol::Packet;

impl AuthPlugin {
    pub(super) async fn scramble(
        self,
        stream: &mut MySqlStream,
        password: &str,
        nonce: &Chain<Bytes, Bytes>,
    ) -> Result<Vec<u8>, Error> {
        match self {
            // https://mariadb.com/kb/en/caching_sha2_password-authentication-plugin/
            AuthPlugin::CachingSha2Password => Ok(scramble_sha256(password, nonce).to_vec()),

            AuthPlugin::MySqlNativePassword => Ok(scramble_sha1(password, nonce).to_vec()),

            // https://mariadb.com/kb/en/sha256_password-plugin/
            AuthPlugin::Sha256Password => encrypt_rsa(stream, 0x01, password, nonce).await,

            AuthPlugin::MySqlClearPassword => {
                let mut pw_bytes = password.as_bytes().to_owned();
                pw_bytes.push(0); // null terminate
                Ok(pw_bytes)
            }
        }
    }

    pub(super) async fn handle(
        self,
        stream: &mut MySqlStream,
        packet: Packet<Bytes>,
        password: &str,
        nonce: &Chain<Bytes, Bytes>,
    ) -> Result<bool, Error> {
        match self {
            AuthPlugin::CachingSha2Password if packet[0] == 0x01 => {
                match packet[1] {
                    // AUTH_OK
                    0x03 => Ok(true),

                    // AUTH_CONTINUE
                    0x04 => {
                        let payload = encrypt_rsa(stream, 0x02, password, nonce).await?;

                        stream.write_packet(&*payload)?;
                        stream.flush().await?;

                        Ok(false)
                    }

                    v => {
                        Err(err_protocol!("unexpected result from fast authentication 0x{:x} when expecting 0x03 (AUTH_OK) or 0x04 (AUTH_CONTINUE)", v))
                    }
                }
            }

            _ => Err(err_protocol!(
                "unexpected packet 0x{:02x} for auth plugin '{}' during authentication",
                packet[0],
                self.name()
            )),
        }
    }
}

fn scramble_sha1(
    password: &str,
    nonce: &Chain<Bytes, Bytes>,
) -> GenericArray<u8, <Sha1 as OutputSizeUser>::OutputSize> {
    // SHA1( password ) ^ SHA1( seed + SHA1( SHA1( password ) ) )
    // https://mariadb.com/kb/en/connection/#mysql_native_password-plugin

    let mut ctx = Sha1::new();

    ctx.update(password);

    let mut pw_hash = ctx.finalize_reset();

    ctx.update(pw_hash);

    let pw_hash_hash = ctx.finalize_reset();

    ctx.update(nonce.first_ref());
    ctx.update(nonce.last_ref());
    ctx.update(pw_hash_hash);

    let pw_seed_hash_hash = ctx.finalize();

    xor_eq(&mut pw_hash, &pw_seed_hash_hash);

    pw_hash
}

fn scramble_sha256(
    password: &str,
    nonce: &Chain<Bytes, Bytes>,
) -> GenericArray<u8, <Sha256 as OutputSizeUser>::OutputSize> {
    // XOR(SHA256(password), SHA256(seed, SHA256(SHA256(password))))
    // https://mariadb.com/kb/en/caching_sha2_password-authentication-plugin/#sha-2-encrypted-password
    let mut ctx = Sha256::new();

    ctx.update(password);

    let mut pw_hash = ctx.finalize_reset();

    ctx.update(pw_hash);

    let pw_hash_hash = ctx.finalize_reset();

    ctx.update(nonce.first_ref());
    ctx.update(nonce.last_ref());
    ctx.update(pw_hash_hash);

    let pw_seed_hash_hash = ctx.finalize();

    xor_eq(&mut pw_hash, &pw_seed_hash_hash);

    pw_hash
}

async fn encrypt_rsa<'s>(
    stream: &'s mut MySqlStream,
    public_key_request_id: u8,
    password: &'s str,
    nonce: &'s Chain<Bytes, Bytes>,
) -> Result<Vec<u8>, Error> {
    // https://mariadb.com/kb/en/caching_sha2_password-authentication-plugin/

    if stream.is_tls {
        // If in a TLS stream, send the password directly in clear text
        return Ok(to_asciz(password));
    }

    // client sends a public key request
    stream.write_packet(&[public_key_request_id][..])?;
    stream.flush().await?;

    // server sends a public key response
    let packet = stream.recv_packet().await?;
    let rsa_pub_key = &packet[1..];

    // xor the password with the given nonce
    let mut pass = to_asciz(password);

    let (a, b) = (nonce.first_ref(), nonce.last_ref());
    let mut nonce = Vec::with_capacity(a.len() + b.len());
    nonce.extend_from_slice(a);
    nonce.extend_from_slice(b);

    xor_eq(&mut pass, &nonce);

    // client sends an RSA encrypted password
    let pkey = parse_rsa_pub_key(rsa_pub_key)?;
    let padding = Oaep::new::<sha1::Sha1>();
    pkey.encrypt(&mut thread_rng(), padding, &pass[..])
        .map_err(Error::protocol)
}

// XOR(x, y)
// If len(y) < len(x), wrap around inside y
fn xor_eq(x: &mut [u8], y: &[u8]) {
    let y_len = y.len();

    for i in 0..x.len() {
        x[i] ^= y[i % y_len];
    }
}

fn to_asciz(s: &str) -> Vec<u8> {
    let mut z = String::with_capacity(s.len() + 1);
    z.push_str(s);
    z.push('\0');

    z.into_bytes()
}

// https://docs.rs/rsa/0.3.0/rsa/struct.RSAPublicKey.html?search=#example-1
fn parse_rsa_pub_key(key: &[u8]) -> Result<RsaPublicKey, Error> {
    let pem = std::str::from_utf8(key).map_err(Error::protocol)?;

    // This takes advantage of the knowledge that we know
    // we are receiving a PKCS#8 RSA Public Key at all
    // times from MySQL

    RsaPublicKey::from_public_key_pem(pem).map_err(Error::protocol)
}
