/*
 * Copyright (C) 2011 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 * 3.  Neither the name of Google, Inc. ("Google") nor the names of
 *     its contributors may be used to endorse or promote products derived
 *     from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY GOOGLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WEBORIGIN_SECURITY_POLICY_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WEBORIGIN_SECURITY_POLICY_H_

#include "services/network/public/mojom/cors_origin_pattern.mojom-blink-forward.h"
#include "services/network/public/mojom/referrer_policy.mojom-blink.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/weborigin/referrer.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class KURL;
class SecurityOrigin;

enum ReferrerPolicyLegacyKeywordsSupport {
  kSupportReferrerPolicyLegacyKeywords,
  kDoNotSupportReferrerPolicyLegacyKeywords,
};

class PLATFORM_EXPORT SecurityPolicy {
  STATIC_ONLY(SecurityPolicy);

 public:
  // True if the referrer should be omitted according to the
  // ReferrerPolicyNoReferrerWhenDowngrade. If you intend to send a
  // referrer header, you should use generateReferrer instead.
  static bool ShouldHideReferrer(const KURL&, const KURL& referrer);

  // Returns the referrer modified according to the referrer policy for a
  // navigation to a given URL. If the referrer returned is empty, the
  // referrer header should be omitted.
  static Referrer GenerateReferrer(network::mojom::ReferrerPolicy,
                                   const KURL&,
                                   const String& referrer);

  static void AddOriginAccessAllowListEntry(
      const SecurityOrigin& source_origin,
      const String& destination_protocol,
      const String& destination_domain,
      const uint16_t port,
      const network::mojom::CorsDomainMatchMode domain_match_mode,
      const network::mojom::CorsPortMatchMode port_match_mode,
      const network::mojom::CorsOriginAccessMatchPriority priority);
  static void AddOriginAccessBlockListEntry(
      const SecurityOrigin& source_origin,
      const String& destination_protocol,
      const String& destination_domain,
      const uint16_t port,
      const network::mojom::CorsDomainMatchMode domain_match_mode,
      const network::mojom::CorsPortMatchMode port_match_mode,
      const network::mojom::CorsOriginAccessMatchPriority priority);
  static void ClearOriginAccessListForOrigin(
      const SecurityOrigin& source_origin);
  static void ClearOriginAccessList();

  static bool IsOriginAccessAllowed(const SecurityOrigin* active_origin,
                                    const SecurityOrigin* target_origin);
  static bool IsOriginAccessToURLAllowed(const SecurityOrigin* active_origin,
                                         const KURL&);

  static bool ReferrerPolicyFromString(const String& policy,
                                       ReferrerPolicyLegacyKeywordsSupport,
                                       network::mojom::ReferrerPolicy* result);
  static String ReferrerPolicyAsString(network::mojom::ReferrerPolicy policy);

  static bool ReferrerPolicyFromHeaderValue(
      const String& header_value,
      ReferrerPolicyLegacyKeywordsSupport,
      network::mojom::ReferrerPolicy* result);

  static bool IsSharedArrayBufferAlwaysAllowedForOrigin(
      const SecurityOrigin* origin);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WEBORIGIN_SECURITY_POLICY_H_
