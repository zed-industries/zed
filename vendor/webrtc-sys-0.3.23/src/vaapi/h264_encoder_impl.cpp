#include "h264_encoder_impl.h"

#include <algorithm>
#include <limits>
#include <string>

#include "absl/strings/match.h"
#include "absl/types/optional.h"
#include "api/video/video_codec_constants.h"
#include "api/video_codecs/scalability_mode.h"
#include "common_video/libyuv/include/webrtc_libyuv.h"
#include "modules/video_coding/include/video_codec_interface.h"
#include "modules/video_coding/include/video_error_codes.h"
#include "modules/video_coding/svc/create_scalability_structure.h"
#include "modules/video_coding/utility/simulcast_rate_allocator.h"
#include "modules/video_coding/utility/simulcast_utility.h"
#include "rtc_base/checks.h"
#include "rtc_base/logging.h"
#include "rtc_base/time_utils.h"
#include "system_wrappers/include/metrics.h"
#include "third_party/libyuv/include/libyuv/convert.h"
#include "third_party/libyuv/include/libyuv/scale.h"

#define VA_FOURCC_I420 0x30323449  // I420

namespace webrtc {

// Used by histograms. Values of entries should not be changed.
enum H264EncoderImplEvent {
  kH264EncoderEventInit = 0,
  kH264EncoderEventError = 1,
  kH264EncoderEventMax = 16,
};

VAAPIH264EncoderWrapper::VAAPIH264EncoderWrapper(const webrtc::Environment& env,
                                                 const SdpVideoFormat& format)
    : env_(env),
      encoder_(new livekit_ffi::VaapiH264EncoderWrapper()),
      packetization_mode_(
          H264EncoderSettings::Parse(format).packetization_mode),
      format_(format) {
  std::string hexString = format_.parameters.at("profile-level-id");
  std::optional<webrtc::H264ProfileLevelId> profile_level_id =
      webrtc::ParseH264ProfileLevelId(hexString.c_str());
  if (profile_level_id.has_value()) {
    profile_ = profile_level_id->profile;
    level_ = profile_level_id->level;
  }
}

VAProfile VAAPIH264EncoderWrapper::GetVAProfile() const {
  switch (profile_) {
    case H264Profile::kProfileConstrainedBaseline:
    case H264Profile::kProfileBaseline:
      return VAProfileH264ConstrainedBaseline;

    case H264Profile::kProfileMain:
      return VAProfileH264Main;

    case H264Profile::kProfileConstrainedHigh:
    case H264Profile::kProfileHigh:
      return VAProfileH264High;
  }
  return VAProfileNone;
}

VAAPIH264EncoderWrapper::~VAAPIH264EncoderWrapper() {
  Release();
}

void VAAPIH264EncoderWrapper::ReportInit() {
  if (has_reported_init_)
    return;
  RTC_HISTOGRAM_ENUMERATION("WebRTC.Video.H264EncoderImpl.Event",
                            kH264EncoderEventInit, kH264EncoderEventMax);
  has_reported_init_ = true;
}

void VAAPIH264EncoderWrapper::ReportError() {
  if (has_reported_error_)
    return;
  RTC_HISTOGRAM_ENUMERATION("WebRTC.Video.H264EncoderImpl.Event",
                            kH264EncoderEventError, kH264EncoderEventMax);
  has_reported_error_ = true;
}

int32_t VAAPIH264EncoderWrapper::InitEncode(
    const VideoCodec* inst,
    const VideoEncoder::Settings& settings) {
  if (!inst || inst->codecType != kVideoCodecH264) {
    ReportError();
    return WEBRTC_VIDEO_CODEC_ERR_PARAMETER;
  }
  if (inst->maxFramerate == 0) {
    ReportError();
    return WEBRTC_VIDEO_CODEC_ERR_PARAMETER;
  }
  if (inst->width < 1 || inst->height < 1) {
    ReportError();
    return WEBRTC_VIDEO_CODEC_ERR_PARAMETER;
  }

  int32_t release_ret = Release();
  if (release_ret != WEBRTC_VIDEO_CODEC_OK) {
    ReportError();
    return release_ret;
  }

  codec_ = *inst;

  // Code expects simulcastStream resolutions to be correct, make sure they are
  // filled even when there are no simulcast layers.
  if (codec_.numberOfSimulcastStreams == 0) {
    codec_.simulcastStream[0].width = codec_.width;
    codec_.simulcastStream[0].height = codec_.height;
  }

  // Initialize encoded image. Default buffer size: size of unencoded data.
  const size_t new_capacity =
      CalcBufferSize(VideoType::kI420, codec_.width, codec_.height);
  encoded_image_.SetEncodedData(EncodedImageBuffer::Create(new_capacity));
  encoded_image_._encodedWidth = codec_.width;
  encoded_image_._encodedHeight = codec_.height;
  encoded_image_.set_size(0);

  configuration_.sending = false;
  configuration_.frame_dropping_on = codec_.GetFrameDropEnabled();
  configuration_.key_frame_interval = codec_.H264()->keyFrameInterval;

  configuration_.width = codec_.width;
  configuration_.height = codec_.height;

  configuration_.max_frame_rate = codec_.maxFramerate;
  configuration_.target_bps = codec_.startBitrate * 1000;
  configuration_.max_bps = codec_.maxBitrate * 1000;

  if (!encoder_->IsInitialized()) {
    // Initialize encoder.
    int keyFrameInterval = 60;
    if (codec_.maxFramerate > 0) {
      keyFrameInterval = codec_.maxFramerate * 5;
    }
    auto va_profile = GetVAProfile();
    if (va_profile == VAProfileNone) {
      RTC_LOG(LS_ERROR) << "Unsupported H264 profile: "
                        << static_cast<int>(profile_);
      ReportError();
      return WEBRTC_VIDEO_CODEC_ERR_PARAMETER;
    }
    encoder_->Initialize(codec_.width, codec_.height,
                         codec_.startBitrate * 1000, keyFrameInterval,
                         keyFrameInterval, 1, codec_.maxFramerate,
                         va_profile, VA_RC_CBR);
  }

  SimulcastRateAllocator init_allocator(env_, codec_);
  VideoBitrateAllocation allocation =
      init_allocator.Allocate(VideoBitrateAllocationParameters(
          DataRate::KilobitsPerSec(codec_.startBitrate), codec_.maxFramerate));
  SetRates(RateControlParameters(allocation, codec_.maxFramerate));
  return WEBRTC_VIDEO_CODEC_OK;
}

int32_t VAAPIH264EncoderWrapper::RegisterEncodeCompleteCallback(
    EncodedImageCallback* callback) {
  encoded_image_callback_ = callback;
  return WEBRTC_VIDEO_CODEC_OK;
}

int32_t VAAPIH264EncoderWrapper::Release() {
  if (encoder_->IsInitialized()) {
    encoder_->Destroy();
  }
  return WEBRTC_VIDEO_CODEC_OK;
}

int32_t VAAPIH264EncoderWrapper::Encode(
    const VideoFrame& input_frame,
    const std::vector<VideoFrameType>* frame_types) {
  if (!encoder_) {
    ReportError();
    return WEBRTC_VIDEO_CODEC_UNINITIALIZED;
  }
  if (!encoded_image_callback_) {
    RTC_LOG(LS_WARNING)
        << "InitEncode() has been called, but a callback function "
           "has not been set with RegisterEncodeCompleteCallback()";
    ReportError();
    return WEBRTC_VIDEO_CODEC_UNINITIALIZED;
  }

  webrtc::scoped_refptr<I420BufferInterface> frame_buffer =
      input_frame.video_frame_buffer()->ToI420();
  if (!frame_buffer) {
    RTC_LOG(LS_ERROR) << "Failed to convert "
                      << VideoFrameBufferTypeToString(
                             input_frame.video_frame_buffer()->type())
                      << " image to I420. Can't encode frame.";
    return WEBRTC_VIDEO_CODEC_ENCODER_FAILURE;
  }
  RTC_CHECK(frame_buffer->type() == VideoFrameBuffer::Type::kI420 ||
            frame_buffer->type() == VideoFrameBuffer::Type::kI420A);

  bool is_keyframe_needed = false;
  if (configuration_.key_frame_request && configuration_.sending) {
    is_keyframe_needed = true;
  }

  bool send_key_frame =
      is_keyframe_needed ||
      (frame_types && (*frame_types)[0] == VideoFrameType::kVideoFrameKey);
  if (send_key_frame) {
    is_keyframe_needed = true;
    configuration_.key_frame_request = false;
  }

  RTC_DCHECK_EQ(configuration_.width, frame_buffer->width());
  RTC_DCHECK_EQ(configuration_.height, frame_buffer->height());

  if (!configuration_.sending) {
    return WEBRTC_VIDEO_CODEC_NO_OUTPUT;
  }

  if (frame_types != nullptr) {
    // Skip frame?
    if ((*frame_types)[0] == VideoFrameType::kEmptyFrame) {
      return WEBRTC_VIDEO_CODEC_NO_OUTPUT;
    }
  }

  std::vector<uint8_t> output;
  encoder_->Encode(VA_FOURCC_I420, frame_buffer->DataY(), frame_buffer->DataU(),
                   frame_buffer->DataV(), send_key_frame, output);

  if (output.empty()) {
    RTC_LOG(LS_ERROR) << "Failed to encode frame.";
    return WEBRTC_VIDEO_CODEC_ERROR;
  }

  encoded_image_.SetEncodedData(
      EncodedImageBuffer::Create(output.data(), output.size()));

  h264_bitstream_parser_.ParseBitstream(encoded_image_);

  encoded_image_.qp_ = h264_bitstream_parser_.GetLastSliceQp().value_or(-1);

  encoded_image_._encodedWidth = configuration_.width;
  encoded_image_._encodedHeight = configuration_.height;
  encoded_image_.SetRtpTimestamp(input_frame.rtp_timestamp());
  encoded_image_.SetColorSpace(input_frame.color_space());
  encoded_image_._frameType = send_key_frame ? VideoFrameType::kVideoFrameKey
                                             : VideoFrameType::kVideoFrameDelta;
  CodecSpecificInfo codec_specific;
  codec_specific.codecType = kVideoCodecH264;
  codec_specific.codecSpecific.H264.packetization_mode = packetization_mode_;
  codec_specific.codecSpecific.H264.temporal_idx = kNoTemporalIdx;
  codec_specific.codecSpecific.H264.base_layer_sync = false;
  codec_specific.codecSpecific.H264.idr_frame = send_key_frame;
  encoded_image_callback_->OnEncodedImage(encoded_image_, &codec_specific);

  return WEBRTC_VIDEO_CODEC_OK;
}

VideoEncoder::EncoderInfo VAAPIH264EncoderWrapper::GetEncoderInfo() const {
  EncoderInfo info;
  info.supports_native_handle = false;
  info.implementation_name = "VAAPI H264 Encoder";
  info.scaling_settings = VideoEncoder::ScalingSettings::kOff;
  info.is_hardware_accelerated = true;
  info.supports_simulcast = false;
  info.preferred_pixel_formats = {VideoFrameBuffer::Type::kI420};
  return info;
}

void VAAPIH264EncoderWrapper::SetRates(
    const RateControlParameters& parameters) {
  if (!encoder_) {
    RTC_LOG(LS_WARNING) << "SetRates() while uninitialized.";
    return;
  }

  if (parameters.framerate_fps < 1.0) {
    RTC_LOG(LS_WARNING) << "Invalid frame rate: " << parameters.framerate_fps;
    return;
  }

  if (parameters.bitrate.get_sum_bps() == 0) {
    configuration_.SetStreamState(false);
    return;
  }

  codec_.maxFramerate = static_cast<uint32_t>(parameters.framerate_fps);

  configuration_.target_bps = parameters.bitrate.GetSpatialLayerSum(0);
  configuration_.max_frame_rate = parameters.framerate_fps;

  if (configuration_.target_bps) {
    configuration_.SetStreamState(true);
    // Update max_frame_rate/target_bitrate for vaapi encoder.
    encoder_->UpdateRates(configuration_.max_frame_rate,
                          configuration_.target_bps);
  } else {
    configuration_.SetStreamState(false);
  }
}

void VAAPIH264EncoderWrapper::LayerConfig::SetStreamState(bool send_stream) {
  if (send_stream && !sending) {
    // Need a key frame if we have not sent this stream before.
    key_frame_request = true;
  }
  sending = send_stream;
}

}  // namespace webrtc
