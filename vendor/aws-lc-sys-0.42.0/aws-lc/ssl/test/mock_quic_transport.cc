// Copyright (c) 2019, Google Inc.
// SPDX-License-Identifier: ISC

#include "mock_quic_transport.h"

#include <openssl/span.h>

#include <algorithm>
#include <climits>
#include <cstring>

MockQuicTransport::MockQuicTransport(bssl::UniquePtr<BIO> bio, SSL *ssl)
    : bio_(std::move(bio)),
      read_levels_(ssl_encryption_application + 1),
      write_levels_(ssl_encryption_application + 1),
      ssl_(ssl) {}

bool MockQuicTransport::SetReadSecret(enum ssl_encryption_level_t level,
                                      const SSL_CIPHER *cipher,
                                      const uint8_t *secret,
                                      size_t secret_len) {
  // TODO(davidben): Assert the various encryption secret invariants.
  read_levels_[level].cipher = SSL_CIPHER_get_protocol_id(cipher);
  read_levels_[level].secret.assign(secret, secret + secret_len);
  return true;
}

bool MockQuicTransport::SetWriteSecret(enum ssl_encryption_level_t level,
                                       const SSL_CIPHER *cipher,
                                       const uint8_t *secret,
                                       size_t secret_len) {
  // TODO(davidben): Assert the various encryption secret invariants.
  write_levels_[level].cipher = SSL_CIPHER_get_protocol_id(cipher);
  write_levels_[level].secret.assign(secret, secret + secret_len);
  return true;
}

namespace {

bool ReadAll(BIO *bio, bssl::Span<uint8_t> out) {
  size_t len = out.size();
  uint8_t *buf = out.data();
  while (len > 0) {
    size_t chunk_len = std::min(len, size_t{INT_MAX});
    int ret = BIO_read(bio, buf, static_cast<int>(chunk_len));
    if (ret <= 0) {
      return false;
    }
    buf += ret;
    len -= ret;
  }
  return true;
}

const char *LevelToString(ssl_encryption_level_t level) {
  switch (level) {
    case ssl_encryption_initial:
      return "initial";
    case ssl_encryption_early_data:
      return "early_data";
    case ssl_encryption_handshake:
      return "handshake";
    case ssl_encryption_application:
      return "application";
  }
  return "";
}

}  // namespace

bool MockQuicTransport::ReadHeader(uint8_t *out_type,
                                   enum ssl_encryption_level_t *out_level,
                                   size_t *out_len) {
  for (;;) {
    uint8_t header[8];
    if (!ReadAll(bio_.get(), header)) {
      // TODO(davidben): Distinguish between errors and EOF. See
      // ReadApplicationData.
      return false;
    }

    CBS cbs;
    uint8_t level_id;
    uint16_t cipher_suite;
    uint32_t remaining_bytes;
    CBS_init(&cbs, header, sizeof(header));
    if (!CBS_get_u8(&cbs, out_type) ||
        !CBS_get_u8(&cbs, &level_id) ||
        !CBS_get_u16(&cbs, &cipher_suite) ||
        !CBS_get_u32(&cbs, &remaining_bytes) ||
        level_id >= read_levels_.size()) {
      fprintf(stderr, "Error parsing record header.\n");
      return false;
    }

    auto level = static_cast<ssl_encryption_level_t>(level_id);
    // Non-initial levels must be configured before use.
    uint16_t expect_cipher = read_levels_[level].cipher;
    if (expect_cipher == 0 && level != ssl_encryption_initial) {
      if (level == ssl_encryption_early_data) {
        // If we receive early data records without any early data keys, skip
        // the record. This means early data was rejected.
        std::vector<uint8_t> discard(remaining_bytes);
        if (!ReadAll(bio_.get(), bssl::MakeSpan(discard))) {
          return false;
        }
        continue;
      }
      fprintf(stderr,
              "Got record at level %s, but keys were not configured.\n",
              LevelToString(level));
      return false;
    }
    if (cipher_suite != expect_cipher) {
      fprintf(stderr, "Got cipher suite 0x%04x at level %s, wanted 0x%04x.\n",
              cipher_suite, LevelToString(level), expect_cipher);
      return false;
    }
    const std::vector<uint8_t> &secret = read_levels_[level].secret;
    std::vector<uint8_t> read_secret(secret.size());
    if (remaining_bytes < secret.size()) {
      fprintf(stderr, "Record at level %s too small.\n", LevelToString(level));
      return false;
    }
    remaining_bytes -= secret.size();
    if (!ReadAll(bio_.get(), bssl::MakeSpan(read_secret))) {
      fprintf(stderr, "Error reading record secret.\n");
      return false;
    }
    if (read_secret != secret) {
      fprintf(stderr, "Encryption secret at level %s did not match.\n",
              LevelToString(level));
      return false;
    }
    *out_level = level;
    *out_len = remaining_bytes;
    return true;
  }
}

