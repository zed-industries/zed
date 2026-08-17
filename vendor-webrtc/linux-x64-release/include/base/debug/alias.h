// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_DEBUG_ALIAS_H_
#define BASE_DEBUG_ALIAS_H_

#include <stddef.h>

#include "base/base_export.h"
#include "base/compiler_specific.h"

namespace base {
namespace debug {

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
//   DEBUG_ALIAS_FOR_CSTR(name_copy, p->name, 16);
//   NOTREACHED();
//
// Case #2: Prevent a tail call into a function. This is useful to make sure the
// function containing the call to base::debug::Alias() will be present in the
// call stack. In this case there is no memory that needs to be on
// the stack so we can use nullptr. The call to base::debug::Alias() needs to
// happen after the call that is suspected to be tail called. Note: This
// technique will prevent tail calls at the specific call site only. To prevent
// them for all invocations of a function look at NOT_TAIL_CALLED.
//
// Example usage:
//   NOINLINE void Foo(){
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
// it can be made unique using NO_CODE_FOLDING which is a wrapper around
// base::debug::Alias();
//
// Example usage:
//   NOINLINE void Foo(){
//     NO_CODE_FOLDING();
//     Bar();
//   }
//
// Finally please note that these effects compound. This means that saving a
// stack variable (case #1) using base::debug::Alias() will also inhibit
// tail calls for calls in earlier lines and prevent code folding.

void BASE_EXPORT Alias(const void* var);

}  // namespace debug

// The canonical definitions/declarations for `strlcpy()`, `u16cstrlcpy()`,
// and `wcslcpy()` are in //base/strings/string_util.{cc,h}. These prototypes
// are forward declared here to avoid having to include string_utils.h and its
// transitive tree of headers in an otherwise small header (which is itself
// included in some very popular headers).
BASE_EXPORT size_t strlcpy(char* dst, const char* src, size_t dst_size);
BASE_EXPORT size_t u16cstrlcpy(char16_t* dst,
                               const char16_t* src,
                               size_t dst_size);
BASE_EXPORT size_t wcslcpy(wchar_t* dst, const wchar_t* src, size_t dst_size);

}  // namespace base

// Convenience macro that copies the null-terminated string from `c_str` into a
// stack-allocated char array named `var_name` that holds up to `array_size - 1`
// characters and should be preserved in memory dumps.
#define DEBUG_ALIAS_FOR_CSTR(var_name, c_str, array_size)               \
  char var_name[array_size] = {};                                       \
  UNSAFE_TODO(::base::strlcpy(var_name, (c_str), std::size(var_name))); \
  ::base::debug::Alias(var_name)

#define DEBUG_ALIAS_FOR_U16CSTR(var_name, c_str, array_size)                \
  char16_t var_name[array_size] = {};                                       \
  UNSAFE_TODO(::base::u16cstrlcpy(var_name, (c_str), std::size(var_name))); \
  ::base::debug::Alias(var_name)

#define DEBUG_ALIAS_FOR_WCHARCSTR(var_name, c_str, array_size)          \
  wchar_t var_name[array_size] = {};                                    \
  UNSAFE_TODO(::base::wcslcpy(var_name, (c_str), std::size(var_name))); \
  ::base::debug::Alias(var_name)

// Code folding is a linker optimization whereby the linker identifies functions
// that are bit-identical and overlays them. This saves space but it leads to
// confusing call stacks because multiple symbols are at the same address and
// it is unpredictable which one will be displayed. Disabling of code folding is
// particularly useful when function names are used as signatures in crashes.
// This macro doesn't guarantee that code folding will be prevented but it
// greatly reduces the odds and always prevents it within one source file.
// If using in a function that terminates the process it is safest to put the
// NO_CODE_FOLDING macro at the top of the function.
// Use like:
//   void FooBarFailure(size_t size) { NO_CODE_FOLDING(); OOM_CRASH(size); }
#define NO_CODE_FOLDING()           \
  const int line_number = __LINE__; \
  base::debug::Alias(&line_number)

#endif  // BASE_DEBUG_ALIAS_H_
