// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_ALLOCATOR_ALLOCATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_ALLOCATOR_ALLOCATOR_H_

#include "base/check.h"
#include "base/memory/stack_allocated.h"
#include "build/build_config.h"
#include "third_party/blink/renderer/platform/wtf/allocator/partitions.h"
#include "third_party/blink/renderer/platform/wtf/type_traits.h"

namespace WTF {

using base::NotNullTag;

namespace internal {
// A dummy class used in following macros.
class __thisIsHereToForceASemicolonAfterThisMacro;
}  // namespace internal

// Classes that contain references to garbage-collected objects but aren't
// themselves garbaged allocated, have some extra macros available which
// allows their use to be restricted to cases where the garbage collector
// is able to discover their references. These macros will be useful for
// non-garbage-collected objects to avoid unintended allocations.
//
// STACK_ALLOCATED() classes may contain raw pointers to garbage-collected
// objects.
//
// DISALLOW_NEW(): Cannot be allocated with new operators but can be a
// part of object, a value object in collections or stack allocated. If it has
// Members you need a trace method and the containing object needs to call that
// trace method.
//
#define DISALLOW_NEW()                                          \
 public:                                                        \
  using IsDisallowNewMarker [[maybe_unused]] = int;             \
  void* operator new(size_t, WTF::NotNullTag, void* location) { \
    return location;                                            \
  }                                                             \
  void* operator new(size_t, void* location) {                  \
    return location;                                            \
  }                                                             \
                                                                \
 private:                                                       \
  void* operator new(size_t) = delete;                          \
                                                                \
 public:                                                        \
  friend class ::WTF::internal::__thisIsHereToForceASemicolonAfterThisMacro

#define STATIC_ONLY(Type)                                      \
  Type() = delete;                                             \
  Type(const Type&) = delete;                                  \
  Type& operator=(const Type&) = delete;                       \
  void* operator new(size_t) = delete;                         \
  void* operator new(size_t, WTF::NotNullTag, void*) = delete; \
  void* operator new(size_t, void*) = delete

// Provides customizable overrides of fastMalloc/fastFree and operator
// new/delete
//
// Provided functionality:
//    Macro: USING_FAST_MALLOC
//
// Example usage:
//    class Widget {
//        USING_FAST_MALLOC(Widget)
//    ...
//    };
//
//    struct Data {
//        USING_FAST_MALLOC(Data)
//    public:
//    ...
//    };
//

// In official builds, do not include type info string literals to avoid
// bloating the binary.
#if defined(OFFICIAL_BUILD)
#define WTF_HEAP_PROFILER_TYPE_NAME(T) nullptr
#else
#define WTF_HEAP_PROFILER_TYPE_NAME(T) ::WTF::GetStringWithTypeName<T>()
#endif

// Both of these macros enable fast malloc and provide type info to the heap
// profiler. The regular macro does not provide type info in official builds,
// to avoid bloating the binary with type name strings. The |WITH_TYPE_NAME|
// variant provides type info unconditionally, so it should be used sparingly.
// Furthermore, the |WITH_TYPE_NAME| variant does not work if |type| is a
// template argument; |USING_FAST_MALLOC| does.
#define USING_FAST_MALLOC(type) \
  USING_FAST_MALLOC_INTERNAL(type, WTF_HEAP_PROFILER_TYPE_NAME(type))
#define USING_FAST_MALLOC_WITH_TYPE_NAME(type) \
  USING_FAST_MALLOC_INTERNAL(type, #type)

#define USING_FAST_MALLOC_INTERNAL(type, typeName)              \
 public:                                                        \
  void* operator new(size_t, void* p) {                         \
    return p;                                                   \
  }                                                             \
  void* operator new[](size_t, void* p) {                       \
    return p;                                                   \
  }                                                             \
                                                                \
  void* operator new(size_t size) {                             \
    return ::WTF::Partitions::FastMalloc(size, typeName);       \
  }                                                             \
                                                                \
  void operator delete(void* p) {                               \
    ::WTF::Partitions::FastFree(p);                             \
  }                                                             \
                                                                \
  void* operator new[](size_t size) {                           \
    return ::WTF::Partitions::FastMalloc(size, typeName);       \
  }                                                             \
                                                                \
  void operator delete[](void* p) {                             \
    ::WTF::Partitions::FastFree(p);                             \
  }                                                             \
  void* operator new(size_t, WTF::NotNullTag, void* location) { \
    DCHECK(location);                                           \
    return location;                                            \
  }                                                             \
                                                                \
 private:                                                       \
  friend class ::WTF::internal::__thisIsHereToForceASemicolonAfterThisMacro

}  // namespace WTF

// This version of placement new omits a 0 check.
inline void* operator new(size_t, WTF::NotNullTag, void* location) {
  DCHECK(location);
  return location;
}

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_ALLOCATOR_ALLOCATOR_H_
