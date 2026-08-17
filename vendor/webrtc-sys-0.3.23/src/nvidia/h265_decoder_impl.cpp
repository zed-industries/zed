#include "h265_decoder_impl.h"

#include <api/video/i420_buffer.h>
#include <api/video/video_codec_type.h>
#include <modules/video_coding/include/video_error_codes.h>
#include <third_party/libyuv/include/libyuv/convert.h>

#include "NvDecoder/NvDecoder.h"
#include "Utils/NvCodecUtils.h"
#include "rtc_base/checks.h"
#include "rtc_base/logging.h"

namespace webrtc {

static ColorSpace ExtractColorSpaceFromFormat(const CUVIDEOFORMAT& format) {
  return ColorSpace(
      static_cast<ColorSpace::PrimaryID>(
          format.video_signal_description.color_primaries),
      static_cast<ColorSpace::TransferID>(
          format.video_signal_description.transfer_characteristics),
      static_cast<ColorSpace::MatrixID>(
          format.video_signal_description.matrix_coefficients),
      static_cast<ColorSpace::RangeID>(
          format.video_signal_description.video_full_range_flag));
}

NvidiaH265DecoderImpl::NvidiaH265DecoderImpl(CUcontext context)
    : cu_context_(context),
      decoder_(nullptr),
      is_configured_decoder_(false),
      decoded_complete_callback_(nullptr),
      buffer_pool_(false) {}

NvidiaH265DecoderImpl::~NvidiaH265DecoderImpl() { Release(); }

VideoDecoder::DecoderInfo NvidiaH265DecoderImpl::GetDecoderInfo() const {
  VideoDecoder::DecoderInfo info;
  info.implementation_name = "NVIDIA H265 Decoder";
  info.is_hardware_accelerated = true;
  return info;
}

bool NvidiaH265DecoderImpl::Configure(const Settings& settings) {
  if (settings.codec_type() != kVideoCodecH265) {
    RTC_LOG(LS_ERROR) << "initialization failed: codec type is not H265";
    return false;
  }
  if (!settings.max_render_resolution().Valid()) {
    RTC_LOG(LS_ERROR)
        << "initialization failed on codec_settings width < 0 or height < 0";
    return false;
  }

  settings_ = settings;

  const CUresult result = cuCtxSetCurrent(cu_context_);
  if (!ck(result)) {
    RTC_LOG(LS_ERROR) << "initialization failed on cuCtxSetCurrent result"
                      << result;
    return false;
  }

  int maxWidth = 4096;
  int maxHeight = 4096;

  decoder_ = std::make_unique<NvDecoder>(
      cu_context_, false, cudaVideoCodec_HEVC, true, false, nullptr, nullptr,
      false, maxWidth, maxHeight);
  return true;
}

int32_t NvidiaH265DecoderImpl::RegisterDecodeCompleteCallback(
    DecodedImageCallback* callback) {
  decoded_complete_callback_ = callback;
  return WEBRTC_VIDEO_CODEC_OK;
}

int32_t NvidiaH265DecoderImpl::Release() {
  buffer_pool_.Release();
  return WEBRTC_VIDEO_CODEC_OK;
}

int32_t NvidiaH265DecoderImpl::Decode(const EncodedImage& input_image,
                                      bool /*missing_frames*/,
                                      int64_t /*render_time_ms*/) {
  CUcontext current;
  if (!ck(cuCtxGetCurrent(&current))) {
    RTC_LOG(LS_ERROR) << "decode failed on cuCtxGetCurrent";
    return WEBRTC_VIDEO_CODEC_UNINITIALIZED;
  }
  if (current != cu_context_) {
    RTC_LOG(LS_ERROR)
        << "decode failed: current context does not match held context";
    return WEBRTC_VIDEO_CODEC_UNINITIALIZED;
  }
  if (decoded_complete_callback_ == nullptr) {
    RTC_LOG(LS_ERROR) << "decode failed: decoded_complete_callback_ not set";
    return WEBRTC_VIDEO_CODEC_UNINITIALIZED;
  }
  if (!input_image.data() || !input_image.size()) {
    RTC_LOG(LS_ERROR) << "decode failed: input image is null";
    return WEBRTC_VIDEO_CODEC_ERR_PARAMETER;
  }

  int nFrameReturned = 0;
  do {
    nFrameReturned = decoder_->Decode(
        input_image.data(), static_cast<int>(input_image.size()),
        CUVID_PKT_TIMESTAMP, input_image.RtpTimestamp());
  } while (nFrameReturned == 0);

  is_configured_decoder_ = true;

  if (decoder_->GetOutputFormat() != cudaVideoSurfaceFormat_NV12) {
    RTC_LOG(LS_ERROR) << "not supported output format: "
                      << decoder_->GetOutputFormat();
    return WEBRTC_VIDEO_CODEC_ERR_PARAMETER;
  }

  const ColorSpace& color_space =
      input_image.ColorSpace() ? *input_image.ColorSpace()
                               : ExtractColorSpaceFromFormat(
                                     decoder_->GetVideoFormatInfo());

  for (int i = 0; i < nFrameReturned; i++) {
    int64_t timeStamp;
    uint8_t* pFrame = decoder_->GetFrame(&timeStamp);

    webrtc::scoped_refptr<webrtc::I420Buffer> i420_buffer =
        buffer_pool_.CreateI420Buffer(decoder_->GetWidth(),
                                      decoder_->GetHeight());

    int result = libyuv::NV12ToI420(
        pFrame, decoder_->GetDeviceFramePitch(),
        pFrame + decoder_->GetHeight() * decoder_->GetDeviceFramePitch(),
        decoder_->GetDeviceFramePitch(), i420_buffer->MutableDataY(),
        i420_buffer->StrideY(), i420_buffer->MutableDataU(),
        i420_buffer->StrideU(), i420_buffer->MutableDataV(),
        i420_buffer->StrideV(), decoder_->GetWidth(), decoder_->GetHeight());

    if (result) {
      RTC_LOG(LS_INFO) << "libyuv::NV12ToI420 failed. error:" << result;
    }

    VideoFrame decoded_frame = VideoFrame::Builder()
                                   .set_video_frame_buffer(i420_buffer)
                                   .set_timestamp_rtp(static_cast<uint32_t>(
                                       timeStamp))
                                   .set_color_space(color_space)
                                   .build();

    std::optional<int32_t> decodetime;
    std::optional<int> qp;  // Not parsed for H265 currently
    decoded_complete_callback_->Decoded(decoded_frame, decodetime, qp);
  }

  return WEBRTC_VIDEO_CODEC_OK;
}

}  // end namespace webrtc


