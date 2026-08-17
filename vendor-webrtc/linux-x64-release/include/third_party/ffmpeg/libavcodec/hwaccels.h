/*
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with FFmpeg; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

#ifndef AVCODEC_HWACCELS_H
#define AVCODEC_HWACCELS_H

extern const struct FFHWAccel ff_av1_d3d11va_hwaccel;
extern const struct FFHWAccel ff_av1_d3d11va2_hwaccel;
extern const struct FFHWAccel ff_av1_d3d12va_hwaccel;
extern const struct FFHWAccel ff_av1_dxva2_hwaccel;
extern const struct FFHWAccel ff_av1_nvdec_hwaccel;
extern const struct FFHWAccel ff_av1_vaapi_hwaccel;
extern const struct FFHWAccel ff_av1_vdpau_hwaccel;
extern const struct FFHWAccel ff_av1_videotoolbox_hwaccel;
extern const struct FFHWAccel ff_av1_vulkan_hwaccel;
extern const struct FFHWAccel ff_ffv1_vulkan_hwaccel;
extern const struct FFHWAccel ff_h263_vaapi_hwaccel;
extern const struct FFHWAccel ff_h263_videotoolbox_hwaccel;
extern const struct FFHWAccel ff_h264_d3d11va_hwaccel;
extern const struct FFHWAccel ff_h264_d3d11va2_hwaccel;
extern const struct FFHWAccel ff_h264_d3d12va_hwaccel;
extern const struct FFHWAccel ff_h264_dxva2_hwaccel;
extern const struct FFHWAccel ff_h264_nvdec_hwaccel;
extern const struct FFHWAccel ff_h264_vaapi_hwaccel;
extern const struct FFHWAccel ff_h264_vdpau_hwaccel;
extern const struct FFHWAccel ff_h264_videotoolbox_hwaccel;
extern const struct FFHWAccel ff_h264_vulkan_hwaccel;
extern const struct FFHWAccel ff_hevc_d3d11va_hwaccel;
extern const struct FFHWAccel ff_hevc_d3d11va2_hwaccel;
extern const struct FFHWAccel ff_hevc_d3d12va_hwaccel;
extern const struct FFHWAccel ff_hevc_dxva2_hwaccel;
extern const struct FFHWAccel ff_hevc_nvdec_hwaccel;
extern const struct FFHWAccel ff_hevc_vaapi_hwaccel;
extern const struct FFHWAccel ff_hevc_vdpau_hwaccel;
extern const struct FFHWAccel ff_hevc_videotoolbox_hwaccel;
extern const struct FFHWAccel ff_hevc_vulkan_hwaccel;
extern const struct FFHWAccel ff_mjpeg_nvdec_hwaccel;
extern const struct FFHWAccel ff_mjpeg_vaapi_hwaccel;
extern const struct FFHWAccel ff_mpeg1_nvdec_hwaccel;
extern const struct FFHWAccel ff_mpeg1_vdpau_hwaccel;
extern const struct FFHWAccel ff_mpeg1_videotoolbox_hwaccel;
extern const struct FFHWAccel ff_mpeg2_d3d11va_hwaccel;
extern const struct FFHWAccel ff_mpeg2_d3d11va2_hwaccel;
extern const struct FFHWAccel ff_mpeg2_d3d12va_hwaccel;
extern const struct FFHWAccel ff_mpeg2_dxva2_hwaccel;
extern const struct FFHWAccel ff_mpeg2_nvdec_hwaccel;
extern const struct FFHWAccel ff_mpeg2_vaapi_hwaccel;
extern const struct FFHWAccel ff_mpeg2_vdpau_hwaccel;
extern const struct FFHWAccel ff_mpeg2_videotoolbox_hwaccel;
extern const struct FFHWAccel ff_mpeg4_nvdec_hwaccel;
extern const struct FFHWAccel ff_mpeg4_vaapi_hwaccel;
extern const struct FFHWAccel ff_mpeg4_vdpau_hwaccel;
extern const struct FFHWAccel ff_mpeg4_videotoolbox_hwaccel;
extern const struct FFHWAccel ff_prores_videotoolbox_hwaccel;
extern const struct FFHWAccel ff_vc1_d3d11va_hwaccel;
extern const struct FFHWAccel ff_vc1_d3d11va2_hwaccel;
extern const struct FFHWAccel ff_vc1_d3d12va_hwaccel;
extern const struct FFHWAccel ff_vc1_dxva2_hwaccel;
extern const struct FFHWAccel ff_vc1_nvdec_hwaccel;
extern const struct FFHWAccel ff_vc1_vaapi_hwaccel;
extern const struct FFHWAccel ff_vc1_vdpau_hwaccel;
extern const struct FFHWAccel ff_vp8_nvdec_hwaccel;
extern const struct FFHWAccel ff_vp8_vaapi_hwaccel;
extern const struct FFHWAccel ff_vp9_d3d11va_hwaccel;
extern const struct FFHWAccel ff_vp9_d3d11va2_hwaccel;
extern const struct FFHWAccel ff_vp9_d3d12va_hwaccel;
extern const struct FFHWAccel ff_vp9_dxva2_hwaccel;
extern const struct FFHWAccel ff_vp9_nvdec_hwaccel;
extern const struct FFHWAccel ff_vp9_vaapi_hwaccel;
extern const struct FFHWAccel ff_vp9_vdpau_hwaccel;
extern const struct FFHWAccel ff_vp9_videotoolbox_hwaccel;
extern const struct FFHWAccel ff_vvc_vaapi_hwaccel;
extern const struct FFHWAccel ff_wmv3_d3d11va_hwaccel;
extern const struct FFHWAccel ff_wmv3_d3d11va2_hwaccel;
extern const struct FFHWAccel ff_wmv3_d3d12va_hwaccel;
extern const struct FFHWAccel ff_wmv3_dxva2_hwaccel;
extern const struct FFHWAccel ff_wmv3_nvdec_hwaccel;
extern const struct FFHWAccel ff_wmv3_vaapi_hwaccel;
extern const struct FFHWAccel ff_wmv3_vdpau_hwaccel;

#endif /* AVCODEC_HWACCELS_H */
