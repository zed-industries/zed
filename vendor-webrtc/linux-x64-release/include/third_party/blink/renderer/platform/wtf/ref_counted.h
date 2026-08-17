/*
 * Copyright (C) 2006, 2007, 2008, 2009, 2010 Apple Inc. All rights reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_REF_COUNTED_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_REF_COUNTED_H_

#include "base/memory/ref_counted.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace WTF {

template <typename T, typename Traits>
class RefCounted;

template <typename T>
struct DefaultRefCountedTraits {
  static void Destruct(const T* x) {
    WTF::RefCounted<T, DefaultRefCountedTraits>::DeleteInternal(x);
  }
};

template <typename T, typename Traits = DefaultRefCountedTraits<T>>
class RefCounted : public base::RefCounted<T, Traits> {
  // Put |T| in here instead of |RefCounted| so the heap profiler reports |T|
  // instead of |RefCounted<T>|. This does not affect overloading of operator
  // new.
  USING_FAST_MALLOC(T);

 public:
  REQUIRE_ADOPTION_FOR_REFCOUNTED_TYPE();

 private:
  friend struct DefaultRefCountedTraits<T>;

  template <typename U>
  static void DeleteInternal(const U* x) {
    delete x;
  }
};

}  // namespace WTF

using WTF::RefCounted;

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_REF_COUNTED_H_
