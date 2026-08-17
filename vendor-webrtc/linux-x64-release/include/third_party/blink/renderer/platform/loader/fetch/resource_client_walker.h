/*
    Copyright (C) 1998 Lars Knoll (knoll@mpi-hd.mpg.de)
    Copyright (C) 2001 Dirk Mueller <mueller@kde.org>
    Copyright (C) 2004, 2005, 2006, 2007 Apple Inc. All rights reserved.

    This library is free software; you can redistribute it and/or
    modify it under the terms of the GNU Library General Public
    License as published by the Free Software Foundation; either
    version 2 of the License, or (at your option) any later version.

    This library is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
    Library General Public License for more details.

    You should have received a copy of the GNU Library General Public License
    along with this library; see the file COPYING.LIB.  If not, write to
    the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
    Boston, MA 02110-1301, USA.

    This class provides all functionality needed for loading images, style
    sheets and html pages from the web. It has a memory cache for these objects.
*/

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_CLIENT_WALKER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_CLIENT_WALKER_H_

#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_counted_set.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource_client.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// Call this "walker" instead of iterator so people won't expect Qt or STL-style
// iterator interface. Just keep calling next() on this. It's safe from
// deletions of items.
template <typename T>
class ResourceClientWalker {
  STACK_ALLOCATED();

 public:
  explicit ResourceClientWalker(
      const HeapHashCountedSet<WeakMember<ResourceClient>>& set)
      : client_set_(set), client_vector_(client_set_.Values()) {}

  T* Next() {
    wtf_size_t size = client_vector_.size();
    while (index_ < size) {
      ResourceClient* next = client_vector_[index_++];
      DCHECK(next);
      if (client_set_.Contains(next)) {
        return static_cast<T*>(next);
      }
    }
    return nullptr;
  }

 private:
  const HeapHashCountedSet<WeakMember<ResourceClient>>& client_set_;
  HeapVector<Member<ResourceClient>> client_vector_;
  wtf_size_t index_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_CLIENT_WALKER_H_
