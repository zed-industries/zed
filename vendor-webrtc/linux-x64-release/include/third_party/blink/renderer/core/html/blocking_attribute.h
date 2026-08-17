// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_BLOCKING_ATTRIBUTE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_BLOCKING_ATTRIBUTE_H_

#include "third_party/blink/renderer/core/dom/dom_token_list.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/html_names.h"
#include "third_party/blink/renderer/core/keywords.h"
#include "third_party/blink/renderer/core/loader/render_blocking_level.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

// https://html.spec.whatwg.org/#blocking-attribute
class BlockingAttribute final : public DOMTokenList {
 public:
  explicit BlockingAttribute(Element* element)
      : DOMTokenList(*element, html_names::kBlockingAttr) {}

  static bool HasRenderToken(const String& attribute_value);
  static bool HasFullFrameRateToken(const String& attribute_value);
  bool HasRenderToken() const { return contains(keywords::kRender); }
  bool HasFullFrameRateToken() const {
    return contains(keywords::kFullFrameRate);
  }
  RenderBlockingLevel GetBlockingLevel() const;

  void OnAttributeValueChanged(const AtomicString& old_value,
                               const AtomicString& new_value);

 private:
  HashSet<AtomicString>& SupportedTokens() const;

  bool RenderBlockingFullFrameRateEnabled() const;

  bool ValidateTokenValue(const AtomicString&, ExceptionState&) const override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_BLOCKING_ATTRIBUTE_H_
