// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_METRICS_HISTOGRAM_VARIANTS_READER_H_
#define BASE_TEST_METRICS_HISTOGRAM_VARIANTS_READER_H_

#include <map>
#include <optional>
#include <string>
#include <variant>

namespace base {

using HistogramVariantsEntryMap = std::map<std::string, std::string>;

// Find and read the variants list with the given |variants_name| from
// histograms.xml in the given |subdirectory| of
// tools/metrics/histograms or (if |from_metadata| is set), from
// tools/metrics/histograms/metadata. The default is to source from the
// metadata folder.
//
// Useful for when you want to verify that the set of variants associated with
// a particular set of values actually matches the set of values. For example,
// BrowserUserEducationServiceTest.CheckFeaturePromoHistograms verifies that
// for every registered Chrome Desktop in-product-help experience, there is a
// corresponding variant for metrics collection. This prevents someone from
// adding an IPH experience without adding the corresponding histogram entry.
//
// Returns a map from name to summary, or nullopt on failure.
extern std::optional<HistogramVariantsEntryMap> ReadVariantsFromHistogramsXml(
    const std::string& variants_name,
    const std::string& subdirectory,
    bool from_metadata = true);

}  // namespace base

#endif  // BASE_TEST_METRICS_HISTOGRAM_VARIANTS_READER_H_
