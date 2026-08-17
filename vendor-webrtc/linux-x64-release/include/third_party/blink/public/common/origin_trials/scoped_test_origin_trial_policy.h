// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_ORIGIN_TRIALS_SCOPED_TEST_ORIGIN_TRIAL_POLICY_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_ORIGIN_TRIALS_SCOPED_TEST_ORIGIN_TRIAL_POLICY_H_

#include <vector>

#include "third_party/blink/public/common/origin_trials/origin_trial_policy.h"

namespace blink {

// This class overrides the `OriginTrialPolicy` in `TrialTokenValidator`
// to use the test public key when the instance is in scope.
class ScopedTestOriginTrialPolicy : public OriginTrialPolicy {
 public:
  ScopedTestOriginTrialPolicy();
  ~ScopedTestOriginTrialPolicy() override;

  bool IsOriginTrialsSupported() const override;
  const std::vector<OriginTrialPublicKey>& GetPublicKeys() const override;
  bool IsOriginSecure(const GURL& url) const override;

 private:
  std::vector<OriginTrialPublicKey> public_keys_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_ORIGIN_TRIALS_SCOPED_TEST_ORIGIN_TRIAL_POLICY_H_
