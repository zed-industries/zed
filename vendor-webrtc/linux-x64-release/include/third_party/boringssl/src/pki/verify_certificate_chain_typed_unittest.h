// Copyright 2016 The Chromium Authors
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

#ifndef BSSL_PKI_VERIFY_CERTIFICATE_CHAIN_TYPED_UNITTEST_H_
#define BSSL_PKI_VERIFY_CERTIFICATE_CHAIN_TYPED_UNITTEST_H_

#include <gtest/gtest.h>
#include "input.h"
#include "parsed_certificate.h"
#include "simple_path_builder_delegate.h"
#include "test_helpers.h"
#include "trust_store.h"
#include "verify_certificate_chain.h"

BSSL_NAMESPACE_BEGIN

template <typename TestDelegate>
class VerifyCertificateChainTest : public ::testing::Test {
 public:
  void RunTest(const char *file_name) {
    VerifyCertChainTest test;

    std::string path =
        std::string("testdata/verify_certificate_chain_unittest/") + file_name;

    SCOPED_TRACE("Test file: " + path);

    if (!ReadVerifyCertChainTestFromFile(path, &test)) {
      ADD_FAILURE() << "Couldn't load test case: " << path;
      return;
    }

    TestDelegate::Verify(test, path);
  }
};

// Tests that have only one root. These can be tested without requiring any
// path-building ability.
template <typename TestDelegate>
class VerifyCertificateChainSingleRootTest
    : public VerifyCertificateChainTest<TestDelegate> {};

TYPED_TEST_SUITE_P(VerifyCertificateChainSingleRootTest);

TYPED_TEST_P(VerifyCertificateChainSingleRootTest, Simple) {
  this->RunTest("target-and-intermediate/main.test");
  this->RunTest("target-and-intermediate/ta-with-expiration.test");
  this->RunTest("target-and-intermediate/ta-with-constraints.test");
  this->RunTest("target-and-intermediate/trusted_leaf-and-trust_anchor.test");
}

TYPED_TEST_P(VerifyCertificateChainSingleRootTest, BasicConstraintsCa) {
  this->RunTest("intermediate-lacks-basic-constraints/main.test");
  this->RunTest("intermediate-basic-constraints-ca-false/main.test");
  this->RunTest("intermediate-basic-constraints-not-critical/main.test");
  this->RunTest("root-lacks-basic-constraints/main.test");
  this->RunTest("root-lacks-basic-constraints/ta-with-constraints.test");
  this->RunTest(
      "root-lacks-basic-constraints/ta-with-require-basic-constraints.test");
  this->RunTest(
      "root-lacks-basic-constraints/"
      "ta-with-constraints-require-basic-constraints.test");
  this->RunTest("root-basic-constraints-ca-false/main.test");
  this->RunTest("root-basic-constraints-ca-false/ta-with-constraints.test");

  this->RunTest("target-has-ca-basic-constraints/main.test");
  this->RunTest("target-has-ca-basic-constraints/strict.test");
  this->RunTest(
      "target-has-ca-basic-constraints/target_only-trusted_leaf.test");
  this->RunTest(
      "target-has-ca-basic-constraints/target_only-trusted_leaf-strict.test");
}

TYPED_TEST_P(VerifyCertificateChainSingleRootTest, BasicConstraintsPathlen) {
  this->RunTest("violates-basic-constraints-pathlen-0/main.test");
  this->RunTest("basic-constraints-pathlen-0-self-issued/main.test");
  this->RunTest("target-has-pathlen-but-not-ca/main.test");
  this->RunTest("violates-pathlen-1-from-root/main.test");
  this->RunTest("violates-pathlen-1-from-root/ta-with-constraints.test");
}

TYPED_TEST_P(VerifyCertificateChainSingleRootTest, UnknownExtension) {
  this->RunTest("intermediate-unknown-critical-extension/main.test");
  this->RunTest("intermediate-unknown-non-critical-extension/main.test");
  this->RunTest("target-unknown-critical-extension/main.test");
  this->RunTest(
      "target-unknown-critical-extension/target_only-trusted_leaf.test");
}

