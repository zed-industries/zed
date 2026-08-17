// Copyright (c) 2015, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/ssl.h>

#include <assert.h>
#include <string.h>

#include <utility>

#include <openssl/bn.h>
#include <openssl/bytestring.h>
#include <openssl/curve25519.h>
#include <openssl/ec.h>
#include <openssl/err.h>
#include <openssl/evp.h>
#include <openssl/hrss.h>
#include <openssl/mem.h>
#include <openssl/nid.h>
#include <openssl/rand.h>
#include <openssl/span.h>

#include "internal.h"
#include "../crypto/internal.h"
#include "../crypto/fipsmodule/ec/internal.h"
#include "../crypto/fipsmodule/ml_kem/ml_kem.h"

BSSL_NAMESPACE_BEGIN

namespace {

class ECKeyShare : public SSLKeyShare {
 public:
  ECKeyShare(const EC_GROUP *group, uint16_t group_id)
      : group_(group), group_id_(group_id) {}

  uint16_t GroupID() const override { return group_id_; }

  bool Offer(CBB *out) override {
    assert(!private_key_);
    // Generate a private key.
    private_key_.reset(BN_new());
    if (!private_key_ ||
        !BN_rand_range_ex(private_key_.get(), 1, EC_GROUP_get0_order(group_))) {
      return false;
    }

    // Compute the corresponding public key and serialize it.
    UniquePtr<EC_POINT> public_key(EC_POINT_new(group_));
    if (!public_key ||
        !EC_POINT_mul(group_, public_key.get(), private_key_.get(), nullptr,
                      nullptr, /*ctx=*/nullptr) ||
        !EC_POINT_point2cbb(out, group_, public_key.get(),
                            POINT_CONVERSION_UNCOMPRESSED, /*ctx=*/nullptr)) {
      return false;
    }

    return true;
  }

  bool Finish(Array<uint8_t> *out_secret, uint8_t *out_alert,
              Span<const uint8_t> peer_key) override {
    assert(group_);
    assert(private_key_);
    *out_alert = SSL_AD_INTERNAL_ERROR;

    UniquePtr<EC_POINT> peer_point(EC_POINT_new(group_));
    UniquePtr<EC_POINT> result(EC_POINT_new(group_));
    UniquePtr<BIGNUM> x(BN_new());
    if (!peer_point || !result || !x) {
      return false;
    }

    if (peer_key.empty() || peer_key[0] != POINT_CONVERSION_UNCOMPRESSED ||
        !EC_POINT_oct2point(group_, peer_point.get(), peer_key.data(),
                            peer_key.size(), /*ctx=*/nullptr)) {
      OPENSSL_PUT_ERROR(SSL, SSL_R_BAD_ECPOINT);
      *out_alert = SSL_AD_ILLEGAL_PARAMETER;
      return false;
    }

    // Compute the x-coordinate of |peer_key| * |private_key_|.
    if (!EC_POINT_mul(group_, result.get(), nullptr, peer_point.get(),
                      private_key_.get(), /*ctx=*/nullptr) ||
        !EC_POINT_get_affine_coordinates_GFp(group_, result.get(), x.get(),
                                             nullptr, /*ctx=*/nullptr)) {
      return false;
    }

    // Encode the x-coordinate left-padded with zeros.
    Array<uint8_t> secret;
    if (!secret.Init((EC_GROUP_get_degree(group_) + 7) / 8) ||
        !BN_bn2bin_padded(secret.data(), secret.size(), x.get())) {
      return false;
    }

    *out_secret = std::move(secret);
    return true;
  }

  bool SerializePrivateKey(CBB *out) override {
    assert(group_);
    assert(private_key_);
    // Padding is added to avoid leaking the length.
    size_t len = BN_num_bytes(EC_GROUP_get0_order(group_));
    return BN_bn2cbb_padded(out, len, private_key_.get());
  }

  bool DeserializePrivateKey(CBS *in) override {
    assert(!private_key_);
    private_key_.reset(BN_bin2bn(CBS_data(in), CBS_len(in), nullptr));
    return private_key_ != nullptr;
  }

 private:
  UniquePtr<BIGNUM> private_key_;
  const EC_GROUP *const group_ = nullptr;
  uint16_t group_id_;
};

class X25519KeyShare : public SSLKeyShare {
 public:
  X25519KeyShare() {}

