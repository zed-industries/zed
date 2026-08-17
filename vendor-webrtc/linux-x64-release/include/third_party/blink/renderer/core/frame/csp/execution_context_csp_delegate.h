// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_CSP_EXECUTION_CONTEXT_CSP_DELEGATE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_CSP_EXECUTION_CONTEXT_CSP_DELEGATE_H_

#include "third_party/blink/public/mojom/security_context/insecure_request_policy.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/frame/csp/content_security_policy.h"

namespace blink {

class Document;
class ExecutionContext;
class SecurityContext;

class ExecutionContextCSPDelegate final
    : public GarbageCollected<ExecutionContextCSPDelegate>,
      public ContentSecurityPolicyDelegate {
 public:
  explicit ExecutionContextCSPDelegate(ExecutionContext&);

  void Trace(Visitor*) const override;

  // ContentSecurityPolicyDelegate overrides:
  const SecurityOrigin* GetSecurityOrigin() override;
  const KURL& Url() const override;
  void SetSandboxFlags(network::mojom::blink::WebSandboxFlags) override;
  void SetRequireTrustedTypes() override;
  void AddInsecureRequestPolicy(mojom::blink::InsecureRequestPolicy) override;
  std::unique_ptr<SourceLocation> GetSourceLocation() override;
  std::optional<uint16_t> GetStatusCode() override;
  String GetDocumentReferrer() override;
  void DispatchViolationEvent(const SecurityPolicyViolationEventInit&,
                              Element*) override;
  void PostViolationReport(const SecurityPolicyViolationEventInit&,
                           const String& stringified_report,
                           bool is_frame_ancestors_violation,
                           const Vector<String>& report_endpoints,
                           bool use_reporting_api) override;
  void Count(WebFeature) override;
  void AddConsoleMessage(ConsoleMessage*) override;
  void AddInspectorIssue(AuditsIssue) override;
  void DisableEval(const String& error_message) override;
  void SetWasmEvalErrorMessage(const String& error_message) override;
  void ReportBlockedScriptExecutionToInspector(
      const String& directive_text) override;
  void DidAddContentSecurityPolicies(
      WTF::Vector<network::mojom::blink::ContentSecurityPolicyPtr>) override;

 private:
  SecurityContext& GetSecurityContext();
  Document* GetDocument();
  void DispatchViolationEventInternal(const SecurityPolicyViolationEventInit*,
                                      Element*);

  const Member<ExecutionContext> execution_context_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_CSP_EXECUTION_CONTEXT_CSP_DELEGATE_H_