TYPED_TEST_P(VerifyCertificateChainSingleRootTest, MSApplicationPolicies) {
  this->RunTest("target-msapplicationpolicies-no-eku/main.test");
  this->RunTest("target-msapplicationpolicies-and-eku/main.test");
}

TYPED_TEST_P(VerifyCertificateChainSingleRootTest, WeakSignature) {
  this->RunTest("target-signed-with-sha1/main.test");
  this->RunTest("intermediate-signed-with-sha1/main.test");
}

TYPED_TEST_P(VerifyCertificateChainSingleRootTest, WrongSignature) {
  this->RunTest("target-wrong-signature/main.test");
  this->RunTest("intermediate-and-target-wrong-signature/main.test");
  this->RunTest("incorrect-trust-anchor/main.test");
  this->RunTest("target-wrong-signature-no-authority-key-identifier/main.test");
  this->RunTest(
      "intermediate-wrong-signature-no-authority-key-identifier/main.test");
}

TYPED_TEST_P(VerifyCertificateChainSingleRootTest, LastCertificateNotTrusted) {
  this->RunTest("target-and-intermediate/distrusted-root.test");
  this->RunTest("target-and-intermediate/distrusted-root-expired.test");
  this->RunTest("target-and-intermediate/unspecified-trust-root.test");
  this->RunTest("target-and-intermediate/trusted_leaf-root.test");
}

TYPED_TEST_P(VerifyCertificateChainSingleRootTest, WeakPublicKey) {
  this->RunTest("target-signed-by-512bit-rsa/main.test");
  this->RunTest("target-has-512bit-rsa-key/main.test");
}

TYPED_TEST_P(VerifyCertificateChainSingleRootTest, InvalidPublicKey) {
  this->RunTest("intermediate-invalid-spki/main.test");
}

TYPED_TEST_P(VerifyCertificateChainSingleRootTest, TargetSignedUsingEcdsa) {
  this->RunTest("target-signed-using-ecdsa/main.test");
}

TYPED_TEST_P(VerifyCertificateChainSingleRootTest, Expired) {
  this->RunTest("expired-target/not-before.test");
  this->RunTest("expired-target/not-after.test");
  this->RunTest("expired-intermediate/not-before.test");
  this->RunTest("expired-intermediate/not-after.test");
  this->RunTest("expired-root/not-before.test");
  this->RunTest("expired-root/not-before-ta-with-expiration.test");
  this->RunTest("expired-root/not-after.test");
  this->RunTest("expired-root/not-after-ta-with-expiration.test");
  this->RunTest("expired-root/not-after-ta-with-constraints.test");
  this->RunTest(
      "expired-root/not-after-ta-with-expiration-and-constraints.test");
}

TYPED_TEST_P(VerifyCertificateChainSingleRootTest, TargetNotEndEntity) {
  this->RunTest("target-not-end-entity/main.test");
  this->RunTest("target-not-end-entity/strict.test");
  this->RunTest("target-not-end-entity/strict-leaf.test");
}

TYPED_TEST_P(VerifyCertificateChainSingleRootTest, KeyUsage) {
  this->RunTest("intermediate-lacks-signing-key-usage/main.test");
  this->RunTest("target-has-keycertsign-but-not-ca/main.test");

  this->RunTest("target-serverauth-various-keyusages/rsa-decipherOnly.test");
  this->RunTest(
      "target-serverauth-various-keyusages/rsa-digitalSignature.test");
  this->RunTest("target-serverauth-various-keyusages/rsa-keyAgreement.test");
  this->RunTest("target-serverauth-various-keyusages/rsa-keyEncipherment.test");

  this->RunTest("target-serverauth-various-keyusages/ec-decipherOnly.test");
  this->RunTest("target-serverauth-various-keyusages/ec-digitalSignature.test");
  this->RunTest("target-serverauth-various-keyusages/ec-keyAgreement.test");
  this->RunTest("target-serverauth-various-keyusages/ec-keyEncipherment.test");

  this->RunTest("root-lacks-keycertsign-key-usage/main.test");
  this->RunTest("root-lacks-keycertsign-key-usage/ta-with-constraints.test");
}

