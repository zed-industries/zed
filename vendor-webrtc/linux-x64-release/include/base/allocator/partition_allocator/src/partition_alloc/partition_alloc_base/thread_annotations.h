// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This header file contains macro definitions for thread safety annotations
// that allow developers to document the locking policies of multi-threaded
// code. The annotations can also help program analysis tools to identify
// potential thread safety issues.
//
// Note that no analysis is done inside constructors and destructors,
// regardless of what attributes are used. See
// https://clang.llvm.org/docs/ThreadSafetyAnalysis.html#no-checking-inside-constructors-and-destructors
// for details.
//
// Note that the annotations we use are described as deprecated in the Clang
// documentation, linked below. E.g. we use PA_EXCLUSIVE_LOCKS_REQUIRED where
// the Clang docs use REQUIRES.
//
// http://clang.llvm.org/docs/ThreadSafetyAnalysis.html
//
// We use the deprecated Clang annotations to match Abseil (relevant header
// linked below) and its ecosystem of libraries. We will follow Abseil with
// respect to upgrading to more modern annotations.
//
// https://github.com/abseil/abseil-cpp/blob/master/absl/base/thread_annotations.h
//
// These annotations are implemented using compiler attributes. Using the macros
// defined here instead of raw attributes allow for portability and future
// compatibility.
//
// When referring to mutexes in the arguments of the attributes, you should
// use variable names or more complex expressions (e.g. my_object->mutex_)
// that evaluate to a concrete mutex object whenever possible. If the mutex
// you want to refer to is not in scope, you may use a member pointer
// (e.g. &MyClass::mutex_) to refer to a mutex in some (unknown) object.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_BASE_THREAD_ANNOTATIONS_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_BASE_THREAD_ANNOTATIONS_H_

#include "partition_alloc/build_config.h"
#include "partition_alloc/buildflags.h"

#if defined(__clang__)
#define PA_THREAD_ANNOTATION_ATTRIBUTE__(x) __attribute__((x))
#else
#define PA_THREAD_ANNOTATION_ATTRIBUTE__(x)  // no-op
#endif

// PA_GUARDED_BY()
//
// Documents if a shared field or global variable needs to be protected by a
// mutex. PA_GUARDED_BY() allows the user to specify a particular mutex that
// should be held when accessing the annotated variable.
//
// Example:
//
//   Mutex mu;
//   int p1 PA_GUARDED_BY(mu);
#define PA_GUARDED_BY(x) PA_THREAD_ANNOTATION_ATTRIBUTE__(guarded_by(x))

// PA_PT_GUARDED_BY()
//
// Documents if the memory location pointed to by a pointer should be guarded
// by a mutex when dereferencing the pointer.
//
// Example:
//   Mutex mu;
//   int *p1 PA_PT_GUARDED_BY(mu);
//
// Note that a pointer variable to a shared memory location could itself be a
// shared variable.
//
// Example:
//
//     // `q`, guarded by `mu1`, points to a shared memory location that is
//     // guarded by `mu2`:
//     int *q PA_GUARDED_BY(mu1) PA_PT_GUARDED_BY(mu2);
#define PA_PT_GUARDED_BY(x) PA_THREAD_ANNOTATION_ATTRIBUTE__(pt_guarded_by(x))

// PA_ACQUIRED_AFTER() / PA_ACQUIRED_BEFORE()
//
// Documents the acquisition order between locks that can be held
// simultaneously by a thread. For any two locks that need to be annotated
// to establish an acquisition order, only one of them needs the annotation.
// (i.e. You don't have to annotate both locks with both PA_ACQUIRED_AFTER
// and PA_ACQUIRED_BEFORE.)
//
// Example:
//
//   Mutex m1;
//   Mutex m2 PA_ACQUIRED_AFTER(m1);
#define PA_ACQUIRED_AFTER(...) \
  PA_THREAD_ANNOTATION_ATTRIBUTE__(acquired_after(__VA_ARGS__))

#define PA_ACQUIRED_BEFORE(...) \
  PA_THREAD_ANNOTATION_ATTRIBUTE__(acquired_before(__VA_ARGS__))

