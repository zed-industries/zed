// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PROFILER_STACK_SAMPLER_H_
#define BASE_PROFILER_STACK_SAMPLER_H_

#include <memory>
#include <vector>

#include "base/base_export.h"
#include "base/containers/circular_deque.h"
#include "base/functional/callback.h"
#include "base/gtest_prod_util.h"
#include "base/memory/raw_ptr.h"
#include "base/profiler/frame.h"
#include "base/profiler/register_context.h"
#include "base/profiler/sampling_profiler_thread_token.h"
#include "base/profiler/stack_copier.h"
#include "base/profiler/stack_unwind_data.h"
#include "base/threading/platform_thread.h"
#include "build/build_config.h"

namespace base {

class ModuleCache;
class StackBuffer;
class StackSamplerTestDelegate;
class Unwinder;

// StackSampler is an implementation detail of StackSamplingProfiler. It
// abstracts the native implementation required to record a set of stack frames
// for a given thread. It delegates to StackCopier for the
// platform-specific stack copying implementation.
// This class is used on both the SamplingThread and a worker thread of the
// thread pool. Recording stack frames always occurs on the
// SamplingThread but unwinding the stack can occur on either the SamplingThread
// or a worker thread. Sampling can start before the thread pool is running so
// unwinding will occur on the SamplingThread until the thread pool is ready.
class BASE_EXPORT StackSampler {
 public:
  // Factory for generating a set of Unwinders for use by the profiler.
  using UnwindersFactory =
      OnceCallback<std::vector<std::unique_ptr<Unwinder>>()>;

  // Creates a stack sampler that records samples for thread with
  // |thread_token|. Unwinders in |unwinders| must be stored in increasing
  // priority to guide unwind attempts. Only the unwinder with the lowest
  // priority is allowed to return with UnwindResult::kCompleted. Returns null
  // if this platform does not support stack sampling.
  static std::unique_ptr<StackSampler> Create(
      SamplingProfilerThreadToken thread_token,
      std::unique_ptr<StackUnwindData> stack_unwind_data,
      UnwindersFactory core_unwinders_factory,
      RepeatingClosure record_sample_callback,
      StackSamplerTestDelegate* test_delegate);

  ~StackSampler();

  StackSampler(const StackSampler&) = delete;
  StackSampler& operator=(const StackSampler&) = delete;

  // Gets the required size of the stack buffer.
  static size_t GetStackBufferSize();

  // Creates an instance of the a stack buffer that can be used for calls to
  // any StackSampler object.
  static std::unique_ptr<StackBuffer> CreateStackBuffer();

  // The following functions are all called on the SamplingThread (not the
  // thread being sampled).

  // Performs post-construction initialization on the SamplingThread.
  void Initialize();

  // Stops the sampler.
  void Stop(OnceClosure done_callback);

  // Adds an auxiliary unwinder to handle additional, non-native-code unwind
  // scenarios. Unwinders must be inserted in increasing priority, following
  // |unwinders| provided in Create(), to guide unwind attempts.
  void AddAuxUnwinder(std::unique_ptr<Unwinder> unwinder);

  // Records a set of frames and returns them.
  void RecordStackFrames(StackBuffer* stack_buffer,
                         PlatformThreadId thread_id,
                         OnceClosure done_callback);

  StackUnwindData* GetStackUnwindData();

  // Exposes the internal function for unit testing.
  static std::vector<Frame> WalkStackForTesting(
      ModuleCache* module_cache,
      RegisterContext* thread_context,
      uintptr_t stack_top,
      std::vector<UnwinderCapture> unwinders);

  // Create a StackSampler, overriding the platform-specific components.
  static std::unique_ptr<StackSampler> CreateForTesting(
      std::unique_ptr<StackCopier> stack_copier,
      std::unique_ptr<StackUnwindData> stack_unwind_data,
      UnwindersFactory core_unwinders_factory,
      RepeatingClosure record_sample_callback = RepeatingClosure(),
      StackSamplerTestDelegate* test_delegate = nullptr);

#if BUILDFLAG(IS_CHROMEOS)
  // How often to record the "Memory.StackSamplingProfiler.StackSampleSize2" UMA
  // histogram. Specifically, only 1 in kUMAHistogramDownsampleAmount calls to
  // RecordStackFrames will add a sample to the histogram. RecordStackFrames is
  // called many times a second. We don't need multiple samples per second to
  // get a good understanding of average stack sizes, and it's a lot of data to
  // record. kUMAHistogramDownsampleAmount should give us about 1 sample per 10
  // seconds per process, which is plenty. 199 is prime which should avoid any
  // aliasing issues (e.g. if stacks are larger on second boundaries or some
  // such weirdness).
  static constexpr uint32_t kUMAHistogramDownsampleAmount = 199;
#endif

 private:
  FRIEND_TEST_ALL_PREFIXES(StackSamplerTest,
                           AuxUnwinderInvokedWhileRecordingStackFrames);

  StackSampler(std::unique_ptr<StackCopier> stack_copier,
               std::unique_ptr<StackUnwindData> stack_unwind_data,
               UnwindersFactory core_unwinders_factory,
               RepeatingClosure record_sample_callback,
               StackSamplerTestDelegate* test_delegate);

  static std::vector<Frame> WalkStack(ModuleCache* module_cache,
                                      RegisterContext* thread_context,
                                      uintptr_t stack_top,
                                      std::vector<UnwinderCapture> unwinders);

  void UnwindComplete(TimeTicks timestamp,
                      OnceClosure done_callback,
                      std::vector<Frame> frames);
  void AddAuxUnwinderWithoutInit(std::unique_ptr<Unwinder> unwinder);
  void ThreadPoolRunning();

  const std::unique_ptr<StackCopier> stack_copier_;
  UnwindersFactory unwinders_factory_;

  const RepeatingClosure record_sample_callback_;
  const raw_ptr<StackSamplerTestDelegate> test_delegate_;

#if BUILDFLAG(IS_CHROMEOS)
  // Counter for "Memory.StackSamplingProfiler.StackSampleSize2" UMA histogram.
  // See comments above kUMAHistogramDownsampleAmount. Unsigned so that overflow
  // isn't undefined behavior.
  uint32_t stack_size_histogram_sampling_counter_ = 0;
#endif

  scoped_refptr<SequencedTaskRunner> thread_pool_runner_;

  // The StackUnwindData will be in released on the `thread_pool_runner_` if it
  // is non-null.
  std::unique_ptr<StackUnwindData> unwind_data_;

  bool was_initialized_ = false;
  bool thread_pool_ready_ = false;

  base::WeakPtrFactory<StackSampler> weak_ptr_factory_{this};
};

// StackSamplerTestDelegate provides seams for test code to execute during stack
// collection.
class BASE_EXPORT StackSamplerTestDelegate {
 public:
  StackSamplerTestDelegate(const StackSamplerTestDelegate&) = delete;
  StackSamplerTestDelegate& operator=(const StackSamplerTestDelegate&) = delete;

  virtual ~StackSamplerTestDelegate();

  // Called after copying the stack and resuming the target thread, but prior to
  // walking the stack. Invoked on the SamplingThread.
  virtual void OnPreStackWalk() = 0;

 protected:
  StackSamplerTestDelegate();
};

}  // namespace base

#endif  // BASE_PROFILER_STACK_SAMPLER_H_
