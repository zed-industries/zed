/*
 * Copyright (C) 2009 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_SECURITY_POLICY_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_SECURITY_POLICY_H_

#include "services/network/public/mojom/cors_origin_pattern.mojom-shared.h"
#include "services/network/public/mojom/referrer_policy.mojom-shared.h"
#include "third_party/blink/public/platform/web_common.h"

namespace blink {

class WebString;
class WebURL;

class BLINK_EXPORT WebSecurityPolicy {
 public:
  // Registers a URL scheme to be treated as display-isolated. This means
  // that pages cannot display these URLs unless they are from the same
  // scheme. For example, pages in other origin cannot create iframes or
  // hyperlinks to URLs with the scheme.
  static void RegisterURLSchemeAsDisplayIsolated(const WebString&);

  // Registers a URL scheme that can register a ServiceWorker.
  static void RegisterURLSchemeAsAllowingServiceWorkers(const WebString&);

  // Registers an URL scheme as allowing the not-yet-standardized 'wasm-eval'
  // CSP source directive.
  static void RegisterURLSchemeAsAllowingWasmEvalCSP(const WebString&);

  // Registers an HTTP-like URL scheme that supports the Fetch API.
  static void RegisterURLSchemeAsSupportingFetchAPI(const WebString&);

  // Registers a URL scheme which will always be considered the first-party when
  // loaded in a top-level context.
  static void RegisterURLSchemeAsFirstPartyWhenTopLevel(const WebString&);

  // Registers a URL scheme which will be considered first-party when loaded in
  // a top-level context for child contexts which were loaded over secure
  // schemes.
  static void RegisterURLSchemeAsFirstPartyWhenTopLevelEmbeddingSecure(
      const WebString&);

  // Registers a URL scheme as always allowing access to SharedArrayBuffers.
  // TODO(crbug.com/1184892): Remove once fixed.
  static void RegisterURLSchemeAsAllowingSharedArrayBuffers(const WebString&);

  // Support for managing allow/block access lists to origins beyond the
  // same-origin policy. The block list takes priority over the allow list.
  // When an origin matches an entry on both the allow list and block list
  // the entry with the higher priority defines whether access is allowed.
  // In the case where both an allowlist and blocklist rule of the same
  // priority match a request the blocklist rule takes priority.
  // Callers should use
  // network::mojom::CorsOriginAccessMatchPriority::kDefaultPriority as the
  // default priority unless overriding existing entries is explicitly needed.
  static void AddOriginAccessAllowListEntry(
      const WebURL& source_origin,
      const WebString& destination_protocol,
      const WebString& destination_host,
      const uint16_t destination_port,
      network::mojom::CorsDomainMatchMode domain_match_mode,
      network::mojom::CorsPortMatchMode port_match_mode,
      const network::mojom::CorsOriginAccessMatchPriority priority);
  static void AddOriginAccessBlockListEntry(
      const WebURL& source_origin,
      const WebString& destination_protocol,
      const WebString& destination_host,
      const uint16_t destination_port,
      network::mojom::CorsDomainMatchMode domain_match_mode,
      network::mojom::CorsPortMatchMode port_match_mode,
      const network::mojom::CorsOriginAccessMatchPriority priority);
  static void ClearOriginAccessListForOrigin(const WebURL& source_origin);
  static void ClearOriginAccessList();

  // Add a scheme that is always considered a secure context. The caller is
  // responsible for canonicalizing the input.
  static void AddSchemeToSecureContextSafelist(const WebString&);

  // Returns the referrer modified according to the referrer policy for a
  // navigation to a given URL. If the referrer returned is empty, the
  // referrer header should be omitted.
  static WebString GenerateReferrerHeader(network::mojom::ReferrerPolicy,
                                          const WebURL&,
                                          const WebString& referrer);

  // Registers an URL scheme to not allow manipulation of the loaded page
  // by bookmarklets or javascript: URLs typed in the omnibox.
  static void RegisterURLSchemeAsNotAllowingJavascriptURLs(const WebString&);

  // Registers an URL scheme as allowed in referrers.
  static void RegisterURLSchemeAsAllowedForReferrer(const WebString&);

  // Registers an URL scheme as an error page.
  static void RegisterURLSchemeAsError(const WebString&);

  // Registers an URL scheme as a browser extension.
  static void RegisterURLSchemeAsExtension(const WebString&);

  // Registers an URL scheme as trusted browser UI.
  static void RegisterURLSchemeAsWebUI(const WebString&);

  // Registers an URL scheme which can use code caching but must check in the
  // renderer whether the script content has changed rather than relying on a
  // response time match from the network cache.
  static void RegisterURLSchemeAsCodeCacheWithHashing(const WebString&);

  // Registers a WebUI URL scheme which can use bundled resource bytecode,
  // retrieved from the application's static resource bundle. This is only used
  // for renderer-side checks (the concept does not generalize beyond the
  // renderer). As a result there is no need to mirror this policy into
  // ChildProcessSecurityPolicy or browser-side scheme registration.
  static void RegisterURLSchemeAsWebUIBundledBytecode(const WebString&);

 private:
  WebSecurityPolicy() = delete;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_SECURITY_POLICY_H_
