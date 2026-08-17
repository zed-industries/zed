//===-- Core Structures for strftime ----------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_STDIO_STRFTIME_CORE_CORE_STRUCTS_H
#define LLVM_LIBC_SRC_STDIO_STRFTIME_CORE_CORE_STRUCTS_H

#include "hdr/types/struct_tm.h"
#include "src/__support/CPP/string_view.h"

#include <stdint.h>

namespace LIBC_NAMESPACE_DECL {
namespace strftime_core {

enum class ConvModifier { none, E, O };

// These flags intentionally have different values from the ones used by printf.
// They have different meanings.
enum FormatFlags : uint8_t {
  FORCE_SIGN = 0x01,     // +
  LEADING_ZEROES = 0x02, // 0
  // TODO: look into the glibc extension flags ('_', '-', '^', and '#')
};

struct FormatSection {
  bool has_conv = false;
  cpp::string_view raw_string = {};

  FormatFlags flags = FormatFlags(0);
  ConvModifier modifier = ConvModifier::none;
  char conv_name = '\0';
  int min_width = 0;
};

// TODO: Move this to a better spot
#define RET_IF_RESULT_NEGATIVE(func)                                           \
  {                                                                            \
    int result = (func);                                                       \
    if (result < 0)                                                            \
      return result;                                                           \
  }

constexpr int WRITE_OK = 0;

} // namespace strftime_core
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_STDIO_STRFTIME_CORE_CORE_STRUCTS_H
