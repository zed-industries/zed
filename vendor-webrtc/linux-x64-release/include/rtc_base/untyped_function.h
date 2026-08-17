/*
 *  Copyright 2020 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_UNTYPED_FUNCTION_H_
#define RTC_BASE_UNTYPED_FUNCTION_H_

#include <cstddef>
#include <cstring>
#include <memory>
#include <type_traits>
#include <utility>

#include "rtc_base/system/assume.h"

namespace webrtc {
namespace webrtc_function_impl {

using FunVoid = void();

// Inline storage size is this many machine words.
enum : size_t { kInlineStorageWords = 4 };

union VoidUnion {
  void* void_ptr;
  FunVoid* fun_ptr;
  // std::max_align_t satisfies alignment requirements for every type.
  alignas(std::max_align_t) char inline_storage[kInlineStorageWords *
                                                sizeof(uintptr_t)];
};

// Returns the number of elements of the `inline_storage` array required to
// store an object of type T.
template <typename T>
constexpr size_t InlineStorageSize() {
  // sizeof(T) / sizeof(uintptr_t), but rounded up.
  return (sizeof(T) + sizeof(uintptr_t) - 1) / sizeof(uintptr_t);
}

template <typename T>
struct CallHelpers;
template <typename RetT, typename... ArgT>
struct CallHelpers<RetT(ArgT...)> {
  // Return type of the three helpers below.
  using return_type = RetT;
  // Complete function type of the three helpers below.
  using function_type = RetT(VoidUnion*, ArgT...);
  // Helper for calling the `void_ptr` case of VoidUnion.
  template <typename F>
  static RetT CallVoidPtr(VoidUnion* vu, ArgT... args) {
    return (*static_cast<F*>(vu->void_ptr))(std::forward<ArgT>(args)...);
  }
  // Helper for calling the `fun_ptr` case of VoidUnion.
  static RetT CallFunPtr(VoidUnion* vu, ArgT... args) {
    return (reinterpret_cast<RetT (*)(ArgT...)>(vu->fun_ptr))(
        std::forward<ArgT>(args)...);
  }
  // Helper for calling the `inline_storage` case of VoidUnion.
  template <typename F>
  static RetT CallInlineStorage(VoidUnion* vu, ArgT... args) {
    return (*reinterpret_cast<F*>(&vu->inline_storage))(
        std::forward<ArgT>(args)...);
  }
};

}  // namespace webrtc_function_impl

// A class that holds (and owns) any callable. The same function call signature
// must be provided when constructing and calling the object.
//
// The point of not having the call signature as a class template parameter is
// to have one single concrete type for all signatures; this reduces binary
// size.
class UntypedFunction final {
 public:
  // Callables of at most this size can be stored inline, if they are trivial.
  // (Useful in tests and benchmarks; avoid using this in production code.)
  enum : size_t {
    kInlineStorageSize = sizeof(webrtc_function_impl::VoidUnion::inline_storage)
  };
  static_assert(kInlineStorageSize ==
                    webrtc_function_impl::kInlineStorageWords *
                        sizeof(uintptr_t),
                "");

  // The *UntypedFunctionArgs structs are used to transfer arguments from
  // PrepareArgs() to Create(). They are trivial, but may own heap allocations,
  // so make sure to pass them to Create() exactly once!
  //
  // The point of doing Create(PrepareArgs(foo)) instead of just Create(foo) is
  // to separate the code that has to be inlined (PrepareArgs) from the code
  // that can be noninlined (Create); the *UntypedFunctionArgs types are
  // designed to efficiently carry the required information from one to the
  // other.
  template <size_t N>
  struct TrivialUntypedFunctionArgs {
    static_assert(N >= 1, "");
    static_assert(N <= webrtc_function_impl::kInlineStorageWords, "");
    // We use an uintptr_t array here instead of std::aligned_storage, because
    // the former can be efficiently passed in registers when using
    // TrivialUntypedFunctionArgs as a function argument. (We can't do the same
    // in VoidUnion, because std::aligned_storage but not uintptr_t can be
    // legally reinterpret_casted to arbitrary types.
    // TrivialUntypedFunctionArgs, on the other hand, only needs to handle
    // placement new and memcpy.)
    alignas(std::max_align_t) uintptr_t inline_storage[N];
    webrtc_function_impl::FunVoid* call;
  };
  struct NontrivialUntypedFunctionArgs {
    void* void_ptr;
    webrtc_function_impl::FunVoid* call;
    void (*del)(webrtc_function_impl::VoidUnion*);
  };
  struct FunctionPointerUntypedFunctionArgs {
    webrtc_function_impl::FunVoid* fun_ptr;
    webrtc_function_impl::FunVoid* call;
  };

  // Create function for lambdas and other callables that are trivial and small;
  // it accepts every type of argument except those noted in its enable_if call.
  template <
      typename Signature,
      typename F,
      typename F_deref = typename std::remove_reference<F>::type,
      typename std::enable_if<
          // Not for function pointers; we have another overload for that below.
          !std::is_function<
              typename std::remove_pointer<F_deref>::type>::value &&

          // Not for nullptr; we have a constructor for that below.
          !std::is_same<std::nullptr_t,
                        typename std::remove_cv<F>::type>::value &&

          // Not for UntypedFunction objects; use move construction or
          // assignment.
          !std::is_same<UntypedFunction,
                        typename std::remove_cv<F_deref>::type>::value &&

          // Only for trivial callables that will fit in inline storage.
          std::is_trivially_move_constructible<F_deref>::value &&
          std::is_trivially_destructible<F_deref>::value &&
          sizeof(F_deref) <= kInlineStorageSize>::type* = nullptr,
      size_t InlineSize = webrtc_function_impl::InlineStorageSize<F_deref>()>
  static TrivialUntypedFunctionArgs<InlineSize> PrepareArgs(F&& f) {
    // The callable is trivial and small enough, so we just store its bytes
    // in the inline storage.
    TrivialUntypedFunctionArgs<InlineSize> args;
    new (&args.inline_storage) F_deref(std::forward<F>(f));
    args.call = reinterpret_cast<webrtc_function_impl::FunVoid*>(
        webrtc_function_impl::CallHelpers<
            Signature>::template CallInlineStorage<F_deref>);
    return args;
  }
  template <size_t InlineSize>
  static UntypedFunction Create(TrivialUntypedFunctionArgs<InlineSize> args) {
    webrtc_function_impl::VoidUnion vu;
    std::memcpy(&vu.inline_storage, args.inline_storage,
                sizeof(args.inline_storage));
    return UntypedFunction(vu, args.call, nullptr);
  }

  // Create function for lambdas and other callables that are nontrivial or
  // large; it accepts every type of argument except those noted in its
  // enable_if call.
  template <typename Signature,
            typename F,
            typename F_deref = typename std::remove_reference<F>::type,
            typename std::enable_if<
                // Not for function pointers; we have another overload for that
                // below.
                !std::is_function<
                    typename std::remove_pointer<F_deref>::type>::value &&

                // Not for nullptr; we have a constructor for that below.
                !std::is_same<std::nullptr_t,
                              typename std::remove_cv<F>::type>::value &&

                // Not for UntypedFunction objects; use move construction or
                // assignment.
                !std::is_same<UntypedFunction,
                              typename std::remove_cv<F_deref>::type>::value &&

                // Only for nontrivial callables, or callables that won't fit in
                // inline storage.
                !(std::is_trivially_move_constructible<F_deref>::value &&
                  std::is_trivially_destructible<F_deref>::value &&
                  sizeof(F_deref) <= kInlineStorageSize)>::type* = nullptr>
  static NontrivialUntypedFunctionArgs PrepareArgs(F&& f) {
    // The callable is either nontrivial or too large, so we can't keep it
    // in the inline storage; use the heap instead.
    NontrivialUntypedFunctionArgs args;
    args.void_ptr = new F_deref(std::forward<F>(f));
    args.call = reinterpret_cast<webrtc_function_impl::FunVoid*>(
        webrtc_function_impl::CallHelpers<Signature>::template CallVoidPtr<
            F_deref>);
    args.del = static_cast<void (*)(webrtc_function_impl::VoidUnion*)>(
        [](webrtc_function_impl::VoidUnion* vu) {
          // Assuming that this pointer isn't null allows the
          // compiler to eliminate a null check in the (inlined)
          // delete operation.
          RTC_ASSUME(vu->void_ptr != nullptr);
          delete reinterpret_cast<F_deref*>(vu->void_ptr);
        });
    return args;
  }
  static UntypedFunction Create(NontrivialUntypedFunctionArgs args) {
    webrtc_function_impl::VoidUnion vu;
    vu.void_ptr = args.void_ptr;
    return UntypedFunction(vu, args.call, args.del);
  }

  // Create function that accepts function pointers. If the argument is null,
  // the result is an empty UntypedFunction.
  template <typename Signature>
  static FunctionPointerUntypedFunctionArgs PrepareArgs(Signature* f) {
    FunctionPointerUntypedFunctionArgs args;
    args.fun_ptr = reinterpret_cast<webrtc_function_impl::FunVoid*>(f);
    args.call = reinterpret_cast<webrtc_function_impl::FunVoid*>(
        webrtc_function_impl::CallHelpers<Signature>::CallFunPtr);
    return args;
  }
  static UntypedFunction Create(FunctionPointerUntypedFunctionArgs args) {
    webrtc_function_impl::VoidUnion vu;
    vu.fun_ptr = args.fun_ptr;
    return UntypedFunction(vu, args.fun_ptr == nullptr ? nullptr : args.call,
                           nullptr);
  }

  // Prepares arguments and creates an UntypedFunction in one go.
  template <typename Signature, typename F>
  static UntypedFunction Create(F&& f) {
    return Create(PrepareArgs<Signature>(std::forward<F>(f)));
  }

  // Default constructor. Creates an empty UntypedFunction.
  UntypedFunction() : call_(nullptr), delete_(nullptr) {}

  // Nullptr constructor and assignment. Creates an empty UntypedFunction.
  UntypedFunction(std::nullptr_t)  // NOLINT(runtime/explicit)
      : call_(nullptr), delete_(nullptr) {}
  UntypedFunction& operator=(std::nullptr_t) {
    call_ = nullptr;
    if (delete_) {
      delete_(&f_);
      delete_ = nullptr;
    }
    return *this;
  }

  // Not copyable.
  UntypedFunction(const UntypedFunction&) = delete;
  UntypedFunction& operator=(const UntypedFunction&) = delete;

  // Move construction and assignment.
  UntypedFunction(UntypedFunction&& other)
      : f_(other.f_), call_(other.call_), delete_(other.delete_) {
    other.delete_ = nullptr;
  }
  UntypedFunction& operator=(UntypedFunction&& other) {
    if (delete_) {
      delete_(&f_);
    }
    f_ = other.f_;
    call_ = other.call_;
    delete_ = other.delete_;
    other.delete_ = nullptr;
    return *this;
  }

  ~UntypedFunction() {
    if (delete_) {
      delete_(&f_);
    }
  }

  friend void swap(UntypedFunction& a, UntypedFunction& b) {
    using std::swap;
    swap(a.f_, b.f_);
    swap(a.call_, b.call_);
    swap(a.delete_, b.delete_);
  }

  // Returns true if we have a function, false if we don't (i.e., we're null).
  explicit operator bool() const { return call_ != nullptr; }

  template <typename Signature, typename... ArgT>
  typename webrtc_function_impl::CallHelpers<Signature>::return_type Call(
      ArgT&&... args) {
    return reinterpret_cast<
        typename webrtc_function_impl::CallHelpers<Signature>::function_type*>(
        call_)(&f_, std::forward<ArgT>(args)...);
  }

  // Returns true iff we don't need to call a destructor. This is guaranteed
  // to hold for a moved-from object.
  bool IsTriviallyDestructible() { return delete_ == nullptr; }

 private:
  UntypedFunction(webrtc_function_impl::VoidUnion f,
                  webrtc_function_impl::FunVoid* call,
                  void (*del)(webrtc_function_impl::VoidUnion*))
      : f_(f), call_(call), delete_(del) {}

  // The callable thing, or a pointer to it.
  webrtc_function_impl::VoidUnion f_;

  // Pointer to a dispatch function that knows the type of the callable thing
  // that's stored in f_, and how to call it. An UntypedFunction object is empty
  // (null) iff call_ is null.
  webrtc_function_impl::FunVoid* call_;

  // Pointer to a function that knows how to delete the callable thing that's
  // stored in f_. Null if `f_` is trivially deletable.
  void (*delete_)(webrtc_function_impl::VoidUnion*);
};

}  // namespace webrtc

#endif  // RTC_BASE_UNTYPED_FUNCTION_H_