  uint16_t GroupID() const override { return SSL_GROUP_X25519; }

  bool Offer(CBB *out) override {
    uint8_t public_key[32];
    X25519_keypair(public_key, private_key_);
    return !!CBB_add_bytes(out, public_key, sizeof(public_key));
  }

  bool Finish(Array<uint8_t> *out_secret, uint8_t *out_alert,
              Span<const uint8_t> peer_key) override {
    *out_alert = SSL_AD_INTERNAL_ERROR;

    Array<uint8_t> secret;
    if (!secret.Init(32)) {
      return false;
    }

    if (peer_key.size() != 32 ||
        !X25519(secret.data(), private_key_, peer_key.data())) {
      *out_alert = SSL_AD_ILLEGAL_PARAMETER;
      OPENSSL_PUT_ERROR(SSL, SSL_R_BAD_ECPOINT);
      return false;
    }

    *out_secret = std::move(secret);
    return true;
  }

  bool SerializePrivateKey(CBB *out) override {
    return CBB_add_bytes(out, private_key_, sizeof(private_key_));
  }

  bool DeserializePrivateKey(CBS *in) override {
    if (CBS_len(in) != sizeof(private_key_) ||
        !CBS_copy_bytes(in, private_key_, sizeof(private_key_))) {
      return false;
    }
    return true;
  }

 private:
  uint8_t private_key_[32];
};

class KEMKeyShare : public SSLKeyShare {
 public:
  KEMKeyShare(int nid, uint16_t group_id) : nid_(nid), group_id_(group_id) {}

  uint16_t GroupID() const override { return group_id_; }

  bool Offer(CBB *out) override {
    // Ensure |out| is valid, has no children, and has been initialized.
    // We check that |out| has no children because otherwise CBB_data()
    // will produce a fatal error by way of assert(cbb->child == NULL).
    if (!out || out->child || !CBB_data(out)) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_PASSED_NULL_PARAMETER);
      return false;
    }

