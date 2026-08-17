#ifndef WEBRTC_NVIDIA_H264_ENCODER_IMPL_H_
#define WEBRTC_NVIDIA_H264_ENCODER_IMPL_H_

#include <cuda.h>

#include <memory>
#include <vector>

#include "NvEncoder/NvEncoder.h"
#include "NvEncoder/NvEncoderCuda.h"

#include "absl/container/inlined_vector.h"
#include "api/transport/rtp/dependency_descriptor.h"
#include "api/video/i420_buffer.h"
#include "api/video/video_codec_constants.h"
#include "api/video_codecs/scalability_mode.h"
#include "api/video_codecs/video_encoder.h"
#include "common_video/h264/h264_bitstream_parser.h"
#include "modules/video_coding/codecs/h264/include/h264.h"
#include "modules/video_coding/svc/scalable_video_controller.h"
#include "modules/video_coding/utility/quality_scaler.h"

namespace webrtc {

class NvidiaH264EncoderImpl : public VideoEncoder {
 public:
  struct LayerConfig {
    int simulcast_idx = 0;
    int width = -1;
    int height = -1;
    bool sending = true;
    bool key_frame_request = false;
    float max_frame_rate = 0;
    uint32_t target_bps = 0;
    uint32_t max_bps = 0;
    bool frame_dropping_on = false;
    int key_frame_interval = 0;
    int num_temporal_layers = 1;

    void SetStreamState(bool send_stream);
  };

 public:
  NvidiaH264EncoderImpl(const webrtc::Environment& env,
                           CUcontext context,
                           CUmemorytype memory_type,
                           NV_ENC_BUFFER_FORMAT nv_format,
                           const SdpVideoFormat& format);
  ~NvidiaH264EncoderImpl() override;

  int32_t InitEncode(const VideoCodec* codec_settings,
                     const Settings& settings) override;

  int32_t RegisterEncodeCompleteCallback(
      EncodedImageCallback* callback) override;

  int32_t Release() override;

  int32_t Encode(const VideoFrame& frame,
                 const std::vector<VideoFrameType>* frame_types) override;

  void SetRates(const RateControlParameters& rc_parameters) override;

  EncoderInfo GetEncoderInfo() const override;

 private:
  int32_t ProcessEncodedFrame(std::vector<uint8_t>& packet,
                              const ::webrtc::VideoFrame& inputFrame);
 private:
  const webrtc::Environment& env_;
  EncodedImageCallback* encoded_image_callback_ = nullptr;

  std::unique_ptr<NvEncoder> encoder_;
  CUcontext cu_context_;
  CUmemorytype cu_memory_type_;
  CUarray cu_scaled_array_;
  NV_ENC_BUFFER_FORMAT nv_format_;
  NV_ENC_INITIALIZE_PARAMS nv_initialize_params_;
  NV_ENC_CONFIG nv_encode_config_;
  GUID nv_profile_guid_;
  NV_ENC_LEVEL nv_enc_level_;

  LayerConfig configuration_;
  EncodedImage encoded_image_;
  H264PacketizationMode packetization_mode_;
  VideoCodec codec_;
  void ReportInit();
  void ReportError();
  bool has_reported_init_ = false;
  bool has_reported_error_ = false;
  webrtc::H264BitstreamParser h264_bitstream_parser_;
  const SdpVideoFormat format_;
  H264Profile profile_ = H264Profile::kProfileConstrainedBaseline;
  H264Level level_ = H264Level::kLevel1_b;
};

}  // namespace webrtc

#endif  // WEBRTC_NVIDIA_H264_ENCODER_IMPL_H_
