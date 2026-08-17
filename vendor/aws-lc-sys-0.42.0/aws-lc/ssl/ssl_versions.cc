// Copyright (c) 2017, Google Inc.
// SPDX-License-Identifier: ISC

#include <openssl/ssl.h>

#include <assert.h>

#include <algorithm>

#include <openssl/bytestring.h>
#include <openssl/err.h>
#include <openssl/span.h>

#include "internal.h"
#include "../crypto/internal.h"


BSSL_NAMESPACE_BEGIN

bool ssl_protocol_version_from_wire(uint16_t *out, uint16_t version) {
  switch (version) {
    case TLS1_VERSION:
    case TLS1_1_VERSION:
    case TLS1_2_VERSION:
    case TLS1_3_VERSION:
      *out = version;
      return true;

    case DTLS1_VERSION:
      // DTLS 1.0 is analogous to TLS 1.1, not TLS 1.0.
      *out = TLS1_1_VERSION;
      return true;

    case DTLS1_2_VERSION:
      *out = TLS1_2_VERSION;
      return true;

    default:
      return false;
  }
}

// The follow arrays are the supported versions for TLS and DTLS, in order of
// decreasing preference.

static const uint16_t kTLSVersions[] = {
    TLS1_3_VERSION,
    TLS1_2_VERSION,
    TLS1_1_VERSION,
    TLS1_VERSION,
};

static const uint16_t kDTLSVersions[] = {
    DTLS1_2_VERSION,
    DTLS1_VERSION,
};

static Span<const uint16_t> get_method_versions(
    const SSL_PROTOCOL_METHOD *method) {
  return method->is_dtls ? Span<const uint16_t>(kDTLSVersions)
                         : Span<const uint16_t>(kTLSVersions);
}

bool ssl_method_supports_version(const SSL_PROTOCOL_METHOD *method,
                                 uint16_t version) {
  for (uint16_t supported : get_method_versions(method)) {
    if (supported == version) {
      return true;
    }
  }
  return false;
}

// The following functions map between API versions and wire versions. The
// public API works on wire versions.

static const char* kUnknownVersion = "unknown";

struct VersionInfo {
  uint16_t version;
  const char *name;
};

static const VersionInfo kVersionNames[] = {
    {TLS1_3_VERSION, "TLSv1.3"},
    {TLS1_2_VERSION, "TLSv1.2"},
    {TLS1_1_VERSION, "TLSv1.1"},
    {TLS1_VERSION, "TLSv1"},
    {DTLS1_VERSION, "DTLSv1"},
    {DTLS1_2_VERSION, "DTLSv1.2"},
};

static const char *ssl_version_to_string(uint16_t version) {
  for (const auto &v : kVersionNames) {
    if (v.version == version) {
      return v.name;
    }
  }
  return kUnknownVersion;
}

static uint16_t wire_version_to_api(uint16_t version) {
  return version;
}

// api_version_to_wire maps |version| to some representative wire version.
static bool api_version_to_wire(uint16_t *out, uint16_t version) {
  // Check it is a real protocol version.
  uint16_t unused;
  if (!ssl_protocol_version_from_wire(&unused, version)) {
    return false;
  }

  *out = version;
  return true;
}

static bool set_version_bound(const SSL_PROTOCOL_METHOD *method, uint16_t *out,
                              uint16_t version) {
  if (!api_version_to_wire(&version, version) ||
      !ssl_method_supports_version(method, version)) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_UNKNOWN_SSL_VERSION);
    return false;
  }

  *out = version;
  return true;
}

static bool set_min_version(const SSL_PROTOCOL_METHOD *method, uint16_t *out,
                            uint16_t version) {
  // Zero is interpreted as the default minimum version.
  if (version == 0) {
    *out = method->is_dtls ? DTLS1_VERSION : TLS1_VERSION;
    return true;
  }

  return set_version_bound(method, out, version);
}

static bool set_max_version(const SSL_PROTOCOL_METHOD *method, uint16_t *out,
                            uint16_t version) {
  // Zero is interpreted as the default maximum version.
  if (version == 0) {
    *out = method->is_dtls ? DTLS1_2_VERSION : TLS1_3_VERSION;
    return true;
  }

  return set_version_bound(method, out, version);
}

const struct {
  uint16_t version;
  uint32_t flag;
} kProtocolVersions[] = {
    {TLS1_VERSION, SSL_OP_NO_TLSv1},
    {TLS1_1_VERSION, SSL_OP_NO_TLSv1_1},
    {TLS1_2_VERSION, SSL_OP_NO_TLSv1_2},
    {TLS1_3_VERSION, SSL_OP_NO_TLSv1_3},
};

