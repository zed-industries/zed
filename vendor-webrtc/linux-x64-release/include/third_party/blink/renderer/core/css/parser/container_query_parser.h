// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CONTAINER_QUERY_PARSER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CONTAINER_QUERY_PARSER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/container_query.h"
#include "third_party/blink/renderer/core/css/media_query_exp.h"
#include "third_party/blink/renderer/core/css/parser/css_variable_parser.h"
#include "third_party/blink/renderer/core/css/parser/media_query_parser.h"

namespace blink {

class CSSParserContext;
class CSSIfParser;

class CORE_EXPORT ContainerQueryParser {
  STACK_ALLOCATED();

 public:
  explicit ContainerQueryParser(const CSSParserContext&);

  // https://drafts.csswg.org/css-contain-3/#typedef-container-condition
  const MediaQueryExpNode* ParseCondition(String);
  const MediaQueryExpNode* ParseCondition(CSSParserTokenStream&);

  class StyleFeatureSet : public MediaQueryParser::FeatureSet {
    STACK_ALLOCATED();

   public:
    bool IsAllowed(const AtomicString& feature) const override {
      // TODO(crbug.com/40217044): Only support querying custom properties for
      // now.
      return CSSVariableParser::IsValidVariableName(feature);
    }
    bool IsAllowedWithoutValue(const AtomicString& feature,
                               const ExecutionContext*) const override {
      return true;
    }
    bool IsCaseSensitive(const AtomicString& feature) const override {
      // TODO(crbug.com/40217044): non-custom properties are case-insensitive.
      return true;
    }
    bool SupportsRange() const override { return false; }
    bool SupportsElementDependent() const override { return true; }
  };

 private:
  friend class ContainerQueryParserTest;
  friend class CSSIfParser;

  using FeatureSet = MediaQueryParser::FeatureSet;

  const MediaQueryExpNode* ConsumeQueryInParens(CSSParserTokenStream&);
  const MediaQueryExpNode* ConsumeContainerCondition(CSSParserTokenStream&);
  const MediaQueryExpNode* ConsumeFeatureQuery(CSSParserTokenStream&,
                                               const FeatureSet&);
  const MediaQueryExpNode* ConsumeFeatureQueryInParens(CSSParserTokenStream&,
                                                       const FeatureSet&);
  const MediaQueryExpNode* ConsumeFeatureCondition(CSSParserTokenStream&,
                                                   const FeatureSet&);
  const MediaQueryExpNode* ConsumeFeature(CSSParserTokenStream&,
                                          const FeatureSet&);

  const CSSParserContext& context_;
  MediaQueryParser media_query_parser_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CONTAINER_QUERY_PARSER_H_
