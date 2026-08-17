/*
 * Copyright (c) Lynne
 *
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

#ifndef AVFILTER_VULKAN_FILTER_H
#define AVFILTER_VULKAN_FILTER_H

#include "avfilter.h"

#include "libavutil/vulkan.h"

/**
 * General lavfi IO functions
 */
int ff_vk_filter_init         (AVFilterContext *avctx);
int ff_vk_filter_config_input (AVFilterLink   *inlink);
int ff_vk_filter_config_output(AVFilterLink  *outlink);

/**
 * Can be called manually, if not using ff_vk_filter_config_output.
 */
int ff_vk_filter_init_context(AVFilterContext *avctx, FFVulkanContext *s,
                              AVBufferRef *frames_ref,
                              int width, int height, enum AVPixelFormat sw_format);

/**
 * Submit a compute shader with a zero/one input and single out for execution.
 */
int ff_vk_filter_process_simple(FFVulkanContext *vkctx, FFVkExecPool *e,
                                FFVulkanShader *shd, AVFrame *out_f, AVFrame *in_f,
                                VkSampler sampler, void *push_src, size_t push_size);

/**
 * Submit a compute shader with a single in and single out with 2 stages.
 */
int ff_vk_filter_process_2pass(FFVulkanContext *vkctx, FFVkExecPool *e,
                               FFVulkanShader *shd_list[2],
                               AVFrame *out, AVFrame *tmp, AVFrame *in,
                               VkSampler sampler, void *push_src, size_t push_size);

/**
 * Up to 16 inputs, one output
 */
int ff_vk_filter_process_Nin(FFVulkanContext *vkctx, FFVkExecPool *e,
                             FFVulkanShader *shd,
                             AVFrame *out, AVFrame *in[], int nb_in,
                             VkSampler sampler, void *push_src, size_t push_size);

#endif /* AVFILTER_VULKAN_FILTER_H */
