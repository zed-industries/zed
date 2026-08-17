// Copyright (c) 2015, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/ssl.h>

#include <assert.h>
#include <string.h>

#include <openssl/aead.h>
#include <openssl/err.h>
#include <openssl/rand.h>

#include "../crypto/fipsmodule/cipher/internal.h"
#include "../crypto/internal.h"
#include "internal.h"


#if defined(BORINGSSL_UNSAFE_FUZZER_MODE)
#define FUZZER_MODE true
#else
#define FUZZER_MODE false
#endif

BSSL_NAMESPACE_BEGIN

SSLAEADContext::SSLAEADContext(uint16_t version_arg, bool is_dtls_arg,
                               const SSL_CIPHER *cipher_arg)
    : cipher_(cipher_arg),
      version_(version_arg),
      is_dtls_(is_dtls_arg),
      variable_nonce_included_in_record_(false),
      random_variable_nonce_(false),
      xor_fixed_nonce_(false),
      omit_length_in_ad_(false),
      ad_is_header_(false) {
  OPENSSL_memset(fixed_nonce_, 0, sizeof(fixed_nonce_));
}

SSLAEADContext::~SSLAEADContext() {}

UniquePtr<SSLAEADContext> SSLAEADContext::CreateNullCipher(bool is_dtls) {
  return MakeUnique<SSLAEADContext>(0 /* version */, is_dtls,
                                    nullptr /* cipher */);
}

UniquePtr<SSLAEADContext> SSLAEADContext::Create(
    enum evp_aead_direction_t direction, uint16_t version, bool is_dtls,
    const SSL_CIPHER *cipher, Span<const uint8_t> enc_key,
    Span<const uint8_t> mac_key, Span<const uint8_t> fixed_iv) {
  const EVP_AEAD *aead;
  uint16_t protocol_version;
  size_t expected_mac_key_len, expected_fixed_iv_len;
  if (!ssl_protocol_version_from_wire(&protocol_version, version) ||
      !ssl_cipher_get_evp_aead(&aead, &expected_mac_key_len,
                               &expected_fixed_iv_len, cipher, protocol_version,
                               is_dtls) ||
      // Ensure the caller returned correct key sizes.
      expected_fixed_iv_len != fixed_iv.size() ||
      expected_mac_key_len != mac_key.size()) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
    return nullptr;
  }

  uint8_t merged_key[EVP_AEAD_MAX_KEY_LENGTH];
  if (!mac_key.empty()) {
    // This is a "stateful" AEAD (for compatibility with pre-AEAD cipher
    // suites).
    if (mac_key.size() + enc_key.size() + fixed_iv.size() >
        sizeof(merged_key)) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
      return nullptr;
    }
    OPENSSL_memcpy(merged_key, mac_key.data(), mac_key.size());
    OPENSSL_memcpy(merged_key + mac_key.size(), enc_key.data(), enc_key.size());
    OPENSSL_memcpy(merged_key + mac_key.size() + enc_key.size(),
                   fixed_iv.data(), fixed_iv.size());
    enc_key = MakeConstSpan(merged_key,
                            enc_key.size() + mac_key.size() + fixed_iv.size());
  }

  UniquePtr<SSLAEADContext> aead_ctx =
      MakeUnique<SSLAEADContext>(version, is_dtls, cipher);
  if (!aead_ctx) {
    return nullptr;
  }

  assert(aead_ctx->ProtocolVersion() == protocol_version);

  if (!EVP_AEAD_CTX_init_with_direction(
          aead_ctx->ctx_.get(), aead, enc_key.data(), enc_key.size(),
          EVP_AEAD_DEFAULT_TAG_LENGTH, direction)) {
    return nullptr;
  }

  assert(EVP_AEAD_nonce_length(aead) <= EVP_AEAD_MAX_NONCE_LENGTH);
  static_assert(EVP_AEAD_MAX_NONCE_LENGTH < 256,
                "variable_nonce_len doesn't fit in uint8_t");
  aead_ctx->variable_nonce_len_ = (uint8_t)EVP_AEAD_nonce_length(aead);
  if (mac_key.empty()) {
    assert(fixed_iv.size() <= sizeof(aead_ctx->fixed_nonce_));
    OPENSSL_memcpy(aead_ctx->fixed_nonce_, fixed_iv.data(), fixed_iv.size());
    aead_ctx->fixed_nonce_len_ = fixed_iv.size();

    if (cipher->algorithm_enc & SSL_CHACHA20POLY1305) {
      // The fixed nonce into the actual nonce (the sequence number).
      aead_ctx->xor_fixed_nonce_ = true;
      aead_ctx->variable_nonce_len_ = 8;
    } else {
      // The fixed IV is prepended to the nonce.
      assert(fixed_iv.size() <= aead_ctx->variable_nonce_len_);
      aead_ctx->variable_nonce_len_ -= fixed_iv.size();
    }

    // AES-GCM uses an explicit nonce.
    if (cipher->algorithm_enc & (SSL_AES128GCM | SSL_AES256GCM)) {
      aead_ctx->variable_nonce_included_in_record_ = true;
    }

    // The TLS 1.3 construction XORs the fixed nonce into the sequence number
    // and omits the additional data.
    if (protocol_version >= TLS1_3_VERSION) {
      aead_ctx->xor_fixed_nonce_ = true;
      aead_ctx->variable_nonce_len_ = 8;
      aead_ctx->variable_nonce_included_in_record_ = false;
      aead_ctx->ad_is_header_ = true;
      assert(fixed_iv.size() >= aead_ctx->variable_nonce_len_);
    }
  } else {
    assert(protocol_version < TLS1_3_VERSION);
    aead_ctx->variable_nonce_included_in_record_ = true;
    aead_ctx->random_variable_nonce_ = true;
    aead_ctx->omit_length_in_ad_ = true;
  }

  return aead_ctx;
}

