// Copyright 2023 Google LLC
// SPDX-License-Identifier: Apache-2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Modified from BSD-licensed code
// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// See https://github.com/libjxl/libjxl/blob/main/LICENSE.

#ifndef HIGHWAY_HWY_CONTRIB_THREAD_POOL_THREAD_POOL_H_
#define HIGHWAY_HWY_CONTRIB_THREAD_POOL_THREAD_POOL_H_

#include <stddef.h>
#include <stdint.h>
#include <stdio.h>  // snprintf

#include <array>
#include <atomic>
#include <string>
#include <thread>  // NOLINT
#include <vector>

#include "third_party/highway/hwy/detect_compiler_arch.h"
#if HWY_OS_FREEBSD
#include <pthread_np.h>
#endif

#include "third_party/highway/hwy/aligned_allocator.h"  // HWY_ALIGNMENT
#include "third_party/highway/hwy/auto_tune.h"
#include "third_party/highway/hwy/base.h"
#include "third_party/highway/hwy/cache_control.h"  // Pause
#include "third_party/highway/hwy/contrib/thread_pool/futex.h"
#include "third_party/highway/hwy/contrib/thread_pool/spin.h"
#include "third_party/highway/hwy/contrib/thread_pool/topology.h"
#include "third_party/highway/hwy/stats.h"
#include "third_party/highway/hwy/timer.h"

// Define to HWY_NOINLINE to see profiles of `WorkerRun*` and waits.
#define HWY_POOL_PROFILE

namespace hwy {

// Sets the name of the current thread to the format string `format`, which must
// include %d for `thread`. Currently only implemented for pthreads (*nix and
// OSX); Windows involves throwing an exception.
static inline void SetThreadName(const char* format, int thread) {
  char buf[16] = {};  // Linux limit, including \0
  const int chars_written = snprintf(buf, sizeof(buf), format, thread);
  HWY_ASSERT(0 < chars_written &&
             chars_written <= static_cast<int>(sizeof(buf) - 1));

#if HWY_OS_LINUX && (!defined(__ANDROID__) || __ANDROID_API__ >= 19)
  HWY_ASSERT(0 == pthread_setname_np(pthread_self(), buf));
#elif HWY_OS_FREEBSD
  HWY_ASSERT(0 == pthread_set_name_np(pthread_self(), buf));
#elif HWY_OS_APPLE
  // Different interface: single argument, current thread only.
  HWY_ASSERT(0 == pthread_setname_np(buf));
#endif
}

// Whether workers should block or spin.
enum class PoolWaitMode : uint8_t { kBlock = 1, kSpin };

namespace pool {

#ifndef HWY_POOL_VERBOSITY
#define HWY_POOL_VERBOSITY 0
#endif

static constexpr int kVerbosity = HWY_POOL_VERBOSITY;

// Some CPUs already have more than this many threads, but rather than one
// large pool, we assume applications create multiple pools, ideally per
// cluster (cores sharing a cache), because this improves locality and barrier
// latency. In that case, this is a generous upper bound.
static constexpr size_t kMaxThreads = 63;

// Generates a random permutation of [0, size). O(1) storage.
class ShuffledIota {
 public:
  ShuffledIota() : coprime_(1) {}  // for Worker
  explicit ShuffledIota(uint32_t coprime) : coprime_(coprime) {}

  // Returns the next after `current`, using an LCG-like generator.
  uint32_t Next(uint32_t current, const Divisor64& divisor) const {
    HWY_DASSERT(current < divisor.GetDivisor());
    // (coprime * i + current) % size, see https://lemire.me/blog/2017/09/18/.
    return static_cast<uint32_t>(divisor.Remainder(current + coprime_));
  }

  // Returns true if a and b have no common denominator except 1. Based on
  // binary GCD. Assumes a and b are nonzero. Also used in tests.
  static bool CoprimeNonzero(uint32_t a, uint32_t b) {
    const size_t trailing_a = Num0BitsBelowLS1Bit_Nonzero32(a);
    const size_t trailing_b = Num0BitsBelowLS1Bit_Nonzero32(b);
    // If both have at least one trailing zero, they are both divisible by 2.
    if (HWY_MIN(trailing_a, trailing_b) != 0) return false;

    // If one of them has a trailing zero, shift it out.
    a >>= trailing_a;
    b >>= trailing_b;

    for (;;) {
      // Swap such that a >= b.
      const uint32_t tmp_a = a;
      a = HWY_MAX(tmp_a, b);
      b = HWY_MIN(tmp_a, b);

      // When the smaller number is 1, they were coprime.
      if (b == 1) return true;

      a -= b;
      // a == b means there was a common factor, so not coprime.
      if (a == 0) return false;
      a >>= Num0BitsBelowLS1Bit_Nonzero32(a);
    }
  }

  // Returns another coprime >= `start`, or 1 for small `size`.
  // Used to seed independent ShuffledIota instances.
  static uint32_t FindAnotherCoprime(uint32_t size, uint32_t start) {
    if (size <= 2) {
      return 1;
    }

    // Avoids even x for even sizes, which are sure to be rejected.
    const uint32_t inc = (size & 1) ? 1 : 2;

    for (uint32_t x = start | 1; x < start + size * 16; x += inc) {
      if (CoprimeNonzero(x, static_cast<uint32_t>(size))) {
        return x;
      }
    }

    HWY_UNREACHABLE;
  }

