/*
 * Copyright (C) 2008 Apple Inc. All Rights Reserved.
 * Copyright (C) 2010 Google Inc. All Rights Reserved.
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
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_CSS_PRELOAD_SCANNER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_CSS_PRELOAD_SCANNER_H_

#include "third_party/blink/renderer/core/html/parser/html_token.h"
#include "third_party/blink/renderer/core/html/parser/preload_request.h"
#include "third_party/blink/renderer/platform/wtf/text/string_builder.h"

namespace blink {

class SegmentedString;

class CSSPreloadScanner {
  DISALLOW_NEW();

 public:
  CSSPreloadScanner();
  CSSPreloadScanner(const CSSPreloadScanner&) = delete;
  CSSPreloadScanner& operator=(const CSSPreloadScanner&) = delete;
  ~CSSPreloadScanner();

  void Reset();

  void Scan(const HTMLToken::DataVector&,
            const SegmentedString&,
            PreloadRequestStream&,
            const KURL&,
            const PreloadRequest::ExclusionInfo*);
  void Scan(const String&,
            const SegmentedString&,
            PreloadRequestStream&,
            const KURL&,
            const PreloadRequest::ExclusionInfo*);

  void SetReferrerPolicy(network::mojom::ReferrerPolicy);
  void SetInBody(bool in_body) { in_body_ = in_body; }
  void SetMediaMatches(bool media_matches) { media_matches_ = media_matches; }

 private:
  enum State {
    kInitial,
    kMaybeComment,
    kComment,
    kMaybeCommentEnd,
    kRuleStart,
    kRule,
    kAfterRule,
    kRuleValue,
    kAfterRuleValue,
    kMaybeLayerValue,
    kAfterMaybeLayerValue,
    kDoneParsingImportRules,
  };

  template <typename Char>
  void ScanCommon(base::span<const Char>,
                  const SegmentedString&,
                  PreloadRequestStream&,
                  const KURL&,
                  const PreloadRequest::ExclusionInfo*);

  inline void Tokenize(UChar, const SegmentedString&);
  void EmitRule(const SegmentedString&);

  bool HasFinishedRuleValue() const;
  bool CanPreloadImportRule() const;

  State state_ = kInitial;
  StringBuilder rule_;
  StringBuilder rule_value_;
  // maybe_layer_value_ stores the layer declaration, if any; otherwise, it may
  // store part of the media condition. This is currently fine since we can't
  // handle media conditions yet (crbug.com/1277771).
  StringBuilder maybe_layer_value_;
  bool has_trailing_contents_ = false;

  network::mojom::ReferrerPolicy referrer_policy_ =
      network::mojom::ReferrerPolicy::kDefault;
  bool in_body_ = false;
  bool media_matches_ = true;

  // Below members only non-null during scan()
  PreloadRequestStream* requests_ = nullptr;
  const KURL* predicted_base_element_url_ = nullptr;
  const PreloadRequest::ExclusionInfo* exclusion_info_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_CSS_PRELOAD_SCANNER_H_
