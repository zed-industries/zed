// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// A locale-independent version of strtod(), used to parse floating
// point default values in .proto files, where the decimal separator
// is always a dot.

#ifndef GOOGLE_PROTOBUF_IO_STRTOD_H__
#define GOOGLE_PROTOBUF_IO_STRTOD_H__

#include <string>

// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {
namespace io {

// ----------------------------------------------------------------------
// SimpleDtoa()
// SimpleFtoa()
//    Description: converts a double or float to a string which, if
//    passed to NoLocaleStrtod(), will produce the exact same original double
//    (except in case of NaN; all NaNs are considered the same value).
//    We try to keep the string short but it's not guaranteed to be as
//    short as possible.
//
//    Return value: string
// ----------------------------------------------------------------------
PROTOBUF_EXPORT std::string SimpleDtoa(double value);
PROTOBUF_EXPORT std::string SimpleFtoa(float value);

// A locale-independent version of the standard strtod(), which always
// uses a dot as the decimal separator.
PROTOBUF_EXPORT double NoLocaleStrtod(const char* str, char** endptr);

// Casts a double value to a float value. If the value is outside of the
// representable range of float, it will be converted to positive or negative
// infinity.
PROTOBUF_EXPORT float SafeDoubleToFloat(double value);

}  // namespace io
}  // namespace protobuf
}  // namespace google

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_IO_STRTOD_H__
