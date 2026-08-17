/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_PRIVATE_PTR_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_PRIVATE_PTR_H_

#include "base/check.h"
#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/platform/heap/cross_thread_persistent.h"  // nogncheck
#include "third_party/blink/renderer/platform/heap/persistent.h"  // nogncheck
#include "third_party/blink/renderer/platform/wtf/type_traits.h"  // nogncheck

#include <cstddef>

namespace WTF {
template <class T, typename Traits>
class ThreadSafeRefCounted;
}

namespace blink {

// By default, the destruction of a WebPrivatePtrForGC and
// WebPrivatePtrForRefCounted must happen on the same thread that created it,
// but can optionally be allowed to happen on another thread.
enum class WebPrivatePtrDestruction {
  kSameThread,
  kCrossThread,
};

// The WebPrivatePtrForGC holds by default a strong reference to its Blink
// object, but Blink GC managed objects also support keeping a weak reference by
// way of WebPrivatePtrForGC.
enum class WebPrivatePtrStrength {
  kNormal,
  kWeak,
};

// WebPrivatePtrForRefCounted is an opaque reference to a Blink-internal class
// that can keep an object alive over the Blink API boundary. The object
// referred to must be reference counted. The pointer must be set and cleared
// from within Blink. Outside of Blink, WebPrivatePtrForRefCounted only allows
// constructing an empty reference and checking for an empty reference.
//
// A typical implementation of a class which uses WebPrivatePtrForRefCounted
// might look like this:
//
//   // WebFoo.h
//   class WebFoo {
//    public:
//     BLINK_EXPORT ~WebFoo();
//     WebFoo() = default;
//     WebFoo(const WebFoo& other) { Assign(other); }
//     WebFoo& operator=(const WebFoo& other) {
//       Assign(other);
//       return *this;
//     }
//     // Implemented in the .cc file as it requires full definition of Foo.
//     BLINK_EXPORT void Assign(const WebFoo&);
//
//     // Methods that are exposed to Chromium and which are specific to
//     // WebFoo go here.
//     BLINK_EXPORT DoWebFooThing();
//
//     // Methods that are used only by other Blink classes should only be
//     // declared when INSIDE_BLINK is set.
//     #if INSIDE_BLINK
//     WebFoo(scoped_refptr<Foo>);
//     #endif
//
//     private:
//      WebPrivatePtrForRefCounted<Foo> private_;
//    };
//
//    // WebFoo.cpp
//    WebFoo::~WebFoo() { private_.Reset(); }
//    WebFoo::WebFoo() = default;
//    WebFoo::WebFoo(const WebFoo& other) { Assign(other); }
//    void WebFoo::Assign(const WebFoo& other) { ... }
//
template <typename T,
          WebPrivatePtrDestruction PtrDestruction =
              WebPrivatePtrDestruction::kSameThread>
class WebPrivatePtrForRefCounted final {
 public:
  WebPrivatePtrForRefCounted() = default;

  ~WebPrivatePtrForRefCounted() {
    // We don't destruct the object pointed by storage_ here because we don't
    // want to expose destructors of core classes to embedders which is the case
    // for reference counted objects. The embedder should call Reset() manually
    // in destructors of classes with WebPrivatePtrForRefCounted members.
    CHECK(IsNull());
  }

  // Disable the copy constructor; classes that contain a
  // WebPrivatePtrForRefCounted should implement their copy constructor using
  // assign().
  WebPrivatePtrForRefCounted(const WebPrivatePtrForRefCounted&) = delete;

  bool IsNull() const { return !*reinterpret_cast<void* const*>(&storage_); }

  explicit operator bool() const { return !IsNull(); }

#if INSIDE_BLINK
  void Reset() { AsRefPtr().reset(); }

  T* Get() const { return AsRefPtr().get(); }

  T& operator*() const {
    DCHECK(!IsNull());
    return *Get();
  }

  T* operator->() const {
    DCHECK(!IsNull());
    return Get();
  }

  template <typename U>
  // NOLINTNEXTLINE(google-explicit-constructor)
  WebPrivatePtrForRefCounted(U&& ptr) : WebPrivatePtrForRefCounted() {
    static_assert(
        PtrDestruction == WebPrivatePtrDestruction::kSameThread ||
            WTF::IsSubclassOfTemplate<T, WTF::ThreadSafeRefCounted>::value,
        "Cross thread destructible class must derive from "
        "ThreadSafeRefCounted<>");
    AsRefPtr() = std::forward<U>(ptr);
  }

  template <typename U>
  WebPrivatePtrForRefCounted& operator=(U&& ptr) {
    AsRefPtr() = std::forward<U>(ptr);
    return *this;
  }

  WebPrivatePtrForRefCounted& operator=(
      const WebPrivatePtrForRefCounted& other) {
    AsRefPtr() = other.AsRefPtr();
    return *this;
  }

#else   // !INSIDE_BLINK
  // Disable the assignment operator; we define it above for when
  // INSIDE_BLINK is set, but we need to make sure that it is not
  // used outside there; the compiler-provided version won't handle reference
  // counting properly.
  WebPrivatePtrForRefCounted& operator=(
      const WebPrivatePtrForRefCounted& other) = delete;
#endif  // !INSIDE_BLINK

