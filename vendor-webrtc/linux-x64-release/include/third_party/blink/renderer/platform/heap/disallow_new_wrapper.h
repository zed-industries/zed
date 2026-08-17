// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_DISALLOW_NEW_WRAPPER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_DISALLOW_NEW_WRAPPER_H_

#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"

namespace blink {

// DisallowNewWrapper wraps a disallow new type in a GarbageCollected class.
template <typename T>
class DisallowNewWrapper final
    : public GarbageCollected<DisallowNewWrapper<T>> {
 public:
  explicit DisallowNewWrapper(const T& value) : value_(value) {
    static_assert(WTF::IsDisallowNew<T>, "T needs to be a disallow new type");
    static_assert(WTF::IsTraceable<T>::value, "T needs to be traceable");
  }
  explicit DisallowNewWrapper(T&& value) : value_(std::forward<T>(value)) {
    static_assert(WTF::IsDisallowNew<T>, "T needs to be a disallow new type");
    static_assert(WTF::IsTraceable<T>::value, "T needs to be traceable");
  }

  template <typename... Args>
  explicit DisallowNewWrapper(Args&&... args)
      : value_(std::forward<Args>(args)...) {
    static_assert(WTF::IsDisallowNew<T>, "T needs to be a disallow new type");
    static_assert(WTF::IsTraceable<T>::value, "T needs to be traceable");
  }

  const T& Value() const { return value_; }
  T& Value() { return value_; }
  T&& TakeValue() { return std::move(value_); }

  void Trace(Visitor* visitor) const { visitor->Trace(value_); }

 private:
  T value_;
};

// Wraps a disallow new type in a GarbageCollected class, making it possible to
// be referenced off heap from a Persistent.
template <typename T>
DisallowNewWrapper<T>* WrapDisallowNew(const T& value) {
  return MakeGarbageCollected<DisallowNewWrapper<T>>(value);
}

template <typename T>
DisallowNewWrapper<T>* WrapDisallowNew(T&& value) {
  return MakeGarbageCollected<DisallowNewWrapper<T>>(std::forward<T>(value));
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_DISALLOW_NEW_WRAPPER_H_