    // ctx_ should not have been initialized (by e.g. a previous call
    // to Offer()).
    if (ctx_) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
      return false;
    }

    // Initialize the KEM ctx
    ctx_.reset(EVP_PKEY_CTX_new_id(EVP_PKEY_KEM, nullptr));
    if (!ctx_ || !EVP_PKEY_CTX_kem_set_params(ctx_.get(), nid_)) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
      return false;
    }

    // Generate a KEM keypair and retrieve the length of the public key
    size_t public_key_len = 0;
    EVP_PKEY *raw_key = nullptr;
    if (!EVP_PKEY_keygen_init(ctx_.get()) ||
        !EVP_PKEY_keygen(ctx_.get(), &raw_key) ||
        !raw_key ||
        !EVP_PKEY_get_raw_public_key(raw_key, nullptr, &public_key_len)) {
      EVP_PKEY_free(raw_key);
      OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
      return false;
    }

    // Retain the secret key for decapsulation
    UniquePtr<EVP_PKEY> pkey(raw_key);
    ctx_.reset(EVP_PKEY_CTX_new(pkey.get(), nullptr));
    if (!ctx_) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
      return false;
    }

    uint8_t *public_key = NULL;
    size_t public_key_bytes_written = public_key_len;
    if (!CBB_add_space(out, &public_key, public_key_len) ||
        !EVP_PKEY_get_raw_public_key(pkey.get(), public_key,
                                     &public_key_bytes_written) ||
                                     public_key_bytes_written != public_key_len) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
      return false;
    }

    return true;
  }

  // Because this is a KEM key share, |out_public_key| will be the ciphertext
  // resulting from the encapsulation of the shared secret under the
  // received public key, |peek_key|.
  bool Accept(CBB *out_public_key, Array<uint8_t> *out_secret,
              uint8_t *out_alert, Span<const uint8_t> peer_key) override {
    // Basic nullptr checks
    if (!out_alert) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_PASSED_NULL_PARAMETER);
      return false;
    }

    // Set alert to internal error by default
    *out_alert = SSL_AD_INTERNAL_ERROR;

    if (!out_secret) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_PASSED_NULL_PARAMETER);
      return false;
    }

    // Ensure |out_public_key| is valid, has no children, and has been
    // initialized. We check that |out_public_key| has no children because
    // otherwise CBB_data() will produce a fatal error by way of
    // assert(cbb->child == NULL).
    if (!out_public_key || out_public_key->child || !CBB_data(out_public_key)) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_PASSED_NULL_PARAMETER);
      return false;
    }

    // ctx_ should not have been initialized (by e.g. a call to Offer()).
    if (ctx_) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
      return false;
    }

    // Ensure that peer_key is valid
    if (!peer_key.data() || peer_key.empty()) {
      *out_alert = SSL_AD_DECODE_ERROR;
      OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
      return false;
    }

    // Initialize pkey from the received public key
    UniquePtr<EVP_PKEY> pkey(
      EVP_PKEY_kem_new_raw_public_key(nid_, peer_key.begin(), peer_key.size()));

    if (!pkey) {
      *out_alert = SSL_AD_ILLEGAL_PARAMETER;
      OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
      return false;
    }

    // Initialize the context with the pkey
    ctx_.reset(EVP_PKEY_CTX_new(pkey.get(), nullptr));
    if (!ctx_) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
      return false;
    }

    // Retrieve the lengths of the ciphertext and shared secret
    size_t ciphertext_len = 0;
    size_t secret_len = 0;
    if (!EVP_PKEY_encapsulate(ctx_.get(), nullptr, &ciphertext_len,
                              nullptr, &secret_len)) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
      return false;
    }

    // EVP_PKEY_encapsulate() will generate a shared secret, then encapsulate it
    // with the peer's public key. We send the resultant ciphertext to the
    // peer by writing it to |out_public_key|, then...
    Array<uint8_t> shared_secret;
    if (!shared_secret.Init(secret_len)) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_MALLOC_FAILURE);
      return false;
    }

    uint8_t *ciphertext = NULL;
    size_t ciphertext_bytes_written = ciphertext_len;
    size_t secret_bytes_written = secret_len;

    if (!CBB_add_space(out_public_key, &ciphertext, ciphertext_len) ||
        !EVP_PKEY_encapsulate(ctx_.get(), ciphertext,
                              &ciphertext_bytes_written, shared_secret.data(),
                              &secret_bytes_written) ||
                              ciphertext_bytes_written != ciphertext_len ||
                              secret_bytes_written != secret_len) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
      return false;
    }

    // ...we retain the shared secret for our own use.
    *out_secret = std::move(shared_secret);

    // Success; clear the alert
    *out_alert = 0;
    return true;
  }

  // Because this is a KEM key share, |peer_key| is actually the ciphertext
  // resulting from the peer encapsulating the shared secret under our public key.
  // In Finish(), we use our previously generated secret key to decrypt
  // that ciphertext and obtain the shared secret.
  bool Finish(Array<uint8_t> *out_secret, uint8_t *out_alert,
              Span<const uint8_t> peer_key) override {
    // Basic nullptr checks
    if (!out_alert) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_PASSED_NULL_PARAMETER);
      return false;
    }

    // Set alert to internal error by default
    *out_alert = SSL_AD_INTERNAL_ERROR;

    if (!out_secret) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_PASSED_NULL_PARAMETER);
      return false;
    }

    // ctx_ should have been initialized by a previous call to Offer()
    if (!ctx_) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
      return false;
    }

    // Retrieve the length of the shared secret
    size_t secret_len = 0;
    if (!EVP_PKEY_decapsulate(ctx_.get(), nullptr, &secret_len, nullptr, peer_key.size())) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
      return false;
    }

    // |peer_key| is the ciphertext, encapsulating the shared secret,
    // that the peer generated using our public key. We use our secret
    // key to decapsulate it, and retain the shared secret by writing
    // it to |out_secret|.
    uint8_t *ciphertext = (uint8_t *)peer_key.begin();
    Array<uint8_t> shared_secret;
    if (!shared_secret.Init(secret_len)) {
      OPENSSL_PUT_ERROR(SSL, ERR_R_MALLOC_FAILURE);
      return false;
    }

    size_t secret_bytes_written = secret_len;
    if (!EVP_PKEY_decapsulate(ctx_.get(), shared_secret.data(),
                              &secret_bytes_written, ciphertext,
                              peer_key.size()) ||
                              secret_bytes_written != secret_len) {
      OPENSSL_PUT_ERROR(SSL, SSL_AD_ILLEGAL_PARAMETER);
      return false;
    }

    *out_secret = std::move(shared_secret);

    // Success; clear the alert
    *out_alert = 0;
    return true;
  }

 private:
  int nid_;
  uint16_t group_id_;
  UniquePtr<EVP_PKEY_CTX> ctx_;
};

