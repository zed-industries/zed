// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_DISPLAY_STYLE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_DISPLAY_STYLE_H_

#include "third_party/blink/renderer/core/style/content_data.h"

namespace blink {

class ContentData;

// DisplayStyle contains the subset of ComputedStyle which is needed to
// determine whether an Element needs a LayoutObject or not. It is useful
// to keep this separate from ComputedStyle, because we sometimes need to
// make that determination before the ComputedStyle is created.
class DisplayStyle {
  STACK_ALLOCATED();

 public:
  DisplayStyle(EDisplay display,
               PseudoId style_type,
               const ContentData* content_data)
      : display_(display),
        style_type_(style_type),
        content_data_(content_data) {}

  EDisplay Display() const { return display_; }
  PseudoId StyleType() const { return style_type_; }
  const ContentData* GetContentData() const { return content_data_; }

  bool ContentBehavesAsNormal() const {
    switch (style_type_) {
      case kPseudoIdMarker:
        return !content_data_;
      default:
        return !content_data_ || content_data_->IsNone();
    }
  }

  bool ContentPreventsBoxGeneration() const {
    switch (style_type_) {
      case kPseudoIdCheckMark:
      case kPseudoIdBefore:
      case kPseudoIdAfter:
      case kPseudoIdPickerIcon:
      case kPseudoIdScrollButtonBlockStart:
      case kPseudoIdScrollButtonInlineStart:
      case kPseudoIdScrollButtonInlineEnd:
      case kPseudoIdScrollButtonBlockEnd:
        return ContentBehavesAsNormal();
      case kPseudoIdMarker:
        return content_data_ && content_data_->IsNone();
      default:
        return false;
    }
  }

 private:
  EDisplay display_;
  PseudoId style_type_;
  const ContentData* content_data_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_DISPLAY_STYLE_H_
