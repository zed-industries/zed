// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_DELETE_SELECTION_OPTIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_DELETE_SELECTION_OPTIONS_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// DeleteSelectionOptions of |DeleteSelectionCommand|.
class CORE_EXPORT DeleteSelectionOptions final {
  DISALLOW_NEW();

 public:
  class Builder;

  DeleteSelectionOptions(const DeleteSelectionOptions&);

  bool IsExpandForSpecialElements() const;
  bool IsMergeBlocksAfterDelete() const;
  bool IsSanitizeMarkup() const;
  bool IsSmartDelete() const;

  static DeleteSelectionOptions NormalDelete();
  static DeleteSelectionOptions SmartDelete();

 private:
  DeleteSelectionOptions();

  bool is_expand_for_special_elements_ = false;
  bool is_merge_blocks_after_delete_ = false;
  bool is_sanitize_markup_ = false;
  bool is_smart_delete_ = false;
};

// Build |DeleteSelectionCommand::Options|.
class CORE_EXPORT DeleteSelectionOptions::Builder final {
  DISALLOW_NEW();

 public:
  Builder();
  Builder(const Builder&) = delete;
  Builder& operator=(const Builder*) = delete;

  DeleteSelectionOptions Build() const;

  Builder& SetExpandForSpecialElements(bool);
  Builder& SetMergeBlocksAfterDelete(bool);
  Builder& SetSanitizeMarkup(bool);
  Builder& SetSmartDelete(bool);

 private:
  DeleteSelectionOptions options_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_COMMANDS_DELETE_SELECTION_OPTIONS_H_
