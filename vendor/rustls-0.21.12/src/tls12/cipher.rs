use crate::cipher::{make_nonce, Iv, MessageDecrypter, MessageEncrypter};
use crate::enums::ContentType;
use crate::enums::ProtocolVersion;
use crate::error::Error;
use crate::msgs::base::Payload;
use crate::msgs::codec;
use crate::msgs::fragmenter::MAX_FRAGMENT_LEN;
use crate::msgs::message::{BorrowedPlainMessage, OpaqueMessage, PlainMessage};

use ring::aead;

const TLS12_AAD_SIZE: usize = 8 + 1 + 2 + 2;

fn make_tls12_aad(
    seq: u64,
    typ: ContentType,
    vers: ProtocolVersion,
    len: usize,
) -> aead::Aad<[u8; TLS12_AAD_SIZE]> {
    let mut out = [0; TLS12_AAD_SIZE];
    codec::put_u64(seq, &mut out[0..]);
    out[8] = typ.get_u8();
    codec::put_u16(vers.get_u16(), &mut out[9..]);
    codec::put_u16(len as u16, &mut out[11..]);
    aead::Aad::from(out)
}

pub(crate) struct AesGcm;

impl Tls12AeadAlgorithm for AesGcm {
    fn decrypter(&self, dec_key: aead::LessSafeKey, dec_iv: &[u8]) -> Box<dyn MessageDecrypter> {
        let mut ret = GcmMessageDecrypter {
            dec_key,
            dec_salt: [0u8; 4],
        };

        debug_assert_eq!(dec_iv.len(), 4);
        ret.dec_salt.copy_from_slice(dec_iv);
        Box::new(ret)
    }

    fn encrypter(
        &self,
        enc_key: aead::LessSafeKey,
        write_iv: &[u8],
        explicit: &[u8],
    ) -> Box<dyn MessageEncrypter> {
        debug_assert_eq!(write_iv.len(), 4);
        debug_assert_eq!(explicit.len(), 8);

        // The GCM nonce is constructed from a 32-bit 'salt' derived
        // from the master-secret, and a 64-bit explicit part,
        // with no specified construction.  Thanks for that.
        //
        // We use the same construction as TLS1.3/ChaCha20Poly1305:
        // a starting point extracted from the key block, xored with
        // the sequence number.
        let mut iv = Iv(Default::default());
        iv.0[..4].copy_from_slice(write_iv);
        iv.0[4..].copy_from_slice(explicit);

        Box::new(GcmMessageEncrypter { enc_key, iv })
    }
}

pub(crate) struct ChaCha20Poly1305;

impl Tls12AeadAlgorithm for ChaCha20Poly1305 {
    fn decrypter(&self, dec_key: aead::LessSafeKey, iv: &[u8]) -> Box<dyn MessageDecrypter> {
        Box::new(ChaCha20Poly1305MessageDecrypter {
            dec_key,
            dec_offset: Iv::copy(iv),
        })
    }

    fn encrypter(
        &self,
        enc_key: aead::LessSafeKey,
        enc_iv: &[u8],
        _: &[u8],
    ) -> Box<dyn MessageEncrypter> {
        Box::new(ChaCha20Poly1305MessageEncrypter {
            enc_key,
            enc_offset: Iv::copy(enc_iv),
        })
    }
}

pub(crate) trait Tls12AeadAlgorithm: Send + Sync + 'static {
    fn decrypter(&self, key: aead::LessSafeKey, iv: &[u8]) -> Box<dyn MessageDecrypter>;
    fn encrypter(
        &self,
        key: aead::LessSafeKey,
        iv: &[u8],
        extra: &[u8],
    ) -> Box<dyn MessageEncrypter>;
}

/// A `MessageEncrypter` for AES-GCM AEAD ciphersuites. TLS 1.2 only.
struct GcmMessageEncrypter {
    enc_key: aead::LessSafeKey,
    iv: Iv,
}

/// A `MessageDecrypter` for AES-GCM AEAD ciphersuites.  TLS1.2 only.
struct GcmMessageDecrypter {
    dec_key: aead::LessSafeKey,
    dec_salt: [u8; 4],
}

const GCM_EXPLICIT_NONCE_LEN: usize = 8;
const GCM_OVERHEAD: usize = GCM_EXPLICIT_NONCE_LEN + 16;

