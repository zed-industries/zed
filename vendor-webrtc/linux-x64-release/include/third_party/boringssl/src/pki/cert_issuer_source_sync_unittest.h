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

#ifndef BSSL_PKI_CERT_ISSUER_SOURCE_SYNC_UNITTEST_H_
#define BSSL_PKI_CERT_ISSUER_SOURCE_SYNC_UNITTEST_H_

#include <algorithm>

#include <gtest/gtest.h>
#include <openssl/pool.h>
#include "cert_errors.h"
#include "cert_issuer_source.h"
#include "test_helpers.h"

BSSL_NAMESPACE_BEGIN

namespace {

::testing::AssertionResult ReadTestPem(const std::string &file_name,
                                       const std::string &block_name,
                                       std::string *result) {
  const PemBlockMapping mappings[] = {
      {block_name.c_str(), result},
  };

  return ReadTestDataFromPemFile(file_name, mappings);
}

::testing::AssertionResult ReadTestCert(
    const std::string &file_name,
    std::shared_ptr<const ParsedCertificate> *result) {
  std::string der;
  ::testing::AssertionResult r =
      ReadTestPem("testdata/cert_issuer_source_static_unittest/" + file_name,
                  "CERTIFICATE", &der);
  if (!r) {
    return r;
  }
  CertErrors errors;
  *result = ParsedCertificate::Create(
      bssl::UniquePtr<CRYPTO_BUFFER>(CRYPTO_BUFFER_new(
          reinterpret_cast<const uint8_t *>(der.data()), der.size(), nullptr)),
      {}, &errors);
  if (!*result) {
    return ::testing::AssertionFailure()
           << "ParsedCertificate::Create() failed:\n"
           << errors.ToDebugString();
  }
  return ::testing::AssertionSuccess();
}

}  // namespace

template <typename TestDelegate>
class CertIssuerSourceSyncTest : public ::testing::Test {
 public:
  void SetUp() override {
    ASSERT_TRUE(ReadTestCert("root.pem", &root_));
    ASSERT_TRUE(ReadTestCert("i1_1.pem", &i1_1_));
    ASSERT_TRUE(ReadTestCert("i1_2.pem", &i1_2_));
    ASSERT_TRUE(ReadTestCert("i2.pem", &i2_));
    ASSERT_TRUE(ReadTestCert("i3_1.pem", &i3_1_));
    ASSERT_TRUE(ReadTestCert("i3_2.pem", &i3_2_));
    ASSERT_TRUE(ReadTestCert("c1.pem", &c1_));
    ASSERT_TRUE(ReadTestCert("c2.pem", &c2_));
    ASSERT_TRUE(ReadTestCert("d.pem", &d_));
    ASSERT_TRUE(ReadTestCert("e1.pem", &e1_));
    ASSERT_TRUE(ReadTestCert("e2.pem", &e2_));
  }

  void AddCert(std::shared_ptr<const ParsedCertificate> cert) {
    delegate_.AddCert(std::move(cert));
  }

  void AddAllCerts() {
    AddCert(root_);
    AddCert(i1_1_);
    AddCert(i1_2_);
    AddCert(i2_);
    AddCert(i3_1_);
    AddCert(i3_2_);
    AddCert(c1_);
    AddCert(c2_);
    AddCert(d_);
    AddCert(e1_);
    AddCert(e2_);
  }

  CertIssuerSource &source() { return delegate_.source(); }

 protected:
  bool IssuersMatch(std::shared_ptr<const ParsedCertificate> cert,
                    ParsedCertificateList expected_matches) {
    ParsedCertificateList matches;
    source().SyncGetIssuersOf(cert.get(), &matches);

    std::vector<der::Input> der_result_matches;
    for (const auto &it : matches) {
      der_result_matches.push_back(it->der_cert());
    }
    std::sort(der_result_matches.begin(), der_result_matches.end());

    std::vector<der::Input> der_expected_matches;
    for (const auto &it : expected_matches) {
      der_expected_matches.push_back(it->der_cert());
    }
    std::sort(der_expected_matches.begin(), der_expected_matches.end());

    if (der_expected_matches == der_result_matches) {
      return true;
    }

    // Print some extra information for debugging.
    EXPECT_EQ(der_expected_matches, der_result_matches);
    return false;
  }