// A HybridKeyShare consists of key shares from two or more component groups,
// all of which are used to generate a hybrid shared secret.
// See https://datatracker.ietf.org/doc/html/draft-ietf-tls-hybrid-design.
class HybridKeyShare : public SSLKeyShare {
  public:
    HybridKeyShare(uint16_t group_id) : group_id_(group_id),
    exchange_performed(false), hybrid_group_(nullptr) {
      for (const HybridGroup &hybrid_group : HybridGroups()) {
         if (group_id_ == hybrid_group.group_id) {
           hybrid_group_ = &hybrid_group;
           for (size_t i = 0; i < NUM_HYBRID_COMPONENTS; i++) {
             key_shares_[i] = SSLKeyShare::Create(hybrid_group.component_group_ids[i]);
           }
           return;
         }
      }
      hybrid_group_ = nullptr;
    }

    uint16_t GroupID() const override { return group_id_; }

    bool Offer(CBB *out) override {
      // Ensure |out| is valid, has no children, and has been initialized.
      // We check that |out| has no children because otherwise CBB_data()
      // will produce a fatal error by way of assert(cbb->child == NULL);
      if (!out || out->child || !CBB_data(out)) {
        OPENSSL_PUT_ERROR(SSL, ERR_R_PASSED_NULL_PARAMETER);
        return false;
      }

      if (!hybrid_group_ || this->exchange_performed) {
        OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
        return false;
      }

      // Iterate through the component groups and Offer() each of their key
      // shares. If one of the calls to a component Offer() fails,
      // OPENSSL_PUT_ERROR will be set appropriately in that component.
      for (const UniquePtr<SSLKeyShare> &key_share : key_shares_) {
        if (!key_share || !key_share->Offer(out)) {
          return false;
        }
      }
      this->exchange_performed = true;
      return true;
    }

    bool Accept(CBB *out_public_key, Array<uint8_t> *out_secret,
                uint8_t *out_alert, Span<const uint8_t> peer_key) override {
      if (!out_alert) {
        OPENSSL_PUT_ERROR(SSL, ERR_R_PASSED_NULL_PARAMETER);
        return false;
      }

      // Set alert to internal error by default
      *out_alert = SSL_AD_INTERNAL_ERROR;

      if (!out_secret|| !peer_key.data()) {
        OPENSSL_PUT_ERROR(SSL, ERR_R_PASSED_NULL_PARAMETER);
        return false;
      }

      // Ensure |out_public_key| is valid, has no children, and has been
      // initialized. We check that |out_public_key| has no children because
      // otherwise CBB_data() will produce a fatal error by way of
      // assert(cbb->child == NULL);
      if (!out_public_key || out_public_key->child || !CBB_data(out_public_key)) {
        OPENSSL_PUT_ERROR(SSL, ERR_R_PASSED_NULL_PARAMETER);
        return false;
      }

      if (!hybrid_group_ || this->exchange_performed) {
        OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
        return false;
      }

      // A hybrid shared secret with two components should be 64 bytes.
      // If it happens to be larger, the CBB will grow accordingly.
      CBB hybrid_shared_secret;
      if (!CBB_init(&hybrid_shared_secret, 64)) {
        OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
        return false;
      }

      // Accept() each component key share. Each component's Accept() function
      // will generate a shared secret and a public key to be sent back to
      // the peer. The hybrid public key is the concatenation of all component
      // public keys; the hybrid shared secret is the concatenation of all
      // component shared secrets.
      size_t peer_key_read_index = 0;
      for (size_t i = 0; i < NUM_HYBRID_COMPONENTS; i++) {
        size_t component_key_size = 0;
        if (!get_component_offer_key_share_size(&component_key_size, hybrid_group_->component_group_ids[i])) {
          OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
          return false;
        }

        // Verify that |peer_key| contains enough data
        if (peer_key_read_index + component_key_size > peer_key.size()) {
          CBB_cleanup(&hybrid_shared_secret);
          *out_alert = SSL_AD_ILLEGAL_PARAMETER;
          OPENSSL_PUT_ERROR(SSL, SSL_R_BAD_HYBRID_KEYSHARE);
          return false;
        }

        Span<const uint8_t> component_key =
          peer_key.subspan(peer_key_read_index, component_key_size);

        Array<uint8_t> component_secret;
        if (!key_shares_[i] ||
            !key_shares_[i]->Accept(out_public_key, &component_secret, out_alert, component_key) ||
            !CBB_add_bytes(&hybrid_shared_secret, component_secret.data(), component_secret.size())) {
          CBB_cleanup(&hybrid_shared_secret);
          OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
          return false;
        }

        peer_key_read_index += component_key_size;
      }

      // Final validation that |peer_key| was the correct size
      if (peer_key_read_index != peer_key.size()) {
        CBB_cleanup(&hybrid_shared_secret);
        *out_alert = SSL_AD_DECODE_ERROR;
        OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
        return false;
      }

      // Retain the hybrid shared secret for our use.
      if (!CBBFinishArray(&hybrid_shared_secret, out_secret)) {
        CBB_cleanup(&hybrid_shared_secret);
        OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
        return false;
      }

      // Success; clear the alert
      *out_alert = 0;

      this->exchange_performed = true;
      return true;
    }

