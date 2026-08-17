/*
 * Copyright (C) 2008 Apple Inc. All Rights Reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_STD_LIB_EXTRAS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_STD_LIB_EXTRAS_H_

#include <cstddef>
#include <cstdint>

#include "base/check.h"
#include "base/dcheck_is_on.h"
#include "build/build_config.h"
#include "third_party/blink/renderer/platform/wtf/leak_annotations.h"
#include "third_party/blink/renderer/platform/wtf/sanitizers.h"
#include "third_party/blink/renderer/platform/wtf/type_traits.h"

#if DCHECK_IS_ON()
#include "third_party/blink/renderer/platform/wtf/threading.h"
#endif

#define DEFINE_STATIC_LOCAL_IMPL(Type, Name, Arguments, allow_cross_thread)    \
  static WTF::StaticSingleton<Type> s_##Name(                                  \
      [&]() { return new WTF::StaticSingleton<Type>::WrapperType Arguments; }, \
      [&](void* leaked_ptr) {                                                  \
        new (leaked_ptr) WTF::StaticSingleton<Type>::WrapperType Arguments;    \
      });                                                                      \
  Type& Name = s_##Name.Get(allow_cross_thread)

// Use |DEFINE_STATIC_LOCAL()| to declare and define a static local variable
// (|static T;|) so that it is leaked and its destructors are not called at
// exit.
//
// A |DEFINE_STATIC_LOCAL()| static should only be used on the thread it was
// created on.
//
#define DEFINE_STATIC_LOCAL(Type, Name, Arguments) \
  DEFINE_STATIC_LOCAL_IMPL(Type, Name, Arguments, false)

// |DEFINE_THREAD_SAFE_STATIC_LOCAL()| is the cross-thread accessible variant
// of |DEFINE_STATIC_LOCAL()|; use it if the singleton can be accessed by
// multiple threads.
//
// TODO: rename as DEFINE_CROSS_THREAD_STATIC_LOCAL() ?
#define DEFINE_THREAD_SAFE_STATIC_LOCAL(Type, Name, Arguments) \
  DEFINE_STATIC_LOCAL_IMPL(Type, Name, Arguments, true)

namespace WTF {

template <typename Type>
class StaticSingleton final {
 public:
  template <typename T>
  struct Wrapper {
    using type = T;

    static T& Unwrap(T* singleton) { return *singleton; }
  };

  using WrapperType = typename Wrapper<Type>::type;

  template <typename HeapNew, typename PlacementNew>
  StaticSingleton(const HeapNew& heap_new, const PlacementNew& placement_new)
      : instance_(heap_new, placement_new)
#if DCHECK_IS_ON()
        ,
        safely_initialized_(WTF::IsBeforeThreadCreated()),
        thread_(WTF::CurrentThread())
#endif
  {
    static_assert(!WTF::IsGarbageCollectedType<Type>::value,
                  "Garbage collected objects must be wrapped in a Persistent");
    LEAK_SANITIZER_IGNORE_OBJECT(instance_.Get());
  }

  StaticSingleton(const StaticSingleton&) = delete;
  StaticSingleton& operator=(const StaticSingleton&) = delete;

  Type& Get([[maybe_unused]] bool allow_cross_thread_use) {
#if DCHECK_IS_ON()
    DCHECK(IsNotRacy(allow_cross_thread_use));
#endif
    return Wrapper<Type>::Unwrap(instance_.Get());
  }

  operator Type&() { return Get(); }

 private:
#if DCHECK_IS_ON()

  bool IsNotRacy(bool allow_cross_thread_use) const {
    // Make sure that singleton is safely initialized, or
    // keeps being called on the same thread if cross-thread
    // use is not permitted.
    return allow_cross_thread_use || safely_initialized_ ||
           thread_ == WTF::CurrentThread();
  }
#endif
  template <typename T, bool is_small = sizeof(T) <= 32>
  class InstanceStorage {
   public:
    template <typename HeapNew, typename PlacementNew>
    InstanceStorage(const HeapNew& heap_new, const PlacementNew&)
        : pointer_(heap_new()) {}
    T* Get() { return pointer_; }

   private:
    T* pointer_;
  };

  template <typename T>
  class InstanceStorage<T, true> {
   public:
    template <typename HeapNew, typename PlacementNew>
    InstanceStorage(const HeapNew&, const PlacementNew& placement_new) {
      placement_new(&object_);
    }
    T* Get() { return reinterpret_cast<T*>(object_); }

   private:
    alignas(T) char object_[sizeof(T)];
  };

  InstanceStorage<WrapperType> instance_;
#if DCHECK_IS_ON()
  bool safely_initialized_;
  base::PlatformThreadId thread_;
#endif
};

}  // namespace WTF

// Use this to declare and define a static local pointer to a ref-counted object
// so that it is leaked so that the object's destructors are not called at
// exit.  This macro should be used with ref-counted objects rather than
// DEFINE_STATIC_LOCAL macro, as this macro does not lead to an extra memory
// allocation.
#define DEFINE_STATIC_REF(type, name, arguments)  \
  static type* name = [](scoped_refptr<type> o) { \
    if (o)                                        \
      o->AddRef();                                \
    return o.get();                               \
  }(arguments);

/*
 * The reinterpret_cast<Type1*>([pointer to Type2]) expressions - where
 * sizeof(Type1) > sizeof(Type2) - cause the following warning on ARM with GCC:
 * increases required alignment of target type.
 *
 * An implicit or an extra static_cast<void*> bypasses the warning.
 * For more info see the following bugzilla entries:
 * - https://bugs.webkit.org/show_bug.cgi?id=38045
 * - http://gcc.gnu.org/bugzilla/show_bug.cgi?id=43976
 */
