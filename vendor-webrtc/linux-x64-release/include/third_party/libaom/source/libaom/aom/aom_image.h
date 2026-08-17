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

/*!\file
 * \brief Describes the aom image descriptor and associated operations
 *
 */
#ifndef AOM_AOM_AOM_IMAGE_H_
#define AOM_AOM_AOM_IMAGE_H_

#ifdef __cplusplus
extern "C" {
#endif

#include "aom/aom_integer.h"

/*!\brief Current ABI version number
 *
 * \internal
 * If this file is altered in any way that changes the ABI, this value
 * must be bumped.  Examples include, but are not limited to, changing
 * types, removing or reassigning enums, adding/removing/rearranging
 * fields to structures
 */
#define AOM_IMAGE_ABI_VERSION (9) /**<\hideinitializer*/

#define AOM_IMG_FMT_PLANAR 0x100  /**< Image is a planar format. */
#define AOM_IMG_FMT_UV_FLIP 0x200 /**< V plane precedes U in memory. */
/** 0x400 used to signal alpha channel, skipping for backwards compatibility. */
#define AOM_IMG_FMT_HIGHBITDEPTH 0x800 /**< Image uses 16bit framebuffer. */

/*!\brief List of supported image formats */
typedef enum aom_img_fmt {
  AOM_IMG_FMT_NONE,
  AOM_IMG_FMT_YV12 =
      AOM_IMG_FMT_PLANAR | AOM_IMG_FMT_UV_FLIP | 1, /**< planar YVU */
  AOM_IMG_FMT_I420 = AOM_IMG_FMT_PLANAR | 2,
  AOM_IMG_FMT_AOMYV12 = AOM_IMG_FMT_PLANAR | AOM_IMG_FMT_UV_FLIP |
                        3, /** < planar 4:2:0 format with aom color space */
  AOM_IMG_FMT_AOMI420 = AOM_IMG_FMT_PLANAR | 4,
  AOM_IMG_FMT_I422 = AOM_IMG_FMT_PLANAR | 5,
  AOM_IMG_FMT_I444 = AOM_IMG_FMT_PLANAR | 6,
/*!\brief Allows detection of the presence of AOM_IMG_FMT_NV12 at compile time.
 */
#define AOM_HAVE_IMG_FMT_NV12 1
  AOM_IMG_FMT_NV12 =
      AOM_IMG_FMT_PLANAR | 7, /**< 4:2:0 with U and V interleaved */
  AOM_IMG_FMT_I42016 = AOM_IMG_FMT_I420 | AOM_IMG_FMT_HIGHBITDEPTH,
  AOM_IMG_FMT_YV1216 = AOM_IMG_FMT_YV12 | AOM_IMG_FMT_HIGHBITDEPTH,
  AOM_IMG_FMT_I42216 = AOM_IMG_FMT_I422 | AOM_IMG_FMT_HIGHBITDEPTH,
  AOM_IMG_FMT_I44416 = AOM_IMG_FMT_I444 | AOM_IMG_FMT_HIGHBITDEPTH,
} aom_img_fmt_t; /**< alias for enum aom_img_fmt */

/*!\brief List of supported color primaries */
typedef enum aom_color_primaries {
  AOM_CICP_CP_RESERVED_0 = 0,  /**< For future use */
  AOM_CICP_CP_BT_709 = 1,      /**< BT.709 */
  AOM_CICP_CP_UNSPECIFIED = 2, /**< Unspecified */
  AOM_CICP_CP_RESERVED_3 = 3,  /**< For future use */
  AOM_CICP_CP_BT_470_M = 4,    /**< BT.470 System M (historical) */
  AOM_CICP_CP_BT_470_B_G = 5,  /**< BT.470 System B, G (historical) */
  AOM_CICP_CP_BT_601 = 6,      /**< BT.601 */
  AOM_CICP_CP_SMPTE_240 = 7,   /**< SMPTE 240 */
  AOM_CICP_CP_GENERIC_FILM =
      8, /**< Generic film (color filters using illuminant C) */
  AOM_CICP_CP_BT_2020 = 9,      /**< BT.2020, BT.2100 */
  AOM_CICP_CP_XYZ = 10,         /**< SMPTE 428 (CIE 1921 XYZ) */
  AOM_CICP_CP_SMPTE_431 = 11,   /**< SMPTE RP 431-2 */
  AOM_CICP_CP_SMPTE_432 = 12,   /**< SMPTE EG 432-1  */
  AOM_CICP_CP_RESERVED_13 = 13, /**< For future use (values 13 - 21)  */
  AOM_CICP_CP_EBU_3213 = 22,    /**< EBU Tech. 3213-E  */
  AOM_CICP_CP_RESERVED_23 = 23  /**< For future use (values 23 - 255)  */
} aom_color_primaries_t;        /**< alias for enum aom_color_primaries */

/*!\brief List of supported transfer functions */
typedef enum aom_transfer_characteristics {
  AOM_CICP_TC_RESERVED_0 = 0,  /**< For future use */
  AOM_CICP_TC_BT_709 = 1,      /**< BT.709 */
  AOM_CICP_TC_UNSPECIFIED = 2, /**< Unspecified */
  AOM_CICP_TC_RESERVED_3 = 3,  /**< For future use */
  AOM_CICP_TC_BT_470_M = 4,    /**< BT.470 System M (historical)  */
  AOM_CICP_TC_BT_470_B_G = 5,  /**< BT.470 System B, G (historical) */
  AOM_CICP_TC_BT_601 = 6,      /**< BT.601 */
  AOM_CICP_TC_SMPTE_240 = 7,   /**< SMPTE 240 M */
  AOM_CICP_TC_LINEAR = 8,      /**< Linear */
  AOM_CICP_TC_LOG_100 = 9,     /**< Logarithmic (100 : 1 range) */
  AOM_CICP_TC_LOG_100_SQRT10 =
      10,                     /**< Logarithmic (100 * Sqrt(10) : 1 range) */
  AOM_CICP_TC_IEC_61966 = 11, /**< IEC 61966-2-4 */
  AOM_CICP_TC_BT_1361 = 12,   /**< BT.1361 */
  AOM_CICP_TC_SRGB = 13,      /**< sRGB or sYCC*/
  AOM_CICP_TC_BT_2020_10_BIT = 14, /**< BT.2020 10-bit systems */
  AOM_CICP_TC_BT_2020_12_BIT = 15, /**< BT.2020 12-bit systems */
  AOM_CICP_TC_SMPTE_2084 = 16,     /**< SMPTE ST 2084, ITU BT.2100 PQ */
  AOM_CICP_TC_SMPTE_428 = 17,      /**< SMPTE ST 428 */
  AOM_CICP_TC_HLG = 18,            /**< BT.2100 HLG, ARIB STD-B67 */
  AOM_CICP_TC_RESERVED_19 = 19     /**< For future use (values 19-255) */
} aom_transfer_characteristics_t;  /**< alias for enum
                                      aom_transfer_characteristics */

/*!\brief List of supported matrix coefficients */
typedef enum aom_matrix_coefficients {
  AOM_CICP_MC_IDENTITY = 0,    /**< Identity matrix */
  AOM_CICP_MC_BT_709 = 1,      /**< BT.709 */
  AOM_CICP_MC_UNSPECIFIED = 2, /**< Unspecified */
  AOM_CICP_MC_RESERVED_3 = 3,  /**< For future use */
  AOM_CICP_MC_FCC = 4,         /**< US FCC 73.628 */
  AOM_CICP_MC_BT_470_B_G = 5,  /**< BT.470 System B, G (historical) */
  AOM_CICP_MC_BT_601 = 6,      /**< BT.601 */
  AOM_CICP_MC_SMPTE_240 = 7,   /**< SMPTE 240 M */
  AOM_CICP_MC_SMPTE_YCGCO = 8, /**< YCgCo */
  AOM_CICP_MC_BT_2020_NCL =
      9, /**< BT.2020 non-constant luminance, BT.2100 YCbCr  */
  AOM_CICP_MC_BT_2020_CL = 10, /**< BT.2020 constant luminance */
  AOM_CICP_MC_SMPTE_2085 = 11, /**< SMPTE ST 2085 YDzDx */
  AOM_CICP_MC_CHROMAT_NCL =
      12, /**< Chromaticity-derived non-constant luminance */
  AOM_CICP_MC_CHROMAT_CL = 13, /**< Chromaticity-derived constant luminance */
  AOM_CICP_MC_ICTCP = 14,      /**< BT.2100 ICtCp */
  AOM_CICP_MC_RESERVED_15 = 15 /**< For future use (values 15-255)  */
} aom_matrix_coefficients_t;   /**< alias for enum aom_matrix_coefficients */

/*!\brief List of supported color range */
typedef enum aom_color_range {
  AOM_CR_STUDIO_RANGE = 0, /**<- Y  [16..235],  UV  [16..240]  (bit depth 8) */
                           /**<- Y  [64..940],  UV  [64..960]  (bit depth 10) */
                           /**<- Y [256..3760], UV [256..3840] (bit depth 12) */
  AOM_CR_FULL_RANGE = 1    /**<- YUV/RGB [0..255]  (bit depth 8) */
                           /**<- YUV/RGB [0..1023] (bit depth 10) */
                           /**<- YUV/RGB [0..4095] (bit depth 12) */
} aom_color_range_t;       /**< alias for enum aom_color_range */

/*!\brief List of chroma sample positions */
typedef enum aom_chroma_sample_position {
  AOM_CSP_UNKNOWN = 0,          /**< Unknown */
  AOM_CSP_VERTICAL = 1,         /**< Horizontally co-located with luma(0, 0)*/
                                /**< sample, between two vertical samples */
  AOM_CSP_COLOCATED = 2,        /**< Co-located with luma(0, 0) sample */
  AOM_CSP_RESERVED = 3          /**< Reserved value */
} aom_chroma_sample_position_t; /**< alias for enum aom_chroma_sample_position
                                 */

/*!\brief List of insert flags for Metadata
 *
 * These flags control how the library treats metadata during encode.
 *
 * While encoding, when metadata is added to an aom_image via
 * aom_img_add_metadata(), the flag passed along with the metadata will
 * determine where the metadata OBU will be placed in the encoded OBU stream,
 * and whether it's layer-specific. Metadata will be emitted into the output
 * stream within the next temporal unit if it satisfies the specified insertion
 * flag.
 *
 * If the video contains multiple spatial and/or temporal layers,
 * a layer-specific metadata OBU only applies to the current frame's layer, as
 * determined by the frame's temporal_id and spatial_id. Some metadata types
 * cannot be layer-specific, as listed in Section 6.7.1 of the draft AV1
 * specification as of 2025-03-06.
 *
 * During decoding, when the library encounters a metadata OBU, it is emitted
 * with the next output aom_image. Its insert_flag is set to either
 * AOM_MIF_ANY_FRAME, or AOM_MIF_ANY_FRAME_LAYER_SPECIFIC if the OBU contains an
 * OBU header extension (i.e. the video contains multiple layers AND the
 * metadata was added using *_LAYER_SPECIFC insert flag if using libaom).
 */
typedef enum aom_metadata_insert_flags {
  AOM_MIF_NON_KEY_FRAME = 0, /**< Adds metadata if it's not a keyframe */
  AOM_MIF_KEY_FRAME = 1,     /**< Adds metadata only if it's a keyframe */
  AOM_MIF_ANY_FRAME = 2,     /**< Adds metadata to any type of frame */
  /** Adds layer-specific metadata if it's not a keyframe */
  AOM_MIF_NON_KEY_FRAME_LAYER_SPECIFIC = 16,
  /** Adds layer-specific metadata only if it's a keyframe */
  AOM_MIF_KEY_FRAME_LAYER_SPECIFIC = 17,
  /** Adds layer-specific metadata to any type of frame */
  AOM_MIF_ANY_FRAME_LAYER_SPECIFIC = 18,
} aom_metadata_insert_flags_t;

/*!\brief Array of aom_metadata structs for an image. */
typedef struct aom_metadata_array aom_metadata_array_t;

/*!\brief Metadata payload. */
typedef struct aom_metadata {
  uint32_t type;                           /**< Metadata type */
  uint8_t *payload;                        /**< Metadata payload data */
  size_t sz;                               /**< Metadata payload size */
  aom_metadata_insert_flags_t insert_flag; /**< Metadata insertion flag */
} aom_metadata_t;

/**\brief Image Descriptor */
typedef struct aom_image {
  aom_img_fmt_t fmt;                 /**< Image Format */
  aom_color_primaries_t cp;          /**< CICP Color Primaries */
  aom_transfer_characteristics_t tc; /**< CICP Transfer Characteristics */
  aom_matrix_coefficients_t mc;      /**< CICP Matrix Coefficients */
  int monochrome;                    /**< Whether image is monochrome */
  aom_chroma_sample_position_t csp;  /**< chroma sample position */
  aom_color_range_t range;           /**< Color Range */

  /* Image storage dimensions */
  unsigned int w;         /**< Stored image width */
  unsigned int h;         /**< Stored image height */
  unsigned int bit_depth; /**< Stored image bit-depth */

  /* Image display dimensions */
  unsigned int d_w; /**< Displayed image width */
  unsigned int d_h; /**< Displayed image height */

  /* Image intended rendering dimensions */
  unsigned int r_w; /**< Intended rendering image width */
  unsigned int r_h; /**< Intended rendering image height */

  /* Chroma subsampling info */
  unsigned int x_chroma_shift; /**< subsampling order, X */
  unsigned int y_chroma_shift; /**< subsampling order, Y */

/* Image data pointers. */
#define AOM_PLANE_PACKED 0 /**< To be used for all packed formats */
#define AOM_PLANE_Y 0      /**< Y (Luminance) plane */
#define AOM_PLANE_U 1      /**< U (Chroma) plane */
#define AOM_PLANE_V 2      /**< V (Chroma) plane */
  /* planes[AOM_PLANE_V] = NULL and stride[AOM_PLANE_V] = 0 when fmt ==
   * AOM_IMG_FMT_NV12 */
  unsigned char *planes[3]; /**< pointer to the top left pixel for each plane */
  int stride[3];            /**< stride between rows for each plane */
  size_t sz;                /**< data size */

  int bps; /**< bits per sample (for packed formats) */

  int temporal_id; /**< Temporal layer Id of image */
  int spatial_id;  /**< Spatial layer Id of image */

  /*!\brief The following member may be set by the application to associate
   * data with this image.
   */
  void *user_priv;

  /* The following members should be treated as private. */
  unsigned char *img_data; /**< private */
  int img_data_owner;      /**< private */
  int self_allocd;         /**< private */

  aom_metadata_array_t
      *metadata; /**< Metadata payloads associated with the image. */

  void *fb_priv; /**< Frame buffer data associated with the image. */
} aom_image_t;   /**< alias for struct aom_image */

/*!\brief Open a descriptor, allocating storage for the underlying image
 *
 * Returns a descriptor for storing an image of the given format. The
 * storage for the image is allocated on the heap.
 *
 * \param[in]    img       Pointer to storage for descriptor. If this parameter
 *                         is NULL, the storage for the descriptor will be
 *                         allocated on the heap.
 * \param[in]    fmt       Format for the image
 * \param[in]    d_w       Width of the image. Must not exceed 0x08000000
 *                         (2^27).
 * \param[in]    d_h       Height of the image. Must not exceed 0x08000000
 *                         (2^27).
 * \param[in]    align     Alignment, in bytes, of the image buffer and
 *                         each row in the image (stride). Must not exceed
 *                         65536.
 *
 * \return Returns a pointer to the initialized image descriptor. If the img
 *         parameter is non-null, the value of the img parameter will be
 *         returned.
 */
aom_image_t *aom_img_alloc(aom_image_t *img, aom_img_fmt_t fmt,
                           unsigned int d_w, unsigned int d_h,
                           unsigned int align);

/*!\brief Open a descriptor, using existing storage for the underlying image
 *
 * Returns a descriptor for storing an image of the given format. The
 * storage for the image has been allocated elsewhere, and a descriptor is
 * desired to "wrap" that storage.
 *
 * \param[in]    img       Pointer to storage for descriptor. If this parameter
 *                         is NULL, the storage for the descriptor will be
 *                         allocated on the heap.
 * \param[in]    fmt       Format for the image
 * \param[in]    d_w       Width of the image. Must not exceed 0x08000000
 *                         (2^27).
 * \param[in]    d_h       Height of the image. Must not exceed 0x08000000
 *                         (2^27).
 * \param[in]    align     Alignment, in bytes, of each row in the image
 *                         (stride). Must not exceed 65536.
 * \param[in]    img_data  Storage to use for the image. The storage must
 *                         outlive the returned image descriptor; it can be
 *                         disposed of after calling aom_img_free().
 *
 * \return Returns a pointer to the initialized image descriptor. If the img
 *         parameter is non-null, the value of the img parameter will be
 *         returned.
 */
aom_image_t *aom_img_wrap(aom_image_t *img, aom_img_fmt_t fmt, unsigned int d_w,
                          unsigned int d_h, unsigned int align,
                          unsigned char *img_data);

/*!\brief Open a descriptor, allocating storage for the underlying image with a
 * border
 *
 * Returns a descriptor for storing an image of the given format and its
 * borders. The storage for the image is allocated on the heap.
 *
 * \param[in]    img        Pointer to storage for descriptor. If this parameter
 *                          is NULL, the storage for the descriptor will be
 *                          allocated on the heap.
 * \param[in]    fmt        Format for the image
 * \param[in]    d_w        Width of the image. Must not exceed 0x08000000
 *                          (2^27).
 * \param[in]    d_h        Height of the image. Must not exceed 0x08000000
 *                          (2^27).
 * \param[in]    align      Alignment, in bytes, of the image buffer and
 *                          each row in the image (stride). Must not exceed
 *                          65536.
 * \param[in]    size_align Alignment, in pixels, of the image width and height.
 *                          Must not exceed 65536.
 * \param[in]    border     A border that is padded on four sides of the image.
 *                          Must not exceed 65536.
 *
 * \return Returns a pointer to the initialized image descriptor. If the img
 *         parameter is non-null, the value of the img parameter will be
 *         returned.
 */
aom_image_t *aom_img_alloc_with_border(aom_image_t *img, aom_img_fmt_t fmt,
                                       unsigned int d_w, unsigned int d_h,
                                       unsigned int align,
                                       unsigned int size_align,
                                       unsigned int border);

/*!\brief Set the rectangle identifying the displayed portion of the image
 *
 * Updates the displayed rectangle (aka viewport) on the image surface to
 * match the specified coordinates and size. Specifically, sets img->d_w,
 * img->d_h, and elements of the img->planes[] array.
 *
 * \param[in]    img       Image descriptor
 * \param[in]    x         leftmost column
 * \param[in]    y         topmost row
 * \param[in]    w         width
 * \param[in]    h         height
 * \param[in]    border    A border that is padded on four sides of the image.
 *
 * \return 0 if the requested rectangle is valid, nonzero (-1) otherwise.
 */
int aom_img_set_rect(aom_image_t *img, unsigned int x, unsigned int y,
                     unsigned int w, unsigned int h, unsigned int border);

/*!\brief Flip the image vertically (top for bottom)
 *
 * Adjusts the image descriptor's pointers and strides to make the image
 * be referenced upside-down.
 *
 * \param[in]    img       Image descriptor
 */
void aom_img_flip(aom_image_t *img);

/*!\brief Close an image descriptor
 *
 * Frees all allocated storage associated with an image descriptor.
 *
 * \param[in]    img       Image descriptor
 */
void aom_img_free(aom_image_t *img);

/*!\brief Get the width of a plane
 *
 * Get the width of a plane of an image
 *
 * \param[in]    img       Image descriptor
 * \param[in]    plane     Plane index
 */
int aom_img_plane_width(const aom_image_t *img, int plane);

/*!\brief Get the height of a plane
 *
 * Get the height of a plane of an image
 *
 * \param[in]    img       Image descriptor
 * \param[in]    plane     Plane index
 */
int aom_img_plane_height(const aom_image_t *img, int plane);

/*!\brief Add metadata to image.
 *
 * Adds metadata to aom_image_t.
 * Function makes a copy of the provided data parameter.
 * Metadata insertion point is controlled by insert_flag.
 *
 * \param[in]    img          Image descriptor
 * \param[in]    type         Metadata type
 * \param[in]    data         Metadata contents
 * \param[in]    sz           Metadata contents size
 * \param[in]    insert_flag  Metadata insert flag
 *
 * \return Returns 0 on success. If img or data is NULL, sz is 0, or memory
 * allocation fails, it returns -1.
 */
int aom_img_add_metadata(aom_image_t *img, uint32_t type, const uint8_t *data,
                         size_t sz, aom_metadata_insert_flags_t insert_flag);

/*!\brief Return a metadata payload stored within the image metadata array.
 *
 * Gets the metadata (aom_metadata_t) at the indicated index in the image
 * metadata array.
 *
 * \param[in] img          Pointer to image descriptor to get metadata from
 * \param[in] index        Metadata index to get from metadata array
 *
 * \return Returns a const pointer to the selected metadata, if img and/or index
 * is invalid, it returns NULL.
 */
const aom_metadata_t *aom_img_get_metadata(const aom_image_t *img,
                                           size_t index);

/*!\brief Return the number of metadata blocks within the image.
 *
 * Gets the number of metadata blocks contained within the provided image
 * metadata array.
 *
 * \param[in] img          Pointer to image descriptor to get metadata number
 * from.
 *
 * \return Returns the size of the metadata array. If img or metadata is NULL,
 * it returns 0.
 */
size_t aom_img_num_metadata(const aom_image_t *img);

/*!\brief Remove metadata from image.
 *
 * Removes all metadata in image metadata list and sets metadata list pointer
 * to NULL.
 *
 * \param[in]    img       Image descriptor
 */
void aom_img_remove_metadata(aom_image_t *img);

/*!\brief Allocate memory for aom_metadata struct.
 *
 * Allocates storage for the metadata payload, sets its type and copies the
 * payload data into the aom_metadata struct. A metadata payload buffer of size
 * sz is allocated and sz bytes are copied from data into the payload buffer.
 *
 * \param[in]    type         Metadata type
 * \param[in]    data         Metadata data pointer
 * \param[in]    sz           Metadata size
 * \param[in]    insert_flag  Metadata insert flag
 *
 * \return Returns the newly allocated aom_metadata struct. If data is NULL,
 * sz is 0, or memory allocation fails, it returns NULL.
 */
aom_metadata_t *aom_img_metadata_alloc(uint32_t type, const uint8_t *data,
                                       size_t sz,
                                       aom_metadata_insert_flags_t insert_flag);

/*!\brief Free metadata struct.
 *
 * Free metadata struct and its buffer.
 *
 * \param[in]    metadata       Metadata struct pointer
 */
void aom_img_metadata_free(aom_metadata_t *metadata);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AOM_AOM_IMAGE_H_
