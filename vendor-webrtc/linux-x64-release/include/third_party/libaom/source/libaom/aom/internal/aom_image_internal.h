/*
 * Copyright (c) 2019, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

/*!\file
 * \brief Describes the internal functions associated with the aom image
 * descriptor.
 *
 */
#ifndef AOM_AOM_INTERNAL_AOM_IMAGE_INTERNAL_H_
#define AOM_AOM_INTERNAL_AOM_IMAGE_INTERNAL_H_

#include "aom/aom_image.h"

#ifdef __cplusplus
extern "C" {
#endif

/*!\brief Array of aom_metadata structs for an image. */
struct aom_metadata_array {
  size_t sz;                       /* Number of metadata structs in the list */
  aom_metadata_t **metadata_array; /* Array of metadata structs */
};

/*! \brief Bit in aom_metadata_insert_flags marking metadata as layer-specific.
 */
#define AOM_MIF_LAYER_SPECIFIC 0x10
/*! \brief Bits in aom_metadata_insert_flags used to signal which frames to
 * add the metadata to (keyframes, non keyframes...).
 */
#define AOM_MIF_INSERT_LOCATION_MASK 0x0f

/*!\brief Alloc memory for aom_metadata_array struct.
 *
 * Allocate memory for aom_metadata_array struct.
 * If sz is 0 the aom_metadata_array struct's internal buffer list will be
 * NULL, but the aom_metadata_array struct itself will still be allocated.
 * Returns a pointer to the allocated struct or NULL on failure.
 *
 * \param[in]    sz       Size of internal metadata list buffer
 */
aom_metadata_array_t *aom_img_metadata_array_alloc(size_t sz);

/*!\brief Free metadata array struct.
 *
 * Free metadata array struct and all metadata structs inside.
 *
 * \param[in]    arr       Metadata array struct pointer
 */
void aom_img_metadata_array_free(aom_metadata_array_t *arr);

typedef void *(*aom_alloc_img_data_cb_fn_t)(void *priv, size_t size);

/*!\brief Open a descriptor, allocating storage for the underlying image by
 * using the provided callback function.
 *
 * Returns a descriptor for storing an image of the given format. The storage
 * for the image is allocated by using the provided callback function. Unlike
 * aom_img_alloc(), the returned descriptor does not own the storage for the
 * image. The caller is responsible for freeing the storage for the image.
 *
 * Note: If the callback function is invoked and succeeds,
 * aom_img_alloc_with_cb() is guaranteed to succeed. Therefore, if
 * aom_img_alloc_with_cb() fails, the caller is assured that no storage was
 * allocated.
 *
 * \param[in]    img       Pointer to storage for descriptor. If this parameter
 *                         is NULL, the storage for the descriptor will be
 *                         allocated on the heap.
 * \param[in]    fmt       Format for the image
 * \param[in]    d_w       Width of the image
 * \param[in]    d_h       Height of the image
 * \param[in]    align     Alignment, in bytes, of the image buffer and
 *                         each row in the image (stride).
 * \param[in]    alloc_cb  Callback function used to allocate storage for the
 *                         image.
 * \param[in]    cb_priv   The first argument ('priv') for the callback
 *                         function.
 *
 * \return Returns a pointer to the initialized image descriptor. If the img
 *         parameter is non-null, the value of the img parameter will be
 *         returned.
 */
aom_image_t *aom_img_alloc_with_cb(aom_image_t *img, aom_img_fmt_t fmt,
                                   unsigned int d_w, unsigned int d_h,
                                   unsigned int align,
                                   aom_alloc_img_data_cb_fn_t alloc_cb,
                                   void *cb_priv);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AOM_INTERNAL_AOM_IMAGE_INTERNAL_H_
