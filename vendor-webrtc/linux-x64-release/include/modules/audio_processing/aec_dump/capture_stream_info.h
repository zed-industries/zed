/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AEC_DUMP_CAPTURE_STREAM_INFO_H_
#define MODULES_AUDIO_PROCESSING_AEC_DUMP_CAPTURE_STREAM_INFO_H_

#include <memory>
#include <utility>

#include "modules/audio_processing/include/aec_dump.h"

// Files generated at build-time by the protobuf compiler.
#ifdef WEBRTC_ANDROID_PLATFORM_BUILD
#include "external/webrtc/webrtc/modules/audio_processing/debug.pb.h"
#else
#include "modules/audio_processing/debug.pb.h"
#endif

namespace webrtc {

class CaptureStreamInfo {
 public:
  CaptureStreamInfo() { CreateNewEvent(); }
  CaptureStreamInfo(const CaptureStreamInfo&) = delete;
  CaptureStreamInfo& operator=(const CaptureStreamInfo&) = delete;
  ~CaptureStreamInfo() = default;

  void AddInput(const AudioFrameView<const float>& src);
  void AddOutput(const AudioFrameView<const float>& src);

  void AddInput(const int16_t* const data,
                int num_channels,
                int samples_per_channel);
  void AddOutput(const int16_t* const data,
                 int num_channels,
                 int samples_per_channel);

  void AddAudioProcessingState(const AecDump::AudioProcessingState& state);

  std::unique_ptr<audioproc::Event> FetchEvent() {
    std::unique_ptr<audioproc::Event> result = std::move(event_);
    CreateNewEvent();
    return result;
  }

 private:
  void CreateNewEvent() {
    event_ = std::make_unique<audioproc::Event>();
    event_->set_type(audioproc::Event::STREAM);
  }
  std::unique_ptr<audioproc::Event> event_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AEC_DUMP_CAPTURE_STREAM_INFO_H_
