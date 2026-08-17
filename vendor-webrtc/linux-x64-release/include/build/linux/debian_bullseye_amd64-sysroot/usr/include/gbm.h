/*
 * Copyright © 2011 Intel Corporation
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
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT.  IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
 * HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
 * WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 *
 * Authors:
 *    Benjamin Franzke <benjaminfranzke@googlemail.com>
 */

#ifndef _GBM_H_
#define _GBM_H_

#define __GBM__ 1

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif


/**
 * \file gbm.h
 * \brief Generic Buffer Manager
 */

struct gbm_device;
struct gbm_bo;
struct gbm_surface;

/**
 * \mainpage The Generic Buffer Manager
 *
 * This module provides an abstraction that the caller can use to request a
 * buffer from the underlying memory management system for the platform.
 *
 * This allows the creation of portable code whilst still allowing access to
 * the underlying memory manager.
 */

/**
 * Abstraction representing the handle to a buffer allocated by the
 * manager
 */
union gbm_bo_handle {
   void *ptr;
   int32_t s32;
   uint32_t u32;
   int64_t s64;
   uint64_t u64;
};

/** Format of the allocated buffer */
enum gbm_bo_format {
   /** RGB with 8 bits per channel in a 32 bit value */
   GBM_BO_FORMAT_XRGB8888, 
   /** ARGB with 8 bits per channel in a 32 bit value */
   GBM_BO_FORMAT_ARGB8888
};


/**
 * The FourCC format codes are taken from the drm_fourcc.h definition, and
 * re-namespaced. New GBM formats must not be added, unless they are
 * identical ports from drm_fourcc.
 */
#define __gbm_fourcc_code(a,b,c,d) ((uint32_t)(a) | ((uint32_t)(b) << 8) | \
			      ((uint32_t)(c) << 16) | ((uint32_t)(d) << 24))

#define GBM_FORMAT_BIG_ENDIAN (1<<31) /* format is big endian instead of little endian */

/* color index */
#define GBM_FORMAT_C8		__gbm_fourcc_code('C', '8', ' ', ' ') /* [7:0] C */

/* 8 bpp Red */
#define GBM_FORMAT_R8		__gbm_fourcc_code('R', '8', ' ', ' ') /* [7:0] R */

/* 16 bpp RG */
#define GBM_FORMAT_GR88		__gbm_fourcc_code('G', 'R', '8', '8') /* [15:0] G:R 8:8 little endian */

/* 8 bpp RGB */
#define GBM_FORMAT_RGB332	__gbm_fourcc_code('R', 'G', 'B', '8') /* [7:0] R:G:B 3:3:2 */
#define GBM_FORMAT_BGR233	__gbm_fourcc_code('B', 'G', 'R', '8') /* [7:0] B:G:R 2:3:3 */

/* 16 bpp RGB */
#define GBM_FORMAT_XRGB4444	__gbm_fourcc_code('X', 'R', '1', '2') /* [15:0] x:R:G:B 4:4:4:4 little endian */
#define GBM_FORMAT_XBGR4444	__gbm_fourcc_code('X', 'B', '1', '2') /* [15:0] x:B:G:R 4:4:4:4 little endian */
#define GBM_FORMAT_RGBX4444	__gbm_fourcc_code('R', 'X', '1', '2') /* [15:0] R:G:B:x 4:4:4:4 little endian */
#define GBM_FORMAT_BGRX4444	__gbm_fourcc_code('B', 'X', '1', '2') /* [15:0] B:G:R:x 4:4:4:4 little endian */

#define GBM_FORMAT_ARGB4444	__gbm_fourcc_code('A', 'R', '1', '2') /* [15:0] A:R:G:B 4:4:4:4 little endian */
#define GBM_FORMAT_ABGR4444	__gbm_fourcc_code('A', 'B', '1', '2') /* [15:0] A:B:G:R 4:4:4:4 little endian */
#define GBM_FORMAT_RGBA4444	__gbm_fourcc_code('R', 'A', '1', '2') /* [15:0] R:G:B:A 4:4:4:4 little endian */
#define GBM_FORMAT_BGRA4444	__gbm_fourcc_code('B', 'A', '1', '2') /* [15:0] B:G:R:A 4:4:4:4 little endian */

