/*
 * Copyright (C) 2011 Google, Inc. All rights reserved.
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
 * THIS SOFTWARE IS PROVIDED BY GOOGLE INC. ``AS IS'' AND ANY
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
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_CSP_CONTENT_SECURITY_POLICY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_CSP_CONTENT_SECURITY_POLICY_H_

#include <cstddef>
#include <memory>
#include <optional>

#include "base/gtest_prod_util.h"
#include "base/unguessable_token.h"
#include "services/network/public/cpp/content_security_policy/content_security_policy.h"
#include "services/network/public/mojom/content_security_policy.mojom-blink.h"
#include "services/network/public/mojom/web_sandbox_flags.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/devtools/console_message.mojom-blink.h"
#include "third_party/blink/public/mojom/devtools/inspector_issue.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/fetch/fetch_api_request.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/security_context/insecure_request_policy.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/frame/csp/content_security_policy_violation_type.h"
#include "third_party/blink/renderer/core/frame/web_feature_forward.h"
#include "third_party/blink/renderer/platform/crypto.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/loader/fetch/integrity_metadata.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_loader_options.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_request.h"
#include "third_party/blink/renderer/platform/weborigin/reporting_disposition.h"
#include "third_party/blink/renderer/platform/weborigin/scheme_registry.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace WTF {
class OrdinalNumber;
}

namespace blink {

class AuditsIssue;
class ConsoleMessage;
class DOMWrapperWorld;
class Element;
class ExecutionContext;
class LocalFrame;
class KURL;
class SecurityOrigin;
class SecurityPolicyViolationEventInit;
class SourceLocation;

typedef HeapVector<Member<ConsoleMessage>> ConsoleMessageVector;
using RedirectStatus = ResourceRequest::RedirectStatus;
using network::mojom::blink::CSPDirectiveName;

using CSPCheckResult = network::CSPCheckResult;

//  A delegate interface to implement violation reporting, support for some
//  directives and other miscellaneous functionality.
class CORE_EXPORT ContentSecurityPolicyDelegate : public GarbageCollectedMixin {
 public:
  // Returns the SecurityOrigin this content security policy is bound to. Used
  // for matching the 'self' keyword. Must return a non-null value.
  // See https://w3c.github.io/webappsec-csp/#policy-self-origin.
  virtual const SecurityOrigin* GetSecurityOrigin() = 0;

  // Returns the URL this content security policy is bound to.
  // Used for https://w3c.github.io/webappsec-csp/#violation-url and so.
  // Note: Url() is used for several purposes that are specced slightly
  // differently.
  // See comments at the callers.
  virtual const KURL& Url() const = 0;

  // Directives support.
  virtual void SetSandboxFlags(network::mojom::blink::WebSandboxFlags) = 0;
  virtual void SetRequireTrustedTypes() = 0;
  virtual void AddInsecureRequestPolicy(
      mojom::blink::InsecureRequestPolicy) = 0;

  // Violation reporting.

  // See https://w3c.github.io/webappsec-csp/#create-violation-for-global.
  // These functions are used to create the violation object.
  virtual std::unique_ptr<SourceLocation> GetSourceLocation() = 0;
  virtual std::optional<uint16_t> GetStatusCode() = 0;
  // If the Delegate is not bound to a document, a null string should be
  // returned as the referrer.
  virtual String GetDocumentReferrer() = 0;

  virtual void DispatchViolationEvent(const SecurityPolicyViolationEventInit&,
                                      Element*) = 0;
  virtual void PostViolationReport(const SecurityPolicyViolationEventInit&,
                                   const String& stringified_report,
                                   bool is_frame_ancestors_violaton,
                                   const Vector<String>& report_endpoints,
                                   bool use_reporting_api) = 0;

  virtual void Count(WebFeature) = 0;

  virtual void AddConsoleMessage(ConsoleMessage*) = 0;
  virtual void AddInspectorIssue(AuditsIssue) = 0;
  virtual void DisableEval(const String& error_message) = 0;
  virtual void SetWasmEvalErrorMessage(const String& error_message) = 0;
  virtual void ReportBlockedScriptExecutionToInspector(
      const String& directive_text) = 0;
  virtual void DidAddContentSecurityPolicies(
      WTF::Vector<network::mojom::blink::ContentSecurityPolicyPtr>) = 0;
};

class CORE_EXPORT ContentSecurityPolicy final
    : public GarbageCollected<ContentSecurityPolicy> {
 public:
  enum ExceptionStatus { kWillThrowException, kWillNotThrowException };

  // The |type| argument given to inline checks, e.g.:
  // https://w3c.github.io/webappsec-csp/#should-block-inline
  // Its possible values are listed in:
  // https://w3c.github.io/webappsec-csp/#effective-directive-for-inline-check
  enum class InlineType {
    kNavigation,
    kScript,
    kScriptAttribute,
    kScriptSpeculationRules,  // TODO(https://crbug.com/1382361): Standardize.
    kStyle,
    kStyleAttribute
  };

  // CheckHeaderType can be passed to Allow*FromSource methods to control which
  // types of CSP headers are checked.
  enum class CheckHeaderType {
    // Check both Content-Security-Policy and
    // Content-Security-Policy-Report-Only headers.
    kCheckAll,
    // Check Content-Security-Policy headers only and ignore
    // Content-Security-Policy-Report-Only headers.
    kCheckEnforce,
    // Check Content-Security-Policy-Report-Only headers only and ignore
    // Content-Security-Policy headers.
    kCheckReportOnly
  };

  // Helper type for the method AllowTrustedTypePolicy.
  enum AllowTrustedTypePolicyDetails {
    kAllowed,
    kDisallowedName,
    kDisallowedDuplicateName
  };

  static constexpr size_t kMaxSampleLength = 40;

  ContentSecurityPolicy();
  ~ContentSecurityPolicy();
  void Trace(Visitor*) const;

  bool IsBound();
  void BindToDelegate(ContentSecurityPolicyDelegate&);

  void AddPolicies(
      Vector<network::mojom::blink::ContentSecurityPolicyPtr> policies);

  // Returns whether or not the Javascript code generation should call back the
  // CSP checker before any script evaluation from a string attempts.
  //
  // CSP has two mechanisms for controlling eval: script-src and TrustedTypes.
  // This returns true when any of those should to be checked.
  bool ShouldCheckEval() const;

  // When the reporting status is |SendReport|, the |ExceptionStatus|
  // should indicate whether the caller will throw a JavaScript
  // exception in the event of a violation. When the caller will throw
  // an exception, ContentSecurityPolicy does not log a violation
  // message to the console because it would be redundant.
  bool AllowEval(ReportingDisposition,
                 ExceptionStatus,
                 const String& script_content);
  bool AllowWasmCodeGeneration(ReportingDisposition,
                               ExceptionStatus,
                               const String& script_content);

  HashSet<HashAlgorithm> HashesToReport() const;
  void AddHashReportIfNeeded(
      LocalFrame* frame,
      const String& url,
      const HashMap<HashAlgorithm, String>& integrity_hashes) const;

  // AllowFromSource() wrappers.
  bool AllowBaseURI(const KURL&);
  bool AllowConnectToSource(
      const KURL&,
      const KURL& url_before_redirects,
      RedirectStatus,
      ReportingDisposition = ReportingDisposition::kReport,
      CheckHeaderType = CheckHeaderType::kCheckAll);
  bool AllowFormAction(const KURL&);
  bool AllowImageFromSource(
      const KURL&,
      const KURL& url_before_redirects,
      RedirectStatus,
      ReportingDisposition = ReportingDisposition::kReport,
      CheckHeaderType = CheckHeaderType::kCheckAll);
  bool AllowMediaFromSource(const KURL&);
  bool AllowObjectFromSource(const KURL&);
  bool AllowScriptFromSource(
      const KURL&,
      const String& nonce,
      const IntegrityMetadataSet&,
      ParserDisposition,
      const KURL& url_before_redirects,
      RedirectStatus,
      ReportingDisposition = ReportingDisposition::kReport,
      CheckHeaderType = CheckHeaderType::kCheckAll);
  bool AllowWorkerContextFromSource(const KURL&);

  bool AllowTrustedTypePolicy(
      const String& policy_name,
      bool is_duplicate,
      AllowTrustedTypePolicyDetails& violation_details,
      std::optional<base::UnguessableToken> issue_id = std::nullopt);

  // Passing 'String()' into the |nonce| arguments in the following methods
  // represents an unnonced resource load.
  //
  // For kJavaScriptURL case, |element| will not be present for navigations to
  // javascript URLs, as those checks happen in the middle of the navigation
  // algorithm, and we generally don't have access to the responsible element.
  //
  // For kInlineEventHandler case, |element| will be present almost all of the
  // time, but because of strangeness around targeting handlers for '<body>',
  // '<svg>', and '<frameset>', it will be 'nullptr' for handlers on those
  // elements.
  bool AllowInline(InlineType,
                   Element*,
                   const String& content,
                   const String& nonce,
                   const String& context_url,
                   const WTF::OrdinalNumber& context_line,
                   ReportingDisposition = ReportingDisposition::kReport);

  static bool IsScriptInlineType(InlineType);

  // TODO(crbug.com/889751): Remove "mojom::blink::RequestContextType" once
  // all the code migrates.
  bool AllowRequest(mojom::blink::RequestContextType,
                    network::mojom::RequestDestination,
                    network::mojom::RequestMode,
                    const KURL&,
                    const String& nonce,
                    const IntegrityMetadataSet&,
                    ParserDisposition,
                    const KURL& url_before_redirects,
                    RedirectStatus,
                    ReportingDisposition = ReportingDisposition::kReport,
                    CheckHeaderType = CheckHeaderType::kCheckAll);

  // Determine whether to enforce the assignment failure. Also handle reporting.
  // Returns whether enforcing Trusted Types CSP directives are present.
  bool AllowTrustedTypeAssignmentFailure(
      const String& message,
      const String& sample = String(),
      const String& sample_prefix = String(),
      std::optional<base::UnguessableToken> issue_id = std::nullopt);

  void UsesHashAlgorithm(IntegrityAlgorithm algorithm);

  void SetOverrideAllowInlineStyle(bool);
  void SetOverrideURLForSelf(const KURL&);

  bool IsActive() const;
  bool IsActiveForConnections() const;

  // If a frame is passed in, the message will be logged to its active
  // document's console.  Otherwise, the message will be logged to this object's
  // |m_executionContext|.
  void LogToConsole(ConsoleMessage*, LocalFrame* = nullptr);

  void ReportReportOnlyInMeta(const String&);
  void ReportMetaOutsideHead(const String&);

  // If a frame is passed in, the report will be sent using it as a context. If
  // no frame is passed in, the report will be sent via this object's
  // |m_executionContext| (or dropped on the floor if no such context is
  // available).
  // If |sourceLocation| is not set, the source location will be the context's
  // current location.
  // If an inspector issue is reported, and |issue_id| is present, it will be
  // reported on the issue. This is useful to provide a link from the
  // JavaScript TypeError to the inspector issue in the DevTools front-end.
  void ReportViolation(
      const String& directive_text,
      CSPDirectiveName effective_type,
      const String& console_message,
      const KURL& blocked_url,
      const Vector<String>& report_endpoints,
      bool use_reporting_api,
      const String& header,
      network::mojom::ContentSecurityPolicyType,
      ContentSecurityPolicyViolationType,
      std::unique_ptr<SourceLocation>,
      LocalFrame* = nullptr,
      Element* = nullptr,
      const String& source = g_empty_string,
      const String& source_prefix = g_empty_string,
      std::optional<base::UnguessableToken> issue_id = std::nullopt);

  // Called when mixed content is detected on a page; will trigger a violation
  // report if the 'block-all-mixed-content' directive is specified for a
  // policy.
  void ReportMixedContent(const KURL& blocked_url, RedirectStatus);

  void ReportBlockedScriptExecutionToInspector(
      const String& directive_text) const;

  // Used as <object>'s URL when there is no `src` attribute.
  const KURL FallbackUrlForPlugin() const;

  void EnforceSandboxFlags(network::mojom::blink::WebSandboxFlags);
  void RequireTrustedTypes();
  bool TrustedTypesRequired() const { return require_trusted_types_; }
  String EvalDisabledErrorMessage() const;
  String WasmEvalDisabledErrorMessage() const;

  // Upgrade-Insecure-Requests and Block-All-Mixed-Content are represented in
  // |m_insecureRequestPolicy|
  void EnforceStrictMixedContentChecking();
  void UpgradeInsecureRequests();
  mojom::blink::InsecureRequestPolicy GetInsecureRequestPolicy() const {
    return insecure_request_policy_;
  }

  bool ExperimentalFeaturesEnabled() const;

  // CSP can be set from multiple sources; if a directive is set by multiple
  // sources, the strictest one will be used. A CSP can be considered strict
  // if the `base-uri`, `object-src`, and `script-src` directives are all
  // strict enough (even if the strictest directives come from different CSP
  // sources).
  bool IsStrictPolicyEnforced() const { return enforces_strict_policy_; }

  // Whether the main world's CSP should be bypassed based on the current
  // javascript world we are in.
  // Note: This is deprecated. New usages should not be added. Operations in an
  // isolated world should use the isolated world CSP instead of bypassing the
  // main world CSP. See
  // ExecutionContext::GetContentSecurityPolicyForCurrentWorld.
  static bool ShouldBypassMainWorldDeprecated(const ExecutionContext*);

  // Whether the main world's CSP should be bypassed for operations in the given
  // |world|.
  // Note: This is deprecated. New usages should not be added. Operations in an
  // isolated world should use the isolated world CSP instead of bypassing the
  // main world CSP. See ExecutionContext::GetContentSecurityPolicyForWorld.
  static bool ShouldBypassMainWorldDeprecated(const DOMWrapperWorld* world);

  static bool IsNonceableElement(const Element*);

  static const char* GetDirectiveName(CSPDirectiveName type);
  static CSPDirectiveName GetDirectiveType(const String& name);

  bool HasHeaderDeliveredPolicy() const { return header_delivered_; }

  // Returns the 'wasm-eval' source is supported.
  bool SupportsWasmEval() const { return supports_wasm_eval_; }
  void SetSupportsWasmEval(bool value) { supports_wasm_eval_ = value; }

  // Retrieve the parsed policies.
  const WTF::Vector<network::mojom::blink::ContentSecurityPolicyPtr>&
  GetParsedPolicies() const;

  // Retrieves the parsed sandbox flags. A lot of the time the execution
  // context will be used for all sandbox checks but there are situations
  // (before installing the document that this CSP will bind to) when
  // there is no execution context to enforce the sandbox flags.
  network::mojom::blink::WebSandboxFlags GetSandboxMask() const {
    return sandbox_mask_;
  }

  bool HasPolicyFromSource(network::mojom::ContentSecurityPolicySource) const;

  // Whether policies allow loading an opaque URL in a <fencedframe>.
  //
  // The document is not allowed to retrieve data about the URL, so the only
  // allowed `fenced-frame-src` are the one allowing every HTTPs url:
  // - '*'
  // - https:
  // - https://*:*
  bool AllowFencedFrameOpaqueURL() const;

  // Returns whether enforcing frame-ancestors CSP directives are present.
  bool HasEnforceFrameAncestorsDirectives();

  void Count(WebFeature feature) const;

 private:
  FRIEND_TEST_ALL_PREFIXES(ContentSecurityPolicyTest, NonceInline);
  FRIEND_TEST_ALL_PREFIXES(ContentSecurityPolicyTest, NonceSinglePolicy);
  FRIEND_TEST_ALL_PREFIXES(ContentSecurityPolicyTest, NonceMultiplePolicy);
  FRIEND_TEST_ALL_PREFIXES(ContentSecurityPolicyTest, EmptyCSPIsNoOp);
  FRIEND_TEST_ALL_PREFIXES(BaseFetchContextTest, CanRequest);
  FRIEND_TEST_ALL_PREFIXES(BaseFetchContextTest, CheckCSPForRequest);
  FRIEND_TEST_ALL_PREFIXES(BaseFetchContextTest,
                           AllowResponseChecksReportedAndEnforcedCSP);
  FRIEND_TEST_ALL_PREFIXES(FrameFetchContextTest,
                           PopulateResourceRequestChecksReportOnlyCSP);

  void ApplyPolicySideEffectsToDelegate();
  void ReportUseCounters(
      const Vector<network::mojom::blink::ContentSecurityPolicyPtr>& policies);
  void ComputeInternalStateForParsedPolicy(
      const network::mojom::blink::ContentSecurityPolicy& csp);

  void LogToConsole(
      const String& message,
      mojom::ConsoleMessageLevel = mojom::ConsoleMessageLevel::kError);

  bool ShouldSendViolationReport(const String&) const;
  void DidSendViolationReport(const String&);
  void PostViolationReport(const SecurityPolicyViolationEventInit*,
                           LocalFrame*,
                           const Vector<String>& report_endpoints,
                           bool use_reporting_api);

  bool AllowFromSource(CSPDirectiveName,
                       const KURL&,
                       const KURL& url_before_redirects,
                       RedirectStatus,
                       ReportingDisposition = ReportingDisposition::kReport,
                       CheckHeaderType = CheckHeaderType::kCheckAll,
                       const String& = String(),
                       const IntegrityMetadataSet& = IntegrityMetadataSet(),
                       ParserDisposition = kParserInserted);

  static void FillInCSPHashValues(
      const String& source,
      WTF::HashSet<IntegrityAlgorithm> hash_algorithms_used,
      Vector<network::mojom::blink::CSPHashSourcePtr>& csp_hash_values);

  // checks a vector of csp hashes against policy, probably a good idea
  // to use in tandem with FillInCSPHashValues.
  static bool CheckHashAgainstPolicy(
      Vector<network::mojom::blink::CSPHashSourcePtr>&,
      const network::mojom::blink::ContentSecurityPolicy&,
      InlineType);

  bool ShouldBypassContentSecurityPolicy(
      const KURL&,
      SchemeRegistry::PolicyAreas = SchemeRegistry::kPolicyAreaAll) const;

  // TODO: Consider replacing 'ContentSecurityPolicy::ViolationType' with the
  // mojo enum.
  mojom::blink::ContentSecurityPolicyViolationType BuildCSPViolationType(
      ContentSecurityPolicyViolationType violation_type);

  Member<ContentSecurityPolicyDelegate> delegate_;
  bool override_inline_style_allowed_ = false;
  Vector<network::mojom::blink::ContentSecurityPolicyPtr> policies_;
  ConsoleMessageVector console_messages_;
  bool header_delivered_{false};

  HashSet<unsigned, AlreadyHashedTraits> violation_reports_sent_;

  // We put the hash functions used on the policy object so that we only need
  // to calculate digests using those hashing algorithms which show up in the
  // policy.
  WTF::HashSet<IntegrityAlgorithm> hash_algorithms_used_;

  // State flags used to configure the environment after parsing a policy.
  network::mojom::blink::WebSandboxFlags sandbox_mask_;
  bool require_trusted_types_;
  String disable_eval_error_message_;
  String disable_wasm_eval_error_message_;
  mojom::blink::InsecureRequestPolicy insecure_request_policy_;

  bool supports_wasm_eval_ = false;

  bool enforces_strict_policy_{false};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_CSP_CONTENT_SECURITY_POLICY_H_
