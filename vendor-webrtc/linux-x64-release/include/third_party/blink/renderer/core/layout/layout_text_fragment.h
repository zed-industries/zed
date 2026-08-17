/*
 * (C) 1999 Lars Knoll (knoll@kde.org)
 * (C) 2000 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2004, 2005, 2006, 2007 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_TEXT_FRAGMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_TEXT_FRAGMENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/editing_utilities.h"
#include "third_party/blink/renderer/core/layout/layout_text.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class FirstLetterPseudoElement;

// Used to represent a text substring of an element, e.g., for text runs that
// are split because of first letter and that must therefore have different
// styles (and positions in the layout tree).
// We cache offsets so that text transformations can be applied in such a way
// that we can recover the original unaltered string from our corresponding DOM
// node.
class CORE_EXPORT LayoutTextFragment : public LayoutText {
 public:
  LayoutTextFragment(Node*, const String&, int start_offset, int length);
  ~LayoutTextFragment() override;

  static LayoutTextFragment* Create(Node*,
                                    const String&,
                                    int start_offset,
                                    int length);
  static LayoutTextFragment* CreateAnonymous(Document&, const String&);
  static LayoutTextFragment* CreateAnonymous(Document&,
                                             const String&,
                                             unsigned start,
                                             unsigned length);

  void Trace(Visitor*) const override;

  Position PositionForCaretOffset(unsigned) const override;
  std::optional<unsigned> CaretOffsetForPosition(
      const Position&) const override;

  unsigned Start() const {
    NOT_DESTROYED();
    return start_;
  }
  unsigned FragmentLength() const {
    NOT_DESTROYED();
    return fragment_length_;
  }

  unsigned TextStartOffset() const override {
    NOT_DESTROYED();
    return Start();
  }

  void SetContentString(const String&);
  const String& ContentString() const {
    NOT_DESTROYED();
    return content_string_;
  }
  // The complete text is all of the text in the associated DOM text node.
  String CompleteText() const;
  // The fragment text is the text which will be used by this
  // LayoutTextFragment. For things like first-letter this may differ from the
  // completeText as we maybe using only a portion of the text nodes content.

  String OriginalText() const override;

  void SetTextFragment(String, unsigned start, unsigned length);

  void TransformAndSecureOriginalText() override;

  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutTextFragment";
  }

  void SetFirstLetterPseudoElement(FirstLetterPseudoElement* element) {
    NOT_DESTROYED();
    first_letter_pseudo_element_ = element;
  }
  FirstLetterPseudoElement* GetFirstLetterPseudoElement() const {
    NOT_DESTROYED();
    return first_letter_pseudo_element_.Get();
  }

  void SetIsRemainingTextLayoutObject(bool is_remaining_text) {
    NOT_DESTROYED();
    is_remaining_text_layout_object_ = is_remaining_text;
  }
  bool IsRemainingTextLayoutObject() const {
    NOT_DESTROYED();
    return is_remaining_text_layout_object_;
  }

  Text* AssociatedTextNode() const;
  LayoutText* GetFirstLetterPart() const override;

  String PlainText() const override;

 protected:
  friend class LayoutObjectFactory;
  void WillBeDestroyed() override;

 private:
  void InsertedIntoTree() final {
    NOT_DESTROYED();
    valid_ng_items_ = false;
    LayoutText::InsertedIntoTree();
  }
  LayoutBlock* BlockForAccompanyingFirstLetter() const;
  UChar PreviousCharacter() const override;
  void TextDidChange() override;

  void UpdateHitTestResult(HitTestResult&,
                           const PhysicalOffset&) const override;

  DOMNodeId OwnerNodeId() const final;

  unsigned start_;
  unsigned fragment_length_;
  bool is_remaining_text_layout_object_;
  String content_string_;

  Member<FirstLetterPseudoElement> first_letter_pseudo_element_;
};

template <>
struct DowncastTraits<LayoutTextFragment> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsText() && To<LayoutText>(object).IsTextFragment();
  }
  static bool AllowFrom(const LayoutText& text) {
    return text.IsTextFragment();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_LAYOUT_TEXT_FRAGMENT_H_