bool ssl_get_version_range(const SSL_HANDSHAKE *hs, uint16_t *out_min_version,
                           uint16_t *out_max_version) {
  // For historical reasons, |SSL_OP_NO_DTLSv1| aliases |SSL_OP_NO_TLSv1|, but
  // DTLS 1.0 should be mapped to TLS 1.1.
  uint32_t options = hs->ssl->options;
  if (SSL_is_dtls(hs->ssl)) {
    options &= ~SSL_OP_NO_TLSv1_1;
    if (options & SSL_OP_NO_DTLSv1) {
      options |= SSL_OP_NO_TLSv1_1;
    }
  }

  uint16_t min_version, max_version;
  if (!ssl_protocol_version_from_wire(&min_version,
                                      hs->config->conf_min_version) ||
      !ssl_protocol_version_from_wire(&max_version,
                                      hs->config->conf_max_version)) {
    OPENSSL_PUT_ERROR(SSL, ERR_R_INTERNAL_ERROR);
    return false;
  }

  // QUIC requires TLS 1.3.
  if (hs->ssl->quic_method && min_version < TLS1_3_VERSION) {
    min_version = TLS1_3_VERSION;
  }

  // The |SSL_OP_NO_*| flags disable individual protocols. This has two
  // problems. First, prior to TLS 1.3, the protocol can only express a
  // contiguous range of versions. Second, a library consumer trying to set a
  // maximum version cannot disable protocol versions that get added in a future
  // version of the library.
  //
  // To account for both of these, OpenSSL interprets the client-side bitmask
  // as a min/max range by picking the lowest contiguous non-empty range of
  // enabled protocols. Note that this means it is impossible to set a maximum
  // version of the higest supported TLS version in a future-proof way.
  bool any_enabled = false;
  for (size_t i = 0; i < OPENSSL_ARRAY_SIZE(kProtocolVersions); i++) {
    // Only look at the versions already enabled.
    if (min_version > kProtocolVersions[i].version) {
      continue;
    }
    if (max_version < kProtocolVersions[i].version) {
      break;
    }

    if (!(options & kProtocolVersions[i].flag)) {
      // The minimum version is the first enabled version.
      if (!any_enabled) {
        any_enabled = true;
        min_version = kProtocolVersions[i].version;
      }
      continue;
    }

    // If there is a disabled version after the first enabled one, all versions
    // after it are implicitly disabled.
    if (any_enabled) {
      max_version = kProtocolVersions[i-1].version;
      break;
    }
  }

  if (!any_enabled) {
    OPENSSL_PUT_ERROR(SSL, SSL_R_NO_SUPPORTED_VERSIONS_ENABLED);
    return false;
  }

  *out_min_version = min_version;
  *out_max_version = max_version;
  return true;
}

static uint16_t ssl_version(const SSL *ssl) {
  // In early data, we report the predicted version.
  if (SSL_in_early_data(ssl) && !ssl->server) {
    return ssl->s3->hs->early_session->ssl_version;
  }
  return ssl->version;
}

uint16_t ssl_protocol_version(const SSL *ssl) {
  assert(ssl->s3->have_version);
  uint16_t version;
  if (!ssl_protocol_version_from_wire(&version, ssl->version)) {
    // |ssl->version| will always be set to a valid version.
    assert(0);
    return 0;
  }

  return version;
}

bool ssl_supports_version(const SSL_HANDSHAKE *hs, uint16_t version) {
  const SSL *const ssl = hs->ssl;
  uint16_t protocol_version;
  if (!ssl_method_supports_version(ssl->method, version) ||
      !ssl_protocol_version_from_wire(&protocol_version, version) ||
      hs->min_version > protocol_version ||
      protocol_version > hs->max_version) {
    return false;
  }

  return true;
}

bool ssl_add_supported_versions(const SSL_HANDSHAKE *hs, CBB *cbb,
                                uint16_t extra_min_version) {
  for (uint16_t version : get_method_versions(hs->ssl->method)) {
    uint16_t protocol_version;
    if (ssl_supports_version(hs, version) &&
        ssl_protocol_version_from_wire(&protocol_version, version) &&
        protocol_version >= extra_min_version &&  //
        !CBB_add_u16(cbb, version)) {
      return false;
    }
  }
  return true;
}

