// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
//
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!! DEPRECATED !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Please don't introduce new instances of LazyInstance<T>. Use a function-local
// static of type base::NoDestructor<T> instead:
//
// Factory& Factory::GetInstance() {
//   static base::NoDestructor<Factory> instance;
//   return *instance;
// }
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
//
// The LazyInstance<Type, Traits> class manages a single instance of Type,
// which will be lazily created on the first time it's accessed.  This class is
// useful for places you would normally use a function-level static, but you
// need to have guaranteed thread-safety.  The Type constructor will only ever
// be called once, even if two threads are racing to create the object.  Get()
// and Pointer() will always return the same, completely initialized instance.
// When the instance is constructed it is registered with AtExitManager.  The
// destructor will be called on program exit.
//
// LazyInstance is completely thread safe, assuming that you create it safely.
// The class was designed to be POD initialized, so it shouldn't require a
// static constructor.  It really only makes sense to declare a LazyInstance as
// a global variable using the LAZY_INSTANCE_INITIALIZER initializer.
//
// LazyInstance is similar to Singleton, except it does not have the singleton
// property.  You can have multiple LazyInstance's of the same type, and each
// will manage a unique instance.  It also preallocates the space for Type, as
// to avoid allocating the Type instance on the heap.  This may help with the
// performance of creating the instance, and reducing heap fragmentation.  This
// requires that Type be a complete type so we can determine the size.
//
// Example usage:
//   static LazyInstance<MyClass>::Leaky inst = LAZY_INSTANCE_INITIALIZER;
//   void SomeMethod() {
//     inst.Get().SomeMethod();  // MyClass::SomeMethod()
//
//     MyClass* ptr = inst.Pointer();
//     ptr->DoDoDo();  // MyClass::DoDoDo
//   }

#ifndef BASE_LAZY_INSTANCE_H_
#define BASE_LAZY_INSTANCE_H_

#include <atomic>
#include <new>  // For placement new.

#include "base/check_op.h"
#include "base/dcheck_is_on.h"
#include "base/debug/leak_annotations.h"
#include "base/lazy_instance_helpers.h"
#include "base/threading/thread_restrictions.h"
#include "build/build_config.h"

// LazyInstance uses its own struct initializer-list style static
// initialization, which does not require a constructor.
#define LAZY_INSTANCE_INITIALIZER \
  {}

namespace base {

template <typename Type>
struct LazyInstanceTraitsBase {
  static Type* New(void* instance) {
    DCHECK_EQ(reinterpret_cast<uintptr_t>(instance) & (alignof(Type) - 1), 0u);
    // Use placement new to initialize our instance in our preallocated space.
    // The parenthesis is very important here to force POD type initialization.
    return new (instance) Type();
  }

  static void CallDestructor(Type* instance) {
    // Explicitly call the destructor.
    instance->~Type();
  }
};

// We pull out some of the functionality into non-templated functions, so we
// can implement the more complicated pieces out of line in the .cc file.
namespace internal {

// This traits class causes destruction the contained Type at process exit via
// AtExitManager. This is probably generally not what you want. Instead, prefer
// Leaky below.
template <typename Type>
struct DestructorAtExitLazyInstanceTraits {
  static const bool kRegisterOnExit = true;
#if DCHECK_IS_ON()
  static const bool kAllowedToAccessOnNonjoinableThread = false;
#endif

  static Type* New(void* instance) {
    return LazyInstanceTraitsBase<Type>::New(instance);
  }

