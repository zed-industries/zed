#ifndef VAAPI_H264_ENCODER_WRAPPER_H_
#define VAAPI_H264_ENCODER_WRAPPER_H_

#include <stdbool.h>
#include <stdint.h>
#include <va/va.h>
#include <va/va_enc_h264.h>

#include <memory>
#include <vector>

#if defined(WIN32)
#include "vaapi_display_win32.h"
using VaapiDisplay = livekit_ffi::VaapiDisplayWin32;
#elif defined(__linux__)
#include "vaapi_display_drm.h"
using VaapiDisplay = livekit_ffi::VaapiDisplayDrm ;
#endif
#define SURFACE_NUM 16 /* 16 surfaces for reference */

typedef struct {
  // one of: VAProfileH264ConstrainedBaseline, VAProfileH264Main,
  // VAProfileH264High
  VAProfile h264_profile;
  int h264_entropy_mode;
  int frame_width;
  int frame_height;
  int frame_rate;
  uint32_t bitrate;
  int initial_qp;
  int minimal_qp;
  int intra_period;
  int intra_idr_period;
  int ip_period;
  int rc_mode;
} VA264Config;

typedef struct {
  VADisplay va_dpy;

  VAConfigAttrib attrib[VAConfigAttribTypeMax];
  VAConfigAttrib config_attrib[VAConfigAttribTypeMax];
  int config_attrib_num;
  int enc_packed_header_idx;
  VASurfaceID src_surface[SURFACE_NUM];
  VABufferID coded_buf[SURFACE_NUM];
  VASurfaceID ref_surface[SURFACE_NUM];
  VAConfigID config_id;
  VAContextID context_id;
  VAEncSequenceParameterBufferH264 seq_param;
  VAEncPictureParameterBufferH264 pic_param;
  VAEncSliceParameterBufferH264 slice_param;
  VAPictureH264 current_curr_pic;
  VAPictureH264 reference_frames[SURFACE_NUM];
  VAPictureH264 ref_pic_list0_p[SURFACE_NUM * 2];
  VAPictureH264 ref_pic_list0_b[SURFACE_NUM * 2];
  VAPictureH264 ref_pic_list1_b[SURFACE_NUM * 2];

  // Default entrypoint for Encode
  int requested_entrypoint;
  int selected_entrypoint;

  uint32_t num_short_term;
  int constraint_set_flag;
  int h264_packedheader; /* support pack header? */
  int h264_maxref;
  int frame_width_mbaligned;
  int frame_height_mbaligned;
  uint32_t current_frame_num;
  int current_frame_type;
  uint64_t current_frame_encoding;
  uint64_t current_frame_display;
  uint64_t current_idr_display;

  uint8_t* encoded_buffer;
  VA264Config config;
} VA264Context;

namespace livekit_ffi {

class VaapiH264EncoderWrapper {
 public:
  VaapiH264EncoderWrapper();
  ~VaapiH264EncoderWrapper();

  // Initialize the encoder with the given parameters.
  bool Initialize(int width,
                  int height,
                  int bitrate,
                  int intra_period,
                  int idr_period,
                  int ip_period,
                  int frame_rate,
                  VAProfile profile,
                  int rc_mode);

  // Encode a frame and return the encoded data.
  bool Encode(int fourcc,
              const uint8_t* y,
              const uint8_t* u,
              const uint8_t* v,
              bool forceIDR,
              std::vector<uint8_t>& output);

  void UpdateRates(int frame_rate, int bitrate) {
    if (context_) {
      context_->config.frame_rate = frame_rate;
      context_->config.bitrate = bitrate;
    }
  }

  bool IsInitialized() const {
    return initialized_;
  }

  // Release resources.
  void Destroy();

 private:
  std::unique_ptr<VA264Context> context_;
  std::unique_ptr<VaapiDisplay> va_display_;
  bool initialized_ = false;
};

}  // namespace livekit_ffi

#endif  // VAAPI_H264_ENCODER_WRAPPER_H_
