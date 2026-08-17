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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_CROSS_THREAD_COPIER_BASE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_CROSS_THREAD_COPIER_BASE_H_

#include "base/files/file.h"
#include "base/files/file_error_or.h"
#include "base/memory/scoped_refptr.h"
#include "base/memory/weak_ptr.h"
#include "mojo/public/cpp/base/big_buffer.h"
#include "third_party/blink/renderer/platform/wtf/cross_thread_copier.h"

namespace base {
template <typename T, typename Deleter>
class HeapArray;
template <typename, typename>
class RefCountedThreadSafe;
class TimeDelta;
class TimeTicks;
class Time;
class UnguessableToken;
}  // namespace base

namespace WTF {

template <typename T>
struct CrossThreadCopier<scoped_refptr<T>> {
  STATIC_ONLY(CrossThreadCopier);
  static_assert(IsSubclassOfTemplate<T, base::RefCountedThreadSafe>::value,
                "scoped_refptr<T> can be passed across threads only if T is "
                "ThreadSafeRefCounted or base::RefCountedThreadSafe.");
  using Type = scoped_refptr<T>;
  static scoped_refptr<T> Copy(scoped_refptr<T> pointer) { return pointer; }
};

template <typename T>
struct CrossThreadCopier<base::FileErrorOr<T>>
    : public CrossThreadCopierPassThrough<base::FileErrorOr<T>> {
  STATIC_ONLY(CrossThreadCopier);
};

template <>
struct CrossThreadCopier<base::TimeDelta>
    : public CrossThreadCopierPassThrough<base::TimeDelta> {
  STATIC_ONLY(CrossThreadCopier);
};

template <>
struct CrossThreadCopier<base::TimeTicks>
    : public CrossThreadCopierPassThrough<base::TimeTicks> {
  STATIC_ONLY(CrossThreadCopier);
};

template <>
struct CrossThreadCopier<base::Time>
    : public CrossThreadCopierPassThrough<base::Time> {
  STATIC_ONLY(CrossThreadCopier);
};

template <>
struct CrossThreadCopier<base::File> {
  STATIC_ONLY(CrossThreadCopier);
  using Type = base::File;
  static Type Copy(Type pointer) { return pointer; }
};

template <>
struct CrossThreadCopier<base::UnguessableToken>
    : public CrossThreadCopierPassThrough<base::UnguessableToken> {
  STATIC_ONLY(CrossThreadCopier);
};

template <>
struct CrossThreadCopier<mojo_base::BigBuffer> {
  STATIC_ONLY(CrossThreadCopier);
  using Type = mojo_base::BigBuffer;
  static Type Copy(Type&& value) { return std::move(value); }
};

template <typename T>
struct CrossThreadCopier<base::WeakPtr<T>>
    : public CrossThreadCopierPassThrough<base::WeakPtr<T>> {
  STATIC_ONLY(CrossThreadCopier);
};

template <typename T,
          typename Deleter,
          wtf_size_t inlineCapacity,
          typename Allocator>
struct CrossThreadCopier<
    Vector<base::HeapArray<T, Deleter>, inlineCapacity, Allocator>> {
  STATIC_ONLY(CrossThreadCopier);
  using Type = Vector<base::HeapArray<T, Deleter>, inlineCapacity, Allocator>;
  static Type Copy(Type pointer) {
    return pointer;  // This is in fact a move.
  }
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_CROSS_THREAD_COPIER_BASE_H_
