// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_V8_EXTERNAL_MEMORY_ACCOUNTER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_V8_EXTERNAL_MEMORY_ACCOUNTER_H_

#include <stdlib.h>

#include "base/check_op.h"
#include "v8/include/v8-external-memory-accounter.h"
#include "v8/include/v8-isolate.h"

namespace blink {

using V8ExternalMemoryAccounterBase = v8::ExternalMemoryAccounter;

class V8ExternalMemoryAccounter {
 public:
  void Increase(v8::Isolate* isolate, size_t size) {
    amount_of_external_memory_ += size;
    memory_accounter_base_.Increase(isolate, size);
  }

  void Set(v8::Isolate* isolate, size_t size) {
    int64_t delta = size - amount_of_external_memory_;
    if (delta != 0) {
      amount_of_external_memory_ = size;
      memory_accounter_base_.Update(isolate, delta);
    }
  }

  void Clear(v8::Isolate* isolate) {
    if (amount_of_external_memory_ != 0) {
      memory_accounter_base_.Decrease(isolate, amount_of_external_memory_);
      amount_of_external_memory_ = 0;
    }
  }

 private:
  NO_UNIQUE_ADDRESS V8ExternalMemoryAccounterBase memory_accounter_base_;
  size_t amount_of_external_memory_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_V8_EXTERNAL_MEMORY_ACCOUNTER_H_
