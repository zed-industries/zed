// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_ORIGIN_TRIALS_ORIGIN_TRIAL_POLICY_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_ORIGIN_TRIALS_ORIGIN_TRIAL_POLICY_H_

#include <set>
#include <string_view>
#include <vector>

#include "third_party/blink/public/common/origin_trials/origin_trial_public_key.h"
#include "url/gurl.h"

namespace blink {

// The OriginTrialPolicy provides an interface to the TrialTokenValidator used
// to check for disabled features or tokens.
class OriginTrialPolicy {
 public:
  virtual ~OriginTrialPolicy() = default;

  virtual bool IsOriginTrialsSupported() const { return false; }
  virtual const std::vector<OriginTrialPublicKey>& GetPublicKeys() const = 0;
  virtual bool IsFeatureDisabled(std::string_view feature) const {
    return false;
  }
  virtual bool IsFeatureDisabledForUser(std::string_view feature) const {
    return false;
  }
  virtual bool IsTokenDisabled(std::string_view token_signature) const {
    return false;
  }
  virtual bool IsOriginSecure(const GURL& url) const { return false; }

  virtual const std::set<std::string>* GetDisabledTokensForTesting() const {
    return nullptr;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_ORIGIN_TRIALS_ORIGIN_TRIAL_POLICY_H_
