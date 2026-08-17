use openssl::{
    bn::BigNum,
    dh::Dh,
    hash::{Hasher, MessageDigest, hash},
    md::Md,
    memcmp,
    nid::Nid,
    pkcs5::pbkdf2_hmac,
    pkey::{Id, PKey},
    pkey_ctx::PkeyCtx,
    rand::rand_bytes,
    sign::Signer,
    symm::{Cipher, Crypter, Mode},
};
use zeroize::Zeroizing;

use crate::{Key, Mac, file};

const ENC_ALG: Nid = Nid::AES_128_CBC;
const MAC_ALG: Nid = Nid::SHA256;

pub fn encrypt(
    data: impl AsRef<[u8]>,
    key: &Key,
    iv: impl AsRef<[u8]>,
) -> Result<Vec<u8>, super::Error> {
    let cipher = Cipher::from_nid(ENC_ALG).unwrap();
    let mut encryptor = Crypter::new(cipher, Mode::Encrypt, key.as_ref(), Some(iv.as_ref()))
        .expect("Invalid key or IV length");
    encryptor.pad(true);

    let mut blob = vec![0; data.as_ref().len() + cipher.block_size()];
    // Unwrapping since adding `CIPHER_BLOCK_SIZE` to array is enough space for
    // PKCS7
    let mut encrypted_len = encryptor.update(data.as_ref(), &mut blob)?;
    encrypted_len += encryptor.finalize(&mut blob[encrypted_len..])?;

    blob.truncate(encrypted_len);

    Ok(blob)
}

fn decrypt_with_padding(
    blob: impl AsRef<[u8]>,
    key: &Key,
    iv: impl AsRef<[u8]>,
    pad: bool,
) -> Result<Zeroizing<Vec<u8>>, super::Error> {
    let cipher = Cipher::from_nid(ENC_ALG).unwrap();
    let mut decrypter = Crypter::new(cipher, Mode::Decrypt, key.as_ref(), Some(iv.as_ref()))
        .expect("Invalid key or IV length");
    decrypter.pad(pad);

    let mut data = Zeroizing::new(vec![0; blob.as_ref().len() + cipher.block_size()]);
    let mut decrypted_len = decrypter.update(blob.as_ref(), &mut data)?;
    decrypted_len += decrypter.finalize(&mut data[decrypted_len..])?;

    data.truncate(decrypted_len);

    Ok(data)
}

pub fn decrypt(
    blob: impl AsRef<[u8]>,
    key: &Key,
    iv: impl AsRef<[u8]>,
) -> Result<Zeroizing<Vec<u8>>, super::Error> {
    decrypt_with_padding(blob, key, iv, true)
}

pub(crate) fn decrypt_no_padding(
    blob: impl AsRef<[u8]>,
    key: &Key,
    iv: impl AsRef<[u8]>,
) -> Result<Zeroizing<Vec<u8>>, super::Error> {
    decrypt_with_padding(blob, key, iv, false)
}

pub(crate) fn iv_len() -> usize {
    let cipher = Cipher::from_nid(ENC_ALG).unwrap();
    cipher.iv_len().unwrap()
}

pub(crate) fn generate_private_key() -> Result<Zeroizing<Vec<u8>>, super::Error> {
    let cipher = Cipher::from_nid(ENC_ALG).unwrap();
    let mut buf = Zeroizing::new(vec![0; cipher.key_len()]);
    rand_bytes(&mut buf)?;
    Ok(buf)
}

pub(crate) fn generate_public_key(private_key: impl AsRef<[u8]>) -> Result<Vec<u8>, super::Error> {
    let private_key_bn = BigNum::from_slice(private_key.as_ref()).unwrap();
    let dh = Dh::from_pqg(
        BigNum::get_rfc2409_prime_1024().unwrap(),
        None,
        BigNum::from_u32(2).unwrap(),
    )?;
    Ok(dh.set_private_key(private_key_bn)?.public_key().to_vec())
}