  uint32_t coprime_;
};

// 'Policies' suitable for various worker counts and locality. To define a
// new class, add an enum and update `ToString` plus `FunctorAddWait`. The
// enumerators must be contiguous so we can iterate over them.
enum class WaitType : uint8_t {
  kBlock,
  kSpin1,
  kSpinSeparate,
  kSentinel  // Must be last.
};
enum class BarrierType : uint8_t {
  kOrdered,
  kCounter1,
  kCounter2,
  kCounter4,
  kGroup2,
  kGroup4,
  kSentinel  // Must be last.
};

// For printing which is in use.
static inline const char* ToString(WaitType type) {
  switch (type) {
    case WaitType::kBlock:
      return "Block";
    case WaitType::kSpin1:
      return "Single";
    case WaitType::kSpinSeparate:
      return "Separate";
    case WaitType::kSentinel:
      return nullptr;
    default:
      HWY_UNREACHABLE;
  }
}

static inline const char* ToString(BarrierType type) {
  switch (type) {
    case BarrierType::kOrdered:
      return "Ordered";
    case BarrierType::kCounter1:
      return "Counter1";
    case BarrierType::kCounter2:
      return "Counter2";
    case BarrierType::kCounter4:
      return "Counter4";
    case BarrierType::kGroup2:
      return "Group2";
    case BarrierType::kGroup4:
      return "Group4";
    case BarrierType::kSentinel:
      return nullptr;
    default:
      HWY_UNREACHABLE;
  }
}

// We want predictable struct/class sizes so we can reason about cache lines.
#pragma pack(push, 1)

// Parameters governing the main and worker thread behavior. Can be updated at
// runtime via `SetWaitMode`. Both have copies which are carefully synchronized
// (two-phase barrier). 64-bit allows adding fields (e.g. for load-balancing)
// without having to bit-pack members, and is fine because this is only moved
// with relaxed stores, hence we do not have to fit it in the 32 futex bits.
class Config {  // 8 bytes
 public:
  static std::vector<Config> AllCandidates(PoolWaitMode wait_mode,
                                           size_t num_threads) {
    std::vector<SpinType> spin_types(size_t{1}, DetectSpin());
    // Monitor-based spin may be slower, so also try Pause.
    if (spin_types[0] != SpinType::kPause) {
      spin_types.push_back(SpinType::kPause);
    }

    std::vector<WaitType> wait_types;
    if (wait_mode == PoolWaitMode::kSpin) {
      // All except `kBlock`.
      for (size_t wait = 0;; ++wait) {
        const WaitType wait_type = static_cast<WaitType>(wait);
        if (wait_type == WaitType::kSentinel) break;
        if (wait_type != WaitType::kBlock) wait_types.push_back(wait_type);
      }
    } else {
      wait_types.push_back(WaitType::kBlock);
    }

    std::vector<BarrierType> barrier_types;
    // Note that casting an integer is UB if there is no matching enumerator,
    // but we define a sentinel to prevent this.
    for (size_t barrier = 0;; ++barrier) {
      const BarrierType barrier_type = static_cast<BarrierType>(barrier);
      if (barrier_type == BarrierType::kSentinel) break;
      // If <= 2 workers, group size of 4 is the same as 2.
      if (num_threads <= 1 && barrier_type == BarrierType::kCounter4) continue;
      if (num_threads <= 1 && barrier_type == BarrierType::kGroup4) continue;
      barrier_types.push_back(barrier_type);
    }

    std::vector<Config> candidates;
    candidates.reserve(50);
    for (const SpinType spin_type : spin_types) {
      for (const WaitType wait_type : wait_types) {
        for (const BarrierType barrier_type : barrier_types) {
          candidates.emplace_back(spin_type, wait_type, barrier_type);
        }
      }
    }
    return candidates;
  }

  std::string ToString() const {
    char buf[128];
    snprintf(buf, sizeof(buf), "%14s %9s %9s", hwy::ToString(spin_type),
             pool::ToString(wait_type), pool::ToString(barrier_type));
    return buf;
  }

  Config() {}
  Config(SpinType spin_type, WaitType wait_type, BarrierType barrier_type)
      : spin_type(spin_type),
        wait_type(wait_type),
        barrier_type(barrier_type),
        exit(false) {}

  SpinType spin_type;
  WaitType wait_type;
  BarrierType barrier_type;
  bool exit;
  uint32_t reserved = 0;
};
static_assert(sizeof(Config) == 8, "");

// Per-worker state used by both main and worker threads. `ThreadFunc`
// (threads) and `ThreadPool` (main) have a few additional members of their own.
class alignas(HWY_ALIGNMENT) Worker {  // HWY_ALIGNMENT bytes
  static constexpr size_t kMaxVictims = 4;

  static constexpr auto kAcq = std::memory_order_acquire;
  static constexpr auto kRel = std::memory_order_release;

 public:
  Worker(const size_t worker, const size_t num_threads,
         const Divisor64& div_workers)
      : worker_(worker), num_threads_(num_threads), workers_(this - worker) {
    (void)padding_;

    HWY_DASSERT(IsAligned(this, HWY_ALIGNMENT));
    HWY_DASSERT(worker <= num_threads);
    const size_t num_workers = static_cast<size_t>(div_workers.GetDivisor());
    num_victims_ = static_cast<uint32_t>(HWY_MIN(kMaxVictims, num_workers));

    // Increase gap between coprimes to reduce collisions.
    const uint32_t coprime = ShuffledIota::FindAnotherCoprime(
        static_cast<uint32_t>(num_workers),
        static_cast<uint32_t>((worker + 1) * 257 + worker * 13));
    const ShuffledIota shuffled_iota(coprime);

    // To simplify `WorkerRun`, this worker is the first to 'steal' from.
    victims_[0] = static_cast<uint32_t>(worker);
    for (uint32_t i = 1; i < num_victims_; ++i) {
      victims_[i] = shuffled_iota.Next(victims_[i - 1], div_workers);
      HWY_DASSERT(victims_[i] != worker);
    }
  }

  // Placement-newed by `WorkerLifecycle`, we do not expect any copying.
  Worker(const Worker&) = delete;
  Worker& operator=(const Worker&) = delete;

  size_t Index() const { return worker_; }
  Worker* AllWorkers() { return workers_; }
  const Worker* AllWorkers() const { return workers_; }
  size_t NumThreads() const { return num_threads_; }

  // ------------------------ Per-worker storage for `SendConfig`

  Config LatchedConfig() const { return latched_; }
  // For workers, but no harm if also called by main thread.
  void LatchConfig(Config copy) { latched_ = copy; }

  // ------------------------ Task assignment

  // Called from the main thread.
  void SetRange(const uint64_t begin, const uint64_t end) {
    my_begin_.store(begin, kRel);
    my_end_.store(end, kRel);
  }

  uint64_t MyEnd() const { return my_end_.load(kAcq); }

  Span<const uint32_t> Victims() const {
    return hwy::Span<const uint32_t>(victims_.data(),
                                     static_cast<size_t>(num_victims_));
  }

