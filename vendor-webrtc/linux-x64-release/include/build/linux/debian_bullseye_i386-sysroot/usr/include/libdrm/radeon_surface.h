/*
 * Copyright © 2011 Red Hat All Rights Reserved.
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
 *      Jérôme Glisse <jglisse@redhat.com>
 */
#ifndef RADEON_SURFACE_H
#define RADEON_SURFACE_H

/* Note :
 *
 * For texture array, the n layer are stored one after the other within each
 * mipmap level. 0 value for field than can be hint is always valid.
 */

#define RADEON_SURF_MAX_LEVEL                   32

#define RADEON_SURF_TYPE_MASK                   0xFF
#define RADEON_SURF_TYPE_SHIFT                  0
#define     RADEON_SURF_TYPE_1D                     0
#define     RADEON_SURF_TYPE_2D                     1
#define     RADEON_SURF_TYPE_3D                     2
#define     RADEON_SURF_TYPE_CUBEMAP                3
#define     RADEON_SURF_TYPE_1D_ARRAY               4
#define     RADEON_SURF_TYPE_2D_ARRAY               5
#define RADEON_SURF_MODE_MASK                   0xFF
#define RADEON_SURF_MODE_SHIFT                  8
#define     RADEON_SURF_MODE_LINEAR                 0
#define     RADEON_SURF_MODE_LINEAR_ALIGNED         1
#define     RADEON_SURF_MODE_1D                     2
#define     RADEON_SURF_MODE_2D                     3
#define RADEON_SURF_SCANOUT                     (1 << 16)
#define RADEON_SURF_ZBUFFER                     (1 << 17)
#define RADEON_SURF_SBUFFER                     (1 << 18)
#define RADEON_SURF_Z_OR_SBUFFER                (RADEON_SURF_ZBUFFER | RADEON_SURF_SBUFFER)
#define RADEON_SURF_HAS_SBUFFER_MIPTREE         (1 << 19)
#define RADEON_SURF_HAS_TILE_MODE_INDEX         (1 << 20)
#define RADEON_SURF_FMASK                       (1 << 21)

#define RADEON_SURF_GET(v, field)   (((v) >> RADEON_SURF_ ## field ## _SHIFT) & RADEON_SURF_ ## field ## _MASK)
#define RADEON_SURF_SET(v, field)   (((v) & RADEON_SURF_ ## field ## _MASK) << RADEON_SURF_ ## field ## _SHIFT)
#define RADEON_SURF_CLR(v, field)   ((v) & ~(RADEON_SURF_ ## field ## _MASK << RADEON_SURF_ ## field ## _SHIFT))

/* first field up to mode need to match r6 struct so that we can reuse
 * same function for linear & linear aligned
 */
struct radeon_surface_level {
    uint64_t                    offset;
    uint64_t                    slice_size;
    uint32_t                    npix_x;
    uint32_t                    npix_y;
    uint32_t                    npix_z;
    uint32_t                    nblk_x;
    uint32_t                    nblk_y;
    uint32_t                    nblk_z;
    uint32_t                    pitch_bytes;
    uint32_t                    mode;
};

enum si_tiling_mode {
    SI_TILING_AUTO = 0,

    SI_TILING_COLOR_1D,
    SI_TILING_COLOR_1D_SCANOUT,
    SI_TILING_COLOR_2D_8BPP,
    SI_TILING_COLOR_2D_16BPP,
    SI_TILING_COLOR_2D_32BPP,
    SI_TILING_COLOR_2D_64BPP,
    SI_TILING_COLOR_2D_SCANOUT_16BPP,
    SI_TILING_COLOR_2D_SCANOUT_32BPP,
    SI_TILING_COLOR_LINEAR,

    SI_TILING_STENCIL_1D,
    SI_TILING_STENCIL_2D,
    SI_TILING_STENCIL_2D_2AA,
    SI_TILING_STENCIL_2D_4AA,
    SI_TILING_STENCIL_2D_8AA,

    SI_TILING_DEPTH_1D,
    SI_TILING_DEPTH_2D,
    SI_TILING_DEPTH_2D_2AA,
    SI_TILING_DEPTH_2D_4AA,
    SI_TILING_DEPTH_2D_8AA,

    SI_TILING_LAST_MODE,
};

struct radeon_surface {
    uint32_t                    npix_x;
    uint32_t                    npix_y;
    uint32_t                    npix_z;
    uint32_t                    blk_w;
    uint32_t                    blk_h;
    uint32_t                    blk_d;
    uint32_t                    array_size;
    uint32_t                    last_level;
    uint32_t                    bpe;
    uint32_t                    nsamples;
    uint32_t                    flags;
    /* Following is updated/fill by the allocator. It's allowed to
     * set some of the value but they are use as hint and can be
     * overridden (things lile bankw/bankh on evergreen for
     * instance).
     */
    uint64_t                    bo_size;
    uint64_t                    bo_alignment;
    /* apply to eg */
    uint32_t                    bankw;
    uint32_t                    bankh;
    uint32_t                    mtilea;
    uint32_t                    tile_split;
    uint32_t                    stencil_tile_split;
    uint64_t                    stencil_offset;
    struct radeon_surface_level level[RADEON_SURF_MAX_LEVEL];
    struct radeon_surface_level stencil_level[RADEON_SURF_MAX_LEVEL];
    uint32_t                    tiling_index[RADEON_SURF_MAX_LEVEL];
    uint32_t                    stencil_tiling_index[RADEON_SURF_MAX_LEVEL];
};

struct radeon_surface_manager *radeon_surface_manager_new(int fd);
void radeon_surface_manager_free(struct radeon_surface_manager *surf_man);
int radeon_surface_init(struct radeon_surface_manager *surf_man,
                        struct radeon_surface *surf);
int radeon_surface_best(struct radeon_surface_manager *surf_man,
                        struct radeon_surface *surf);

#endif