// PA_EXCLUSIVE_LOCKS_REQUIRED() / PA_SHARED_LOCKS_REQUIRED()
//
// Documents a function that expects a mutex to be held prior to entry.
// The mutex is expected to be held both on entry to, and exit from, the
// function.
//
// Example:
//
//   Mutex mu1, mu2;
//   int a PA_GUARDED_BY(mu1);
//   int b PA_GUARDED_BY(mu2);
//
//   void foo() PA_EXCLUSIVE_LOCKS_REQUIRED(mu1, mu2) { ... };
#define PA_EXCLUSIVE_LOCKS_REQUIRED(...) \
  PA_THREAD_ANNOTATION_ATTRIBUTE__(exclusive_locks_required(__VA_ARGS__))

#define PA_SHARED_LOCKS_REQUIRED(...) \
  PA_THREAD_ANNOTATION_ATTRIBUTE__(shared_locks_required(__VA_ARGS__))

// PA_LOCKS_EXCLUDED()
//
// Documents the locks acquired in the body of the function. These locks
// cannot be held when calling this function (as Abseil's `Mutex` locks are
// non-reentrant).
#define PA_LOCKS_EXCLUDED(...) \
  PA_THREAD_ANNOTATION_ATTRIBUTE__(locks_excluded(__VA_ARGS__))

// PA_LOCK_RETURNED()
//
// Documents a function that returns a mutex without acquiring it.  For example,
// a public getter method that returns a pointer to a private mutex should
// be annotated with PA_LOCK_RETURNED.
#define PA_LOCK_RETURNED(x) PA_THREAD_ANNOTATION_ATTRIBUTE__(lock_returned(x))

// PA_LOCKABLE
//
// Documents if a class/type is a lockable type (such as the `Mutex` class).
#define PA_LOCKABLE PA_THREAD_ANNOTATION_ATTRIBUTE__(lockable)

// PA_SCOPED_LOCKABLE
//
// Documents if a class does RAII locking (such as the `MutexLock` class).
// The constructor should use `PA_*_LOCK_FUNCTION()` to specify the mutex that
// is acquired, and the destructor should use `PA_UNLOCK_FUNCTION()` with no
// arguments; the analysis will assume that the destructor unlocks whatever the
// constructor locked.
#define PA_SCOPED_LOCKABLE PA_THREAD_ANNOTATION_ATTRIBUTE__(scoped_lockable)

// PA_EXCLUSIVE_LOCK_FUNCTION()
//
// Documents functions that acquire a lock in the body of a function, and do
// not release it.
#define PA_EXCLUSIVE_LOCK_FUNCTION(...) \
  PA_THREAD_ANNOTATION_ATTRIBUTE__(exclusive_lock_function(__VA_ARGS__))

// PA_SHARED_LOCK_FUNCTION()
//
// Documents functions that acquire a shared (reader) lock in the body of a
// function, and do not release it.
#define PA_SHARED_LOCK_FUNCTION(...) \
  PA_THREAD_ANNOTATION_ATTRIBUTE__(shared_lock_function(__VA_ARGS__))

// PA_UNLOCK_FUNCTION()
//
// Documents functions that expect a lock to be held on entry to the function,
// and release it in the body of the function.
#define PA_UNLOCK_FUNCTION(...) \
  PA_THREAD_ANNOTATION_ATTRIBUTE__(unlock_function(__VA_ARGS__))

// PA_EXCLUSIVE_TRYLOCK_FUNCTION() / PA_SHARED_TRYLOCK_FUNCTION()
//
// Documents functions that try to acquire a lock, and return success or failure
// (or a non-boolean value that can be interpreted as a boolean).
// The first argument should be `true` for functions that return `true` on
// success, or `false` for functions that return `false` on success. The second
// argument specifies the mutex that is locked on success. If unspecified, this
// mutex is assumed to be `this`.
#define PA_EXCLUSIVE_TRYLOCK_FUNCTION(...) \
  PA_THREAD_ANNOTATION_ATTRIBUTE__(exclusive_trylock_function(__VA_ARGS__))

#define PA_SHARED_TRYLOCK_FUNCTION(...) \
  PA_THREAD_ANNOTATION_ATTRIBUTE__(shared_trylock_function(__VA_ARGS__))

