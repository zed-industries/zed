// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_RUST_FEND_CORE_V1_WRAPPER_FEND_CORE_H_
#define THIRD_PARTY_RUST_FEND_CORE_V1_WRAPPER_FEND_CORE_H_

#include <optional>
#include <string>

namespace fend_core {

// Try evaluating the query string with fend library, within `timeout_in_ms` ms.
// Returns the result string if the evaluation succeeded, or std::nullopt if
// failed. If `timeout_in_ms` = 0, there is no timeout.
std::optional<std::string> evaluate(std::string_view query,
                                    unsigned int timeout_in_ms = 100);

} // namespace fend_core

#endif // THIRD_PARTY_RUST_FEND_CORE_V1_WRAPPER_FEND_CORE_H_
