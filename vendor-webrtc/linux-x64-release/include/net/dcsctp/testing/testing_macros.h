/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_TESTING_TESTING_MACROS_H_
#define NET_DCSCTP_TESTING_TESTING_MACROS_H_

#include <utility>

namespace dcsctp {

#define DCSCTP_CONCAT_INNER_(x, y) x##y
#define DCSCTP_CONCAT_(x, y) DCSCTP_CONCAT_INNER_(x, y)

// Similar to ASSERT_OK_AND_ASSIGN, this works with an std::optional<> instead
// of an absl::StatusOr<>.
#define ASSERT_HAS_VALUE_AND_ASSIGN(lhs, rexpr)                     \
  auto DCSCTP_CONCAT_(tmp_opt_val__, __LINE__) = rexpr;             \
  ASSERT_TRUE(DCSCTP_CONCAT_(tmp_opt_val__, __LINE__).has_value()); \
  lhs = *std::move(DCSCTP_CONCAT_(tmp_opt_val__, __LINE__));

}  // namespace dcsctp

#endif  // NET_DCSCTP_TESTING_TESTING_MACROS_H_
