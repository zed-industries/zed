// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIARECORDER_MEDIA_RECORDER_ENCODER_WRAPPER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIARECORDER_MEDIA_RECORDER_ENCODER_WRAPPER_H_

#include "base/memory/weak_ptr.h"
#include "base/sequence_checker.h"
#include "base/time/time.h"
#include "media/base/video_encoder.h"
#include "third_party/blink/renderer/modules/mediarecorder/video_track_recorder.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"

namespace media {
class GpuVideoAcceleratorFactories;
}  // namespace media

namespace blink {

// VideoTrackRecorder::Encoder class encodes h264, hevc, vp8, vp9 and av1 using
// media::VideoEncoder implementation.
class MODULES_EXPORT MediaRecorderEncoderWrapper final
    : public VideoTrackRecorder::Encoder {
 public:
  using CreateEncoderCB =
      base::RepeatingCallback<std::unique_ptr<media::VideoEncoder>(
          media::GpuVideoAcceleratorFactories*)>;
  using OnErrorCB = base::OnceCallback<void(const media::EncoderStatus&)>;

  MediaRecorderEncoderWrapper(
      scoped_refptr<base::SequencedTaskRunner> encoding_task_runner,
      media::VideoCodecProfile profile,
      uint32_t bits_per_second,
      bool is_screencast,
      media::GpuVideoAcceleratorFactories* gpu_factories,
      CreateEncoderCB create_encoder_cb,
      VideoTrackRecorder::OnEncodedVideoCB on_encoded_video_cb,
      OnErrorCB on_error_cb);
  ~MediaRecorderEncoderWrapper() override;

  MediaRecorderEncoderWrapper(const MediaRecorderEncoderWrapper&) = delete;
  MediaRecorderEncoderWrapper& operator=(const MediaRecorderEncoderWrapper&) =
      delete;

  bool IsScreenContentEncodingForTesting() const override;

 private:
  friend class MediaRecorderEncoderWrapperTest;

  struct EncodeTask {
    EncodeTask(scoped_refptr<media::VideoFrame> frame,
               base::TimeTicks capture_timestamp,
               bool request_keyframe);
    ~EncodeTask();
    scoped_refptr<media::VideoFrame> frame;
    base::TimeTicks capture_timestamp;
    bool request_keyframe;
  };

  struct VideoParamsAndTimestamp {
    VideoParamsAndTimestamp(const media::Muxer::VideoParameters& params,
                            base::TimeTicks timestamp);
    ~VideoParamsAndTimestamp();
    media::Muxer::VideoParameters params;
    base::TimeTicks timestamp;
  };

  enum class State {
    kEncoding,      // Can call Encode() if the frame size is the same.
    kInitializing,  // Call Flush() and Initialize(), and waiting the
                    // initialization is complete.
    kInError,       // In the error state. It can never encode any more frames.
  };

  // VideoTrackRecorder::Encoder implementation.
  void EncodeFrame(scoped_refptr<media::VideoFrame> frame,
                   base::TimeTicks capture_timestamp,
                   bool request_keyframe) override;
  bool CanEncodeAlphaChannel() const override;

  void EnterErrorState(const media::EncoderStatus& status);
  void Reconfigure(const gfx::Size& frame_size, bool encode_alpha);

  // (Re)creates |encoder_| and initialize the encoder with |frame_size|.
  // |status| can be non kOk only if it is called as flush done callback and the
  // the flush fails.
  void CreateAndInitialize(const gfx::Size& frame_size,
                           bool encode_alpha,
                           media::EncoderStatus status);
  void InitializeDone(media::EncoderStatus status);
  void EncodePendingTasks();
  void EncodeDone(media::EncoderStatus status);
  void OutputEncodeData(
      media::VideoEncoderOutput output,
      std::optional<media::VideoEncoder::CodecDescription> description);

  const raw_ptr<media::GpuVideoAcceleratorFactories> gpu_factories_;

  const media::VideoCodecProfile profile_;
  const media::VideoCodec codec_;

  const CreateEncoderCB create_encoder_cb_;
  OnErrorCB on_error_cb_;

  media::VideoEncoder::Options options_;
  bool encode_alpha_ = false;
  State state_ = State::kEncoding;
  WTF::Deque<EncodeTask> pending_encode_tasks_;
  WTF::Deque<VideoParamsAndTimestamp> params_in_encode_;

  std::unique_ptr<media::VideoEncoder> encoder_;

  SEQUENCE_CHECKER(sequence_checker_);

  base::WeakPtrFactory<MediaRecorderEncoderWrapper> weak_factory_{this};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIARECORDER_MEDIA_RECORDER_ENCODER_WRAPPER_H_
