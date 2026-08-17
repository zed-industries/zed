// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_FUNCTIONAL_BIND_H_
#define BASE_FUNCTIONAL_BIND_H_

#include <functional>
#include <memory>
#include <type_traits>
#include <utility>

#include "base/compiler_specific.h"
#include "base/functional/bind_internal.h"  // IWYU pragma: export
#include "base/memory/raw_ptr.h"
#include "build/build_config.h"

// -----------------------------------------------------------------------------
// Usage documentation
// -----------------------------------------------------------------------------
//
// Overview:
// base::BindOnce() and base::BindRepeating() are helpers for creating
// base::OnceCallback and base::RepeatingCallback objects respectively.
//
// For a runnable object of n-arity, the base::Bind*() family allows partial
// application of the first m arguments. The remaining n - m arguments must be
// passed when invoking the callback with Run().
//
//   // The first argument is bound at callback creation; the remaining
//   // two must be passed when calling Run() on the callback object.
//   base::OnceCallback<long(int, long)> cb = base::BindOnce(
//       [](short x, int y, long z) { return x * y * z; }, 42);
//
// When binding to a method, the receiver object must also be specified at
// callback creation time. When Run() is invoked, the method will be invoked on
// the specified receiver object.
//
//   class C : public base::RefCounted<C> { void F(); };
//   auto instance = base::MakeRefCounted<C>();
//   auto cb = base::BindOnce(&C::F, instance);
//   std::move(cb).Run();  // Identical to instance->F()
//
// See //docs/callback.md for the full documentation.
//
// -----------------------------------------------------------------------------
// Implementation notes
// -----------------------------------------------------------------------------
//
// If you're reading the implementation, before proceeding further, you should
// read the top comment of base/functional/bind_internal.h for a definition of
// common terms and concepts.

