// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
//
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// PLEASE READ: Do you really need a singleton? If possible, use a
// function-local static of type base::NoDestructor<T> instead:
//
// Factory& Factory::GetInstance() {
//   static base::NoDestructor<Factory> instance;
//   return *instance;
// }
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
//
// Singletons make it hard to determine the lifetime of an object, which can
// lead to buggy code and spurious crashes.
//
// Instead of adding another singleton into the mix, try to identify either:
//   a) An existing singleton that can manage your object's lifetime
//   b) Locations where you can deterministically create the object and pass
//      into other objects
//
// If you absolutely need a singleton, please keep them as trivial as possible
// and ideally a leaf dependency. Singletons get problematic when they attempt
// to do too much in their destructor or have circular dependencies.

#ifndef BASE_MEMORY_SINGLETON_H_
#define BASE_MEMORY_SINGLETON_H_

#include <atomic>

#include "base/dcheck_is_on.h"
#include "base/lazy_instance_helpers.h"
#include "base/threading/thread_restrictions.h"

namespace base {

// Default traits for Singleton<Type>. Calls operator new and operator delete on
// the object. Registers automatic deletion at process exit.
// Overload if you need arguments or another memory allocation function.
template <typename Type>
struct DefaultSingletonTraits {
  // Allocates the object.
  static Type* New() {
    // The parenthesis is very important here; it forces POD type
    // initialization.
    return new Type();
  }

  // Destroys the object.
  static void Delete(Type* x) { delete x; }

  // Set to true to automatically register deletion of the object on process
  // exit. See below for the required call that makes this happen.
  static const bool kRegisterAtExit = true;

#if DCHECK_IS_ON()
  // Set to false to disallow access on a non-joinable thread.  This is
  // different from kRegisterAtExit because StaticMemorySingletonTraits allows
  // access on non-joinable threads, and gracefully handles this.
  static const bool kAllowedToAccessOnNonjoinableThread = false;
#endif
};

// Alternate traits for use with the Singleton<Type>.  Identical to
// DefaultSingletonTraits except that the Singleton will not be cleaned up
// at exit.
template <typename Type>
struct LeakySingletonTraits : public DefaultSingletonTraits<Type> {
  static const bool kRegisterAtExit = false;
#if DCHECK_IS_ON()
  static const bool kAllowedToAccessOnNonjoinableThread = true;
#endif
};

// Alternate traits for use with the Singleton<Type>.  Allocates memory
// for the singleton instance from a static buffer.  The singleton will
// be cleaned up at exit, but can't be revived after destruction unless
// the ResurrectForTesting() method is called.
//
// This is useful for a certain category of things, notably logging and
// tracing, where the singleton instance is of a type carefully constructed to
// be safe to access post-destruction.
// In logging and tracing you'll typically get stray calls at odd times, like
// during static destruction, thread teardown and the like, and there's a
// termination race on the heap-based singleton - e.g. if one thread calls
// get(), but then another thread initiates AtExit processing, the first thread
// may call into an object residing in unallocated memory. If the instance is
// allocated from the data segment, then this is survivable.
//
// The destructor is to deallocate system resources, in this case to unregister
// a callback the system will invoke when logging levels change. Note that
// this is also used in e.g. Chrome Frame, where you have to allow for the
// possibility of loading briefly into someone else's process space, and
// so leaking is not an option, as that would sabotage the state of your host
// process once you've unloaded.
template <typename Type>
struct StaticMemorySingletonTraits {
  // WARNING: User has to support a New() which returns null.
  static Type* New() {
    // Only constructs once and returns pointer; otherwise returns null.
    if (dead_.exchange(true, std::memory_order_relaxed)) {
      return nullptr;
    }

    return new (buffer_) Type();
  }

  static void Delete(Type* p) {
    if (p) {
      p->Type::~Type();
    }
  }

  static const bool kRegisterAtExit = true;

#if DCHECK_IS_ON()
  static const bool kAllowedToAccessOnNonjoinableThread = true;
#endif

  static void ResurrectForTesting() {
    dead_.store(false, std::memory_order_relaxed);
  }

