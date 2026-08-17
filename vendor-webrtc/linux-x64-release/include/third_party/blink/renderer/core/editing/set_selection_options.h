// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SET_SELECTION_OPTIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SET_SELECTION_OPTIONS_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/text_granularity.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

enum class CursorAlignOnScroll { kIfNeeded, kAlways };
enum class SetSelectionBy { kSystem = 0, kUser = 1 };

// This class represents parameters of
// |FrameSelection::SetSelectionAndEndTyping()|.
class CORE_EXPORT SetSelectionOptions final {
  STACK_ALLOCATED();

 public:
  class CORE_EXPORT Builder;

  SetSelectionOptions();
  SetSelectionOptions(const SetSelectionOptions&);
  SetSelectionOptions& operator=(const SetSelectionOptions&);

  CursorAlignOnScroll GetCursorAlignOnScroll() const {
    return cursor_align_on_scroll_;
  }
  bool DoNotClearStrategy() const { return do_not_clear_strategy_; }
  bool DoNotSetFocus() const { return do_not_set_focus_; }
  TextGranularity Granularity() const { return granularity_; }
  SetSelectionBy GetSetSelectionBy() const { return set_selection_by_; }
  bool ShouldClearTypingStyle() const { return should_clear_typing_style_; }
  bool ShouldCloseTyping() const { return should_close_typing_; }
  bool ShouldShowHandle() const { return should_show_handle_; }
  bool ShouldShrinkNextTap() const { return should_shrink_next_tap_; }
  bool IsDirectional() const { return is_directional_; }

 private:
  CursorAlignOnScroll cursor_align_on_scroll_ = CursorAlignOnScroll::kIfNeeded;
  bool do_not_clear_strategy_ = false;
  bool do_not_set_focus_ = false;
  TextGranularity granularity_ = TextGranularity::kCharacter;
  SetSelectionBy set_selection_by_ = SetSelectionBy::kSystem;
  bool should_clear_typing_style_ = false;
  bool should_close_typing_ = false;
  bool should_show_handle_ = false;
  bool should_shrink_next_tap_ = false;
  bool is_directional_ = false;
};

// This class is used for building |SelectionData| object.
class CORE_EXPORT SetSelectionOptions::Builder final {
  STACK_ALLOCATED();

 public:
  explicit Builder(const SetSelectionOptions&);
  Builder();
  Builder(const Builder&) = delete;
  Builder& operator=(const Builder&) = delete;

  SetSelectionOptions Build() const;

  Builder& SetCursorAlignOnScroll(CursorAlignOnScroll);
  Builder& SetDoNotClearStrategy(bool);
  Builder& SetDoNotSetFocus(bool);
  Builder& SetGranularity(TextGranularity);
  Builder& SetSetSelectionBy(SetSelectionBy);
  Builder& SetShouldClearTypingStyle(bool);
  Builder& SetShouldCloseTyping(bool);
  Builder& SetShouldShowHandle(bool);
  Builder& SetShouldShrinkNextTap(bool);
  Builder& SetIsDirectional(bool);

 private:
  SetSelectionOptions data_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SET_SELECTION_OPTIONS_H_
