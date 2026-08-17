// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_THROW_ON_DYNAMIC_MARKUP_INSERTION_COUNT_INCREMENTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_THROW_ON_DYNAMIC_MARKUP_INSERTION_COUNT_INCREMENTER_H_

#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class ThrowOnDynamicMarkupInsertionCountIncrementer {
  STACK_ALLOCATED();

 public:
  explicit ThrowOnDynamicMarkupInsertionCountIncrementer(Document* document)
      : count_(document ? &document->throw_on_dynamic_markup_insertion_count_
                        : nullptr) {
    if (!count_)
      return;
    ++(*count_);
  }
  ThrowOnDynamicMarkupInsertionCountIncrementer(
      const ThrowOnDynamicMarkupInsertionCountIncrementer&) = delete;
  ThrowOnDynamicMarkupInsertionCountIncrementer& operator=(
      const ThrowOnDynamicMarkupInsertionCountIncrementer&) = delete;

  ~ThrowOnDynamicMarkupInsertionCountIncrementer() {
    if (!count_)
      return;
    --(*count_);
  }

 private:
  unsigned* count_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_THROW_ON_DYNAMIC_MARKUP_INSERTION_COUNT_INCREMENTER_H_
