// Copyright 2024 The Centipede Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef FUZZTEST_CENTIPEDE_RESOURCE_RESOURCE_POOL_H_
#define FUZZTEST_CENTIPEDE_RESOURCE_RESOURCE_POOL_H_

#include <sys/syscall.h>
#include <sys/types.h>
#include <unistd.h>

#include <ostream>
#include <string>
#include <thread>  // NOLINT: for thread IDs.

#include "absl/base/thread_annotations.h"
#include "absl/status/status.h"
#include "absl/synchronization/mutex.h"
#include "absl/time/clock.h"
#include "absl/time/time.h"

namespace fuzztest::internal {

//------------------------------------------------------------------------------
//                              ResourcePool
//
// `ResourcePool` is an accounting mechanism to effectively share a limited
// resource between concurrent consumer threads, never exceeding a quota while
// maximizing resource utilization, and thus parallelism.
//
// The quota amount is picked by the client. It can be arbitrary, or it can
// reflect an actual amount of the resource on the system (e.g. the available
// RAM).
//
// Each of the consumer threads determines a conservative estimate of its peak
// resource utilization, and requests that amount from the pool. The request
// blocks until a sufficient amount becomes available. The amount is then
// "leased" to the thread for as long as it holds the lease token, and
// auto-returned back to the pool via RAII.
//
// Notes on using in combination with `ThreadPool`:
// 1. The requested number of concurrent threads in a `ThreadPool` is often an
//    attempt to indirectly control the resource usage. `ResourcePool` enables a
//    more direct way of controlling it, and therefore `ThreadPool`'s thread
//    count can be made as high as necessary for other purposes.
// 2. The `ResourcePool` object must is defined before the `ThreadPool` one to
//    avoid dangling references to a destructed pool in the threads.
//
// The currently supported (and explicitly instantiated in the .cc) types of
// the `ResourceT` template argument are `RUsageMemory` and `RUsageTiming`.
//
// Example:
//
// {
//   constexpr RUsageMemory kRssQuota{.mem_rss = RLimits::FreeRss() * 0.75};
//   ResourcePool rss_pool{kRssQuota};
//   ThreadPool threads{100};
//   for (...) {
//     threads.Schedule([&rss_pool]() {
//         // The thread blocks here until either the requested amount of RSS
//         // becomes available as the peer threads return their leases, or the
//         // 10-minute timeout expires.
//         const ResourcePool::LeaseToken rss_lease =
//             rss_pool.AcquireLeaseBlocking({
//                 .id = absl::StrCat("rss_", shard_id),
//                 .amount = RUsageMemory{.mem_rss = EstimateShardPeakRss()},
//                 .timeout = absl::Minutes(10),
//             });
//         CHECK_OK(rss_lease.status());
//         ...
//       }
//       // `rss_lease` dtor returns the leased RSS to `rss_pool` and unblocks
//       // other waiting threads.
//     );
//   }
// }  // `threads` dtor runs and joins the threads; then `rss_pool` dtor runs.
//
// TODO(ussuri): Add monitoring of claimed vs actual use by each leaser and
//  a final report of over- and underutilization (possibly via RUsageProfiler).
//------------------------------------------------------------------------------
template <typename ResourceT>
class ResourcePool {
 public:
  //----------------------------------------------------------------------------
  //                               Request
  //
  // Specifies a projected resource consumption between the time this request is
  // submitted and the time the acquired LeaseToken goes out of scope. A
  // convenient way to construct Requests is by using designated initializers
  // (cf. ResourcePool's top-level doc just above).
  struct LeaseRequest {
    // Optional. Used in the debug logging and always included in returned
    // failure statuses.
    std::string id = "";
    // Mandatory. Must be > `ResourceT::Zero()`; otherwise,
    // `AcquireLeaseBlocking()` immediately returns a failure.
    ResourceT amount;
    // Optional. `AcquireLeaseBlocking()` waits for up to this long for other
    // resource consumers to free up enough of it to satisfy this request. If
    // the required amount is still unavailable, `absl::DeadlineExceededError`
    // is returned. The default is to acquire or fail immediately.
    absl::Duration timeout = absl::ZeroDuration();
    // Should not normally be overridden by clients (but can be). Used for
    // logging only.
    absl::Time created_at = absl::Now();

