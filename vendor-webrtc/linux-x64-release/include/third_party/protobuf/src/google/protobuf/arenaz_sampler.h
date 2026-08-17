// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_SRC_GOOGLE_PROTOBUF_ARENAZ_SAMPLER_H__
#define GOOGLE_PROTOBUF_SRC_GOOGLE_PROTOBUF_ARENAZ_SAMPLER_H__

#include <array>
#include <atomic>
#include <cstddef>
#include <cstdint>
#include <utility>


// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
namespace internal {

#if defined(PROTOBUF_ARENAZ_SAMPLE)
struct ThreadSafeArenaStats;
void RecordAllocateSlow(ThreadSafeArenaStats* info, size_t used,
                        size_t allocated, size_t wasted);
// Stores information about a sampled thread safe arena.  All mutations to this
// *must* be made through `Record*` functions below.  All reads from this *must*
// only occur in the callback to `ThreadSafeArenazSampler::Iterate`.
struct ThreadSafeArenaStats
    : public absl::profiling_internal::Sample<ThreadSafeArenaStats> {
  // Constructs the object but does not fill in any fields.
  ThreadSafeArenaStats();
  ~ThreadSafeArenaStats();

  // Puts the object into a clean state, fills in the logically `const` members,
  // blocking for any readers that are currently sampling the object.  The
  // 'stride' parameter is the number of ThreadSafeArenas that were instantiated
  // between this sample and the previous one.
  void PrepareForSampling(int64_t stride)
      ABSL_EXCLUSIVE_LOCKS_REQUIRED(init_mu);

  // These fields are mutated by the various Record* APIs and need to be
  // thread-safe.
  struct BlockStats {
    std::atomic<int> num_allocations;
    std::atomic<size_t> bytes_allocated;
    std::atomic<size_t> bytes_used;
    std::atomic<size_t> bytes_wasted;

    void PrepareForSampling();
  };

  // block_histogram is a kBlockHistogramBins sized histogram.  The zeroth bin
  // stores info about blocks of size \in [1, 1 << kLogMaxSizeForBinZero]. Bin
  // i, where i > 0, stores info for blocks of size \in (max_size_bin (i-1),
  // 1 << (kLogMaxSizeForBinZero + i)].  The final bin stores info about blocks
  // of size \in [kMaxSizeForPenultimateBin + 1,
  // std::numeric_limits<size_t>::max()].
  static constexpr size_t kBlockHistogramBins = 15;
  static constexpr size_t kLogMaxSizeForBinZero = 7;
  static constexpr size_t kMaxSizeForBinZero = (1 << kLogMaxSizeForBinZero);
  static constexpr size_t kMaxSizeForPenultimateBin =
      1 << (kLogMaxSizeForBinZero + kBlockHistogramBins - 2);
  std::array<BlockStats, kBlockHistogramBins> block_histogram;

  // Records the largest block allocated for the arena.
  std::atomic<size_t> max_block_size;
  // Bit `i` is set to 1 indicates that a thread with `tid % 63 = i` accessed
  // the underlying arena.  We use `% 63` as a rudimentary hash to ensure some
  // bit mixing for thread-ids; `% 64` would only grab the low bits and might
  // create sampling artifacts.
  std::atomic<uint64_t> thread_ids;

  // All of the fields below are set by `PrepareForSampling`, they must not
  // be mutated in `Record*` functions.  They are logically `const` in that
  // sense. These are guarded by init_mu, but that is not externalized to
  // clients, who can only read them during
  // `ThreadSafeArenazSampler::Iterate` which will hold the lock.
  static constexpr int kMaxStackDepth = 64;
  int32_t depth;
  void* stack[kMaxStackDepth];
  static void RecordAllocateStats(ThreadSafeArenaStats* info, size_t used,
                                  size_t allocated, size_t wasted) {
    if (ABSL_PREDICT_TRUE(info == nullptr)) return;
    RecordAllocateSlow(info, used, allocated, wasted);
  }

  // Returns the bin for the provided size.
  static size_t FindBin(size_t bytes);

  // Returns the min and max bytes that can be stored in the histogram for
  // blocks in the provided bin.
  static std::pair<size_t, size_t> MinMaxBlockSizeForBin(size_t bin);
};

struct SamplingState {
  // Number of ThreadSafeArenas that should be instantiated before the next
  // ThreadSafeArena is sampled.  This variable is decremented with each
  // instantiation.
  int64_t next_sample;
  // When we make a sampling decision, we record that distance between from the
  // previous sample so we can weight each sample.  'distance' here is the
  // number of instantiations of ThreadSafeArena.
  int64_t sample_stride;
};

ThreadSafeArenaStats* SampleSlow(SamplingState& sampling_state);
void UnsampleSlow(ThreadSafeArenaStats* info);

class ThreadSafeArenaStatsHandle {
 public:
  explicit ThreadSafeArenaStatsHandle() = default;
  explicit ThreadSafeArenaStatsHandle(ThreadSafeArenaStats* info)
      : info_(info) {}

