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

#ifndef SRC_TRACE_PROCESSOR_DATAFRAME_IMPL_BYTECODE_REGISTERS_H_
#define SRC_TRACE_PROCESSOR_DATAFRAME_IMPL_BYTECODE_REGISTERS_H_

#include <cstdint>
#include <limits>
#include <variant>

#include "src/trace_processor/dataframe/impl/slab.h"
#include "src/trace_processor/dataframe/impl/types.h"

namespace perfetto::trace_processor::dataframe::impl::bytecode::reg {

// Set an upper bound on registers to allow for using std::array to
// store register values.
// Arbitrary value chosen to be larger than any reasonable bytecode program.
static constexpr uint32_t kMaxRegisters = 64;

// Register system for the bytecode interpreter.
// Provides typed handles for accessing virtual registers with appropriate
// read/write permissions.

// Base class for all register handle types with common index field.
struct HandleBase {
  uint32_t index = std::numeric_limits<uint32_t>::max();
};

// Handle for read-write registers of type T.
template <typename T>
struct RwHandle : HandleBase {
  RwHandle() = default;
  explicit RwHandle(uint32_t _index) : HandleBase{_index} {}

#if !PERFETTO_BUILDFLAG(PERFETTO_COMPILER_MSVC)
  static constexpr bool VerifyPreconditions() {
    static_assert(std::is_trivial_v<RwHandle<T>>);
    static_assert(sizeof(RwHandle<T>) == sizeof(uint32_t));
    return true;
  }
  static constexpr bool kPreconditions = VerifyPreconditions();
#endif
};

// Handle for read-only registers of type T.
template <typename T>
struct ReadHandle : HandleBase {
#if !PERFETTO_BUILDFLAG(PERFETTO_COMPILER_MSVC)
  static constexpr bool VerifyPreconditions() {
    static_assert(std::is_trivial_v<ReadHandle<T>>);
    static_assert(sizeof(ReadHandle<T>) == sizeof(uint32_t));
    return true;
  }
  static constexpr bool kPreconditions = VerifyPreconditions();
#endif
  ReadHandle() = default;
  ReadHandle(RwHandle<T> _index) : HandleBase{_index.index} {}
  explicit ReadHandle(uint32_t _index) : HandleBase{_index} {}
};

// Handle for write-only registers of type T.
template <typename T>
struct WriteHandle : HandleBase {
#if !PERFETTO_BUILDFLAG(PERFETTO_COMPILER_MSVC)
  static constexpr bool VerifyPreconditions() {
    static_assert(std::is_trivial_v<WriteHandle<T>>);
    static_assert(sizeof(WriteHandle<T>) == sizeof(uint32_t));
    return true;
  }
  static constexpr bool kPreconditions = VerifyPreconditions();
#endif
  WriteHandle() = default;
  WriteHandle(RwHandle<T> _index) : HandleBase{_index.index} {}
  explicit WriteHandle(uint32_t _index) : HandleBase{_index} {}
};

// Empty placeholder type for register values.
struct Empty {};

// Values that can be stored in a register.
using Value = std::variant<Empty,
                           Range,
                           Slab<uint32_t>,
                           Span<uint32_t>,
                           CastFilterValueResult,
                           Slab<uint8_t>>;

}  // namespace perfetto::trace_processor::dataframe::impl::bytecode::reg

#endif  // SRC_TRACE_PROCESSOR_DATAFRAME_IMPL_BYTECODE_REGISTERS_H_