 private:
#if INSIDE_BLINK
  scoped_refptr<T>& AsRefPtr() {
    return *reinterpret_cast<scoped_refptr<T>*>(&storage_);
  }
  const scoped_refptr<T>& AsRefPtr() const {
    return const_cast<WebPrivatePtrForRefCounted*>(this)->AsRefPtr();
  }
#endif  // INSIDE_BLINK

  alignas(scoped_refptr<T>) std::byte storage_[sizeof(scoped_refptr<T>)] = {
      std::byte{0}};
};

// Translates WebPrivatePtrDestruction and WebPrivatePtrStrength to the
// appropriate Blink Persistent type.
template <typename T, WebPrivatePtrDestruction, WebPrivatePtrStrength>
struct WebPrivatePtrPersistentStorageType;

#define TRAIT_TO_TYPE_LIST(V)                        \
  V(kSameThread, kNormal, Persistent<T>)             \
  V(kSameThread, kWeak, WeakPersistent<T>)           \
  V(kCrossThread, kNormal, CrossThreadPersistent<T>) \
  V(kCrossThread, kWeak, CrossThreadWeakPersistent<T>)

#define DEFINE_TYPE_SPECIALIZATION(PtrDestruction, PtrStrengh, ReferenceType) \
  template <typename T>                                                       \
  struct WebPrivatePtrPersistentStorageType<                                  \
      T, WebPrivatePtrDestruction::PtrDestruction,                            \
      WebPrivatePtrStrength::PtrStrengh> {                                    \
    using Type = ReferenceType;                                               \
  };

TRAIT_TO_TYPE_LIST(DEFINE_TYPE_SPECIALIZATION)
#undef DEFINE_TYPE_TRAIT
#undef TRAIT_TO_TYPE_LIST

// For a general description see WebPrivatePtrForRefCounted. The difference is
// that WebPrivatePtrForGC points to a garbage collected object. The destructor
// of WebPrivatePtrForGC automatically clears the pointer as there's no
// immediate effect on other objects' destruction behavior.
//
// The implementation can use Blink handle types directly as the ctor/dtor can
// be used with forward declared types.
template <typename T,
          WebPrivatePtrDestruction PtrDestruction =
              WebPrivatePtrDestruction::kSameThread,
          WebPrivatePtrStrength PtrStrength = WebPrivatePtrStrength::kNormal>
class WebPrivatePtrForGC final {
 public:
  explicit WebPrivatePtrForGC(
      const PersistentLocation& loc = PERSISTENT_LOCATION_FROM_HERE)
      : handle_(loc) {}

  // GCed pointers can be immediately destroyed as the don't directly invoke
  // destructors of the value pointed to. For the same reason the destructor can
  // also be used with forward declared types.
  ~WebPrivatePtrForGC() = default;

  // Disable the copy constructor; classes that contain a WebPrivatePtrForGC
  // should implement their copy constructor using assign().
  WebPrivatePtrForGC(const WebPrivatePtrForGC&) = delete;

  bool IsNull() const { return !static_cast<bool>(handle_); }

  explicit operator bool() const { return !IsNull(); }

#if INSIDE_BLINK
  void Reset() { handle_.Clear(); }

  T* Get() const { return handle_.Get(); }

  T& operator*() const {
    DCHECK(!IsNull());
    return *Get();
  }

  T* operator->() const {
    DCHECK(!IsNull());
    return Get();
  }

  template <typename U>
  // NOLINTNEXTLINE(google-explicit-constructor)
  WebPrivatePtrForGC(
      U&& ptr,
      const PersistentLocation& loc = PERSISTENT_LOCATION_FROM_HERE)
      : handle_(std::forward<U>(ptr), loc) {
    static_assert(WTF::IsGarbageCollectedType<T>::value,
                  "WebPrivatePtrForGC can only be used on T inheriting from "
                  "GarbageCollected or GarbageCollectedMixin");
  }

  template <typename U>
  WebPrivatePtrForGC& operator=(U&& ptr) {
    handle_ = std::forward<U>(ptr);
    return *this;
  }

  WebPrivatePtrForGC& operator=(const WebPrivatePtrForGC& other) {
    handle_ = other.handle_;
    return *this;
  }

#else   // !INSIDE_BLINK
  // Disable the assignment operator; we define it above for when
  // INSIDE_BLINK is set, but we need to make sure that it is not
  // used outside there; the compiler-provided version won't handle reference
  // counting properly.
  WebPrivatePtrForGC& operator=(const WebPrivatePtrForGC& other) = delete;
#endif  // !INSIDE_BLINK

 private:
  using BlinkPtrType =
      typename WebPrivatePtrPersistentStorageType<T,
                                                  PtrDestruction,
                                                  PtrStrength>::Type;
  BlinkPtrType handle_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_PRIVATE_PTR_H_