// PA_ASSERT_EXCLUSIVE_LOCK() / PA_ASSERT_SHARED_LOCK()
//
// Documents functions that dynamically check to see if a lock is held, and fail
// if it is not held.
#define PA_ASSERT_EXCLUSIVE_LOCK(...) \
  PA_THREAD_ANNOTATION_ATTRIBUTE__(assert_exclusive_lock(__VA_ARGS__))

#define PA_ASSERT_SHARED_LOCK(...) \
  PA_THREAD_ANNOTATION_ATTRIBUTE__(assert_shared_lock(__VA_ARGS__))

// PA_NO_THREAD_SAFETY_ANALYSIS
//
// Turns off thread safety checking within the body of a particular function.
// This annotation is used to mark functions that are known to be correct, but
// the locking behavior is more complicated than the analyzer can handle.
#define PA_NO_THREAD_SAFETY_ANALYSIS \
  PA_THREAD_ANNOTATION_ATTRIBUTE__(no_thread_safety_analysis)

//------------------------------------------------------------------------------
// Tool-Supplied Annotations
//------------------------------------------------------------------------------

// PA_TS_UNCHECKED should be placed around lock expressions that are not valid
// C++ syntax, but which are present for documentation purposes.  These
// annotations will be ignored by the analysis.
#define PA_TS_UNCHECKED(x) ""

// PA_TS_FIXME is used to mark lock expressions that are not valid C++ syntax.
// It is used by automated tools to mark and disable invalid expressions.
// The annotation should either be fixed, or changed to PA_TS_UNCHECKED.
#define PA_TS_FIXME(x) ""

// Like PA_NO_THREAD_SAFETY_ANALYSIS, this turns off checking within the body of
// a particular function.  However, this attribute is used to mark functions
// that are incorrect and need to be fixed.  It is used by automated tools to
// avoid breaking the build when the analysis is updated.
// Code owners are expected to eventually fix the routine.
#define PA_NO_THREAD_SAFETY_ANALYSIS_FIXME PA_NO_THREAD_SAFETY_ANALYSIS

// Similar to PA_NO_THREAD_SAFETY_ANALYSIS_FIXME, this macro marks a
// PA_GUARDED_BY annotation that needs to be fixed, because it is producing
// thread safety warning.  It disables the PA_GUARDED_BY.
#define PA_GUARDED_BY_FIXME(x)

// Disables warnings for a single read operation.  This can be used to avoid
// warnings when it is known that the read is not actually involved in a race,
// but the compiler cannot confirm that.
#define PA_TS_UNCHECKED_READ(x) \
  partition_alloc::internal::thread_safety_analysis::ts_unchecked_read(x)

namespace partition_alloc::internal::thread_safety_analysis {

// Takes a reference to a guarded data member, and returns an unguarded
// reference.
template <typename T>
inline const T& ts_unchecked_read(const T& v) PA_NO_THREAD_SAFETY_ANALYSIS {
  return v;
}

template <typename T>
inline T& ts_unchecked_read(T& v) PA_NO_THREAD_SAFETY_ANALYSIS {
  return v;
}

}  // namespace partition_alloc::internal::thread_safety_analysis

// The above is imported as-is from abseil-cpp. The following Chromium-specific
// synonyms are added for Chromium concepts (SequenceChecker/ThreadChecker).
#if PA_BUILDFLAG(DCHECKS_ARE_ON)

// Equivalent to PA_GUARDED_BY for SequenceChecker/ThreadChecker. Currently,
#define PA_GUARDED_BY_CONTEXT(name) PA_GUARDED_BY(name)

// Equivalent to PA_EXCLUSIVE_LOCKS_REQUIRED for SequenceChecker/ThreadChecker.
#define PA_VALID_CONTEXT_REQUIRED(name) PA_EXCLUSIVE_LOCKS_REQUIRED(name)

#else  // PA_BUILDFLAG(DCHECKS_ARE_ON)

#define PA_GUARDED_BY_CONTEXT(name)
#define PA_VALID_CONTEXT_REQUIRED(name)

#endif  // PA_BUILDFLAG(DCHECKS_ARE_ON)

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_BASE_THREAD_ANNOTATIONS_H_
