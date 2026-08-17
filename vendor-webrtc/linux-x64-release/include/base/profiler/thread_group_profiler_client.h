// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PROFILER_THREAD_GROUP_PROFILER_CLIENT_H_
#define BASE_PROFILER_THREAD_GROUP_PROFILER_CLIENT_H_

#include <memory>

#include "base/base_export.h"
#include "base/functional/callback.h"
#include "base/profiler/stack_sampling_profiler.h"

namespace base {

class CommandLine;
class ProfileBuilder;

// Interface for controlling thread group profiling behavior.
// This interface is designed to be implemented by embedders who will
// configure profiling behavior for worker threads in a thread pool.
class BASE_EXPORT ThreadGroupProfilerClient {
 public:
  ThreadGroupProfilerClient() = default;
  ThreadGroupProfilerClient(const ThreadGroupProfilerClient&) = delete;
  ThreadGroupProfilerClient& operator=(const ThreadGroupProfilerClient&) =
      delete;
  virtual ~ThreadGroupProfilerClient() = default;

  // Returns the sampling parameters for a new profiler instance.
  virtual StackSamplingProfiler::SamplingParams GetSamplingParams() = 0;

  // Creates a ProfileBuilder for recording profile data.
  virtual std::unique_ptr<ProfileBuilder> CreateProfileBuilder(
      OnceClosure builder_completed_callback) = 0;

  // Returns a factory function for creating unwinders.
  virtual base::StackSamplingProfiler::UnwindersFactory
  GetUnwindersFactory() = 0;

  // Determines whether profiling is enabled for the current process.
  virtual bool IsProfilerEnabledForCurrentProcess() = 0;

  // Checks if the embedder is in single-process mode based on the command line.
  virtual bool IsSingleProcess(const base::CommandLine& command_line) = 0;
};

}  // namespace base

#endif  // BASE_PROFILER_THREAD_GROUP_PROFILER_CLIENT_H_
