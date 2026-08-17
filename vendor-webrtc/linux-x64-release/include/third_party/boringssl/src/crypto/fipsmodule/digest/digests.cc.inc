// Copyright 1995-2016 The OpenSSL Project Authors. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#include <openssl/digest.h>

#include <assert.h>
#include <string.h>

#include <openssl/nid.h>

#include "../../internal.h"
#include "../bcm_interface.h"
#include "../delocate.h"
#include "internal.h"

#if defined(NDEBUG)
#define CHECK(x) (void)(x)
#else
#define CHECK(x) assert(x)
#endif


static void sha1_init(EVP_MD_CTX *ctx) {
  BCM_sha1_init(reinterpret_cast<SHA_CTX *>(ctx->md_data));
}

static void sha1_update(EVP_MD_CTX *ctx, const void *data, size_t count) {
  BCM_sha1_update(reinterpret_cast<SHA_CTX *>(ctx->md_data), data, count);
}

static void sha1_final(EVP_MD_CTX *ctx, uint8_t *md) {
  BCM_sha1_final(md, reinterpret_cast<SHA_CTX *>(ctx->md_data));
}

DEFINE_METHOD_FUNCTION(EVP_MD, EVP_sha1) {
  out->type = NID_sha1;
  out->md_size = BCM_SHA_DIGEST_LENGTH;
  out->flags = 0;
  out->init = sha1_init;
  out->update = sha1_update;
  out->final = sha1_final;
  out->block_size = 64;
  out->ctx_size = sizeof(SHA_CTX);
}


static void sha224_init(EVP_MD_CTX *ctx) {
  BCM_sha224_init(reinterpret_cast<SHA256_CTX *>(ctx->md_data));
}

static void sha224_update(EVP_MD_CTX *ctx, const void *data, size_t count) {
  BCM_sha224_update(reinterpret_cast<SHA256_CTX *>(ctx->md_data), data, count);
}

static void sha224_final(EVP_MD_CTX *ctx, uint8_t *md) {
  BCM_sha224_final(md, reinterpret_cast<SHA256_CTX *>(ctx->md_data));
}

DEFINE_METHOD_FUNCTION(EVP_MD, EVP_sha224) {
  out->type = NID_sha224;
  out->md_size = BCM_SHA224_DIGEST_LENGTH;
  out->flags = 0;
  out->init = sha224_init;
  out->update = sha224_update;
  out->final = sha224_final;
  out->block_size = 64;
  out->ctx_size = sizeof(SHA256_CTX);
}


static void sha256_init(EVP_MD_CTX *ctx) {
  BCM_sha256_init(reinterpret_cast<SHA256_CTX *>(ctx->md_data));
}

static void sha256_update(EVP_MD_CTX *ctx, const void *data, size_t count) {
  BCM_sha256_update(reinterpret_cast<SHA256_CTX *>(ctx->md_data), data, count);
}

static void sha256_final(EVP_MD_CTX *ctx, uint8_t *md) {
  BCM_sha256_final(md, reinterpret_cast<SHA256_CTX *>(ctx->md_data));
}

DEFINE_METHOD_FUNCTION(EVP_MD, EVP_sha256) {
  out->type = NID_sha256;
  out->md_size = BCM_SHA256_DIGEST_LENGTH;
  out->flags = 0;
  out->init = sha256_init;
  out->update = sha256_update;
  out->final = sha256_final;
  out->block_size = 64;
  out->ctx_size = sizeof(SHA256_CTX);
}


static void sha384_init(EVP_MD_CTX *ctx) {
  BCM_sha384_init(reinterpret_cast<SHA512_CTX *>(ctx->md_data));
}

static void sha384_update(EVP_MD_CTX *ctx, const void *data, size_t count) {
  BCM_sha384_update(reinterpret_cast<SHA512_CTX *>(ctx->md_data), data, count);
}

static void sha384_final(EVP_MD_CTX *ctx, uint8_t *md) {
  BCM_sha384_final(md, reinterpret_cast<SHA512_CTX *>(ctx->md_data));
}

DEFINE_METHOD_FUNCTION(EVP_MD, EVP_sha384) {
  out->type = NID_sha384;
  out->md_size = BCM_SHA384_DIGEST_LENGTH;
  out->flags = 0;
  out->init = sha384_init;
  out->update = sha384_update;
  out->final = sha384_final;
  out->block_size = 128;
  out->ctx_size = sizeof(SHA512_CTX);
}


static void sha512_init(EVP_MD_CTX *ctx) {
  BCM_sha512_init(reinterpret_cast<SHA512_CTX *>(ctx->md_data));
}

static void sha512_update(EVP_MD_CTX *ctx, const void *data, size_t count) {
  BCM_sha512_update(reinterpret_cast<SHA512_CTX *>(ctx->md_data), data, count);
}

static void sha512_final(EVP_MD_CTX *ctx, uint8_t *md) {
  BCM_sha512_final(md, reinterpret_cast<SHA512_CTX *>(ctx->md_data));
}

DEFINE_METHOD_FUNCTION(EVP_MD, EVP_sha512) {
  out->type = NID_sha512;
  out->md_size = BCM_SHA512_DIGEST_LENGTH;
  out->flags = 0;
  out->init = sha512_init;
  out->update = sha512_update;
  out->final = sha512_final;
  out->block_size = 128;
  out->ctx_size = sizeof(SHA512_CTX);
}


static void sha512_256_init(EVP_MD_CTX *ctx) {
  BCM_sha512_256_init(reinterpret_cast<SHA512_CTX *>(ctx->md_data));
}

static void sha512_256_update(EVP_MD_CTX *ctx, const void *data, size_t count) {
  BCM_sha512_256_update(reinterpret_cast<SHA512_CTX *>(ctx->md_data), data,
                        count);
}

static void sha512_256_final(EVP_MD_CTX *ctx, uint8_t *md) {
  BCM_sha512_256_final(md, reinterpret_cast<SHA512_CTX *>(ctx->md_data));
}

DEFINE_METHOD_FUNCTION(EVP_MD, EVP_sha512_256) {
  out->type = NID_sha512_256;
  out->md_size = BCM_SHA512_256_DIGEST_LENGTH;
  out->flags = 0;
  out->init = sha512_256_init;
  out->update = sha512_256_update;
  out->final = sha512_256_final;
  out->block_size = 128;
  out->ctx_size = sizeof(SHA512_CTX);
}

#undef CHECK