  ~ThreadSafeArenaStatsHandle() {
    if (ABSL_PREDICT_TRUE(info_ == nullptr)) return;
    UnsampleSlow(info_);
  }

  ThreadSafeArenaStatsHandle(ThreadSafeArenaStatsHandle&& other) noexcept
      : info_(std::exchange(other.info_, nullptr)) {}

  ThreadSafeArenaStatsHandle& operator=(
      ThreadSafeArenaStatsHandle&& other) noexcept {
    if (ABSL_PREDICT_FALSE(info_ != nullptr)) {
      UnsampleSlow(info_);
    }
    info_ = std::exchange(other.info_, nullptr);
    return *this;
  }

  ThreadSafeArenaStats* MutableStats() { return info_; }

  friend void swap(ThreadSafeArenaStatsHandle& lhs,
                   ThreadSafeArenaStatsHandle& rhs) {
    std::swap(lhs.info_, rhs.info_);
  }

  friend class ThreadSafeArenaStatsHandlePeer;

 private:
  ThreadSafeArenaStats* info_ = nullptr;
};

using ThreadSafeArenazSampler =
    ::absl::profiling_internal::SampleRecorder<ThreadSafeArenaStats>;

extern PROTOBUF_THREAD_LOCAL SamplingState global_sampling_state;

// Returns an RAII sampling handle that manages registration and unregistation
// with the global sampler.
inline ThreadSafeArenaStatsHandle Sample() {
  if (ABSL_PREDICT_TRUE(--global_sampling_state.next_sample > 0)) {
    return ThreadSafeArenaStatsHandle(nullptr);
  }
  return ThreadSafeArenaStatsHandle(SampleSlow(global_sampling_state));
}

#else

using SamplingState = int64_t;

struct ThreadSafeArenaStats {
  static void RecordAllocateStats(ThreadSafeArenaStats*, size_t /*requested*/,
                                  size_t /*allocated*/, size_t /*wasted*/) {}
};

ThreadSafeArenaStats* SampleSlow(SamplingState& next_sample);
void UnsampleSlow(ThreadSafeArenaStats* info);

class ThreadSafeArenaStatsHandle {
 public:
  explicit ThreadSafeArenaStatsHandle() = default;
  explicit ThreadSafeArenaStatsHandle(ThreadSafeArenaStats*) {}

  void RecordReset() {}

  ThreadSafeArenaStats* MutableStats() { return nullptr; }

  friend void swap(ThreadSafeArenaStatsHandle&, ThreadSafeArenaStatsHandle&) {}

 private:
  friend class ThreadSafeArenaStatsHandlePeer;
};

class ThreadSafeArenazSampler {
 public:
  void Unregister(ThreadSafeArenaStats*) {}
  void SetMaxSamples(int32_t) {}
};

// Returns an RAII sampling handle that manages registration and unregistation
// with the global sampler.
inline ThreadSafeArenaStatsHandle Sample() {
  return ThreadSafeArenaStatsHandle(nullptr);
}
#endif  // defined(PROTOBUF_ARENAZ_SAMPLE)

// Returns a global Sampler.
ThreadSafeArenazSampler& GlobalThreadSafeArenazSampler();

using ThreadSafeArenazConfigListener = void (*)();
void SetThreadSafeArenazConfigListener(ThreadSafeArenazConfigListener l);

// Enables or disables sampling for thread safe arenas.
void SetThreadSafeArenazEnabled(bool enabled);
void SetThreadSafeArenazEnabledInternal(bool enabled);

// Returns true if sampling is on, false otherwise.
bool IsThreadSafeArenazEnabled();

// Sets the rate at which thread safe arena will be sampled.
void SetThreadSafeArenazSampleParameter(int32_t rate);
void SetThreadSafeArenazSampleParameterInternal(int32_t rate);

// Returns the rate at which thread safe arena will be sampled.
int32_t ThreadSafeArenazSampleParameter();

// Sets a soft max for the number of samples that will be kept.
void SetThreadSafeArenazMaxSamples(int32_t max);
void SetThreadSafeArenazMaxSamplesInternal(int32_t max);

// Returns the max number of samples that will be kept.
size_t ThreadSafeArenazMaxSamples();

// Sets the current value for when arenas should be next sampled.
void SetThreadSafeArenazGlobalNextSample(int64_t next_sample);

}  // namespace internal
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"
#endif  // GOOGLE_PROTOBUF_SRC_PROTOBUF_ARENAZ_SAMPLER_H__