#define GBM_FORMAT_XRGB1555	__gbm_fourcc_code('X', 'R', '1', '5') /* [15:0] x:R:G:B 1:5:5:5 little endian */
#define GBM_FORMAT_XBGR1555	__gbm_fourcc_code('X', 'B', '1', '5') /* [15:0] x:B:G:R 1:5:5:5 little endian */
#define GBM_FORMAT_RGBX5551	__gbm_fourcc_code('R', 'X', '1', '5') /* [15:0] R:G:B:x 5:5:5:1 little endian */
#define GBM_FORMAT_BGRX5551	__gbm_fourcc_code('B', 'X', '1', '5') /* [15:0] B:G:R:x 5:5:5:1 little endian */

#define GBM_FORMAT_ARGB1555	__gbm_fourcc_code('A', 'R', '1', '5') /* [15:0] A:R:G:B 1:5:5:5 little endian */
#define GBM_FORMAT_ABGR1555	__gbm_fourcc_code('A', 'B', '1', '5') /* [15:0] A:B:G:R 1:5:5:5 little endian */
#define GBM_FORMAT_RGBA5551	__gbm_fourcc_code('R', 'A', '1', '5') /* [15:0] R:G:B:A 5:5:5:1 little endian */
#define GBM_FORMAT_BGRA5551	__gbm_fourcc_code('B', 'A', '1', '5') /* [15:0] B:G:R:A 5:5:5:1 little endian */

#define GBM_FORMAT_RGB565	__gbm_fourcc_code('R', 'G', '1', '6') /* [15:0] R:G:B 5:6:5 little endian */
#define GBM_FORMAT_BGR565	__gbm_fourcc_code('B', 'G', '1', '6') /* [15:0] B:G:R 5:6:5 little endian */

/* 24 bpp RGB */
#define GBM_FORMAT_RGB888	__gbm_fourcc_code('R', 'G', '2', '4') /* [23:0] R:G:B little endian */
#define GBM_FORMAT_BGR888	__gbm_fourcc_code('B', 'G', '2', '4') /* [23:0] B:G:R little endian */

/* 32 bpp RGB */
#define GBM_FORMAT_XRGB8888	__gbm_fourcc_code('X', 'R', '2', '4') /* [31:0] x:R:G:B 8:8:8:8 little endian */
#define GBM_FORMAT_XBGR8888	__gbm_fourcc_code('X', 'B', '2', '4') /* [31:0] x:B:G:R 8:8:8:8 little endian */
#define GBM_FORMAT_RGBX8888	__gbm_fourcc_code('R', 'X', '2', '4') /* [31:0] R:G:B:x 8:8:8:8 little endian */
#define GBM_FORMAT_BGRX8888	__gbm_fourcc_code('B', 'X', '2', '4') /* [31:0] B:G:R:x 8:8:8:8 little endian */

#define GBM_FORMAT_ARGB8888	__gbm_fourcc_code('A', 'R', '2', '4') /* [31:0] A:R:G:B 8:8:8:8 little endian */
#define GBM_FORMAT_ABGR8888	__gbm_fourcc_code('A', 'B', '2', '4') /* [31:0] A:B:G:R 8:8:8:8 little endian */
#define GBM_FORMAT_RGBA8888	__gbm_fourcc_code('R', 'A', '2', '4') /* [31:0] R:G:B:A 8:8:8:8 little endian */
#define GBM_FORMAT_BGRA8888	__gbm_fourcc_code('B', 'A', '2', '4') /* [31:0] B:G:R:A 8:8:8:8 little endian */

#define GBM_FORMAT_XRGB2101010	__gbm_fourcc_code('X', 'R', '3', '0') /* [31:0] x:R:G:B 2:10:10:10 little endian */
#define GBM_FORMAT_XBGR2101010	__gbm_fourcc_code('X', 'B', '3', '0') /* [31:0] x:B:G:R 2:10:10:10 little endian */
#define GBM_FORMAT_RGBX1010102	__gbm_fourcc_code('R', 'X', '3', '0') /* [31:0] R:G:B:x 10:10:10:2 little endian */
#define GBM_FORMAT_BGRX1010102	__gbm_fourcc_code('B', 'X', '3', '0') /* [31:0] B:G:R:x 10:10:10:2 little endian */

