/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_VIDEO_CODING_IMPL_H_
#define MODULES_VIDEO_CODING_VIDEO_CODING_IMPL_H_

#include <cstddef>
#include <cstdint>
#include <map>
#include <optional>

#include "api/field_trials_view.h"
#include "api/rtp_headers.h"
#include "api/sequence_checker.h"
#include "api/video_codecs/video_decoder.h"
#include "modules/rtp_rtcp/source/rtp_video_header.h"
#include "modules/video_coding/deprecated/jitter_buffer.h"
#include "modules/video_coding/deprecated/receiver.h"
#include "modules/video_coding/generic_decoder.h"
#include "modules/video_coding/include/video_coding_defines.h"
#include "modules/video_coding/timing/timing.h"
#include "rtc_base/one_time_event.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/thread_annotations.h"
#include "system_wrappers/include/clock.h"

namespace webrtc {

class VideoBitrateAllocator;
class VideoBitrateAllocationObserver;

namespace vcm {

class VCMProcessTimer {
 public:
  static const int64_t kDefaultProcessIntervalMs = 1000;

  VCMProcessTimer(int64_t periodMs, Clock* clock)
      : _clock(clock),
        _periodMs(periodMs),
        _latestMs(_clock->TimeInMilliseconds()) {}
  int64_t Period() const;
  int64_t TimeUntilProcess() const;
  void Processed();

 private:
  Clock* _clock;
  int64_t _periodMs;
  int64_t _latestMs;
};

class DEPRECATED_VCMDecoderDataBase {
 public:
  DEPRECATED_VCMDecoderDataBase();
  DEPRECATED_VCMDecoderDataBase(const DEPRECATED_VCMDecoderDataBase&) = delete;
  DEPRECATED_VCMDecoderDataBase& operator=(
      const DEPRECATED_VCMDecoderDataBase&) = delete;
  ~DEPRECATED_VCMDecoderDataBase() = default;

  // Returns a pointer to the previously registered decoder or nullptr if none
  // was registered for the `payload_type`.
  VideoDecoder* DeregisterExternalDecoder(uint8_t payload_type);
  void RegisterExternalDecoder(uint8_t payload_type,
                               VideoDecoder* external_decoder);
  bool IsExternalDecoderRegistered(uint8_t payload_type) const;

  void RegisterReceiveCodec(uint8_t payload_type,
                            const VideoDecoder::Settings& settings);
  bool DeregisterReceiveCodec(uint8_t payload_type);

  // Returns a decoder specified by frame.PayloadType. The decoded frame
  // callback of the decoder is set to `decoded_frame_callback`. If no such
  // decoder already exists an instance will be created and initialized.
  // nullptr is returned if no decoder with the specified payload type was found
  // and the function failed to create one.
  VCMGenericDecoder* GetDecoder(
      const VCMEncodedFrame& frame,
      VCMDecodedFrameCallback* decoded_frame_callback);

 private:
  void CreateAndInitDecoder(const VCMEncodedFrame& frame)
      RTC_RUN_ON(decoder_sequence_checker_);

  SequenceChecker decoder_sequence_checker_;

  std::optional<uint8_t> current_payload_type_;
  std::optional<VCMGenericDecoder> current_decoder_
      RTC_GUARDED_BY(decoder_sequence_checker_);
  // Initialization paramaters for decoders keyed by payload type.
  std::map<uint8_t, VideoDecoder::Settings> decoder_settings_;
  // Decoders keyed by payload type.
  std::map<uint8_t, VideoDecoder*> decoders_
      RTC_GUARDED_BY(decoder_sequence_checker_);
};

class VideoReceiver {
 public:
  VideoReceiver(Clock* clock,
                VCMTiming* timing,
                const FieldTrialsView& field_trials);
  ~VideoReceiver();

  void RegisterReceiveCodec(uint8_t payload_type,
                            const VideoDecoder::Settings& settings);

  void RegisterExternalDecoder(VideoDecoder* externalDecoder,
                               uint8_t payloadType);
  int32_t RegisterReceiveCallback(VCMReceiveCallback* receiveCallback);
  int32_t RegisterFrameTypeCallback(VCMFrameTypeCallback* frameTypeCallback);
  int32_t RegisterPacketRequestCallback(VCMPacketRequestCallback* callback);

  int32_t Decode(uint16_t maxWaitTimeMs);

  int32_t IncomingPacket(const uint8_t* incomingPayload,
                         size_t payloadLength,
                         const RTPHeader& rtp_header,
                         const RTPVideoHeader& video_header);

  void SetNackSettings(size_t max_nack_list_size,
                       int max_packet_age_to_nack,
                       int max_incomplete_time_ms);

  void Process();

 protected:
  int32_t Decode(const webrtc::VCMEncodedFrame& frame);
  int32_t RequestKeyFrame();

 private:
  // Used for DCHECKing thread correctness.
  // In build where DCHECKs are enabled, will return false before
  // DecoderThreadStarting is called, then true until DecoderThreadStopped
  // is called.
  // In builds where DCHECKs aren't enabled, it will return true.
  bool IsDecoderThreadRunning();

  SequenceChecker construction_thread_checker_;
  SequenceChecker decoder_thread_checker_;
  SequenceChecker module_thread_checker_;
  Clock* const clock_;
  Mutex process_mutex_;
  VCMTiming* _timing;
  VCMReceiver _receiver;
  VCMDecodedFrameCallback _decodedFrameCallback;

  // These callbacks are set on the construction thread before being attached
  // to the module thread or decoding started, so a lock is not required.
  VCMFrameTypeCallback* _frameTypeCallback;
  VCMPacketRequestCallback* _packetRequestCallback;

  // Used on both the module and decoder thread.
  bool _scheduleKeyRequest RTC_GUARDED_BY(process_mutex_);
  bool drop_frames_until_keyframe_ RTC_GUARDED_BY(process_mutex_);

  // Modified on the construction thread while not attached to the process
  // thread.  Once attached to the process thread, its value is only read
  // so a lock is not required.
  size_t max_nack_list_size_;

  // Callbacks are set before the decoder thread starts.
  // Once the decoder thread has been started, usage of `_codecDataBase` moves
  // over to the decoder thread.
  DEPRECATED_VCMDecoderDataBase _codecDataBase;

  VCMProcessTimer _retransmissionTimer RTC_GUARDED_BY(module_thread_checker_);
  VCMProcessTimer _keyRequestTimer RTC_GUARDED_BY(module_thread_checker_);
  ThreadUnsafeOneTimeEvent first_frame_received_
      RTC_GUARDED_BY(decoder_thread_checker_);
};

}  // namespace vcm
}  // namespace webrtc
#endif  // MODULES_VIDEO_CODING_VIDEO_CODING_IMPL_H_