UniquePtr<SSLAEADContext> SSLAEADContext::CreatePlaceholderForQUIC(
    uint16_t version, const SSL_CIPHER *cipher) {
  return MakeUnique<SSLAEADContext>(version, false, cipher);
}

void SSLAEADContext::SetVersionIfNullCipher(uint16_t version) {
  if (is_null_cipher()) {
    version_ = version;
  }
}

uint16_t SSLAEADContext::ProtocolVersion() const {
  uint16_t protocol_version;
  if (!ssl_protocol_version_from_wire(&protocol_version, version_)) {
    assert(false);
    return 0;
  }
  return protocol_version;
}

uint16_t SSLAEADContext::RecordVersion() const {
  if (version_ == 0) {
    assert(is_null_cipher());
    return is_dtls_ ? DTLS1_VERSION : TLS1_VERSION;
  }

  if (ProtocolVersion() <= TLS1_2_VERSION) {
    return version_;
  }

  return TLS1_2_VERSION;
}

size_t SSLAEADContext::ExplicitNonceLen() const {
  if (!FUZZER_MODE && variable_nonce_included_in_record_) {
    return variable_nonce_len_;
  }
  return 0;
}

bool SSLAEADContext::SuffixLen(size_t *out_suffix_len, const size_t in_len,
                               const size_t extra_in_len) const {
  if (is_null_cipher() || FUZZER_MODE) {
    *out_suffix_len = extra_in_len;
    return true;
  }
  return !!EVP_AEAD_CTX_tag_len(ctx_.get(), out_suffix_len, in_len,
                                extra_in_len);
}

bool SSLAEADContext::CiphertextLen(size_t *out_len, const size_t in_len,
                                   const size_t extra_in_len) const {
  size_t len;
  if (!SuffixLen(&len, in_len, extra_in_len)) {
    return false;
  }
  len += ExplicitNonceLen();
  len += in_len;
  if (len < in_len || len >= 0xffff) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_OVERFLOW);
    return false;
  }
  *out_len = len;
  return true;
}

size_t SSLAEADContext::MaxOverhead() const {
  return ExplicitNonceLen() +
         (is_null_cipher() || FUZZER_MODE
              ? 0
              : EVP_AEAD_max_overhead(EVP_AEAD_CTX_aead(ctx_.get())));
}

Span<const uint8_t> SSLAEADContext::GetAdditionalData(
    uint8_t storage[13], uint8_t type, uint16_t record_version,
    const uint8_t seqnum[8], size_t plaintext_len, Span<const uint8_t> header) {
  if (ad_is_header_) {
    return header;
  }

  OPENSSL_memcpy(storage, seqnum, 8);
  size_t len = 8;
  storage[len++] = type;
  storage[len++] = static_cast<uint8_t>((record_version >> 8));
  storage[len++] = static_cast<uint8_t>(record_version);
  if (!omit_length_in_ad_) {
    storage[len++] = static_cast<uint8_t>((plaintext_len >> 8));
    storage[len++] = static_cast<uint8_t>(plaintext_len);
  }
  return MakeConstSpan(storage, len);
}

