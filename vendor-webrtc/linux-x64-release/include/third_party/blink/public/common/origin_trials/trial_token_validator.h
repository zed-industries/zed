// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_ORIGIN_TRIALS_TRIAL_TOKEN_VALIDATOR_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_ORIGIN_TRIALS_TRIAL_TOKEN_VALIDATOR_H_

#include <memory>
#include <string>
#include <string_view>
#include <vector>

#include "base/containers/flat_map.h"
#include "base/containers/span.h"
#include "base/functional/callback.h"
#include "base/time/time.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/origin_trials/trial_token.h"
#include "third_party/blink/public/mojom/origin_trials/origin_trial_feature.mojom-forward.h"
#include "url/origin.h"

namespace net {
class HttpResponseHeaders;
class URLRequest;
}  // namespace net

namespace blink {

class OriginTrialPolicy;
class TrialTokenResult;

// The expiry grace period for origin trials that must be manually completed.
constexpr base::TimeDelta kExpiryGracePeriod = base::Days(30);

// TrialTokenValidator checks that a page's OriginTrial token enables a certain
// feature.
//
// Given a policy, feature and token, this class determines if the feature
// should be enabled or not for a specific document.
class BLINK_COMMON_EXPORT TrialTokenValidator {
 public:
  // Wrapper for url::Origin with explicit information about the security
  // status of the origin.
  // This should rarely be constructed by calling code.
  // See ValidateTokenAndTrialWithOriginInfo for more info.
  struct BLINK_COMMON_EXPORT OriginInfo {
    url::Origin origin;
    bool is_secure;

    explicit OriginInfo(const url::Origin& origin);
    OriginInfo(const url::Origin& origin, bool is_secure);

    // Movable & Copyable
    OriginInfo(const OriginInfo&) = default;
    OriginInfo(OriginInfo&&) = default;
    OriginInfo& operator=(const OriginInfo&) = default;
    OriginInfo& operator=(OriginInfo&&) = default;
  };

  TrialTokenValidator();
  virtual ~TrialTokenValidator();

  using FeatureToTokensMap =
      base::flat_map<std::string /* feature_name */,
                     std::vector<std::string /* token */>>;

  // Convenience function for non-third-party tokens.
  virtual TrialTokenResult ValidateTokenAndTrial(std::string_view token,
                                                 const url::Origin& origin,
                                                 base::Time current_time) const;

  // Validates a trial token as `ValidateToken`. If the token itself is valid,
  // it is then validated against the trial configurations in
  // runtime_enabled_features.json5 to ensure that
  // * The trial exists
  // * If the token is third-party, that the trial allows third-party
  // * If the trial does not allow insecure origins, `origin` is checked to
  //   confirm it is secure, and if the token is a third_party token, the
  //   `third_party_origins` are checked to ensure the token is validated
  //   against a secure origin.
  virtual TrialTokenResult ValidateTokenAndTrial(
      std::string_view token,
      const url::Origin& origin,
      base::span<const url::Origin> third_party_origins,
      base::Time current_time) const;

  // Dedicated version of `ValidateTokenAndTrial` intended for use by
  // `blink::OriginTrialContext`, so it can pass in its own evaluation
  // of origin security based on `blink::OriginTrialContext::IsSecureContext`.
  // The browser process should call `ValidateTokenAndTrial` instead, which
  // takes care of the origin security evaluation internally.
  virtual TrialTokenResult ValidateTokenAndTrialWithOriginInfo(
      std::string_view token,
      const OriginInfo& origin,
      base::span<const OriginInfo> third_party_origins,
      base::Time current_time) const;