  static void Delete(Type* instance) {
    LazyInstanceTraitsBase<Type>::CallDestructor(instance);
  }
};

// Use LazyInstance<T>::Leaky for a less-verbose call-site typedef; e.g.:
// base::LazyInstance<T>::Leaky my_leaky_lazy_instance;
// instead of:
// base::LazyInstance<T, base::internal::LeakyLazyInstanceTraits<T> >
// my_leaky_lazy_instance;
// (especially when T is MyLongTypeNameImplClientHolderFactory).
// Only use this internal::-qualified verbose form to extend this traits class
// (depending on its implementation details).
template <typename Type>
struct LeakyLazyInstanceTraits {
  static const bool kRegisterOnExit = false;
#if DCHECK_IS_ON()
  static const bool kAllowedToAccessOnNonjoinableThread = true;
#endif

  static Type* New(void* instance) {
    ANNOTATE_SCOPED_MEMORY_LEAK;
    return LazyInstanceTraitsBase<Type>::New(instance);
  }
  static void Delete(Type* instance) {}
};

template <typename Type>
struct ErrorMustSelectLazyOrDestructorAtExitForLazyInstance {};

}  // namespace internal

template <
    typename Type,
    typename Traits =
        internal::ErrorMustSelectLazyOrDestructorAtExitForLazyInstance<Type>>
class LazyInstance {
 public:
  // Do not define a destructor, as doing so makes LazyInstance a
  // non-POD-struct. We don't want that because then a static initializer will
  // be created to register the (empty) destructor with atexit() under MSVC, for
  // example. We handle destruction of the contained Type class explicitly via
  // the OnExit member function, where needed.
  // ~LazyInstance() {}

  // Convenience typedef to avoid having to repeat Type for leaky lazy
  // instances.
  typedef LazyInstance<Type, internal::LeakyLazyInstanceTraits<Type>> Leaky;
  typedef LazyInstance<Type, internal::DestructorAtExitLazyInstanceTraits<Type>>
      DestructorAtExit;

  Type& Get() { return *Pointer(); }

  Type* Pointer() {
#if DCHECK_IS_ON()
    if (!Traits::kAllowedToAccessOnNonjoinableThread) {
      internal::AssertSingletonAllowed();
    }
#endif

    return subtle::GetOrCreateLazyPointer(
        private_instance_, &Traits::New, private_buf_,
        Traits::kRegisterOnExit ? OnExit : nullptr, this);
  }

  // Returns true if the lazy instance has been created.  Unlike Get() and
  // Pointer(), calling IsCreated() will not instantiate the object of Type.
  bool IsCreated() {
    // Return true (i.e. "created") if |private_instance_| is either being
    // created right now (i.e. |private_instance_| has value of
    // internal::kLazyInstanceStateCreating) or was already created (i.e.
    // |private_instance_| has any other non-zero value).
    return 0 != private_instance_.load(std::memory_order_relaxed);
  }

  // MSVC gives a warning that the alignment expands the size of the
  // LazyInstance struct to make the size a multiple of the alignment. This
  // is expected in this case.
#if BUILDFLAG(IS_WIN)
#pragma warning(push)
#pragma warning(disable : 4324)
#endif

  // Effectively private: member data is only public to allow the linker to
  // statically initialize it and to maintain a POD class. DO NOT USE FROM
  // OUTSIDE THIS CLASS.
  std::atomic<uintptr_t> private_instance_;

  // Preallocated space for the Type instance.
  alignas(Type) char private_buf_[sizeof(Type)];

#if BUILDFLAG(IS_WIN)
#pragma warning(pop)
#endif

 private:
  Type* instance() {
    return reinterpret_cast<Type*>(
        private_instance_.load(std::memory_order_relaxed));
  }

  // Adapter function for use with AtExit.  This should be called single
  // threaded, so don't synchronize across threads.
  // Calling OnExit while the instance is in use by other threads is a mistake.
  static void OnExit(void* lazy_instance) {
    LazyInstance<Type, Traits>* me =
        reinterpret_cast<LazyInstance<Type, Traits>*>(lazy_instance);
    Traits::Delete(me->instance());
    me->private_instance_.store(0, std::memory_order_relaxed);
  }
};

}  // namespace base

#endif  // BASE_LAZY_INSTANCE_H_
