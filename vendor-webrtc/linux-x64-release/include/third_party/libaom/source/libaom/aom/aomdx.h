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

/*!\defgroup aom_decoder AOMedia AOM/AV1 Decoder
 * \ingroup aom
 *
 * @{
 */
/*!\file
 * \brief Provides definitions for using AOM or AV1 within the aom Decoder
 *        interface.
 */
#ifndef AOM_AOM_AOMDX_H_
#define AOM_AOM_AOMDX_H_

#ifdef __cplusplus
extern "C" {
#endif

/* Include controls common to both the encoder and decoder */
#include "aom/aom.h"

/*!\name Algorithm interface for AV1
 *
 * This interface provides the capability to decode AV1 streams.
 * @{
 */

/*!\brief A single instance of the AV1 decoder.
 *\deprecated This access mechanism is provided for backwards compatibility;
 * prefer aom_codec_av1_dx().
 */
extern aom_codec_iface_t aom_codec_av1_dx_algo;
/*!\brief The interface to the AV1 decoder.
 */
extern aom_codec_iface_t *aom_codec_av1_dx(void);

/*!@} - end algorithm interface member group */

/** Data structure that stores bit accounting for debug
 */
typedef struct Accounting Accounting;

#ifndef AOM_INSPECTION_H_
/** Callback that inspects decoder frame data.
 */
typedef void (*aom_inspect_cb)(void *decoder, void *ctx);

#endif

/*!\brief Structure to hold inspection callback and context.
 *
 * Defines a structure to hold the inspection callback function and calling
 * context.
 */
typedef struct aom_inspect_init {
  /*! Inspection callback. */
  aom_inspect_cb inspect_cb;

  /*! Inspection context. */
  void *inspect_ctx;
} aom_inspect_init;

/*!\brief Structure to collect a buffer index when inspecting.
 *
 * Defines a structure to hold the buffer and return an index
 * when calling decode from inspect. This enables us to decode
 * non showable sub frames.
 */
typedef struct {
  /*! Pointer for new position in compressed buffer after decoding 1 OBU. */
  const unsigned char *buf;
  /*! Index into reference buffer array to see result of decoding 1 OBU. */
  int idx;
  /*! Is a show existing frame. */
  int show_existing;
} Av1DecodeReturn;

/*!\brief Structure to hold a tile's start address and size in the bitstream.
 *
 * Defines a structure to hold a tile's start address and size in the bitstream.
 */
typedef struct aom_tile_data {
  /*! Tile data size. */
  size_t coded_tile_data_size;
  /*! Tile's start address. */
  const void *coded_tile_data;
  /*! Extra size information. */
  size_t extra_size;
} aom_tile_data;

/*!\brief Max number of tile columns
 *
 * This is the limit of number of tile columns allowed within a frame.
 *
 * Currently same as "MAX_TILE_COLS" in AV1, the maximum that AV1 supports.
 *
 */
#define AOM_MAX_TILE_COLS 64
/*!\brief Max number of tile rows
 *
 * This is the limit of number of tile rows allowed within a frame.
 *
 * Currently same as "MAX_TILE_ROWS" in AV1, the maximum that AV1 supports.
 *
 */
#define AOM_MAX_TILE_ROWS 64

/*!\brief Structure to hold information about tiles in a frame.
 *
 * Defines a structure to hold a frame's tile information, namely
 * number of tile columns, number of tile_rows, and the width and
 * height of each tile.
 */
typedef struct aom_tile_info {
  /*! Indicates the number of tile columns. */
  int tile_columns;
  /*! Indicates the number of tile rows. */
  int tile_rows;
  /*! Indicates the tile widths in units of SB. */
  int tile_widths[AOM_MAX_TILE_COLS];
  /*! Indicates the tile heights in units of SB. */
  int tile_heights[AOM_MAX_TILE_ROWS];
  /*! Indicates the number of tile groups present in a frame. */
  int num_tile_groups;
} aom_tile_info;

/*!\brief Structure to hold information about still image coding.
 *
 * Defines a structure to hold a information regarding still picture
 * and its header type.
 */
typedef struct aom_still_picture_info {
  /*! Video is a single frame still picture */
  int is_still_picture;
  /*! Use full header for still picture */
  int is_reduced_still_picture_hdr;
} aom_still_picture_info;

/*!\brief Structure to hold information about S_FRAME.
 *
 * Defines a structure to hold a information regarding S_FRAME
 * and its position.
 */
typedef struct aom_s_frame_info {
  /*! Indicates if current frame is S_FRAME */
  int is_s_frame;
  /*! Indicates if current S_FRAME is present at ALTREF frame*/
  int is_s_frame_at_altref;
} aom_s_frame_info;

/*!\brief Structure to hold information about screen content tools.
 *
 * Defines a structure to hold information about screen content
 * tools, namely: allow_screen_content_tools, allow_intrabc, and
 * force_integer_mv.
 */
typedef struct aom_screen_content_tools_info {
  /*! Are screen content tools allowed */
  int allow_screen_content_tools;
  /*! Is intrabc allowed */
  int allow_intrabc;
  /*! Is integer mv forced */
  int force_integer_mv;
} aom_screen_content_tools_info;

/*!\brief Structure to hold the external reference frame pointer.
 *
 * Define a structure to hold the external reference frame pointer.
 */
typedef struct av1_ext_ref_frame {
  /*! Start pointer of external references. */
  aom_image_t *img;
  /*! Number of available external references. */
  int num;
} av1_ext_ref_frame_t;

/*!\enum aom_dec_control_id
 * \brief AOM decoder control functions
 *
 * This set of macros define the control functions available for the AOM
 * decoder interface.
 * The range for decoder control ID is >= 256.
 *
 * \sa #aom_codec_control(aom_codec_ctx_t *ctx, int ctrl_id, ...)
 */
enum aom_dec_control_id {
  /*!\brief Codec control function to get info on which reference frames were
   * updated by the last decode, int* parameter
   */
  AOMD_GET_LAST_REF_UPDATES = AOM_DECODER_CTRL_ID_START,

  /*!\brief Codec control function to check if the indicated frame is
    corrupted, int* parameter
  */
  AOMD_GET_FRAME_CORRUPTED,

  /*!\brief Codec control function to get info on which reference frames were
   * used by the last decode, int* parameter
   */
  AOMD_GET_LAST_REF_USED,

  /*!\brief Codec control function to get the dimensions that the current
   * frame is decoded at, int* parameter
   *
   * This may be different to the intended display size for the frame as
   * specified in the wrapper or frame header (see AV1D_GET_DISPLAY_SIZE).
   */
  AV1D_GET_FRAME_SIZE,

  /*!\brief Codec control function to get the current frame's intended display
   * dimensions (as specified in the wrapper or frame header), int* parameter
   *
   * This may be different to the decoded dimensions of this frame (see
   * AV1D_GET_FRAME_SIZE).
   */
  AV1D_GET_DISPLAY_SIZE,

  /*!\brief Codec control function to get the bit depth of the stream,
   * unsigned int* parameter
   */
  AV1D_GET_BIT_DEPTH,

  /*!\brief Codec control function to get the image format of the stream,
   * aom_img_fmt_t* parameter
   */
  AV1D_GET_IMG_FORMAT,

  /*!\brief Codec control function to get the width and height (in pixels) of
   * the tiles in a tile list, unsigned int* parameter
   *
   * Tile width is in the high 16 bits of the output value, and tile height is
   * in the low 16 bits of the output value.
   */
  AV1D_GET_TILE_SIZE,

  /*!\brief Codec control function to get the tile count in a tile list,
   * unsigned int* parameter
   */
  AV1D_GET_TILE_COUNT,

  /*!\brief Codec control function to set the byte alignment of the planes in
   * the reference buffers, int parameter
   *
   * Valid values are power of 2, from 32 to 1024. A value of 0 sets
   * legacy alignment. I.e. Y plane is aligned to 32 bytes, U plane directly
   * follows Y plane, and V plane directly follows U plane. Default value is 0.
   */
  AV1_SET_BYTE_ALIGNMENT,

  /*!\brief Codec control function to invert the decoding order to from right to
   * left, int parameter
   *
   * The function is used in a test to confirm the decoding independence of tile
   * columns. The function may be used in application where this order
   * of decoding is desired. int parameter
   *
   * TODO(yaowu): Rework the unit test that uses this control, and in a future
   *              release, this test-only control shall be removed.
   */
  AV1_INVERT_TILE_DECODE_ORDER,

  /*!\brief Codec control function to set the skip loop filter flag, int
   * parameter
   *
   * Valid values are integers. The decoder will skip the loop filter
   * when its value is set to nonzero. If the loop filter is skipped the
   * decoder may accumulate decode artifacts. The default value is 0.
   */
  AV1_SET_SKIP_LOOP_FILTER,

  /*!\brief Codec control function to retrieve a pointer to the Accounting
   * struct, takes Accounting** as parameter
   *
   * If called before a frame has been decoded, this returns AOM_CODEC_ERROR.
   * The caller should ensure that AOM_CODEC_OK is returned before attempting
   * to dereference the Accounting pointer.
   *
   * \attention When configured with -DCONFIG_ACCOUNTING=0, the default, this
   * returns AOM_CODEC_INCAPABLE.
   */
  AV1_GET_ACCOUNTING,

  /*!\brief Codec control function to get last decoded frame quantizer,
   * int* parameter
   *
   * Returned value uses internal quantizer scale defined by the codec.
   */
  AOMD_GET_LAST_QUANTIZER,

  /*!\brief Codec control function to set the range of tile decoding, int
   * parameter
   *
   * A value that is greater and equal to zero indicates only the specific
   * row/column is decoded. A value that is -1 indicates the whole row/column
   * is decoded. A special case is both values are -1 that means the whole
   * frame is decoded.
   */
  AV1_SET_DECODE_TILE_ROW,
  AV1_SET_DECODE_TILE_COL,

  /*!\brief Codec control function to set the tile coding mode, unsigned int
   * parameter
   *
   * - 0 = tiles are coded in normal tile mode
   * - 1 = tiles are coded in large-scale tile mode
   */
  AV1_SET_TILE_MODE,

  /*!\brief Codec control function to get the frame header information of an
   * encoded frame, aom_tile_data* parameter
   */
  AV1D_GET_FRAME_HEADER_INFO,

  /*!\brief Codec control function to get the start address and size of a
   * tile in the coded bitstream, aom_tile_data* parameter.
   */
  AV1D_GET_TILE_DATA,

  /*!\brief Codec control function to set the external references' pointers in
   * the decoder, av1_ext_ref_frame_t* parameter.
   *
   * This is used while decoding the tile list OBU in large-scale tile coding
   * mode.
   */
  AV1D_SET_EXT_REF_PTR,

  /*!\brief Codec control function to enable the ext-tile software debug and
   * testing code in the decoder, unsigned int parameter
   */
  AV1D_EXT_TILE_DEBUG,

  /*!\brief Codec control function to enable the row based multi-threading of
   * decoding, unsigned int parameter
   *
   * - 0 = disabled
   * - 1 = enabled (default)
   */
  AV1D_SET_ROW_MT,

  /*!\brief Codec control function to indicate whether bitstream is in
   * Annex-B format, unsigned int parameter
   */
  AV1D_SET_IS_ANNEXB,

  /*!\brief Codec control function to indicate which operating point to use,
   * int parameter
   *
   * A scalable stream may define multiple operating points, each of which
   * defines a set of temporal and spatial layers to be processed. The
   * operating point index may take a value between 0 and
   * operating_points_cnt_minus_1 (which is at most 31).
   */
  AV1D_SET_OPERATING_POINT,

  /*!\brief Codec control function to indicate whether to output one frame per
   * temporal unit (the default), or one frame per spatial layer, int parameter
   *
   * In a scalable stream, each temporal unit corresponds to a single "frame"
   * of video, and within a temporal unit there may be multiple spatial layers
   * with different versions of that frame.
   * For video playback, only the highest-quality version (within the
   * selected operating point) is needed, but for some use cases it is useful
   * to have access to multiple versions of a frame when they are available.
   */
  AV1D_SET_OUTPUT_ALL_LAYERS,

  /*!\brief Codec control function to set an aom_inspect_cb callback that is
   * invoked each time a frame is decoded, aom_inspect_init* parameter
   *
   * \attention When configured with -DCONFIG_INSPECTION=0, the default, this
   * returns AOM_CODEC_INCAPABLE.
   */
  AV1_SET_INSPECTION_CALLBACK,

  /*!\brief Codec control function to set the skip film grain flag, int
   * parameter
   *
   * Valid values are integers. The decoder will skip the film grain when its
   * value is set to nonzero. The default value is 0.
   */
  AV1D_SET_SKIP_FILM_GRAIN,

  /*!\brief Codec control function to check the presence of forward key frames,
   * int* parameter
   */
  AOMD_GET_FWD_KF_PRESENT,

  /*!\brief Codec control function to get the frame flags of the previous frame
   * decoded, int* parameter
   *
   * This will return a flag of type aom_codec_frame_flags_t.
   */
  AOMD_GET_FRAME_FLAGS,

  /*!\brief Codec control function to check the presence of altref frames, int*
   * parameter
   */
  AOMD_GET_ALTREF_PRESENT,

  /*!\brief Codec control function to get tile information of the previous frame
   * decoded, aom_tile_info* parameter
   *
   * This will return a struct of type aom_tile_info.
   */
  AOMD_GET_TILE_INFO,

  /*!\brief Codec control function to get screen content tools information,
   * aom_screen_content_tools_info* parameter
   *
   * It returns a struct of type aom_screen_content_tools_info, which contains
   * the header flags allow_screen_content_tools, allow_intrabc, and
   * force_integer_mv.
   */
  AOMD_GET_SCREEN_CONTENT_TOOLS_INFO,

  /*!\brief Codec control function to get the still picture coding information,
   * aom_still_picture_info* parameter
   */
  AOMD_GET_STILL_PICTURE,

  /*!\brief Codec control function to get superblock size,
   * aom_superblock_size_t* parameter
   *
   * It returns an enum, indicating the superblock size read from the sequence
   * header(0 for BLOCK_64X64 and 1 for BLOCK_128X128)
   */
  AOMD_GET_SB_SIZE,

  /*!\brief Codec control function to check if the previous frame
   * decoded has show existing frame flag set, int* parameter
   */
  AOMD_GET_SHOW_EXISTING_FRAME_FLAG,

  /*!\brief Codec control function to get the S_FRAME coding information,
   * aom_s_frame_info* parameter
   */
  AOMD_GET_S_FRAME_INFO,

  /*!\brief Codec control function to get the show frame flag, int* parameter
   */
  AOMD_GET_SHOW_FRAME_FLAG,

  /*!\brief Codec control function to get the base q index of a frame, int*
   * parameter
   */
  AOMD_GET_BASE_Q_IDX,

  /*!\brief Codec control function to get the order hint of a frame, unsigned
   * int* parameter
   */
  AOMD_GET_ORDER_HINT,

  /*!\brief Codec control function to get the info of a 4x4 block.
   * Parameters: int mi_row, int mi_col, and MB_MODE_INFO*.
   *
   * \note This only returns a shallow copy, so all pointer members should not
   * be used.
   */
  AV1D_GET_MI_INFO,
};

/*!\cond */
/*!\brief AOM decoder control function parameter type
 *
 * Defines the data types that AOMD control functions take.
 *
 * \note Additional common controls are defined in aom.h.
 *
 * \note For each control ID "X", a macro-define of
 * AOM_CTRL_X is provided. It is used at compile time to determine
 * if the control ID is supported by the libaom library available,
 * when the libaom version cannot be controlled.
 */
AOM_CTRL_USE_TYPE(AOMD_GET_LAST_REF_UPDATES, int *)
#define AOM_CTRL_AOMD_GET_LAST_REF_UPDATES

AOM_CTRL_USE_TYPE(AOMD_GET_FRAME_CORRUPTED, int *)
#define AOM_CTRL_AOMD_GET_FRAME_CORRUPTED

AOM_CTRL_USE_TYPE(AOMD_GET_LAST_REF_USED, int *)
#define AOM_CTRL_AOMD_GET_LAST_REF_USED

AOM_CTRL_USE_TYPE(AV1D_GET_FRAME_SIZE, int *)
#define AOM_CTRL_AV1D_GET_FRAME_SIZE

AOM_CTRL_USE_TYPE(AV1D_GET_DISPLAY_SIZE, int *)
#define AOM_CTRL_AV1D_GET_DISPLAY_SIZE

AOM_CTRL_USE_TYPE(AV1D_GET_BIT_DEPTH, unsigned int *)
#define AOM_CTRL_AV1D_GET_BIT_DEPTH

AOM_CTRL_USE_TYPE(AV1D_GET_IMG_FORMAT, aom_img_fmt_t *)
#define AOM_CTRL_AV1D_GET_IMG_FORMAT

AOM_CTRL_USE_TYPE(AV1D_GET_TILE_SIZE, unsigned int *)
#define AOM_CTRL_AV1D_GET_TILE_SIZE

AOM_CTRL_USE_TYPE(AV1D_GET_TILE_COUNT, unsigned int *)
#define AOM_CTRL_AV1D_GET_TILE_COUNT

AOM_CTRL_USE_TYPE(AV1_INVERT_TILE_DECODE_ORDER, int)
#define AOM_CTRL_AV1_INVERT_TILE_DECODE_ORDER

AOM_CTRL_USE_TYPE(AV1_SET_SKIP_LOOP_FILTER, int)
#define AOM_CTRL_AV1_SET_SKIP_LOOP_FILTER

AOM_CTRL_USE_TYPE(AV1_GET_ACCOUNTING, Accounting **)
#define AOM_CTRL_AV1_GET_ACCOUNTING

AOM_CTRL_USE_TYPE(AOMD_GET_LAST_QUANTIZER, int *)
#define AOM_CTRL_AOMD_GET_LAST_QUANTIZER

AOM_CTRL_USE_TYPE(AV1_SET_DECODE_TILE_ROW, int)
#define AOM_CTRL_AV1_SET_DECODE_TILE_ROW

AOM_CTRL_USE_TYPE(AV1_SET_DECODE_TILE_COL, int)
#define AOM_CTRL_AV1_SET_DECODE_TILE_COL

AOM_CTRL_USE_TYPE(AV1_SET_TILE_MODE, unsigned int)
#define AOM_CTRL_AV1_SET_TILE_MODE

AOM_CTRL_USE_TYPE(AV1D_GET_FRAME_HEADER_INFO, aom_tile_data *)
#define AOM_CTRL_AV1D_GET_FRAME_HEADER_INFO

AOM_CTRL_USE_TYPE(AV1D_GET_TILE_DATA, aom_tile_data *)
#define AOM_CTRL_AV1D_GET_TILE_DATA

AOM_CTRL_USE_TYPE(AV1D_SET_EXT_REF_PTR, av1_ext_ref_frame_t *)
#define AOM_CTRL_AV1D_SET_EXT_REF_PTR

AOM_CTRL_USE_TYPE(AV1D_EXT_TILE_DEBUG, unsigned int)
#define AOM_CTRL_AV1D_EXT_TILE_DEBUG

AOM_CTRL_USE_TYPE(AV1D_SET_ROW_MT, unsigned int)
#define AOM_CTRL_AV1D_SET_ROW_MT

AOM_CTRL_USE_TYPE(AV1D_SET_IS_ANNEXB, unsigned int)
#define AOM_CTRL_AV1D_SET_IS_ANNEXB

AOM_CTRL_USE_TYPE(AV1D_SET_OPERATING_POINT, int)
#define AOM_CTRL_AV1D_SET_OPERATING_POINT

AOM_CTRL_USE_TYPE(AV1D_SET_OUTPUT_ALL_LAYERS, int)
#define AOM_CTRL_AV1D_SET_OUTPUT_ALL_LAYERS

AOM_CTRL_USE_TYPE(AV1_SET_INSPECTION_CALLBACK, aom_inspect_init *)
#define AOM_CTRL_AV1_SET_INSPECTION_CALLBACK

AOM_CTRL_USE_TYPE(AV1D_SET_SKIP_FILM_GRAIN, int)
#define AOM_CTRL_AV1D_SET_SKIP_FILM_GRAIN

AOM_CTRL_USE_TYPE(AOMD_GET_FWD_KF_PRESENT, int *)
#define AOM_CTRL_AOMD_GET_FWD_KF_PRESENT

AOM_CTRL_USE_TYPE(AOMD_GET_FRAME_FLAGS, int *)
#define AOM_CTRL_AOMD_GET_FRAME_FLAGS

AOM_CTRL_USE_TYPE(AOMD_GET_ALTREF_PRESENT, int *)
#define AOM_CTRL_AOMD_GET_ALTREF_PRESENT

AOM_CTRL_USE_TYPE(AOMD_GET_TILE_INFO, aom_tile_info *)
#define AOM_CTRL_AOMD_GET_TILE_INFO

AOM_CTRL_USE_TYPE(AOMD_GET_SCREEN_CONTENT_TOOLS_INFO,
                  aom_screen_content_tools_info *)
#define AOM_CTRL_AOMD_GET_SCREEN_CONTENT_TOOLS_INFO

AOM_CTRL_USE_TYPE(AOMD_GET_STILL_PICTURE, aom_still_picture_info *)
#define AOM_CTRL_AOMD_GET_STILL_PICTURE

AOM_CTRL_USE_TYPE(AOMD_GET_SB_SIZE, aom_superblock_size_t *)
#define AOMD_CTRL_AOMD_GET_SB_SIZE

AOM_CTRL_USE_TYPE(AOMD_GET_SHOW_EXISTING_FRAME_FLAG, int *)
#define AOMD_CTRL_AOMD_GET_SHOW_EXISTING_FRAME_FLAG

AOM_CTRL_USE_TYPE(AOMD_GET_S_FRAME_INFO, aom_s_frame_info *)
#define AOMD_CTRL_AOMD_GET_S_FRAME_INFO

AOM_CTRL_USE_TYPE(AOMD_GET_SHOW_FRAME_FLAG, int *)
#define AOM_CTRL_AOMD_GET_SHOW_FRAME_FLAG

AOM_CTRL_USE_TYPE(AOMD_GET_BASE_Q_IDX, int *)
#define AOM_CTRL_AOMD_GET_BASE_Q_IDX

AOM_CTRL_USE_TYPE(AOMD_GET_ORDER_HINT, unsigned int *)
#define AOM_CTRL_AOMD_GET_ORDER_HINT

// The AOM_CTRL_USE_TYPE macro can't be used with AV1D_GET_MI_INFO because
// AV1D_GET_MI_INFO takes more than one parameter.
#define AOM_CTRL_AV1D_GET_MI_INFO
/*!\endcond */
/*! @} - end defgroup aom_decoder */
#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AOM_AOMDX_H_
