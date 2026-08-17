// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_SCOPED_PDH_QUERY_H_
#define BASE_WIN_SCOPED_PDH_QUERY_H_

#include "base/base_export.h"
#include "base/scoped_generic.h"
#include "base/win/pdh_shim.h"

namespace base::win {

namespace internal {

struct ScopedPdhQueryTraits {
  static PDH_HQUERY InvalidValue() { return nullptr; }
  static void Free(PDH_HQUERY query) { ::PdhCloseQuery(query); }
};

}  // namespace internal

// ScopedPdhQuery is a wrapper around a PDH_HQUERY, the handle used by
// Performance Counters functions (see
// https://learn.microsoft.com/en-us/windows/win32/api/_perf/.) Prefer this to
// using PDH_HQUERY directly to make sure that handles are always closed when
// going out of scope.
//
// Example use:
//
//   ScopedPdhQuery pdh_query = ScopedPdhQuery::Create();
//   if (pdh_query.is_valid()) {
//     ::PdhCollectQueryData(pdh_query.get(), ...);
//   }
//
// To adopt an already-open handle:
//
//   PDH_HQUERY pdh_handle;
//   PDH_STATUS status = ::PdhOpenQuery(..., &pdh_handle);
//   if (status == ERROR_SUCCESS) {
//     ScopedPdhQuery pdh_query(pdh_handle);
//     ::PdhCollectCollectQueryData(pdh_query.get(), ...);
//   }
class BASE_EXPORT ScopedPdhQuery
    : public ScopedGeneric<PDH_HQUERY, internal::ScopedPdhQueryTraits> {
 public:
  // Constructs a ScopedPdhQuery from a PDH_HQUERY, and takes ownership of
  // `pdh_query` if it is not null.
  explicit ScopedPdhQuery(PDH_HQUERY pdh_query = nullptr)
      : ScopedGeneric(pdh_query) {}

  // Returns a ScopedPdhQuery to the default real-time data source. Equivalent
  // to ::PdhOpenQuery(nullptr, nullptr, &pdh_query).
  static ScopedPdhQuery Create();
};

}  // namespace base::win

#endif  // BASE_WIN_SCOPED_PDH_QUERY_H_
