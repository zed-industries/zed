/*
 * Copyright (C) 2004, 2006, 2009 Apple Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_ITERATORS_TEXT_ITERATOR_TEXT_STATE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_ITERATORS_TEXT_ITERATOR_TEXT_STATE_H_

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/iterators/text_iterator_behavior.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace WTF {
class StringBuilder;
}  // namespace WTF

namespace blink {

class ContainerNode;
class HTMLElement;
class Node;
class Text;

class CORE_EXPORT TextIteratorTextState {
  STACK_ALLOCATED();

 public:
  explicit TextIteratorTextState(const TextIteratorBehavior&);
  TextIteratorTextState(const TextIteratorTextState&) = delete;
  TextIteratorTextState& operator=(const TextIteratorTextState&) = delete;

  // Return properties of the current text.
  unsigned length() const { return text_length_; }
  UChar CharacterAt(unsigned index) const;
  // TODO(xiaochengh): Rename to |GetText()| as it's used in production code.
  String GetTextForTesting() const;
  void AppendTextToStringBuilder(WTF::StringBuilder&,
                                 unsigned position = 0,
                                 unsigned max_length = UINT_MAX) const;

  // Emits code unit relative to |node|.
  void EmitChar16AfterNode(UChar code_unit, const Node& node);
  void EmitChar16AsNode(UChar code_unit, const Node& node);
  void EmitChar16BeforeChildren(UChar code_unit,
                                const ContainerNode& container_node);
  void EmitChar16BeforeNode(UChar code_unit, const Node& node);

  // Emits |code_unit| before |offset| in |text_node|.
  void EmitChar16Before(UChar code_unit,
                        const Text& text_node,
                        unsigned offset);
  // Emits |code_unit| as replacement of code unit at |offset| in |text_node|.
  void EmitReplacmentCodeUnit(UChar code_unit,
                              const Text& text_node,
                              unsigned offset);
  void EmitText(const Text&,
                unsigned position_start_offset,
                unsigned position_end_offset,
                const String&,
                unsigned text_start_offset,
                unsigned text_end_offset);
  void EmitAltText(const HTMLElement&);
  void UpdateForReplacedElement(const Node& node);

  // Return position of the current text.
  void UpdatePositionOffsets(const ContainerNode& container_node,
                             unsigned index) const;
  unsigned PositionStartOffset() const;
  unsigned PositionEndOffset() const;
  const Node* PositionNode() const { return position_node_; }
  const Node* PositionContainerNode() const { return position_container_node_; }
  bool IsAfterPositionNode() const {
    return position_node_type_ == PositionNodeType::kAfterNode;
  }
  bool IsBeforePositionNode() const {
    return position_node_type_ == PositionNodeType::kBeforeNode;
  }
  bool IsBeforeCharacter() const {
    return position_node_type_ == PositionNodeType::kBeforeCharacter;
  }
  bool IsBeforeChildren() const {
    return position_node_type_ == PositionNodeType::kBeforeChildren;
  }
  bool IsInTextNode() const {
    return position_node_type_ == PositionNodeType::kInText;
  }

  bool HasEmitted() const { return has_emitted_; }
  UChar LastCharacter() const { return last_character_; }
  void ResetRunInformation() {
    position_node_ = nullptr;
    text_length_ = 0;
  }

 private:
  // Location of text run relative to |position_node_|.
  enum class PositionNodeType {
    kNone,
    kAfterNode,
    kAltText,
    kAsNode,
    kBeforeCharacter,
    kBeforeChildren,
    kBeforeNode,
    kInText,
  };

  void ResetPositionContainerNode(PositionNodeType node_type, const Node& node);
  void SetTextNodePosition(const Text& text_node,
                           unsigned start_offset,
                           unsigned end_offset);
  void PopulateStringBuffer(const String& text, unsigned start, unsigned end);
  void PopulateStringBufferFromChar16(UChar code_unit);

  const TextIteratorBehavior behavior_;
  unsigned text_length_ = 0;

  // TODO(editing-dev): We should integrate single character buffer and
  // string buffer.
  // Used for whitespace characters that aren't in the DOM, so we can point at
  // them.
  // If non-zero, overrides |text_|.
  UChar single_character_buffer_ = 0;

  // The current text when |single_character_buffer_| is zero, in which case it
  // is |text_.Substring(text_start_offset_, text_length_)|.
  String text_;
  // TODO(editing-dev): We should make |text_| to hold substring instead of
  // entire string with |text_start_offset_| and |text_length_|.
  unsigned text_start_offset_ = 0;

  // Position of the current text, in the form to be returned from the iterator.
  const Node* position_node_ = nullptr;
  // |Text| node when |position_node_type_ == kInText| or |ContainerNode|.
  mutable const Node* position_container_node_ = nullptr;
  mutable std::optional<unsigned> position_start_offset_;
  mutable std::optional<unsigned> position_end_offset_;
  PositionNodeType position_node_type_ = PositionNodeType::kNone;

  // Used when deciding whether to emit a "positioning" (e.g. newline) before
  // any other content
  bool has_emitted_ = false;
  UChar last_character_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_ITERATORS_TEXT_ITERATOR_TEXT_STATE_H_
