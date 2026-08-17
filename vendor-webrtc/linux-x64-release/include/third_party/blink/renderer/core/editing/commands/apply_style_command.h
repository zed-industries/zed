/*
 * Copyright (C) 2005, 2006, 2008, 2009 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_APPLY_STYLE_COMMAND_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_APPLY_STYLE_COMMAND_H_

#include "mojo/public/mojom/base/text_direction.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/commands/composite_edit_command.h"
#include "third_party/blink/renderer/core/html/html_element.h"

namespace blink {

class EditingStyle;
class HTMLSpanElement;
class StyleChange;

enum ShouldIncludeTypingStyle { kIncludeTypingStyle, kIgnoreTypingStyle };

class CORE_EXPORT ApplyStyleCommand final : public CompositeEditCommand {
 public:
  enum PropertyLevel { kPropertyDefault, kForceBlockProperties };
  enum InlineStyleRemovalMode { kRemoveIfNeeded, kRemoveAlways, kRemoveNone };
  enum AddStyledElement { kAddStyledElement, kDoNotAddStyledElement };
  typedef bool (*IsInlineElementToRemoveFunction)(const Element*);

  ApplyStyleCommand(Document&,
                    const EditingStyle*,
                    InputEvent::InputType,
                    PropertyLevel = kPropertyDefault);
  ApplyStyleCommand(Document&,
                    const EditingStyle*,
                    const Position& start,
                    const Position& end);
  ApplyStyleCommand(Element*, bool remove_only);
  ApplyStyleCommand(Document&,
                    const EditingStyle*,
                    bool (*is_inline_element_to_remove)(const Element*),
                    InputEvent::InputType);

  void Trace(Visitor*) const override;

 private:
  void DoApply(EditingState*) override;
  InputEvent::InputType GetInputType() const override;

  // style-removal helpers
  bool IsStyledInlineElementToRemove(Element*) const;
  bool ShouldApplyInlineStyleToRun(EditingStyle*,
                                   Node* run_start,
                                   Node* past_end_node);
  void RemoveConflictingInlineStyleFromRun(EditingStyle*,
                                           Member<Node>& run_start,
                                           Member<Node>& run_end,
                                           Node* past_end_node,
                                           EditingState*);
  bool RemoveInlineStyleFromElement(EditingStyle*,
                                    HTMLElement*,
                                    EditingState*,
                                    InlineStyleRemovalMode = kRemoveIfNeeded,
                                    EditingStyle* extracted_style = nullptr);
  inline bool ShouldRemoveInlineStyleFromElement(EditingStyle* style,
                                                 HTMLElement* element) {
    return RemoveInlineStyleFromElement(style, element, ASSERT_NO_EDITING_ABORT,
                                        kRemoveNone);
  }
  void ReplaceWithSpanOrRemoveIfWithoutAttributes(HTMLElement*, EditingState*);
  bool RemoveImplicitlyStyledElement(EditingStyle*,
                                     HTMLElement*,
                                     InlineStyleRemovalMode,
                                     EditingStyle* extracted_style,
                                     EditingState*);
  bool RemoveCSSStyle(EditingStyle*,
                      HTMLElement*,
                      EditingState*,
                      InlineStyleRemovalMode = kRemoveIfNeeded,
                      EditingStyle* extracted_style = nullptr);
  HTMLElement* HighestAncestorWithConflictingInlineStyle(EditingStyle*, Node*);
  void ApplyInlineStyleToPushDown(Node*, EditingStyle*, EditingState*);
  void PushDownInlineStyleAroundNode(EditingStyle*, Node*, EditingState*);
  void RemoveInlineStyle(EditingStyle*,
                         const EphemeralRange& range,
                         EditingState*);
  bool ElementFullySelected(const HTMLElement&,
                            const Position& start,
                            const Position& end) const;

  // style-application helpers
  void ApplyBlockStyle(EditingStyle*, EditingState*);
  void ApplyRelativeFontStyleChange(EditingStyle*, EditingState*);
  void ApplyInlineStyle(EditingStyle*, EditingState*);
  void FixRangeAndApplyInlineStyle(EditingStyle*,
                                   const Position& start,
                                   const Position& end,
                                   EditingState*);
  void ApplyInlineStyleToNodeRange(EditingStyle*,
                                   Node* start_node,
                                   Node* past_end_node,
                                   EditingState*);
  void AddBlockStyle(const StyleChange&, HTMLElement*);
  void AddInlineStyleIfNeeded(EditingStyle*,
                              Node* start,
                              Node* end,
                              EditingState*);
  Position PositionToComputeInlineStyleChange(
      Node*,
      Member<HTMLSpanElement>& dummy_element,
      EditingState*);
  void ApplyInlineStyleChange(Node* start_node,
                              Node* end_node,
                              StyleChange&,
                              AddStyledElement,
                              EditingState*);
  void SplitTextAtStart(const Position& start, const Position& end);
  void SplitTextAtEnd(const Position& start, const Position& end);
  void SplitTextElementAtStart(const Position& start, const Position& end);
  void SplitTextElementAtEnd(const Position& start, const Position& end);
  bool ShouldSplitTextElement(Element*, EditingStyle*);
  bool IsValidCaretPositionInTextNode(const Position&);
  bool MergeStartWithPreviousIfIdentical(const Position& start,
                                         const Position& end,
                                         EditingState*);
  bool MergeEndWithNextIfIdentical(const Position& start,
                                   const Position& end,
                                   EditingState*);
  void CleanupUnstyledAppleStyleSpans(ContainerNode* dummy_span_ancestor,
                                      EditingState*);

  void SurroundNodeRangeWithElement(Node* start,
                                    Node* end,
                                    Element*,
                                    EditingState*);
  float ComputedFontSize(Node*);
  void JoinChildTextNodes(ContainerNode*,
                          const Position& start,
                          const Position& end);

  HTMLElement* SplitAncestorsWithUnicodeBidi(
      Node*,
      bool before,
      mojo_base::mojom::blink::TextDirection allowed_direction);
  void RemoveEmbeddingUpToEnclosingBlock(Node*,
                                         HTMLElement* unsplit_ancestor,
                                         EditingState*);

  void UpdateStartEnd(const EphemeralRange&);
  Position StartPosition();
  Position EndPosition();

  const Member<EditingStyle> style_;
  const InputEvent::InputType input_type_;
  const PropertyLevel property_level_;
  Position start_;
  Position end_;
  bool use_ending_selection_;
  const Member<Element> styled_inline_element_;
  const bool remove_only_;
  IsInlineElementToRemoveFunction const is_inline_element_to_remove_function_;
};

enum ShouldStyleAttributeBeEmpty {
  kAllowNonEmptyStyleAttribute,
  kStyleAttributeShouldBeEmpty
};
bool IsEmptyFontTag(const Element*,
                    ShouldStyleAttributeBeEmpty = kStyleAttributeShouldBeEmpty);
bool IsLegacyAppleHTMLSpanElement(const Node*);
bool IsStyleSpanOrSpanWithOnlyStyleAttribute(const Element*);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_APPLY_STYLE_COMMAND_H_