bool MockQuicTransport::ReadHandshake() {
  uint8_t type;
  ssl_encryption_level_t level;
  size_t len;
  if (!ReadHeader(&type, &level, &len)) {
    return false;
  }
  if (type != SSL3_RT_HANDSHAKE) {
    return false;
  }

  std::vector<uint8_t> buf(len);
  if (!ReadAll(bio_.get(), bssl::MakeSpan(buf))) {
    return false;
  }
  return SSL_provide_quic_data(ssl_, level, buf.data(), buf.size());
}

int MockQuicTransport::ReadApplicationData(uint8_t *out, size_t max_out) {
  if (pending_app_data_.size() > 0) {
    size_t len = pending_app_data_.size() - app_data_offset_;
    if (len > max_out) {
      len = max_out;
    }
    memcpy(out, pending_app_data_.data() + app_data_offset_, len);
    app_data_offset_ += len;
    if (app_data_offset_ == pending_app_data_.size()) {
      pending_app_data_.clear();
      app_data_offset_ = 0;
    }
    return len;
  }

  uint8_t type = 0;
  ssl_encryption_level_t level;
  size_t len;
  while (true) {
    if (!ReadHeader(&type, &level, &len)) {
      // Assume that a failure to read the header means there's no more to read,
      // not an error reading.
      return 0;
    }
    if (type == SSL3_RT_APPLICATION_DATA) {
      break;
    }
    if (type != SSL3_RT_HANDSHAKE) {
      return -1;
    }

    std::vector<uint8_t> buf(len);
    if (!ReadAll(bio_.get(), bssl::MakeSpan(buf))) {
      return -1;
    }
    if (SSL_provide_quic_data(ssl_, level, buf.data(), buf.size()) != 1) {
      return -1;
    }
    if (SSL_in_init(ssl_)) {
      int ret = SSL_do_handshake(ssl_);
      if (ret < 0) {
        int ssl_err = SSL_get_error(ssl_, ret);
        if (ssl_err == SSL_ERROR_WANT_READ) {
          continue;
        }
        return -1;
      }
    } else if (SSL_process_quic_post_handshake(ssl_) != 1) {
      return -1;
    }
  }

  uint8_t *buf = out;
  if (len > max_out) {
    pending_app_data_.resize(len);
    buf = pending_app_data_.data();
  }
  app_data_offset_ = 0;
  if (!ReadAll(bio_.get(), bssl::MakeSpan(buf, len))) {
    return -1;
  }
  if (len > max_out) {
    memcpy(out, buf, max_out);
    app_data_offset_ = max_out;
    return max_out;
  }
  return len;
}

bool MockQuicTransport::WriteRecord(enum ssl_encryption_level_t level,
                                    uint8_t type, const uint8_t *data,
                                    size_t len) {
  uint16_t cipher_suite = write_levels_[level].cipher;
  const std::vector<uint8_t> &secret = write_levels_[level].secret;
  size_t tlv_len = secret.size() + len;
  uint8_t header[8];
  header[0] = type;
  header[1] = level;
  header[2] = (cipher_suite >> 8) & 0xff;
  header[3] = cipher_suite & 0xff;
  header[4] = (tlv_len >> 24) & 0xff;
  header[5] = (tlv_len >> 16) & 0xff;
  header[6] = (tlv_len >> 8) & 0xff;
  header[7] = tlv_len & 0xff;
  return BIO_write_all(bio_.get(), header, sizeof(header)) &&
         BIO_write_all(bio_.get(), secret.data(), secret.size()) &&
         BIO_write_all(bio_.get(), data, len);
}

bool MockQuicTransport::WriteHandshakeData(enum ssl_encryption_level_t level,
                                           const uint8_t *data, size_t len) {
  return WriteRecord(level, SSL3_RT_HANDSHAKE, data, len);
}

bool MockQuicTransport::WriteApplicationData(const uint8_t *in, size_t len) {
  enum ssl_encryption_level_t level = ssl_encryption_application;
  if (SSL_in_early_data(ssl_) && !SSL_is_server(ssl_)) {
    level = ssl_encryption_early_data;
  }
  return WriteRecord(level, SSL3_RT_APPLICATION_DATA, in, len);
}

bool MockQuicTransport::Flush() { return BIO_flush(bio_.get()) > 0; }

bool MockQuicTransport::SendAlert(enum ssl_encryption_level_t level,
                                  uint8_t alert) {
  uint8_t alert_msg[] = {2, alert};
  return WriteRecord(level, SSL3_RT_ALERT, alert_msg, sizeof(alert_msg));
}
