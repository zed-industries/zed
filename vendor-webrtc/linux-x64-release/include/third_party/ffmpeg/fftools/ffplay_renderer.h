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

#ifndef FFTOOLS_FFPLAY_RENDERER_H
#define FFTOOLS_FFPLAY_RENDERER_H

#include <SDL.h>

#include "libavutil/frame.h"

typedef struct VkRenderer VkRenderer;

VkRenderer *vk_get_renderer(void);

int vk_renderer_create(VkRenderer *renderer, SDL_Window *window,
                       AVDictionary *opt);

int vk_renderer_get_hw_dev(VkRenderer *renderer, AVBufferRef **dev);

int vk_renderer_display(VkRenderer *renderer, AVFrame *frame);

int vk_renderer_resize(VkRenderer *renderer, int width, int height);

void vk_renderer_destroy(VkRenderer *renderer);

#endif /* FFTOOLS_FFPLAY_RENDERER_H */
