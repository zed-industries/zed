/* mach64_drm.h -- Public header for the mach64 driver -*- linux-c -*-
 * Created: Thu Nov 30 20:04:32 2000 by gareth@valinux.com
 */
/*
 * Copyright 2000 Gareth Hughes
 * Copyright 2002 Frank C. Earl
 * Copyright 2002-2003 Leif Delgass
 * All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the next
 * paragraph) shall be included in all copies or substantial portions of the
 * Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT OWNER(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors:
 *    Gareth Hughes <gareth@valinux.com>
 *    Frank C. Earl <fearl@airmail.net>
 *    Leif Delgass <ldelgass@retinalburn.net>
 */

#ifndef __MACH64_DRM_H__
#define __MACH64_DRM_H__

/* WARNING: If you change any of these defines, make sure to change the
 * defines in the Xserver file (mach64_sarea.h)
 */
#ifndef __MACH64_SAREA_DEFINES__
#define __MACH64_SAREA_DEFINES__

/* What needs to be changed for the current vertex buffer?
 * GH: We're going to be pedantic about this.  We want the card to do as
 * little as possible, so let's avoid having it fetch a whole bunch of
 * register values that don't change all that often, if at all.
 */
#define MACH64_UPLOAD_DST_OFF_PITCH	0x0001
#define MACH64_UPLOAD_Z_OFF_PITCH	0x0002
#define MACH64_UPLOAD_Z_ALPHA_CNTL	0x0004
#define MACH64_UPLOAD_SCALE_3D_CNTL	0x0008
#define MACH64_UPLOAD_DP_FOG_CLR	0x0010
#define MACH64_UPLOAD_DP_WRITE_MASK	0x0020
#define MACH64_UPLOAD_DP_PIX_WIDTH	0x0040
#define MACH64_UPLOAD_SETUP_CNTL	0x0080
#define MACH64_UPLOAD_MISC		0x0100
#define MACH64_UPLOAD_TEXTURE		0x0200
#define MACH64_UPLOAD_TEX0IMAGE		0x0400
#define MACH64_UPLOAD_TEX1IMAGE		0x0800
#define MACH64_UPLOAD_CLIPRECTS		0x1000	/* handled client-side */
#define MACH64_UPLOAD_CONTEXT		0x00ff
#define MACH64_UPLOAD_ALL		0x1fff

/* DMA buffer size
 */
#define MACH64_BUFFER_SIZE		16384

/* Max number of swaps allowed on the ring
 * before the client must wait
 */
#define MACH64_MAX_QUEUED_FRAMES        3U

/* Byte offsets for host blit buffer data
 */
#define MACH64_HOSTDATA_BLIT_OFFSET	104

/* Keep these small for testing.
 */
#define MACH64_NR_SAREA_CLIPRECTS	8

#define MACH64_CARD_HEAP		0
#define MACH64_AGP_HEAP			1
#define MACH64_NR_TEX_HEAPS		2
#define MACH64_NR_TEX_REGIONS		64
#define MACH64_LOG_TEX_GRANULARITY	16

#define MACH64_TEX_MAXLEVELS		1

#define MACH64_NR_CONTEXT_REGS		15
#define MACH64_NR_TEXTURE_REGS		4

#endif				/* __MACH64_SAREA_DEFINES__ */

typedef struct {
	unsigned int dst_off_pitch;

	unsigned int z_off_pitch;
	unsigned int z_cntl;
	unsigned int alpha_tst_cntl;

	unsigned int scale_3d_cntl;

	unsigned int sc_left_right;
	unsigned int sc_top_bottom;

	unsigned int dp_fog_clr;
	unsigned int dp_write_mask;
	unsigned int dp_pix_width;
	unsigned int dp_mix;
	unsigned int dp_src;

	unsigned int clr_cmp_cntl;
	unsigned int gui_traj_cntl;

	unsigned int setup_cntl;

	unsigned int tex_size_pitch;
	unsigned int tex_cntl;
	unsigned int secondary_tex_off;
	unsigned int tex_offset;
} drm_mach64_context_regs_t;

typedef struct drm_mach64_sarea {
	/* The channel for communication of state information to the kernel
	 * on firing a vertex dma buffer.
	 */
	drm_mach64_context_regs_t context_state;
	unsigned int dirty;
	unsigned int vertsize;

	/* The current cliprects, or a subset thereof.
	 */
	struct drm_clip_rect boxes[MACH64_NR_SAREA_CLIPRECTS];
	unsigned int nbox;

	/* Counters for client-side throttling of rendering clients.
	 */
	unsigned int frames_queued;

	/* Texture memory LRU.
	 */
	struct drm_tex_region tex_list[MACH64_NR_TEX_HEAPS][MACH64_NR_TEX_REGIONS +
						       1];
	unsigned int tex_age[MACH64_NR_TEX_HEAPS];
	int ctx_owner;
} drm_mach64_sarea_t;

