// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_PROFILER_FRAME_H_
#define BASE_PROFILER_FRAME_H_

#include "base/base_export.h"
#include "base/memory/raw_ptr.h"
#include "base/profiler/module_cache.h"

namespace base {

// Frame represents an individual sampled stack frame with full module
// information.
struct BASE_EXPORT Frame {
  Frame(uintptr_t instruction_pointer, const ModuleCache::Module* module);

  // The function name should only be populated by Android Java frames.
  Frame(uintptr_t instruction_pointer,
        const ModuleCache::Module* module,
        std::string function_name);
  ~Frame();

  // The sampled instruction pointer within the function.
  uintptr_t instruction_pointer;

  // The module information.
  raw_ptr<const ModuleCache::Module, DanglingUntriaged> module;

  // Function name associated with the frame. Currently populated only for
  // Android Java frames.
  std::string function_name;
};

}  // namespace base

#endif  // BASE_PROFILER_FRAME_H_
