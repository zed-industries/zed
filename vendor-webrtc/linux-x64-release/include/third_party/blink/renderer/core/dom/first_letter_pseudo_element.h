/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2007 David Smith (catfish.man@gmail.com)
 * Copyright (C) 2003, 2004, 2005, 2006, 2007, 2008, 2009, 2010, 2011 Apple Inc.
 * All rights reserved.
 * Copyright (C) Research In Motion Limited 2010. All rights reserved.
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
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_FIRST_LETTER_PSEUDO_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_FIRST_LETTER_PSEUDO_ELEMENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/dom/pseudo_element.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class Element;
class LayoutText;
class LayoutTextFragment;

class CORE_EXPORT FirstLetterPseudoElement final : public PseudoElement {
 public:
  explicit FirstLetterPseudoElement(Element*);
  FirstLetterPseudoElement(const FirstLetterPseudoElement&) = delete;
  FirstLetterPseudoElement& operator=(const FirstLetterPseudoElement&) = delete;
  ~FirstLetterPseudoElement() override;
  void Trace(Visitor*) const override;

  static LayoutText* FirstLetterTextLayoutObject(const Element&);

  enum class Punctuation {
    // No punctuation seen in preceding text nodes
    kNotSeen,
    // Consecutive punctuation seen in preceding text nodes with no spaces after
    kSeen,
    // Punctuation seen in preceding text nodes, with trailing spaces. For
    // signaling that we should stop looking for first letter text.
    kDisallow,
  };

  // |punctuation| is used to validate combinations of ::first-letter text and
  // punctuation that spans across text nodes. Punctuation is initially set to
  // Punctuation::kNotSeen and is updated to Punctuation::kSeen if the text ends
  // with punctuation, but did not otherwise include valid ::first-letter text.
  // If the out value of |punctuation| is Punctuation::kDisallow, it's a signal
  // that we should continue to look for ::first-letter text.
  static unsigned FirstLetterLength(const String&,
                                    bool preserve_breaks,
                                    Punctuation& punctuation);

  void ClearRemainingTextLayoutObject();
  LayoutTextFragment* RemainingTextLayoutObject() const {
    return remaining_text_layout_object_.Get();
  }

  void UpdateTextFragments();

  void AttachLayoutTree(AttachContext&) override;
  void DetachLayoutTree(bool performing_reattach) override;
  Node* InnerNodeForHitTesting() override;

 private:
  LayoutObject* CreateLayoutObject(const ComputedStyle&) override;

  const ComputedStyle* CustomStyleForLayoutObject(
      const StyleRecalcContext&) override;

  void AttachFirstLetterTextLayoutObjects(LayoutText* first_letter_text);

  Member<LayoutTextFragment> remaining_text_layout_object_;
};

template <>
struct DowncastTraits<FirstLetterPseudoElement> {
  static bool AllowFrom(const Node& node) {
    return node.IsFirstLetterPseudoElement();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_FIRST_LETTER_PSEUDO_ELEMENT_H_
