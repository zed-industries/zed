/*
 * Copyright (C) 2011 Nokia Inc. All rights reserved.
 * Copyright (C) 2012 Google Inc. All rights reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_QUOTE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_QUOTE_H_

#include "third_party/blink/renderer/core/layout/layout_inline.h"
#include "third_party/blink/renderer/platform/text/quotes_data.h"

namespace blink {

class StyleContainmentScope;
class LayoutTextFragment;
class PseudoElement;

// LayoutQuote is the layout object associated with generated quotes
// ("content: open-quote | close-quote | no-open-quote | no-close-quote").
// http://www.w3.org/TR/CSS2/generate.html#quotes-insert
//
// This object is generated thus always anonymous.
class LayoutQuote final : public LayoutInline {
 public:
  LayoutQuote(LayoutObject& owner, const QuoteType);
  ~LayoutQuote() override;
  void Trace(Visitor*) const override;

  // Will return nullptr, if this doesn't originate from a pseudo element, but
  // rather an @page margin box.
  PseudoElement* GetOwningPseudo() const {
    NOT_DESTROYED();
    return owning_pseudo_.Get();
  }
  bool IsInScope() const {
    NOT_DESTROYED();
    return !!scope_;
  }
  StyleContainmentScope* GetScope() const {
    NOT_DESTROYED();
    return scope_.Get();
  }
  void SetScope(StyleContainmentScope* scope) {
    NOT_DESTROYED();
    scope_ = scope;
  }

  int GetDepth() const {
    NOT_DESTROYED();
    return depth_;
  }
  int GetNextDepth() const {
    NOT_DESTROYED();
    return type_ == QuoteType::kOpen || type_ == QuoteType::kNoOpen
               ? depth_ + 1
               : std::max(0, depth_ - 1);
  }
  void SetDepth(int depth) {
    NOT_DESTROYED();
    depth_ = depth;
  }

  void UpdateText();

  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutQuote";
  }

 private:
  void WillBeDestroyed() override;
  bool IsQuote() const final {
    NOT_DESTROYED();
    return true;
  }
  void StyleDidChange(StyleDifference, const ComputedStyle*) override;
  void WillBeRemovedFromTree() override;

  String ComputeText() const;
  scoped_refptr<const QuotesData> GetQuotesData() const;

  LayoutTextFragment* FindFragmentChild() const;

  // Type of this LayoutQuote: open-quote, close-quote, no-open-quote,
  // no-close-quote.
  QuoteType type_;

  // Number of open quotes in the tree. Also called the nesting level
  // in CSS 2.1.
  // Used to determine if a LayoutQuote is invalid (closing quote without a
  // matching opening quote) and which quote character to use (see the 'quote'
  // property that is used to define quote character pairs).
  int depth_;

  // The pseudo-element that owns us.
  //
  // Lifetime is the same as LayoutObject::m_node, so this is safe.
  Member<PseudoElement> owning_pseudo_;

  // The contain style scope this quote belongs to.
  Member<StyleContainmentScope> scope_;

  // Cached text for this quote.
  String text_;
};

template <>
struct DowncastTraits<LayoutQuote> {
  static bool AllowFrom(const LayoutObject& object) { return object.IsQuote(); }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_QUOTE_H_
