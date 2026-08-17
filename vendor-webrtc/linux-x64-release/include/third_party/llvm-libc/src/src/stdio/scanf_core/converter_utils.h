//===-- Format specifier converter for scanf -------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_STDIO_SCANF_CORE_CONVERTER_UTILS_H
#define LLVM_LIBC_SRC_STDIO_SCANF_CORE_CONVERTER_UTILS_H

#include "src/__support/ctype_utils.h"
#include "src/__support/macros/config.h"
#include "src/__support/str_to_float.h"
#include "src/stdio/scanf_core/core_structs.h"

#include <stddef.h>

namespace LIBC_NAMESPACE_DECL {
namespace scanf_core {

LIBC_INLINE void write_int_with_length(uintmax_t output_val,
                                       const FormatSection &to_conv) {
  if ((to_conv.flags & NO_WRITE) != 0) {
    return;
  }
  void *output_ptr = to_conv.output_ptr;
  // The %p conversion uses this function, and is always void*.
  if (to_conv.conv_name == 'p') {
    *reinterpret_cast<void **>(output_ptr) =
        reinterpret_cast<void *>(output_val);
    return;
  }
  LengthModifier lm = to_conv.length_modifier;
  switch (lm) {
  case (LengthModifier::hh):
    *reinterpret_cast<unsigned char *>(output_ptr) =
        static_cast<unsigned char>(output_val);
    break;
  case (LengthModifier::h):
    *reinterpret_cast<unsigned short *>(output_ptr) =
        static_cast<unsigned short>(output_val);
    break;
  case (LengthModifier::NONE):
    *reinterpret_cast<unsigned int *>(output_ptr) =
        static_cast<unsigned int>(output_val);
    break;
  case (LengthModifier::l):
    *reinterpret_cast<unsigned long *>(output_ptr) =
        static_cast<unsigned long>(output_val);
    break;
  case (LengthModifier::ll):
  case (LengthModifier::L):
    *reinterpret_cast<unsigned long long *>(output_ptr) =
        static_cast<unsigned long long>(output_val);
    break;
  case (LengthModifier::j):
    *reinterpret_cast<uintmax_t *>(output_ptr) =
        static_cast<uintmax_t>(output_val);
    break;
  case (LengthModifier::z):
    *reinterpret_cast<size_t *>(output_ptr) = static_cast<size_t>(output_val);
    break;
  case (LengthModifier::t):
    *reinterpret_cast<ptrdiff_t *>(output_ptr) =
        static_cast<ptrdiff_t>(output_val);
    break;
  }
}

LIBC_INLINE void write_float_with_length(char *str,
                                         const FormatSection &to_conv) {
  if ((to_conv.flags & NO_WRITE) != 0) {
    return;
  }

  void *output_ptr = to_conv.output_ptr;

  LengthModifier lm = to_conv.length_modifier;
  switch (lm) {
  case (LengthModifier::l): {
    auto value = internal::strtofloatingpoint<double>(str);
    *reinterpret_cast<double *>(output_ptr) = value;
    break;
  }
  case (LengthModifier::L): {
    auto value = internal::strtofloatingpoint<long double>(str);
    *reinterpret_cast<long double *>(output_ptr) = value;
    break;
  }
  default: {
    auto value = internal::strtofloatingpoint<float>(str);
    *reinterpret_cast<float *>(output_ptr) = value;
    break;
  }
  }
}

} // namespace scanf_core
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_STDIO_SCANF_CORE_CONVERTER_UTILS_H
