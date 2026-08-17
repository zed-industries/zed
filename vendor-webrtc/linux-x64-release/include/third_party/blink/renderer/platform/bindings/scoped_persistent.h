/*
 * Copyright (C) 2009 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_SCOPED_PERSISTENT_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_SCOPED_PERSISTENT_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "v8/include/v8.h"

namespace blink {

// Holds a persistent handle to a V8 object; use ScopedPersistent instead of
// directly using v8::Persistent. Introducing a (non-weak) ScopedPersistent
// has a risk of producing memory leaks, ask blink-reviews-bindings@ for a
// review.
template <typename T>
class ScopedPersistent {
  USING_FAST_MALLOC(ScopedPersistent);

 public:
  ScopedPersistent() = default;
  ScopedPersistent(v8::Isolate* isolate, v8::Local<T> handle)
      : handle_(isolate, handle) {}
  ScopedPersistent(const ScopedPersistent&) = delete;
  ScopedPersistent& operator=(const ScopedPersistent&) = delete;

  ~ScopedPersistent() { Clear(); }

  ALWAYS_INLINE v8::Local<T> NewLocal(v8::Isolate* isolate) const {
    return v8::Local<T>::New(isolate, handle_);
  }

  // If you don't need to get weak callback, use setPhantom instead.
  // setPhantom is faster than setWeak.
  template <typename P>
  void SetWeak(P* parameters,
               void (*callback)(const v8::WeakCallbackInfo<P>&),
               v8::WeakCallbackType type = v8::WeakCallbackType::kParameter) {
    handle_.SetWeak(parameters, callback, type);
  }

  // Turns this handle into a weak phantom handle without
  // finalization callback.
  void SetPhantom() { handle_.SetWeak(); }

  void ClearWeak() { handle_.template ClearWeak<void>(); }

  bool IsEmpty() const { return handle_.IsEmpty(); }
  bool IsWeak() const { return handle_.IsWeak(); }

  void Set(v8::Isolate* isolate, v8::Local<T> handle) {
    handle_.Reset(isolate, handle);
  }

  // Note: This is clear in the std::unique_ptr sense, not the v8::Local sense.
  void Clear() { handle_.Reset(); }

  bool operator==(const ScopedPersistent<T>& other) {
    return handle_ == other.handle_;
  }

  template <class S>
  bool operator==(const v8::Local<S> other) const {
    return handle_ == other;
  }

  ALWAYS_INLINE v8::Persistent<T>& Get() { return handle_; }

 private:
  v8::Persistent<T> handle_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_SCOPED_PERSISTENT_H_
