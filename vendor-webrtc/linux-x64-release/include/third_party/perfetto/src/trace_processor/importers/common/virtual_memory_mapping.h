
/*
 * Copyright (C) 2024 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_VIRTUAL_MEMORY_MAPPING_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_VIRTUAL_MEMORY_MAPPING_H_

#include <cstddef>
#include <cstdint>
#include <optional>
#include <string>
#include <vector>

#include "perfetto/ext/base/flat_hash_map.h"
#include "perfetto/ext/base/hash.h"
#include "perfetto/ext/base/string_view.h"
#include "src/trace_processor/importers/common/address_range.h"
#include "src/trace_processor/importers/common/create_mapping_params.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/types/trace_processor_context.h"
#include "src/trace_processor/util/build_id.h"

namespace perfetto {
namespace trace_processor {

// TODO(carlscab): Reconsider whether jit is the best abstraction here. All we
// really care is about mapping a `rel_pc` to a symbol (aka symbolization) and
// whether is this is constant.
class JitCache;

// Represents a mapping in virtual memory.
class VirtualMemoryMapping {
 public:
  virtual ~VirtualMemoryMapping();
  // Range of virtual memory this mapping covers.
  AddressRange memory_range() const { return memory_range_; }
  MappingId mapping_id() const { return mapping_id_; }
  // This name could be the path of the underlying file mapped into memory.
  const std::string& name() const { return name_; }
  // For file mappings, this is the offset into the file for the first byte in
  // the mapping
  uint64_t offset() const { return offset_; }
  // If the mapped file is an executable or shared library this will return the
  // load bias, if known. Returns 0 otherwise.
  uint64_t load_bias() const { return load_bias_; }
  // If the mapped file is an executable or shared library this will return its
  // build id, if known.
  const std::optional<BuildId>& build_id() const { return build_id_; }

  // Whether this maps to a region that holds jitted code.
  bool is_jitted() const { return jit_cache_ != nullptr; }

  // Converts an absolute address into a relative one.
  uint64_t ToRelativePc(uint64_t address) const {
    return address - memory_range_.start() + offset_ + load_bias_;
  }

  // Converts a relative address to an absolute one.
  uint64_t ToAddress(uint64_t rel_pc) const {
    return rel_pc + (memory_range_.start() - offset_ - load_bias_);
  }

  // Creates a frame for the given `rel_pc`. Note that if the mapping
  // `is_jitted()` same `rel_pc` values can return different mappings (as jitted
  // functions can be created and deleted over time.) So for such mappings the
  // returned `FrameId` should not be cached.
  FrameId InternFrame(uint64_t rel_pc, base::StringView function_name);

  // Returns all frames ever created in this mapping for the given `rel_pc`.
  std::vector<FrameId> FindFrameIds(uint64_t rel_pc) const;

 protected:
  VirtualMemoryMapping(TraceProcessorContext* context,
                       CreateMappingParams params);

  TraceProcessorContext* context() const { return context_; }

 private:
  friend class MappingTracker;

  std::pair<FrameId, bool> InternFrameImpl(uint64_t rel_pc,
                                           base::StringView function_name);

  void SetJitCache(JitCache* jit_cache) { jit_cache_ = jit_cache; }

  TraceProcessorContext* const context_;
  const MappingId mapping_id_;
  const AddressRange memory_range_;
  const uint64_t offset_;
  const uint64_t load_bias_;
  const std::string name_;
  std::optional<BuildId> const build_id_;
  JitCache* jit_cache_ = nullptr;

  struct FrameKey {
    uint64_t rel_pc;
    // It doesn't seem to make too much sense to key on name, as for the same
    // mapping and same rel_pc the name should always be the same. But who knows
    // how producers behave.
    StringId name_id;

    bool operator==(const FrameKey& o) const {
      return rel_pc == o.rel_pc && name_id == o.name_id;
    }

    struct Hasher {
      size_t operator()(const FrameKey& k) const {
        return static_cast<size_t>(
            base::Hasher::Combine(k.rel_pc, k.name_id.raw_id()));
      }
    };
  };
  base::FlatHashMap<FrameKey, FrameId, FrameKey::Hasher> interned_frames_;
  base::FlatHashMap<uint64_t, std::vector<FrameId>> frames_by_rel_pc_;
};

class KernelMemoryMapping : public VirtualMemoryMapping {
 public:
  ~KernelMemoryMapping() override;

 private:
  friend class MappingTracker;
  KernelMemoryMapping(TraceProcessorContext* context,
                      CreateMappingParams params);
};

class UserMemoryMapping : public VirtualMemoryMapping {
 public:
  ~UserMemoryMapping() override;
  UniquePid upid() const { return upid_; }

 private:
  friend class MappingTracker;
  UserMemoryMapping(TraceProcessorContext* context,
                    UniquePid upid,
                    CreateMappingParams params);

  const UniquePid upid_;
};

// Dummy mapping to be able to create frames when we have no real pc addresses
// or real mappings.
class DummyMemoryMapping : public VirtualMemoryMapping {
 public:
  ~DummyMemoryMapping() override;

  // Interns a frame based solely on function name and source file. This is
  // useful for profilers that do not emit an address nor a mapping.
  FrameId InternDummyFrame(base::StringView function_name,
                           base::StringView source_file);

 private:
  friend class MappingTracker;
  DummyMemoryMapping(TraceProcessorContext* context,
                     CreateMappingParams params);

  struct DummyFrameKey {
    StringId function_name_id;
    StringId source_file_id;

    bool operator==(const DummyFrameKey& o) const {
      return function_name_id == o.function_name_id &&
             source_file_id == o.source_file_id;
    }

    struct Hasher {
      size_t operator()(const DummyFrameKey& k) const {
        return static_cast<size_t>(base::Hasher::Combine(
            k.function_name_id.raw_id(), k.source_file_id.raw_id()));
      }
    };
  };
  base::FlatHashMap<DummyFrameKey, FrameId, DummyFrameKey::Hasher>
      interned_dummy_frames_;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_VIRTUAL_MEMORY_MAPPING_H_
