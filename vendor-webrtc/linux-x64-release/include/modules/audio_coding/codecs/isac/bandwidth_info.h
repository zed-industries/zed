/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_CODECS_ISAC_BANDWIDTH_INFO_H_
#define MODULES_AUDIO_CODING_CODECS_ISAC_BANDWIDTH_INFO_H_

#include <stdint.h>

typedef struct {
  int in_use;
  int32_t send_bw_avg;
  int32_t send_max_delay_avg;
  int16_t bottleneck_idx;
  int16_t jitter_info;
} IsacBandwidthInfo;

#endif  // MODULES_AUDIO_CODING_CODECS_ISAC_BANDWIDTH_INFO_H_
