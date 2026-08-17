/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef NET_DCSCTP_PACKET_CRC32C_H_
#define NET_DCSCTP_PACKET_CRC32C_H_

#include <cstdint>

#include "api/array_view.h"

namespace dcsctp {

// Generates the CRC32C checksum of `data`.
uint32_t GenerateCrc32C(webrtc::ArrayView<const uint8_t> data);

}  // namespace dcsctp

#endif  // NET_DCSCTP_PACKET_CRC32C_H_
