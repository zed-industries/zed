// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_BASE_DEBUG_ALIAS_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_BASE_DEBUG_ALIAS_H_

#include <cstddef>

#include "partition_alloc/partition_alloc_base/component_export.h"

namespace partition_alloc::internal::base::debug {

// Make the optimizer think that |var| is aliased. This can be used to inhibit
// three different kinds of optimizations:
//
// Case #1: Prevent a local variable from being optimized out if it would not
// otherwise be live at the point of a potential crash. This can only be done
// with local variables, not globals, object members, or function return values
// - these must be copied to locals if you want to ensure they are recorded in
// crash dumps. Function arguments are fine to use since the
// base::debug::Alias() call on them will make sure they are copied to the stack
// even if they were passed in a register. Note that if the local variable is a
// pointer then its value will be retained but the memory that it points to will
// probably not be saved in the crash dump - by default only stack memory is
// saved. Therefore the aliasing technique is usually only worthwhile with
// non-pointer variables. If you have a pointer to an object and you want to
// retain the object's state you need to copy the object or its fields to local
// variables.
//
// Example usage:
//   int last_error = err_;
//   base::debug::Alias(&last_error);
//   char name_copy[16];
//   strncpy(name_copy, p->name, sizeof(name_copy)-1);
//   name_copy[sizeof(name_copy)-1] = '\0';;
//   base::debug::alias(name_copy);
//   NOTREACHED();
//
// Case #2: Prevent a tail call into a function. This is useful to make sure the
// function containing the call to base::debug::Alias() will be present in the
// call stack. In this case there is no memory that needs to be on
// the stack so we can use nullptr. The call to base::debug::Alias() needs to
// happen after the call that is suspected to be tail called. Note: This
// technique will prevent tail calls at the specific call site only. To prevent
// them for all invocations of a function look at PA_NOT_TAIL_CALLED.
//
// Example usage:
//   PA_NOINLINE void Foo(){
//     ... code ...
//
//     Bar();
//     base::debug::Alias(nullptr);
//   }
//
// Case #3: Prevent code folding of a non-unique function. Code folding can
// cause the same address to be assigned to different functions if they are
// identical. If finding the precise signature of a function in the call-stack
// is important and it's suspected the function is identical to other functions
// it can be made unique using PA_NO_CODE_FOLDING which is a wrapper around
// base::debug::Alias();
//
// Example usage:
//   PA_NOINLINE void Foo(){
//     PA_NO_CODE_FOLDING();
//     Bar();
//   }
//
// Finally please note that these effects compound. This means that saving a
// stack variable (case #1) using base::debug::Alias() will also inhibit
// tail calls for calls in earlier lines and prevent code folding.

void PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE) Alias(const void* var);

}  // namespace partition_alloc::internal::base::debug

// Code folding is a linker optimization whereby the linker identifies functions
// that are bit-identical and overlays them. This saves space but it leads to
// confusing call stacks because multiple symbols are at the same address and
// it is unpredictable which one will be displayed. Disabling of code folding is
// particularly useful when function names are used as signatures in crashes.
// This macro doesn't guarantee that code folding will be prevented but it
// greatly reduces the odds and always prevents it within one source file.
// If using in a function that terminates the process it is safest to put the
// PA_NO_CODE_FOLDING macro at the top of the function.
// Use like:
//   void FooBarFailure(size_t size) { PA_NO_CODE_FOLDING(); OOM_CRASH(size); }
#define PA_NO_CODE_FOLDING()        \
  const int line_number = __LINE__; \
  ::partition_alloc::internal::base::debug::Alias(&line_number)

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_BASE_DEBUG_ALIAS_H_