 private:
  alignas(Type) static char buffer_[sizeof(Type)];
  // Signal the object was already deleted, so it is not revived.
  static std::atomic<bool> dead_;
};

template <typename Type>
alignas(Type) char StaticMemorySingletonTraits<Type>::buffer_[sizeof(Type)];
template <typename Type>
std::atomic<bool> StaticMemorySingletonTraits<Type>::dead_ = false;

// The Singleton<Type, Traits, DifferentiatingType> class manages a single
// instance of Type which will be created on first use and will be destroyed at
// normal process exit). The Trait::Delete function will not be called on
// abnormal process exit.
//
// DifferentiatingType is used as a key to differentiate two different
// singletons having the same memory allocation functions but serving a
// different purpose. This is mainly used for Locks serving different purposes.
//
// Example usage:
//
// In your header:
//   namespace base {
//   template <typename T>
//   struct DefaultSingletonTraits;
//   }
//   class FooClass {
//    public:
//     static FooClass* GetInstance();  <-- See comment below on this.
//
//     FooClass(const FooClass&) = delete;
//     FooClass& operator=(const FooClass&) = delete;
//
//     void Bar() { ... }
//
//    private:
//     FooClass() { ... }
//     friend struct base::DefaultSingletonTraits<FooClass>;
//   };
//
// In your source file:
//  #include "base/memory/singleton.h"
//  FooClass* FooClass::GetInstance() {
//    return base::Singleton<FooClass>::get();
//  }
//
// Or for leaky singletons:
//  #include "base/memory/singleton.h"
//  FooClass* FooClass::GetInstance() {
//    return base::Singleton<
//        FooClass, base::LeakySingletonTraits<FooClass>>::get();
//  }
//
// And to call methods on FooClass:
//   FooClass::GetInstance()->Bar();
//
// NOTE: The method accessing Singleton<T>::get() has to be named as GetInstance
// and it is important that FooClass::GetInstance() is not inlined in the
// header. This makes sure that when source files from multiple targets include
// this header they don't end up with different copies of the inlined code
// creating multiple copies of the singleton.
//
// Singleton<> has no non-static members and doesn't need to actually be
// instantiated.
//
// This class is itself thread-safe. The underlying Type must of course be
// thread-safe if you want to use it concurrently. Two parameters may be tuned
// depending on the user's requirements.
//
// Glossary:
//   RAE = kRegisterAtExit
//
// On every platform, if Traits::RAE is true, the singleton will be destroyed at
// process exit. More precisely it uses AtExitManager which requires an
// object of this type to be instantiated. AtExitManager mimics the semantics
// of atexit() such as LIFO order but under Windows is safer to call. For more
// information see at_exit.h.
//
// If Traits::RAE is false, the singleton will not be freed at process exit,
// thus the singleton will be leaked if it is ever accessed. Traits::RAE
// shouldn't be false unless absolutely necessary. Remember that the heap where
// the object is allocated may be destroyed by the CRT anyway.
//
// Caveats:
// (a) Every call to get(), operator->() and operator*() incurs some overhead
//     (16ns on my P4/2.8GHz) to check whether the object has already been
//     initialized.  You may wish to cache the result of get(); it will not
//     change.
//
// (b) Your factory function must never throw an exception. This class is not
//     exception-safe.
//

template <typename Type,
          typename Traits = DefaultSingletonTraits<Type>,
          typename DifferentiatingType = Type>
class Singleton {
 private:
  // A class T using the Singleton<T> pattern should declare a GetInstance()
  // method and call Singleton::get() from within that. T may also declare a
  // GetInstanceIfExists() method to invoke Singleton::GetIfExists().
  friend Type;

  // This class is safe to be constructed and copy-constructed since it has no
  // member.

  // Returns a pointer to the one true instance of the class.
  static Type* get() {
#if DCHECK_IS_ON()
    if (!Traits::kAllowedToAccessOnNonjoinableThread) {
      internal::AssertSingletonAllowed();
    }
#endif

    return subtle::GetOrCreateLazyPointer(
        instance_, &CreatorFunc, nullptr,
        Traits::kRegisterAtExit ? OnExit : nullptr, nullptr);
  }

  // Returns the same result as get() if the instance exists but doesn't
  // construct it (and returns null) if it doesn't.
  static Type* GetIfExists() {
#if DCHECK_IS_ON()
    if (!Traits::kAllowedToAccessOnNonjoinableThread) {
      internal::AssertSingletonAllowed();
    }
#endif

    if (!instance_.load(std::memory_order_relaxed)) {
      return nullptr;
    }

    // Need to invoke get() nonetheless as some Traits return null after
    // destruction (even though |instance_| still holds garbage).
    return get();
  }

  // Internal method used as an adaptor for GetOrCreateLazyPointer(). Do not use
  // outside of that use case.
  static Type* CreatorFunc(void* /* creator_arg*/) { return Traits::New(); }

  // Adapter function for use with AtExit().  This should be called single
  // threaded, so don't use atomic operations.
  // Calling OnExit while singleton is in use by other threads is a mistake.
  static void OnExit(void* /*unused*/) {
    // AtExit should only ever be register after the singleton instance was
    // created.  We should only ever get here with a valid instance_ pointer.
    Traits::Delete(
        reinterpret_cast<Type*>(instance_.load(std::memory_order_relaxed)));
    instance_.store(0, std::memory_order_relaxed);
  }
  static std::atomic<uintptr_t> instance_;
};

template <typename Type, typename Traits, typename DifferentiatingType>
std::atomic<uintptr_t> Singleton<Type, Traits, DifferentiatingType>::instance_ =
    0;

}  // namespace base

#endif  // BASE_MEMORY_SINGLETON_H_
