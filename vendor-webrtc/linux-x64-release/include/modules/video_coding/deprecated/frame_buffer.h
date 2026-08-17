/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_DEPRECATED_FRAME_BUFFER_H_
#define MODULES_VIDEO_CODING_DEPRECATED_FRAME_BUFFER_H_

#include <stddef.h>
#include <stdint.h>

#include <vector>

#include "api/scoped_refptr.h"
#include "api/video/encoded_image.h"
#include "api/video/video_frame_type.h"
#include "modules/video_coding/codecs/h264/include/h264_globals.h"
#include "modules/video_coding/codecs/vp9/include/vp9_globals.h"
#include "modules/video_coding/deprecated/jitter_buffer_common.h"
#include "modules/video_coding/deprecated/packet.h"
#include "modules/video_coding/deprecated/session_info.h"
#include "modules/video_coding/encoded_frame.h"

namespace webrtc {

class VCMFrameBuffer : public VCMEncodedFrame {
 public:
  VCMFrameBuffer();
  virtual ~VCMFrameBuffer();

  virtual void Reset();

  VCMFrameBufferEnum InsertPacket(const VCMPacket& packet,
                                  int64_t timeInMs,
                                  const FrameData& frame_data);

  // State
  // Get current state of frame
  VCMFrameBufferStateEnum GetState() const;
  void PrepareForDecode(bool continuous);

  bool IsSessionComplete() const;
  bool HaveFirstPacket() const;
  int NumPackets() const;

  // Sequence numbers
  // Get lowest packet sequence number in frame
  int32_t GetLowSeqNum() const;
  // Get highest packet sequence number in frame
  int32_t GetHighSeqNum() const;

  int PictureId() const;
  int TemporalId() const;
  bool LayerSync() const;
  int Tl0PicId() const;

  std::vector<NaluInfo> GetNaluInfos() const;

  void SetGofInfo(const GofInfoVP9& gof_info, size_t idx);

  // Increments a counter to keep track of the number of packets of this frame
  // which were NACKed before they arrived.
  void IncrementNackCount();
  // Returns the number of packets of this frame which were NACKed before they
  // arrived.
  int16_t GetNackCount() const;

  int64_t LatestPacketTimeMs() const;

  webrtc::VideoFrameType FrameType() const;

 private:
  void SetState(VCMFrameBufferStateEnum state);  // Set state of frame

  VCMFrameBufferStateEnum _state;  // Current state of the frame
  // Set with SetEncodedData, but keep pointer to the concrete class here, to
  // enable reallocation and mutation.
  scoped_refptr<EncodedImageBuffer> encoded_image_buffer_;
  VCMSessionInfo _sessionInfo;
  uint16_t _nackCount;
  int64_t _latestPacketTimeMs;
};

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_DEPRECATED_FRAME_BUFFER_H_
