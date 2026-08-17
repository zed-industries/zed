/*
 * Copyright (C) 2011 Google Inc. All Rights Reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY GOOGLE, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EXECUTION_CONTEXT_SECURITY_CONTEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EXECUTION_CONTEXT_SECURITY_CONTEXT_H_

#include <memory>
#include <optional>
#include <vector>

#include "base/memory/scoped_refptr.h"
#include "services/network/public/mojom/permissions_policy/permissions_policy_feature.mojom-blink-forward.h"
#include "services/network/public/mojom/web_sandbox_flags.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/permissions_policy/document_policy_feature.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/security_context/insecure_request_policy.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace network {
class PermissionsPolicy;
}  // namespace network

namespace blink {

class DocumentPolicy;
class ExecutionContext;
class PolicyValue;
class SecurityOrigin;

enum class SecureContextMode { kInsecureContext, kSecureContext };

// Explanation as to why |SecureContextMode| was set as it was set.
enum class SecureContextModeExplanation {
  kSecure,
  kSecureLocalhost,
  kInsecureScheme,
  kInsecureAncestor,
};

// Whether to report policy violations when checking whether a feature is
// enabled.
enum class ReportOptions { kReportOnFailure, kDoNotReport };

// Defines the security properties (such as the security origin, and other
// restrictions) of an environment in which script execution or other activity
// may occur.
//
// Mostly 1:1 with ExecutionContext, except that while remote (i.e.,
// out-of-process) environments do not have an ExecutionContext in the local
// process (as execution cannot occur locally), they do have a SecurityContext
// to allow those properties to be queried.
class CORE_EXPORT SecurityContext {
  DISALLOW_NEW();

 public:
  explicit SecurityContext(ExecutionContext*);
  SecurityContext(const SecurityContext&) = delete;
  SecurityContext& operator=(const SecurityContext&) = delete;
  virtual ~SecurityContext();

  void Trace(Visitor*) const;

  using InsecureNavigationsSet = HashSet<unsigned, AlreadyHashedTraits>;
  static WTF::Vector<unsigned> SerializeInsecureNavigationSet(
      const InsecureNavigationsSet&);

  const SecurityOrigin* GetSecurityOrigin() const {
    return security_origin_.get();
  }
  SecurityOrigin* GetMutableSecurityOrigin() { return security_origin_.get(); }

  // Explicitly override the security origin for this security context with
  // safety CHECKs.
  void SetSecurityOrigin(scoped_refptr<SecurityOrigin>);

  // Like SetSecurityOrigin(), but no security CHECKs.
  void SetSecurityOriginForTesting(scoped_refptr<SecurityOrigin>);

  network::mojom::blink::WebSandboxFlags GetSandboxFlags() const {
    return sandbox_flags_;
  }
  bool IsSandboxed(network::mojom::blink::WebSandboxFlags mask) const;
  void SetSandboxFlags(network::mojom::blink::WebSandboxFlags flags);

  // https://w3c.github.io/webappsec-upgrade-insecure-requests/#upgrade-insecure-navigations-set
  void SetInsecureNavigationsSet(const Vector<unsigned>& set) {
    insecure_navigations_to_upgrade_.clear();
    for (unsigned hash : set)
      insecure_navigations_to_upgrade_.insert(hash);
  }
  void AddInsecureNavigationUpgrade(unsigned hashed_host) {
    insecure_navigations_to_upgrade_.insert(hashed_host);
  }
  const InsecureNavigationsSet& InsecureNavigationsToUpgrade() const {
    return insecure_navigations_to_upgrade_;
  }
  void ClearInsecureNavigationsToUpgradeForTest() {
    insecure_navigations_to_upgrade_.clear();
  }

  // https://w3c.github.io/webappsec-upgrade-insecure-requests/#insecure-requests-policy
  void SetInsecureRequestPolicy(mojom::blink::InsecureRequestPolicy policy) {
    insecure_request_policy_ = policy;
  }
  mojom::blink::InsecureRequestPolicy GetInsecureRequestPolicy() const {
    return insecure_request_policy_;
  }

  const network::PermissionsPolicy* GetPermissionsPolicy() const {
    return permissions_policy_.get();
  }
  const network::PermissionsPolicy* GetReportOnlyPermissionsPolicy() const {
    return report_only_permissions_policy_.get();
  }
  void SetPermissionsPolicy(std::unique_ptr<network::PermissionsPolicy>);
  void SetReportOnlyPermissionsPolicy(
      std::unique_ptr<network::PermissionsPolicy>);

  const DocumentPolicy* GetDocumentPolicy() const {
    return document_policy_.get();
  }
  void SetDocumentPolicy(std::unique_ptr<DocumentPolicy> policy);

  const DocumentPolicy* GetReportOnlyDocumentPolicy() const {
    return report_only_document_policy_.get();
  }
  void SetReportOnlyDocumentPolicy(std::unique_ptr<DocumentPolicy> policy);

  // Used by both Permissions and Document Policy
  struct FeatureStatus {
    // Whether the feature is enabled.
    bool enabled;
    // Whether a report should be sent (to Reporting API, ReportingObservers,
    // and the console).
    bool should_report;
    // Where a report should be sent, if one should be. empty if no
    // reporting is configured for this feature.
    String reporting_endpoint;
  };

  // Permissions Policy

  // Tests whether the policy-controlled feature is enabled in this frame.
  // Note: For consistency in reporting, most code should use
  // ExecutionContext::IsFeatureEnabled if a failure should be reported.
  FeatureStatus IsFeatureEnabled(
      network::mojom::PermissionsPolicyFeature) const;

  // Document Policy
  bool IsFeatureEnabled(mojom::blink::DocumentPolicyFeature) const;
  FeatureStatus IsFeatureEnabled(mojom::blink::DocumentPolicyFeature,
                                 PolicyValue threshold_value) const;

  SecureContextMode GetSecureContextMode() const {
    return secure_context_mode_;
  }

  SecureContextModeExplanation GetSecureContextModeExplanation() const {
    return secure_context_explanation_;
  }

  void SetIsWorkerLoadedFromDataURL(bool is_worker_loaded_from_data_url) {
    is_worker_loaded_from_data_url_ = is_worker_loaded_from_data_url;
  }

 protected:
  network::mojom::blink::WebSandboxFlags sandbox_flags_;
  scoped_refptr<SecurityOrigin> security_origin_;
  std::unique_ptr<network::PermissionsPolicy> permissions_policy_;
  std::unique_ptr<network::PermissionsPolicy> report_only_permissions_policy_;
  std::unique_ptr<DocumentPolicy> document_policy_;
  std::unique_ptr<DocumentPolicy> report_only_document_policy_;

 private:
  // execution_context_ will be nullptr if this is a RemoteSecurityContext.
  Member<ExecutionContext> execution_context_;
  mojom::blink::InsecureRequestPolicy insecure_request_policy_;
  InsecureNavigationsSet insecure_navigations_to_upgrade_;
  SecureContextMode secure_context_mode_ = SecureContextMode::kInsecureContext;
  SecureContextModeExplanation secure_context_explanation_ =
      SecureContextModeExplanation::kInsecureScheme;
  bool is_worker_loaded_from_data_url_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EXECUTION_CONTEXT_SECURITY_CONTEXT_H_