bool SSLAEADContext::Open(Span<uint8_t> *out, uint8_t type,
                          uint16_t record_version, const uint8_t seqnum[8],
                          Span<const uint8_t> header, Span<uint8_t> in) {
  if (is_null_cipher() || FUZZER_MODE) {
    // Handle the initial NULL cipher.
    *out = in;
    return true;
  }

  // TLS 1.2 AEADs include the length in the AD and are assumed to have fixed
  // overhead. Otherwise the parameter is unused.
  size_t plaintext_len = 0;
  if (!omit_length_in_ad_) {
    size_t overhead = MaxOverhead();
    if (in.size() < overhead) {
      // Publicly invalid.
      OPENSSL_PUT_ERROR(SSL, SSL_R_BAD_PACKET_LENGTH);
      return false;
    }
    plaintext_len = in.size() - overhead;
  }

  uint8_t ad_storage[13];
  Span<const uint8_t> ad = GetAdditionalData(ad_storage, type, record_version,
                                             seqnum, plaintext_len, header);

  // Assemble the nonce.
  uint8_t nonce[EVP_AEAD_MAX_NONCE_LENGTH];
  size_t nonce_len = 0;

  // Prepend the fixed nonce, or left-pad with zeros if XORing.
  if (xor_fixed_nonce_) {
    nonce_len = fixed_nonce_len_ - variable_nonce_len_;
    OPENSSL_memset(nonce, 0, nonce_len);
  } else {
    OPENSSL_memcpy(nonce, fixed_nonce_, fixed_nonce_len_);
    nonce_len += fixed_nonce_len_;
  }

  // Add the variable nonce.
  if (variable_nonce_included_in_record_) {
    if (in.size() < variable_nonce_len_) {
      // Publicly invalid.
      OPENSSL_PUT_ERROR(SSL, SSL_R_BAD_PACKET_LENGTH);
      return false;
    }
    OPENSSL_memcpy(nonce + nonce_len, in.data(), variable_nonce_len_);
    in = in.subspan(variable_nonce_len_);
  } else {
    assert(variable_nonce_len_ == 8);
    OPENSSL_memcpy(nonce + nonce_len, seqnum, variable_nonce_len_);
  }
  nonce_len += variable_nonce_len_;

  // XOR the fixed nonce, if necessary.
  if (xor_fixed_nonce_) {
    assert(nonce_len == fixed_nonce_len_);
    for (size_t i = 0; i < fixed_nonce_len_; i++) {
      nonce[i] ^= fixed_nonce_[i];
    }
  }

  // Decrypt in-place.
  size_t len;
  if (!EVP_AEAD_CTX_open(ctx_.get(), in.data(), &len, in.size(), nonce,
                         nonce_len, in.data(), in.size(), ad.data(),
                         ad.size())) {
    return false;
  }
  *out = in.subspan(0, len);
  return true;
}

bool SSLAEADContext::SealScatter(uint8_t *out_prefix, uint8_t *out,
                                 uint8_t *out_suffix, uint8_t type,
                                 uint16_t record_version,
                                 const uint8_t seqnum[8],
                                 Span<const uint8_t> header, const uint8_t *in,
                                 size_t in_len, const uint8_t *extra_in,
                                 size_t extra_in_len) {
  const size_t prefix_len = ExplicitNonceLen();
  size_t suffix_len;
  if (!SuffixLen(&suffix_len, in_len, extra_in_len)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_RECORD_TOO_LARGE);
    return false;
  }
  if ((in != out && buffers_alias(in, in_len, out, in_len)) ||
      buffers_alias(in, in_len, out_prefix, prefix_len) ||
      buffers_alias(in, in_len, out_suffix, suffix_len)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_OUTPUT_ALIASES_INPUT);
    return false;
  }

  if (is_null_cipher() || FUZZER_MODE) {
    // Handle the initial NULL cipher.
    OPENSSL_memmove(out, in, in_len);
    OPENSSL_memmove(out_suffix, extra_in, extra_in_len);
    return true;
  }

  uint8_t ad_storage[13];
  Span<const uint8_t> ad = GetAdditionalData(ad_storage, type, record_version,
                                             seqnum, in_len, header);

  // Assemble the nonce.
  uint8_t nonce[EVP_AEAD_MAX_NONCE_LENGTH];
  size_t nonce_len = 0;

  // Prepend the fixed nonce, or left-pad with zeros if XORing.
  if (xor_fixed_nonce_) {
    nonce_len = fixed_nonce_len_ - variable_nonce_len_;
    OPENSSL_memset(nonce, 0, nonce_len);
  } else {
    OPENSSL_memcpy(nonce, fixed_nonce_, fixed_nonce_len_);
    nonce_len += fixed_nonce_len_;
  }

  // Select the variable nonce.
  if (random_variable_nonce_) {
    assert(variable_nonce_included_in_record_);
    if (!RAND_bytes(nonce + nonce_len, variable_nonce_len_)) {
      return false;
    }
  } else {
    // When sending we use the sequence number as the variable part of the
    // nonce.
    assert(variable_nonce_len_ == 8);
    OPENSSL_memcpy(nonce + nonce_len, seqnum, variable_nonce_len_);
  }
  nonce_len += variable_nonce_len_;

  // Emit the variable nonce if included in the record.
  if (variable_nonce_included_in_record_) {
    assert(!xor_fixed_nonce_);
    if (buffers_alias(in, in_len, out_prefix, variable_nonce_len_)) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_OUTPUT_ALIASES_INPUT);
      return false;
    }
    OPENSSL_memcpy(out_prefix, nonce + fixed_nonce_len_, variable_nonce_len_);
  }

  // XOR the fixed nonce, if necessary.
  if (xor_fixed_nonce_) {
    assert(nonce_len == fixed_nonce_len_);
    for (size_t i = 0; i < fixed_nonce_len_; i++) {
      nonce[i] ^= fixed_nonce_[i];
    }
  }

  size_t written_suffix_len;
  bool result = !!EVP_AEAD_CTX_seal_scatter(
      ctx_.get(), out, out_suffix, &written_suffix_len, suffix_len, nonce,
      nonce_len, in, in_len, extra_in, extra_in_len, ad.data(), ad.size());
  assert(!result || written_suffix_len == suffix_len);
  return result;
}