  // Returns the next task to execute. If >= MyEnd(), it must be skipped.
  uint64_t WorkerReserveTask() {
    // TODO(janwas): replace with cooperative work-stealing.
    return my_begin_.fetch_add(1, std::memory_order_relaxed);
  }

  // ------------------------ Waiter: Threads wait for tasks

  // WARNING: some `Wait*` do not set this for all Worker instances. For
  // example, `WaitType::kBlock` only uses the first worker's `Waiter` because
  // one futex can wake multiple waiters. Hence we never load this directly
  // without going through `Wait*` policy classes, and must ensure all threads
  // use the same wait mode.

  const std::atomic<uint32_t>& Waiter() const { return wait_epoch_; }
  std::atomic<uint32_t>& MutableWaiter() { return wait_epoch_; }  // futex
  void StoreWaiter(uint32_t epoch) { wait_epoch_.store(epoch, kRel); }

  // ------------------------ Barrier: Main thread waits for workers

  const std::atomic<uint32_t>& Barrier() const { return barrier_epoch_; }
  std::atomic<uint32_t>& MutableBarrier() { return barrier_epoch_; }
  void StoreBarrier(uint32_t epoch) { barrier_epoch_.store(epoch, kRel); }

 private:
  // Atomics first because arm7 clang otherwise makes them unaligned.

  // Set by `SetRange`:
  alignas(8) std::atomic<uint64_t> my_begin_;
  alignas(8) std::atomic<uint64_t> my_end_;

  // Use u32 to match futex.h.
  alignas(4) std::atomic<uint32_t> wait_epoch_{0};
  alignas(4) std::atomic<uint32_t> barrier_epoch_{0};  // is reset

  uint32_t num_victims_;  // <= kPoolMaxVictims
  std::array<uint32_t, kMaxVictims> victims_;

  Config latched_;

  const size_t worker_;
  const size_t num_threads_;
  Worker* const workers_;

  uint8_t padding_[HWY_ALIGNMENT - 64 - sizeof(victims_)];
};
static_assert(sizeof(Worker) == HWY_ALIGNMENT, "");

#pragma pack(pop)

// Creates/destroys `Worker` using preallocated storage. See comment at
// `ThreadPool::worker_bytes_` for why we do not dynamically allocate.
class WorkerLifecycle {  // 0 bytes
 public:
  // Placement new for `Worker` into `storage` because its ctor requires
  // the worker index. Returns array of all workers.
  static Worker* Init(uint8_t* storage, size_t num_threads,
                      const Divisor64& div_workers) {
    Worker* workers = new (storage) Worker(0, num_threads, div_workers);
    for (size_t worker = 1; worker <= num_threads; ++worker) {
      new (Addr(storage, worker)) Worker(worker, num_threads, div_workers);
      // Ensure pointer arithmetic is the same (will be used in Destroy).
      HWY_DASSERT(reinterpret_cast<uintptr_t>(workers + worker) ==
                  reinterpret_cast<uintptr_t>(Addr(storage, worker)));
    }

    // Publish non-atomic stores in `workers`.
    std::atomic_thread_fence(std::memory_order_release);

    return workers;
  }

  static void Destroy(Worker* workers, size_t num_threads) {
    for (size_t worker = 0; worker <= num_threads; ++worker) {
      workers[worker].~Worker();
    }
  }

 private:
  static uint8_t* Addr(uint8_t* storage, size_t worker) {
    return storage + worker * sizeof(Worker);
  }
};

#pragma pack(push, 1)
// Stores arguments to `Run`: the function and range of task indices. Set by
// the main thread, read by workers including the main thread.
class alignas(8) Tasks {
  static constexpr auto kAcq = std::memory_order_acquire;

  // Signature of the (internal) function called from workers(s) for each
  // `task` in the [`begin`, `end`) passed to Run(). Closures (lambdas) do not
  // receive the first argument, which points to the lambda object.
  typedef void (*RunFunc)(const void* opaque, uint64_t task, size_t worker);

 public:
  Tasks() { HWY_DASSERT(IsAligned(this, 8)); }

  template <class Closure>
  void Set(uint64_t begin, uint64_t end, const Closure& closure) {
    constexpr auto kRel = std::memory_order_release;
    // `TestTasks` and `SetWaitMode` call this with `begin == end`.
    HWY_DASSERT(begin <= end);
    begin_.store(begin, kRel);
    end_.store(end, kRel);
    func_.store(static_cast<RunFunc>(&CallClosure<Closure>), kRel);
    opaque_.store(reinterpret_cast<const void*>(&closure), kRel);
  }

  // Assigns workers their share of `[begin, end)`. Called from the main
  // thread; workers are initializing or spinning for a command.
  static void DivideRangeAmongWorkers(const uint64_t begin, const uint64_t end,
                                      const Divisor64& div_workers,
                                      Worker* workers) {
    const size_t num_workers = static_cast<size_t>(div_workers.GetDivisor());
    HWY_DASSERT(num_workers > 1);  // Else Run() runs on the main thread.
    HWY_DASSERT(begin <= end);
    const size_t num_tasks = static_cast<size_t>(end - begin);

    // Assigning all remainders to the last worker causes imbalance. We instead
    // give one more to each worker whose index is less. This may be zero when
    // called from `TestTasks`.
    const size_t min_tasks = static_cast<size_t>(div_workers.Divide(num_tasks));
    const size_t remainder =
        static_cast<size_t>(div_workers.Remainder(num_tasks));

    uint64_t my_begin = begin;
    for (size_t worker = 0; worker < num_workers; ++worker) {
      const uint64_t my_end = my_begin + min_tasks + (worker < remainder);
      workers[worker].SetRange(my_begin, my_end);
      my_begin = my_end;
    }
    HWY_DASSERT(my_begin == end);
  }

  // Runs the worker's assigned range of tasks, plus work stealing if needed.
  HWY_POOL_PROFILE void WorkerRun(Worker* worker) const {
    if (NumTasks() > worker->NumThreads() + 1) {
      WorkerRunWithStealing(worker);
    } else {
      WorkerRunSingle(worker->Index());
    }
  }

