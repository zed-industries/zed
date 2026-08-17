// Copyright 2019 The BoringSSL Authors
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

#ifndef HEADER_MOCK_QUIC_TRANSPORT
#define HEADER_MOCK_QUIC_TRANSPORT

#include <openssl/base.h>
#include <openssl/bio.h>
#include <openssl/ssl.h>

#include <vector>

class MockQuicTransport {
 public:
  explicit MockQuicTransport(bssl::UniquePtr<BIO> bio, SSL *ssl);

  bool SetReadSecret(enum ssl_encryption_level_t level,
                     const SSL_CIPHER *cipher, const uint8_t *secret,
                     size_t secret_len);
  bool SetWriteSecret(enum ssl_encryption_level_t level,
                      const SSL_CIPHER *cipher, const uint8_t *secret,
                      size_t secret_len);

  bool ReadHandshake();
  bool WriteHandshakeData(enum ssl_encryption_level_t level,
                          const uint8_t *data, size_t len);
  // Returns the number of bytes read.
  int ReadApplicationData(uint8_t *out, size_t max_out);
  bool WriteApplicationData(const uint8_t *in, size_t len);
  bool Flush();
  bool SendAlert(enum ssl_encryption_level_t level, uint8_t alert);

 private:
  // Reads a record header from |bio_| and returns whether the record was read
  // successfully. As part of reading the header, this function checks that the
  // cipher suite and secret in the header are correct. On success, the TLS
  // record type is put in |*out_type|, the encryption level is put in
  // |*out_level|, the length of the TLS record is put in |*out_len|, and the
  // next thing to be read from |bio_| is |*out_len| bytes of the TLS record.
  bool ReadHeader(uint8_t *out_type, enum ssl_encryption_level_t *out_level,
                  size_t *out_len);

  // Writes a MockQuicTransport record to |bio_| at encryption level |level|
  // with record type |type| and a TLS record payload of length |len| from
  // |data|.
  bool WriteRecord(enum ssl_encryption_level_t level, uint8_t type,
                   const uint8_t *data, size_t len);

  bssl::UniquePtr<BIO> bio_;

  std::vector<uint8_t> pending_app_data_;
  size_t app_data_offset_;

  struct EncryptionLevel {
    uint16_t cipher;
    std::vector<uint8_t> secret;
  };

  std::vector<EncryptionLevel> read_levels_;
  std::vector<EncryptionLevel> write_levels_;

  SSL *ssl_;  // Unowned.
};


#endif  // HEADER_MOCK_QUIC_TRANSPORT
