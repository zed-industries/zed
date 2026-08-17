/*
 * Copyright © 2008 Jérôme Glisse
 * All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining
 * a copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sub license, and/or sell copies of the Software, and to
 * permit persons to whom the Software is furnished to do so, subject to
 * the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
 * OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NON-INFRINGEMENT. IN NO EVENT SHALL THE COPYRIGHT HOLDERS, AUTHORS
 * AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
 * USE OR OTHER DEALINGS IN THE SOFTWARE.
 *
 * The above copyright notice and this permission notice (including the
 * next paragraph) shall be included in all copies or substantial portions
 * of the Software.
 */
/*
 * Authors:
 *      Jérôme Glisse <glisse@freedesktop.org>
 */
#ifndef RADEON_BO_H
#define RADEON_BO_H

#include <stdio.h>
#include <stdint.h>

/* bo object */
#define RADEON_BO_FLAGS_MACRO_TILE  1
#define RADEON_BO_FLAGS_MICRO_TILE  2
#define RADEON_BO_FLAGS_MICRO_TILE_SQUARE 0x20

struct radeon_bo_manager;
struct radeon_cs;

struct radeon_bo {
    void                        *ptr;
    uint32_t                    flags;
    uint32_t                    handle;
    uint32_t                    size;
};

struct radeon_bo_manager;

void radeon_bo_debug(struct radeon_bo *bo, const char *op);

struct radeon_bo *radeon_bo_open(struct radeon_bo_manager *bom,
                                 uint32_t handle,
                                 uint32_t size,
                                 uint32_t alignment,
                                 uint32_t domains,
                                 uint32_t flags);

void radeon_bo_ref(struct radeon_bo *bo);
struct radeon_bo *radeon_bo_unref(struct radeon_bo *bo);
int radeon_bo_map(struct radeon_bo *bo, int write);
int radeon_bo_unmap(struct radeon_bo *bo);
int radeon_bo_wait(struct radeon_bo *bo);
int radeon_bo_is_busy(struct radeon_bo *bo, uint32_t *domain);
int radeon_bo_set_tiling(struct radeon_bo *bo, uint32_t tiling_flags, uint32_t pitch);
int radeon_bo_get_tiling(struct radeon_bo *bo, uint32_t *tiling_flags, uint32_t *pitch);
int radeon_bo_is_static(struct radeon_bo *bo);
int radeon_bo_is_referenced_by_cs(struct radeon_bo *bo, struct radeon_cs *cs);
uint32_t radeon_bo_get_handle(struct radeon_bo *bo);
uint32_t radeon_bo_get_src_domain(struct radeon_bo *bo);
#endif