 private:
  // Special case for <= 1 task per worker, where stealing is unnecessary.
  void WorkerRunSingle(size_t worker) const {
    const uint64_t begin = begin_.load(kAcq);
    const uint64_t end = end_.load(kAcq);
    HWY_DASSERT(begin <= end);

    const uint64_t task = begin + worker;
    // We might still have more workers than tasks, so check first.
    if (HWY_LIKELY(task < end)) {
      const void* opaque = Opaque();
      const RunFunc func = Func();
      func(opaque, task, worker);
    }
  }

  // Must be called for each `worker` in [0, num_workers).
  //
  // A prior version of this code attempted to assign only as much work as a
  // worker will actually use. As with OpenMP's 'guided' strategy, we assigned
  // remaining/(k*num_threads) in each iteration. Although the worst-case
  // imbalance is bounded, this required several rounds of work allocation, and
  // the atomic counter did not scale to > 30 threads.
  //
  // We now use work stealing instead, where already-finished workers look for
  // and perform work from others, as if they were that worker. This deals with
  // imbalances as they arise, but care is required to reduce contention. We
  // randomize the order in which threads choose victims to steal from.
  HWY_POOL_PROFILE void WorkerRunWithStealing(Worker* worker) const {
    Worker* workers = worker->AllWorkers();
    const size_t index = worker->Index();
    const RunFunc func = Func();
    const void* opaque = Opaque();

    // For each worker in random order, starting with our own, attempt to do
    // all their work.
    for (uint32_t victim : worker->Victims()) {
      Worker* other_worker = workers + victim;

      // Until all of other_worker's work is done:
      const uint64_t other_end = other_worker->MyEnd();
      for (;;) {
        // The worker that first sets `task` to `other_end` exits this loop.
        // After that, `task` can be incremented up to `num_workers - 1` times,
        // once per other worker.
        const uint64_t task = other_worker->WorkerReserveTask();
        if (HWY_UNLIKELY(task >= other_end)) {
          hwy::Pause();  // Reduce coherency traffic while stealing.
          break;
        }
        // Pass the index we are actually running on; this is important
        // because it is the TLS index for user code.
        func(opaque, task, index);
      }
    }
  }

  size_t NumTasks() const {
    return static_cast<size_t>(end_.load(kAcq) - begin_.load(kAcq));
  }

  const void* Opaque() const { return opaque_.load(kAcq); }
  RunFunc Func() const { return func_.load(kAcq); }

  // Calls closure(task, worker). Signature must match `RunFunc`.
  template <class Closure>
  static void CallClosure(const void* opaque, uint64_t task, size_t worker) {
    (*reinterpret_cast<const Closure*>(opaque))(task, worker);
  }

  std::atomic<uint64_t> begin_;
  std::atomic<uint64_t> end_;
  std::atomic<RunFunc> func_;
  std::atomic<const void*> opaque_;
};
static_assert(sizeof(Tasks) == 16 + 2 * sizeof(void*), "");
#pragma pack(pop)

// ------------------------------ Threads wait, main wakes them

// Considerations:
// - uint32_t storage per `Worker` so we can use `futex.h`.
// - avoid atomic read-modify-write. These are implemented on x86 using a LOCK
//   prefix, which interferes with other cores' cache-coherency transactions
//   and drains our core's store buffer. We use only store-release and
//   load-acquire. Although expressed using `std::atomic`, these are normal
//   loads/stores in the strong x86 memory model.
// - prefer to avoid resetting the state. "Sense-reversing" (flipping a flag)
//   would work, but we we prefer an 'epoch' counter because it is more useful
//   and easier to understand/debug, and as fast.

// Both the main thread and each worker maintain their own counter, which are
// implicitly synchronized by the barrier. To wake, the main thread does a
// store-release, and each worker does a load-acquire. The policy classes differ
// in whether they block or spin (with pause/monitor to reduce power), and
// whether workers check their own counter or a shared one.
//
// All methods are const because they only use storage in `Worker`, and we
// prefer to pass const-references to empty classes to enable type deduction.

// Futex: blocking reduces apparent CPU usage, but has higher wake latency.
struct WaitBlock {
  WaitType Type() const { return WaitType::kBlock; }

  // Wakes all workers by storing the current `epoch`.
  void WakeWorkers(Worker* workers, const uint32_t epoch) const {
    HWY_DASSERT(epoch != 0);
    workers[0].StoreWaiter(epoch);
    WakeAll(workers[0].MutableWaiter());  // futex: expensive syscall
  }

  // Waits until `WakeWorkers(_, epoch)` has been called.
  template <class Spin>
  void UntilWoken(const Worker* worker, const Spin& /*spin*/,
                  const uint32_t epoch) const {
    BlockUntilDifferent(epoch - 1, worker->AllWorkers()->Waiter());
  }
};

// Single u32: single store by the main thread. All worker threads poll this
// one cache line and thus have it in a shared state, which means the store
// will invalidate each of them, leading to more transactions than SpinSeparate.
struct WaitSpin1 {
  WaitType Type() const { return WaitType::kSpin1; }

  void WakeWorkers(Worker* workers, const uint32_t epoch) const {
    workers[0].StoreWaiter(epoch);
  }

  template <class Spin>
  void UntilWoken(const Worker* worker, const Spin& spin,
                  const uint32_t epoch) const {
    (void)spin.UntilEqual(epoch, worker->AllWorkers()->Waiter());
    // TODO: store reps in stats.
  }
};

// Separate u32 per thread: more stores for the main thread, but each worker
// only polls its own cache line, leading to fewer cache-coherency transactions.
struct WaitSpinSeparate {
  WaitType Type() const { return WaitType::kSpinSeparate; }

  void WakeWorkers(Worker* workers, const uint32_t epoch) const {
    for (size_t thread = 0; thread < workers->NumThreads(); ++thread) {
      workers[thread].StoreWaiter(epoch);
    }
  }

  template <class Spin>
  void UntilWoken(const Worker* worker, const Spin& spin,
                  const uint32_t epoch) const {
    (void)spin.UntilEqual(epoch, worker->Waiter());
    // TODO: store reps in stats.
  }
};

// ------------------------------ Barrier: Main thread waits for workers

// Single atomic counter. TODO: remove if not competitive?
template <size_t kShards>
class BarrierCounter {
  static_assert(kShards == 1 || kShards == 2 || kShards == 4, "");  // pow2

