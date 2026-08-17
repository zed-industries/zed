// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PROFILER_STACK_COPIER_H_
#define BASE_PROFILER_STACK_COPIER_H_

#include <stdint.h>

#include <vector>

#include "base/base_export.h"
#include "base/profiler/register_context.h"
#include "base/time/time.h"

namespace base {

class StackBuffer;

// StackCopier causes a thread to be suspended, copies its stack, and resumes
// the thread's execution. It's intended to provide an abstraction over stack
// copying techniques where the thread suspension is performed directly by the
// profiler thread (Windows and Mac platforms) vs. where the thread suspension
// is performed by the OS through signals (Android).
class BASE_EXPORT StackCopier {
 public:
  // Interface that may be implemented by the caller of CopyStack() to receive a
  // callback when the stack is copied, while the target thread is suspended.
  class BASE_EXPORT Delegate {
   public:
    virtual ~Delegate() = default;

    // Invoked at the time the stack is copied.
    // IMPORTANT NOTE: to avoid deadlock implementations of this interface must
    // not invoke any non-reentrant code that is also invoked by the target
    // thread. In particular, it may not perform any heap allocation or
    // deallocation, including indirectly via use of DCHECK/CHECK or other
    // logging statements.
    virtual void OnStackCopy() = 0;
  };

  virtual ~StackCopier();

  // Copies the thread's register context into |thread_context|, the stack into
  // |stack_buffer|, and the top of stack address into |stack_top|. Records
  // |timestamp| at the time the stack was copied. delegate->OnStackCopy() will
  // be invoked while the thread is suspended. Returns true if successful.
  virtual bool CopyStack(StackBuffer* stack_buffer,
                         uintptr_t* stack_top,
                         TimeTicks* timestamp,
                         RegisterContext* thread_context,
                         Delegate* delegate) = 0;

  // Creates a copy of StackBuffer so that it can be passed to another thread.
  // `stack_top` as passed in will match stack_top in `stack_buffer` but
  // will be modified to match the returned StackBuffer. Similarly any
  // registers in `thread_context` will be manipulated so they are relative to
  // the result of this function.
  std::unique_ptr<StackBuffer> CloneStack(const StackBuffer& stack_buffer,
                                          uintptr_t* stack_top,
                                          RegisterContext* thread_context);

 protected:
  virtual std::vector<uintptr_t*> GetRegistersToRewrite(
      RegisterContext* thread_context) = 0;

  // If the value at |pointer| points to the original stack, rewrite it to point
  // to the corresponding location in the copied stack.
  //
  // NO HEAP ALLOCATIONS.
  static uintptr_t RewritePointerIfInOriginalStack(
      const uint8_t* original_stack_bottom,
      const uintptr_t* original_stack_top,
      const uint8_t* stack_copy_bottom,
      uintptr_t pointer);

  // Copies the stack to a buffer while rewriting possible pointers to locations
  // within the stack to point to the corresponding locations in the copy. This
  // is necessary to handle stack frames with dynamic stack allocation, where a
  // pointer to the beginning of the dynamic allocation area is stored on the
  // stack and/or in a non-volatile register.
  //
  // Eager rewriting of anything that looks like a pointer to the stack, as done
  // in this function, does not adversely affect the stack unwinding. The only
  // other values on the stack the unwinding depends on are return addresses,
  // which should not point within the stack memory. The rewriting is guaranteed
  // to catch all pointers because the stacks are guaranteed by the ABI to be
  // sizeof(uintptr_t*) aligned.
  //
  // |original_stack_bottom| and |original_stack_top| are different pointer
  // types due on their differing guaranteed alignments -- the bottom may only
  // be 1-byte aligned while the top is aligned to double the pointer width.
  //
  // Returns a pointer to the bottom address in the copied stack. This value
  // matches the alignment of |original_stack_bottom| to ensure that the stack
  // contents have the same alignment as in the original stack. As a result the
  // value will be different than |stack_buffer_bottom| if
  // |original_stack_bottom| is not aligned to double the pointer width.
  //
  // NO HEAP ALLOCATIONS.
  static const uint8_t* CopyStackContentsAndRewritePointers(
      const uint8_t* original_stack_bottom,
      const uintptr_t* original_stack_top,
      size_t platform_stack_alignment,
      uintptr_t* stack_buffer_bottom);
};

}  // namespace base

#endif  // BASE_PROFILER_STACK_COPIER_H_
