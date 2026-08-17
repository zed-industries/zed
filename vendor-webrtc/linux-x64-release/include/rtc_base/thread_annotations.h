//
// Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the LICENSE file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS.  All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
//
// Borrowed from
// https://code.google.com/p/gperftools/source/browse/src/base/thread_annotations.h
// but adapted for clang attributes instead of the gcc.
//
// This header file contains the macro definitions for thread safety
// annotations that allow the developers to document the locking policies
// of their multi-threaded code. The annotations can also help program
// analysis tools to identify potential thread safety issues.

#ifndef RTC_BASE_THREAD_ANNOTATIONS_H_
#define RTC_BASE_THREAD_ANNOTATIONS_H_

#if defined(__clang__) && (!defined(SWIG))
#define RTC_THREAD_ANNOTATION_ATTRIBUTE__(x) __attribute__((x))
#else
#define RTC_THREAD_ANNOTATION_ATTRIBUTE__(x)  // no-op
#endif

// Document if a shared variable/field needs to be protected by a lock.
// GUARDED_BY allows the user to specify a particular lock that should be
// held when accessing the annotated variable.
#define RTC_GUARDED_BY(x) RTC_THREAD_ANNOTATION_ATTRIBUTE__(guarded_by(x))

// Document if the memory location pointed to by a pointer should be guarded
// by a lock when dereferencing the pointer. Note that a pointer variable to a
// shared memory location could itself be a shared variable. For example, if a
// shared global pointer q, which is guarded by mu1, points to a shared memory
// location that is guarded by mu2, q should be annotated as follows:
//     int *q GUARDED_BY(mu1) PT_GUARDED_BY(mu2);
#define RTC_PT_GUARDED_BY(x) RTC_THREAD_ANNOTATION_ATTRIBUTE__(pt_guarded_by(x))

// Document the acquisition order between locks that can be held
// simultaneously by a thread. For any two locks that need to be annotated
// to establish an acquisition order, only one of them needs the annotation.
// (i.e. You don't have to annotate both locks with both ACQUIRED_AFTER
// and ACQUIRED_BEFORE.)
#define RTC_ACQUIRED_AFTER(x) \
  RTC_THREAD_ANNOTATION_ATTRIBUTE__(acquired_after(x))
#define RTC_ACQUIRED_BEFORE(x) \
  RTC_THREAD_ANNOTATION_ATTRIBUTE__(acquired_before(x))

// The following three annotations document the lock requirements for
// functions/methods.

// Document if a function expects certain locks to be held before it is called
#define RTC_EXCLUSIVE_LOCKS_REQUIRED(...) \
  RTC_THREAD_ANNOTATION_ATTRIBUTE__(exclusive_locks_required(__VA_ARGS__))
#define RTC_SHARED_LOCKS_REQUIRED(...) \
  RTC_THREAD_ANNOTATION_ATTRIBUTE__(shared_locks_required(__VA_ARGS__))

// Document the locks acquired in the body of the function. These locks
// cannot be held when calling this function (as google3's Mutex locks are
// non-reentrant).
#define RTC_LOCKS_EXCLUDED(...) \
  RTC_THREAD_ANNOTATION_ATTRIBUTE__(locks_excluded(__VA_ARGS__))

// Document the lock the annotated function returns without acquiring it.
#define RTC_LOCK_RETURNED(x) RTC_THREAD_ANNOTATION_ATTRIBUTE__(lock_returned(x))

// Document if a class/type is a lockable type (such as the Mutex class).
#define RTC_LOCKABLE RTC_THREAD_ANNOTATION_ATTRIBUTE__(lockable)

// Document if a class is a scoped lockable type (such as the MutexLock class).
#define RTC_SCOPED_LOCKABLE RTC_THREAD_ANNOTATION_ATTRIBUTE__(scoped_lockable)

// The following annotations specify lock and unlock primitives.
#define RTC_EXCLUSIVE_LOCK_FUNCTION(...) \
  RTC_THREAD_ANNOTATION_ATTRIBUTE__(exclusive_lock_function(__VA_ARGS__))

#define RTC_SHARED_LOCK_FUNCTION(...) \
  RTC_THREAD_ANNOTATION_ATTRIBUTE__(shared_lock_function(__VA_ARGS__))

#define RTC_EXCLUSIVE_TRYLOCK_FUNCTION(...) \
  RTC_THREAD_ANNOTATION_ATTRIBUTE__(exclusive_trylock_function(__VA_ARGS__))

#define RTC_SHARED_TRYLOCK_FUNCTION(...) \
  RTC_THREAD_ANNOTATION_ATTRIBUTE__(shared_trylock_function(__VA_ARGS__))

#define RTC_UNLOCK_FUNCTION(...) \
  RTC_THREAD_ANNOTATION_ATTRIBUTE__(unlock_function(__VA_ARGS__))

#define RTC_ASSERT_EXCLUSIVE_LOCK(...) \
  RTC_THREAD_ANNOTATION_ATTRIBUTE__(assert_exclusive_lock(__VA_ARGS__))

// An escape hatch for thread safety analysis to ignore the annotated function.
#define RTC_NO_THREAD_SAFETY_ANALYSIS \
  RTC_THREAD_ANNOTATION_ATTRIBUTE__(no_thread_safety_analysis)

#endif  // RTC_BASE_THREAD_ANNOTATIONS_H_