impl MessageDecrypter for GcmMessageDecrypter {
    fn decrypt(&self, mut msg: OpaqueMessage, seq: u64) -> Result<PlainMessage, Error> {
        let payload = &mut msg.payload.0;
        if payload.len() < GCM_OVERHEAD {
            return Err(Error::DecryptError);
        }

        let nonce = {
            let mut nonce = [0u8; 12];
            nonce[..4].copy_from_slice(&self.dec_salt);
            nonce[4..].copy_from_slice(&payload[..8]);
            aead::Nonce::assume_unique_for_key(nonce)
        };

        let aad = make_tls12_aad(seq, msg.typ, msg.version, payload.len() - GCM_OVERHEAD);

        let plain_len = self
            .dec_key
            .open_within(nonce, aad, payload, GCM_EXPLICIT_NONCE_LEN..)
            .map_err(|_| Error::DecryptError)?
            .len();

        if plain_len > MAX_FRAGMENT_LEN {
            return Err(Error::PeerSentOversizedRecord);
        }

        payload.truncate(plain_len);
        Ok(msg.into_plain_message())
    }
}

impl MessageEncrypter for GcmMessageEncrypter {
    fn encrypt(&self, msg: BorrowedPlainMessage, seq: u64) -> Result<OpaqueMessage, Error> {
        let nonce = make_nonce(&self.iv, seq);
        let aad = make_tls12_aad(seq, msg.typ, msg.version, msg.payload.len());

        let total_len = msg.payload.len() + self.enc_key.algorithm().tag_len();
        let mut payload = Vec::with_capacity(GCM_EXPLICIT_NONCE_LEN + total_len);
        payload.extend_from_slice(&nonce.as_ref()[4..]);
        payload.extend_from_slice(msg.payload);

        self.enc_key
            .seal_in_place_separate_tag(nonce, aad, &mut payload[GCM_EXPLICIT_NONCE_LEN..])
            .map(|tag| payload.extend(tag.as_ref()))
            .map_err(|_| Error::EncryptError)?;

        Ok(OpaqueMessage {
            typ: msg.typ,
            version: msg.version,
            payload: Payload::new(payload),
        })
    }
}

/// The RFC7905/RFC7539 ChaCha20Poly1305 construction.
/// This implementation does the AAD construction required in TLS1.2.
/// TLS1.3 uses `TLS13MessageEncrypter`.
struct ChaCha20Poly1305MessageEncrypter {
    enc_key: aead::LessSafeKey,
    enc_offset: Iv,
}

/// The RFC7905/RFC7539 ChaCha20Poly1305 construction.
/// This implementation does the AAD construction required in TLS1.2.
/// TLS1.3 uses `TLS13MessageDecrypter`.
struct ChaCha20Poly1305MessageDecrypter {
    dec_key: aead::LessSafeKey,
    dec_offset: Iv,
}

const CHACHAPOLY1305_OVERHEAD: usize = 16;

impl MessageDecrypter for ChaCha20Poly1305MessageDecrypter {
    fn decrypt(&self, mut msg: OpaqueMessage, seq: u64) -> Result<PlainMessage, Error> {
        let payload = &mut msg.payload.0;

        if payload.len() < CHACHAPOLY1305_OVERHEAD {
            return Err(Error::DecryptError);
        }

        let nonce = make_nonce(&self.dec_offset, seq);
        let aad = make_tls12_aad(
            seq,
            msg.typ,
            msg.version,
            payload.len() - CHACHAPOLY1305_OVERHEAD,
        );

        let plain_len = self
            .dec_key
            .open_in_place(nonce, aad, payload)
            .map_err(|_| Error::DecryptError)?
            .len();

        if plain_len > MAX_FRAGMENT_LEN {
            return Err(Error::PeerSentOversizedRecord);
        }

        payload.truncate(plain_len);
        Ok(msg.into_plain_message())
    }
}

impl MessageEncrypter for ChaCha20Poly1305MessageEncrypter {
    fn encrypt(&self, msg: BorrowedPlainMessage, seq: u64) -> Result<OpaqueMessage, Error> {
        let nonce = make_nonce(&self.enc_offset, seq);
        let aad = make_tls12_aad(seq, msg.typ, msg.version, msg.payload.len());

        let total_len = msg.payload.len() + self.enc_key.algorithm().tag_len();
        let mut buf = Vec::with_capacity(total_len);
        buf.extend_from_slice(msg.payload);

        self.enc_key
            .seal_in_place_append_tag(nonce, aad, &mut buf)
            .map_err(|_| Error::EncryptError)?;

        Ok(OpaqueMessage {
            typ: msg.typ,
            version: msg.version,
            payload: Payload::new(buf),
        })
    }
}
