// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TYPES_SUPPORTS_TO_STRING_H_
#define BASE_TYPES_SUPPORTS_TO_STRING_H_

namespace base::internal {

template <typename T>
concept SupportsToString = requires(const T& t) { t.ToString(); };

}  // namespace base::internal

#endif  // BASE_TYPES_SUPPORTS_TO_STRING_H_
