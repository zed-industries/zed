/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_CODECS_TEST_VIDEOPROCESSOR_H_
#define MODULES_VIDEO_CODING_CODECS_TEST_VIDEOPROCESSOR_H_

#include <stddef.h>
#include <stdint.h>

#include <map>
#include <memory>
#include <optional>
#include <utility>
#include <vector>

#include "api/environment/environment.h"
#include "api/scoped_refptr.h"
#include "api/sequence_checker.h"
#include "api/task_queue/task_queue_base.h"
#include "api/test/videocodec_test_fixture.h"
#include "api/test/videocodec_test_stats.h"
#include "api/video/encoded_image.h"
#include "api/video/i420_buffer.h"
#include "api/video/video_bitrate_allocator.h"
#include "api/video/video_codec_type.h"
#include "api/video/video_frame.h"
#include "api/video/video_frame_buffer.h"
#include "api/video_codecs/video_decoder.h"
#include "api/video_codecs/video_encoder.h"
#include "modules/video_coding/codecs/test/videocodec_test_stats_impl.h"
#include "modules/video_coding/include/video_codec_interface.h"
#include "modules/video_coding/utility/ivf_file_writer.h"
#include "rtc_base/checks.h"
#include "rtc_base/system/no_unique_address.h"
#include "rtc_base/thread_annotations.h"
#include "test/testsupport/frame_reader.h"
#include "test/testsupport/frame_writer.h"

namespace webrtc {
namespace test {

// Handles encoding/decoding of video using the VideoEncoder/VideoDecoder
// interfaces. This is done in a sequential manner in order to be able to
// measure times properly.
// The class processes a frame at the time for the configured input file.
// It maintains state of where in the source input file the processing is at.
// TODO(webrtc:14852): Deprecated in favor VideoCodecTester.
class VideoProcessor {
 public:
  using VideoDecoderList = std::vector<std::unique_ptr<VideoDecoder>>;
  using LayerKey = std::pair<int /* spatial_idx */, int /* temporal_idx */>;
  using IvfFileWriterMap = std::map<LayerKey, std::unique_ptr<IvfFileWriter>>;
  // TODO(brandtr): Consider changing FrameWriterList to be a FrameWriterMap,
  // to be able to save different TLs separately.
  using FrameWriterList = std::vector<std::unique_ptr<FrameWriter>>;
  using FrameStatistics = VideoCodecTestStats::FrameStatistics;

  VideoProcessor(const Environment& env,
                 VideoEncoder* encoder,
                 VideoDecoderList* decoders,
                 FrameReader* input_frame_reader,
                 const VideoCodecTestFixture::Config& config,
                 VideoCodecTestStatsImpl* stats,
                 IvfFileWriterMap* encoded_frame_writers,
                 FrameWriterList* decoded_frame_writers);
  ~VideoProcessor();

  VideoProcessor(const VideoProcessor&) = delete;
  VideoProcessor& operator=(const VideoProcessor&) = delete;

  // Reads a frame and sends it to the encoder. When the encode callback
  // is received, the encoded frame is buffered. After encoding is finished
  // buffered frame is sent to decoder. Quality evaluation is done in
  // the decode callback.
  void ProcessFrame();

  // Updates the encoder with target rates. Must be called at least once.
  void SetRates(size_t bitrate_kbps, double framerate_fps);

  // Signals processor to finalize frame processing and handle possible tail
  // drops. If not called expelicitly, this will be called in dtor. It is
  // unexpected to get ProcessFrame() or SetRates() calls after Finalize().
  void Finalize();

 private:
  class VideoProcessorEncodeCompleteCallback
      : public webrtc::EncodedImageCallback {
   public:
    explicit VideoProcessorEncodeCompleteCallback(
        VideoProcessor* video_processor)
        : video_processor_(video_processor),
          task_queue_(TaskQueueBase::Current()) {
      RTC_DCHECK(video_processor_);
      RTC_DCHECK(task_queue_);
    }

    Result OnEncodedImage(
        const webrtc::EncodedImage& encoded_image,
        const webrtc::CodecSpecificInfo* codec_specific_info) override {
      RTC_CHECK(codec_specific_info);

      // Post the callback to the right task queue, if needed.
      if (!task_queue_->IsCurrent()) {
        VideoProcessor* video_processor = video_processor_;
        task_queue_->PostTask([video_processor, encoded_image,
                               codec_specific_info = *codec_specific_info] {
          video_processor->FrameEncoded(encoded_image, codec_specific_info);
        });
        return Result(Result::OK, 0);
      }

      video_processor_->FrameEncoded(encoded_image, *codec_specific_info);
      return Result(Result::OK, 0);
    }

   private:
    VideoProcessor* const video_processor_;
    TaskQueueBase* const task_queue_;
  };

  class VideoProcessorDecodeCompleteCallback
      : public webrtc::DecodedImageCallback {
   public:
    explicit VideoProcessorDecodeCompleteCallback(
        VideoProcessor* video_processor,
        size_t simulcast_svc_idx)
        : video_processor_(video_processor),
          simulcast_svc_idx_(simulcast_svc_idx),
          task_queue_(TaskQueueBase::Current()) {
      RTC_DCHECK(video_processor_);
      RTC_DCHECK(task_queue_);
    }

    int32_t Decoded(webrtc::VideoFrame& image) override;

    int32_t Decoded(webrtc::VideoFrame& image,
                    int64_t /* decode_time_ms */) override {
      return Decoded(image);
    }

    void Decoded(webrtc::VideoFrame& image,
                 std::optional<int32_t> /* decode_time_ms */,
                 std::optional<uint8_t> /* qp */) override {
      Decoded(image);
    }

   private:
    VideoProcessor* const video_processor_;
    const size_t simulcast_svc_idx_;
    TaskQueueBase* const task_queue_;
  };

