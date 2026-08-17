// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_CSP_CSP_DIRECTIVE_LIST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_CSP_CSP_DIRECTIVE_LIST_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/frame/csp/content_security_policy.h"
#include "third_party/blink/renderer/platform/crypto.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_request.h"
#include "third_party/blink/renderer/platform/weborigin/reporting_disposition.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class KURL;

enum class ResourceType : uint8_t;

struct CORE_EXPORT CSPOperativeDirective {
  CSPDirectiveName type;
  const network::mojom::blink::CSPSourceList* source_list;
};

CORE_EXPORT
bool CSPDirectiveListIsReportOnly(
    const network::mojom::blink::ContentSecurityPolicy& csp);

CORE_EXPORT
CSPCheckResult CSPDirectiveListAllowFromSource(
    const network::mojom::blink::ContentSecurityPolicy& csp,
    ContentSecurityPolicy* policy,
    CSPDirectiveName type,
    const KURL& url,
    const KURL& url_before_redirects,
    ResourceRequest::RedirectStatus redirect_status,
    ReportingDisposition reporting_disposition,
    const String& nonce = String(),
    const IntegrityMetadataSet& integrity_metadata = IntegrityMetadataSet(),
    ParserDisposition parser_disposition = kParserInserted);

CORE_EXPORT
bool CSPDirectiveListAllowTrustedTypeAssignmentFailure(
    const network::mojom::blink::ContentSecurityPolicy& csp,
    ContentSecurityPolicy* policy,
    const String& message,
    const String& sample,
    const String& sample_prefix,
    std::optional<base::UnguessableToken> issue_id);

CORE_EXPORT
bool CSPDirectiveListAllowTrustedTypePolicy(
    const network::mojom::blink::ContentSecurityPolicy& csp,
    ContentSecurityPolicy* policy,
    const String& policy_name,
    bool is_duplicate,
    ContentSecurityPolicy::AllowTrustedTypePolicyDetails& violation_details,
    std::optional<base::UnguessableToken> issue_id);

CORE_EXPORT
bool CSPDirectiveListRequiresTrustedTypes(
    const network::mojom::blink::ContentSecurityPolicy& csp);

CORE_EXPORT
std::optional<HashAlgorithm> CSPDirectiveListHashToReport(
    const network::mojom::blink::ContentSecurityPolicy& csp);

CORE_EXPORT
bool CSPDirectiveListAllowInline(
    const network::mojom::blink::ContentSecurityPolicy& csp,
    ContentSecurityPolicy* policy,
    ContentSecurityPolicy::InlineType inline_type,
    Element* element,
    const String& content,
    const String& nonce,
    const String& context_url,
    const WTF::OrdinalNumber& context_line,
    ReportingDisposition reporting_disposition);

// Returns whether or not the Javascript code generation should call back the
// CSP checker before any script evaluation from a string is being made.
CORE_EXPORT
bool CSPDirectiveListShouldCheckEval(
    const network::mojom::blink::ContentSecurityPolicy& csp);

CORE_EXPORT
bool CSPDirectiveListAllowEval(
    const network::mojom::blink::ContentSecurityPolicy& csp,
    ContentSecurityPolicy* policy,
    ReportingDisposition reporting_disposition,
    ContentSecurityPolicy::ExceptionStatus exception_status,
    const String& content);

CORE_EXPORT
bool CSPDirectiveListAllowWasmCodeGeneration(
    const network::mojom::blink::ContentSecurityPolicy& csp,
    ContentSecurityPolicy* policy,
    ReportingDisposition reporting_disposition,
    ContentSecurityPolicy::ExceptionStatus exception_status,
    const String& content);

CORE_EXPORT
bool CSPDirectiveListShouldDisableEval(
    const network::mojom::blink::ContentSecurityPolicy& csp,
    String& error_message);

// We need to pass both `csp` and `policy` in because for now, we need to
// ensure the policy supports `wasm-unsafe-eval`.
// TODO(crbug.com/1342523): when we don't need this check, remove the `policy`
// argument here.
CORE_EXPORT
bool CSPDirectiveListShouldDisableWasmEval(
    const network::mojom::blink::ContentSecurityPolicy& csp,
    const ContentSecurityPolicy* policy,
    String& error_message);

CORE_EXPORT
bool CSPDirectiveListAllowDynamic(
    const network::mojom::blink::ContentSecurityPolicy& csp,
    CSPDirectiveName directive_type);

CORE_EXPORT
bool CSPDirectiveListAllowHash(
    const network::mojom::blink::ContentSecurityPolicy& csp,
    const network::mojom::blink::CSPHashSource& hash_value,
    const ContentSecurityPolicy::InlineType inline_type);

// We consider `object-src` restrictions to be reasonable iff they're
// equivalent to `object-src 'none'`.
CORE_EXPORT
bool CSPDirectiveListIsObjectRestrictionReasonable(
    const network::mojom::blink::ContentSecurityPolicy& csp);

// We consider `base-uri` restrictions to be reasonable iff they're equivalent
// to `base-uri 'none'` or `base-uri 'self'`.
CORE_EXPORT
bool CSPDirectiveListIsBaseRestrictionReasonable(
    const network::mojom::blink::ContentSecurityPolicy& csp);

// We consider `script-src` restrictions to be reasonable iff they're not
// URL-based (e.g. they contain only nonces and hashes, or they use
// 'strict-dynamic'). Neither `'unsafe-eval'` nor `'unsafe-hashes'` affect
// this judgement.
CORE_EXPORT
bool CSPDirectiveListIsScriptRestrictionReasonable(
    const network::mojom::blink::ContentSecurityPolicy& csp);

CORE_EXPORT
bool CSPDirectiveListIsActiveForConnections(
    const network::mojom::blink::ContentSecurityPolicy& csp);

// Return the operative directive name and CSPSourceList for a given directive
// name, falling back to generic directives according to Content Security
// Policies rules. For example, if 'default-src' is defined but 'media-src' is
// not, OperativeDirective(CSPDirectiveName::MediaSrc) will return type
// CSPDirectiveName::DefaultSrc and the corresponding CSPSourceList. If no
// operative directive for the given type is defined, this will return
// CSPDirectiveName::Unknown and nullptr.
CORE_EXPORT
CSPOperativeDirective CSPDirectiveListOperativeDirective(
    const network::mojom::blink::ContentSecurityPolicy& csp,
    CSPDirectiveName type);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_CSP_CSP_DIRECTIVE_LIST_H_