bool ssl_negotiate_version(SSL_HANDSHAKE *hs, uint8_t *out_alert,
                           uint16_t *out_version, const CBS *peer_versions) {
  for (uint16_t version : get_method_versions(hs->ssl->method)) {
    if (!ssl_supports_version(hs, version)) {
      continue;
    }

    // JDK 11, prior to 11.0.2, has a buggy TLS 1.3 implementation which fails
    // to send SNI when offering 1.3 sessions. Disable TLS 1.3 for such
    // clients. We apply this logic here rather than |ssl_supports_version| so
    // the downgrade signal continues to query the true capabilities. (The
    // workaround is a limitation of the peer's capabilities rather than our
    // own.)
    //
    // See https://bugs.openjdk.java.net/browse/JDK-8211806.
    if (version == TLS1_3_VERSION && hs->apply_jdk11_workaround) {
      continue;
    }

    CBS copy = *peer_versions;
    while (CBS_len(&copy) != 0) {
      uint16_t peer_version;
      if (!CBS_get_u16(&copy, &peer_version)) {
        OPENSSL_PUT_ERROR(SSL, SSL_R_DECODE_ERROR);
        *out_alert = SSL_AD_DECODE_ERROR;
        return false;
      }

      if (peer_version == version) {
        *out_version = version;
        return true;
      }
    }
  }

  OPENSSL_PUT_ERROR(SSL, SSL_R_UNSUPPORTED_PROTOCOL);
  *out_alert = SSL_AD_PROTOCOL_VERSION;
  return false;
}

BSSL_NAMESPACE_END

using namespace bssl;

int SSL_CTX_set_min_proto_version(SSL_CTX *ctx, uint16_t version) {
  ctx->conf_min_version_use_default = (version == 0);
  return set_min_version(ctx->method, &ctx->conf_min_version, version);
}

int SSL_CTX_set_max_proto_version(SSL_CTX *ctx, uint16_t version) {
  ctx->conf_max_version_use_default = (version == 0);
  return set_max_version(ctx->method, &ctx->conf_max_version, version);
}

uint16_t SSL_CTX_get_min_proto_version(const SSL_CTX *ctx) {
  if (ctx->conf_min_version_use_default) {
    return 0;
  }
  return ctx->conf_min_version;
}

uint16_t SSL_CTX_get_max_proto_version(const SSL_CTX *ctx) {
  if (ctx->conf_max_version_use_default) {
    return 0;
  }
  return ctx->conf_max_version;
}

int SSL_set_min_proto_version(SSL *ssl, uint16_t version) {
  if (!ssl->config) {
    return 0;
  }
  ssl->config->conf_min_version_use_default = (version == 0);
  return set_min_version(ssl->method, &ssl->config->conf_min_version, version);
}

int SSL_set_max_proto_version(SSL *ssl, uint16_t version) {
  if (!ssl->config) {
    return 0;
  }
  ssl->config->conf_max_version_use_default = (version == 0);
  return set_max_version(ssl->method, &ssl->config->conf_max_version, version);
}

uint16_t SSL_get_min_proto_version(const SSL *ssl) {
  if (!ssl->config || ssl->config->conf_min_version_use_default) {
    return 0;
  }
  return ssl->config->conf_min_version;
}

uint16_t SSL_get_max_proto_version(const SSL *ssl) {
  if (!ssl->config || ssl->config->conf_max_version_use_default) {
    return 0;
  }
  return ssl->config->conf_max_version;
}

int SSL_version(const SSL *ssl) {
  return wire_version_to_api(ssl_version(ssl));
}

const char *SSL_get_version(const SSL *ssl) {
  return ssl_version_to_string(ssl_version(ssl));
}

size_t SSL_get_all_version_names(const char **out, size_t max_out) {
  return GetAllNames(out, max_out, MakeConstSpan(&kUnknownVersion, 1),
                     &VersionInfo::name, MakeConstSpan(kVersionNames));
}

const char *SSL_SESSION_get_version(const SSL_SESSION *session) {
  return ssl_version_to_string(session->ssl_version);
}

uint16_t SSL_SESSION_get_protocol_version(const SSL_SESSION *session) {
  return wire_version_to_api(session->ssl_version);
}

int SSL_SESSION_set_protocol_version(SSL_SESSION *session, uint16_t version) {
  // This picks a representative TLS 1.3 version, but this API should only be
  // used on unit test sessions anyway.
  return api_version_to_wire(&session->ssl_version, version);
}

int SSL_CTX_set_record_protocol_version(SSL_CTX *ctx, int version) {
  return version == 0;
}