bool SSLAEADContext::Seal(uint8_t *out, size_t *out_len, size_t max_out_len,
                          uint8_t type, uint16_t record_version,
                          const uint8_t seqnum[8], Span<const uint8_t> header,
                          const uint8_t *in, size_t in_len) {
  const size_t prefix_len = ExplicitNonceLen();
  size_t suffix_len;
  if (!SuffixLen(&suffix_len, in_len, 0)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_RECORD_TOO_LARGE);
    return false;
  }
  if (in_len + prefix_len < in_len ||
      in_len + prefix_len + suffix_len < in_len + prefix_len) {
    OPENSSL_PUT_ERROR(CIPHER, SSL_R_RECORD_TOO_LARGE);
    return false;
  }
  if (in_len + prefix_len + suffix_len > max_out_len) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_BUFFER_TOO_SMALL);
    return false;
  }

  if (!SealScatter(out, out + prefix_len, out + prefix_len + in_len, type,
                   record_version, seqnum, header, in, in_len, 0, 0)) {
    return false;
  }
  *out_len = prefix_len + in_len + suffix_len;
  return true;
}

bool SSLAEADContext::GetIV(const uint8_t **out_iv, size_t *out_iv_len) const {
  return !is_null_cipher() &&
         EVP_AEAD_CTX_get_iv(ctx_.get(), out_iv, out_iv_len);
}

#define SSLAEADCONTEXT_SERDE_VERSION 1

/*
 * SSLAEADContextVersion ::= INTEGER {v1 (1)}
 *
 * SSLAEADContext ::= SEQUENCE {
 *   serializationVersion SSLAEADContextVersion,
 *   cipher         INTEGER,
 *   cipherState    EvpAeadCtxState
 * }
 */
int SSLAEADContext::SerializeState(CBB *cbb) const {
  uint32_t cipher_id = SSL_CIPHER_get_id(cipher_);

  CBB seq;

  if (!CBB_add_asn1(cbb, &seq, CBS_ASN1_SEQUENCE) ||
      !CBB_add_asn1_uint64(&seq, SSLAEADCONTEXT_SERDE_VERSION) ||
      !CBB_add_asn1_uint64(&seq, cipher_id)) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_MALLOC_FAILURE);
    return 0;
  }

  if (!EVP_AEAD_CTX_serialize_state(ctx_.get(), &seq)) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_MALLOC_FAILURE);
    return 0;
  }

  return CBB_flush(cbb);
}

// See |SSLAEADContext::SerializeState| for a description of the serialization
// format.
int SSLAEADContext::DeserializeState(CBS *cbs) const {
  CBS seq;

  uint64_t serde_version;
  uint64_t cipher_id;

  if (!CBS_get_asn1(cbs, &seq, CBS_ASN1_SEQUENCE) ||
      !CBS_get_asn1_uint64(&seq, &serde_version) ||
      serde_version != SSLAEADCONTEXT_SERDE_VERSION ||
      !CBS_get_asn1_uint64(&seq, &cipher_id) || cipher_id > UINT32_MAX ||
      cipher_id != SSL_CIPHER_get_id(cipher_)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL_AEAD_CONTEXT);
    return 0;
  }

  if (!EVP_AEAD_CTX_deserialize_state(ctx_.get(), &seq)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_SERIALIZATION_INVALID_SSL_AEAD_CONTEXT);
    return 0;
  }

  return 1;
}

BSSL_NAMESPACE_END