 public:
  BarrierType Type() const {
    return kShards == 1   ? BarrierType::kCounter1
           : kShards == 2 ? BarrierType::kCounter2
                          : BarrierType::kCounter4;
  }

  void Reset(Worker* workers) const {
    for (size_t i = 0; i < kShards; ++i) {
      // Use last worker(s) to avoid contention with other stores to the Worker.
      // Note that there are kMaxThreads + 1 workers, hence i == 0 is the last.
      workers[kMaxThreads - i].StoreBarrier(0);
    }
  }

  template <class Spin>
  void WorkerReached(Worker* worker, const Spin& /*spin*/,
                     uint32_t /*epoch*/) const {
    const size_t shard = worker->Index() & (kShards - 1);
    const auto kAcqRel = std::memory_order_acq_rel;
    worker->AllWorkers()[kMaxThreads - shard].MutableBarrier().fetch_add(
        1, kAcqRel);
  }

  template <class Spin>
  void UntilReached(size_t num_threads, const Worker* workers, const Spin& spin,
                    uint32_t /*epoch*/) const {
    HWY_IF_CONSTEXPR(kShards == 1) {
      (void)spin.UntilEqual(static_cast<uint32_t>(num_threads),
                            workers[kMaxThreads].Barrier());
    }
    HWY_IF_CONSTEXPR(kShards == 2) {
      const auto kAcq = std::memory_order_acquire;
      for (;;) {
        hwy::Pause();
        const uint64_t sum = workers[kMaxThreads - 0].Barrier().load(kAcq) +
                             workers[kMaxThreads - 1].Barrier().load(kAcq);
        if (sum == num_threads) break;
      }
    }
    HWY_IF_CONSTEXPR(kShards == 4) {
      const auto kAcq = std::memory_order_acquire;
      for (;;) {
        hwy::Pause();
        const uint64_t sum = workers[kMaxThreads - 0].Barrier().load(kAcq) +
                             workers[kMaxThreads - 1].Barrier().load(kAcq) +
                             workers[kMaxThreads - 2].Barrier().load(kAcq) +
                             workers[kMaxThreads - 3].Barrier().load(kAcq);
        if (sum == num_threads) break;
      }
    }
  }
};

// As with the wait, a store-release of the same local epoch counter serves as a
// "have arrived" flag that does not require resetting.

// Main thread loops over each worker.
class BarrierOrdered {
 public:
  BarrierType Type() const { return BarrierType::kOrdered; }

  void Reset(Worker* /*workers*/) const {}

  template <class Spin>
  void WorkerReached(Worker* worker, const Spin&, uint32_t epoch) const {
    worker->StoreBarrier(epoch);
  }

  template <class Spin>
  void UntilReached(size_t num_threads, const Worker* workers, const Spin& spin,
                    uint32_t epoch) const {
    for (size_t i = 0; i < num_threads; ++i) {
      (void)spin.UntilEqual(epoch, workers[i].Barrier());
    }
  }
};

// Leader threads wait for others in the group, main thread loops over leaders.
template <size_t kGroupSize>
class BarrierGroup {
 public:
  BarrierType Type() const {
    return kGroupSize == 2 ? BarrierType::kGroup2 : BarrierType::kGroup4;
  }

  void Reset(Worker* /*workers*/) const {}

  template <class Spin>
  void WorkerReached(Worker* worker, const Spin& spin, uint32_t epoch) const {
    const size_t thread = worker->Index();
    // Leaders wait for all others in their group before marking themselves.
    if (thread % kGroupSize == 0) {
      for (size_t i = thread + 1;
           i < HWY_MIN(thread + kGroupSize, worker->NumThreads()); ++i) {
        (void)spin.UntilEqual(epoch, worker->AllWorkers()[i].Barrier());
      }
    }
    worker->StoreBarrier(epoch);
  }

  template <class Spin>
  void UntilReached(size_t num_threads, const Worker* workers, const Spin& spin,
                    uint32_t epoch) const {
    for (size_t i = 0; i < num_threads; i += kGroupSize) {
      (void)spin.UntilEqual(epoch, workers[i].Barrier());
    }
  }
};

// ------------------------------ Inlining policy classes

// We want to inline the various spin/wait/barrier policy classes into larger
// code sections because both the main and worker threads use two or three of
// them at a time, and we do not want separate branches around each.
//
// We generate code for three combinations of the enums, hence implement
// composable adapters that 'add' `Wait` and `Barrier` arguments. `spin.h`
// provides a `CallWithSpin`, hence it is the outermost. C++11 lacks generic
// lambdas, so we implement these as classes.
template <class Func>
class FunctorAddWait {
 public:
  FunctorAddWait(WaitType wait_type, Func&& func)
      : func_(std::forward<Func>(func)), wait_type_(wait_type) {}

  template <class Spin>
  HWY_INLINE void operator()(const Spin& spin) {
    switch (wait_type_) {
      case WaitType::kBlock:
        return func_(spin, WaitBlock());
      case WaitType::kSpin1:
        return func_(spin, WaitSpin1());
      case WaitType::kSpinSeparate:
        return func_(spin, WaitSpinSeparate());
      default:
        HWY_UNREACHABLE;
    }
  }

 private:
  Func&& func_;
  WaitType wait_type_;
};

template <class Func>
class FunctorAddBarrier {
 public:
  FunctorAddBarrier(BarrierType barrier_type, Func&& func)
      : func_(std::forward<Func>(func)), barrier_type_(barrier_type) {}

