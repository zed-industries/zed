#include "h264_decoder_impl.h"

#include <api/video/i420_buffer.h>
#include <api/video/video_codec_type.h>
#include <modules/video_coding/include/video_error_codes.h>
#include <third_party/libyuv/include/libyuv/convert.h>

#include "NvDecoder/NvDecoder.h"
#include "Utils/NvCodecUtils.h"
#include "rtc_base/checks.h"
#include "rtc_base/logging.h"

namespace webrtc {

ColorSpace ExtractH264ColorSpace(const CUVIDEOFORMAT& format) {
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

NvidiaH264DecoderImpl::NvidiaH264DecoderImpl(CUcontext context)
    : cu_context_(context),
      decoder_(nullptr),
      is_configured_decoder_(false),
      decoded_complete_callback_(nullptr),
      buffer_pool_(false) {}

NvidiaH264DecoderImpl::~NvidiaH264DecoderImpl() {
  Release();
}

VideoDecoder::DecoderInfo NvidiaH264DecoderImpl::GetDecoderInfo() const {
  VideoDecoder::DecoderInfo info;
  info.implementation_name = "NVIDIA H264 Decoder";
  info.is_hardware_accelerated = true;
  return info;
}

bool NvidiaH264DecoderImpl::Configure(const Settings& settings) {
  if (settings.codec_type() != kVideoCodecH264) {
    RTC_LOG(LS_ERROR)
        << "initialization failed on codectype is not kVideoCodecH264";
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

  // todo(kazuki): Max resolution is differred each architecture.
  // Refer to the table in Video Decoder Capabilities.
  // https://docs.nvidia.com/video-technologies/video-codec-sdk/nvdec-video-decoder-api-prog-guide
  int maxWidth = 4096;
  int maxHeight = 4096;

  // bUseDeviceFrame: allocate in memory or cuda device memory
  decoder_ = std::make_unique<NvDecoder>(
      cu_context_, false, cudaVideoCodec_H264, true, false, nullptr, nullptr,
      false, maxWidth, maxHeight);
  return true;
}

int32_t NvidiaH264DecoderImpl::RegisterDecodeCompleteCallback(
    DecodedImageCallback* callback) {
  this->decoded_complete_callback_ = callback;
  return WEBRTC_VIDEO_CODEC_OK;
}

int32_t NvidiaH264DecoderImpl::Release() {
  buffer_pool_.Release();
  return WEBRTC_VIDEO_CODEC_OK;
}

int32_t NvidiaH264DecoderImpl::Decode(const EncodedImage& input_image,
                                      bool missing_frames,
                                      int64_t render_time_ms) {
  CUcontext current;
  if (!ck(cuCtxGetCurrent(&current))) {
    RTC_LOG(LS_ERROR) << "decode failed on cuCtxGetCurrent is failed";
    return WEBRTC_VIDEO_CODEC_UNINITIALIZED;
  }
  if (current != cu_context_) {
    RTC_LOG(LS_ERROR)
        << "decode failed on not match current context and hold context";
    return WEBRTC_VIDEO_CODEC_UNINITIALIZED;
  }
  if (decoded_complete_callback_ == nullptr) {
    RTC_LOG(LS_ERROR) << "decode failed on not set m_decodedCompleteCallback";
    return WEBRTC_VIDEO_CODEC_UNINITIALIZED;
  }
  if (!input_image.data() || !input_image.size()) {
    RTC_LOG(LS_ERROR) << "decode failed on input image is null";
    return WEBRTC_VIDEO_CODEC_ERR_PARAMETER;
  }

  h264_bitstream_parser_.ParseBitstream(input_image);
  std::optional<int> qp = h264_bitstream_parser_.GetLastSliceQp();
  std::optional<SpsParser::SpsState> sps = h264_bitstream_parser_.sps();

  if (is_configured_decoder_) {
    if (!sps ||
        sps.value().width != static_cast<uint32_t>(decoder_->GetWidth()) ||
        sps.value().height != static_cast<uint32_t>(decoder_->GetHeight())) {
      decoder_->setReconfigParams(nullptr, nullptr);
    }
  }

  int nFrameReturnd = 0;
  do {
    nFrameReturnd = decoder_->Decode(
        input_image.data(), static_cast<int>(input_image.size()),
        CUVID_PKT_TIMESTAMP, input_image.RtpTimestamp());
  } while (nFrameReturnd == 0);

  is_configured_decoder_ = true;

  // todo: support other output format
  // Chromium's H264 Encoder is output on NV12, so currently only NV12 is
  // supported.
  if (decoder_->GetOutputFormat() != cudaVideoSurfaceFormat_NV12) {
    RTC_LOG(LS_ERROR) << "not supported this format: "
                      << decoder_->GetOutputFormat();
    return WEBRTC_VIDEO_CODEC_ERR_PARAMETER;
  }

  // Pass on color space from input frame if explicitly specified.
  const ColorSpace& color_space =
      input_image.ColorSpace()
          ? *input_image.ColorSpace()
          : ExtractH264ColorSpace(decoder_->GetVideoFormatInfo());

  for (int i = 0; i < nFrameReturnd; i++) {
    int64_t timeStamp;
    uint8_t* pFrame = decoder_->GetFrame(&timeStamp);

    webrtc::scoped_refptr<webrtc::I420Buffer> i420_buffer =
        buffer_pool_.CreateI420Buffer(decoder_->GetWidth(),
                                      decoder_->GetHeight());

    int result;
    {
      result = libyuv::NV12ToI420(
          pFrame, decoder_->GetDeviceFramePitch(),
          pFrame + decoder_->GetHeight() * decoder_->GetDeviceFramePitch(),
          decoder_->GetDeviceFramePitch(), i420_buffer->MutableDataY(),
          i420_buffer->StrideY(), i420_buffer->MutableDataU(),
          i420_buffer->StrideU(), i420_buffer->MutableDataV(),
          i420_buffer->StrideV(), decoder_->GetWidth(), decoder_->GetHeight());
    }

    if (result) {
      RTC_LOG(LS_INFO) << "libyuv::NV12ToI420 failed. error:" << result;
    }

    VideoFrame decoded_frame =
        VideoFrame::Builder()
            .set_video_frame_buffer(i420_buffer)
            .set_timestamp_rtp(static_cast<uint32_t>(timeStamp))
            .set_color_space(color_space)
            .build();

    // todo: measurement decoding time
    std::optional<int32_t> decodetime;
    decoded_complete_callback_->Decoded(decoded_frame, decodetime, qp);
  }

  return WEBRTC_VIDEO_CODEC_OK;
}

}  // end namespace webrtc
