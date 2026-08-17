/*
 *  Copyright (c) 2023 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_CODECS_VIDEO_ENCODING_GENERAL_H_
#define API_VIDEO_CODECS_VIDEO_ENCODING_GENERAL_H_

namespace webrtc {

struct EncodingFormat {
  enum SubSampling { k420, k422, k444 };
  SubSampling sub_sampling;
  int bit_depth;
};

}  // namespace webrtc
#endif  // API_VIDEO_CODECS_VIDEO_ENCODING_GENERAL_H_
