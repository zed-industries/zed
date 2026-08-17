
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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_STRING_ENCODING_UTILS_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_STRING_ENCODING_UTILS_H_

#include <string>

#include "perfetto/protozero/field.h"

namespace perfetto {
namespace trace_processor {

// Converts a byte stream that represents a latin-1
// (https://en.wikipedia.org/wiki/ISO/IEC_8859-1) encoded string to a UTF-8
// (https://en.wikipedia.org/wiki/UTF-8) encoded std::string.
// This operation will never fail.
std::string ConvertLatin1ToUtf8(protozero::ConstBytes latin1);

// Converts a byte stream that represents a UTF16 Little Endian
// (https://en.wikipedia.org/wiki/ISO/IEC_8859-1) encoded string to a UTF-8
// (https://en.wikipedia.org/wiki/UTF-8) encoded std::string.
//
// NOTE: UTF16 CodeUnits that can not be correctly parsed will be converted to
// the invalid CodePoint U+FFFD.
//
// ATTENTION: This function performs no special handling of special characters
// such as BOM (byte order mark). In particular this means that the caller is
// responsible of determining the right endianness and remove those characters
// if needed.
std::string ConvertUtf16LeToUtf8(protozero::ConstBytes utf16);

// Converts a byte stream that represents a UTF16 Big Endian
// (https://en.wikipedia.org/wiki/ISO/IEC_8859-1) encoded string to a UTF-8
// (https://en.wikipedia.org/wiki/UTF-8) encoded std::string.
//
// NOTE: UTF16 CodeUnits that can not be correctly parsed will be converted to
// the invalid CodePoint U+FFFD.
//
// ATTENTION: This function performs no special handling of special characters
// such as BOM (byte order mark). In particular this means that the caller is
// responsible of determining the right endianness and remove any special
// character if  needed.
std::string ConvertUtf16BeToUtf8(protozero::ConstBytes utf16);

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_STRING_ENCODING_UTILS_H_