#define GBM_FORMAT_ARGB2101010	__gbm_fourcc_code('A', 'R', '3', '0') /* [31:0] A:R:G:B 2:10:10:10 little endian */
#define GBM_FORMAT_ABGR2101010	__gbm_fourcc_code('A', 'B', '3', '0') /* [31:0] A:B:G:R 2:10:10:10 little endian */
#define GBM_FORMAT_RGBA1010102	__gbm_fourcc_code('R', 'A', '3', '0') /* [31:0] R:G:B:A 10:10:10:2 little endian */
#define GBM_FORMAT_BGRA1010102	__gbm_fourcc_code('B', 'A', '3', '0') /* [31:0] B:G:R:A 10:10:10:2 little endian */

/*
 * Floating point 64bpp RGB
 * IEEE 754-2008 binary16 half-precision float
 * [15:0] sign:exponent:mantissa 1:5:10
 */
#define GBM_FORMAT_XBGR16161616F __gbm_fourcc_code('X', 'B', '4', 'H') /* [63:0] x:B:G:R 16:16:16:16 little endian */

#define GBM_FORMAT_ABGR16161616F __gbm_fourcc_code('A', 'B', '4', 'H') /* [63:0] A:B:G:R 16:16:16:16 little endian */

/* packed YCbCr */
#define GBM_FORMAT_YUYV		__gbm_fourcc_code('Y', 'U', 'Y', 'V') /* [31:0] Cr0:Y1:Cb0:Y0 8:8:8:8 little endian */
#define GBM_FORMAT_YVYU		__gbm_fourcc_code('Y', 'V', 'Y', 'U') /* [31:0] Cb0:Y1:Cr0:Y0 8:8:8:8 little endian */
#define GBM_FORMAT_UYVY		__gbm_fourcc_code('U', 'Y', 'V', 'Y') /* [31:0] Y1:Cr0:Y0:Cb0 8:8:8:8 little endian */
#define GBM_FORMAT_VYUY		__gbm_fourcc_code('V', 'Y', 'U', 'Y') /* [31:0] Y1:Cb0:Y0:Cr0 8:8:8:8 little endian */

#define GBM_FORMAT_AYUV		__gbm_fourcc_code('A', 'Y', 'U', 'V') /* [31:0] A:Y:Cb:Cr 8:8:8:8 little endian */

/*
 * 2 plane YCbCr
 * index 0 = Y plane, [7:0] Y
 * index 1 = Cr:Cb plane, [15:0] Cr:Cb little endian
 * or
 * index 1 = Cb:Cr plane, [15:0] Cb:Cr little endian
 */
#define GBM_FORMAT_NV12		__gbm_fourcc_code('N', 'V', '1', '2') /* 2x2 subsampled Cr:Cb plane */
#define GBM_FORMAT_NV21		__gbm_fourcc_code('N', 'V', '2', '1') /* 2x2 subsampled Cb:Cr plane */
#define GBM_FORMAT_NV16		__gbm_fourcc_code('N', 'V', '1', '6') /* 2x1 subsampled Cr:Cb plane */
#define GBM_FORMAT_NV61		__gbm_fourcc_code('N', 'V', '6', '1') /* 2x1 subsampled Cb:Cr plane */

/*
 * 3 plane YCbCr
 * index 0: Y plane, [7:0] Y
 * index 1: Cb plane, [7:0] Cb
 * index 2: Cr plane, [7:0] Cr
 * or
 * index 1: Cr plane, [7:0] Cr
 * index 2: Cb plane, [7:0] Cb
 */
