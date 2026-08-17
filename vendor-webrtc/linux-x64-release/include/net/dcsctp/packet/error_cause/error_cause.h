/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_PACKET_ERROR_CAUSE_ERROR_CAUSE_H_
#define NET_DCSCTP_PACKET_ERROR_CAUSE_ERROR_CAUSE_H_

#include <stddef.h>

#include <cstdint>
#include <iosfwd>
#include <memory>
#include <optional>
#include <string>
#include <type_traits>
#include <utility>
#include <vector>

#include "absl/algorithm/container.h"
#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "net/dcsctp/packet/parameter/parameter.h"
#include "net/dcsctp/packet/tlv_trait.h"

namespace dcsctp {

// Converts the Error Causes in `parameters` to a human readable string,
// to be used in error reporting and logging.
std::string ErrorCausesToString(const Parameters& parameters);

}  // namespace dcsctp

#endif  // NET_DCSCTP_PACKET_ERROR_CAUSE_ERROR_CAUSE_H_
