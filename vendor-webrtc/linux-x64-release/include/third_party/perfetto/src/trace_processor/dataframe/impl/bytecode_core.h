/*
 * Copyright (C) 2025 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_DATAFRAME_IMPL_BYTECODE_CORE_H_
#define SRC_TRACE_PROCESSOR_DATAFRAME_IMPL_BYTECODE_CORE_H_

#include <array>
#include <cstddef>
#include <cstdint>
#include <string>
#include <string_view>
#include <tuple>
#include <type_traits>
#include <utility>
#include <variant>
#include <vector>

#include "perfetto/base/logging.h"
#include "perfetto/ext/base/small_vector.h"
#include "perfetto/ext/base/string_utils.h"
#include "perfetto/public/compiler.h"
#include "src/trace_processor/dataframe/impl/bytecode_registers.h"
#include "src/trace_processor/dataframe/impl/types.h"
#include "src/trace_processor/dataframe/specs.h"

namespace perfetto::trace_processor::dataframe::impl::bytecode {

// Base bytecode structure representing a single instruction with operation
// code and fixed-size buffer for arguments.
struct Bytecode {
  uint32_t option;                      // Opcode determining instruction type
  std::array<uint8_t, 28> args_buffer;  // Storage for instruction arguments

 protected:
  // Helper for generating the offsets array for instruction arguments.
  template <typename T>
  static constexpr auto MakeOffsetsArray() {
    constexpr auto kOffsets = MakeOffsetsArrayImpl<T>(
        std::make_index_sequence<std::tuple_size_v<T>>());
    static_assert(kOffsets[std::tuple_size_v<T>] <= sizeof(args_buffer));
    return kOffsets;
  }
  template <typename T, size_t... I>
  static constexpr auto MakeOffsetsArrayImpl(std::index_sequence<I...>) {
    std::array<uint32_t, std::tuple_size_v<T> + 1> offsets{};
    ((offsets[I + 1] = offsets[I] + sizeof(std::tuple_element_t<I, T>)), ...);
    // Ensure that all the types have aligned offsets.
    ((offsets[I + 1] % alignof(std::tuple_element_t<I, T>) == 0
          ? void()
          : PERFETTO_ELOG("Type index %zu is not aligned to %zu", I,
                          alignof(std::tuple_element_t<I, T>))),
     ...);
    return offsets;
  }
};
static_assert(std::is_trivially_copyable_v<Bytecode>);
static_assert(sizeof(Bytecode) <= 32);

// Indicates that the bytecode has a fixed cost.
struct FixedCost {
  double cost;
};

// Indicates that the bytecode has `cost` multiplied by `log2(estimated row
// count)`.
struct LogPerRowCost {
  double cost;
};

// Indicates that the bytecode has `cost` multiplied by `estimated row count`.
struct LinearPerRowCost {
  double cost;
};

// Indicates that the bytecode has `cost` multiplied by `log2(estimated row
// count) * estimated row count`.
struct LogLinearPerRowCost {
  double cost;
};

// Indicates that the bytecode has `cost` multiplied by the `estimated row
// count` *after* the operation completes (as opposed to `LinearPerRowCost`
// which is *before* the operation completes).
struct PostOperationLinearPerRowCost {
  double cost;
};

// A variant used to specify the cost of a bytecode operation.
using Cost = std::variant<FixedCost,
                          LogPerRowCost,
                          LinearPerRowCost,
                          LogLinearPerRowCost,
                          PostOperationLinearPerRowCost>;

// Bytecode with one template parameter for dispatching.
template <typename TypeSet1>
struct TemplatedBytecode1 : Bytecode {
  using TS1 = TypeSet1;
  PERFETTO_ALWAYS_INLINE static constexpr uint32_t OpcodeOffset(const TS1& ts) {
    return ts.index();
  }
};

// Bytecode with two template parameters for dispatching.
template <typename TypeSet1, typename TypeSet2>
struct TemplatedBytecode2 : Bytecode {
  using TS1 = TypeSet1;
  using TS2 = TypeSet2;
  PERFETTO_ALWAYS_INLINE static constexpr uint32_t OpcodeOffset(
      const TS1& ts1,
      const TS2& ts2) {
    return (ts1.index() * TS2::kSize) + ts2.index();
  }
};

// Vector type for storing sequences of bytecode instructions.
using BytecodeVector = base::SmallVector<Bytecode, 16>;

// String conversion utilities for bytecode arguments.
PERFETTO_NO_INLINE inline base::StackString<32> ArgToString(uint32_t value) {
  return base::StackString<32>("%u", value);
}

PERFETTO_NO_INLINE inline base::StackString<64> ArgToString(
    const reg::HandleBase& value) {
  return base::StackString<64>("Register(%u)", value.index);
}

PERFETTO_NO_INLINE inline base::StackString<64> ArgToString(NonNullOp value) {
  return base::StackString<64>("NonNullOp(%u)", value.index());
}

PERFETTO_NO_INLINE inline base::StackString<64> ArgToString(
    FilterValueHandle value) {
  return base::StackString<64>("FilterValue(%u)", value.index);
}

PERFETTO_NO_INLINE inline base::StackString<64> ArgToString(
    impl::BoundModifier bound) {
  return base::StackString<64>("BoundModifier(%u)", bound.index());
}

PERFETTO_NO_INLINE inline base::StackString<64> ArgToString(
    SortDirection direction) {
  return base::StackString<64>("SortDirection(%u)",
                               static_cast<uint32_t>(direction));
}

PERFETTO_NO_INLINE inline base::StackString<64> ArgToString(
    NullsLocation location) {
  return base::StackString<64>("NullsLocation(%u)", location.index());
}

PERFETTO_NO_INLINE inline void BytecodeFieldToString(
    std::string_view name,
    const char* value,
    std::vector<std::string>& fields) {
  if (name.compare(0, 3, "pad") == 0) {
    return;
  }
  base::StackString<64> str("%.*s=%s", int(name.size()), name.data(), value);
  fields.push_back(str.ToStdString());
}

PERFETTO_NO_INLINE inline std::string BytecodeFieldsFormat(
    const std::vector<std::string>& fields) {
  std::string res;
  res.append("[");
  res.append(base::Join(fields, ", "));
  res.append("]");
  return res;
}

// Macro to define bytecode instruction with 5 fields.
#define PERFETTO_DATAFRAME_BYTECODE_IMPL_7(t1, n1, t2, n2, t3, n3, t4, n4, t5, \
                                           n5, t6, n6, t7, n7)                 \
  enum Field : uint8_t { n1 = 0, n2, n3, n4, n5, n6, n7 };                     \
  using tuple = std::tuple<t1, t2, t3, t4, t5, t6, t7>;                        \
  static constexpr auto kOffsets = MakeOffsetsArray<tuple>();                  \
  static constexpr auto kNames =                                               \
      std::array{#n1, #n2, #n3, #n4, #n5, #n6, #n7};                           \
                                                                               \
  template <Field N>                                                           \
  const auto& arg() const {                                                    \
    return *reinterpret_cast<const std::tuple_element_t<N, tuple>*>(           \
        args_buffer.data() + kOffsets[N]);                                     \
  }                                                                            \
  template <Field N>                                                           \
  auto& arg() {                                                                \
    return *reinterpret_cast<std::tuple_element_t<N, tuple>*>(                 \
        args_buffer.data() + kOffsets[N]);                                     \
  }                                                                            \
  std::string ToString() const {                                               \
    std::vector<std::string> fields;                                           \
    BytecodeFieldToString(#n1, ArgToString(arg<n1>()).c_str(), fields);        \
    BytecodeFieldToString(#n2, ArgToString(arg<n2>()).c_str(), fields);        \
    BytecodeFieldToString(#n3, ArgToString(arg<n3>()).c_str(), fields);        \
    BytecodeFieldToString(#n4, ArgToString(arg<n4>()).c_str(), fields);        \
    BytecodeFieldToString(#n5, ArgToString(arg<n5>()).c_str(), fields);        \
    BytecodeFieldToString(#n6, ArgToString(arg<n6>()).c_str(), fields);        \
    BytecodeFieldToString(#n7, ArgToString(arg<n7>()).c_str(), fields);        \
    return BytecodeFieldsFormat(fields);                                       \
  }                                                                            \
  static void UnusedForWarningSuppresssion()

// Simplified macros that add padding fields automatically.
#define PERFETTO_DATAFRAME_BYTECODE_IMPL_6(t1, n1, t2, n2, t3, n3, t4, n4, t5, \
                                           n5, t6, n6)                         \
  PERFETTO_DATAFRAME_BYTECODE_IMPL_7(t1, n1, t2, n2, t3, n3, t4, n4, t5, n5,   \
                                     t6, n6, uint32_t, pad7)

#define PERFETTO_DATAFRAME_BYTECODE_IMPL_5(t1, n1, t2, n2, t3, n3, t4, n4, t5, \
                                           n5)                                 \
  PERFETTO_DATAFRAME_BYTECODE_IMPL_6(t1, n1, t2, n2, t3, n3, t4, n4, t5, n5,   \
                                     uint32_t, pad6)

#define PERFETTO_DATAFRAME_BYTECODE_IMPL_4(t1, n1, t2, n2, t3, n3, t4, n4)     \
  PERFETTO_DATAFRAME_BYTECODE_IMPL_5(t1, n1, t2, n2, t3, n3, t4, n4, uint32_t, \
                                     pad5)

#define PERFETTO_DATAFRAME_BYTECODE_IMPL_3(t1, n1, t2, n2, t3, n3) \
  PERFETTO_DATAFRAME_BYTECODE_IMPL_4(t1, n1, t2, n2, t3, n3, uint32_t, pad4)

#define PERFETTO_DATAFRAME_BYTECODE_IMPL_2(t1, n1, t2, n2) \
  PERFETTO_DATAFRAME_BYTECODE_IMPL_3(t1, n1, t2, n2, uint32_t, pad3)

#define PERFETTO_DATAFRAME_BYTECODE_IMPL_1(t1, n1) \
  PERFETTO_DATAFRAME_BYTECODE_IMPL_2(t1, n1, uint32_t, pad2)

}  // namespace perfetto::trace_processor::dataframe::impl::bytecode

#endif  // SRC_TRACE_PROCESSOR_DATAFRAME_IMPL_BYTECODE_CORE_H_
