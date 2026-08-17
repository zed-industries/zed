/*
 * Copyright (C) 2010 Nokia Corporation and/or its subsidiary(-ies)
 * Copyright (C) 2010 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_EDITING_BEHAVIOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_EDITING_BEHAVIOR_H_

#include "third_party/blink/public/mojom/webpreferences/web_preferences.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class KeyboardEvent;
enum class WritingMode : uint8_t;

class CORE_EXPORT EditingBehavior {
  STACK_ALLOCATED();

 public:
  explicit EditingBehavior(mojom::blink::EditingBehavior type) : type_(type) {}

  // Individual functions for each case where we have more than one style of
  // editing behavior. Create a new function for any platform difference so we
  // can control it here.

  // When extending a selection beyond the top or bottom boundary of an editable
  // area, maintain the horizontal position on Windows, Android and ChromeOS but
  // extend it to the boundary of the editable content on Mac and Linux.
  bool ShouldMoveCaretToHorizontalBoundaryWhenPastTopOrBottom() const {
    return type_ != mojom::blink::EditingBehavior::kEditingWindowsBehavior &&
           type_ != mojom::blink::EditingBehavior::kEditingAndroidBehavior &&
           type_ != mojom::blink::EditingBehavior::kEditingChromeOSBehavior;
  }

  bool ShouldSelectReplacement() const {
    return type_ == mojom::blink::EditingBehavior::kEditingAndroidBehavior;
  }

  // On Windows, selections should always be considered as directional,
  // regardless if it is mouse-based or keyboard-based.
  bool ShouldConsiderSelectionAsDirectional() const {
    return type_ != mojom::blink::EditingBehavior::kEditingMacBehavior;
  }

  // On Mac, when revealing a selection (for example as a result of a Find
  // operation on the Browser), content should be scrolled such that the
  // selection gets certer aligned.
  bool ShouldCenterAlignWhenSelectionIsRevealed() const {
    return type_ == mojom::blink::EditingBehavior::kEditingMacBehavior;
  }

  // On Mac, style is considered present when present at the beginning of
  // selection. On other platforms, style has to be present throughout the
  // selection.
  bool ShouldToggleStyleBasedOnStartOfSelection() const {
    return type_ == mojom::blink::EditingBehavior::kEditingMacBehavior;
  }

  // Standard Mac behavior when extending to a boundary is grow the selection
  // rather than leaving the base in place and moving the extent. Matches
  // NSTextView.
  bool ShouldAlwaysGrowSelectionWhenExtendingToBoundary() const {
    return type_ == mojom::blink::EditingBehavior::kEditingMacBehavior;
  }

  // On Mac/ChromeOS, when processing a contextual click, the object being
  // clicked upon should be selected.
  bool ShouldSelectOnContextualMenuClick() const {
    return type_ == mojom::blink::EditingBehavior::kEditingMacBehavior ||
           type_ == mojom::blink::EditingBehavior::kEditingChromeOSBehavior;
  }

  // On Mac, selecting backwards by word/line from the middle of a word/line,
  // and then going forward leaves the caret back in the middle with no
  // selection, instead of directly selecting to the other end of the line/word
  // (Unix/Windows behavior).
  bool ShouldExtendSelectionByWordOrLineAcrossCaret() const {
    return type_ != mojom::blink::EditingBehavior::kEditingMacBehavior;
  }

  // Based on native behavior, when using ctrl(alt)+arrow to move caret by word,
  // ctrl(alt)+left arrow moves caret to immediately before the word in all
  // platforms. For example, the word break positions are:
  //   "|abc |def |hij |opq".
  // But ctrl+right arrow moves caret to "abc |def |hij |opq" on Windows and
  // "abc| def| hij| opq|" on Mac and Linux.
  bool ShouldSkipSpaceWhenMovingRight() const {
    return type_ == mojom::blink::EditingBehavior::kEditingWindowsBehavior;
  }

  // On Mac, undo of delete/forward-delete of text should select the deleted
  // text. On other platforms deleted text should not be selected and the cursor
  // should be placed where the deletion started.
  bool ShouldUndoOfDeleteSelectText() const {
    return type_ == mojom::blink::EditingBehavior::kEditingMacBehavior;
  }

  // On Mac, backspacing at the start of a blocks merges with the
  // previous table block, as we do with regular blocks. On other
  // platforms backspace event does nothing if the block above is a
  // table, but allows mergin otherwise.
  bool ShouldMergeContentWithTablesOnBackspace() const {
    return type_ == mojom::blink::EditingBehavior::kEditingMacBehavior;
  }

  // On ChromeOS, tapping the caret should toggle showing/hiding the touch
  // selection quick menu.
  bool ShouldToggleMenuWhenCaretTapped() const {
    return type_ == mojom::blink::EditingBehavior::kEditingChromeOSBehavior;
  }

  // Support for global selections, used on platforms like the X Window
  // System that treat selection as a type of clipboard.
  bool SupportsGlobalSelection() const {
    return type_ != mojom::blink::EditingBehavior::kEditingWindowsBehavior &&
           type_ != mojom::blink::EditingBehavior::kEditingMacBehavior;
  }

  // Convert a KeyboardEvent to a command name like "Copy", "Undo" and so on.
  // If nothing, return empty string.
  const char* InterpretKeyEvent(const KeyboardEvent&,
                                WritingMode writing_mode) const;

  bool ShouldInsertCharacter(const KeyboardEvent&) const;

 private:
  mojom::blink::EditingBehavior type_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_EDITING_BEHAVIOR_H_