  template <class Wait>
  HWY_INLINE void operator()(const Wait& wait) {
    switch (barrier_type_) {
      case BarrierType::kOrdered:
        return func_(wait, BarrierOrdered());
      case BarrierType::kCounter1:
        return func_(wait, BarrierCounter<1>());
      case BarrierType::kCounter2:
        return func_(wait, BarrierCounter<2>());
      case BarrierType::kCounter4:
        return func_(wait, BarrierCounter<4>());
      case BarrierType::kGroup2:
        return func_(wait, BarrierGroup<2>());
      case BarrierType::kGroup4:
        return func_(wait, BarrierGroup<4>());
      default:
        HWY_UNREACHABLE;
    }
  }
  template <class Spin, class Wait>
  HWY_INLINE void operator()(const Spin& spin, const Wait& wait) {
    switch (barrier_type_) {
      case BarrierType::kOrdered:
        return func_(spin, wait, BarrierOrdered());
      case BarrierType::kCounter1:
        return func_(spin, wait, BarrierCounter<1>());
      case BarrierType::kCounter2:
        return func_(spin, wait, BarrierCounter<2>());
      case BarrierType::kCounter4:
        return func_(spin, wait, BarrierCounter<4>());
      case BarrierType::kGroup2:
        return func_(spin, wait, BarrierGroup<2>());
      case BarrierType::kGroup4:
        return func_(spin, wait, BarrierGroup<4>());
      default:
        HWY_UNREACHABLE;
    }
  }

 private:
  Func&& func_;
  BarrierType barrier_type_;
};

// Calls unrolled code selected by all 3 enums.
template <class Func>
HWY_INLINE void CallWithConfig(const Config& config, Func&& func) {
  CallWithSpin(
      config.spin_type,
      FunctorAddWait<FunctorAddBarrier<Func>>(
          config.wait_type, FunctorAddBarrier<Func>(config.barrier_type,
                                                    std::forward<Func>(func))));
}

// For `WorkerAdapter`, `Spin` and `Wait`.
template <class Func>
HWY_INLINE void CallWithSpinWait(const Config& config, Func&& func) {
  CallWithSpin(
      config.spin_type,
      FunctorAddWait<Func>(config.wait_type, std::forward<Func>(func)));
}

// For `WorkerAdapter`, only `Spin` and `Barrier`.
template <class Func>
HWY_INLINE void CallWithSpinBarrier(const Config& config, Func&& func) {
  CallWithSpin(
      config.spin_type,
      FunctorAddBarrier<Func>(config.barrier_type, std::forward<Func>(func)));
}

// ------------------------------ Adapters

// Logic of the main and worker threads, again packaged as classes because
// C++11 lacks generic lambdas, called by `CallWith*`.

class MainAdapter {
 public:
  MainAdapter(Worker* main, const Tasks* tasks) : main_(main), tasks_(tasks) {}

  void SetEpoch(uint32_t epoch) { epoch_ = epoch; }

  template <class Spin, class Wait, class Barrier>
  HWY_POOL_PROFILE void operator()(const Spin& spin, const Wait& wait,
                                   const Barrier& barrier) const {
    Worker* workers = main_->AllWorkers();
    const size_t num_threads = main_->NumThreads();
    barrier.Reset(workers);

    wait.WakeWorkers(workers, epoch_);
    // Threads might still be starting up and wake up late, but we wait for
    // them at the barrier below.

    // Also perform work on the main thread before the barrier.
    tasks_->WorkerRun(main_);

    // Waits until all *threads* (not the main thread, because it already knows
    // it is here) called `WorkerReached`. All `barrier` types use spinning.

    barrier.UntilReached(num_threads, workers, spin, epoch_);

    // Threads may already be waiting `UntilWoken`, which serves as the
    // 'release' phase of the barrier.
  }

 private:
  Worker* const main_;
  const Tasks* const tasks_;
  uint32_t epoch_;
};

class WorkerAdapter {
 public:
  explicit WorkerAdapter(Worker* worker) : worker_(worker) {}

  void SetEpoch(uint32_t epoch) { epoch_ = epoch; }

  // Split into separate wait/barrier functions because `ThreadFunc` latches
  // the config in between them.
  template <class Spin, class Wait,
            HWY_IF_SAME(decltype(Wait().Type()), WaitType)>
  void operator()(const Spin& spin, const Wait& wait) const {
    wait.UntilWoken(worker_, spin, epoch_);
  }

  template <class Spin, class Barrier,
            HWY_IF_SAME(decltype(Barrier().Type()), BarrierType)>
  void operator()(const Spin& spin, const Barrier& barrier) const {
    barrier.WorkerReached(worker_, spin, epoch_);
  }

 private:
  Worker* const worker_;
  uint32_t epoch_;
};

// Could also be a lambda in ThreadPool ctor, but this allows annotating with
// `HWY_POOL_PROFILE` so we can more easily inspect the generated code.
class ThreadFunc {
 public:
  ThreadFunc(Worker* worker, Tasks* tasks, Config config)
      : worker_(worker),
        tasks_(tasks),
        config_(config),
        worker_adapter_(worker_) {
    worker->LatchConfig(config);
  }

  HWY_POOL_PROFILE void operator()() {
    SetThreadName("worker%03zu", static_cast<int>(worker_->Index()));

    // Ensure main thread's writes are visible (synchronizes with fence in
    // `WorkerLifecycle::Init`).
    std::atomic_thread_fence(std::memory_order_acquire);

    // Initialization must match pre-increment in `MainAdapter::SetEpoch`.
    // Loop termination is triggered by `~ThreadPool`.
    for (uint32_t epoch = 1;; ++epoch) {
      worker_adapter_.SetEpoch(epoch);
      CallWithSpinWait(config_, worker_adapter_);

      // Must happen before `WorkerRun` because `SendConfig` writes it there.
      config_ = worker_->LatchedConfig();

      tasks_->WorkerRun(worker_);

      // Notify barrier after `WorkerRun`.
      CallWithSpinBarrier(config_, worker_adapter_);

      // Check after notifying the barrier, otherwise the main thread deadlocks.
      if (HWY_UNLIKELY(config_.exit)) break;
    }
  }

 private:
  Worker* const worker_;
  Tasks* const tasks_;

