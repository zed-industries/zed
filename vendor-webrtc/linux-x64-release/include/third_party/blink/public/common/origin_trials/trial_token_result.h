// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_ORIGIN_TRIALS_TRIAL_TOKEN_RESULT_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_ORIGIN_TRIALS_TRIAL_TOKEN_RESULT_H_

#include <memory>
#include "third_party/blink/public/common/common_export.h"

namespace blink {

class TrialToken;
enum class OriginTrialTokenStatus;

// TrialTokenResult can have following states:
// - Valid Token:       Status() == kSuccess, ParsedToken() != nullptr
// - Unparsable Token:  Status() != kSuccess, ParsedToken() == nullptr
// - Invalid Token:     Status() != kSuccess, ParsedToken() != nullptr
class BLINK_COMMON_EXPORT TrialTokenResult {
 public:
  TrialTokenResult(TrialTokenResult&&) = default;
  TrialTokenResult& operator=(TrialTokenResult&&) = default;

  // Initialize TrialTokenResult with `parsed_token` as nullptr.
  // The status is not allowed to be `OriginTrialTokenStatus::kSuccess`.
  explicit TrialTokenResult(OriginTrialTokenStatus);

  // Initialize TrialTokenResult with a parsed token. `parsed_token`
  // cannot be nullptr.
  TrialTokenResult(OriginTrialTokenStatus status,
                   std::unique_ptr<TrialToken> parsed_token);
  ~TrialTokenResult() = default;

  OriginTrialTokenStatus Status() const { return status_; }

  const TrialToken* ParsedToken() const { return parsed_token_.get(); }

  void SetStatus(OriginTrialTokenStatus status) { status_ = status; }

 private:
  OriginTrialTokenStatus status_;
  std::unique_ptr<TrialToken> parsed_token_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_ORIGIN_TRIALS_TRIAL_TOKEN_RESULT_H_