/* WARNING: If you change any of these defines, make sure to change the
 * defines in the Xserver file (mach64_common.h)
 */

/* Mach64 specific ioctls
 * The device specific ioctl range is 0x40 to 0x79.
 */

#define DRM_MACH64_INIT           0x00
#define DRM_MACH64_IDLE           0x01
#define DRM_MACH64_RESET          0x02
#define DRM_MACH64_SWAP           0x03
#define DRM_MACH64_CLEAR          0x04
#define DRM_MACH64_VERTEX         0x05
#define DRM_MACH64_BLIT           0x06
#define DRM_MACH64_FLUSH          0x07
#define DRM_MACH64_GETPARAM       0x08

#define DRM_IOCTL_MACH64_INIT           DRM_IOW( DRM_COMMAND_BASE + DRM_MACH64_INIT, drm_mach64_init_t)
#define DRM_IOCTL_MACH64_IDLE           DRM_IO(  DRM_COMMAND_BASE + DRM_MACH64_IDLE )
#define DRM_IOCTL_MACH64_RESET          DRM_IO(  DRM_COMMAND_BASE + DRM_MACH64_RESET )
#define DRM_IOCTL_MACH64_SWAP           DRM_IO(  DRM_COMMAND_BASE + DRM_MACH64_SWAP )
#define DRM_IOCTL_MACH64_CLEAR          DRM_IOW( DRM_COMMAND_BASE + DRM_MACH64_CLEAR, drm_mach64_clear_t)
#define DRM_IOCTL_MACH64_VERTEX         DRM_IOW( DRM_COMMAND_BASE + DRM_MACH64_VERTEX, drm_mach64_vertex_t)
#define DRM_IOCTL_MACH64_BLIT           DRM_IOW( DRM_COMMAND_BASE + DRM_MACH64_BLIT, drm_mach64_blit_t)
#define DRM_IOCTL_MACH64_FLUSH          DRM_IO(  DRM_COMMAND_BASE + DRM_MACH64_FLUSH )
#define DRM_IOCTL_MACH64_GETPARAM       DRM_IOWR( DRM_COMMAND_BASE + DRM_MACH64_GETPARAM, drm_mach64_getparam_t)

/* Buffer flags for clears
 */
#define MACH64_FRONT			0x1
#define MACH64_BACK			0x2
#define MACH64_DEPTH			0x4

/* Primitive types for vertex buffers
 */
#define MACH64_PRIM_POINTS		0x00000000
#define MACH64_PRIM_LINES		0x00000001
#define MACH64_PRIM_LINE_LOOP		0x00000002
#define MACH64_PRIM_LINE_STRIP		0x00000003
#define MACH64_PRIM_TRIANGLES		0x00000004
#define MACH64_PRIM_TRIANGLE_STRIP	0x00000005
#define MACH64_PRIM_TRIANGLE_FAN	0x00000006
#define MACH64_PRIM_QUADS		0x00000007
#define MACH64_PRIM_QUAD_STRIP		0x00000008
#define MACH64_PRIM_POLYGON		0x00000009

typedef enum _drm_mach64_dma_mode_t {
	MACH64_MODE_DMA_ASYNC,
	MACH64_MODE_DMA_SYNC,
	MACH64_MODE_MMIO
} drm_mach64_dma_mode_t;

typedef struct drm_mach64_init {
	enum {
		DRM_MACH64_INIT_DMA = 0x01,
		DRM_MACH64_CLEANUP_DMA = 0x02
	} func;

	unsigned long sarea_priv_offset;
	int is_pci;
	drm_mach64_dma_mode_t dma_mode;

	unsigned int fb_bpp;
	unsigned int front_offset, front_pitch;
	unsigned int back_offset, back_pitch;

	unsigned int depth_bpp;
	unsigned int depth_offset, depth_pitch;

	unsigned long fb_offset;
	unsigned long mmio_offset;
	unsigned long ring_offset;
	unsigned long buffers_offset;
	unsigned long agp_textures_offset;
} drm_mach64_init_t;

typedef struct drm_mach64_clear {
	unsigned int flags;
	int x, y, w, h;
	unsigned int clear_color;
	unsigned int clear_depth;
} drm_mach64_clear_t;

typedef struct drm_mach64_vertex {
	int prim;
	void *buf;		/* Address of vertex buffer */
	unsigned long used;	/* Number of bytes in buffer */
	int discard;		/* Client finished with buffer? */
} drm_mach64_vertex_t;

typedef struct drm_mach64_blit {
	void *buf;
	int pitch;
	int offset;
	int format;
	unsigned short x, y;
	unsigned short width, height;
} drm_mach64_blit_t;

typedef struct drm_mach64_getparam {
	enum {
		MACH64_PARAM_FRAMES_QUEUED = 0x01,
		MACH64_PARAM_IRQ_NR = 0x02
	} param;
	void *value;
} drm_mach64_getparam_t;

#endif
