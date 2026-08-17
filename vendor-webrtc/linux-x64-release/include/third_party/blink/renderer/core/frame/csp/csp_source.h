// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_CSP_CSP_SOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_CSP_CSP_SOURCE_H_

#include "services/network/public/mojom/content_security_policy.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_request.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class KURL;

CORE_EXPORT
bool CSPSourceIsSchemeOnly(const network::mojom::blink::CSPSource& source);

CORE_EXPORT
bool CSPSourceMatches(const network::mojom::blink::CSPSource& source,
                      const String& self_protocol,
                      const KURL& url,
                      ResourceRequest::RedirectStatus =
                          ResourceRequest::RedirectStatus::kNoRedirect);

CORE_EXPORT
bool CSPSourceMatchesAsSelf(const network::mojom::blink::CSPSource& source,
                            const KURL& url);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_CSP_CSP_SOURCE_H_