    bool Finish(Array<uint8_t> *out_secret, uint8_t *out_alert,
                Span<const uint8_t> peer_key) override {
      if (!out_alert) {
        OPENSSL_PUT_ERROR(SSL, ERR_R_PASSED_NULL_PARAMETER);
        return false;
      }

      // Set alert to internal error by default
      *out_alert = SSL_AD_INTERNAL_ERROR;

      if (!out_secret || !peer_key.data()) {
        OPENSSL_PUT_ERROR(SSL, ERR_R_PASSED_NULL_PARAMETER);
        return false;
      }

      if (!hybrid_group_ || !this->exchange_performed) {
        OPENSSL_PUT_ERROR(SSL, ERR_R_SHOULD_NOT_HAVE_BEEN_CALLED);
        return false;
      }

      // A hybrid shared secret with two components should be 64 bytes.
      // If it happens to be larger, the CBB will grow accordingly.
      CBB hybrid_shared_secret;
      if (!CBB_init(&hybrid_shared_secret, 64)) {
        OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
        return false;
      }

      // Finish() each component key share. Each component's Finish() function
      // will generate a shared secret. The hybrid shared secret is the
      // concatenation of all component shared secrets.
      size_t peer_key_index = 0;
      for (size_t i = 0; i < NUM_HYBRID_COMPONENTS; i++) {

        size_t component_key_size = 0;
        if (!get_component_accept_key_share_size(&component_key_size, hybrid_group_->component_group_ids[i])) {
          OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
          return false;
        }

        // Verify that |peer_key| contains enough data
        if (peer_key_index + component_key_size > peer_key.size()) {
          CBB_cleanup(&hybrid_shared_secret);
          *out_alert = SSL_AD_DECODE_ERROR;
          OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
          return false;
        }

        Span<const uint8_t> component_key =
          peer_key.subspan(peer_key_index, component_key_size);

        Array<uint8_t> component_secret;
        if (!key_shares_[i] ||
            !key_shares_[i]->Finish(&component_secret, out_alert, component_key) ||
            !CBB_add_bytes(&hybrid_shared_secret, component_secret.data(), component_secret.size())) {
          CBB_cleanup(&hybrid_shared_secret);
          OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
          return false;
        }

        peer_key_index += component_key_size;
      }

      // Final validation that |peer_key| was the correct size
      if (peer_key_index != peer_key.size()) {
        CBB_cleanup(&hybrid_shared_secret);
        *out_alert = SSL_AD_ILLEGAL_PARAMETER;
        OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
        return false;
      }

      // Retain the hybrid shared secret for our use.
      if (!CBBFinishArray(&hybrid_shared_secret, out_secret)) {
        CBB_cleanup(&hybrid_shared_secret);
        OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
        return false;
      }

      // Success; clear the alert
      *out_alert = 0;
      return true;
    }

  private:
   // Only need to support SSL Groups that are a component of a supported HybridKeyShare
   bool get_component_offer_key_share_size(size_t *out, uint16_t component_group_id) {
     switch (component_group_id) {
     case SSL_GROUP_SECP256R1:
       *out = 1 + (2 * EC_P256R1_FIELD_ELEM_BYTES);
       return true;
     case SSL_GROUP_SECP384R1:
       *out = 1 + (2 * EC_P384R1_FIELD_ELEM_BYTES);
       return true;
     case SSL_GROUP_MLKEM768:
       *out = MLKEM768_PUBLIC_KEY_BYTES;
       return true;
    case SSL_GROUP_MLKEM1024:
       *out = MLKEM1024_PUBLIC_KEY_BYTES;
       return true;
     case SSL_GROUP_X25519:
       *out = 32;
       return true;
     default:
       return false;
     }
   }

