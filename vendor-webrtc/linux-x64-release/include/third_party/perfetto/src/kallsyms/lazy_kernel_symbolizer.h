/*
 * Copyright (C) 2020 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_KALLSYMS_LAZY_KERNEL_SYMBOLIZER_H_
#define SRC_KALLSYMS_LAZY_KERNEL_SYMBOLIZER_H_

#include <memory>

#include "perfetto/ext/base/thread_checker.h"

namespace perfetto {

class KernelSymbolMap;

// This class is a wrapper around KernelSymbolMap. It serves two purposes:
// 1. Deals with /proc/kallsyms reads and temporary lowering of kptr_restrict.
//    KernelSymbolMap is just a parser and doesn't do I/O.
// 2. Allows to share the same KernelSymbolMap instance across several clients
//    and tear it down when tracing stops.
//
// LazyKernelSymbolizer is owned by the (one) FtraceController. FtraceController
// handles LazyKernelSymbolizer pointers to  N CpuReader-s (one per CPU). In
// this way all CpuReader instances can share the same symbol map instance.
// The object being shared is LazyKernelSymbolizer, which is cheap and always
// valid. LazyKernelSymbolizer may or may not contain a valid symbol map.
class LazyKernelSymbolizer {
 public:
  // Constructs an empty instance. Does NOT load any symbols upon construction.
  // Loading and parsing happens on the first GetOrCreateKernelSymbolMap() call.
  LazyKernelSymbolizer();
  ~LazyKernelSymbolizer();

  // Returns |instance_|, creating it if doesn't exist or was destroyed.
  KernelSymbolMap* GetOrCreateKernelSymbolMap();

  bool is_valid() const { return !!symbol_map_; }

  // Destroys the |symbol_map_| freeing up memory. A further call to
  // GetOrCreateKernelSymbolMap() will create it again.
  void Destroy();

  // Exposed for testing.
  static bool CanReadKernelSymbolAddresses(
      const char* ksyms_path_for_testing = nullptr);

 private:
  std::unique_ptr<KernelSymbolMap> symbol_map_;
  PERFETTO_THREAD_CHECKER(thread_checker_)
};

}  // namespace perfetto

#endif  // SRC_KALLSYMS_LAZY_KERNEL_SYMBOLIZER_H_
