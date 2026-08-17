// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_WRITABLE_STREAM_TRANSFERRING_OPTIMIZER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_WRITABLE_STREAM_TRANSFERRING_OPTIMIZER_H_

#include "third_party/blink/renderer/core/core_export.h"

namespace blink {

class UnderlyingSinkBase;
class ScriptState;

// WritableStreamTransferringOptimizer is the base class used to optimize
// transferring a WritableStream. Please see
// https://docs.google.com/document/d/1_KuZzg5c3pncLJPFa8SuVm23AP4tft6mzPCL5at3I9M/.
//
// A WritableStreamTransferringOptimizer is associated with the sink of a
// WritableStream. When transferring the stream in another realm, the optimizer
// is used to construct the transferred stream in the destination realm. Note
// that two realms can be in different threads (in the same process), in which
// case the optimizer is used across threads.
class CORE_EXPORT WritableStreamTransferringOptimizer {
  USING_FAST_MALLOC(WritableStreamTransferringOptimizer);

 public:
  WritableStreamTransferringOptimizer() = default;
  WritableStreamTransferringOptimizer(
      const WritableStreamTransferringOptimizer&) = delete;
  WritableStreamTransferringOptimizer& operator=(
      const WritableStreamTransferringOptimizer&) = delete;
  virtual ~WritableStreamTransferringOptimizer() = default;

  // Returns an UnderlyingSinkBase for the associated sink. This method may
  // return null, in which case it is no-op.
  // This method can be called at most once.
  virtual UnderlyingSinkBase* PerformInProcessOptimization(
      ScriptState* script_state) {
    return nullptr;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_WRITABLE_STREAM_TRANSFERRING_OPTIMIZER_H_
