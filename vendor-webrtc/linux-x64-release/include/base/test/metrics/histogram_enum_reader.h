// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_METRICS_HISTOGRAM_ENUM_READER_H_
#define BASE_TEST_METRICS_HISTOGRAM_ENUM_READER_H_

#include <map>
#include <optional>
#include <string>

#include "base/metrics/histogram_base.h"

namespace base {

using HistogramEnumEntryMap = std::map<HistogramBase::Sample32, std::string>;

// Find and read the enum with the given |enum_name| (with integer values) from
// tools/metrics/histograms/enums.xml, or from enums.xml in the given
// |subdirectory| of tools/metrics/histograms/metadata.
//
// Returns map { value => label } so that:
//   <int value="9" label="enable-pinch-virtual-viewport"/>
// becomes:
//   { 9 => "enable-pinch-virtual-viewport" }
// Returns empty std::nullopt on failure.
std::optional<HistogramEnumEntryMap> ReadEnumFromEnumsXml(
    const std::string& enum_name,
    const std::optional<std::string>& subdirectory = std::nullopt);

}  // namespace base

#endif  // BASE_TEST_METRICS_HISTOGRAM_ENUM_READER_H_