#define GBM_FORMAT_YUV410	__gbm_fourcc_code('Y', 'U', 'V', '9') /* 4x4 subsampled Cb (1) and Cr (2) planes */
#define GBM_FORMAT_YVU410	__gbm_fourcc_code('Y', 'V', 'U', '9') /* 4x4 subsampled Cr (1) and Cb (2) planes */
#define GBM_FORMAT_YUV411	__gbm_fourcc_code('Y', 'U', '1', '1') /* 4x1 subsampled Cb (1) and Cr (2) planes */
#define GBM_FORMAT_YVU411	__gbm_fourcc_code('Y', 'V', '1', '1') /* 4x1 subsampled Cr (1) and Cb (2) planes */
#define GBM_FORMAT_YUV420	__gbm_fourcc_code('Y', 'U', '1', '2') /* 2x2 subsampled Cb (1) and Cr (2) planes */
#define GBM_FORMAT_YVU420	__gbm_fourcc_code('Y', 'V', '1', '2') /* 2x2 subsampled Cr (1) and Cb (2) planes */
#define GBM_FORMAT_YUV422	__gbm_fourcc_code('Y', 'U', '1', '6') /* 2x1 subsampled Cb (1) and Cr (2) planes */
#define GBM_FORMAT_YVU422	__gbm_fourcc_code('Y', 'V', '1', '6') /* 2x1 subsampled Cr (1) and Cb (2) planes */
#define GBM_FORMAT_YUV444	__gbm_fourcc_code('Y', 'U', '2', '4') /* non-subsampled Cb (1) and Cr (2) planes */
#define GBM_FORMAT_YVU444	__gbm_fourcc_code('Y', 'V', '2', '4') /* non-subsampled Cr (1) and Cb (2) planes */

struct gbm_format_name_desc {
   char name[5];
};

/**
 * Flags to indicate the intended use for the buffer - these are passed into
 * gbm_bo_create(). The caller must set the union of all the flags that are
 * appropriate
 *
 * \sa Use gbm_device_is_format_supported() to check if the combination of format
 * and use flags are supported
 */
enum gbm_bo_flags {
   /**
    * Buffer is going to be presented to the screen using an API such as KMS
    */
   GBM_BO_USE_SCANOUT      = (1 << 0),
   /**
    * Buffer is going to be used as cursor
    */
   GBM_BO_USE_CURSOR       = (1 << 1),
   /**
    * Deprecated
    */
   GBM_BO_USE_CURSOR_64X64 = GBM_BO_USE_CURSOR,
   /**
    * Buffer is to be used for rendering - for example it is going to be used
    * as the storage for a color buffer
    */
   GBM_BO_USE_RENDERING    = (1 << 2),
   /**
    * Buffer can be used for gbm_bo_write.  This is guaranteed to work
    * with GBM_BO_USE_CURSOR, but may not work for other combinations.
    */
   GBM_BO_USE_WRITE    = (1 << 3),
   /**
    * Buffer is linear, i.e. not tiled.
    */
   GBM_BO_USE_LINEAR = (1 << 4),
   /**
    * Buffer is protected, i.e. encrypted and not readable by CPU or any
    * other non-secure / non-trusted components nor by non-trusted OpenGL,
    * OpenCL, and Vulkan applications.
    */
   GBM_BO_USE_PROTECTED = (1 << 5),
};

int
gbm_device_get_fd(struct gbm_device *gbm);

const char *
gbm_device_get_backend_name(struct gbm_device *gbm);

int
gbm_device_is_format_supported(struct gbm_device *gbm,
                               uint32_t format, uint32_t usage);

int
gbm_device_get_format_modifier_plane_count(struct gbm_device *gbm,
                                           uint32_t format,
                                           uint64_t modifier);

void
gbm_device_destroy(struct gbm_device *gbm);

struct gbm_device *
gbm_create_device(int fd);

struct gbm_bo *
gbm_bo_create(struct gbm_device *gbm,
              uint32_t width, uint32_t height,
              uint32_t format, uint32_t flags);

struct gbm_bo *
gbm_bo_create_with_modifiers(struct gbm_device *gbm,
                             uint32_t width, uint32_t height,
                             uint32_t format,
                             const uint64_t *modifiers,
                             const unsigned int count);
#define GBM_BO_IMPORT_WL_BUFFER         0x5501
#define GBM_BO_IMPORT_EGL_IMAGE         0x5502
#define GBM_BO_IMPORT_FD                0x5503
#define GBM_BO_IMPORT_FD_MODIFIER       0x5504

struct gbm_import_fd_data {
   int fd;
   uint32_t width;
   uint32_t height;
   uint32_t stride;
   uint32_t format;
};