  TestDelegate delegate_;
  std::shared_ptr<const ParsedCertificate> root_;
  std::shared_ptr<const ParsedCertificate> i1_1_;
  std::shared_ptr<const ParsedCertificate> i1_2_;
  std::shared_ptr<const ParsedCertificate> i2_;
  std::shared_ptr<const ParsedCertificate> i3_1_;
  std::shared_ptr<const ParsedCertificate> i3_2_;
  std::shared_ptr<const ParsedCertificate> c1_;
  std::shared_ptr<const ParsedCertificate> c2_;
  std::shared_ptr<const ParsedCertificate> d_;
  std::shared_ptr<const ParsedCertificate> e1_;
  std::shared_ptr<const ParsedCertificate> e2_;
};

TYPED_TEST_SUITE_P(CertIssuerSourceSyncTest);

TYPED_TEST_P(CertIssuerSourceSyncTest, NoMatch) {
  this->AddCert(this->root_);

  EXPECT_TRUE(this->IssuersMatch(this->c1_, ParsedCertificateList()));
}

TYPED_TEST_P(CertIssuerSourceSyncTest, OneMatch) {
  this->AddAllCerts();

  EXPECT_TRUE(this->IssuersMatch(this->i1_1_, {this->root_}));
  EXPECT_TRUE(this->IssuersMatch(this->d_, {this->i2_}));
}

TYPED_TEST_P(CertIssuerSourceSyncTest, MultipleMatches) {
  this->AddAllCerts();

  EXPECT_TRUE(this->IssuersMatch(this->e1_, {this->i3_1_, this->i3_2_}));
  EXPECT_TRUE(this->IssuersMatch(this->e2_, {this->i3_1_, this->i3_2_}));
}

// Searching for the issuer of a self-issued cert returns the same cert if it
// happens to be in the CertIssuerSourceStatic.
// Conceptually this makes sense, though probably not very useful in practice.
// Doesn't hurt anything though.
TYPED_TEST_P(CertIssuerSourceSyncTest, SelfIssued) {
  this->AddAllCerts();

  EXPECT_TRUE(this->IssuersMatch(this->root_, {this->root_}));
}

// CertIssuerSourceStatic never returns results asynchronously.
TYPED_TEST_P(CertIssuerSourceSyncTest, IsNotAsync) {
  this->AddCert(this->i1_1_);
  std::unique_ptr<CertIssuerSource::Request> request;
  this->source().AsyncGetIssuersOf(this->c1_.get(), &request);
  EXPECT_EQ(nullptr, request);
}

// These are all the tests that should have the same result with or without
// normalization.
REGISTER_TYPED_TEST_SUITE_P(CertIssuerSourceSyncTest, NoMatch, OneMatch,
                            MultipleMatches, SelfIssued, IsNotAsync);

template <typename TestDelegate>
class CertIssuerSourceSyncNormalizationTest
    : public CertIssuerSourceSyncTest<TestDelegate> {};
TYPED_TEST_SUITE_P(CertIssuerSourceSyncNormalizationTest);

TYPED_TEST_P(CertIssuerSourceSyncNormalizationTest,
             MultipleMatchesAfterNormalization) {
  this->AddAllCerts();

  EXPECT_TRUE(this->IssuersMatch(this->c1_, {this->i1_1_, this->i1_2_}));
  EXPECT_TRUE(this->IssuersMatch(this->c2_, {this->i1_1_, this->i1_2_}));
}

// These tests require (utf8) normalization.
REGISTER_TYPED_TEST_SUITE_P(CertIssuerSourceSyncNormalizationTest,
                            MultipleMatchesAfterNormalization);

template <typename TestDelegate>
class CertIssuerSourceSyncNotNormalizedTest
    : public CertIssuerSourceSyncTest<TestDelegate> {};
TYPED_TEST_SUITE_P(CertIssuerSourceSyncNotNormalizedTest);

TYPED_TEST_P(CertIssuerSourceSyncNotNormalizedTest,
             OneMatchWithoutNormalization) {
  this->AddAllCerts();

  // Without normalization c1 and c2 should at least be able to find their
  // exact matching issuer. (c1 should match i1_1, and c2 should match i1_2.)
  EXPECT_TRUE(this->IssuersMatch(this->c1_, {this->i1_1_}));
  EXPECT_TRUE(this->IssuersMatch(this->c2_, {this->i1_2_}));
}

// These tests are for implementations which do not do utf8 normalization.
REGISTER_TYPED_TEST_SUITE_P(CertIssuerSourceSyncNotNormalizedTest,
                            OneMatchWithoutNormalization);

BSSL_NAMESPACE_END

#endif  // BSSL_PKI_CERT_ISSUER_SOURCE_SYNC_UNITTEST_H_