  // If the token validates, status will be set to
  // OriginTrialTokenStatus::kSuccess, the rest will be populated with name of
  // the feature this token enables, the expiry time of the token and whether it
  // is a third-party token. Otherwise, only the status will be set.
  // This method is thread-safe.
  virtual TrialTokenResult ValidateToken(std::string_view token,
                                         const url::Origin& origin,
                                         base::Time current_time) const;
  // Validates a token for the given `origin`. If identified as a third-party
  // token, instead validate for the given list in `third_party_origins`.
  // Validation of a third-party token will fail if `third_party_origins` is
  // empty. Returns the same result as ValidateToken() above.
  // This method is thread-safe.
  virtual TrialTokenResult ValidateToken(
      std::string_view token,
      const url::Origin& origin,
      base::span<const url::Origin> third_party_origins,
      base::Time current_time) const;

  // Re-validate that `trial_name` is still enabled given the token information.
  // The token from which the information was obtained should previously have
  // been validated with either `ValidateToken` or `ValidateTokenAndTrial`, to
  // ensure that it was a valid token for the origin to which we are applying
  // it.
  virtual bool RevalidateTokenAndTrial(
      std::string_view trial_name,
      const base::Time token_expiry_time,
      const TrialToken::UsageRestriction token_usage_restriction,
      std::string_view token_signature,
      const base::Time current_time) const;

  // Return the set of features enabled by the given `trial_name`.
  // TODO(crbug.com/1227440): Refactor this to be a part of more general
  //                          validation flows instead of a stand-alone.
  std::vector<mojom::OriginTrialFeature> FeaturesEnabledByTrial(
      std::string_view trial_name);

  // Return true if the trial in question enables at least one feature on the
  // current OS platform.
  // TODO(crbug.com/1227440): Refactor this to be a part of more general
  //                          validation flows instead of a stand-alone.
  bool TrialEnablesFeaturesForOS(std::string_view trial_name);

  // `request` must not be nullptr.
  // NOTE: This is not currently used, but remains here for future trials.
  bool RequestEnablesFeature(const net::URLRequest* request,
                             std::string_view feature_name,
                             base::Time current_time) const;

  // Returns whether the given response for the given URL enables the named
  // Origin or Deprecation Trial at the given time.
  //
  // `response_headers` must not be nullptr.
  bool RequestEnablesFeature(const GURL& request_url,
                             const net::HttpResponseHeaders* response_headers,
                             std::string_view feature_name,
                             base::Time current_time) const;

  // Similar to `RequestEnablesFeature()`, but for Deprecation Trials that may
  // be enabled on insecure origins.
  //
  // For Origin Trials (as opposed to Deprecation Trials) or Deprecation Trials
  // that are enabled exclusively on secure origins, use
  // `RequestEnablesFeature()` instead.
  //
  // Functionally, the only difference is that this can return true even if
  // `request_url`'s origin is not secure.
  //
  // `response_headers` must not be nullptr.
  bool RequestEnablesDeprecatedFeature(
      const GURL& request_url,
      const net::HttpResponseHeaders* response_headers,
      std::string_view feature_name,
      base::Time current_time) const;

  // Returns all valid tokens in `headers`.
  std::unique_ptr<FeatureToTokensMap> GetValidTokensFromHeaders(
      const url::Origin& origin,
      const net::HttpResponseHeaders* headers,
      base::Time current_time) const;

  // Returns all valid tokens in `tokens`. This method is used to re-validate
  // previously stored tokens.
  std::unique_ptr<FeatureToTokensMap> GetValidTokens(
      const url::Origin& origin,
      const FeatureToTokensMap& tokens,
      base::Time current_time) const;

  static void SetOriginTrialPolicyGetter(
      base::RepeatingCallback<OriginTrialPolicy*()> policy);
  static void ResetOriginTrialPolicyGetter();

  static bool IsTrialPossibleOnOrigin(const GURL& url);

 private:
  // Helper for `RequestEnablesFeature()` and
  // `RequestEnablesDeprecatedFeature()`.
  bool ResponseBearsValidTokenForFeature(
      const GURL& request_url,
      const net::HttpResponseHeaders& response_headers,
      std::string_view feature_name,
      base::Time current_time) const;
};  // class TrialTokenValidator

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_ORIGIN_TRIALS_TRIAL_TOKEN_VALIDATOR_H_
