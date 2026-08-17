/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_CODECS_INTERFACE_LIBVPX_INTERFACE_H_
#define MODULES_VIDEO_CODING_CODECS_INTERFACE_LIBVPX_INTERFACE_H_

#include <stdint.h>

#include <memory>

#include "vpx/vp8cx.h"
#include "vpx/vpx_codec.h"
#include "vpx/vpx_encoder.h"
#include "vpx/vpx_ext_ratectrl.h"
#include "vpx/vpx_image.h"

namespace webrtc {

// This interface is a proxy to the static libvpx functions, so that they
// can be mocked for testing. Currently supports VP8 encoder functions.
// TODO(sprang): Extend this to VP8 decoder and VP9 encoder/decoder too.
class LibvpxInterface {
 public:
  LibvpxInterface() = default;
  virtual ~LibvpxInterface() = default;

  virtual vpx_image_t* img_alloc(vpx_image_t* img,
                                 vpx_img_fmt_t fmt,
                                 unsigned int d_w,
                                 unsigned int d_h,
                                 unsigned int align) const = 0;
  virtual vpx_image_t* img_wrap(vpx_image_t* img,
                                vpx_img_fmt_t fmt,
                                unsigned int d_w,
                                unsigned int d_h,
                                unsigned int stride_align,
                                unsigned char* img_data) const = 0;
  virtual void img_free(vpx_image_t* img) const = 0;

  virtual vpx_codec_err_t codec_enc_config_set(
      vpx_codec_ctx_t* ctx,
      const vpx_codec_enc_cfg_t* cfg) const = 0;
  virtual vpx_codec_err_t codec_enc_config_default(
      vpx_codec_iface_t* iface,
      vpx_codec_enc_cfg_t* cfg,
      unsigned int usage) const = 0;

  virtual vpx_codec_err_t codec_enc_init(vpx_codec_ctx_t* ctx,
                                         vpx_codec_iface_t* iface,
                                         const vpx_codec_enc_cfg_t* cfg,
                                         vpx_codec_flags_t flags) const = 0;
  virtual vpx_codec_err_t codec_enc_init_multi(vpx_codec_ctx_t* ctx,
                                               vpx_codec_iface_t* iface,
                                               vpx_codec_enc_cfg_t* cfg,
                                               int num_enc,
                                               vpx_codec_flags_t flags,
                                               vpx_rational_t* dsf) const = 0;
  virtual vpx_codec_err_t codec_destroy(vpx_codec_ctx_t* ctx) const = 0;

  virtual vpx_codec_err_t codec_control(vpx_codec_ctx_t* ctx,
                                        vp8e_enc_control_id ctrl_id,
                                        uint32_t param) const = 0;
  virtual vpx_codec_err_t codec_control(vpx_codec_ctx_t* ctx,
                                        vp8e_enc_control_id ctrl_id,
                                        int param) const = 0;
  virtual vpx_codec_err_t codec_control(vpx_codec_ctx_t* ctx,
                                        vp8e_enc_control_id ctrl_id,
                                        int* param) const = 0;
  virtual vpx_codec_err_t codec_control(vpx_codec_ctx_t* ctx,
                                        vp8e_enc_control_id ctrl_id,
                                        vpx_roi_map* param) const = 0;
  virtual vpx_codec_err_t codec_control(vpx_codec_ctx_t* ctx,
                                        vp8e_enc_control_id ctrl_id,
                                        vpx_active_map* param) const = 0;
  virtual vpx_codec_err_t codec_control(vpx_codec_ctx_t* ctx,
                                        vp8e_enc_control_id ctrl_id,
                                        vpx_scaling_mode* param) const = 0;
  virtual vpx_codec_err_t codec_control(vpx_codec_ctx_t* ctx,
                                        vp8e_enc_control_id ctrl_id,
                                        vpx_svc_extra_cfg_t* param) const = 0;
  virtual vpx_codec_err_t codec_control(vpx_codec_ctx_t* ctx,
                                        vp8e_enc_control_id ctrl_id,
                                        vpx_svc_frame_drop_t* param) const = 0;
  virtual vpx_codec_err_t codec_control(vpx_codec_ctx_t* ctx,
                                        vp8e_enc_control_id ctrl_id,
                                        void* param) const = 0;
  virtual vpx_codec_err_t codec_control(vpx_codec_ctx_t* ctx,
                                        vp8e_enc_control_id ctrl_id,
                                        vpx_svc_layer_id_t* param) const = 0;
  virtual vpx_codec_err_t codec_control(
      vpx_codec_ctx_t* ctx,
      vp8e_enc_control_id ctrl_id,
      vpx_svc_ref_frame_config_t* param) const = 0;
  virtual vpx_codec_err_t codec_control(
      vpx_codec_ctx_t* ctx,
      vp8e_enc_control_id ctrl_id,
      vpx_svc_spatial_layer_sync_t* param) const = 0;
  virtual vpx_codec_err_t codec_control(vpx_codec_ctx_t* ctx,
                                        vp8e_enc_control_id ctrl_id,
                                        vpx_rc_funcs_t* param) const = 0;
  virtual vpx_codec_err_t codec_encode(vpx_codec_ctx_t* ctx,
                                       const vpx_image_t* img,
                                       vpx_codec_pts_t pts,
                                       uint64_t duration,
                                       vpx_enc_frame_flags_t flags,
                                       uint64_t deadline) const = 0;

  virtual const vpx_codec_cx_pkt_t* codec_get_cx_data(
      vpx_codec_ctx_t* ctx,
      vpx_codec_iter_t* iter) const = 0;

  virtual const char* codec_error_detail(vpx_codec_ctx_t* ctx) const = 0;
  virtual const char* codec_error(vpx_codec_ctx_t* ctx) const = 0;
  virtual const char* codec_err_to_string(vpx_codec_err_t err) const = 0;

  // Returns interface wrapping the actual libvpx functions.
  static std::unique_ptr<LibvpxInterface> Create();
};

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_CODECS_INTERFACE_LIBVPX_INTERFACE_H_
