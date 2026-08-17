// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_ATTRIBUTES_RANGES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_ATTRIBUTES_RANGES_H_

#include "base/check_op.h"
#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/dom/attribute.h"
#include "third_party/blink/renderer/core/html/parser/html_parser_idioms.h"
#include "third_party/blink/renderer/core/html/parser/literal_buffer.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

// Tracks the ranges (offsets into into the input stream) of the attributes of a
// token.
class HTMLAttributesRanges {
  USING_FAST_MALLOC(HTMLAttributesRanges);

 public:
  class Range {
    DISALLOW_NEW();

   public:
    static constexpr int kInvalidOffset = -1;

    inline void Clear() {
#if DCHECK_IS_ON()
      start = kInvalidOffset;
      end = kInvalidOffset;
#endif
    }

    // Check Range instance that is actively being parsed.
    inline void CheckValidStart() const {
      DCHECK_NE(start, kInvalidOffset);
      DCHECK_GE(start, 0);
    }

    // Check Range instance which finished parse.
    inline void CheckValid() const {
      CheckValidStart();
      DCHECK_NE(end, kInvalidOffset);
      DCHECK_GE(end, 0);
      DCHECK_LE(start, end);
    }

    int start;
    int end;
  };

  struct Attribute {
    Range name_range;
    Range value_range;
  };

  using AttributeList = Vector<Attribute, kAttributePrealloc>;

  void Clear() {
    current_attribute_ = nullptr;
    attributes_.clear();
  }

  void AddAttribute(int offset) {
    attributes_.Grow(attributes_.size() + 1);
    current_attribute_ = &attributes_.back();
    current_attribute_->name_range.start = offset;
    current_attribute_->name_range.CheckValidStart();
  }

  void EndAttributeName(int offset) {
    DCHECK(current_attribute_);
    current_attribute_->name_range.end = offset;
    current_attribute_->name_range.CheckValid();
    current_attribute_->value_range.start = offset;
    current_attribute_->value_range.end = offset;
  }

  void BeginAttributeValue(int offset) {
    DCHECK(current_attribute_);
    current_attribute_->value_range.Clear();
    current_attribute_->value_range.start = offset;
    current_attribute_->value_range.CheckValidStart();
  }

  void EndAttributeValue(int offset) {
    DCHECK(current_attribute_);
    current_attribute_->value_range.end = offset;
    current_attribute_->value_range.CheckValid();
  }

  const AttributeList& attributes() const { return attributes_; }

 private:
  AttributeList attributes_;
  Attribute* current_attribute_ = nullptr;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_ATTRIBUTES_RANGES_H_
