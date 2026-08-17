/*
 * Copyright (C) 2006, 2007, 2008 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_EDITOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_EDITOR_H_

#include <memory>

#include "mojo/public/mojom/base/text_direction.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/webpreferences/web_preferences.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/editing_style.h"
#include "third_party/blink/renderer/core/editing/finder/find_options.h"
#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/core/editing/visible_selection.h"
#include "third_party/blink/renderer/core/events/input_event.h"
#include "third_party/blink/renderer/core/loader/resource/image_resource_observer.h"
#include "third_party/blink/renderer/core/scroll/scroll_alignment.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class CompositeEditCommand;
class DragData;
class EditingBehavior;
class EditorCommand;
class FrameSelection;
class LocalFrame;
class HitTestResult;
class KeyboardEvent;
class KillRing;
class SpellChecker;
enum class SyncCondition;
class CSSPropertyValueSet;
class TextEvent;
class UndoStack;
class SelectionForUndoStep;

enum class DeleteDirection;
enum class DeleteMode { kSimple, kSmart };
enum class InsertMode { kSimple, kSmart };
enum class DragSourceType { kHTMLSource, kPlainTextSource };
enum class EditorParagraphSeparator { kIsDiv, kIsP };
enum class EditorCommandSource { kMenuOrKeyBinding, kDOM };

class CORE_EXPORT Editor final : public GarbageCollected<Editor> {
 public:
  explicit Editor(LocalFrame&);
  Editor(const Editor&) = delete;
  Editor& operator=(const Editor&) = delete;
  ~Editor();

  CompositeEditCommand* LastEditCommand() { return last_edit_command_.Get(); }
  void SetLastEditCommand(CompositeEditCommand* last_edit_command) {
    last_edit_command_ = last_edit_command;
  }

  void HandleKeyboardEvent(KeyboardEvent*);
  bool HandleTextEvent(TextEvent*);

  bool CanEdit() const;
  bool CanEditRichly() const;

  bool CanCut() const;
  bool CanCopy() const;
  bool CanPaste() const;
  bool CanDelete() const;

  static void CountEvent(ExecutionContext*, const Event&);
  void CopyImage(const HitTestResult&);
  void CopyImage(const HitTestResult& result,
                 const scoped_refptr<Image>& image);

  void RespondToChangedContents(const Position&);
  void NotifyAccessibilityOfDeletionOrInsertionInTextField(
      const SelectionForUndoStep&,
      bool is_deletion);

  void RegisterCommandGroup(CompositeEditCommand* command_group_wrapper);

  void DeleteSelectionWithSmartDelete(
      DeleteMode,
      InputEvent::InputType,
      const Position& reference_move_position = Position());

  void ApplyParagraphStyle(CSSPropertyValueSet*, InputEvent::InputType);
  void ApplyParagraphStyleToSelection(CSSPropertyValueSet*,
                                      InputEvent::InputType);

  void SetShouldStyleWithCSS(bool flag) { should_style_with_css_ = flag; }
  bool ShouldStyleWithCSS() const { return should_style_with_css_; }

  EditorCommand CreateCommand(const String& command_name)
      const;  // Command source is CommandFromMenuOrKeyBinding.
  EditorCommand CreateCommand(const String& command_name,
                              EditorCommandSource) const;

  // |Editor::executeCommand| is implementation of |WebFrame::executeCommand|
  // rather than |Document::execCommand|.
  bool ExecuteCommand(const String&);
  bool ExecuteCommand(const String& command_name, const String& value);
  bool IsCommandEnabled(const String&) const;

  bool InsertText(const String&, KeyboardEvent* triggering_event);
  bool InsertTextWithoutSendingTextEvent(
      const String&,
      bool select_inserted_text,
      TextEvent* triggering_event,
      InputEvent::InputType = InputEvent::InputType::kInsertText);
  bool InsertLineBreak();
  bool InsertParagraphSeparator();

  bool CanUndo();
  void Undo();
  bool CanRedo();
  void Redo();

  // Exposed for IdleSpellCheckController only.
  // Supposed to be used as |const UndoStack&|.
  UndoStack& GetUndoStack() const { return *undo_stack_; }

  void SetBaseWritingDirection(mojo_base::mojom::blink::TextDirection);

  // smartInsertDeleteEnabled and selectTrailingWhitespaceEnabled are
  // mutually exclusive, meaning that enabling one will disable the other.
  bool SmartInsertDeleteEnabled() const;
  bool IsSelectTrailingWhitespaceEnabled() const;

  bool PreventRevealSelection() const { return prevent_reveal_selection_; }
  void IncreasePreventRevealSelection() { ++prevent_reveal_selection_; }
  void DecreasePreventRevealSelection() { --prevent_reveal_selection_; }

  void SetStartNewKillRingSequence(bool);

  void ElementRemoved(Element* element);

  void Clear();

  SelectionInDOMTree SelectionForCommand(Event*);

  KillRing& GetKillRing() const { return *kill_ring_; }

  EditingBehavior Behavior() const;

  EphemeralRange SelectedRange();

  void AddToKillRing(const EphemeralRange&);

  static bool FindString(LocalFrame&, const String&, FindOptions);

  static Range* FindRangeOfString(
      Document&,
      const String& target,
      const EphemeralRangeInFlatTree& reference_range,
      FindOptions,
      bool* wrapped_around = nullptr);

  const VisibleSelection& Mark() const;  // Mark, to be used as emacs uses it.
  bool MarkIsDirectional() const;
  void SetMark();

  void ComputeAndSetTypingStyle(CSSPropertyValueSet*, InputEvent::InputType);

  EphemeralRange RangeForPoint(const gfx::Point&) const;
  EphemeralRange RangeBetweenPoints(const gfx::Point& start_point,
                                    const gfx::Point& end_point) const;

  void RespondToChangedSelection();
  void SyncSelection(blink::SyncCondition force_sync);

  void ReplaceSelectionWithFragment(DocumentFragment*,
                                    bool select_replacement,
                                    bool smart_replace,
                                    bool match_style,
                                    InputEvent::InputType);
  void ReplaceSelectionWithText(const String&,
                                bool select_replacement,
                                bool smart_replace,
                                InputEvent::InputType);

  // Implementation of WebLocalFrameImpl::ReplaceSelection. Does not use smart
  // replacement.
  void ReplaceSelection(const String&);

  void ReplaceSelectionAfterDragging(DocumentFragment*,
                                     InsertMode,
                                     DragSourceType);

  // Return false if frame was destroyed by event handler, should stop executing
  // remaining actions.
  bool DeleteSelectionAfterDraggingWithEvents(
      Element* drag_source,
      DeleteMode,
      const Position& reference_move_position);
  bool ReplaceSelectionAfterDraggingWithEvents(Element* drop_target,
                                               DragData*,
                                               DocumentFragment*,
                                               Range* drop_caret_range,
                                               InsertMode,
                                               DragSourceType);

  EditorParagraphSeparator DefaultParagraphSeparator() const {
    return default_paragraph_separator_;
  }
  void SetDefaultParagraphSeparator(EditorParagraphSeparator separator) {
    default_paragraph_separator_ = separator;
  }

  EditingStyle* TypingStyle() const;
  void SetTypingStyle(EditingStyle*);
  void ClearTypingStyle();

  void Trace(Visitor*) const;

  void RevealSelectionAfterEditingOperation(
      const mojom::blink::ScrollAlignment& = ScrollAlignment::ToEdgeIfNeeded());

  void AddImageResourceObserver(ImageResourceObserver*);
  void RemoveImageResourceObserver(ImageResourceObserver*);

 private:
  Member<LocalFrame> frame_;
  Member<CompositeEditCommand> last_edit_command_;
  const Member<UndoStack> undo_stack_;
  int prevent_reveal_selection_;
  bool should_start_new_kill_ring_sequence_;
  bool should_style_with_css_;
  const std::unique_ptr<KillRing> kill_ring_;
  VisibleSelection mark_;
  EditorParagraphSeparator default_paragraph_separator_;
  Member<EditingStyle> typing_style_;
  bool mark_is_directional_ = false;
  HeapHashSet<Member<ImageResourceObserver>> image_resource_observers_;

  LocalFrame& GetFrame() const {
    DCHECK(frame_);
    return *frame_;
  }

  SpellChecker& GetSpellChecker() const;
  FrameSelection& GetFrameSelection() const;

  bool HandleEditingKeyboardEvent(KeyboardEvent*);
};

inline void Editor::SetStartNewKillRingSequence(bool flag) {
  should_start_new_kill_ring_sequence_ = flag;
}

inline const VisibleSelection& Editor::Mark() const {
  return mark_;
}

inline bool Editor::MarkIsDirectional() const {
  return mark_is_directional_;
}

inline EditingStyle* Editor::TypingStyle() const {
  return typing_style_.Get();
}

inline void Editor::ClearTypingStyle() {
  typing_style_.Clear();
}

inline void Editor::SetTypingStyle(EditingStyle* style) {
  typing_style_ = style;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_EDITOR_H_
