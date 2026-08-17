/*
 * Copyright (C) 2024 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef INCLUDE_PERFETTO_BASE_THREAD_ANNOTATIONS_H_
#define INCLUDE_PERFETTO_BASE_THREAD_ANNOTATIONS_H_

// This header file contains macro definitions for thread safety annotations
// that allow developers to document the locking policies of multi-threaded
// code. The annotations can also help program analysis tools to identify
// potential thread safety issues.
//
// These macro definitions are copied from the Chromium code base:
// https://source.chromium.org/chromium/chromium/src/+/main:base/thread_annotations.h;drc=10d865767e72f494da1e4e868eb6ae9befe87422
// with the 'PERFETTO_' prefix added.
//
// Note that no analysis is done inside constructors and destructors,
// regardless of what attributes are used. See
// https://clang.llvm.org/docs/ThreadSafetyAnalysis.html#no-checking-inside-constructors-and-destructors
// for details.
//
// Note that the annotations we use are described as deprecated in the Clang
// documentation, linked below. E.g. we use PERFETTO_EXCLUSIVE_LOCKS_REQUIRED
// where the Clang docs use REQUIRES.
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

#include "perfetto/base/build_config.h"

#if defined(__clang__) && PERFETTO_BUILDFLAG(PERFETTO_THREAD_SAFETY_ANNOTATIONS)
#define PERFETTO_THREAD_ANNOTATION_ATTRIBUTE__(x) __attribute__((x))
#else
#define PERFETTO_THREAD_ANNOTATION_ATTRIBUTE__(x)  // no-op
#endif

// PERFETTO_GUARDED_BY()
//
// Documents if a shared field or global variable needs to be protected by a
// mutex. PERFETTO_GUARDED_BY() allows the user to specify a particular mutex
// that should be held when accessing the annotated variable.
//
// Example:
//
//   Mutex mu;
//   int p1 PERFETTO_GUARDED_BY(mu);
#define PERFETTO_GUARDED_BY(x) \
  PERFETTO_THREAD_ANNOTATION_ATTRIBUTE__(guarded_by(x))

// PERFETTO_PT_GUARDED_BY()
//
// Documents if the memory location pointed to by a pointer should be guarded
// by a mutex when dereferencing the pointer.
//
// Example:
//   Mutex mu;
//   int *p1 PERFETTO_PT_GUARDED_BY(mu);
//
// Note that a pointer variable to a shared memory location could itself be a
// shared variable.
//
// Example:
//
//     // `q`, guarded by `mu1`, points to a shared memory location that is
//     // guarded by `mu2`:
//     int *q PERFETTO_GUARDED_BY(mu1) PERFETTO_PT_GUARDED_BY(mu2);
#define PERFETTO_PT_GUARDED_BY(x) \
  PERFETTO_THREAD_ANNOTATION_ATTRIBUTE__(pt_guarded_by(x))

// PERFETTO_ACQUIRED_AFTER() / PERFETTO_ACQUIRED_BEFORE()
//
// Documents the acquisition order between locks that can be held
// simultaneously by a thread. For any two locks that need to be annotated
// to establish an acquisition order, only one of them needs the annotation.
// (i.e. You don't have to annotate both locks with both PERFETTO_ACQUIRED_AFTER
// and PERFETTO_ACQUIRED_BEFORE.)
//
// Example:
//
//   Mutex m1;
//   Mutex m2 PERFETTO_ACQUIRED_AFTER(m1);
#define PERFETTO_ACQUIRED_AFTER(...) \
  PERFETTO_THREAD_ANNOTATION_ATTRIBUTE__(acquired_after(__VA_ARGS__))

#define PERFETTO_ACQUIRED_BEFORE(...) \
  PERFETTO_THREAD_ANNOTATION_ATTRIBUTE__(acquired_before(__VA_ARGS__))

// PERFETTO_EXCLUSIVE_LOCKS_REQUIRED() / PERFETTO_SHARED_LOCKS_REQUIRED()
//
// Documents a function that expects a mutex to be held prior to entry.
// The mutex is expected to be held both on entry to, and exit from, the
// function.
//
// Example:
//
//   Mutex mu1, mu2;
//   int a PERFETTO_GUARDED_BY(mu1);
//   int b PERFETTO_GUARDED_BY(mu2);
//
//   void foo() PERFETTO_EXCLUSIVE_LOCKS_REQUIRED(mu1, mu2) { ... };
#define PERFETTO_EXCLUSIVE_LOCKS_REQUIRED(...) \
  PERFETTO_THREAD_ANNOTATION_ATTRIBUTE__(exclusive_locks_required(__VA_ARGS__))

#define PERFETTO_SHARED_LOCKS_REQUIRED(...) \
  PERFETTO_THREAD_ANNOTATION_ATTRIBUTE__(shared_locks_required(__VA_ARGS__))

// PERFETTO_LOCKS_EXCLUDED()
//
// Documents the locks acquired in the body of the function. These locks
// cannot be held when calling this function (as Abseil's `Mutex` locks are
// non-reentrant).
#define PERFETTO_LOCKS_EXCLUDED(...) \
  PERFETTO_THREAD_ANNOTATION_ATTRIBUTE__(locks_excluded(__VA_ARGS__))

// PERFETTO_LOCK_RETURNED()
//
// Documents a function that returns a mutex without acquiring it.  For example,
// a public getter method that returns a pointer to a private mutex should
// be annotated with PERFETTO_LOCK_RETURNED.
#define PERFETTO_LOCK_RETURNED(x) \
  PERFETTO_THREAD_ANNOTATION_ATTRIBUTE__(lock_returned(x))

// PERFETTO_LOCKABLE
//
// Documents if a class/type is a lockable type (such as the `Mutex` class).
#define PERFETTO_LOCKABLE PERFETTO_THREAD_ANNOTATION_ATTRIBUTE__(lockable)

// PERFETTO_SCOPED_LOCKABLE
//
// Documents if a class does RAII locking (such as the `MutexLock` class).
// The constructor should use `LOCK_FUNCTION()` to specify the mutex that is
// acquired, and the destructor should use `PERFETTO_UNLOCK_FUNCTION()` with no
// arguments; the analysis will assume that the destructor unlocks whatever the
// constructor locked.
#define PERFETTO_SCOPED_LOCKABLE \
  PERFETTO_THREAD_ANNOTATION_ATTRIBUTE__(scoped_lockable)

// PERFETTO_EXCLUSIVE_LOCK_FUNCTION()
//
// Documents functions that acquire a lock in the body of a function, and do
// not release it.
#define PERFETTO_EXCLUSIVE_LOCK_FUNCTION(...) \
  PERFETTO_THREAD_ANNOTATION_ATTRIBUTE__(exclusive_lock_function(__VA_ARGS__))

// PERFETTO_SHARED_LOCK_FUNCTION()
//
// Documents functions that acquire a shared (reader) lock in the body of a
// function, and do not release it.
#define PERFETTO_SHARED_LOCK_FUNCTION(...) \
  PERFETTO_THREAD_ANNOTATION_ATTRIBUTE__(shared_lock_function(__VA_ARGS__))

// PERFETTO_UNLOCK_FUNCTION()
//
// Documents functions that expect a lock to be held on entry to the function,
// and release it in the body of the function.
#define PERFETTO_UNLOCK_FUNCTION(...) \
  PERFETTO_THREAD_ANNOTATION_ATTRIBUTE__(unlock_function(__VA_ARGS__))

// PERFETTO_EXCLUSIVE_TRYLOCK_FUNCTION() / PERFETTO_SHARED_TRYLOCK_FUNCTION()
//
// Documents functions that try to acquire a lock, and return success or failure
// (or a non-boolean value that can be interpreted as a boolean).
// The first argument should be `true` for functions that return `true` on
// success, or `false` for functions that return `false` on success. The second
// argument specifies the mutex that is locked on success. If unspecified, this
// mutex is assumed to be `this`.
#define PERFETTO_EXCLUSIVE_TRYLOCK_FUNCTION(...) \
  PERFETTO_THREAD_ANNOTATION_ATTRIBUTE__(        \
      exclusive_trylock_function(__VA_ARGS__))

#define PERFETTO_SHARED_TRYLOCK_FUNCTION(...) \
  PERFETTO_THREAD_ANNOTATION_ATTRIBUTE__(shared_trylock_function(__VA_ARGS__))

// PERFETTO_ASSERT_EXCLUSIVE_LOCK() / PERFETTO_ASSERT_SHARED_LOCK()
//
// Documents functions that dynamically check to see if a lock is held, and fail
// if it is not held.
#define PERFETTO_ASSERT_EXCLUSIVE_LOCK(...) \
  PERFETTO_THREAD_ANNOTATION_ATTRIBUTE__(assert_exclusive_lock(__VA_ARGS__))

#define PERFETTO_ASSERT_SHARED_LOCK(...) \
  PERFETTO_THREAD_ANNOTATION_ATTRIBUTE__(assert_shared_lock(__VA_ARGS__))

// PERFETTO_NO_THREAD_SAFETY_ANALYSIS is special and differs from other
// macros defined in this file, it was defined in `compiler.h` and used before
// we introduce Thread Safety Analysis. Therefore, we define it here even if
// 'PERFETTO_ENABLE_THREAD_SAFETY_ANNOTATIONS' macro is not defined.

#if defined(__clang__)
// PERFETTO_NO_THREAD_SAFETY_ANALYSIS
//
// Turns off thread safety checking within the body of a particular function.
// This annotation is used to mark functions that are known to be correct, but
// the locking behavior is more complicated than the analyzer can handle.
#define PERFETTO_NO_THREAD_SAFETY_ANALYSIS \
  __attribute__((no_thread_safety_analysis))
#else
#define PERFETTO_NO_THREAD_SAFETY_ANALYSIS
#endif

//------------------------------------------------------------------------------
// Tool-Supplied Annotations
//------------------------------------------------------------------------------

// PERFETTO_TS_UNCHECKED should be placed around lock expressions that are not
// valid C++ syntax, but which are present for documentation purposes.  These
// annotations will be ignored by the analysis.
#define PERFETTO_TS_UNCHECKED(x) ""

// TS_FIXME is used to mark lock expressions that are not valid C++ syntax.
// It is used by automated tools to mark and disable invalid expressions.
// The annotation should either be fixed, or changed to PERFETTO_TS_UNCHECKED.
#define PERFETTO_TS_FIXME(x) ""

// Like NO_THREAD_SAFETY_ANALYSIS, this turns off checking within the body of
// a particular function.  However, this attribute is used to mark functions
// that are incorrect and need to be fixed.  It is used by automated tools to
// avoid breaking the build when the analysis is updated.
// Code owners are expected to eventually fix the routine.
#define PERFETTO_NO_THREAD_SAFETY_ANALYSIS_FIXME \
  PERFETTO_NO_THREAD_SAFETY_ANALYSIS

// Similar to NO_THREAD_SAFETY_ANALYSIS_FIXME, this macro marks a
// PERFETTO_GUARDED_BY annotation that needs to be fixed, because it is
// producing thread safety warning.  It disables the PERFETTO_GUARDED_BY.
#define PERFETTO_PERFETTO_GUARDED_BY_FIXME(x)

// Disables warnings for a single read operation.  This can be used to avoid
// warnings when it is known that the read is not actually involved in a race,
// but the compiler cannot confirm that.
#define PERFETTO_TS_UNCHECKED_READ(x) \
  perfetto::thread_safety_analysis::ts_unchecked_read(x)

namespace perfetto {
namespace thread_safety_analysis {

// Takes a reference to a guarded data member, and returns an unguarded
// reference.
template <typename T>
inline const T& ts_unchecked_read(const T& v)
    PERFETTO_NO_THREAD_SAFETY_ANALYSIS {
  return v;
}

template <typename T>
inline T& ts_unchecked_read(T& v) PERFETTO_NO_THREAD_SAFETY_ANALYSIS {
  return v;
}

}  // namespace thread_safety_analysis
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_BASE_THREAD_ANNOTATIONS_H_
