// Copyright (c) 2018, Google Inc.
// SPDX-License-Identifier: ISC

#ifndef OPENSSL_HEADER_CRYPTO_TEST_WYCHEPROOF_UTIL_H
#define OPENSSL_HEADER_CRYPTO_TEST_WYCHEPROOF_UTIL_H

#include <openssl/base.h>

#include <string>
#include <vector>


// This header contains convenience functions for Wycheproof tests.

static constexpr const char kWycheproofV1Path[] =
    "third_party/vectors/converted/wycheproof/testvectors_v1/";

class FileTest;

enum class WycheproofRawResult {
  kValid,
  kInvalid,
  kAcceptable,
};

struct WycheproofResult {
  WycheproofRawResult raw_result;
  std::vector<std::string> flags;

  // IsValid returns true if the Wycheproof test should be considered valid. A
  // test result of "acceptable" is treated as valid if all flags are included
  // in |acceptable_flags| and invalid otherwise.
  bool IsValid(const std::vector<std::string> &acceptable_flags = {}) const;

  // StringifyFlags returns a printable string of all flags.
  std::string StringifyFlags();

  // HasFlag returns true if |flag| is present in the flags vector.
  bool HasFlag(const std::string &flag) const;
};

// GetWycheproofResult sets |*out| to the parsed "result" and "flags" keys of |t|.
bool GetWycheproofResult(FileTest *t, WycheproofResult *out);

// GetWycheproofDigest returns a digest function using the Wycheproof name, or
// nullptr on error.
const EVP_MD *GetWycheproofDigest(FileTest *t, const char *key,
                                  bool instruction);

// GetWycheproofCurve returns a curve using the Wycheproof name, or nullptr on
// error.
const EC_GROUP *GetWycheproofCurve(FileTest *t, const char *key,
                                   bool instruction);

// GetWycheproofBIGNUM returns a BIGNUM in the Wycheproof format, or nullptr on
// error.
bssl::UniquePtr<BIGNUM> GetWycheproofBIGNUM(FileTest *t, const char *key,
                                            bool instruction);


#endif  // OPENSSL_HEADER_CRYPTO_TEST_WYCHEPROOF_UTIL_H
