// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ORIGIN_TRIALS_ORIGIN_TRIAL_CONTEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ORIGIN_TRIALS_ORIGIN_TRIAL_CONTEXT_H_

#include <optional>

#include "base/time/time.h"
#include "third_party/blink/public/common/origin_trials/trial_token.h"
#include "third_party/blink/public/common/origin_trials/trial_token_validator.h"
#include "third_party/blink/public/mojom/origin_trials/origin_trial_state_host.mojom-blink.h"
#include "third_party/blink/public/mojom/runtime_feature_state/runtime_feature.mojom-shared.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class Document;
class ExecutionContext;
class ScriptState;
class TrialToken;
class TrialTokenResult;

enum class OriginTrialStatus {
  kEnabled = 0,
  kValidTokenNotProvided = 1,
  kOSNotSupported = 2,
  kTrialNotAllowed = 3,
};

struct OriginTrialTokenResult {
  OriginTrialTokenResult(const String& raw_token,
                         OriginTrialTokenStatus status,
                         const std::optional<TrialToken>& parsed_token);
  ~OriginTrialTokenResult() = default;

  String raw_token;
  OriginTrialTokenStatus status;
  std::optional<TrialToken> parsed_token;
};

struct OriginTrialResult {
  String trial_name;
  OriginTrialStatus status;
  Vector<OriginTrialTokenResult> token_results;
};

// `status` is kEnabled if one or more OriginTrialFeatures are enabled.
// `features` is a Vector containing all of the enabled features.
struct OriginTrialFeaturesEnabled {
  OriginTrialStatus status;
  Vector<mojom::blink::OriginTrialFeature> features;
};

