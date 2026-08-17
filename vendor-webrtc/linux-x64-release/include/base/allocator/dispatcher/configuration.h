// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ALLOCATOR_DISPATCHER_CONFIGURATION_H_
#define BASE_ALLOCATOR_DISPATCHER_CONFIGURATION_H_

#include <cstddef>

namespace base::allocator::dispatcher::configuration {

// The maximum number of optional observers that may be present depending on
// command line parameters.
constexpr size_t kMaximumNumberOfOptionalObservers = 4;

// The total number of observers including mandatory and optional observers.
// Primarily the number of observers affects the performance at allocation time.
// The current value of 4 doesn't have hard evidence. Keep in mind that
// also a single observer can severely impact performance.
constexpr size_t kMaximumNumberOfObservers = 4;

}  // namespace base::allocator::dispatcher::configuration

#endif  // BASE_ALLOCATOR_DISPATCHER_CONFIGURATION_H_
