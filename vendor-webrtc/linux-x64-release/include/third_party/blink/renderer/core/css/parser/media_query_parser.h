// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_MEDIA_QUERY_PARSER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_MEDIA_QUERY_PARSER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/media_list.h"
#include "third_party/blink/renderer/core/css/media_query.h"
#include "third_party/blink/renderer/core/css/media_query_exp.h"
#include "third_party/blink/renderer/core/css/parser/css_parser_mode.h"
#include "third_party/blink/renderer/core/css/parser/css_parser_token.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class MediaQuerySet;
class CSSParserContext;
class ContainerQueryParser;
class CSSIfParser;

class CORE_EXPORT MediaQueryParser {
  STACK_ALLOCATED();

 public:
  MediaQueryParser(const MediaQueryParser&) = delete;
  MediaQueryParser& operator=(const MediaQueryParser&) = delete;

  static MediaQuerySet* ParseMediaQuerySet(StringView, ExecutionContext*);
  static MediaQuerySet* ParseMediaQuerySet(CSSParserTokenStream&,
                                           ExecutionContext*);
  static MediaQuerySet* ParseMediaCondition(CSSParserTokenStream&,
                                            ExecutionContext*);
  static MediaQuerySet* ParseMediaQuerySetInMode(CSSParserTokenStream&,
                                                 CSSParserMode,
                                                 ExecutionContext*);

  // Passed to ConsumeFeature to determine which features are allowed.
  class FeatureSet {
    STACK_ALLOCATED();

   public:
    // Returns true if the feature name is allowed in this set.
    virtual bool IsAllowed(const AtomicString& feature) const = 0;

    // Returns true if the feature can be queried without a value.
    virtual bool IsAllowedWithoutValue(const AtomicString& feature,
                                       const ExecutionContext*) const = 0;

    // Returns true is the feature name is case sensitive.
    virtual bool IsCaseSensitive(const AtomicString& feature) const = 0;

    // Whether the features support range syntax. This is typically false for
    // style container queries.
    virtual bool SupportsRange() const = 0;

    // Whether the features are evaluated in an element context
    // (true for container queries, false for media queries).
    virtual bool SupportsElementDependent() const = 0;
  };

  class MediaQueryFeatureSet : public MediaQueryParser::FeatureSet {
    STACK_ALLOCATED();

   public:
    MediaQueryFeatureSet() = default;

    bool IsAllowed(const AtomicString& feature) const override;
    bool IsAllowedWithoutValue(
        const AtomicString& feature,
        const ExecutionContext* execution_context) const override;
    bool IsCaseSensitive(const AtomicString& feature) const override {
      return false;
    }
    bool SupportsRange() const override { return true; }
    bool SupportsElementDependent() const override { return false; }
  };

 private:
  friend class ContainerQueryParser;
  friend class CSSIfParser;

  enum ParserType {
    kMediaQuerySetParser,
    kMediaConditionParser,
  };

  enum class SyntaxLevel {
    // Determined by CSSMediaQueries4 flag.
    kAuto,
    // Use mediaqueries-4 syntax regardless of flags.
    kLevel4,
  };

  MediaQueryParser(ParserType,
                   CSSParserMode,
                   ExecutionContext*,
                   SyntaxLevel = SyntaxLevel::kAuto);

  // [ not | only ]
  static MediaQuery::RestrictorType ConsumeRestrictor(CSSParserTokenStream&);

  // https://drafts.csswg.org/mediaqueries-4/#typedef-media-type
  static AtomicString ConsumeType(CSSParserTokenStream&);

  // https://drafts.csswg.org/mediaqueries-4/#typedef-mf-comparison
  static MediaQueryOperator ConsumeComparison(CSSParserTokenStream&);

  // https://drafts.csswg.org/mediaqueries-4/#typedef-mf-name
  //
  // The <mf-name> is only consumed if the name is allowed by the specified
  // FeatureSet.
  AtomicString ConsumeAllowedName(CSSParserTokenStream&, const FeatureSet&);

  // Like ConsumeAllowedName, except returns null if the name has a min-
  // or max- prefix.
  AtomicString ConsumeUnprefixedName(CSSParserTokenStream&, const FeatureSet&);

  enum class NameAffinity {
    // <mf-name> appears on the left, e.g. width < 10px.
    kLeft,
    // <mf-name> appears on the right, e.g. 10px > width.
    kRight
  };

  // https://drafts.csswg.org/mediaqueries-4/#typedef-media-feature
  //
  // Currently, only <mf-boolean> and <mf-plain> productions are supported.
  const MediaQueryExpNode* ConsumeFeature(CSSParserTokenStream&,
                                          const FeatureSet&);

  enum class ConditionMode {
    // https://drafts.csswg.org/mediaqueries-4/#typedef-media-condition
    kNormal,
    // https://drafts.csswg.org/mediaqueries-4/#typedef-media-condition-without-or
    kWithoutOr,
  };

  // https://drafts.csswg.org/mediaqueries-4/#typedef-media-condition
  const MediaQueryExpNode* ConsumeCondition(
      CSSParserTokenStream&,
      ConditionMode = ConditionMode::kNormal);

  // https://drafts.csswg.org/mediaqueries-4/#typedef-media-in-parens
  const MediaQueryExpNode* ConsumeInParens(CSSParserTokenStream&);

  // https://drafts.csswg.org/mediaqueries-4/#typedef-general-enclosed
  const MediaQueryExpNode* ConsumeGeneralEnclosed(CSSParserTokenStream&);

  // https://drafts.csswg.org/mediaqueries-4/#typedef-media-query
  MediaQuery* ConsumeQuery(CSSParserTokenStream&);

  // Used for ParserType::kMediaConditionParser.
  //
  // Parsing a single condition is useful for the 'sizes' attribute.
  //
  // https://html.spec.whatwg.org/multipage/images.html#sizes-attribute
  MediaQuerySet* ConsumeSingleCondition(CSSParserTokenStream&);

  MediaQuerySet* ParseImpl(CSSParserTokenStream&);

  void UseCountRangeSyntax();

  ParserType parser_type_;
  CSSParserMode mode_;
  ExecutionContext* execution_context_;
  SyntaxLevel syntax_level_;
  // A fake CSSParserContext for use counter only.
  // TODO(xiaochengh): Plumb the real CSSParserContext from the document.
  const CSSParserContext& fake_context_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_MEDIA_QUERY_PARSER_H_
