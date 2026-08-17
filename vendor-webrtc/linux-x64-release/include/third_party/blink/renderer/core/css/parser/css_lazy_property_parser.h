// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_LAZY_PROPERTY_PARSER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_LAZY_PROPERTY_PARSER_H_

#include "third_party/blink/renderer/core/css/css_property_value_set.h"
#include "third_party/blink/renderer/core/css/parser/css_tokenizer.h"

namespace blink {

class CSSLazyParsingState;

// This class is responsible for lazily parsing a single CSS declaration list.
class CSSLazyPropertyParser : public GarbageCollected<CSSLazyPropertyParser> {
 public:
  CSSLazyPropertyParser(wtf_size_t offset, CSSLazyParsingState*);

  // CSSLazyPropertyParser:
  CSSPropertyValueSet* ParseProperties();

  void Trace(Visitor* visitor) const;

 private:
  wtf_size_t offset_;
  Member<CSSLazyParsingState> lazy_state_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_LAZY_PROPERTY_PARSER_H_
