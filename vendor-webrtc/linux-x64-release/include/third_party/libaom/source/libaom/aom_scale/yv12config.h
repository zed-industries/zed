/*
 * Copyright (c) 2016, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AOM_SCALE_YV12CONFIG_H_
#define AOM_AOM_SCALE_YV12CONFIG_H_

#ifdef __cplusplus
extern "C" {
#endif

#include <stdbool.h>

#include "config/aom_config.h"

#include "aom/aom_codec.h"
#include "aom/aom_frame_buffer.h"
#include "aom/aom_integer.h"
#include "aom/internal/aom_image_internal.h"

/*!\cond */

#define AOMINNERBORDERINPIXELS 160
#define AOM_INTERP_EXTEND 4
#define AOM_BORDER_IN_PIXELS 288
#define AOM_ENC_NO_SCALE_BORDER 160
#define AOM_ENC_ALLINTRA_BORDER 64
#define AOM_DEC_BORDER_IN_PIXELS 64

#if CONFIG_AV1_ENCODER && !CONFIG_REALTIME_ONLY
struct image_pyramid;
struct corner_list;
#endif  // CONFIG_AV1_ENCODER && !CONFIG_REALTIME_ONLY

/*!\endcond */
/*!
 * \brief YV12 frame buffer data structure
 */
typedef struct yv12_buffer_config {
  /*!\cond */
  union {
    struct {
      // The aligned frame width of luma.
      // It is aligned to a multiple of 8:
      // y_width = (y_crop_width + 7) & ~7
      int y_width;
      // The aligned frame width of chroma.
      // uv_width = y_width >> subsampling_x
      int uv_width;
    };
    int widths[2];
  };
  union {
    struct {
      // The aligned frame height of luma.
      // It is aligned to a multiple of 8:
      // y_height = (y_crop_height + 7) & ~7
      int y_height;
      // The aligned frame height of chroma.
      // uv_height = y_height >> subsampling_y
      int uv_height;
    };
    int heights[2];
  };
  // The frame size en/decoded by AV1
  union {
    struct {
      int y_crop_width;
      int uv_crop_width;
    };
    int crop_widths[2];
  };
  union {
    struct {
      int y_crop_height;
      int uv_crop_height;
    };
    int crop_heights[2];
  };
  union {
    struct {
      int y_stride;
      int uv_stride;
    };
    int strides[2];
  };
  union {
    struct {
      uint8_t *y_buffer;
      uint8_t *u_buffer;
      uint8_t *v_buffer;
    };
    uint8_t *buffers[3];
  };

  // Indicate whether y_buffer, u_buffer, and v_buffer points to the internally
  // allocated memory or external buffers.
  int use_external_reference_buffers;
  // This is needed to store y_buffer, u_buffer, and v_buffer when set reference
  // uses an external refernece, and restore those buffer pointers after the
  // external reference frame is no longer used.
  uint8_t *store_buf_adr[3];

  // Global motion search data
#if CONFIG_AV1_ENCODER && !CONFIG_REALTIME_ONLY
  // 8-bit downsampling pyramid for the Y plane
  struct image_pyramid *y_pyramid;
  struct corner_list *corners;
#endif  // CONFIG_AV1_ENCODER && !CONFIG_REALTIME_ONLY

  uint8_t *buffer_alloc;
  size_t buffer_alloc_sz;
  int border;
  size_t frame_size;
  int subsampling_x;
  int subsampling_y;
  unsigned int bit_depth;
  aom_color_primaries_t color_primaries;
  aom_transfer_characteristics_t transfer_characteristics;
  aom_matrix_coefficients_t matrix_coefficients;
  uint8_t monochrome;
  aom_chroma_sample_position_t chroma_sample_position;
  aom_color_range_t color_range;
  int render_width;
  int render_height;

  int corrupted;
  int flags;
  aom_metadata_array_t *metadata;
  /*!\endcond */
} YV12_BUFFER_CONFIG;

/*!\cond */

#define YV12_FLAG_HIGHBITDEPTH 8

// Allocate a frame buffer
//
// If ybf currently contains an image, all associated memory will be freed and
// then reallocated. In contrast, aom_realloc_frame_buffer() will reuse any
// existing allocations where possible. So, if ybf is likely to already be
// set up, please consider aom_realloc_frame_buffer() instead.
//
// See aom_realloc_frame_buffer() for the meanings of the arguments, and
// available return values.
int aom_alloc_frame_buffer(YV12_BUFFER_CONFIG *ybf, int width, int height,
                           int ss_x, int ss_y, int use_highbitdepth, int border,
                           int byte_alignment, bool alloc_pyramid,
                           int alloc_y_plane_only);

// Updates the yv12 buffer config with the frame buffer. |byte_alignment| must
// be a power of 2, from 32 to 1024. 0 sets legacy alignment. If cb is not
// NULL, then libaom is using the frame buffer callbacks to handle memory.
// If cb is not NULL, libaom will call cb with minimum size in bytes needed
// to decode the current frame. If cb is NULL, libaom will allocate memory
// internally to decode the current frame.
//
// If alloc_pyramid is true, then an image pyramid will be allocated
// for use in global motion estimation. This is only needed if this frame
// buffer will be used to store a source frame or a reference frame in
// the encoder. Any other framebuffers (eg, intermediates for filtering,
// or any buffer in the decoder) can set alloc_pyramid = false.
//
// Returns 0 on success. Returns < 0  on failure.
int aom_realloc_frame_buffer(YV12_BUFFER_CONFIG *ybf, int width, int height,
                             int ss_x, int ss_y, int use_highbitdepth,
                             int border, int byte_alignment,
                             aom_codec_frame_buffer_t *fb,
                             aom_get_frame_buffer_cb_fn_t cb, void *cb_priv,
                             bool alloc_pyramid, int alloc_y_plane_only);

int aom_free_frame_buffer(YV12_BUFFER_CONFIG *ybf);

/*!\endcond */
/*!\brief Removes metadata from YUV_BUFFER_CONFIG struct.
 *
 * Frees metadata in frame buffer.
 * Frame buffer metadata pointer will be set to NULL.
 *
 * \param[in]    ybf       Frame buffer struct pointer
 */
void aom_remove_metadata_from_frame_buffer(YV12_BUFFER_CONFIG *ybf);

/*!\brief Copy metadata to YUV_BUFFER_CONFIG struct.
 *
 * Copies metadata to frame buffer.
 * Frame buffer will clear any previous metadata and will reallocate the
 * metadata array to the new metadata size. Then, it will copy the new metadata
 * array into it.
 * If arr metadata pointer points to the same address as current metadata in the
 * frame buffer, function will do nothing and return 0.
 * Returns 0 on success or -1 on failure.
 *
 * \param[in]    ybf       Frame buffer struct pointer
 * \param[in]    arr       Metadata array struct pointer
 */
int aom_copy_metadata_to_frame_buffer(YV12_BUFFER_CONFIG *ybf,
                                      const aom_metadata_array_t *arr);

/*!\brief Calculate the stride required for the image.
 *
 * Calculates the stride value for an image from aligned width and border.
 * Returns the y stride value.
 *
 * \param[in]    aligned_width       Aligned width of the image
 * \param[in]    border              Border in pixels
 */
static inline int aom_calc_y_stride(int aligned_width, int border) {
  return ((aligned_width + 2 * border) + 31) & ~31;
}

#ifdef __cplusplus
}
#endif

#endif  // AOM_AOM_SCALE_YV12CONFIG_H_
