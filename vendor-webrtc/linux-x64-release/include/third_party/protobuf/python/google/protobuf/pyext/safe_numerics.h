// Protocol Buffers - Google's data interchange format
// Copyright 2008 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_PYTHON_CPP_SAFE_NUMERICS_H__
#define GOOGLE_PROTOBUF_PYTHON_CPP_SAFE_NUMERICS_H__
// Copied from chromium with only changes to the namespace.

#include <limits>

#include "absl/log/absl_check.h"

namespace google {
namespace protobuf {
namespace python {

template <bool SameSize, bool DestLarger,
          bool DestIsSigned, bool SourceIsSigned>
struct IsValidNumericCastImpl;

#define BASE_NUMERIC_CAST_CASE_SPECIALIZATION(A, B, C, D, Code) \
template <> struct IsValidNumericCastImpl<A, B, C, D> { \
  template <class Source, class DestBounds> static inline bool Test( \
      Source source, DestBounds min, DestBounds max) { \
    return Code; \
  } \
}

#define BASE_NUMERIC_CAST_CASE_SAME_SIZE(DestSigned, SourceSigned, Code) \
  BASE_NUMERIC_CAST_CASE_SPECIALIZATION( \
      true, true, DestSigned, SourceSigned, Code); \
  BASE_NUMERIC_CAST_CASE_SPECIALIZATION( \
      true, false, DestSigned, SourceSigned, Code)

#define BASE_NUMERIC_CAST_CASE_SOURCE_LARGER(DestSigned, SourceSigned, Code) \
  BASE_NUMERIC_CAST_CASE_SPECIALIZATION( \
      false, false, DestSigned, SourceSigned, Code); \

#define BASE_NUMERIC_CAST_CASE_DEST_LARGER(DestSigned, SourceSigned, Code) \
  BASE_NUMERIC_CAST_CASE_SPECIALIZATION( \
      false, true, DestSigned, SourceSigned, Code); \

// The three top level cases are:
// - Same size
// - Source larger
// - Dest larger
// And for each of those three cases, we handle the 4 different possibilities
// of signed and unsigned. This gives 12 cases to handle, which we enumerate
// below.
//
// The last argument in each of the macros is the actual comparison code. It
// has three arguments available, source (the value), and min/max which are
// the ranges of the destination.


// These are the cases where both types have the same size.

// Both signed.
BASE_NUMERIC_CAST_CASE_SAME_SIZE(true, true, true);
// Both unsigned.
BASE_NUMERIC_CAST_CASE_SAME_SIZE(false, false, true);
// Dest unsigned, Source signed.
BASE_NUMERIC_CAST_CASE_SAME_SIZE(false, true, source >= 0);
// Dest signed, Source unsigned.
// This cast is OK because Dest's max must be less than Source's.
BASE_NUMERIC_CAST_CASE_SAME_SIZE(true, false,
                                 source <= static_cast<Source>(max));


// These are the cases where Source is larger.

// Both unsigned.
BASE_NUMERIC_CAST_CASE_SOURCE_LARGER(false, false, source <= max);
// Both signed.
BASE_NUMERIC_CAST_CASE_SOURCE_LARGER(true, true,
                                     source >= min && source <= max);
// Dest is unsigned, Source is signed.
BASE_NUMERIC_CAST_CASE_SOURCE_LARGER(false, true,
                                     source >= 0 && source <= max);
// Dest is signed, Source is unsigned.
// This cast is OK because Dest's max must be less than Source's.
BASE_NUMERIC_CAST_CASE_SOURCE_LARGER(true, false,
                                     source <= static_cast<Source>(max));


// These are the cases where Dest is larger.

// Both unsigned.
BASE_NUMERIC_CAST_CASE_DEST_LARGER(false, false, true);
// Both signed.
BASE_NUMERIC_CAST_CASE_DEST_LARGER(true, true, true);
// Dest is unsigned, Source is signed.
BASE_NUMERIC_CAST_CASE_DEST_LARGER(false, true, source >= 0);
// Dest is signed, Source is unsigned.
BASE_NUMERIC_CAST_CASE_DEST_LARGER(true, false, true);

#undef BASE_NUMERIC_CAST_CASE_SPECIALIZATION
#undef BASE_NUMERIC_CAST_CASE_SAME_SIZE
#undef BASE_NUMERIC_CAST_CASE_SOURCE_LARGER
#undef BASE_NUMERIC_CAST_CASE_DEST_LARGER


// The main test for whether the conversion will under or overflow.
template <class Dest, class Source>
inline bool IsValidNumericCast(Source source) {
  typedef std::numeric_limits<Source> SourceLimits;
  typedef std::numeric_limits<Dest> DestLimits;
  static_assert(SourceLimits::is_specialized, "argument must be numeric");
  static_assert(SourceLimits::is_integer, "argument must be integral");
  static_assert(DestLimits::is_specialized, "result must be numeric");
  static_assert(DestLimits::is_integer, "result must be integral");

  return IsValidNumericCastImpl<
      sizeof(Dest) == sizeof(Source),
      (sizeof(Dest) > sizeof(Source)),
      DestLimits::is_signed,
      SourceLimits::is_signed>::Test(
          source,
          DestLimits::min(),
          DestLimits::max());
}

// checked_numeric_cast<> is analogous to static_cast<> for numeric types,
// except that it CHECKs that the specified numeric conversion will not
// overflow or underflow. Floating point arguments are not currently allowed
// (this is static_asserted), though this could be supported if necessary.
template <class Dest, class Source>
inline Dest checked_numeric_cast(Source source) {
  ABSL_CHECK(IsValidNumericCast<Dest>(source));
  return static_cast<Dest>(source);
}

}  // namespace python
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_PYTHON_CPP_SAFE_NUMERICS_H__
