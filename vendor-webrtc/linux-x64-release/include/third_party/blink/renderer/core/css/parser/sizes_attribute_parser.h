// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_SIZES_ATTRIBUTE_PARSER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_SIZES_ATTRIBUTE_PARSER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/media_values.h"
#include "third_party/blink/renderer/core/css/parser/media_query_parser.h"
#include "third_party/blink/renderer/core/html/html_image_element.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ExecutionContext;

class CORE_EXPORT SizesAttributeParser {
  STACK_ALLOCATED();

 public:
  SizesAttributeParser(MediaValues*,
                       const String&,
                       ExecutionContext*,
                       const HTMLImageElement* = nullptr);

  bool IsAuto();
  float Size();

 private:
  bool Parse(CSSParserTokenStream&);
  float EffectiveSize();
  bool CalculateLengthInPixels(CSSParserTokenStream&, float& result);
  bool MediaConditionMatches(const MediaQuerySet& media_condition);
  float EffectiveSizeDefaultValue();

  MediaValues* media_values_{};
  ExecutionContext* execution_context_{};
  float size_{};
  bool size_was_set_{};
  bool is_valid_{};
  bool is_auto_{};
  const HTMLImageElement* img_{};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_SIZES_ATTRIBUTE_PARSER_H_
