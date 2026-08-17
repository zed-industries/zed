/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_PC_E2E_ANALYZER_VIDEO_QUALITY_ANALYZING_VIDEO_DECODER_H_
#define TEST_PC_E2E_ANALYZER_VIDEO_QUALITY_ANALYZING_VIDEO_DECODER_H_

#include <map>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/environment/environment.h"
#include "api/test/video_quality_analyzer_interface.h"
#include "api/video/encoded_image.h"
#include "api/video/video_frame.h"
#include "api/video_codecs/sdp_video_format.h"
#include "api/video_codecs/video_decoder.h"
#include "api/video_codecs/video_decoder_factory.h"
#include "rtc_base/synchronization/mutex.h"
#include "test/pc/e2e/analyzer/video/encoded_image_data_injector.h"

namespace webrtc {
namespace webrtc_pc_e2e {

// QualityAnalyzingVideoDecoder is used to wrap origin video decoder and inject
// VideoQualityAnalyzerInterface before and after decoder.
//
// QualityAnalyzingVideoDecoder propagates all calls to the origin decoder.
// It registers its own DecodedImageCallback in the origin decoder and will
// store user specified callback inside itself.
//
// When Decode(...) will be invoked, quality decoder first will extract frame id
// from passed EncodedImage with EncodedImageIdExtracor that was specified in
// constructor, then will call video quality analyzer, with correct
// EncodedImage and only then will pass image to origin decoder.
//
// When origin decoder decodes the image it will call quality decoder's special
// callback, where video analyzer will be called again and then decoded frame
// will be passed to origin callback, provided by user.
//
// Quality decoder registers its own callback in origin decoder, at the same
// time the user registers their callback in quality decoder.
class QualityAnalyzingVideoDecoder : public VideoDecoder {
 public:
  QualityAnalyzingVideoDecoder(absl::string_view peer_name,
                               std::unique_ptr<VideoDecoder> delegate,
                               EncodedImageDataExtractor* extractor,
                               VideoQualityAnalyzerInterface* analyzer);
  ~QualityAnalyzingVideoDecoder() override;

  // Methods of VideoDecoder interface.
  bool Configure(const Settings& settings) override;
  int32_t Decode(const EncodedImage& input_image,
                 int64_t render_time_ms) override;
  int32_t RegisterDecodeCompleteCallback(
      DecodedImageCallback* callback) override;
  int32_t Release() override;
  DecoderInfo GetDecoderInfo() const override;
  const char* ImplementationName() const override;

 private:
  class DecoderCallback : public DecodedImageCallback {
   public:
    explicit DecoderCallback(QualityAnalyzingVideoDecoder* decoder);
    ~DecoderCallback() override;

    void SetDelegateCallback(DecodedImageCallback* delegate);

    // Methods of DecodedImageCallback interface.
    int32_t Decoded(VideoFrame& decodedImage) override;
    int32_t Decoded(VideoFrame& decodedImage, int64_t decode_time_ms) override;
    void Decoded(VideoFrame& decodedImage,
                 std::optional<int32_t> decode_time_ms,
                 std::optional<uint8_t> qp) override;

    int32_t IrrelevantSimulcastStreamDecoded(uint16_t frame_id,
                                             uint32_t timestamp_ms);

   private:
    scoped_refptr<webrtc::VideoFrameBuffer> GetDummyFrameBuffer();

    QualityAnalyzingVideoDecoder* const decoder_;

    scoped_refptr<webrtc::VideoFrameBuffer> dummy_frame_buffer_;

    Mutex callback_mutex_;
    DecodedImageCallback* delegate_callback_ RTC_GUARDED_BY(callback_mutex_);
  };

  void OnFrameDecoded(VideoFrame* frame,
                      std::optional<int32_t> decode_time_ms,
                      std::optional<uint8_t> qp);

  const std::string peer_name_;
  const std::string implementation_name_;
  std::unique_ptr<VideoDecoder> delegate_;
  EncodedImageDataExtractor* const extractor_;
  VideoQualityAnalyzerInterface* const analyzer_;
  std::unique_ptr<DecoderCallback> analyzing_callback_;

  // VideoDecoder interface assumes async delivery of decoded video frames.
  // This lock is used to protect shared state, that have to be propagated
  // from received EncodedImage to resulted VideoFrame.
  Mutex mutex_;

  // Name of the video codec type used. Ex: VP8, VP9, H264 etc.
  std::string codec_name_ RTC_GUARDED_BY(mutex_);
  std::map<uint32_t, std::optional<uint16_t>> timestamp_to_frame_id_
      RTC_GUARDED_BY(mutex_);
  // Stores currently being decoded images by timestamp. Because
  // EncodedImageDataExtractor can create new copy on EncodedImage we need to
  // ensure, that this image won't be deleted during async decoding. To do it
  // all images are putted into this map and removed from here inside callback.
  std::map<uint32_t, EncodedImage> decoding_images_ RTC_GUARDED_BY(mutex_);
};

// Produces QualityAnalyzingVideoDecoder, which hold decoders, produced by
// specified factory as delegates. Forwards all other calls to specified
// factory.
class QualityAnalyzingVideoDecoderFactory : public VideoDecoderFactory {
 public:
  QualityAnalyzingVideoDecoderFactory(
      absl::string_view peer_name,
      std::unique_ptr<VideoDecoderFactory> delegate,
      EncodedImageDataExtractor* extractor,
      VideoQualityAnalyzerInterface* analyzer);
  ~QualityAnalyzingVideoDecoderFactory() override;

  // Methods of VideoDecoderFactory interface.
  std::vector<SdpVideoFormat> GetSupportedFormats() const override;
  std::unique_ptr<VideoDecoder> Create(const Environment& env,
                                       const SdpVideoFormat& format) override;

 private:
  const std::string peer_name_;
  std::unique_ptr<VideoDecoderFactory> delegate_;
  EncodedImageDataExtractor* const extractor_;
  VideoQualityAnalyzerInterface* const analyzer_;
};

}  // namespace webrtc_pc_e2e
}  // namespace webrtc

#endif  // TEST_PC_E2E_ANALYZER_VIDEO_QUALITY_ANALYZING_VIDEO_DECODER_H_
