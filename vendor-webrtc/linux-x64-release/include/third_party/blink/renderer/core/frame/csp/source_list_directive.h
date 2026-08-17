// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_CSP_SOURCE_LIST_DIRECTIVE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_CSP_SOURCE_LIST_DIRECTIVE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/frame/csp/content_security_policy.h"
#include "third_party/blink/renderer/core/frame/csp/csp_source.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_request.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class KURL;

CORE_EXPORT
CSPCheckResult CSPSourceListAllows(
    const network::mojom::blink::CSPSourceList& source_list,
    const network::mojom::blink::CSPSource& self_source,
    const KURL&,
    ResourceRequest::RedirectStatus =
        ResourceRequest::RedirectStatus::kNoRedirect);

CORE_EXPORT
bool CSPSourceListAllowNonce(
    const network::mojom::blink::CSPSourceList& source_list,
    const String& nonce);

CORE_EXPORT
bool CSPSourceListAllowHash(
    const network::mojom::blink::CSPSourceList& source_list,
    const network::mojom::blink::CSPHashSource& hash);

CORE_EXPORT
bool CSPSourceListIsNone(
    const network::mojom::blink::CSPSourceList& source_list);

CORE_EXPORT
bool CSPSourceListIsSelf(
    const network::mojom::blink::CSPSourceList& source_list);

CORE_EXPORT
bool CSPSourceListIsHashOrNoncePresent(
    const network::mojom::blink::CSPSourceList& source_list);

CORE_EXPORT
bool CSPSourceListAllowAllInline(
    network::mojom::blink::CSPDirectiveName directive_type,
    ContentSecurityPolicy::InlineType inline_type,
    const network::mojom::blink::CSPSourceList& source_list);

CORE_EXPORT
bool CSPSourceListAllowsURLBasedMatching(
    const network::mojom::blink::CSPSourceList& source_list);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_CSP_SOURCE_LIST_DIRECTIVE_H_
