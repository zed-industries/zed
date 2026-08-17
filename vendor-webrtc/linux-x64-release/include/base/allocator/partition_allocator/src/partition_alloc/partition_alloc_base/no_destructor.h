// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_BASE_NO_DESTRUCTOR_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_BASE_NO_DESTRUCTOR_H_

#include <new>
#include <type_traits>
#include <utility>

namespace partition_alloc::internal::base {

// Helper type to create a function-local static variable of type `T` when `T`
// has a non-trivial destructor. Storing a `T` in a `base::NoDestructor<T>` will
// prevent `~T()` from running, even when the variable goes out of scope.
//
// Useful when a variable has static storage duration but its type has a
// non-trivial destructor. Chromium bans global constructors and destructors:
// using a function-local static variable prevents the former, while using
// `base::NoDestructor<T>` prevents the latter.
//
// ## Caveats
//
// - Must only be used as a function-local static variable. Declaring a global
//   variable of type `base::NoDestructor<T>` will still generate a global
//   constructor; declaring a local or member variable will lead to memory leaks
//   or other surprising and undesirable behaviour.
//
// - If the data is rarely used, consider creating it on demand rather than
//   caching it for the lifetime of the program. Though `base::NoDestructor<T>`
//   does not heap allocate, the compiler still reserves space in bss for
//   storing `T`, which costs memory at runtime.
//
// - If `T` is trivially destructible, do not use `base::NoDestructor<T>`:
//
//     const uint64_t GetUnstableSessionSeed() {
//       // No need to use `base::NoDestructor<T>` as `uint64_t` is trivially
//       // destructible and does not require a global destructor.
//       static const uint64_t kSessionSeed = base::RandUint64();
//       return kSessionSeed;
//     }
//
// ## Example Usage
//
// const std::string& GetDefaultText() {
//   // Required since `static const std::string` requires a global destructor.
//   static const base::NoDestructor<std::string> s("Hello world!");
//   return *s;
// }
//
// More complex initialization using a lambda:
//
// const std::string& GetRandomNonce() {
//   // `nonce` is initialized with random data the first time this function is
//   // called, but its value is fixed thereafter.
//   static const base::NoDestructor<std::string> nonce([] {
//     std::string s(16);
//     crypto::RandString(s.data(), s.size());
//     return s;
//   }());
//   return *nonce;
// }
//
// ## Thread safety
//
// Initialisation of function-local static variables is thread-safe since C++11.
// The standard guarantees that:
//
// - function-local static variables will be initialised the first time
//   execution passes through the declaration.
//
// - if another thread's execution concurrently passes through the declaration
//   in the middle of initialisation, that thread will wait for the in-progress
//   initialisation to complete.
template <typename T>
class NoDestructor {
 public:
  template <typename... Args>
  constexpr explicit NoDestructor(Args&&... args)
      : storage_(std::forward<Args>(args)...) {}

  // Allows copy and move construction of the contained type, to allow
  // construction from an initializer list, e.g. for std::vector.
  explicit NoDestructor(const T& x) : storage_(x) {}
  explicit NoDestructor(T&& x) : storage_(std::move(x)) {}

  NoDestructor(const NoDestructor&) = delete;
  NoDestructor& operator=(const NoDestructor&) = delete;

  ~NoDestructor() = default;

  const T& operator*() const { return *storage_.get()(); }
  T& operator*() { return *storage_.get(); }

  const T* operator->() const { return storage_.get()(); }
  T* operator->() { return storage_.get(); }

  const T* get() const { return storage_.get(); }
  T* get() { return storage_.get(); }

 private:
  // Do not friend this. This is an implementation detail.
  class DirectStorage {
   public:
    template <typename... Args>
    constexpr explicit DirectStorage(Args&&... args)
        : storage_(std::forward<Args>(args)...) {}

    const T* get() const { return &storage_; }
    T* get() { return &storage_; }

   private:
    T storage_;
  };

  // Do not friend this. This is an implementation detail.
  class PlacementStorage {
   public:
    template <typename... Args>
    explicit PlacementStorage(Args&&... args) {
      new (storage_) T(std::forward<Args>(args)...);
    }

    const T* get() const { return const_cast<PlacementStorage*>(this)->get(); }
    T* get() { return reinterpret_cast<T*>(storage_); }

   private:
    alignas(T) char storage_[sizeof(T)];
  };

  // C++20 provides a constexpr `std::construct_at`, so in theory, both branches
  // could use PlacementStorage. There are some advantages to providing
  // `DirectStorage` though; it can avoid the need to have runtime once-init
  // tracking at all.
  std::conditional_t<std::is_trivially_destructible_v<T>,
                     DirectStorage,
                     PlacementStorage>
      storage_;
};

}  // namespace partition_alloc::internal::base

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_BASE_NO_DESTRUCTOR_H_
