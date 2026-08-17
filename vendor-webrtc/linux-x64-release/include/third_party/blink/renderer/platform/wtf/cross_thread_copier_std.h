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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_CROSS_THREAD_COPIER_STD_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_CROSS_THREAD_COPIER_STD_H_

#include <memory>
#include <string>
#include <vector>

#include "third_party/blink/renderer/platform/wtf/cross_thread_copier.h"

namespace WTF {

// nullptr_t can be passed through without any changes.
template <>
struct CrossThreadCopier<std::nullptr_t>
    : public CrossThreadCopierPassThrough<std::nullptr_t> {
  STATIC_ONLY(CrossThreadCopier);
};

template <typename T, typename Deleter>
struct CrossThreadCopier<std::unique_ptr<T, Deleter>> {
  STATIC_ONLY(CrossThreadCopier);
  using Type = std::unique_ptr<T, Deleter>;
  static std::unique_ptr<T, Deleter> Copy(std::unique_ptr<T, Deleter> pointer) {
    return pointer;  // This is in fact a move.
  }
};

template <typename T, wtf_size_t inlineCapacity, typename Allocator>
struct CrossThreadCopier<
    Vector<std::unique_ptr<T>, inlineCapacity, Allocator>> {
  STATIC_ONLY(CrossThreadCopier);
  using Type = Vector<std::unique_ptr<T>, inlineCapacity, Allocator>;
  static Type Copy(Type pointer) {
    return pointer;  // This is in fact a move.
  }
};

template <>
struct CrossThreadCopier<std::vector<uint8_t>> {
  STATIC_ONLY(CrossThreadCopier);
  using Type = std::vector<uint8_t>;
  static Type Copy(Type value) { return value; }
};

template <class CharT, class Traits, class Allocator>
struct CrossThreadCopier<std::basic_string<CharT, Traits, Allocator>> {
  STATIC_ONLY(CrossThreadCopier);
  using Type = std::basic_string<CharT, Traits, Allocator>;
  static Type Copy(Type string) {
    return string;  // This is in fact a move.
  }
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_CROSS_THREAD_COPIER_STD_H_
