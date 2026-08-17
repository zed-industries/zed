// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTIES_CSS_DIRECTION_AWARE_RESOLVER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTIES_CSS_DIRECTION_AWARE_RESOLVER_H_

#include "third_party/blink/renderer/platform/text/writing_direction_mode.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

enum class CSSPropertyID;
class CSSProperty;
class StylePropertyShorthand;

class CSSDirectionAwareResolver {
  STATIC_ONLY(CSSDirectionAwareResolver);

 private:
  template <size_t size>
  class Group {
   public:
    explicit Group(const StylePropertyShorthand&);
    explicit Group(const CSSProperty* (&properties)[size]);
    const CSSProperty& GetProperty(size_t index) const;
    bool Contains(CSSPropertyID) const;

   private:
    const CSSProperty* const* properties_;
  };

 public:
  // A group of logical properties that's used by the 'Resolve*' functions
  // to convert a physical property into a direction-aware property.
  // It represents the properties in a logical property group [1] with
  // a flow-relative mapping logic [2].
  // [1]: https://drafts.csswg.org/css-logical/#logical-property-group
  // [2]: https://drafts.csswg.org/css-logical/#mapping-logic
  template <size_t size>
  class LogicalMapping : public Group<size> {
    using Group<size>::Group;
  };

  // A group of physical properties that's used by the 'Resolve*' functions
  // to convert a direction-aware property into a physical property.
  // It represents the properties in a logical property group [1] with
  // a physical mapping logic [2].
  // [1]: https://drafts.csswg.org/css-logical/#logical-property-group
  // [2]: https://drafts.csswg.org/css-logical/#mapping-logic
  template <size_t size>
  class PhysicalMapping : public Group<size> {
    using Group<size>::Group;
  };

  static LogicalMapping<4> LogicalBorderMapping();
  static LogicalMapping<4> LogicalBorderColorMapping();
  static LogicalMapping<4> LogicalBorderRadiusMapping();
  static LogicalMapping<4> LogicalBorderStyleMapping();
  static LogicalMapping<4> LogicalBorderWidthMapping();
  static LogicalMapping<4> LogicalCornerShapeMapping();
  static LogicalMapping<4> LogicalInsetMapping();
  static LogicalMapping<4> LogicalMarginMapping();
  static LogicalMapping<2> LogicalMaxSizeMapping();
  static LogicalMapping<2> LogicalMinSizeMapping();
  static LogicalMapping<2> LogicalOverflowMapping();
  static LogicalMapping<2> LogicalOverscrollBehaviorMapping();
  static LogicalMapping<4> LogicalPaddingMapping();
  static LogicalMapping<4> LogicalScrollMarginMapping();
  static LogicalMapping<4> LogicalScrollPaddingMapping();
  static LogicalMapping<2> LogicalScrollStartMapping();
  static LogicalMapping<2> LogicalSizeMapping();
  static LogicalMapping<4> LogicalVisitedBorderColorMapping();

  static PhysicalMapping<4> PhysicalBorderMapping();
  static PhysicalMapping<4> PhysicalBorderColorMapping();
  static PhysicalMapping<4> PhysicalBorderRadiusMapping();
  static PhysicalMapping<4> PhysicalBorderStyleMapping();
  static PhysicalMapping<4> PhysicalBorderWidthMapping();
  static PhysicalMapping<2> PhysicalContainIntrinsicSizeMapping();
  static PhysicalMapping<4> PhysicalCornerShapeMapping();
  static PhysicalMapping<4> PhysicalInsetMapping();
  static PhysicalMapping<4> PhysicalMarginMapping();
  static PhysicalMapping<2> PhysicalMaxSizeMapping();
  static PhysicalMapping<2> PhysicalMinSizeMapping();
  static PhysicalMapping<2> PhysicalOverflowMapping();
  static PhysicalMapping<2> PhysicalOverscrollBehaviorMapping();
  static PhysicalMapping<4> PhysicalPaddingMapping();
  static PhysicalMapping<4> PhysicalScrollMarginMapping();
  static PhysicalMapping<4> PhysicalScrollPaddingMapping();
  static PhysicalMapping<2> PhysicalScrollStartMapping();
  static PhysicalMapping<2> PhysicalSizeMapping();
  static PhysicalMapping<4> PhysicalVisitedBorderColorMapping();

  // These resolvers expect a PhysicalMapping with box sides, in the following
  // order: top, right, bottom, left.
  static const CSSProperty& ResolveInlineStart(WritingDirectionMode,
                                               const PhysicalMapping<4>&);
  static const CSSProperty& ResolveInlineEnd(WritingDirectionMode,
                                             const PhysicalMapping<4>&);
  static const CSSProperty& ResolveBlockStart(WritingDirectionMode,
                                              const PhysicalMapping<4>&);
  static const CSSProperty& ResolveBlockEnd(WritingDirectionMode,
                                            const PhysicalMapping<4>&);

  // These resolvers expect a LogicalMapping with box sides, in the following
  // order: block-start, block-end, inline-start, inline-end.
  static const CSSProperty& ResolveTop(WritingDirectionMode,
                                       const LogicalMapping<4>&);
  static const CSSProperty& ResolveBottom(WritingDirectionMode,
                                          const LogicalMapping<4>&);
  static const CSSProperty& ResolveLeft(WritingDirectionMode,
                                        const LogicalMapping<4>&);
  static const CSSProperty& ResolveRight(WritingDirectionMode,
                                         const LogicalMapping<4>&);

  // These resolvers expect a PhysicalMapping with dimensions, in the following
  // order: horizontal, vertical.
  static const CSSProperty& ResolveInline(WritingDirectionMode,
                                          const PhysicalMapping<2>&);
  static const CSSProperty& ResolveBlock(WritingDirectionMode,
                                         const PhysicalMapping<2>&);

  // These resolvers expect a LogicalMapping with dimensions, in the following
  // order: block, inline.
  static const CSSProperty& ResolveHorizontal(WritingDirectionMode,
                                              const LogicalMapping<2>&);
  static const CSSProperty& ResolveVertical(WritingDirectionMode,
                                            const LogicalMapping<2>&);

  // These resolvers expect a a PhysicalMapping with box corners, in the
  // following order: top-left, top-right, bottom-right, bottom-left.
  static const CSSProperty& ResolveStartStart(WritingDirectionMode,
                                              const PhysicalMapping<4>&);
  static const CSSProperty& ResolveStartEnd(WritingDirectionMode,
                                            const PhysicalMapping<4>&);
  static const CSSProperty& ResolveEndStart(WritingDirectionMode,
                                            const PhysicalMapping<4>&);
  static const CSSProperty& ResolveEndEnd(WritingDirectionMode,
                                          const PhysicalMapping<4>&);

  // These resolvers expect a a LogicalMapping with box corners, in the
  // following order: start-start, start-end, end-start, end-end.
  static const CSSProperty& ResolveTopLeft(WritingDirectionMode,
                                           const LogicalMapping<4>&);
  static const CSSProperty& ResolveTopRight(WritingDirectionMode,
                                            const LogicalMapping<4>&);
  static const CSSProperty& ResolveBottomRight(WritingDirectionMode,
                                               const LogicalMapping<4>&);
  static const CSSProperty& ResolveBottomLeft(WritingDirectionMode,
                                              const LogicalMapping<4>&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTIES_CSS_DIRECTION_AWARE_RESOLVER_H_