// The Origin Trials Framework provides limited access to experimental features,
// on a per-origin basis (origin trials). This class provides the implementation
// to check if the experimental feature should be enabled for the current
// context.  This class is not for direct use by feature implementers.
// Instead, the OriginTrials generated namespace provides a method for each
// trial to check if it is enabled. Experimental features must be defined in
// runtime_enabled_features.json5, which is used to generate origin_trials.h/cc.
//
// Origin trials are defined by string names, provided by the implementers. The
// framework does not maintain an enum or constant list for trial names.
// Instead, the name provided by the feature implementation is validated against
// any provided tokens.
//
// For more information, see https://github.com/GoogleChrome/OriginTrials.
class CORE_EXPORT OriginTrialContext final
    : public GarbageCollected<OriginTrialContext> {
 public:
  explicit OriginTrialContext(ExecutionContext*);

  void SetTrialTokenValidatorForTesting(std::unique_ptr<TrialTokenValidator>);

  // Parses an Origin-Trial header into individual tokens.
  // Returns null if the header value was malformed and could not be parsed.
  // If the header does not contain any tokens, this returns an empty vector.
  static std::unique_ptr<Vector<String>> ParseHeaderValue(
      const String& header_value);

  static void AddTokensFromHeader(ExecutionContext*,
                                  const String& header_value);
  static void AddTokens(ExecutionContext*, const Vector<String>* tokens);

  // Returns the trial tokens that are active in a specific ExecutionContext.
  // Returns null if no tokens were added to the ExecutionContext.
  static std::unique_ptr<Vector<String>> GetTokens(ExecutionContext*);

  // Returns the all enabled features to be inherited by worker.
  static std::unique_ptr<Vector<mojom::blink::OriginTrialFeature>>
  GetInheritedTrialFeatures(ExecutionContext*);

  // Returns the navigation trial features that are enabled in the specified
  // ExecutionContext, that should be forwarded to (and activated in)
  // ExecutionContexts navigated to from the given ExecutionContext. Returns
  // null if no such trials were added to the ExecutionContext.
  static std::unique_ptr<Vector<mojom::blink::OriginTrialFeature>>
  GetEnabledNavigationFeatures(ExecutionContext*);

  // Activates trial features for dedicated worker or worklet. The input trial
  // features are inherited from page loading the worker.
  static void ActivateWorkerInheritedFeatures(
      ExecutionContext*,
      const Vector<mojom::blink::OriginTrialFeature>*);

  // Activates navigation trial features forwarded from the ExecutionContext
  // that navigated to the specified ExecutionContext. Only features for which
  // origin_trials::IsCrossNavigationFeature returns true can be activated via
  // this method. Trials activated via this method will return true from
  // IsNavigationFeatureActivated, for the specified ExecutionContext.
  static void ActivateNavigationFeaturesFromInitiator(
      ExecutionContext*,
      const Vector<mojom::blink::OriginTrialFeature>*);

  void AddToken(const String& token);
  // Add a token injected from external script. The token may be a third-party
  // token, which will be validated against the given origin(s) of the injecting
  // script. This should only be called with at least one valid external origin,
  // otherwise use AddToken().
  void AddTokenFromExternalScript(
      const String& token,
      const Vector<scoped_refptr<SecurityOrigin>>& external_origins);
  void AddTokens(const Vector<String>& tokens);

  void ActivateWorkerInheritedFeatures(
      const Vector<mojom::blink::OriginTrialFeature>& features);

  void ActivateNavigationFeaturesFromInitiator(
      const Vector<mojom::blink::OriginTrialFeature>& features);

  // Forces a given origin-trial-enabled feature to be enabled in this context
  // and immediately adds required bindings to already initialized JS contexts.
  void AddFeature(mojom::blink::OriginTrialFeature feature);

  // Forces given trials to be enabled in this context and immediately adds
  // required bindings to already initialized JS contexts.
  void AddForceEnabledTrials(const Vector<String>& trial_names);

  // Returns true if the feature should be considered enabled for the current
  // execution context.
  bool IsFeatureEnabled(mojom::blink::OriginTrialFeature feature) const;

  // Gets the latest expiry time of all valid tokens that enable |feature|. If
  // there are no valid tokens enabling the feature, this will return the null
  // time (base::Time()). Note: This will only find expiry times for features
  // backed by a token, so will not work for features enabled via |AddFeature|.
  base::Time GetFeatureExpiry(mojom::blink::OriginTrialFeature feature);

  std::unique_ptr<Vector<mojom::blink::OriginTrialFeature>>
  GetInheritedTrialFeatures() const;

  std::unique_ptr<Vector<mojom::blink::OriginTrialFeature>>
  GetEnabledNavigationFeatures() const;

  // Returns true if the navigation feature is activated in the current
  // ExecutionContext. Navigation features are features that are enabled in one
  // ExecutionContext, but whose behavior is activated in ExecutionContexts that
  // are navigated to from that context. For example, if navigating from context
  // A to B, a navigation feature is enabled in A, and activated in B.
  bool IsNavigationFeatureActivated(
      const mojom::blink::OriginTrialFeature feature) const;

  // Installs JavaScript bindings on the relevant objects for any features which
  // should be enabled by the current set of trial tokens. This method is called
  // every time a token is added to the document (including when tokens are
  // added via script). JavaScript-exposed members will be properly visible, for
  // existing objects in the V8 context. If the V8 context is not initialized,
  // or there are no enabled features, or all enabled features are already
  // initialized, this method returns without doing anything. That is, it is
  // safe to call this method multiple times, even if no trials are newly
  // enabled.
  void InitializePendingFeatures();

  void Trace(Visitor*) const;

  // A copy of the HashMap is returned as new entries can be added to
  // `trial_results_` afterwards, which potentially causes
  // inconsistency.
  const HashMap<String, OriginTrialResult> GetOriginTrialResultsForDevtools()
      const {
    return trial_results_;
  }

  const HashMap<mojom::blink::OriginTrialFeature, Vector<String>>
  GetFeatureToTokensForTesting() const {
    return feature_to_tokens_;
  }

 private:
  struct OriginInfo {
    const scoped_refptr<const SecurityOrigin> origin;
    bool is_secure;
  };

  // Handle token from document origin or third party origins, initialize
  // features if the token is valid.
  void AddTokenInternal(const String& token,
                        const OriginInfo origin_info,
                        const Vector<OriginInfo>* script_origins);

  // If this returns false, the trial cannot be enabled (e.g. due to it is
  // invalid in the browser's present configuration).
  bool CanEnableTrialFromName(const StringView& trial_name);

  // Enable features by trial name. Returns a OriginTrialFeaturesEnabled struct
  // containing whether one or more trials were enabled, and a Vector of the
  // OriginTrialFeatures representing those trials.
  OriginTrialFeaturesEnabled EnableTrialFromName(const String& trial_name,
                                                 base::Time expiry_time);

  // Validate the trial token. If valid, the trial named in the token is
  // added to the list of enabled trials. Returns true or false to indicate if
  // the token is valid.
  bool EnableTrialFromToken(const String& token, const OriginInfo origin_info);

  // Validate the trial token injected by external script from script_origins.
  // If is_third_party flag is set on the token, script_origins will be used for
  // validation. Otherwise it's the same as above.
  bool EnableTrialFromToken(const String& token,
                            const OriginInfo origin_info,
                            const Vector<OriginInfo>* script_origins);

  // Installs a series of OriginTrialFeatures listed in a HashSet. The return
  // value indicates whether binding features were added, signalling that V8
  // has to proceed with installing the conditional features.
  bool InstallFeatures(
      const HashSet<mojom::blink::OriginTrialFeature>& features,
      Document&,
      ScriptState*);

  // Installs a settings feature for the relevant Document instance. Returns
  // whether the given OriginTrialFeature describes a setting feature.
  bool InstallSettingFeature(Document&, mojom::blink::OriginTrialFeature);

  // Caches raw origin trial token along with the parse result to
  // `trial_results_`.
  void CacheToken(const String& raw_token,
                  const TrialTokenResult&,
                  OriginTrialStatus);

  const SecurityOrigin* GetSecurityOrigin();
  bool IsSecureContext();
  OriginInfo GetCurrentOriginInfo();

  // Send the token to the browser process to update persistent origin trial
  // tokens. Will check if |parsed_token| is persistable before sending to the
  // browser, and |parsed_token| should be the parsed version of |raw_token|.
  void SendTokenToBrowser(const OriginInfo& origin_info,
                          const TrialToken& parsed_token,
                          const String& raw_token,
                          const Vector<OriginInfo>* script_origin_info);

  HashSet<mojom::blink::OriginTrialFeature> enabled_features_;
  HashSet<mojom::blink::OriginTrialFeature> installed_features_;
  HashSet<mojom::blink::OriginTrialFeature> navigation_activated_features_;
  WTF::HashMap<mojom::blink::OriginTrialFeature, base::Time>
      feature_expiry_times_;
  std::unique_ptr<TrialTokenValidator> trial_token_validator_;
  Member<ExecutionContext> context_;
  // Stores raw origin trial token along with the parse result.
  // This field is mainly used for devtools support, but
  // `OriginTrialContext::GetTokens` also depends on the structure.
  HashMap</* Trial Name */ String, OriginTrialResult> trial_results_;
  // Stores the OriginTrialFeature enum value along with its corresponding
  // validated and parsed trial tokens. Used for security checks in the
  // browser's RuntimeFeatureChangeImpl.
  HashMap<mojom::blink::OriginTrialFeature, Vector</*Raw Token*/ String>>
      feature_to_tokens_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ORIGIN_TRIALS_ORIGIN_TRIAL_CONTEXT_H_