#define GBM_MAX_PLANES 4

struct gbm_import_fd_modifier_data {
   uint32_t width;
   uint32_t height;
   uint32_t format;
   uint32_t num_fds;
   int fds[GBM_MAX_PLANES];
   int strides[GBM_MAX_PLANES];
   int offsets[GBM_MAX_PLANES];
   uint64_t modifier;
};

struct gbm_bo *
gbm_bo_import(struct gbm_device *gbm, uint32_t type,
              void *buffer, uint32_t usage);

/**
 * Flags to indicate the type of mapping for the buffer - these are
 * passed into gbm_bo_map(). The caller must set the union of all the
 * flags that are appropriate.
 *
 * These flags are independent of the GBM_BO_USE_* creation flags. However,
 * mapping the buffer may require copying to/from a staging buffer.
 *
 * See also: pipe_map_flags
 */
enum gbm_bo_transfer_flags {
   /**
    * Buffer contents read back (or accessed directly) at transfer
    * create time.
    */
   GBM_BO_TRANSFER_READ       = (1 << 0),
   /**
    * Buffer contents will be written back at unmap time
    * (or modified as a result of being accessed directly).
    */
   GBM_BO_TRANSFER_WRITE      = (1 << 1),
   /**
    * Read/modify/write
    */
   GBM_BO_TRANSFER_READ_WRITE = (GBM_BO_TRANSFER_READ | GBM_BO_TRANSFER_WRITE),
};

void *
gbm_bo_map(struct gbm_bo *bo,
           uint32_t x, uint32_t y, uint32_t width, uint32_t height,
           uint32_t flags, uint32_t *stride, void **map_data);

void
gbm_bo_unmap(struct gbm_bo *bo, void *map_data);

uint32_t
gbm_bo_get_width(struct gbm_bo *bo);

uint32_t
gbm_bo_get_height(struct gbm_bo *bo);

uint32_t
gbm_bo_get_stride(struct gbm_bo *bo);

uint32_t
gbm_bo_get_stride_for_plane(struct gbm_bo *bo, int plane);

uint32_t
gbm_bo_get_format(struct gbm_bo *bo);

uint32_t
gbm_bo_get_bpp(struct gbm_bo *bo);

uint32_t
gbm_bo_get_offset(struct gbm_bo *bo, int plane);

struct gbm_device *
gbm_bo_get_device(struct gbm_bo *bo);

union gbm_bo_handle
gbm_bo_get_handle(struct gbm_bo *bo);

int
gbm_bo_get_fd(struct gbm_bo *bo);

uint64_t
gbm_bo_get_modifier(struct gbm_bo *bo);

int
gbm_bo_get_plane_count(struct gbm_bo *bo);

union gbm_bo_handle
gbm_bo_get_handle_for_plane(struct gbm_bo *bo, int plane);

int
gbm_bo_write(struct gbm_bo *bo, const void *buf, size_t count);

void
gbm_bo_set_user_data(struct gbm_bo *bo, void *data,
		     void (*destroy_user_data)(struct gbm_bo *, void *));

void *
gbm_bo_get_user_data(struct gbm_bo *bo);

void
gbm_bo_destroy(struct gbm_bo *bo);

struct gbm_surface *
gbm_surface_create(struct gbm_device *gbm,
                   uint32_t width, uint32_t height,
		   uint32_t format, uint32_t flags);

struct gbm_surface *
gbm_surface_create_with_modifiers(struct gbm_device *gbm,
                                  uint32_t width, uint32_t height,
                                  uint32_t format,
                                  const uint64_t *modifiers,
                                  const unsigned int count);

struct gbm_bo *
gbm_surface_lock_front_buffer(struct gbm_surface *surface);

void
gbm_surface_release_buffer(struct gbm_surface *surface, struct gbm_bo *bo);

int
gbm_surface_has_free_buffers(struct gbm_surface *surface);

void
gbm_surface_destroy(struct gbm_surface *surface);

char *
gbm_format_get_name(uint32_t gbm_format, struct gbm_format_name_desc *desc);

#ifdef __cplusplus
}
#endif

#endif