    // The age of this request.
    absl::Duration age() const { return absl::Now() - created_at; }
  };

  //----------------------------------------------------------------------------
  //                             LeaseToken
  //
  // A RAII-based resource lock, similar to `MutexLock`. Must be held by a
  // client that called `AcquireLeaseBlocking()` for as long as it continues to
  // use the leased amount of the resource. Returns the resource to the leaser
  // `ResourcePool` in the dtor.
  class [[nodiscard]] LeaseToken {
   public:
    // Move-copyable only.
    LeaseToken(const LeaseToken&) = delete;
    LeaseToken& operator=(const LeaseToken&) = delete;
    LeaseToken(LeaseToken&&) noexcept = default;
    LeaseToken& operator=(LeaseToken&&) noexcept = delete;

    // Automatically returns itself to the leaser (the issuing ResourcePool).
    ~LeaseToken();

    // The outcome of resource acquisition (ie. of
    // `ResourcePool::AcquireLeaseBlocking()`). Must be consulted by the client
    // at least once, otherwise the dtor will CHECK.
    const absl::Status& status() const;
    // The originating request.
    const LeaseRequest& request() const;
    // A short description that can be used in logs.
    std::string id() const;
    // The thread ID that submitted the request.
    std::thread::id thread_id() const;
    // The creation time and the age of the lease.
    absl::Time created_at() const;
    absl::Duration age() const;

   private:
    // Only ResourcePool can create.
    friend class ResourcePool;

    // Constructs a token for a successfully acquired resource.
    LeaseToken(ResourcePool& leaser, LeaseRequest request);
    // Constructs a token for a resource that couldn't be acquired.
    LeaseToken(ResourcePool& leaser, LeaseRequest request, absl::Status error);

    friend std::ostream& operator<<(std::ostream& os, const LeaseToken& lt) {
      return os << lt.id() << ": " << lt.request().amount.ShortStr();
    }

    ResourcePool& leaser_;
    LeaseRequest request_ = {};
    absl::Status status_ = absl::OkStatus();
    mutable bool status_checked_ = false;
    std::thread::id thread_id_ = std::this_thread::get_id();
    absl::Time created_at_ = absl::Now();
  };

  // `quota` is the initially available amount of the resource to be shared
  // between all concurrent consumers.
  // Example: `ResourcePool pool{RUsageMemory{.mem_rss = ComputeFreeRss()}};`.
  explicit ResourcePool(const ResourceT& quota);

  // Blocks the current thread and waits until `request.amount` of the resources
  // becomes available in the pool or until `request.timeout` expires, whichever
  // comes first. When the returned object goes out of scope, the leased
  // resource gets automatically returned to the pool via RAII.
  // Example: `const auto lease = pool.AcquireLeaseBlocking({.mem_rss = 100});`.
  LeaseToken AcquireLeaseBlocking(LeaseRequest&& request);

 private:
  // `LeaseToken`'s dtor calls this to return the leased resource to the pool.
  void ReturnLease(const LeaseToken& lease);

  // The total pool capacity.
  const ResourceT quota_;

  // The currently available amount.
  absl::Mutex pool_mu_;
  ResourceT pool_ ABSL_GUARDED_BY(pool_mu_);
};

// An explicit deduction guide to allow `ResourcePool pool{RUsageMemory{...}}`.
template <typename R>
ResourcePool(R r) -> ResourcePool<R>;

}  // namespace fuzztest::internal

#endif  // FUZZTEST_CENTIPEDE_RESOURCE_RESOURCE_POOL_H_
