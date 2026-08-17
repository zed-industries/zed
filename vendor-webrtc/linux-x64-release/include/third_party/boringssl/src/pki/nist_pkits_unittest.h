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

#ifndef BSSL_PKI_NIST_PKITS_UNITTEST_H_
#define BSSL_PKI_NIST_PKITS_UNITTEST_H_

#include <set>

#include <gtest/gtest.h>
#include "parse_values.h"
#include "test_helpers.h"

BSSL_NAMESPACE_BEGIN

// Describes the inputs and outputs (other than the certificates) for
// the PKITS tests.
struct PkitsTestInfo {
  // Default construction results in the "default settings".
  PkitsTestInfo();
  PkitsTestInfo(const PkitsTestInfo &other);
  ~PkitsTestInfo();

  // Sets |initial_policy_set| to the specified policies. The
  // policies are described as comma-separated symbolic strings like
  // "anyPolicy" and "NIST-test-policy-1".
  //
  // If this isn't called, the default is "anyPolicy".
  void SetInitialPolicySet(const char *const policy_names);

  // Sets |user_constrained_policy_set| to the specified policies. The
  // policies are described as comma-separated symbolic strings like
  // "anyPolicy" and "NIST-test-policy-1".
  //
  // If this isn't called, the default is "NIST-test-policy-1".
  void SetUserConstrainedPolicySet(const char *const policy_names);

  void SetInitialExplicitPolicy(bool b);
  void SetInitialPolicyMappingInhibit(bool b);
  void SetInitialInhibitAnyPolicy(bool b);

  // ----------------
  // Info
  // ----------------

  // The PKITS test number. For example, "4.1.1".
  const char *test_number = nullptr;

  // ----------------
  // Inputs
  // ----------------

  // A set of policy OIDs to use for "initial-policy-set".
  std::set<der::Input> initial_policy_set;

  // The value of "initial-explicit-policy".
  InitialExplicitPolicy initial_explicit_policy = InitialExplicitPolicy::kFalse;

  // The value of "initial-policy-mapping-inhibit".
  InitialPolicyMappingInhibit initial_policy_mapping_inhibit =
      InitialPolicyMappingInhibit::kFalse;

  // The value of "initial-inhibit-any-policy".
  InitialAnyPolicyInhibit initial_inhibit_any_policy =
      InitialAnyPolicyInhibit::kFalse;

  // This is the time when PKITS was published.
  der::GeneralizedTime time = {2011, 4, 15, 0, 0, 0};

  // ----------------
  // Expected outputs
  // ----------------

  // Whether path validation should succeed.
  bool should_validate = false;

  std::set<der::Input> user_constrained_policy_set;
};

// Parameterized test class for PKITS tests.
// The instantiating code should define a PkitsTestDelegate with an appropriate
// static RunTest method, and then INSTANTIATE_TYPED_TEST_SUITE_P for each
// testcase (each TYPED_TEST_SUITE_P in pkits_testcases-inl.h).
template <typename PkitsTestDelegate>
class PkitsTest : public ::testing::Test {
 public:
  template <size_t num_certs, size_t num_crls>
  void RunTest(const char *const (&cert_names)[num_certs],
               const char *const (&crl_names)[num_crls],
               const PkitsTestInfo &info) {
    std::vector<std::string> cert_ders;
    for (const std::string s : cert_names) {
      cert_ders.push_back(bssl::ReadTestFileToString(
          "testdata/nist-pkits/certs/" + s + ".crt"));
    }
    std::vector<std::string> crl_ders;
    for (const std::string s : crl_names) {
      crl_ders.push_back(
          bssl::ReadTestFileToString("testdata/nist-pkits/crls/" + s + ".crl"));
    }

    std::string_view test_number = info.test_number;

    // Some of the PKITS tests are intentionally given different expectations
    // from PKITS.pdf.
    //
    // Empty user_constrained_policy_set due to short-circuit on invalid
    // signatures:
    //
    //   4.1.2 - Invalid CA Signature Test2
    //   4.1.3 - Invalid EE Signature Test3
    //   4.1.6 - Invalid DSA Signature Test6
    //
    // Expected to fail because DSA signatures are not supported:
    //
    //   4.1.4 - Valid DSA Signatures Test4
    //   4.1.5 - Valid DSA Parameter Inheritance Test5
    //
    // Expected to fail because Name constraints on
    // uniformResourceIdentifiers are not supported:
    //
    //   4.13.34 - Valid URI nameConstraints Test34
    //   4.13.36 - Valid URI nameConstraints Test36
    if (test_number == "4.1.2" || test_number == "4.1.3" ||
        test_number == "4.1.6") {
      PkitsTestInfo modified_info = info;
      modified_info.user_constrained_policy_set = {};
      PkitsTestDelegate::RunTest(cert_ders, crl_ders, modified_info);
    } else if (test_number == "4.1.4" || test_number == "4.1.5") {
      PkitsTestInfo modified_info = info;
      modified_info.user_constrained_policy_set = {};
      modified_info.should_validate = false;
      PkitsTestDelegate::RunTest(cert_ders, crl_ders, modified_info);
    } else if (test_number == "4.13.34" || test_number == "4.13.36") {
      PkitsTestInfo modified_info = info;
      modified_info.should_validate = false;
      PkitsTestDelegate::RunTest(cert_ders, crl_ders, modified_info);
    } else {
      PkitsTestDelegate::RunTest(cert_ders, crl_ders, info);
    }
  }
};

// Inline the generated test code:
#include "testdata/nist-pkits/pkits_testcases-inl.h"

BSSL_NAMESPACE_END

#endif  // BSSL_PKI_NIST_PKITS_UNITTEST_H_
