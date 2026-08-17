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

#ifndef AVUTIL_VULKAN_SPIRV_H
#define AVUTIL_VULKAN_SPIRV_H

#include "vulkan.h"

#include "config.h"

typedef struct FFVkSPIRVCompiler {
    void *priv;
    int (*compile_shader)(FFVulkanContext *s, struct FFVkSPIRVCompiler *ctx,
                          FFVulkanShader *shd, uint8_t **data,
                          size_t *size, const char *entrypoint, void **opaque);
    void (*free_shader)(struct FFVkSPIRVCompiler *ctx, void **opaque);
    void (*uninit)(struct FFVkSPIRVCompiler **ctx);
} FFVkSPIRVCompiler;

#if CONFIG_LIBGLSLANG
FFVkSPIRVCompiler *ff_vk_glslang_init(void);
#define ff_vk_spirv_init ff_vk_glslang_init
#endif
#if CONFIG_LIBSHADERC
FFVkSPIRVCompiler *ff_vk_shaderc_init(void);
#define ff_vk_spirv_init ff_vk_shaderc_init
#endif

#endif /* AVUTIL_VULKAN_H */
