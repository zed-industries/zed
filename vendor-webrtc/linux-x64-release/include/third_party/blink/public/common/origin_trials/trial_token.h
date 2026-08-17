// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_ORIGIN_TRIALS_TRIAL_TOKEN_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_ORIGIN_TRIALS_TRIAL_TOKEN_H_

#include <memory>
#include <string>
#include <string_view>

#include "base/time/time.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/origin_trials/origin_trial_public_key.h"
#include "url/origin.h"

extern "C" int LLVMFuzzerTestOneInput(const uint8_t* data, size_t size);

namespace blink {

// New entries must be added immediately before the end.
enum class OriginTrialTokenStatus {
  kSuccess = 0,
  kNotSupported = 1,
  kInsecure = 2,
  kExpired = 3,
  kWrongOrigin = 4,
  kInvalidSignature = 5,
  kMalformed = 6,
  kWrongVersion = 7,
  kFeatureDisabled = 8,
  kTokenDisabled = 9,
  kFeatureDisabledForUser = 10,
  kUnknownTrial = 11,
  kMaxValue = kUnknownTrial
};

// The Origin Trials Framework (OT) provides limited access to experimental
// features, on a per-origin basis. This class defines the trial token data
// structure, used to securely provide access to an experimental feature.
//
// Features are defined by string names, provided by the implementers. The OT
// code does not maintain an enum or constant list for feature names. Instead,
// it validates the name provided by the feature implementation against any
// provided tokens.
//
// More documentation on the token format can be found at
// origin_trials_token_structure.md

class BLINK_COMMON_EXPORT TrialToken {
 public:
  ~TrialToken();

  enum class UsageRestriction { kNone, kSubset };

  // If the string represents a signed well-formed token, a token object is
  // returned, and success is returned in the |out_status| parameter. Otherwise,
  // the |out_status| parameter indicates what was wrong with the string, and
  // nullptr is returned.
  // Note that success does not mean that the token is currently valid, or
  // appropriate for a given origin / feature. It only means that it is
  // correctly formatted and signed by the supplied public key, and can be
  // parsed.
  static std::unique_ptr<TrialToken> From(
      std::string_view token_text,
      const OriginTrialPublicKey& public_key,
      OriginTrialTokenStatus* out_status);

  // Returns success if this token is appropriate for use by the given origin
  // and has not yet expired. Otherwise, the return value indicates why the
  // token is not valid.
  OriginTrialTokenStatus IsValid(const url::Origin& origin,
                                 const base::Time& now) const;

  url::Origin origin() const { return origin_; }
  bool match_subdomains() const { return match_subdomains_; }
  std::string feature_name() const { return feature_name_; }
  base::Time expiry_time() const { return expiry_time_; }
  std::string signature() const { return signature_; }
  bool is_third_party() const { return is_third_party_; }
  UsageRestriction usage_restriction() const { return usage_restriction_; }

  static std::unique_ptr<TrialToken> CreateTrialTokenForTesting(
      const url::Origin& origin,
      bool match_subdomains,
      const std::string& feature_name,
      base::Time expiry_time,
      bool is_third_party,
      UsageRestriction usage_restriction,
      const std::string& signature);

 protected:
  // Tests can access the Parse method directly to validate it, and so are
  // declared as friends here. All other access to Parse should be made through
  // TrialToken::From, which will always also ensure that there is a valid
  // signature attached to the token.
  friend class TrialTokenTest;
  friend int ::LLVMFuzzerTestOneInput(const uint8_t*, size_t);

  // If the string represents a properly signed and well-formed token, success
  // is returned, with the token payload, signature and version returned in the
  // |out_token_payload|, |out_token_signature| and |out_token_version|
  // parameters, respectively. Otherwise,the return code indicates what was
  // wrong with the string, and |out_token_payload|, |out_token_signature| and
  // |out_token_version| are unchanged.
  static OriginTrialTokenStatus Extract(std::string_view token_text,
                                        const OriginTrialPublicKey& public_key,
                                        std::string* out_token_payload,
                                        std::string* out_token_signature,
                                        uint8_t* out_token_version);

  // Returns a token object if the string represents a well-formed JSON token
  // payload, or nullptr otherwise.
  static std::unique_ptr<TrialToken> Parse(const std::string& token_payload,
                                           const uint8_t version);

  bool ValidateOrigin(const url::Origin& origin) const;
  bool ValidateFeatureName(std::string_view feature_name) const;
  bool ValidateDate(const base::Time& now) const;

  static bool ValidateSignature(std::string_view signature_text,
                                const std::string& data,
                                const OriginTrialPublicKey& public_key);

 private:
  TrialToken(const url::Origin& origin,
             bool match_subdomains,
             const std::string& feature_name,
             base::Time expiry_time,
             bool is_third_party,
             UsageRestriction usage_restriction);

  // The origin for which this token is valid. Must be a secure origin.
  url::Origin origin_;

  // Indicates if the token should match all subdomains of the origin.
  bool match_subdomains_;

  // The name of the experimental feature which this token enables.
  std::string feature_name_;

  // The time until which this token should be considered valid.
  base::Time expiry_time_;

  // The signature identifying the fully signed contents of the token.
  std::string signature_;

  // Indicates whether the token should validate against third party origin.
  // See design doc
  // https://docs.google.com/document/d/1xALH9W7rWmX0FpjudhDeS2TNTEOXuPn4Tlc9VmuPdHA
  // for more details.
  bool is_third_party_;

  // Indicates the alternative usage restriction mode imposed on the token.
  // See design doc
  // https://docs.google.com/document/d/1xALH9W7rWmX0FpjudhDeS2TNTEOXuPn4Tlc9VmuPdHA
  // for more details.
  UsageRestriction usage_restriction_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_ORIGIN_TRIALS_TRIAL_TOKEN_H_
