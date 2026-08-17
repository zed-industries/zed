// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_TRACE_WRAPPER_V8_REFERENCE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_TRACE_WRAPPER_V8_REFERENCE_H_

#include <type_traits>
#include <utility>

#include "base/compiler_specific.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_traits.h"
#include "third_party/blink/renderer/platform/wtf/vector_traits.h"
#include "v8/include/v8-cppgc.h"
#include "v8/include/v8-traced-handle.h"

namespace blink {

template <typename T>
using TraceWrapperV8Reference = v8::TracedReference<T>;

}  // namespace blink

namespace WTF {

template <typename T>
struct IsTraceable<blink::TraceWrapperV8Reference<T>> {
  STATIC_ONLY(IsTraceable);
  static const bool value = true;
};

template <typename T>
struct VectorTraits<blink::TraceWrapperV8Reference<T>>
    : VectorTraitsBase<blink::TraceWrapperV8Reference<T>> {
  STATIC_ONLY(VectorTraits);

  static constexpr bool kNeedsDestruction =
      !std::is_trivially_destructible<blink::TraceWrapperV8Reference<T>>::value;

  // TraceWrapperV8Reference has non-trivial construction/copying/moving.
  // However, write barriers in Vector are properly emitted through
  // ConstructTraits and as such the type can be trivially initialized, cleared,
  // copied, and moved.
  static constexpr bool kCanInitializeWithMemset = true;
  static constexpr bool kCanClearUnusedSlotsWithMemset = true;
  // v8::TracedReference assumes that references uniquely point to an internal
  // node.
  static constexpr bool kCanCopyWithMemcpy = false;
  // TODO(chromium:1322114): Temporarily disable move with memcpy to evaluate
  // impact on crashers. Move should always be followed by a clear (non-dtor).
  static constexpr bool kCanMoveWithMemcpy = false;

  // TraceWrapperV8Reference supports concurrent tracing.
  static constexpr bool kCanTraceConcurrently = true;

  // Wanted behavior that should not break for performance reasons.
  static_assert(!kNeedsDestruction,
                "TraceWrapperV8Reference should be trivially destructible.");
};

template <typename T>
struct HashTraits<blink::TraceWrapperV8Reference<T>>
    : GenericHashTraits<blink::TraceWrapperV8Reference<T>> {
  STATIC_ONLY(HashTraits);
  static constexpr bool kCanTraceConcurrently = true;
  static constexpr bool kSupportsCompaction = true;
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BINDINGS_TRACE_WRAPPER_V8_REFERENCE_H_
