/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_PC_E2E_ANALYZER_VIDEO_VIDEO_FRAME_TRACKING_ID_INJECTOR_H_
#define TEST_PC_E2E_ANALYZER_VIDEO_VIDEO_FRAME_TRACKING_ID_INJECTOR_H_

#include <cstdint>

#include "api/video/encoded_image.h"
#include "test/pc/e2e/analyzer/video/encoded_image_data_injector.h"

namespace webrtc {
namespace webrtc_pc_e2e {

// This injector sets and retrieves the provided id in the EncodedImage
// video_frame_tracking_id field. This is only possible with the RTP header
// extension VideoFrameTrackingIdExtension that will propagate the input
// tracking id to the received EncodedImage. This RTP header extension is
// enabled with the field trial WebRTC-VideoFrameTrackingIdAdvertised
// (http://www.webrtc.org/experiments/rtp-hdrext/video-frame-tracking-id).
//
// Note that this injector doesn't allow to discard frames.
class VideoFrameTrackingIdInjector : public EncodedImageDataPropagator {
 public:
  EncodedImage InjectData(uint16_t id,
                          bool unused_discard,
                          const EncodedImage& source) override;

  EncodedImageExtractionResult ExtractData(const EncodedImage& source) override;

  void Start(int) override {}
  void AddParticipantInCall() override {}
  void RemoveParticipantInCall() override {}
};

}  // namespace webrtc_pc_e2e
}  // namespace webrtc

#endif  // TEST_PC_E2E_ANALYZER_VIDEO_VIDEO_FRAME_TRACKING_ID_INJECTOR_H_
