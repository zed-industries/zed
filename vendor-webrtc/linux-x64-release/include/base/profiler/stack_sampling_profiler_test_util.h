// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PROFILER_STACK_SAMPLING_PROFILER_TEST_UTIL_H_
#define BASE_PROFILER_STACK_SAMPLING_PROFILER_TEST_UTIL_H_

#include <memory>
#include <string>
#include <string_view>
#include <vector>

#include "base/base_export.h"
#include "base/functional/callback.h"
#include "base/memory/raw_ptr.h"
#include "base/native_library.h"
#include "base/profiler/frame.h"
#include "base/profiler/module_cache.h"
#include "base/profiler/sampling_profiler_thread_token.h"
#include "base/profiler/stack_sampling_profiler.h"
#include "base/synchronization/waitable_event.h"
#include "base/threading/platform_thread.h"

namespace base {

class Unwinder;

// A thread to target for profiling that will run the supplied closure.
class TargetThread : public PlatformThread::Delegate {
 public:
  explicit TargetThread(OnceClosure to_run);

  TargetThread(const TargetThread&) = delete;
  TargetThread& operator=(const TargetThread&) = delete;

  ~TargetThread() override;

  void Start();
  void Join();

  // PlatformThread::Delegate:
  void ThreadMain() override;

  SamplingProfilerThreadToken thread_token() const { return thread_token_; }

 private:
  SamplingProfilerThreadToken thread_token_ = {kInvalidThreadId};
  OnceClosure to_run_;
  PlatformThreadHandle target_thread_handle_;
};

// Addresses near the start and end of a function.
struct FunctionAddressRange {
  raw_ptr<const void> start;
  raw_ptr<const void> end;
};

// Represents a stack unwind scenario to be sampled by the
// StackSamplingProfiler.
class UnwindScenario {
 public:
  // A callback provided by the caller that sets up the unwind scenario, then
  // calls into the passed closure to wait for a sample to be taken. Returns the
  // address range of the function that sets up the unwind scenario. The passed
  // closure will be null when invoked solely to obtain the address range.
  using SetupFunction = RepeatingCallback<FunctionAddressRange(OnceClosure)>;

  // Events to coordinate the sampling.
  struct SampleEvents {
    WaitableEvent ready_for_sample;
    WaitableEvent sample_finished;
  };

  explicit UnwindScenario(const SetupFunction& setup_function);
  ~UnwindScenario();

  UnwindScenario(const UnwindScenario&) = delete;
  UnwindScenario& operator=(const UnwindScenario&) = delete;

  // The address range of the innermost function that waits for the sample.
  FunctionAddressRange GetWaitForSampleAddressRange() const;

  // The address range of the provided setup function.
  FunctionAddressRange GetSetupFunctionAddressRange() const;

  // The address range of the outer function that indirectly invokes the setup
  // function.
  FunctionAddressRange GetOuterFunctionAddressRange() const;

  // Executes the scenario.
  void Execute(SampleEvents* events);

 private:
  static FunctionAddressRange InvokeSetupFunction(
      const SetupFunction& setup_function,
      SampleEvents* events);

  static FunctionAddressRange WaitForSample(SampleEvents* events);

  const SetupFunction setup_function_;
};

class TestModule : public ModuleCache::Module {
 public:
  explicit TestModule(uintptr_t base_address = 0,
                      size_t size = 0,
                      bool is_native = true)
      : base_address_(base_address), size_(size), is_native_(is_native) {}

  uintptr_t GetBaseAddress() const override;
  std::string GetId() const override;
  FilePath GetDebugBasename() const override;
  size_t GetSize() const override;
  bool IsNative() const override;

  void set_id(const std::string& id) { id_ = id; }
  void set_debug_basename(const FilePath& basename) {
    debug_basename_ = basename;
  }

 private:
  const uintptr_t base_address_;
  const size_t size_;
  const bool is_native_;
  std::string id_;
  FilePath debug_basename_;
};

bool operator==(const Frame& a, const Frame& b);

// UnwindScenario setup function that calls into |wait_for_sample| without doing
// any special unwinding setup, to exercise the "normal" unwind scenario.
FunctionAddressRange CallWithPlainFunction(OnceClosure wait_for_sample);

// Calls into |wait_for_sample| after using alloca(), to test unwinding with a
// frame pointer.
FunctionAddressRange CallWithAlloca(OnceClosure wait_for_sample);

// Calls into |wait_for_sample| through a function within another library, to
// test unwinding through multiple modules and scenarios involving unloaded
// modules.
FunctionAddressRange CallThroughOtherLibrary(NativeLibrary library,
                                             OnceClosure wait_for_sample);

// The callback to perform profiling on the provided thread.
using ProfileCallback = OnceCallback<void(SamplingProfilerThreadToken)>;

// Executes |profile_callback| while running |scenario| on the target
// thread. Performs all necessary target thread startup and shutdown work before
// and afterward.
void WithTargetThread(UnwindScenario* scenario,
                      ProfileCallback profile_callback);

using UnwinderFactory = OnceCallback<std::unique_ptr<Unwinder>()>;

// Returns the sample seen when taking one sample of |scenario|.
std::vector<Frame> SampleScenario(
    UnwindScenario* scenario,
    ModuleCache* module_cache,
    UnwinderFactory aux_unwinder_factory = UnwinderFactory());

// Formats a sample into a string that can be output for test diagnostics.
std::string FormatSampleForDiagnosticOutput(const std::vector<Frame>& sample);

// Expects that the stack contains the functions with the specified address
// ranges, in the specified order.
void ExpectStackContains(const std::vector<Frame>& stack,
                         const std::vector<FunctionAddressRange>& functions);

// Expects that the stack does not contain the functions with the specified
// address ranges.
void ExpectStackDoesNotContain(
    const std::vector<Frame>& stack,
    const std::vector<FunctionAddressRange>& functions);

// Load test library with given name.
NativeLibrary LoadTestLibrary(std::string_view library_name);

// Loads the other library, which defines a function to be called in the
// WITH_OTHER_LIBRARY configuration.
NativeLibrary LoadOtherLibrary();

uintptr_t GetAddressInOtherLibrary(NativeLibrary library);

// Creates a list of core unwinders required for StackSamplingProfilerTest.
// This is useful notably on Android, which requires ChromeUnwinderAndroid in
// addition to the native one.
StackSamplingProfiler::UnwindersFactory CreateCoreUnwindersFactoryForTesting(
    ModuleCache* module_cache);

}  // namespace base

#endif  // BASE_PROFILER_STACK_SAMPLING_PROFILER_TEST_UTIL_H_