TYPED_TEST_P(VerifyCertificateChainSingleRootTest, ExtendedKeyUsage) {
  this->RunTest("intermediate-eku-clientauth/any.test");
  this->RunTest("intermediate-eku-clientauth/serverauth.test");
  this->RunTest("intermediate-eku-clientauth/clientauth.test");
  this->RunTest("intermediate-eku-clientauth/serverauth-strict.test");
  this->RunTest("intermediate-eku-clientauth/serverauth-strict-leaf.test");
  this->RunTest("intermediate-eku-clientauth/clientauth-strict.test");
  this->RunTest("intermediate-eku-clientauth/clientauth-strict-leaf.test");
  this->RunTest("intermediate-eku-mlsclientauth/any.test");
  this->RunTest("intermediate-eku-mlsclientauth/serverauth.test");
  this->RunTest("intermediate-eku-mlsclientauth/clientauth.test");
  this->RunTest("intermediate-eku-mlsclientauth/mlsclientauth.test");
  this->RunTest("intermediate-eku-mlsclientauth-extra/any.test");
  this->RunTest("intermediate-eku-mlsclientauth-extra/mlsclientauth.test");
  this->RunTest("intermediate-eku-mlsclientauth-extra/any.test");
  this->RunTest("intermediate-eku-mlsclientauth-extra/mlsclientauth.test");
  this->RunTest("intermediate-eku-c2patimestamping/any.test");
  this->RunTest("intermediate-eku-c2patimestamping/serverauth.test");
  this->RunTest("intermediate-eku-c2patimestamping/clientauth.test");
  this->RunTest("intermediate-eku-c2patimestamping/c2patimestamp.test");
  this->RunTest("intermediate-eku-c2patimestamping/c2pamanifest.test");
  this->RunTest("intermediate-eku-c2pamanifest/any.test");
  this->RunTest("intermediate-eku-c2pamanifest/serverauth.test");
  this->RunTest("intermediate-eku-c2pamanifest/clientauth.test");
  this->RunTest("intermediate-eku-c2pamanifest/c2patimestamp.test");
  this->RunTest("intermediate-eku-c2pamanifest/c2pamanifest.test");
  this->RunTest("intermediate-eku-any-and-clientauth/any.test");
  this->RunTest("intermediate-eku-any-and-clientauth/serverauth.test");
  this->RunTest("intermediate-eku-any-and-clientauth/serverauth-strict.test");
  this->RunTest("intermediate-eku-any-and-clientauth/serverauth-strict-leaf.test");
  this->RunTest("intermediate-eku-any-and-clientauth/clientauth.test");
  this->RunTest("intermediate-eku-any-and-clientauth/clientauth-strict.test");
  this->RunTest("intermediate-eku-any-and-clientauth/clientauth-strict-leaf.test");
  this->RunTest("target-eku-clientauth/any.test");
  this->RunTest("target-eku-clientauth/serverauth.test");
  this->RunTest("target-eku-clientauth/clientauth.test");
  this->RunTest("target-eku-clientauth/serverauth-strict.test");
  this->RunTest("target-eku-clientauth/clientauth-strict.test");
  this->RunTest("target-eku-any/any.test");
  this->RunTest("target-eku-any/serverauth.test");
  this->RunTest("target-eku-any/serverauth-strict-leaf.test");
  this->RunTest("target-eku-any/clientauth.test");
  this->RunTest("target-eku-any/serverauth-strict.test");
  this->RunTest("target-eku-any/clientauth-strict.test");
  this->RunTest("target-eku-any/clientauth-strict-leaf.test");
  this->RunTest("target-eku-any/mlsclientauth.test");
  this->RunTest("target-eku-many/any.test");
  this->RunTest("target-eku-many/serverauth.test");
  this->RunTest("target-eku-many/clientauth.test");
  this->RunTest("target-eku-many/serverauth-strict.test");
  this->RunTest("target-eku-many/serverauth-strict-leaf.test");
  this->RunTest("target-eku-many/clientauth-strict.test");
  this->RunTest("target-eku-many/clientauth-strict-leaf.test");
  this->RunTest("target-eku-many/mlsclientauth.test");
  this->RunTest("target-eku-none/any.test");
  this->RunTest("target-eku-none/serverauth.test");
  this->RunTest("target-eku-none/clientauth.test");
  this->RunTest("target-eku-none/serverauth-strict.test");
  this->RunTest("target-eku-none/clientauth-strict.test");
  this->RunTest("target-eku-none/clientauth-strict-leaf.test");
  this->RunTest("target-eku-none/mlsclientauth.test");
  this->RunTest("root-eku-clientauth/serverauth.test");
  this->RunTest("root-eku-clientauth/serverauth-strict.test");
  this->RunTest("root-eku-clientauth/serverauth-ta-with-constraints.test");
  this->RunTest("root-eku-clientauth/serverauth-ta-with-expiration.test");
  this->RunTest(
      "root-eku-clientauth/serverauth-ta-with-expiration-and-constraints.test");
  this->RunTest(
      "root-eku-clientauth/serverauth-ta-with-constraints-strict.test");
  this->RunTest("intermediate-eku-server-gated-crypto/sha1-eku-any.test");
  this->RunTest(
      "intermediate-eku-server-gated-crypto/sha1-eku-clientAuth.test");
  this->RunTest(
      "intermediate-eku-server-gated-crypto/sha1-eku-clientAuth-strict.test");
  this->RunTest(
      "intermediate-eku-server-gated-crypto/sha1-eku-serverAuth.test");
  this->RunTest(
      "intermediate-eku-server-gated-crypto/sha1-eku-serverAuth-strict.test");
  this->RunTest("intermediate-eku-server-gated-crypto/sha256-eku-any.test");
  this->RunTest(
      "intermediate-eku-server-gated-crypto/sha256-eku-clientAuth-strict.test");
  this->RunTest(
      "intermediate-eku-server-gated-crypto/sha256-eku-serverAuth.test");
  this->RunTest(
      "intermediate-eku-server-gated-crypto/sha256-eku-serverAuth-strict.test");
}

