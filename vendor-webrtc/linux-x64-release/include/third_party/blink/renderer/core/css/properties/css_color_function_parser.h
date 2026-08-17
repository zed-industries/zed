// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTIES_CSS_COLOR_FUNCTION_PARSER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTIES_CSS_COLOR_FUNCTION_PARSER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/color_function.h"
#include "third_party/blink/renderer/core/css/css_color_channel_map.h"
#include "third_party/blink/renderer/core/css/parser/css_parser_context.h"
#include "third_party/blink/renderer/core/css/parser/css_parser_token_stream.h"
#include "third_party/blink/renderer/platform/graphics/color.h"

namespace blink {

class CSSValue;

class CORE_EXPORT ColorFunctionParser {
  STACK_ALLOCATED();

 public:
  ColorFunctionParser() = default;
  // Parses the color inputs rgb(), rgba(), hsl(), hsla(), hwb(), lab(),
  // oklab(), lch(), oklch() and color(). https://www.w3.org/TR/css-color-4/
  CSSValue* ConsumeFunctionalSyntaxColor(CSSParserTokenStream& stream,
                                         const CSSParserContext& context);

  // These are exposed so that StyleColor::UnresolvedRelativeColor
  // or similar can reuse our logic.
  enum class ChannelType { kNone, kPercentage, kNumber, kRelative };
  static void MakePerColorSpaceAdjustments(
      bool is_relative_color,
      bool is_legacy_syntax,
      Color::ColorSpace color_space,
      std::array<std::optional<double>, 3>& channels,
      std::optional<double>& alpha);

 private:
  bool ConsumeColorSpaceAndOriginColor(CSSParserTokenStream& stream,
                                       CSSValueID function_id,
                                       const CSSParserContext& context);
  bool ConsumeChannel(CSSParserTokenStream& stream,
                      const CSSParserContext& context,
                      int index);
  bool ConsumeAlpha(CSSParserTokenStream& stream,
                    const CSSParserContext& context);
  void MakePerColorSpaceAdjustments();

  static double ResolveColorChannel(
      const CSSValue* value,
      ChannelType channel_type,
      double percentage_base,
      const CSSColorChannelMap& color_channel_map);
  static double ResolveAlpha(const CSSValue* value,
                             ChannelType channel_type,
                             const CSSColorChannelMap& color_channel_map);
  static double ResolveRelativeChannelValue(
      const CSSValue* value,
      ChannelType channel_type,
      double percentage_base,
      const CSSColorChannelMap& color_channel_map);

  bool IsRelativeColor() const;
  bool AllChannelsAreResolvable() const;

  Color::ColorSpace color_space_ = Color::ColorSpace::kNone;
  std::array<const CSSValue*, 3> unresolved_channels_;
  std::array<ChannelType, 3> channel_types_;
  const CSSValue* unresolved_alpha_ = nullptr;
  ChannelType alpha_channel_type_;

  // Metadata about the current function being parsed. Set by
  // `ConsumeColorSpaceAndOriginColor()` after parsing the preamble of the
  // function.
  const ColorFunction::Metadata* function_metadata_ = nullptr;

  // Legacy colors have commas separating their channels. This syntax is
  // incompatible with CSSColor4 features like "none" or alpha with a slash.
  bool is_legacy_syntax_ = false;
  bool has_none_ = false;

  // For relative colors
  const CSSValue* unresolved_origin_color_ = nullptr;
  std::optional<Color> origin_color_;
  CSSColorChannelMap color_channel_map_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTIES_CSS_COLOR_FUNCTION_PARSER_H_
