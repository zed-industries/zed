// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PROFILER_UNWINDER_H_
#define BASE_PROFILER_UNWINDER_H_

#include <vector>

#include "base/base_export.h"
#include "base/memory/raw_ptr.h"
#include "base/profiler/frame.h"
#include "base/profiler/module_cache.h"
#include "base/profiler/register_context.h"

namespace base {

// The result of attempting to unwind stack frames.
enum class UnwindResult {
  // The end of the stack was reached successfully.
  kCompleted,

  // The walk reached a frame that it doesn't know how to unwind, but might be
  // unwindable by the other native/aux unwinder.
  kUnrecognizedFrame,

  // The walk was aborted and is not resumable.
  kAborted,
};

// State information from stack capture. This may capture information from
// the `Unwinder` that is outside of the scope of the capture stack. This
// is allocated (and destroyed) per each stack capture and is provided
// to the Unwinder on each of its virtuals.
class BASE_EXPORT UnwinderStateCapture {
 public:
  virtual ~UnwinderStateCapture();
};

// Unwinder provides an interface for stack frame unwinder implementations for
// use with the StackSamplingProfiler. Initialize() must be invoked prior to the
// invocation of any other function on the interface. The profiler is expected
// to call CanUnwind() to determine if the Unwinder thinks it can unwind from
// the frame represented by the context values, then TryUnwind() to attempt the
// unwind.
class BASE_EXPORT Unwinder {
 public:
  virtual ~Unwinder() = default;

  // Initializes this unwinder to use |module_cache| in subsequent methods
  // UpdateModules() and TryUnwinder(). This unwinder may add any modules it
  // recognizes or register a module factory to the ModuleCache. |module_cache|
  // must outlive this Unwinder.
  void Initialize(ModuleCache* module_cache);

  // Invoked before the stack is captured. This can allocate any memory needed
  // to capture state in `OnStateCapture`. This is invoked on the stack sampling
  // thread.
  virtual std::unique_ptr<UnwinderStateCapture> CreateUnwinderStateCapture();

  // Invoked at the time the stack is captured. IMPORTANT NOTE: this function is
  // invoked while the target thread is suspended. To avoid deadlock it must not
  // invoke any non-reentrant code that is also invoked by the target thread. In
  // particular, it may not perform any heap allocation or deallocation,
  // including indirectly via use of DCHECK/CHECK or other logging statements.
  virtual void OnStackCapture(UnwinderStateCapture* capture_state) {}

  // Allows the unwinder to update ModuleCache with any modules it's responsible
  // for. Invoked for each sample between OnStackCapture() and the initial
  // invocations of CanUnwindFrom()/TryUnwind().
  virtual void UpdateModules(UnwinderStateCapture* capture_state) {}

  // Returns true if the unwinder recognizes the code referenced by
  // |current_frame| as code from which it should be able to unwind. When
  // multiple unwinders are in use, each should return true for a disjoint set
  // of frames. Note that if the unwinder returns true it may still legitmately
  // fail to unwind; e.g. in the case of a native unwind for a function that
  // doesn't have unwind information.
  virtual bool CanUnwindFrom(const Frame& current_frame) const = 0;

  // Attempts to unwind the frame represented by the context values.
  // Walks the native frames on the stack pointed to by the stack pointer in
  // `thread_context`, appending the frames to `stack`. When invoked
  // stack->back() contains the frame corresponding to the state in
  // `thread_context`.
  // `capture_state` was allocated in the call to `CreateUnwinderStateCapture`
  // Precondition: RegisterContextStackPointer(thread_context) is less than
  // `stack_top`.
  // Postcondition: If the implementation returns UNRECOGNIZED_FRAME, indicating
  // that it successfully unwound, RegisterContextStackPointer(thread_context)
  // is greater than the previous value and less than `stack_top`.
  virtual UnwindResult TryUnwind(UnwinderStateCapture* capture_state,
                                 RegisterContext* thread_context,
                                 uintptr_t stack_top,
                                 std::vector<Frame>* stack) = 0;

  Unwinder(const Unwinder&) = delete;
  Unwinder& operator=(const Unwinder&) = delete;

 protected:
  Unwinder() = default;

  // Invoked to allow the unwinder to add any modules it recognizes or register
  // a module factory to the ModuleCache.
  virtual void InitializeModules() {}

  ModuleCache* module_cache() const { return module_cache_; }

 private:
  raw_ptr<ModuleCache> module_cache_ = nullptr;
};

}  // namespace base

#endif  // BASE_PROFILER_UNWINDER_H_
