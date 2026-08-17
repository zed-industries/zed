/*
 * Copyright (C) 2010 Google, Inc. All Rights Reserved.
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
 * THIS SOFTWARE IS PROVIDED BY GOOGLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL GOOGLE INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_IGNORE_DESTRUCTIVE_WRITE_COUNT_INCREMENTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_IGNORE_DESTRUCTIVE_WRITE_COUNT_INCREMENTER_H_

#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class IgnoreDestructiveWriteCountIncrementer {
  STACK_ALLOCATED();

 public:
  IgnoreDestructiveWriteCountIncrementer(
      const IgnoreDestructiveWriteCountIncrementer&) = delete;
  IgnoreDestructiveWriteCountIncrementer& operator=(
      const IgnoreDestructiveWriteCountIncrementer&) = delete;
  explicit IgnoreDestructiveWriteCountIncrementer(Document* document)
      : count_(document ? &document->ignore_destructive_write_count_
                        : nullptr) {
    if (!count_)
      return;
    ++(*count_);
  }

  ~IgnoreDestructiveWriteCountIncrementer() {
    if (!count_)
      return;
    --(*count_);
  }

 private:
  unsigned* count_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_IGNORE_DESTRUCTIVE_WRITE_COUNT_INCREMENTER_H_
