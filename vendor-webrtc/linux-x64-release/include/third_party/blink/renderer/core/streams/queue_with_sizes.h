// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_QUEUE_WITH_SIZES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_QUEUE_WITH_SIZES_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_deque.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "v8/include/v8.h"

namespace blink {

class ExceptionState;

// Implementation of the "Queue-with-sizes" operations from the standard. Unlike
// the standard, these operations do not operate polymorphically on the
// container, but require it to have a QueueWithSizes member.
// https://streams.spec.whatwg.org/#queue-with-sizes
class CORE_EXPORT QueueWithSizes final
    : public GarbageCollected<QueueWithSizes> {
 public:
  QueueWithSizes();

  // https://streams.spec.whatwg.org/#dequeue-value
  v8::Local<v8::Value> DequeueValue(v8::Isolate*);

  // Unlike in the standard, this implementation expects the conversion of
  // |size| to a number to be done by the caller.
  // https://streams.spec.whatwg.org/#enqueue-value-with-size
  void EnqueueValueWithSize(v8::Isolate*,
                            v8::Local<v8::Value> value,
                            double size,
                            ExceptionState&);

  // https://streams.spec.whatwg.org/#peek-queue-value
  v8::Local<v8::Value> PeekQueueValue(v8::Isolate*);

  double TotalSize() { return queue_total_size_; }

  // Not part of the standard.
  bool IsEmpty() { return queue_.empty(); }

  // https://streams.spec.whatwg.org/#reset-queue
  void ResetQueue();

  void Trace(Visitor* visitor) const;

 private:
  class ValueSizePair;

  HeapDeque<Member<ValueSizePair>> queue_;
  double queue_total_size_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STREAMS_QUEUE_WITH_SIZES_H_
