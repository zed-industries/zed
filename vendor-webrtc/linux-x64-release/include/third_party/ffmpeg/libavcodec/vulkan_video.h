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

#ifndef AVCODEC_VULKAN_VIDEO_H
#define AVCODEC_VULKAN_VIDEO_H

#include "avcodec.h"
#include "libavutil/vulkan.h"

#include <vk_video/vulkan_video_codecs_common.h>

#define CODEC_VER_MAJ(ver) (ver >> 22)
#define CODEC_VER_MIN(ver) ((ver >> 12) & ((1 << 10) - 1))
#define CODEC_VER_PAT(ver) (ver & ((1 << 12) - 1))
#define CODEC_VER(ver) CODEC_VER_MAJ(ver), CODEC_VER_MIN(ver), CODEC_VER_PAT(ver)

typedef struct FFVkVideoSession {
    VkVideoSessionKHR session;
    VkDeviceMemory *mem;
    uint32_t nb_mem;

    VkSamplerYcbcrConversion yuv_sampler;

    AVBufferRef *dpb_hwfc_ref;
    int layered_dpb;
    AVFrame *layered_frame;
    VkImageView layered_view;
    VkImageAspectFlags layered_aspect;
} FFVkVideoCommon;

/**
 * Get pixfmt from a Vulkan format.
 */
enum AVPixelFormat ff_vk_pix_fmt_from_vkfmt(VkFormat vkf);

/**
 * Get aspect bits which include all planes from a VkFormat.
 */
VkImageAspectFlags ff_vk_aspect_bits_from_vkfmt(VkFormat vkf);

/**
 * Get Vulkan's chroma subsampling from a pixfmt descriptor.
 */
VkVideoChromaSubsamplingFlagBitsKHR ff_vk_subsampling_from_av_desc(const AVPixFmtDescriptor *desc);

/**
 * Get Vulkan's bit depth from an [8:12] integer.
 */
VkVideoComponentBitDepthFlagBitsKHR ff_vk_depth_from_av_depth(int depth);

/**
 * Convert level from Vulkan to AV.
 */
int ff_vk_h264_level_to_av(StdVideoH264LevelIdc level);
int ff_vk_h265_level_to_av(StdVideoH265LevelIdc level);

StdVideoH264LevelIdc ff_vk_h264_level_to_vk(int level_idc);
StdVideoH265LevelIdc ff_vk_h265_level_to_vk(int level_idc);

/**
 * Convert profile from/to AV to Vulkan
 */
StdVideoH264ProfileIdc ff_vk_h264_profile_to_vk(int profile);
StdVideoH265ProfileIdc ff_vk_h265_profile_to_vk(int profile);
int ff_vk_h264_profile_to_av(StdVideoH264ProfileIdc profile);
int ff_vk_h265_profile_to_av(StdVideoH264ProfileIdc profile);

/**
 * Creates image views for video frames.
 */
int ff_vk_create_view(FFVulkanContext *s, FFVkVideoCommon *common,
                      VkImageView *view, VkImageAspectFlags *aspect,
                      AVVkFrame *src, VkFormat vkf, int is_dpb);

/**
 * Initialize video session, allocating and binding necessary memory.
 */
int ff_vk_video_common_init(AVCodecContext *avctx, FFVulkanContext *s,
                            FFVkVideoCommon *common,
                            VkVideoSessionCreateInfoKHR *session_create);

/**
 * Free video session and required resources.
 */
void ff_vk_video_common_uninit(FFVulkanContext *s, FFVkVideoCommon *common);

#endif /* AVCODEC_VULKAN_VIDEO_H */