TYPED_TEST_P(VerifyCertificateChainSingleRootTest,
             IssuerAndSubjectNotByteForByteEqual) {
  this->RunTest("issuer-and-subject-not-byte-for-byte-equal/target.test");
  this->RunTest("issuer-and-subject-not-byte-for-byte-equal/anchor.test");
}

TYPED_TEST_P(VerifyCertificateChainSingleRootTest, TrustAnchorNotSelfSigned) {
  this->RunTest("non-self-signed-root/main.test");
  this->RunTest("non-self-signed-root/ta-with-constraints.test");
}

TYPED_TEST_P(VerifyCertificateChainSingleRootTest, KeyRollover) {
  this->RunTest("key-rollover/oldchain.test");
  this->RunTest("key-rollover/rolloverchain.test");
  this->RunTest("key-rollover/longrolloverchain.test");
  this->RunTest("key-rollover/newchain.test");
}

// Test coverage of policies comes primarily from the PKITS tests. The
// tests here only cover aspects not already tested by PKITS.
TYPED_TEST_P(VerifyCertificateChainSingleRootTest, Policies) {
  this->RunTest("unknown-critical-policy-qualifier/main.test");
  this->RunTest("unknown-non-critical-policy-qualifier/main.test");

  this->RunTest("policies-ok/main.test");
  this->RunTest("policies-ok/ta-with-constraints.test");

  this->RunTest("policies-on-root-ok/main.test");
  this->RunTest("policies-on-root-ok/ta-with-constraints.test");
  this->RunTest("policies-on-root-wrong/main.test");
  this->RunTest("policies-on-root-wrong/ta-with-constraints.test");

  this->RunTest("policies-required-by-root-ok/main.test");
  this->RunTest("policies-required-by-root-ok/ta-with-constraints.test");
  this->RunTest("policies-required-by-root-fail/main.test");
  this->RunTest("policies-required-by-root-fail/ta-with-constraints.test");

  this->RunTest("policies-inhibit-mapping-by-root-ok/main.test");
  this->RunTest("policies-inhibit-mapping-by-root-ok/ta-with-constraints.test");
  this->RunTest("policies-inhibit-mapping-by-root-fail/main.test");
  this->RunTest(
      "policies-inhibit-mapping-by-root-fail/ta-with-constraints.test");

  this->RunTest("policy-mappings-on-root-ok/main.test");
  this->RunTest("policy-mappings-on-root-ok/ta-with-constraints.test");
  this->RunTest("policy-mappings-on-root-fail/main.test");
  this->RunTest("policy-mappings-on-root-fail/ta-with-constraints.test");

  this->RunTest("policies-inhibit-anypolicy-by-root-ok/main.test");
  this->RunTest(
      "policies-inhibit-anypolicy-by-root-ok/ta-with-constraints.test");
  this->RunTest("policies-inhibit-anypolicy-by-root-fail/main.test");
  this->RunTest(
      "policies-inhibit-anypolicy-by-root-fail/ta-with-constraints.test");
}

