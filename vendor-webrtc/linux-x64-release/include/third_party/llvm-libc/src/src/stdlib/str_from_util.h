//===-- Implementation header for strfromx() utilitites -------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

// According to the C23 standard, any input character sequences except a
// precision specifier and the usual floating point formats, namely
// %{a,A,e,E,f,F,g,G}, are not allowed and any code that does otherwise results
// in undefined behaviour(including use of a '%%' conversion specifier); which
// in this case is that the buffer string is simply populated with the format
// string. The case of the input being nullptr should be handled in the calling
// function (strfromf, strfromd, strfroml) itself.

#ifndef LLVM_LIBC_SRC_STDLIB_STRFROM_UTIL_H
#define LLVM_LIBC_SRC_STDLIB_STRFROM_UTIL_H

#include "src/__support/CPP/type_traits.h"
#include "src/__support/macros/config.h"
#include "src/__support/str_to_integer.h"
#include "src/stdio/printf_core/converter_atlas.h"
#include "src/stdio/printf_core/core_structs.h"
#include "src/stdio/printf_core/writer.h"

#include <stddef.h>

namespace LIBC_NAMESPACE_DECL {
namespace internal {

template <typename T>
using storage_type = typename fputil::FPBits<T>::StorageType;

template <typename T>
printf_core::FormatSection parse_format_string(const char *__restrict format,
                                               T fp) {
  printf_core::FormatSection section;
  size_t cur_pos = 0;

  // There is no typed conversion function to convert single precision float
  // to hex exponential format, and the function convert_float_hex_exp()
  // requires a double or long double value to work correctly.
  // To work around this, we convert fp to double if it is single precision, and
  // then use that double precision value in the %{A, a} conversion specifiers.
  [[maybe_unused]] double new_fp;
  bool t_is_single_prec_type = cpp::is_same<T, float>::value;
  if (t_is_single_prec_type)
    new_fp = (double)fp;

  if (format[cur_pos] == '%') {
    section.has_conv = true;
    ++cur_pos;

    // handle precision
    section.precision = -1;
    if (format[cur_pos] == '.') {
      ++cur_pos;
      section.precision = 0;

      // The standard does not allow the '*' (asterisk) operator for strfromx()
      // functions
      if (internal::isdigit(format[cur_pos])) {
        auto result = internal::strtointeger<int>(format + cur_pos, 10);
        section.precision += result.value;
        cur_pos += result.parsed_len;
      }
    }

    section.conv_name = format[cur_pos];
    switch (format[cur_pos]) {
    case 'a':
    case 'A':
      if (t_is_single_prec_type)
        section.conv_val_raw = cpp::bit_cast<storage_type<double>>(new_fp);
      else
        section.conv_val_raw = cpp::bit_cast<storage_type<T>>(fp);
      break;
    case 'e':
    case 'E':
    case 'f':
    case 'F':
    case 'g':
    case 'G':
      section.conv_val_raw = cpp::bit_cast<storage_type<T>>(fp);
      break;
    default:
      section.has_conv = false;
      while (format[cur_pos] != '\0')
        ++cur_pos;
      break;
    }

    if (format[cur_pos] != '\0')
      ++cur_pos;
  } else {
    section.has_conv = false;
    // We are looking for exactly one section, so no more '%'
    while (format[cur_pos] != '\0')
      ++cur_pos;
  }

  section.raw_string = {format, cur_pos};
  return section;
}

template <typename T, printf_core::WriteMode write_mode>
int strfromfloat_convert(printf_core::Writer<write_mode> *writer,
                         const printf_core::FormatSection &section) {
  if (!section.has_conv)
    return writer->write(section.raw_string);

  auto res = static_cast<storage_type<T>>(section.conv_val_raw);

  fputil::FPBits<T> strfromfloat_bits(res);
  if (strfromfloat_bits.is_inf_or_nan())
    return convert_inf_nan(writer, section);

  switch (section.conv_name) {
  case 'f':
  case 'F':
    return convert_float_decimal_typed(writer, section, strfromfloat_bits);
  case 'e':
  case 'E':
    return convert_float_dec_exp_typed(writer, section, strfromfloat_bits);
  case 'a':
  case 'A':
    return convert_float_hex_exp(writer, section);
  case 'g':
  case 'G':
    return convert_float_dec_auto_typed(writer, section, strfromfloat_bits);
  default:
    return writer->write(section.raw_string);
  }
  return -1;
}

} // namespace internal
} // namespace LIBC_NAMESPACE_DECL

#endif // LLVM_LIBC_SRC_STDLIB_STRFROM_UTIL_H
