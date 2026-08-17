// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_THREADING_SCOPED_BLOCKING_CALL_H_
#define BASE_THREADING_SCOPED_BLOCKING_CALL_H_

#include "base/base_export.h"
#include "base/functional/callback_forward.h"
#include "base/location.h"
#include "base/threading/scoped_blocking_call_internal.h"
#include "base/types/strong_alias.h"

namespace base {

// A "blocking call" refers to any call that causes the calling thread to wait
// off-CPU. It includes but is not limited to calls that wait on synchronous
// file I/O operations: read or write a file from disk, interact with a pipe or
// a socket, rename or delete a file, enumerate files in a directory, etc.
// Acquiring a low contention lock is not considered a blocking call.

// BlockingType indicates the likelihood that a blocking call will actually
// block.
enum class BlockingType {
  // The call might block (e.g. file I/O that might hit in memory cache).
  MAY_BLOCK,
  // The call will definitely block (e.g. cache already checked and now pinging
  // server synchronously).
  WILL_BLOCK
};

// This class must be instantiated in every scope where a blocking call is made
// and serves as a precise annotation of the scope that may/will block for the
// scheduler. When a ScopedBlockingCall is instantiated, it asserts that
// blocking calls are allowed in its scope with a call to
// base::AssertBlockingAllowed(). CPU usage should be minimal within that scope.
// //base APIs that block instantiate their own ScopedBlockingCall; it is not
// necessary to instantiate another ScopedBlockingCall in the scope where these
// APIs are used. Nested ScopedBlockingCalls are supported (mostly a no-op
// except for WILL_BLOCK nested within MAY_BLOCK which will result in immediate
// WILL_BLOCK semantics).
//
// Good:
//   Data data;
//   {
//     ScopedBlockingCall scoped_blocking_call(
//         FROM_HERE, BlockingType::WILL_BLOCK);
//     data = GetDataFromNetwork();
//   }
//   CPUIntensiveProcessing(data);
//
// Bad:
//   ScopedBlockingCall scoped_blocking_call(FROM_HERE,
//       BlockingType::WILL_BLOCK);
//   Data data = GetDataFromNetwork();
//   CPUIntensiveProcessing(data);  // CPU usage within a ScopedBlockingCall.
//
// Good:
//   Data a;
//   Data b;
//   {
//     ScopedBlockingCall scoped_blocking_call(
//         FROM_HERE, BlockingType::MAY_BLOCK);
//     a = GetDataFromMemoryCacheOrNetwork();
//     b = GetDataFromMemoryCacheOrNetwork();
//   }
//   CPUIntensiveProcessing(a);
//   CPUIntensiveProcessing(b);
//
// Bad:
//   ScopedBlockingCall scoped_blocking_call(
//       FROM_HERE, BlockingType::MAY_BLOCK);
//   Data a = GetDataFromMemoryCacheOrNetwork();
//   Data b = GetDataFromMemoryCacheOrNetwork();
//   CPUIntensiveProcessing(a);  // CPU usage within a ScopedBlockingCall.
//   CPUIntensiveProcessing(b);  // CPU usage within a ScopedBlockingCall.
//
// Good:
//   base::WaitableEvent waitable_event(...);
//   waitable_event.Wait();
//
// Bad:
//  base::WaitableEvent waitable_event(...);
//  ScopedBlockingCall scoped_blocking_call(
//      FROM_HERE, BlockingType::WILL_BLOCK);
//  waitable_event.Wait();  // Wait() instantiates its own ScopedBlockingCall.
//
// When a ScopedBlockingCall is instantiated from a ThreadPool parallel or
// sequenced task, the thread pool size is incremented to compensate for the
// blocked thread (more or less aggressively depending on BlockingType).
class BASE_EXPORT [[nodiscard]] ScopedBlockingCall
    : public internal::UncheckedScopedBlockingCall {
 public:
  ScopedBlockingCall(const Location& from_here, BlockingType blocking_type);
  ~ScopedBlockingCall();
};

// Usage reserved for //base callers.
namespace internal {

// This class must be instantiated in every scope where a sync primitive is
// used. When a ScopedBlockingCallWithBaseSyncPrimitives is instantiated, it
// asserts that sync primitives are allowed in its scope with a call to
// internal::AssertBaseSyncPrimitivesAllowed(). The same guidelines as for
// ScopedBlockingCall should be followed.
class BASE_EXPORT [[nodiscard]] ScopedBlockingCallWithBaseSyncPrimitives
    : public UncheckedScopedBlockingCall {
 public:
  ScopedBlockingCallWithBaseSyncPrimitives(const Location& from_here,
                                           BlockingType blocking_type);
  ~ScopedBlockingCallWithBaseSyncPrimitives();
};

}  // namespace internal

using IOJankReportingCallback =
    RepeatingCallback<void(int janky_intervals_per_minute,
                           int total_janks_per_minute)>;
using OnlyObservedThreadsForTest =
    StrongAlias<class OnlyObservedThreadsTag, bool>;
// Enables IO jank monitoring and reporting for this process. Should be called
// at most once per process and only if
// base::TimeTicks::IsConsistentAcrossProcesses() (the algorithm is unsafe
// otherwise). |reporting_callback| will be invoked each time a monitoring
// window completes, see internal::~IOJankMonitoringWindow() for details
// (must be thread-safe). |only_observed_threads| can be set to true to have
// the IOJank implementation ignore ScopedBlockingCalls on threads without a
// BlockingObserver in tests that need to deterministically observe
// ScopedBlockingCall side-effects.
void BASE_EXPORT EnableIOJankMonitoringForProcess(
    IOJankReportingCallback reporting_callback,
    OnlyObservedThreadsForTest only_observed_threads =
        OnlyObservedThreadsForTest(false));

}  // namespace base

#endif  // BASE_THREADING_SCOPED_BLOCKING_CALL_H_
