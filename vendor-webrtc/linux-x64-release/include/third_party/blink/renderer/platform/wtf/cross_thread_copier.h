/*
 * Copyright (C) 2009, 2010 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_CROSS_THREAD_COPIER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_CROSS_THREAD_COPIER_H_

#include <type_traits>

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"

namespace WTF {

class SegmentedBuffer;
class String;

template <typename T>
struct CrossThreadCopierPassThrough {
  STATIC_ONLY(CrossThreadCopierPassThrough);
  typedef T Type;
  static Type Copy(const T& parameter) { return parameter; }
};

template <typename T>
struct CrossThreadCopierByValuePassThrough {
  STATIC_ONLY(CrossThreadCopierByValuePassThrough);
  typedef T Type;
  static Type Copy(T receiver) {
    return receiver;  // This is in fact a move.
  }
};

template <typename T, bool isArithmeticOrEnum>
struct CrossThreadCopierBase;

// Arithmetic values (integers or floats) and enums can be safely copied.
template <typename T>
struct CrossThreadCopierBase<T, true> : public CrossThreadCopierPassThrough<T> {
  STATIC_ONLY(CrossThreadCopierBase);
};

template <typename T>
struct CrossThreadCopier
    : public CrossThreadCopierBase<T,
                                   std::is_arithmetic<T>::value ||
                                       std::is_enum<T>::value> {
  STATIC_ONLY(CrossThreadCopier);
};

// To allow a type to be passed across threads using its copy constructor,
// provide a specialization of CrossThreadCopier<T>.
//
// * If the type is not defined in third_party/blink/
//   ==> Choose one of the existing cross_thread_copier_*.h or add new one,
//       and add a forward declaration and a CrossThreadCopier specialization
//       for the type to the file.
// * If the type is defined in third_party/blink/public/
//   ==> Add a forward declaration and a CrossThreadCopier specialization for
//       the type to cross_thread_copier_public.h.
// * If the type is defined in third_party/blink/
//   ==> Include cross_thread_copier.h from the header defining the type, and
//       add a CrossThreadCopier specialization for the type to the header.

template <typename T, wtf_size_t inlineCapacity, typename Allocator>
struct CrossThreadCopier<Vector<T, inlineCapacity, Allocator>> {
  STATIC_ONLY(CrossThreadCopier);
  static_assert(std::is_enum<T>() || std::is_arithmetic<T>(),
                "Vectors of this type are not known to be safe to pass to "
                "another thread.");
  using Type = Vector<T, inlineCapacity, Allocator>;
  static Type Copy(Type value) { return value; }
};

template <wtf_size_t inlineCapacity, typename Allocator>
struct CrossThreadCopier<Vector<String, inlineCapacity, Allocator>>
    : public CrossThreadCopierPassThrough<
          Vector<String, inlineCapacity, Allocator>> {
  STATIC_ONLY(CrossThreadCopier);
};

template <>
struct CrossThreadCopier<SegmentedBuffer>
    : CrossThreadCopierByValuePassThrough<SegmentedBuffer> {
  STATIC_ONLY(CrossThreadCopier);
};

template <>
struct CrossThreadCopier<AtomicString>
    : public CrossThreadCopierPassThrough<AtomicString> {
  STATIC_ONLY(CrossThreadCopier);
};

template <>
struct CrossThreadCopier<String> : public CrossThreadCopierPassThrough<String> {
  STATIC_ONLY(CrossThreadCopier);
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_CROSS_THREAD_COPIER_H_
