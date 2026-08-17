/*
 * Copyright © 2020, VideoLAN and dav1d authors
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this
 *    list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#include <inttypes.h>
#include <string.h>

#include "dav1d/dav1d.h"

#include <SDL.h>
#if HAVE_PLACEBO
# include <libplacebo/config.h>
#endif

// Check libplacebo Vulkan rendering
#if HAVE_VULKAN && defined(SDL_VIDEO_VULKAN)
# if defined(PL_HAVE_VULKAN) && PL_HAVE_VULKAN
#  define HAVE_RENDERER_PLACEBO 1
#  define HAVE_PLACEBO_VULKAN 1
# endif
#endif

// Check libplacebo OpenGL rendering
#if defined(PL_HAVE_OPENGL) && PL_HAVE_OPENGL
# define HAVE_RENDERER_PLACEBO 1
# define HAVE_PLACEBO_OPENGL 1
#endif

#ifndef HAVE_RENDERER_PLACEBO
#define HAVE_RENDERER_PLACEBO 0
#endif
#ifndef HAVE_PLACEBO_VULKAN
#define HAVE_PLACEBO_VULKAN 0
#endif
#ifndef HAVE_PLACEBO_OPENGL
#define HAVE_PLACEBO_OPENGL 0
#endif

/**
 * Settings structure
 * Hold all settings available for the player,
 * this is usually filled by parsing arguments
 * from the console.
 */
typedef struct {
    const char *inputfile;
    const char *renderer_name;
    int highquality;
    int untimed;
    int zerocopy;
    int gpugrain;
    int fullscreen;
} Dav1dPlaySettings;

#define WINDOW_WIDTH  910
#define WINDOW_HEIGHT 512

enum {
    DAV1D_EVENT_NEW_FRAME,
    DAV1D_EVENT_SEEK_FRAME,
    DAV1D_EVENT_DEC_QUIT
};

/**
 * Renderer info
 */
typedef struct rdr_info
{
    // Renderer name
    const char *name;
    // Cookie passed to the renderer implementation callbacks
    void *cookie;
    // Callback to create the renderer
    void* (*create_renderer)(const Dav1dPlaySettings *settings);
    // Callback to destroy the renderer
    void (*destroy_renderer)(void *cookie);
    // Callback to the render function that renders a prevously sent frame
    void (*render)(void *cookie, const Dav1dPlaySettings *settings);
    // Callback to the send frame function, _may_ also unref dav1d_pic!
    int (*update_frame)(void *cookie, Dav1dPicture *dav1d_pic,
                        const Dav1dPlaySettings *settings);
    // Callback for alloc/release pictures (optional)
    int (*alloc_pic)(Dav1dPicture *pic, void *cookie);
    void (*release_pic)(Dav1dPicture *pic, void *cookie);
    // Whether or not this renderer can apply on-GPU film grain synthesis
    int supports_gpu_grain;
} Dav1dPlayRenderInfo;

extern const Dav1dPlayRenderInfo rdr_placebo_vk;
extern const Dav1dPlayRenderInfo rdr_placebo_gl;
extern const Dav1dPlayRenderInfo rdr_sdl;

// Available renderes ordered by priority
static const Dav1dPlayRenderInfo* const dp_renderers[] = {
    &rdr_placebo_vk,
    &rdr_placebo_gl,
    &rdr_sdl,
};

static inline const Dav1dPlayRenderInfo *dp_get_renderer(const char *name)
{
    for (size_t i = 0; i < (sizeof(dp_renderers)/sizeof(*dp_renderers)); ++i)
    {
        if (dp_renderers[i]->name == NULL)
            continue;

        if (name == NULL || strcmp(name, dp_renderers[i]->name) == 0) {
            return dp_renderers[i];
        }
    }
    return NULL;
}

static inline SDL_Window *dp_create_sdl_window(int window_flags)
{
    SDL_Window *win;
    window_flags |= SDL_WINDOW_SHOWN | SDL_WINDOW_ALLOW_HIGHDPI;

    win = SDL_CreateWindow("Dav1dPlay", SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED,
        WINDOW_WIDTH, WINDOW_HEIGHT, window_flags);
    SDL_SetWindowResizable(win, SDL_TRUE);

    return win;
}