pub(crate) fn generate_aes_key(
    private_key: impl AsRef<[u8]>,
    server_public_key: impl AsRef<[u8]>,
) -> Result<Zeroizing<Vec<u8>>, super::Error> {
    let private_key_bn = BigNum::from_slice(private_key.as_ref()).unwrap();
    let server_public_key_bn = BigNum::from_slice(server_public_key.as_ref()).unwrap();
    let dh = Dh::from_pqg(
        BigNum::get_rfc2409_prime_1024().unwrap(),
        None,
        BigNum::from_u32(2).unwrap(),
    )?;
    let mut common_secret_bytes = dh
        .set_private_key(private_key_bn)?
        .compute_key(&server_public_key_bn)?;

    let mut common_secret_padded = vec![0; 128 - common_secret_bytes.len()];
    // inefficient, but ok for now
    common_secret_padded.append(&mut common_secret_bytes);

    // hkdf
    // input_keying_material
    let ikm = common_secret_padded;

    let mut okm = Zeroizing::new(vec![0; 16]);
    let mut ctx = PkeyCtx::new_id(Id::HKDF)?;
    ctx.derive_init()?;
    ctx.set_hkdf_md(Md::sha256())?;
    ctx.set_hkdf_key(&ikm)?;
    ctx.derive(Some(okm.as_mut()))
        .expect("hkdf expand should never fail");
    Ok(okm)
}

pub fn generate_iv() -> Result<Vec<u8>, super::Error> {
    let mut buf = vec![0; iv_len()];
    rand_bytes(&mut buf)?;
    Ok(buf)
}

pub(crate) fn mac_len() -> usize {
    let md = MessageDigest::from_nid(MAC_ALG).unwrap();
    md.size()
}

pub(crate) fn compute_mac(data: impl AsRef<[u8]>, key: &Key) -> Result<Mac, super::Error> {
    let md = MessageDigest::from_nid(MAC_ALG).unwrap();
    let mac_key = PKey::hmac(key.as_ref())?;
    let mut signer = Signer::new(md, &mac_key)?;
    signer.update(data.as_ref())?;
    signer.sign_to_vec().map_err(From::from).map(Mac::new)
}

pub(crate) fn verify_mac(
    data: impl AsRef<[u8]>,
    key: &Key,
    expected_mac: impl AsRef<[u8]>,
) -> Result<bool, super::Error> {
    Ok(memcmp::eq(
        compute_mac(&data, key)?.as_slice(),
        expected_mac.as_ref(),
    ))
}

pub(crate) fn verify_checksum_md5(digest: impl AsRef<[u8]>, content: impl AsRef<[u8]>) -> bool {
    memcmp::eq(
        &hash(MessageDigest::md5(), content.as_ref()).unwrap(),
        digest.as_ref(),
    )
}

pub(crate) fn derive_key(
    secret: impl AsRef<[u8]>,
    key_strength: Result<(), file::WeakKeyError>,
    salt: impl AsRef<[u8]>,
    iteration_count: usize,
) -> Result<Key, super::Error> {
    let cipher = Cipher::from_nid(ENC_ALG).unwrap();
    let mut key = Key::new_with_strength(vec![0; cipher.block_size()], key_strength);

    let md = MessageDigest::from_nid(MAC_ALG).unwrap();
    pbkdf2_hmac(
        secret.as_ref(),
        salt.as_ref(),
        iteration_count,
        md,
        key.as_mut(),
    )?;

    Ok(key)
}

pub(crate) fn legacy_derive_key_and_iv(
    secret: impl AsRef<[u8]>,
    key_strength: Result<(), file::WeakKeyError>,
    salt: impl AsRef<[u8]>,
    iteration_count: usize,
) -> Result<(Key, Vec<u8>), super::Error> {
    let cipher = Cipher::from_nid(ENC_ALG).unwrap();
    let mut buffer = vec![0; cipher.key_len() + cipher.iv_len().unwrap()];
    let mut hasher = Hasher::new(MessageDigest::sha256())?;
    let mut pos = 0usize;

    loop {
        hasher.update(secret.as_ref())?;
        hasher.update(salt.as_ref())?;
        let mut digest = hasher.finish()?;

        for _ in 1..iteration_count {
            // We can't pass an instance, the borrow checker
            // would complain about digest being dropped at the end of
            // for block
            #[allow(clippy::needless_borrows_for_generic_args)]
            hasher.update(&digest)?;
            digest = hasher.finish()?;
        }

        let to_read = usize::min(digest.len(), buffer.len() - pos);
        buffer[pos..].copy_from_slice(&(&*digest)[..to_read]);
        pos += to_read;

        if pos == buffer.len() {
            break;
        }

        // We can't pass an instance, the borrow checker
        // would complain about digest being dropped at the end of
        // for block
        #[allow(clippy::needless_borrows_for_generic_args)]
        hasher.update(&digest)?;
    }

    let iv = buffer.split_off(cipher.key_len());
    Ok((Key::new_with_strength(buffer, key_strength), iv))
}