TYPED_TEST_P(VerifyCertificateChainSingleRootTest, ManyNames) {
  // TODO(bbe) fix this to run these with correct numbers.
#if 0
  this->RunTest("many-names/ok-all-types.test");
  this->RunTest("many-names/toomany-all-types.test");
  this->RunTest("many-names/toomany-dns-excluded.test");
  this->RunTest("many-names/toomany-dns-permitted.test");
  this->RunTest("many-names/toomany-ips-excluded.test");
  this->RunTest("many-names/toomany-ips-permitted.test");
  this->RunTest("many-names/toomany-dirnames-excluded.test");
  this->RunTest("many-names/toomany-dirnames-permitted.test");
#endif
}

TYPED_TEST_P(VerifyCertificateChainSingleRootTest, TargetOnly) {
  this->RunTest("target-only/trusted_anchor.test");
  this->RunTest("target-only/trusted_leaf-and-trust_anchor.test");
  this->RunTest("target-only/trusted_leaf.test");
  this->RunTest("target-only/trusted_leaf_require_self_signed.test");
  this->RunTest("target-only/trusted_leaf-not_after.test");
  this->RunTest("target-only/trusted_leaf-wrong_eku.test");

  this->RunTest("target-selfissued/trusted_anchor.test");
  this->RunTest("target-selfissued/trusted_leaf-and-trust_anchor.test");
  this->RunTest("target-selfissued/trusted_leaf.test");
  this->RunTest("target-selfissued/trusted_leaf_require_self_signed.test");
}

TYPED_TEST_P(VerifyCertificateChainSingleRootTest, TargetSelfSigned) {
  // Note that there is not a test here of target-selfsigned with
  // TRUSTED_ANCHOR, since it will have different results when run under
  // verify_certificate_chain_unittest.cc and
  // path_builder_verify_certificate_chain_unittest.cc

  this->RunTest("target-selfsigned/trusted_leaf-and-trust_anchor.test");
  this->RunTest("target-selfsigned/trusted_leaf.test");
  this->RunTest("target-selfsigned/trusted_leaf_require_self_signed.test");
  this->RunTest("target-selfsigned/trusted_leaf-not_after.test");
  this->RunTest("target-selfsigned/trusted_leaf-wrong_eku.test");
}

// TODO(eroman): Add test that invalid validity dates where the day or month
// ordinal not in range, like "March 39, 2016" are rejected.

REGISTER_TYPED_TEST_SUITE_P(VerifyCertificateChainSingleRootTest, Simple,
                            BasicConstraintsCa, BasicConstraintsPathlen,
                            UnknownExtension, MSApplicationPolicies,
                            WeakSignature, WrongSignature,
                            LastCertificateNotTrusted, WeakPublicKey,
                            InvalidPublicKey, TargetSignedUsingEcdsa, Expired,
                            TargetNotEndEntity, KeyUsage, ExtendedKeyUsage,
                            IssuerAndSubjectNotByteForByteEqual,
                            TrustAnchorNotSelfSigned, KeyRollover, Policies,
                            ManyNames, TargetOnly, TargetSelfSigned);

BSSL_NAMESPACE_END

#endif  // BSSL_PKI_VERIFY_CERTIFICATE_CHAIN_TYPED_UNITTEST_H_
