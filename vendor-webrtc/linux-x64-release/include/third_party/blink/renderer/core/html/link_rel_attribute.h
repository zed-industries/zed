/*
 * Copyright (C) 2011 Google Inc. All rights reserved.
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
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_LINK_REL_ATTRIBUTE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_LINK_REL_ATTRIBUTE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/icon_url.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class CORE_EXPORT LinkRelAttribute {
  DISALLOW_NEW();

 public:
  LinkRelAttribute() = default;
  explicit LinkRelAttribute(const String&);

  bool IsStyleSheet() const { return is_style_sheet_; }
  mojom::blink::FaviconIconType GetIconType() const { return icon_type_; }
  bool IsAlternate() const { return is_alternate_; }
  bool IsDNSPrefetch() const { return is_dns_prefetch_; }
  bool IsPreconnect() const { return is_preconnect_; }
  bool IsLinkPrefetch() const { return is_link_prefetch_; }
  bool IsLinkPreload() const { return is_link_preload_; }
  bool IsLinkPrerender() const { return is_link_prerender_; }
  bool IsLinkNext() const { return is_link_next_; }
  bool IsManifest() const { return is_manifest_; }
  bool IsModulePreload() const { return is_module_preload_; }
  bool IsServiceWorker() const { return is_service_worker_; }
  bool IsCanonical() const { return is_canonical_; }
  bool IsMonetization() const { return is_monetization_; }
  bool IsCompressionDictionary() const { return is_compression_dictionary_; }
  bool IsPrivacyPolicy() const { return is_privacy_policy_; }
  bool IsTermsOfService() const { return is_terms_of_service_; }
  bool IsExpect() const { return is_expect_; }
  bool IsFacilitatedPayment() const { return is_facilitated_payment_; }

 private:
  mojom::blink::FaviconIconType icon_type_ =
      mojom::blink::FaviconIconType::kInvalid;
  bool is_style_sheet_ : 1 = false;
  bool is_alternate_ : 1 = false;
  bool is_dns_prefetch_ : 1 = false;
  bool is_preconnect_ : 1 = false;
  bool is_link_prefetch_ : 1 = false;
  bool is_link_preload_ : 1 = false;
  bool is_link_prerender_ : 1 = false;
  bool is_link_next_ : 1 = false;
  bool is_manifest_ : 1 = false;
  bool is_module_preload_ : 1 = false;
  bool is_service_worker_ : 1 = false;
  bool is_canonical_ : 1 = false;
  bool is_monetization_ : 1 = false;
  bool is_compression_dictionary_ : 1 = false;
  bool is_privacy_policy_ : 1 = false;
  bool is_terms_of_service_ : 1 = false;
  bool is_expect_ : 1 = false;
  bool is_facilitated_payment_ : 1 = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_LINK_REL_ATTRIBUTE_H_
