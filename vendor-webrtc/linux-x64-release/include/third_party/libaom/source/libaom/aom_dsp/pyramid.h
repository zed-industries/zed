/*
 * Copyright (c) 2022, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AOM_DSP_PYRAMID_H_
#define AOM_AOM_DSP_PYRAMID_H_

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>

#include "config/aom_config.h"

#include "aom_scale/yv12config.h"
#include "aom_util/aom_pthread.h"

#ifdef __cplusplus
extern "C" {
#endif

// Minimum dimensions of a downsampled image
#define MIN_PYRAMID_SIZE_LOG2 3
#define MIN_PYRAMID_SIZE (1 << MIN_PYRAMID_SIZE_LOG2)

// Size of border around each pyramid image, in pixels
// Similarly to the border around regular image buffers, this border is filled
// with copies of the outermost pixels of the frame, to allow for more efficient
// convolution code
// TODO(rachelbarker): How many pixels do we actually need here?
// I think we only need 9 for disflow, but how many for corner matching?
#define PYRAMID_PADDING 16

// Byte alignment of each line within the image pyramids.
// That is, the first pixel inside the image (ie, not in the border region),
// on each row of each pyramid level, is aligned to this byte alignment.
// This value must be a power of 2.
#define PYRAMID_ALIGNMENT 32

typedef struct {
  uint8_t *buffer;
  int width;
  int height;
  int stride;
} PyramidLayer;

// Struct for an image pyramid
typedef struct image_pyramid {
#if CONFIG_MULTITHREAD
  // Mutex which is used to prevent the pyramid being computed twice at the
  // same time
  //
  // Semantics:
  // * This mutex must be held whenever reading or writing the
  //   `filled_levels` field
  //
  // * This mutex must also be held while computing the image pyramid,
  //   to ensure that only one thread may do so at a time.
  //
  // * However, once you have read the filled_levels field and observed
  //   a value N, it is safe to drop the mutex and read from the remaining
  //   fields, including the first N pyramid levels (but no higher).
  //   Note that filled_levels must be read once and cached in a local variable
  //   in order for this to be safe - it cannot be re-read without retaking
  //   the mutex.
  //
  //   This works because, once the image pyramid is computed, its contents
  //   will not be changed until the parent frame buffer is recycled,
  //   which will not happen until there are no more outstanding references
  //   to the frame buffer.
  pthread_mutex_t mutex;
#endif
  // Maximum number of levels for the given frame size
  // We always allocate enough memory for this many levels, as the memory
  // cost of higher levels of the pyramid is minimal.
  int max_levels;
  // Number of levels which currently hold valid data
  int filled_levels;
  // Pointer to allocated buffer
  uint8_t *buffer_alloc;
  // Data for each level
  // The `buffer` pointers inside this array point into the region which
  // is stored in the `buffer_alloc` field here
  PyramidLayer *layers;
} ImagePyramid;

size_t aom_get_pyramid_alloc_size(int width, int height, bool image_is_16bit);

ImagePyramid *aom_alloc_pyramid(int width, int height, bool image_is_16bit);

// Fill out a downsampling pyramid for a given frame.
//
// The top level (index 0) will always be an 8-bit copy of the input frame,
// regardless of the input bit depth. Additional levels are then downscaled
// by powers of 2.
//
// This function will ensure that the first `n_levels` levels of the pyramid
// are filled, unless the frame is too small to have this many levels.
// In that case, we will fill all available levels and then stop.
//
// Returns the actual number of levels filled, capped at n_levels,
// or -1 on error.
int aom_compute_pyramid(const YV12_BUFFER_CONFIG *frame, int bit_depth,
                        int n_levels, ImagePyramid *pyr);

#ifndef NDEBUG
// Check if a pyramid has already been computed to at least n levels
// This is mostly a debug helper - as it is necessary to hold pyr->mutex
// while reading the number of already-computed levels, we cannot just write:
//   assert(pyr->filled_levels >= n_levels);
// This function allows the check to be correctly written as:
//   assert(aom_is_pyramid_valid(pyr, n_levels));
//
// Note: This deliberately does not restrict n_levels based on the maximum
// number of permitted levels for the frame size. This allows the check to
// catch cases where the caller forgets to handle the case where
// max_levels is less than the requested number of levels
bool aom_is_pyramid_valid(ImagePyramid *pyr, int n_levels);
#endif

// Mark a pyramid as no longer containing valid data.
// This must be done whenever the corresponding frame buffer is reused
void aom_invalidate_pyramid(ImagePyramid *pyr);

// Release the memory associated with a pyramid
void aom_free_pyramid(ImagePyramid *pyr);

#ifdef __cplusplus
}
#endif

#endif  // AOM_AOM_DSP_PYRAMID_H_