#if defined(ARCH_CPU_ARMEL) && defined(COMPILER_GCC)
template <typename Type>
bool isPointerTypeAlignmentOkay(Type* ptr) {
  return !(reinterpret_cast<intptr_t>(ptr) % __alignof__(Type));
}

template <typename TypePtr>
TypePtr reinterpret_cast_ptr(void* ptr) {
  DCHECK(isPointerTypeAlignmentOkay(reinterpret_cast<TypePtr>(ptr)));
  return reinterpret_cast<TypePtr>(ptr);
}

template <typename TypePtr>
TypePtr reinterpret_cast_ptr(const void* ptr) {
  DCHECK(isPointerTypeAlignmentOkay(reinterpret_cast<TypePtr>(ptr)));
  return reinterpret_cast<TypePtr>(ptr);
}
#else
template <typename Type>
bool isPointerTypeAlignmentOkay(Type*) {
  return true;
}
#define reinterpret_cast_ptr reinterpret_cast
#endif

template <typename TypePtr>
NO_SANITIZE_UNRELATED_CAST
TypePtr unsafe_reinterpret_cast_ptr(void* ptr) {
#if defined(ARCH_CPU_ARMEL) && defined(COMPILER_GCC)
  DCHECK(isPointerTypeAlignmentOkay(reinterpret_cast<TypePtr>(ptr)));
#endif
  return reinterpret_cast<TypePtr>(ptr);
}

template <typename TypePtr>
NO_SANITIZE_UNRELATED_CAST
TypePtr unsafe_reinterpret_cast_ptr(const void* ptr) {
#if defined(ARCH_CPU_ARMEL) && defined(COMPILER_GCC)
  DCHECK(isPointerTypeAlignmentOkay(reinterpret_cast<TypePtr>(ptr)));
#endif
  return reinterpret_cast<TypePtr>(ptr);
}

namespace WTF {

// Use the following macros to prevent errors caused by accidental
// implicit casting of function arguments.  For example, this can
// be used to prevent overflows from non-promoting conversions.
//
// Example:
//
// HAS_STRICTLY_TYPED_ARG
// void sendData(void* data, STRICTLY_TYPED_ARG(size))
// {
//    ALLOW_NUMERIC_ARG_TYPES_PROMOTABLE_TO(size_t);
//    ...
// }
//
// The previous example will prevent callers from passing, for example, an
// 'int'. On a 32-bit build, it will prevent use of an 'unsigned long long'.
#define HAS_STRICTLY_TYPED_ARG template <typename ActualArgType>
#define STRICTLY_TYPED_ARG(argName) ActualArgType argName
#define STRICT_ARG_TYPE(ExpectedArgType)                                     \
  static_assert(std::is_same<ActualArgType, ExpectedArgType>::value,         \
                "Strictly typed argument must be of type '" #ExpectedArgType \
                "'.")
#define ALLOW_NUMERIC_ARG_TYPES_PROMOTABLE_TO(ExpectedArgType)              \
  static_assert(                                                            \
      std::numeric_limits<ExpectedArgType>::is_integer ==                   \
          std::numeric_limits<ActualArgType>::is_integer,                   \
      "Conversion between integer and non-integer types not allowed.");     \
  static_assert(sizeof(ExpectedArgType) >= sizeof(ActualArgType),           \
                "Truncating conversions not allowed.");                     \
  static_assert(!std::numeric_limits<ActualArgType>::is_signed ||           \
                    std::numeric_limits<ExpectedArgType>::is_signed,        \
                "Signed to unsigned conversion not allowed.");              \
  static_assert((sizeof(ExpectedArgType) != sizeof(ActualArgType)) ||       \
                    (std::numeric_limits<ActualArgType>::is_signed ==       \
                     std::numeric_limits<ExpectedArgType>::is_signed),      \
                "Unsigned to signed conversion not allowed for types with " \
                "identical size (could overflow).");

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_STD_LIB_EXTRAS_H_
