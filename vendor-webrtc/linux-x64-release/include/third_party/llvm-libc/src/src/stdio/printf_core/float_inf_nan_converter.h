//===-- Inf or Nan Converter for printf -------------------------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#ifndef LLVM_LIBC_SRC_STDIO_PRINTF_CORE_FLOAT_INF_NAN_CONVERTER_H
#define LLVM_LIBC_SRC_STDIO_PRINTF_CORE_FLOAT_INF_NAN_CONVERTER_H

#include "src/__support/FPUtil/FPBits.h"
#include "src/__support/ctype_utils.h"
#include "src/__support/macros/config.h"
#include "src/stdio/printf_core/converter_utils.h"
#include "src/stdio/printf_core/core_structs.h"
#include "src/stdio/printf_core/writer.h"

#include <inttypes.h>
#include <stddef.h>

namespace LIBC_NAMESPACE_DECL {
namespace printf_core {

using StorageType = fputil::FPBits<long double>::StorageType;

template <WriteMode write_mode>
LIBC_INLINE int convert_inf_nan(Writer<write_mode> *writer,
                                const FormatSection &to_conv) {
  // All of the letters will be defined relative to variable a, which will be
  // the appropriate case based on the case of the conversion.
  bool is_negative;
  StorageType mantissa;
  if (to_conv.length_modifier == LengthModifier::L) {
    fputil::FPBits<long double>::StorageType float_raw = to_conv.conv_val_raw;
    fputil::FPBits<long double> float_bits(float_raw);
    is_negative = float_bits.is_neg();
    mantissa = float_bits.get_mantissa();
  } else {
    fputil::FPBits<double>::StorageType float_raw =
        static_cast<fputil::FPBits<double>::StorageType>(to_conv.conv_val_raw);
    fputil::FPBits<double> float_bits(float_raw);
    is_negative = float_bits.is_neg();
    mantissa = float_bits.get_mantissa();
  }

  char sign_char = 0;

  if (is_negative)
    sign_char = '-';
  else if ((to_conv.flags & FormatFlags::FORCE_SIGN) == FormatFlags::FORCE_SIGN)
    sign_char = '+'; // FORCE_SIGN has precedence over SPACE_PREFIX
  else if ((to_conv.flags & FormatFlags::SPACE_PREFIX) ==
           FormatFlags::SPACE_PREFIX)
    sign_char = ' ';

  // Both "inf" and "nan" are the same number of characters, being 3.
  int padding = to_conv.min_width - (sign_char > 0 ? 1 : 0) - 3;

  // The right justified pattern is (spaces), (sign), inf/nan
  // The left justified pattern is  (sign), inf/nan, (spaces)

  if (padding > 0 && ((to_conv.flags & FormatFlags::LEFT_JUSTIFIED) !=
                      FormatFlags::LEFT_JUSTIFIED))
    RET_IF_RESULT_NEGATIVE(writer->write(' ', padding));

  if (sign_char)
    RET_IF_RESULT_NEGATIVE(writer->write(sign_char));
  if (mantissa == 0) { // inf
    RET_IF_RESULT_NEGATIVE(
        writer->write(internal::islower(to_conv.conv_name) ? "inf" : "INF"));
  } else { // nan
    RET_IF_RESULT_NEGATIVE(
        writer->write(internal::islower(to_conv.conv_name) ? "nan" : "NAN"));
  }

  if (padding > 0 && ((to_conv.flags & FormatFlags::LEFT_JUSTIFIED) ==
                      FormatFlags::LEFT_JUSTIFIED))
    RET_IF_RESULT_NEGATIVE(writer->write(' ', padding));

  return WRITE_OK;
}

} // namespace printf_core
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_STDIO_PRINTF_CORE_FLOAT_INF_NAN_CONVERTER_H
