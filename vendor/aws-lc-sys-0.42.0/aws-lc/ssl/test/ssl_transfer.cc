/* Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0 OR ISC
 */

#include "ssl_transfer.h"

#include <stdio.h>
#include <time.h>

#include <openssl/ssl.h>

#include "test_config.h"
#include "../internal.h"

SSLTransfer::SSLTransfer() {}

// WriteData writes |input| to disk when |prefix| is not empty.
// This is to generate data to seed related Fuzz corpus generation.
static bool WriteData(std::string prefix, const uint8_t *input, size_t len) {
  if (prefix.empty()) {
    // WriteData is not needed because related config is not enabled.
    return true;
  }

  struct FileCloser {
    void operator()(FILE *f) const { fclose(f); }
  };

  using ScopedFILE = std::unique_ptr<FILE, FileCloser>;
  std::string path = prefix + "-" + std::to_string(rand());
  ScopedFILE file(fopen(path.c_str(), "w"));
  if (!file) {
    return false;
  }

  return fwrite(input, len, 1, file.get()) == 1;
}

// EncodeAndDecodeSSL  transfers the states of |in| to a newly allocated SSL
// by using |SSL_to/from_bytes|. When success, |in| is freed and |out| holds
// the transferred SSL.
static bool EncodeAndDecodeSSL(const TestConfig *config, SSL *in, SSL_CTX *ctx,
  bssl::UniquePtr<SSL> *out) {
  // Encoding SSL to bytes.
  size_t encoded_len;
  bssl::UniquePtr<uint8_t> encoded;
  uint8_t *encoded_raw;
  if (!SSL_to_bytes(in, &encoded_raw, &encoded_len)) {
    fprintf(stderr, "SSL_to_bytes failed. Error code: %s\n",
      ERR_reason_error_string(ERR_peek_last_error()));
    return false;
  }
  encoded.reset(encoded_raw);
  // Decoding SSL from the bytes.
  std::string path_prefix = config->ssl_fuzz_seed_path_prefix;
  if (!WriteData(path_prefix, encoded.get(), encoded_len)) {
    fprintf(stderr, "Failed to write the output of |SSL_to_bytes|.\n");
    return false;
  }
  const uint8_t *ptr2 = encoded.get();
  SSL *server2_ = SSL_from_bytes(ptr2, encoded_len, ctx);
  if (server2_ == nullptr) {
    fprintf(stderr, "SSL_from_bytes failed. Error code: %s\n", ERR_reason_error_string(ERR_peek_last_error()));
    return false;
  }
  out->reset(server2_);
  return true;
}

// MoveBIOs moves the |BIO|s of |src| to |dst|.
static void MoveBIOs(SSL *dest, SSL *src) {
  BIO *rbio = SSL_get_rbio(src);
  BIO_up_ref(rbio);
  SSL_set0_rbio(dest, rbio);

  BIO *wbio = SSL_get_wbio(src);
  BIO_up_ref(wbio);
  SSL_set0_wbio(dest, wbio);

  SSL_set0_rbio(src, nullptr);
  SSL_set0_wbio(src, nullptr);
}

// TransferSSL transfers |in| to |out|.
static bool TransferSSL(const TestConfig *config, bssl::UniquePtr<SSL> *in, bssl::UniquePtr<SSL> *out) {
  if (!in || !in->get()) {
    return false;
  }
  SSL_CTX *in_ctx = SSL_get_SSL_CTX(in->get());
  // Encode the SSL |in| into bytes.
  // Decode the bytes into a new SSL.
  bssl::UniquePtr<SSL> decoded_ssl;
  if (!EncodeAndDecodeSSL(config, in->get(), in_ctx, &decoded_ssl)){
    return false;
  }
  // Move the bio.
  MoveBIOs(decoded_ssl.get(), in->get());
  if (!SetTestConfig(decoded_ssl.get(), GetTestConfig(in->get()))) {
    return false;
  }
  // Move the test state.
  std::unique_ptr<TestState> state(GetTestState(in->get()));
  if (!SetTestState(decoded_ssl.get(), std::move(state))) {
    return false;
  }
  // Unset the test state of |in|.
  std::unique_ptr<TestState> tmp1;
  if (!SetTestState(in->get(), std::move(tmp1)) || !SetTestConfig(in->get(), nullptr)) {
    return false;
  }
  // Free the SSL of |in|.
  SSL_free(in->release());
  // If |out| is not nullptr, |out| will hold the decoded SSL.
  // Else, |in| will get reset to hold the decoded SSL.
  if (out == nullptr) {
    in->reset(decoded_ssl.release());
  } else {
    out->reset(decoded_ssl.release());
  }
  return true;
}

void SSLTransfer::MarkTest(const TestConfig *config, const SSL *ssl) {
  if (config->check_ssl_transfer && IsSupported(ssl)) {
    // Below message is to inform runner.go that this test case can
    // be converted to test SSL transfer.
    // In the converted test, |IsSupported| should be called again
    // before |TransferSSL| because each test case may perform
    // multiple connections. Not all connections can be transferred.
    fprintf(stderr, "Eligible for testing SSL transfer.\n");
  }
}

bool SSLTransfer::ResetSSL(const TestConfig *config, bssl::UniquePtr<SSL> *in) {
  if (config->do_ssl_transfer && IsSupported(in->get())) {
    // Below message is to inform runner.go that this test case 
    // is going to test SSL transfer.
    fprintf(stderr, "SSL transfer is going to be tested.\n");
    if (!TransferSSL(config, in, nullptr)) {
      return false;
    }
  }
  return true;
}

// IsSupported is wrapper of |ssl_transfer_supported| and includes
// some logics to clean error code that may get generated when not supported.
bool SSLTransfer::IsSupported(const SSL *in) {
  // |ssl_transfer_supported| may generate new error code.
  // |ERR_set_mark| and |ERR_pop_to_mark| are used to clean the error states.
  ERR_set_mark();
  bool ret = true;
  if (!bssl::ssl_transfer_supported(in)) {
    ret = false;
    ERR_pop_to_mark();
  }
  return ret;
}

bool SSLTransfer::MarkOrReset(const TestConfig *config, bssl::UniquePtr<SSL> *in) {
  if (!config || !in) {
    return false;
  }
  bool supported = IsSupported(in->get());
  if (!supported) {
    // Return true because not all ssl connections can be transferred.
    return true;
  }
  MarkTest(config, in->get());
  return ResetSSL(config, in);
}