  Config config_;
  WorkerAdapter worker_adapter_;
};

}  // namespace pool

// Highly efficient parallel-for, intended for workloads with thousands of
// fork-join regions which consist of calling tasks[t](i) for a few hundred i,
// using dozens of threads.
//
// To reduce scheduling overhead, we assume that tasks are statically known and
// that threads do not schedule new work themselves. This allows us to avoid
// queues and only store a counter plus the current task. The latter is a
// pointer to a lambda function, without the allocation/indirection required for
// std::function.
//
// To reduce fork/join latency, we choose an efficient barrier, optionally
// enable spin-waits via SetWaitMode, and avoid any mutex/lock. We largely even
// avoid atomic RMW operations (LOCK prefix): currently for the wait and
// barrier, in future hopefully also for work stealing.
//
// To eliminate false sharing and enable reasoning about cache line traffic, the
// class is aligned and holds all worker state.
//
// For load-balancing, we use work stealing in random order.
class alignas(HWY_ALIGNMENT) ThreadPool {
 public:
  // This typically includes hyperthreads, hence it is a loose upper bound.
  // -1 because these are in addition to the main thread.
  static size_t MaxThreads() {
    LogicalProcessorSet lps;
    // This is OS dependent, but more accurate if available because it takes
    // into account restrictions set by cgroups or numactl/taskset.
    if (GetThreadAffinity(lps)) {
      return lps.Count() - 1;
    }
    return static_cast<size_t>(std::thread::hardware_concurrency() - 1);
  }

  // `num_threads` is the number of *additional* threads to spawn, which should
  // not exceed `MaxThreads()`. Note that the main thread also performs work.
  explicit ThreadPool(size_t num_threads)
      : have_timer_stop_(platform::HaveTimerStop(cpu100_)),
        num_threads_(ClampedNumThreads(num_threads)),
        div_workers_(num_threads_ + 1),
        workers_(pool::WorkerLifecycle::Init(worker_bytes_, num_threads_,
                                             div_workers_)),
        main_adapter_(workers_ + num_threads_, &tasks_) {
    // Leaves the default wait mode as `kBlock`, which means futex, because
    // spinning only makes sense when threads are pinned and wake latency is
    // important, so it must explicitly be requested by calling `SetWaitMode`.
    for (PoolWaitMode mode : {PoolWaitMode::kSpin, PoolWaitMode::kBlock}) {
      wait_mode_ = mode;  // for AutoTuner
      AutoTuner().SetCandidates(
          pool::Config::AllCandidates(mode, num_threads_));
    }
    config_ = AutoTuner().Candidates()[0];

    threads_.reserve(num_threads_);
    for (size_t thread = 0; thread < num_threads_; ++thread) {
      threads_.emplace_back(
          pool::ThreadFunc(workers_ + thread, &tasks_, config_));
    }

    // No barrier is required here because wakeup works regardless of the
    // relative order of wake and wait.
  }

  // Waits for all threads to exit.
  ~ThreadPool() {
    // There is no portable way to request threads to exit like `ExitThread` on
    // Windows, otherwise we could call that from `Run`. Instead, we must cause
    // the thread to wake up and exit. We can use the same `SendConfig`
    // mechanism as `SetWaitMode`.
    pool::Config copy = config_;
    copy.exit = true;
    SendConfig(copy);

    for (std::thread& thread : threads_) {
      HWY_DASSERT(thread.joinable());
      thread.join();
    }

    pool::WorkerLifecycle::Destroy(workers_, num_threads_);
  }

  ThreadPool(const ThreadPool&) = delete;
  ThreadPool& operator&(const ThreadPool&) = delete;

  // Returns number of Worker, i.e., one more than the largest `worker`
  // argument. Useful for callers that want to allocate thread-local storage.
  size_t NumWorkers() const {
    return static_cast<size_t>(div_workers_.GetDivisor());
  }

  // `mode` defaults to `kBlock`, which means futex. Switching to `kSpin`
  // reduces fork-join overhead especially when there are many calls to `Run`,
  // but wastes power when waiting over long intervals. Must not be called
  // concurrently with any `Run`, because this uses the same waiter/barrier.
  void SetWaitMode(PoolWaitMode mode) {
    wait_mode_ = mode;
    SendConfig(AutoTuneComplete() ? *AutoTuner().Best()
                                  : AutoTuner().NextConfig());
  }

  // For printing which are in use.
  pool::Config config() const { return config_; }

  bool AutoTuneComplete() const { return AutoTuner().Best(); }
  Span<CostDistribution> AutoTuneCosts() { return AutoTuner().Costs(); }

  // parallel-for: Runs `closure(task, worker)` on workers for every `task` in
  // `[begin, end)`. Note that the unit of work should be large enough to
  // amortize the function call overhead, but small enough that each worker
  // processes a few tasks. Thus each `task` is usually a loop.
  //
  // Not thread-safe - concurrent parallel-for in the same `ThreadPool` are
  // forbidden unless `NumWorkers() == 1` or `end <= begin + 1`.
  template <class Closure>
  void Run(uint64_t begin, uint64_t end, const Closure& closure) {
    const size_t num_tasks = static_cast<size_t>(end - begin);
    const size_t num_workers = NumWorkers();

    // If zero or one task, or no extra threads, run on the main thread without
    // setting any member variables, because we may be re-entering Run.
    if (HWY_UNLIKELY(num_tasks <= 1 || num_workers == 1)) {
      for (uint64_t task = begin; task < end; ++task) {
        closure(task, /*worker=*/0);
      }
      return;
    }

    SetBusy();
    tasks_.Set(begin, end, closure);

    // More than one task per worker: use work stealing.
    if (HWY_LIKELY(num_tasks > num_workers)) {
      pool::Tasks::DivideRangeAmongWorkers(begin, end, div_workers_, workers_);
    }

    main_adapter_.SetEpoch(++epoch_);

    AutoTuneT& auto_tuner = AutoTuner();
    if (HWY_LIKELY(auto_tuner.Best())) {
      CallWithConfig(config_, main_adapter_);
      ClearBusy();
    } else {
      const uint64_t t0 = timer::Start();
      CallWithConfig(config_, main_adapter_);
      const uint64_t t1 = have_timer_stop_ ? timer::Stop() : timer::Start();
      auto_tuner.NotifyCost(t1 - t0);
      ClearBusy();              // before `SendConfig`
      if (auto_tuner.Best()) {  // just finished
        HWY_IF_CONSTEXPR(pool::kVerbosity >= 1) {
          const size_t idx_best = static_cast<size_t>(
              auto_tuner.Best() - auto_tuner.Candidates().data());
          HWY_DASSERT(idx_best < auto_tuner.Costs().size());
          auto& AT = auto_tuner.Costs()[idx_best];
          const double best_cost = AT.EstimateCost();
          HWY_DASSERT(best_cost > 0.0);  // will divide by this below

          Stats s_ratio;
          for (size_t i = 0; i < auto_tuner.Costs().size(); ++i) {
            if (i == idx_best) continue;
            const double cost = auto_tuner.Costs()[i].EstimateCost();
            s_ratio.Notify(static_cast<float>(cost / best_cost));
          }

          fprintf(stderr, "  %s %5.0f +/- %4.0f. Gain %.2fx [%.2fx, %.2fx]\n",
                  auto_tuner.Best()->ToString().c_str(), best_cost, AT.Stddev(),
                  s_ratio.GeometricMean(), s_ratio.Min(), s_ratio.Max());
        }
        SendConfig(*auto_tuner.Best());
      } else {
        HWY_IF_CONSTEXPR(pool::kVerbosity >= 2) {
          fprintf(stderr, "  %s %5lu\n", config_.ToString().c_str(), t1 - t0);
        }
        SendConfig(auto_tuner.NextConfig());
      }
    }
  }

