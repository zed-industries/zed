/*
 * Copyright 2022 LiveKit
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#include "libyuv/convert.h"
#include "rtc_base/system/rtc_export.h"
#include "stdint.h"
#include "third_party/libyuv/include/libyuv.h"
#include "video_rotation.h"

namespace webrtc {

RTC_EXPORT int I420Rotate(const uint8_t* src_y,
                          int src_stride_y,
                          const uint8_t* src_u,
                          int src_stride_u,
                          const uint8_t* src_v,
                          int src_stride_v,
                          uint8_t* dst_y,
                          int dst_stride_y,
                          uint8_t* dst_u,
                          int dst_stride_u,
                          uint8_t* dst_v,
                          int dst_stride_v,
                          int width,
                          int height,
                          VideoRotation mode);

RTC_EXPORT int I420ToNV12(const uint8_t* src_y,
                          int src_stride_y,
                          const uint8_t* src_u,
                          int src_stride_u,
                          const uint8_t* src_v,
                          int src_stride_v,
                          uint8_t* dst_y,
                          int dst_stride_y,
                          uint8_t* dst_uv,
                          int dst_stride_uv,
                          int width,
                          int height);

RTC_EXPORT int I420ToARGB(const uint8_t* src_y,
                          int src_stride_y,
                          const uint8_t* src_u,
                          int src_stride_u,
                          const uint8_t* src_v,
                          int src_stride_v,
                          uint8_t* dst_argb,
                          int dst_stride_argb,
                          int width,
                          int height);

RTC_EXPORT int I420ToBGRA(const uint8_t* src_y,
                          int src_stride_y,
                          const uint8_t* src_u,
                          int src_stride_u,
                          const uint8_t* src_v,
                          int src_stride_v,
                          uint8_t* dst_bgra,
                          int dst_stride_bgra,
                          int width,
                          int height);

RTC_EXPORT int I420ToABGR(const uint8_t* src_y,
                          int src_stride_y,
                          const uint8_t* src_u,
                          int src_stride_u,
                          const uint8_t* src_v,
                          int src_stride_v,
                          uint8_t* dst_abgr,
                          int dst_stride_abgr,
                          int width,
                          int height);

RTC_EXPORT int I420ToRGBA(const uint8_t* src_y,
                          int src_stride_y,
                          const uint8_t* src_u,
                          int src_stride_u,
                          const uint8_t* src_v,
                          int src_stride_v,
                          uint8_t* dst_rgba,
                          int dst_stride_rgba,
                          int width,
                          int height);

RTC_EXPORT int I420ToRGB24(const uint8_t* src_y,
                           int src_stride_y,
                           const uint8_t* src_u,
                           int src_stride_u,
                           const uint8_t* src_v,
                           int src_stride_v,
                           uint8_t* dst_rgb24,
                           int dst_stride_rgb24,
                           int width,
                           int height);

RTC_EXPORT int I420Scale(const uint8_t* src_y,
                         int src_stride_y,
                         const uint8_t* src_u,
                         int src_stride_u,
                         const uint8_t* src_v,
                         int src_stride_v,
                         int src_width,
                         int src_height,
                         uint8_t* dst_y,
                         int dst_stride_y,
                         uint8_t* dst_u,
                         int dst_stride_u,
                         uint8_t* dst_v,
                         int dst_stride_v,
                         int dst_width,
                         int dst_height,
                         libyuv::FilterMode filtering);

RTC_EXPORT int ARGBToI420(const uint8_t* src_argb,
                          int src_stride_argb,
                          uint8_t* dst_y,
                          int dst_stride_y,
                          uint8_t* dst_u,
                          int dst_stride_u,
                          uint8_t* dst_v,
                          int dst_stride_v,
                          int width,
                          int height);

RTC_EXPORT int ABGRToI420(const uint8_t* src_abgr,
                          int src_stride_abgr,
                          uint8_t* dst_y,
                          int dst_stride_y,
                          uint8_t* dst_u,
                          int dst_stride_u,
                          uint8_t* dst_v,
                          int dst_stride_v,
                          int width,
                          int height);

RTC_EXPORT int ARGBToRGB24(const uint8_t* src_argb,
                           int src_stride_argb,
                           uint8_t* dst_rgb24,
                           int dst_stride_rgb24,
                           int width,
                           int height);

RTC_EXPORT int NV12ToI420(const uint8_t* src_y,
                          int src_stride_y,
                          const uint8_t* src_uv,
                          int src_stride_uv,
                          uint8_t* dst_y,
                          int dst_stride_y,
                          uint8_t* dst_u,
                          int dst_stride_u,
                          uint8_t* dst_v,
                          int dst_stride_v,
                          int width,
                          int height);

RTC_EXPORT int I444ToI420(const uint8_t* src_y,
                          int src_stride_y,
                          const uint8_t* src_u,
                          int src_stride_u,
                          const uint8_t* src_v,
                          int src_stride_v,
                          uint8_t* dst_y,
                          int dst_stride_y,
                          uint8_t* dst_u,
                          int dst_stride_u,
                          uint8_t* dst_v,
                          int dst_stride_v,
                          int width,
                          int height);

RTC_EXPORT int I422ToI420(const uint8_t* src_y,
                          int src_stride_y,
                          const uint8_t* src_u,
                          int src_stride_u,
                          const uint8_t* src_v,
                          int src_stride_v,
                          uint8_t* dst_y,
                          int dst_stride_y,
                          uint8_t* dst_u,
                          int dst_stride_u,
                          uint8_t* dst_v,
                          int dst_stride_v,
                          int width,
                          int height);

RTC_EXPORT int I010ToI420(const uint16_t* src_y,
                          int src_stride_y,
                          const uint16_t* src_u,
                          int src_stride_u,
                          const uint16_t* src_v,
                          int src_stride_v,
                          uint8_t* dst_y,
                          int dst_stride_y,
                          uint8_t* dst_u,
                          int dst_stride_u,
                          uint8_t* dst_v,
                          int dst_stride_v,
                          int width,
                          int height);

RTC_EXPORT int NV12ToARGB(const uint8_t* src_y,
                          int src_stride_y,
                          const uint8_t* src_uv,
                          int src_stride_uv,
                          uint8_t* dst_argb,
                          int dst_stride_argb,
                          int width,
                          int height);

RTC_EXPORT int NV12ToABGR(const uint8_t* src_y,
                          int src_stride_y,
                          const uint8_t* src_uv,
                          int src_stride_uv,
                          uint8_t* dst_abgr,
                          int dst_stride_abgr,
                          int width,
                          int height);

RTC_EXPORT int I444ToARGB(const uint8_t* src_y,
                          int src_stride_y,
                          const uint8_t* src_u,
                          int src_stride_u,
                          const uint8_t* src_v,
                          int src_stride_v,
                          uint8_t* dst_abgr,
                          int dst_stride_abgr,
                          int width,
                          int height);

RTC_EXPORT int I444ToABGR(const uint8_t* src_y,
                          int src_stride_y,
                          const uint8_t* src_u,
                          int src_stride_u,
                          const uint8_t* src_v,
                          int src_stride_v,
                          uint8_t* dst_abgr,
                          int dst_stride_abgr,
                          int width,
                          int height);

RTC_EXPORT int I422ToARGB(const uint8_t* src_y,
                          int src_stride_y,
                          const uint8_t* src_u,
                          int src_stride_u,
                          const uint8_t* src_v,
                          int src_stride_v,
                          uint8_t* dst_argb,
                          int dst_stride_argb,
                          int width,
                          int height);

RTC_EXPORT int I422ToABGR(const uint8_t* src_y,
                          int src_stride_y,
                          const uint8_t* src_u,
                          int src_stride_u,
                          const uint8_t* src_v,
                          int src_stride_v,
                          uint8_t* dst_abgr,
                          int dst_stride_abgr,
                          int width,
                          int height);

RTC_EXPORT int I010ToARGB(const uint16_t* src_y,
                          int src_stride_y,
                          const uint16_t* src_u,
                          int src_stride_u,
                          const uint16_t* src_v,
                          int src_stride_v,
                          uint8_t* dst_argb,
                          int dst_stride_argb,
                          int width,
                          int height);

RTC_EXPORT int I010ToABGR(const uint16_t* src_y,
                          int src_stride_y,
                          const uint16_t* src_u,
                          int src_stride_u,
                          const uint16_t* src_v,
                          int src_stride_v,
                          uint8_t* dst_abgr,
                          int dst_stride_abgr,
                          int width,
                          int height);

RTC_EXPORT int ABGRToNV12(const uint8_t* src_abgr,
                          int src_stride_abgr,
                          uint8_t* dst_y,
                          int dst_stride_y,
                          uint8_t* dst_uv,
                          int dst_stride_uv,
                          int width,
                          int height);

RTC_EXPORT int ARGBToNV12(const uint8_t* src_argb,
                          int src_stride_argb,
                          uint8_t* dst_y,
                          int dst_stride_y,
                          uint8_t* dst_uv,
                          int dst_stride_uv,
                          int width,
                          int height);

}  // namespace webrtc