   // Only need to support SSL Groups that are a component of a supported HybridKeyShare
   bool get_component_accept_key_share_size(size_t *out, uint16_t component_group_id) {
     switch (component_group_id) {
     case SSL_GROUP_SECP256R1:
       *out = 1 + (2 * EC_P256R1_FIELD_ELEM_BYTES);
       return true;
     case SSL_GROUP_SECP384R1:
       *out = 1 + (2 * EC_P384R1_FIELD_ELEM_BYTES);
       return true;
     case SSL_GROUP_MLKEM768:
       *out = MLKEM768_CIPHERTEXT_BYTES;
       return true;
     case SSL_GROUP_MLKEM1024:
       *out = MLKEM1024_CIPHERTEXT_BYTES;
       return true;
     case SSL_GROUP_X25519:
       *out = 32;
       return true;
     default:
       return false;
     }
   }

   uint16_t group_id_;
   bool exchange_performed;
   const HybridGroup *hybrid_group_;
   UniquePtr<SSLKeyShare> key_shares_[NUM_HYBRID_COMPONENTS];
};

CONSTEXPR_ARRAY NamedGroup kNamedGroups[] = {
    {NID_secp224r1, SSL_GROUP_SECP224R1, "P-224", "secp224r1"},
    {NID_X9_62_prime256v1, SSL_GROUP_SECP256R1, "P-256", "prime256v1"},
    {NID_secp384r1, SSL_GROUP_SECP384R1, "P-384", "secp384r1"},
    {NID_secp521r1, SSL_GROUP_SECP521R1, "P-521", "secp521r1"},
    {NID_X25519, SSL_GROUP_X25519, "X25519", "x25519"},
    {NID_SecP256r1MLKEM768, SSL_GROUP_SECP256R1_MLKEM768, "SecP256r1MLKEM768", ""},
    {NID_X25519MLKEM768, SSL_GROUP_X25519_MLKEM768, "X25519MLKEM768", ""},
    {NID_SecP384r1MLKEM1024, SSL_GROUP_SECP384R1_MLKEM1024, "SecP384r1MLKEM1024", ""},
    {NID_MLKEM512, SSL_GROUP_MLKEM512, "MLKEM512", "ML-KEM-512"},
    {NID_MLKEM768, SSL_GROUP_MLKEM768, "MLKEM768", "ML-KEM-768"},
    {NID_MLKEM1024, SSL_GROUP_MLKEM1024, "MLKEM1024", "ML-KEM-1024"},
};

CONSTEXPR_ARRAY uint16_t kPQGroups[] = {
    SSL_GROUP_MLKEM512,
    SSL_GROUP_MLKEM768,
    SSL_GROUP_MLKEM1024,
    SSL_GROUP_SECP256R1_MLKEM768,
    SSL_GROUP_X25519_MLKEM768,
    SSL_GROUP_SECP384R1_MLKEM1024,
};

CONSTEXPR_ARRAY HybridGroup kHybridGroups[] = {
  {
    SSL_GROUP_SECP256R1_MLKEM768,     // group_id
    {
      SSL_GROUP_SECP256R1,         // component_group_ids[0]
      SSL_GROUP_MLKEM768,          // component_group_ids[1]
    },
  },
  {
    SSL_GROUP_X25519_MLKEM768,     // group_id
    {
      // Note: MLKEM768 is sent first due to FIPS requirements.
      // For more details, see https://datatracker.ietf.org/doc/html/draft-kwiatkowski-tls-ecdhe-mlkem.html#section-3
      SSL_GROUP_MLKEM768,          // component_group_ids[0]
      SSL_GROUP_X25519,            // component_group_ids[1]
    },
  },
  {
    SSL_GROUP_SECP384R1_MLKEM1024,     // group_id
    {
      SSL_GROUP_SECP384R1,         // component_group_ids[0]
      SSL_GROUP_MLKEM1024,         // component_group_ids[1]
    },
  },
};

} // namespace

Span<const NamedGroup> NamedGroups() {
  return MakeConstSpan(kNamedGroups, OPENSSL_ARRAY_SIZE(kNamedGroups));
}

