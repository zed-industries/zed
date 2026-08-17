// Copyright 2015 The Chromium Authors
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

#ifndef BSSL_PKI_TEST_HELPERS_H_
#define BSSL_PKI_TEST_HELPERS_H_

#include <stddef.h>

#include <ostream>
#include <string>
#include <string_view>
#include <vector>

#include <gtest/gtest.h>
#include "input.h"
#include "parsed_certificate.h"
#include "simple_path_builder_delegate.h"
#include "trust_store.h"
#include "verify_certificate_chain.h"

BSSL_NAMESPACE_BEGIN

namespace der {

// This function is used by GTest to support EXPECT_EQ() for der::Input.
void PrintTo(Input data, ::std::ostream *os);

}  // namespace der

// Parses |s| as a DER SEQUENCE TLV and returns a der::Input corresponding to
// the value portion. On error returns an empty der::Input and adds a gtest
// failure.
//
// The returned der::Input() is only valid so long as the input string is alive
// and is not mutated.
der::Input SequenceValueFromString(std::string_view s);

// Helper structure that maps a PEM block header (for instance "CERTIFICATE") to
// the destination where the value for that block should be written.
struct PemBlockMapping {
  // The name of the PEM header. Example "CERTIFICATE".
  const char *block_name;

  // The destination where the read value should be written to.
  std::string *value;

  // True to indicate that the block is not required to be present. If the
  // block is optional and is not present, then |value| will not be modified.
  bool optional = false;
};

// ReadTestDataFromPemFile() is a helper function that reads a PEM test file
// rooted in the "src/" directory.
//
//   * file_path_ascii:
//       The path to the PEM file, relative to src. For instance
//       "testdata/verify_signed_data_unittest/foopy.pem"
//
//   * mappings:
//       An array of length |mappings_length| which maps the expected PEM
//       headers to the destination to write its data.
//
// The function ensures that each of the chosen mappings is satisfied exactly
// once. In other words, the header must be present (unless marked as
// optional=true), have valid data, and appear no more than once.
::testing::AssertionResult ReadTestDataFromPemFile(
    const std::string &file_path_ascii, const PemBlockMapping *mappings,
    size_t mappings_length);

// This is the same as the variant above, however it uses template magic so an
// mappings array can be passed in directly (and the correct length is
// inferred).
template <size_t N>
::testing::AssertionResult ReadTestDataFromPemFile(
    const std::string &file_path_ascii, const PemBlockMapping (&mappings)[N]) {
  return ReadTestDataFromPemFile(file_path_ascii, mappings, N);
}

// Test cases are comprised of all the parameters to certificate
// verification, as well as the expected outputs.
struct VerifyCertChainTest {
  VerifyCertChainTest();
  ~VerifyCertChainTest();

  // The chain of certificates (with the zero-th being the target).
  ParsedCertificateList chain;

  // Details on the trustedness of the last certificate.
  CertificateTrust last_cert_trust;

  // The time to use when verifying the chain.
  der::GeneralizedTime time;

  // The Key Purpose to use when verifying the chain.
  KeyPurpose key_purpose = KeyPurpose::ANY_EKU;

  InitialExplicitPolicy initial_explicit_policy = InitialExplicitPolicy::kFalse;

  std::set<der::Input> user_initial_policy_set;

  InitialPolicyMappingInhibit initial_policy_mapping_inhibit =
      InitialPolicyMappingInhibit::kFalse;

  InitialAnyPolicyInhibit initial_any_policy_inhibit =
      InitialAnyPolicyInhibit::kFalse;

  // The expected errors/warnings from verification (as a string).
  std::string expected_errors;

  // Expected user_constrained_policy_set, as a set of numeric OID strings.
  std::set<std::string> expected_user_constrained_policy_set;

  SimplePathBuilderDelegate::DigestPolicy digest_policy =
      SimplePathBuilderDelegate::DigestPolicy::kWeakAllowSha1;

  // Returns true if |expected_errors| contains any high severity errors (a
  // non-empty expected_errors doesn't necessarily mean verification is
  // expected to fail, as it may have contained warnings).
  bool HasHighSeverityErrors() const;
};

// Reads a test case from |file_path_ascii| (which is relative to //src).
// Generally |file_path_ascii| will start with:
//   net/data/verify_certificate_chain_unittest/
bool ReadVerifyCertChainTestFromFile(const std::string &file_path_ascii,
                                     VerifyCertChainTest *test);

// Reads a certificate chain from |file_path_ascii|
bool ReadCertChainFromFile(const std::string &file_path_ascii,
                           ParsedCertificateList *chain);

// Reads a certificate from |file_path_ascii|. Returns nullptr if the file
// contained more that one certificate.
std::shared_ptr<const ParsedCertificate> ReadCertFromFile(
    const std::string &file_path_ascii);

// Reads a data file relative to the src root directory.
std::string ReadTestFileToString(const std::string &file_path_ascii);

// Asserts that |actual_errors| matches |expected_errors_str|.
//
// This is a helper function to simplify rebasing the error expectations when
// they originate from a test file.
void VerifyCertPathErrors(const std::string &expected_errors_str,
                          const CertPathErrors &actual_errors,
                          const ParsedCertificateList &chain,
                          const std::string &errors_file_path);

// Asserts that |actual_errors| matches |expected_errors_str|.
//
// This is a helper function to simplify rebasing the error expectations when
// they originate from a test file.
void VerifyCertErrors(const std::string &expected_errors_str,
                      const CertErrors &actual_errors,
                      const std::string &errors_file_path);

// Asserts that |actual_user_constrained_policy_set| matches
// |expected_user_constrained_policy_set|.
void VerifyUserConstrainedPolicySet(
    const std::set<std::string> &expected_user_constrained_policy_str_set,
    const std::set<der::Input> &actual_user_constrained_policy_set,
    const std::string &errors_file_path);

BSSL_NAMESPACE_END

#endif  // BSSL_PKI_TEST_HELPERS_H_
