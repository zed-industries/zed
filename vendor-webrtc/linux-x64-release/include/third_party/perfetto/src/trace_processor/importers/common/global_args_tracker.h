/*
 * Copyright (C) 2019 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_GLOBAL_ARGS_TRACKER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_GLOBAL_ARGS_TRACKER_H_

#include <cstdint>
#include <type_traits>
#include <vector>
#include "perfetto/base/logging.h"
#include "perfetto/ext/base/flat_hash_map.h"
#include "perfetto/ext/base/hash.h"
#include "perfetto/ext/base/small_vector.h"
#include "src/trace_processor/db/column.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/tables/metadata_tables_py.h"
#include "src/trace_processor/types/variadic.h"

namespace perfetto {
namespace trace_processor {

// Interns args into the storage from all ArgsTrackers across trace processor.
// Note: most users will want to use ArgsTracker to push args to the strorage
// and not this class. This class is really intended for ArgsTracker to use for
// that purpose.
class GlobalArgsTracker {
 public:
  // How to behave if two or more args with the same key were added into the
  // same ArgSet. If |kSkipIfExists|, the arg will be ignored if another arg
  // with the same key already exists. If |kAddOrUpdate|, any existing arg with
  // the same key will be overridden.
  enum class UpdatePolicy { kSkipIfExists, kAddOrUpdate };

  struct CompactArg {
    StringId flat_key = kNullStringId;
    StringId key = kNullStringId;
    Variadic value = Variadic::Integer(0);
    UpdatePolicy update_policy = UpdatePolicy::kAddOrUpdate;
  };
  static_assert(std::is_trivially_destructible<CompactArg>::value,
                "Args must be trivially destructible");

  struct Arg : public CompactArg {
    ColumnLegacy* column;
    uint32_t row;

    // Object slices this Arg to become a CompactArg.
    CompactArg ToCompactArg() const { return CompactArg(*this); }
  };
  static_assert(std::is_trivially_destructible<Arg>::value,
                "Args must be trivially destructible");

  struct ArgHasher {
    uint64_t operator()(const CompactArg& arg) const noexcept {
      base::Hasher hash;
      hash.Update(arg.key.raw_id());
      // We don't hash arg.flat_key because it's a subsequence of arg.key.
      switch (arg.value.type) {
        case Variadic::Type::kInt:
          hash.Update(arg.value.int_value);
          break;
        case Variadic::Type::kUint:
          hash.Update(arg.value.uint_value);
          break;
        case Variadic::Type::kString:
          hash.Update(arg.value.string_value.raw_id());
          break;
        case Variadic::Type::kReal:
          hash.Update(arg.value.real_value);
          break;
        case Variadic::Type::kPointer:
          hash.Update(arg.value.pointer_value);
          break;
        case Variadic::Type::kBool:
          hash.Update(arg.value.bool_value);
          break;
        case Variadic::Type::kJson:
          hash.Update(arg.value.json_value.raw_id());
          break;
        case Variadic::Type::kNull:
          hash.Update(0);
          break;
      }
      return hash.digest();
    }
  };

  explicit GlobalArgsTracker(TraceStorage* storage);

  ArgSetId AddArgSet(const std::vector<Arg>& args,
                     uint32_t begin,
                     uint32_t end) {
    return AddArgSet(args.data() + begin, args.data() + end, sizeof(Arg));
  }
  ArgSetId AddArgSet(Arg* args, uint32_t begin, uint32_t end) {
    return AddArgSet(args + begin, args + end, sizeof(Arg));
  }
  ArgSetId AddArgSet(CompactArg* args, uint32_t begin, uint32_t end) {
    return AddArgSet(args + begin, args + end, sizeof(CompactArg));
  }

 private:
  using ArgSetHash = uint64_t;

  // Assumes that the interval [begin, end) of |args| has args with the same key
  // grouped together.
  ArgSetId AddArgSet(const void* start, const void* end, uint32_t stride) {
    base::SmallVector<const CompactArg*, 64> valid;

    // TODO(eseckler): Also detect "invalid" key combinations in args sets (e.g.
    // "foo" and "foo.bar" in the same arg set)?
    for (const void* ptr = start; ptr != end;
         ptr = reinterpret_cast<const uint8_t*>(ptr) + stride) {
      const auto& arg = *reinterpret_cast<const CompactArg*>(ptr);
      if (!valid.empty() && valid.back()->key == arg.key) {
        // Last arg had the same key as this one. In case of kSkipIfExists, skip
        // this arg. In case of kAddOrUpdate, remove the last arg and add this
        // arg instead.
        if (arg.update_policy == UpdatePolicy::kSkipIfExists) {
          continue;
        }
        PERFETTO_DCHECK(arg.update_policy == UpdatePolicy::kAddOrUpdate);
        valid.pop_back();
      }
      valid.emplace_back(&arg);
    }

    base::Hasher hash;
    for (const auto* it : valid) {
      hash.Update(ArgHasher()(*it));
    }

    auto& arg_table = *storage_->mutable_arg_table();

    uint32_t arg_set_id = arg_table.row_count();
    ArgSetHash digest = hash.digest();
    auto [it, inserted] = arg_row_for_hash_.Insert(digest, arg_set_id);
    if (!inserted) {
      // Already inserted.
      return *it;
    }

    for (const CompactArg* ptr : valid) {
      const auto& arg = *ptr;
      tables::ArgTable::Row row;
      row.arg_set_id = arg_set_id;
      row.flat_key = arg.flat_key;
      row.key = arg.key;
      switch (arg.value.type) {
        case Variadic::Type::kInt:
          row.int_value = arg.value.int_value;
          break;
        case Variadic::Type::kUint:
          row.int_value = static_cast<int64_t>(arg.value.uint_value);
          break;
        case Variadic::Type::kString:
          row.string_value = arg.value.string_value;
          break;
        case Variadic::Type::kReal:
          row.real_value = arg.value.real_value;
          break;
        case Variadic::Type::kPointer:
          row.int_value = static_cast<int64_t>(arg.value.pointer_value);
          break;
        case Variadic::Type::kBool:
          row.int_value = arg.value.bool_value;
          break;
        case Variadic::Type::kJson:
          row.string_value = arg.value.json_value;
          break;
        case Variadic::Type::kNull:
          break;
      }
      row.value_type = storage_->GetIdForVariadicType(arg.value.type);
      arg_table.Insert(row);
    }
    return arg_set_id;
  }

  base::FlatHashMap<ArgSetHash, uint32_t, base::AlreadyHashed<ArgSetHash>>
      arg_row_for_hash_;

  TraceStorage* storage_;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_GLOBAL_ARGS_TRACKER_H_
