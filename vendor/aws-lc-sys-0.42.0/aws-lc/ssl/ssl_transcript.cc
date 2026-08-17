// Copyright (C) 1995-1998 Eric Young (eay@cryptsoft.com)
// Copyright (c) 1998-2007 The OpenSSL Project.  All rights reserved.
// Copyright 2005 Nokia. All rights reserved.
//
// The Contribution, originally written by Mika Kousa and Pasi Eronen of
// Nokia Corporation, consists of the "PSK" (Pre-Shared Key) ciphersuites
// support (see RFC 4279) to OpenSSL.
//
// SPDX-License-Identifier: Apache-2.0

#include <openssl/ssl.h>

#include <openssl/buf.h>
#include <openssl/digest.h>
#include <openssl/err.h>

#include "internal.h"


BSSL_NAMESPACE_BEGIN

SSLTranscript::SSLTranscript() {}

SSLTranscript::~SSLTranscript() {}

bool SSLTranscript::Init() {
  buffer_.reset(BUF_MEM_new());
  if (!buffer_) {
    return false;
  }

  hash_.Reset();
  return true;
}

bool SSLTranscript::InitHash(uint16_t version, const SSL_CIPHER *cipher) {
  const EVP_MD *md = ssl_get_handshake_digest(version, cipher);
  if (Digest() == md) {
    // No need to re-hash the buffer.
    return true;
  }
  return EVP_DigestInit_ex(hash_.get(), md, nullptr) &&
         EVP_DigestUpdate(hash_.get(), buffer_->data, buffer_->length);
}

void SSLTranscript::FreeBuffer() {
  buffer_.reset();
}

size_t SSLTranscript::DigestLen() const {
  return EVP_MD_size(Digest());
}

const EVP_MD *SSLTranscript::Digest() const {
  return EVP_MD_CTX_md(hash_.get());
}

bool SSLTranscript::UpdateForHelloRetryRequest() {
  if (buffer_) {
    buffer_->length = 0;
  }

  uint8_t old_hash[EVP_MAX_MD_SIZE];
  size_t hash_len;
  if (!GetHash(old_hash, &hash_len)) {
    return false;
  }
  const uint8_t header[4] = {SSL3_MT_MESSAGE_HASH, 0, 0,
                             static_cast<uint8_t>(hash_len)};
  if (!EVP_DigestInit_ex(hash_.get(), Digest(), nullptr) ||
      !Update(header) ||
      !Update(MakeConstSpan(old_hash, hash_len))) {
    return false;
  }
  return true;
}

bool SSLTranscript::CopyToHashContext(EVP_MD_CTX *ctx,
                                      const EVP_MD *digest) const {
  const EVP_MD *transcript_digest = Digest();
  if (transcript_digest != nullptr &&
      EVP_MD_type(transcript_digest) == EVP_MD_type(digest)) {
    return EVP_MD_CTX_copy_ex(ctx, hash_.get());
  }

  if (buffer_) {
    return EVP_DigestInit_ex(ctx, digest, nullptr) &&
           EVP_DigestUpdate(ctx, buffer_->data, buffer_->length);
  }

  OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
  return false;
}

bool SSLTranscript::Update(Span<const uint8_t> in) {
  // Depending on the state of the handshake, either the handshake buffer may be
  // active, the rolling hash, or both.
  if (buffer_ &&
      !BUF_MEM_append(buffer_.get(), in.data(), in.size())) {
    return false;
  }

  if (EVP_MD_CTX_md(hash_.get()) != NULL) {
    EVP_DigestUpdate(hash_.get(), in.data(), in.size());
  }

  return true;
}

bool SSLTranscript::GetHash(uint8_t *out, size_t *out_len) const {
  ScopedEVP_MD_CTX ctx;
  unsigned len;
  if (!EVP_MD_CTX_copy_ex(ctx.get(), hash_.get()) ||
      !EVP_DigestFinal_ex(ctx.get(), out, &len)) {
    return false;
  }
  *out_len = len;
  return true;
}

bool SSLTranscript::GetFinishedMAC(uint8_t *out, size_t *out_len,
                                   const SSL_SESSION *session,
                                   bool from_server) const {
  static const char kClientLabel[] = "client finished";
  static const char kServerLabel[] = "server finished";
  auto label = from_server
                   ? MakeConstSpan(kServerLabel, sizeof(kServerLabel) - 1)
                   : MakeConstSpan(kClientLabel, sizeof(kClientLabel) - 1);

  uint8_t digest[EVP_MAX_MD_SIZE];
  size_t digest_len;
  if (!GetHash(digest, &digest_len)) {
    return false;
  }

  static const size_t kFinishedLen = 12;
  if (!tls1_prf(Digest(), MakeSpan(out, kFinishedLen),
                MakeConstSpan(session->secret, session->secret_length), label,
                MakeConstSpan(digest, digest_len), {})) {
    return false;
  }

  *out_len = kFinishedLen;
  return true;
}

BSSL_NAMESPACE_END