Span<const HybridGroup> HybridGroups() {
  return MakeConstSpan(kHybridGroups, OPENSSL_ARRAY_SIZE(kHybridGroups));
}

Span<const uint16_t> PQGroups() {
  return MakeConstSpan(kPQGroups, OPENSSL_ARRAY_SIZE(kPQGroups));
}

UniquePtr<SSLKeyShare> SSLKeyShare::Create(uint16_t group_id) {
  switch (group_id) {
    case SSL_GROUP_SECP224R1:
      return MakeUnique<ECKeyShare>(EC_group_p224(), SSL_GROUP_SECP224R1);
    case SSL_GROUP_SECP256R1:
      return MakeUnique<ECKeyShare>(EC_group_p256(), SSL_GROUP_SECP256R1);
    case SSL_GROUP_SECP384R1:
      return MakeUnique<ECKeyShare>(EC_group_p384(), SSL_GROUP_SECP384R1);
    case SSL_GROUP_SECP521R1:
      return MakeUnique<ECKeyShare>(EC_group_p521(), SSL_GROUP_SECP521R1);
    case SSL_GROUP_X25519:
      return MakeUnique<X25519KeyShare>();
    case SSL_GROUP_MLKEM512:
      return MakeUnique<KEMKeyShare>(NID_MLKEM512, SSL_GROUP_MLKEM512);
    case SSL_GROUP_MLKEM768:
      return MakeUnique<KEMKeyShare>(NID_MLKEM768, SSL_GROUP_MLKEM768);
    case SSL_GROUP_MLKEM1024:
      return MakeUnique<KEMKeyShare>(NID_MLKEM1024, SSL_GROUP_MLKEM1024);
    case SSL_GROUP_SECP256R1_MLKEM768:
      return MakeUnique<HybridKeyShare>(SSL_GROUP_SECP256R1_MLKEM768);
    case SSL_GROUP_X25519_MLKEM768:
      return MakeUnique<HybridKeyShare>(SSL_GROUP_X25519_MLKEM768);
    case SSL_GROUP_SECP384R1_MLKEM1024:
      return MakeUnique<HybridKeyShare>(SSL_GROUP_SECP384R1_MLKEM1024);
    default:
      return nullptr;
  }
}

bool SSLKeyShare::Accept(CBB *out_public_key, Array<uint8_t> *out_secret,
                         uint8_t *out_alert, Span<const uint8_t> peer_key) {
  *out_alert = SSL_AD_INTERNAL_ERROR;
  return Offer(out_public_key) &&
         Finish(out_secret, out_alert, peer_key);
}

bool ssl_nid_to_group_id(uint16_t *out_group_id, int nid) {
  for (const auto &group : kNamedGroups) {
    if (group.nid == nid) {
      *out_group_id = group.group_id;
      return true;
    }
  }
  return false;
}

bool ssl_group_id_to_nid(uint16_t *out_nid, int group_id) {
  GUARD_PTR(out_nid);
  for (const auto &group : kNamedGroups) {
    if (group.group_id == group_id) {
      *out_nid = group.nid;
      return true;
    }
  }
  return false;
}

bool ssl_name_to_group_id(uint16_t *out_group_id, const char *name, size_t len) {
  for (const auto &group : kNamedGroups) {
    if (len == strlen(group.name) &&
        !strncmp(group.name, name, len)) {
      *out_group_id = group.group_id;
      return true;
    }
    if ((strlen(group.alias) > 0) && len == strlen(group.alias) &&
        !strncmp(group.alias, name, len)) {
      *out_group_id = group.group_id;
      return true;
    }
  }
  return false;
}

int ssl_group_id_to_nid(uint16_t group_id) {
  for (const auto &group : kNamedGroups) {
    if (group.group_id == group_id) {
      return group.nid;
    }
  }
  return NID_undef;
}

BSSL_NAMESPACE_END

using namespace bssl;

const char* SSL_get_group_name(uint16_t group_id) {
  for (const auto &group : kNamedGroups) {
    if (group.group_id == group_id) {
      return group.name;
    }
  }
  return nullptr;
}

size_t SSL_get_all_group_names(const char **out, size_t max_out) {
  return GetAllNames(out, max_out, Span<const char *>(), &NamedGroup::name,
                     MakeConstSpan(kNamedGroups));
}
