// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LIST_LIST_MARKER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LIST_LIST_MARKER_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/layout_object.h"

namespace blink {

class CounterStyle;
class LayoutListItem;
class LayoutTextFragment;

// This class holds code shared among all classes for list markers, for both
// legacy layout and LayoutNG.
class CORE_EXPORT ListMarker {
  friend class LayoutInlineListItem;
  friend class LayoutListItem;

 public:
  explicit ListMarker();

  static const ListMarker* Get(const LayoutObject*);
  static ListMarker* Get(LayoutObject*);

  static LayoutObject* MarkerFromListItem(const LayoutObject*);

  LayoutObject* ListItem(const LayoutObject&) const;

  String MarkerTextWithSuffix(const LayoutObject&) const;
  String MarkerTextWithoutSuffix(const LayoutObject&) const;

  // Marker text with suffix, e.g. "1. ", for use in accessibility.
  String TextAlternative(const LayoutObject&) const;

  bool IsMarkerImage(const LayoutObject&) const;

  void UpdateMarkerTextIfNeeded(LayoutObject& marker) {
    DCHECK_EQ(Get(&marker), this);
    if (marker_text_type_ == kUnresolved)
      UpdateMarkerText(marker);
  }
  void UpdateMarkerContentIfNeeded(LayoutObject&);

  LayoutObject* SymbolMarkerLayoutText(const LayoutObject&) const;

  // Compute inline margins for 'list-style-position: inside' and 'outside'.
  static std::pair<LayoutUnit, LayoutUnit> InlineMarginsForInside(
      Document&,
      const ComputedStyleBuilder& marker_style_builder,
      const ComputedStyle& list_item_style);
  static std::pair<LayoutUnit, LayoutUnit> InlineMarginsForOutside(
      Document&,
      const ComputedStyle& marker_style,
      const ComputedStyle& list_item_style,
      LayoutUnit marker_inline_size);

  static PhysicalRect RelativeSymbolMarkerRect(const ComputedStyle&,
                                               const AtomicString& list_style,
                                               LayoutUnit);
  static LayoutUnit WidthOfSymbol(const ComputedStyle&,
                                  const AtomicString& list_style);

  // A reduced set of list style categories allowing for more concise expression
  // of list style specific logic.
  enum class ListStyleCategory { kNone, kSymbol, kLanguage, kStaticString };

  // Returns the list's style as one of a reduced high level categorical set of
  // styles.
  static ListStyleCategory GetListStyleCategory(Document&,
                                                const ComputedStyle&);

  static const CounterStyle& GetCounterStyle(Document&, const ComputedStyle&);

 private:
  enum MarkerTextFormat {
    kWithPrefixSuffix,
    kWithoutPrefixSuffix,
    kAlternativeText
  };
  enum MarkerTextType {
    kNotText,  // The marker doesn't have a LayoutText, either because it has
               // not been created yet or because 'list-style-type' is 'none',
               // 'list-style-image' is not 'none', or 'content' is not
               // 'normal'.
    kUnresolved,    // The marker has a LayoutText that needs to be updated.
    kOrdinalValue,  // The marker text depends on the ordinal.
    kStatic,        // The marker text doesn't depend on the ordinal.
    kSymbolValue,   // Like kStatic, but the marker is painted as a symbol.
  };
  MarkerTextType MarkerText(const LayoutObject&,
                            StringBuilder*,
                            MarkerTextFormat) const;
  void UpdateMarkerText(LayoutObject&);

  void ListStyleTypeChanged(LayoutObject&);
  void OrdinalValueChanged(LayoutObject&);
  void CounterStyleChanged(LayoutObject&);

  int ListItemValue(const LayoutObject&) const;

  LayoutTextFragment& GetTextChild(const LayoutObject& marker) const;
  LayoutObject* GetContentChild(const LayoutObject& marker) const;

  unsigned marker_text_type_ : 3;  // MarkerTextType

  friend class StyleEngineTest;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LIST_LIST_MARKER_H_