namespace base {

// Bind as OnceCallback.
template <typename Functor, typename... Args>
inline auto BindOnce(Functor&& functor, Args&&... args) {
  return internal::BindHelper<OnceCallback>::Bind(
      std::forward<Functor>(functor), std::forward<Args>(args)...);
}

// Bind as RepeatingCallback.
template <typename Functor, typename... Args>
inline auto BindRepeating(Functor&& functor, Args&&... args) {
  return internal::BindHelper<RepeatingCallback>::Bind(
      std::forward<Functor>(functor), std::forward<Args>(args)...);
}

// Overloads to allow nicer compile errors when attempting to pass the address
// an overloaded function to `BindOnce()` or `BindRepeating()`. Otherwise, clang
// provides only the error message "no matching function [...] candidate
// template ignored: couldn't infer template argument 'Functor'", with no
// reference to the fact that `&` is being used on an overloaded function.
//
// These overloads to provide better error messages will never be selected
// unless template type deduction fails because of how overload resolution
// works; per [over.ics.rank/2.2]:
//
//   When comparing the basic forms of implicit conversion sequences (as defined
//   in [over.best.ics])
//   - a standard conversion sequence is a better conversion sequence than a
//     user-defined conversion sequence or an ellipsis conversion sequence, and
//   - a user-defined conversion sequence is a better conversion sequence than
//     an ellipsis conversion sequence.
//
// So these overloads will only be selected as a last resort iff template type
// deduction fails.
BindFailedCheckPreviousErrors BindOnce(...);
BindFailedCheckPreviousErrors BindRepeating(...);

// Unretained(), UnsafeDangling() and UnsafeDanglingUntriaged() allow binding a
// non-refcounted class, and to disable refcounting on arguments that are
// refcounted. The main difference is whether or not the raw pointers will be
// checked for dangling references (e.g. a pointer that points to an already
// destroyed object) when the callback is run.
//
// It is _required_ to use one of Unretained(), UnsafeDangling() or
// UnsafeDanglingUntriaged() for raw pointer receivers now. For other arguments,
// it remains optional. If not specified, default behavior is Unretained().

// Unretained() pointers will be checked for dangling pointers when the
// callback is run, *if* the callback has not been cancelled.
//
// Example of Unretained() usage:
//
//   class Foo {
//    public:
//     void func() { cout << "Foo:f" << endl; }
//   };
//
//   // In some function somewhere.
//   Foo foo;
//   OnceClosure foo_callback =
//       BindOnce(&Foo::func, Unretained(&foo));
//   std::move(foo_callback).Run();  // Prints "Foo:f".
//
// Without the Unretained() wrapper on |&foo|, the above call would fail
// to compile because Foo does not support the AddRef() and Release() methods.
//
// Unretained() does not allow dangling pointers, e.g.:
//   class MyClass {
//    public:
//     OnError(int error);
//    private:
//     scoped_refptr<base::TaskRunner> runner_;
//     std::unique_ptr<AnotherClass> obj_;
//   };
//
//   void MyClass::OnError(int error) {
//     // the pointer (which is also the receiver here) to `AnotherClass`
//     // might dangle depending on when the task is invoked.
//     runner_->PostTask(FROM_HERE, base::BindOnce(&AnotherClass::OnError,
//         base::Unretained(obj_.get()), error));
//     // one of the way to solve this issue here would be:
//     // runner_->PostTask(FROM_HERE,
//     //                   base::BindOnce(&AnotherClass::OnError,
//     //                   base::Owned(std::move(obj_)), error));
//     delete this;
//   }
//
// the above example is a BAD USAGE of Unretained(), which might result in a
// use-after-free, as `AnotherClass::OnError` might be invoked with a dangling
// pointer as receiver.
template <typename T>
inline auto Unretained(T* o) {
  return internal::UnretainedWrapper<T, unretained_traits::MayNotDangle>(o);
}

template <typename T, RawPtrTraits Traits>
inline auto Unretained(const raw_ptr<T, Traits>& o) {
  return internal::UnretainedWrapper<T, unretained_traits::MayNotDangle,
                                     Traits>(o);
}

template <typename T, RawPtrTraits Traits>
inline auto Unretained(raw_ptr<T, Traits>&& o) {
  return internal::UnretainedWrapper<T, unretained_traits::MayNotDangle,
                                     Traits>(std::move(o));
}

template <typename T, RawPtrTraits Traits>
inline auto Unretained(const raw_ref<T, Traits>& o) {
  return internal::UnretainedRefWrapper<T, unretained_traits::MayNotDangle,
                                        Traits>(o);
}

template <typename T, RawPtrTraits Traits>
inline auto Unretained(raw_ref<T, Traits>&& o) {
  return internal::UnretainedRefWrapper<T, unretained_traits::MayNotDangle,
                                        Traits>(std::move(o));
}

// Similar to `Unretained()`, but allows dangling pointers, e.g.:
//
//   class MyClass {
//     public:
//       DoSomething(HandlerClass* handler);
//     private:
//       void MyClass::DoSomethingInternal(HandlerClass::Id id,
//                                         HandlerClass* handler);
//
//       std::unordered_map<HandlerClass::Id, HandlerClass*> handlers_;
//       scoped_refptr<base::SequencedTaskRunner> runner_;
//       base::Lock lock_;
//   };
//   void MyClass::DoSomething(HandlerClass* handler) {
//      runner_->PostTask(FROM_HERE,
//          base::BindOnce(&MyClass::DoSomethingInternal,
//                         base::Unretained(this),
//                         handler->id(),
//                         base::Unretained(handler)));
//   }
//   void MyClass::DoSomethingInternal(HandlerClass::Id id,
//                                     HandlerClass* handler) {
//     base::AutoLock locker(lock_);
//     if (handlers_.find(id) == std::end(handlers_)) return;
//     // Now we can use `handler`.
//   }
//
// As `DoSomethingInternal` is run on a sequence (and we can imagine
// `handlers_` being modified on it as well), we protect the function from
// using a dangling `handler` by making sure it is still contained in the
// map.
//
// Strongly prefer `Unretained()`. This is useful in limited situations such as
// the one above.
//
// When using `UnsafeDangling()`, the receiver must be of type MayBeDangling<>.
template <typename T>
inline auto UnsafeDangling(T* o) {
  return internal::UnretainedWrapper<T, unretained_traits::MayDangle>(o);
}

template <typename T, RawPtrTraits Traits>
auto UnsafeDangling(const raw_ptr<T, Traits>& o) {
  return internal::UnretainedWrapper<T, unretained_traits::MayDangle, Traits>(
      o);
}

template <typename T, RawPtrTraits Traits>
auto UnsafeDangling(raw_ptr<T, Traits>&& o) {
  return internal::UnretainedWrapper<T, unretained_traits::MayDangle, Traits>(
      std::move(o));
}

template <typename T, RawPtrTraits Traits>
auto UnsafeDangling(const raw_ref<T, Traits>& o) {
  return internal::UnretainedRefWrapper<T, unretained_traits::MayDangle,
                                        Traits>(o);
}

template <typename T, RawPtrTraits Traits>
auto UnsafeDangling(raw_ref<T, Traits>&& o) {
  return internal::UnretainedRefWrapper<T, unretained_traits::MayDangle,
                                        Traits>(std::move(o));
}

// Like `UnsafeDangling()`, but used to annotate places that still need to be
// triaged and either migrated to `Unretained()` and safer ownership patterns
// (preferred) or `UnsafeDangling()` if the correct pattern to use is the one
// in the `UnsafeDangling()` example above for example.
//
// Unlike `UnsafeDangling()`, the receiver doesn't have to be MayBeDangling<>.
template <typename T>
inline auto UnsafeDanglingUntriaged(T* o) {
  return internal::UnretainedWrapper<T, unretained_traits::MayDangleUntriaged>(
      o);
}

template <typename T, RawPtrTraits Traits>
auto UnsafeDanglingUntriaged(const raw_ptr<T, Traits>& o) {
  return internal::UnretainedWrapper<T, unretained_traits::MayDangleUntriaged,
                                     Traits>(o);
}

template <typename T, RawPtrTraits Traits>
auto UnsafeDanglingUntriaged(raw_ptr<T, Traits>&& o) {
  return internal::UnretainedWrapper<T, unretained_traits::MayDangleUntriaged,
                                     Traits>(std::move(o));
}

template <typename T, RawPtrTraits Traits>
auto UnsafeDanglingUntriaged(const raw_ref<T, Traits>& o) {
  return internal::UnretainedRefWrapper<
      T, unretained_traits::MayDangleUntriaged, Traits>(o);
}

template <typename T, RawPtrTraits Traits>
auto UnsafeDanglingUntriaged(raw_ref<T, Traits>&& o) {
  return internal::UnretainedRefWrapper<
      T, unretained_traits::MayDangleUntriaged, Traits>(std::move(o));
}

// RetainedRef() accepts a ref counted object and retains a reference to it.
// When the callback is called, the object is passed as a raw pointer.
//
// EXAMPLE OF RetainedRef():
//
//    void foo(RefCountedBytes* bytes) {}
//
//    scoped_refptr<RefCountedBytes> bytes = ...;
//    OnceClosure callback = BindOnce(&foo, base::RetainedRef(bytes));
//    std::move(callback).Run();
//
// Without RetainedRef, the scoped_refptr would try to implicitly convert to
// a raw pointer and fail compilation:
//
//    OnceClosure callback = BindOnce(&foo, bytes); // ERROR!
template <typename T>
inline internal::RetainedRefWrapper<T> RetainedRef(T* o) {
  return internal::RetainedRefWrapper<T>(o);
}
template <typename T>
inline internal::RetainedRefWrapper<T> RetainedRef(scoped_refptr<T> o) {
  return internal::RetainedRefWrapper<T>(std::move(o));
}

// Owned() transfers ownership of an object to the callback resulting from
// bind; the object will be deleted when the callback is deleted.
//
// EXAMPLE OF Owned():
//
//   void foo(int* arg) { cout << *arg << endl }
//
//   int* pn = new int(1);
//   RepeatingClosure foo_callback = BindRepeating(&foo, Owned(pn));
//
//   foo_callback.Run();  // Prints "1"
//   foo_callback.Run();  // Prints "1"
//   *pn = 2;
//   foo_callback.Run();  // Prints "2"
//
//   foo_callback.Reset();  // |pn| is deleted.  Also will happen when
//                          // |foo_callback| goes out of scope.
//
// Without Owned(), someone would have to know to delete |pn| when the last
// reference to the callback is deleted.
template <typename T>
inline internal::OwnedWrapper<T> Owned(T* o) {
  return internal::OwnedWrapper<T>(o);
}

template <typename T, typename Deleter>
inline internal::OwnedWrapper<T, Deleter> Owned(
    std::unique_ptr<T, Deleter>&& ptr) {
  return internal::OwnedWrapper<T, Deleter>(std::move(ptr));
}

// OwnedRef() stores an object in the callback resulting from
// bind and passes a reference to the object to the bound function.
//
// EXAMPLE OF OwnedRef():
//
//   void foo(int& arg) { cout << ++arg << endl }
//
//   int counter = 0;
//   RepeatingClosure foo_callback = BindRepeating(&foo, OwnedRef(counter));
//
//   foo_callback.Run();  // Prints "1"
//   foo_callback.Run();  // Prints "2"
//   foo_callback.Run();  // Prints "3"
//
//   cout << counter;     // Prints "0", OwnedRef creates a copy of counter.
//
//  Supports OnceCallbacks as well, useful to pass placeholder arguments:
//
//   void bar(int& ignore, const std::string& s) { cout << s << endl }
//
//   OnceClosure bar_callback = BindOnce(&bar, OwnedRef(0), "Hello");
//
//   std::move(bar_callback).Run(); // Prints "Hello"
//
// Without OwnedRef() it would not be possible to pass a mutable reference to an
// object owned by the callback.
template <typename T>
internal::OwnedRefWrapper<std::decay_t<T>> OwnedRef(T&& t) {
  return internal::OwnedRefWrapper<std::decay_t<T>>(std::forward<T>(t));
}

// Passed() is for transferring movable-but-not-copyable types (eg. unique_ptr)
// through a RepeatingCallback. Logically, this signifies a destructive transfer
// of the state of the argument into the target function. Invoking
// RepeatingCallback::Run() twice on a callback that was created with a Passed()
// argument will CHECK() because the first invocation would have already
// transferred ownership to the target function.
//
// Note that Passed() is not necessary with BindOnce(), as std::move() does the
// same thing. Avoid Passed() in favor of std::move() with BindOnce().
//
// EXAMPLE OF Passed():
//
//   void TakesOwnership(std::unique_ptr<Foo> arg) { }
//   std::unique_ptr<Foo> CreateFoo() { return std::make_unique<Foo>();
//   }
//
//   auto f = std::make_unique<Foo>();
//
//   // |cb| is given ownership of Foo(). |f| is now NULL.
//   // You can use std::move(f) in place of &f, but it's more verbose.
//   RepeatingClosure cb = BindRepeating(&TakesOwnership, Passed(&f));
//
//   // Run was never called so |cb| still owns Foo() and deletes
//   // it on Reset().
//   cb.Reset();
//
//   // |cb| is given a new Foo created by CreateFoo().
//   cb = BindRepeating(&TakesOwnership, Passed(CreateFoo()));
//
//   // |arg| in TakesOwnership() is given ownership of Foo(). |cb|
//   // no longer owns Foo() and, if reset, would not delete Foo().
//   cb.Run();  // Foo() is now transferred to |arg| and deleted.
//   cb.Run();  // This CHECK()s since Foo() already been used once.
//
// We offer 2 syntaxes for calling Passed(). The first takes an rvalue and is
// best suited for use with the return value of a function or other temporary
// rvalues. The second takes a pointer to the scoper and is just syntactic sugar
// to avoid having to write Passed(std::move(scoper)).
//
// Both versions of Passed() prevent T from being an lvalue reference. The first
// via use of enable_if, and the second takes a T* which will not bind to T&.
//
// DEPRECATED - Do not use in new code. See https://crbug.com/1326449
template <typename T>
  requires(!std::is_lvalue_reference_v<T>)
inline internal::PassedWrapper<T> Passed(T&& scoper) {
  return internal::PassedWrapper<T>(std::move(scoper));
}
template <typename T>
inline internal::PassedWrapper<T> Passed(T* scoper) {
  return internal::PassedWrapper<T>(std::move(*scoper));
}

// IgnoreResult() is used to adapt a function or callback with a return type to
// one with a void return. This is most useful if you have a function with,
// say, a pesky ignorable bool return that you want to use with PostTask or
// something else that expect a callback with a void return.
//
// EXAMPLE OF IgnoreResult():
//
//   int DoSomething(int arg) { cout << arg << endl; }
//
//   // Assign to a callback with a void return type.
//   OnceCallback<void(int)> cb = BindOnce(IgnoreResult(&DoSomething));
//   std::move(cb).Run(1);  // Prints "1".
//
//   // Prints "2" on |ml|.
//   ml->PostTask(FROM_HERE, BindOnce(IgnoreResult(&DoSomething), 2);
template <typename T>
inline internal::IgnoreResultHelper<T> IgnoreResult(T data) {
  return internal::IgnoreResultHelper<T>(std::move(data));
}

}  // namespace base

#endif  // BASE_FUNCTIONAL_BIND_H_
