/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_RTP_RTCP_SOURCE_FEC_PRIVATE_TABLES_BURSTY_H_
#define MODULES_RTP_RTCP_SOURCE_FEC_PRIVATE_TABLES_BURSTY_H_

// This file contains a set of packets masks for the FEC code. The masks in
// this table are specifically designed to favor recovery of bursty/consecutive
// loss network conditions. The tradeoff is worse recovery for random losses.
// These packet masks are currently defined to protect up to 12 media packets.
// They have the following property: for any packet mask defined by the
// parameters (k,m), where k = number of media packets, m = number of FEC
// packets, all "consecutive" losses of size <= m are completely recoverable.
// By consecutive losses we mean consecutive with respect to the sequence
// number ordering of the list (media and FEC) of packets. The difference
// between these masks (`kFecMaskBursty`) and `kFecMaskRandom` type, defined
// in fec_private_tables.h, is more significant for longer codes
// (i.e., more packets/symbols in the code, so larger (k,m), i.e.,  k > 4,
// m > 3).

#include <stdint.h>

namespace webrtc {
namespace fec_private_tables {

extern const uint8_t kPacketMaskBurstyTbl[];

}  // namespace fec_private_tables
}  // namespace webrtc
#endif  // MODULES_RTP_RTCP_SOURCE_FEC_PRIVATE_TABLES_BURSTY_H_
