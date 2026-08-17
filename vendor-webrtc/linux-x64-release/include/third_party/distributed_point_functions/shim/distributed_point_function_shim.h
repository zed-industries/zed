// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef CONTENT_BROWSER_AGGREGATION_SERVICE_DISTRIBUTED_POINT_FUNCTION_SHIM_H_
#define CONTENT_BROWSER_AGGREGATION_SERVICE_DISTRIBUTED_POINT_FUNCTION_SHIM_H_

#include "third_party/distributed_point_functions/shim/buildflags.h"

static_assert(BUILDFLAG(USE_DISTRIBUTED_POINT_FUNCTIONS),
              "This header must not be included when "
              "distributed_point_functions is omitted from the build");

#include <optional>
#include <utility>
#include <vector>

#include "third_party/abseil-cpp/absl/numeric/int128.h"
#include "third_party/distributed_point_functions/dpf/distributed_point_function.pb.h"

namespace distributed_point_functions {

// Generates a pair of keys for a DPF that evaluates to `beta` when given
// `alpha`. On failure, returns std::nullopt.
std::optional<std::pair<DpfKey, DpfKey>> GenerateKeysIncremental(
    std::vector<DpfParameters> parameters,
    absl::uint128 alpha,
    std::vector<absl::uint128> beta);

}  // namespace distributed_point_functions

#endif  // CONTENT_BROWSER_AGGREGATION_SERVICE_DISTRIBUTED_POINT_FUNCTION_SHIM_H_
