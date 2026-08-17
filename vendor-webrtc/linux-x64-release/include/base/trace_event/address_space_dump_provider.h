// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TRACE_EVENT_ADDRESS_SPACE_DUMP_PROVIDER_H_
#define BASE_TRACE_EVENT_ADDRESS_SPACE_DUMP_PROVIDER_H_

#include "base/base_export.h"
#include "base/memory/raw_ptr.h"
#include "base/trace_event/memory_dump_provider.h"
#include "partition_alloc/address_space_stats.h"

namespace base::trace_event {

// Collects PartitionAlloc address space metrics.
class BASE_EXPORT AddressSpaceDumpProvider : public MemoryDumpProvider {
 public:
  AddressSpaceDumpProvider(const AddressSpaceDumpProvider&) = delete;
  AddressSpaceDumpProvider& operator=(const AddressSpaceDumpProvider&) = delete;

  AddressSpaceDumpProvider();
  ~AddressSpaceDumpProvider() override;

  static AddressSpaceDumpProvider* GetInstance();

  // MemoryDumpProvider implementation.
  bool OnMemoryDump(const MemoryDumpArgs& args,
                    ProcessMemoryDump* pmd) override;
};

}  // namespace base::trace_event

#endif  // BASE_TRACE_EVENT_ADDRESS_SPACE_DUMP_PROVIDER_H_