  // Invoked by the callback adapter when a frame has completed encoding.
  void FrameEncoded(const webrtc::EncodedImage& encoded_image,
                    const webrtc::CodecSpecificInfo& codec_specific);

  // Invoked by the callback adapter when a frame has completed decoding.
  void FrameDecoded(const webrtc::VideoFrame& image, size_t simulcast_svc_idx);

  void DecodeFrame(const EncodedImage& encoded_image, size_t simulcast_svc_idx);

  // In order to supply the SVC decoders with super frames containing all
  // lower layer frames, we merge and store the layer frames in this method.
  const webrtc::EncodedImage* BuildAndStoreSuperframe(
      const EncodedImage& encoded_image,
      VideoCodecType codec,
      size_t frame_number,
      size_t simulcast_svc_idx,
      bool inter_layer_predicted) RTC_RUN_ON(sequence_checker_);

  void CalcFrameQuality(const I420BufferInterface& decoded_frame,
                        FrameStatistics* frame_stat);

  void WriteDecodedFrame(const I420BufferInterface& decoded_frame,
                         FrameWriter& frame_writer);

  void HandleTailDrops();

  // Test config.
  const VideoCodecTestFixture::Config config_;
  const size_t num_simulcast_or_spatial_layers_;
  const bool analyze_frame_quality_;

  // Frame statistics.
  VideoCodecTestStatsImpl* const stats_;

  // Codecs.
  webrtc::VideoEncoder* const encoder_;
  VideoDecoderList* const decoders_;
  const std::unique_ptr<VideoBitrateAllocator> bitrate_allocator_;

  // Target bitrate and framerate per frame.
  std::map<size_t, RateProfile> target_rates_ RTC_GUARDED_BY(sequence_checker_);

  // Adapters for the codec callbacks.
  VideoProcessorEncodeCompleteCallback encode_callback_;
  // Assign separate callback object to each decoder. This allows us to identify
  // decoded layer in frame decode callback.
  // simulcast_svc_idx -> decode callback.
  std::vector<std::unique_ptr<VideoProcessorDecodeCompleteCallback>>
      decode_callback_;

  // Each call to ProcessFrame() will read one frame from `input_frame_reader_`.
  FrameReader* const input_frame_reader_;

  // Input frames are used as reference for frame quality evaluations.
  // Async codecs might queue frames. To handle that we keep input frame
  // and release it after corresponding coded frame is decoded and quality
  // measurement is done.
  // frame_number -> frame.
  std::map<size_t, VideoFrame> input_frames_ RTC_GUARDED_BY(sequence_checker_);

  // Encoder delivers coded frame layer-by-layer. We store coded frames and
  // then, after all layers are encoded, decode them. Such separation of
  // frame processing on superframe level simplifies encoding/decoding time
  // measurement.
  // simulcast_svc_idx -> merged SVC encoded frame.
  std::vector<EncodedImage> merged_encoded_frames_
      RTC_GUARDED_BY(sequence_checker_);

  // These (optional) file writers are used to persistently store the encoded
  // and decoded bitstreams. Each frame writer is enabled by being non-null.
  IvfFileWriterMap* const encoded_frame_writers_;
  FrameWriterList* const decoded_frame_writers_;

  // Metadata for inputed/encoded/decoded frames. Used for frame identification,
  // frame drop detection, etc. We assume that encoded/decoded frames are
  // ordered within each simulcast/spatial layer, but we do not make any
  // assumptions of frame ordering between layers.
  size_t last_inputed_frame_num_ RTC_GUARDED_BY(sequence_checker_);
  size_t last_inputed_timestamp_ RTC_GUARDED_BY(sequence_checker_);
  // simulcast_svc_idx -> encode status.
  std::vector<bool> first_encoded_frame_ RTC_GUARDED_BY(sequence_checker_);
  // simulcast_svc_idx -> frame_number.
  std::vector<size_t> last_encoded_frame_num_ RTC_GUARDED_BY(sequence_checker_);
  // simulcast_svc_idx -> decode status.
  std::vector<bool> first_decoded_frame_ RTC_GUARDED_BY(sequence_checker_);
  // simulcast_svc_idx -> frame_number.
  std::vector<size_t> last_decoded_frame_num_ RTC_GUARDED_BY(sequence_checker_);
  // simulcast_svc_idx -> buffer.
  std::vector<scoped_refptr<I420Buffer>> last_decoded_frame_buffer_
      RTC_GUARDED_BY(sequence_checker_);

  // Time spent in frame encode callback. It is accumulated for layers and
  // reset when frame encode starts. When next layer is encoded post-encode time
  // is substracted from measured encode time. Thus we get pure encode time.
  int64_t post_encode_time_ns_ RTC_GUARDED_BY(sequence_checker_);

  // Indicates whether Finalize() was called or not.
  bool is_finalized_ RTC_GUARDED_BY(sequence_checker_);

  // This class must be operated on a TaskQueue.
  RTC_NO_UNIQUE_ADDRESS SequenceChecker sequence_checker_;
};

}  // namespace test
}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_CODECS_TEST_VIDEOPROCESSOR_H_
