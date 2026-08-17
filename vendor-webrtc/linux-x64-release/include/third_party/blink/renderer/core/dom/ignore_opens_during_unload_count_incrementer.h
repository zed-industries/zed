// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_IGNORE_OPENS_DURING_UNLOAD_COUNT_INCREMENTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_IGNORE_OPENS_DURING_UNLOAD_COUNT_INCREMENTER_H_

#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class IgnoreOpensDuringUnloadCountIncrementer {
  STACK_ALLOCATED();

 public:
  explicit IgnoreOpensDuringUnloadCountIncrementer(Document* document)
      : count_(document ? &document->ignore_opens_during_unload_count_
                        : nullptr) {
    if (!count_)
      return;
    ++(*count_);
  }
  IgnoreOpensDuringUnloadCountIncrementer(
      const IgnoreOpensDuringUnloadCountIncrementer&) = delete;
  IgnoreOpensDuringUnloadCountIncrementer& operator=(
      const IgnoreOpensDuringUnloadCountIncrementer&) = delete;

  ~IgnoreOpensDuringUnloadCountIncrementer() {
    if (!count_)
      return;
    --(*count_);
  }

 private:
  unsigned* count_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_IGNORE_OPENS_DURING_UNLOAD_COUNT_INCREMENTER_H_