  // Can pass this as init_closure when no initialization is needed.
  // DEPRECATED, better to call the Run() overload without the init_closure arg.
  static bool NoInit(size_t /*num_threads*/) { return true; }  // DEPRECATED

  // DEPRECATED equivalent of NumWorkers. Note that this is not the same as the
  // ctor argument because num_threads = 0 has the same effect as 1.
  size_t NumThreads() const { return NumWorkers(); }  // DEPRECATED

  // DEPRECATED prior interface with 32-bit tasks and first calling
  // `init_closure(num_threads)`. Instead, perform any init before this, calling
  // NumWorkers() for an upper bound on the worker index, then call the other
  // overload of Run().
  template <class InitClosure, class RunClosure>
  bool Run(uint64_t begin, uint64_t end, const InitClosure& init_closure,
           const RunClosure& run_closure) {
    if (!init_closure(NumThreads())) return false;
    Run(begin, end, run_closure);
    return true;
  }

 private:
  // Used to initialize ThreadPool::num_threads_ from its ctor argument.
  static size_t ClampedNumThreads(size_t num_threads) {
    // Upper bound is required for `worker_bytes_`.
    if (HWY_UNLIKELY(num_threads > pool::kMaxThreads)) {
      HWY_WARN("ThreadPool: clamping num_threads %zu to %zu.", num_threads,
               pool::kMaxThreads);
      num_threads = pool::kMaxThreads;
    }
    return num_threads;
  }

  // Debug-only re-entrancy detection.
  void SetBusy() { HWY_DASSERT(!busy_.test_and_set()); }
  void ClearBusy() { HWY_IF_CONSTEXPR(HWY_IS_DEBUG_BUILD) busy_.clear(); }

  // Two-phase barrier protocol for sending `copy` to workers, similar to the
  // 'quiescent state' used in RCU.
  //
  // Phase 1:
  // - Main wakes threads using the old config.
  // - Threads latch `copy` during `WorkerRun`.
  // - Threads notify a barrier and wait for the next wake using the old config.
  //
  // Phase 2:
  // - Main wakes threads still using the old config.
  // - Threads switch their config to their latched `copy`.
  // - Threads notify a barrier and wait, BOTH with the new config.
  // - Main thread switches to `copy` for the next wake.
  HWY_NOINLINE void SendConfig(pool::Config copy) {
    if (NumWorkers() == 1) {
      config_ = copy;
      return;
    }

    SetBusy();

    const auto closure = [this, copy](uint64_t task, size_t worker) {
      (void)task;
      HWY_DASSERT(task == worker);  // one task per worker
      workers_[worker].LatchConfig(copy);
    };
    tasks_.Set(0, NumWorkers(), closure);
    // Same config as workers are *currently* using.
    main_adapter_.SetEpoch(++epoch_);
    CallWithConfig(config_, main_adapter_);
    // All workers have latched `copy` and are waiting with the old config.

    // No-op task; will not be called because begin == end.
    tasks_.Set(0, 0, [](uint64_t /*task*/, size_t /*worker*/) {});
    // Threads are waiting using the old config, but will switch after waking,
    // which means we must already use the new barrier.
    pool::Config new_barrier = config_;
    new_barrier.barrier_type = copy.barrier_type;
    main_adapter_.SetEpoch(++epoch_);
    CallWithConfig(new_barrier, main_adapter_);
    // All have woken and are, or will be, waiting per the *new* config. Now we
    // can entirely switch the main thread's config for the next wake.
    config_ = copy;

    ClearBusy();
  }

  using AutoTuneT = AutoTune<pool::Config, 30>;
  AutoTuneT& AutoTuner() {
    static_assert(static_cast<size_t>(PoolWaitMode::kBlock) == 1, "");
    return auto_tune_[static_cast<size_t>(wait_mode_) - 1];
  }
  const AutoTuneT& AutoTuner() const {
    return auto_tune_[static_cast<size_t>(wait_mode_) - 1];
  }

  char cpu100_[100];
  const bool have_timer_stop_;
  const size_t num_threads_;  // not including main thread
  const Divisor64 div_workers_;
  pool::Worker* const workers_;  // points into `worker_bytes_`

  pool::MainAdapter main_adapter_;

  // The only mutable state:
  pool::Tasks tasks_;    // written by `Run` and read by workers.
  pool::Config config_;  // for use by the next `Run`. Updated via `SendConfig`.
  uint32_t epoch_ = 0;   // passed to `MainAdapter`.

  // In debug builds, detects if functions are re-entered.
  std::atomic_flag busy_ = ATOMIC_FLAG_INIT;

  // Unmodified after ctor, but cannot be const because we call thread::join().
  std::vector<std::thread> threads_;

  PoolWaitMode wait_mode_;
  AutoTuneT auto_tune_[2];  // accessed via `AutoTuner`

  // Last because it is large. Store inside `ThreadPool` so that callers can
  // bind it to the NUMA node's memory. Not stored inside `WorkerLifecycle`
  // because that class would be initialized after `workers_`.
  alignas(HWY_ALIGNMENT) uint8_t
      worker_bytes_[sizeof(pool::Worker) * (pool::kMaxThreads + 1)];
};

}  // namespace hwy

#endif  // HIGHWAY_HWY_CONTRIB_THREAD_POOL_THREAD_POOL_H_
