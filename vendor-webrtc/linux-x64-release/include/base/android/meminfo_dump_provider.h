// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_MEMINFO_DUMP_PROVIDER_H_
#define BASE_ANDROID_MEMINFO_DUMP_PROVIDER_H_

#include "base/base_export.h"
#include "base/no_destructor.h"
#include "base/time/time.h"
#include "base/trace_event/base_tracing.h"

namespace base::android {

class BASE_EXPORT MeminfoDumpProvider
    : public base::trace_event::MemoryDumpProvider {
 public:
  // Returns the instance for testing.
  static MeminfoDumpProvider& Initialize();
  bool OnMemoryDump(const base::trace_event::MemoryDumpArgs& args,
                    base::trace_event::ProcessMemoryDump* pmd) override;

  static constexpr char kDumpProviderName[] = "android_meminfo";
  static constexpr char kDumpName[] = "meminfo";
  static constexpr char kIsStaleName[] = "is_stale";
  static constexpr char kPssMetricName[] = "other_pss";
  static constexpr char kPrivateDirtyMetricName[] = "other_private_dirty";

 private:
  friend class base::NoDestructor<MeminfoDumpProvider>;
  MeminfoDumpProvider();

  base::TimeTicks last_collection_time_;
};

}  // namespace base::android

#endif  // BASE_ANDROID_MEMINFO_DUMP_PROVIDER_H_
