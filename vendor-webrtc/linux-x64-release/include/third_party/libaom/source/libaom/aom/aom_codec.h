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

///////////////////////////////////////////////////////////////////////////////
// Internal implementation details
///////////////////////////////////////////////////////////////////////////////
//
// There are two levels of interfaces used to access the AOM codec: the
// aom_codec_iface and the aom_codec_ctx.
//
// 1. aom_codec_iface_t
//    (Related files: aom/aom_codec.h, aom/src/aom_codec.c,
//    aom/internal/aom_codec_internal.h, av1/av1_cx_iface.c,
//    av1/av1_dx_iface.c)
//
// Used to initialize the codec context, which contains the configuration for
// for modifying the encoder/decoder during run-time. See the other
// documentation in this header file for more details. For the most part,
// users will call helper functions, such as aom_codec_iface_name,
// aom_codec_get_caps, etc., to interact with it.
//
// The main purpose of the aom_codec_iface_t is to provide a way to generate
// a default codec config, find out what capabilities the implementation has,
// and create an aom_codec_ctx_t (which is actually used to interact with the
// codec).
//
// Note that the implementations for the AV1 algorithm are located in
// av1/av1_cx_iface.c and av1/av1_dx_iface.c
//
//
// 2. aom_codec_ctx_t
//  (Related files: aom/aom_codec.h, av1/av1_cx_iface.c, av1/av1_dx_iface.c,
//   aom/aomcx.h, aom/aomdx.h, aom/src/aom_encoder.c, aom/src/aom_decoder.c)
//
// The actual interface between user code and the codec. It stores the name
// of the codec, a pointer back to the aom_codec_iface_t that initialized it,
// initialization flags, a config for either encoder or the decoder, and a
// pointer to internal data.
//
// The codec is configured / queried through calls to aom_codec_control,
// which takes a control ID (listed in aomcx.h and aomdx.h) and a parameter.
// In the case of "getter" control IDs, the parameter is modified to have
// the requested value; in the case of "setter" control IDs, the codec's
// configuration is changed based on the parameter. Note that a aom_codec_err_t
// is returned, which indicates if the operation was successful or not.
//
// Note that for the encoder, the aom_codec_alg_priv_t points to the
// the aom_codec_alg_priv structure in av1/av1_cx_iface.c, and for the decoder,
// the struct in av1/av1_dx_iface.c. Variables such as AV1_COMP cpi are stored
// here and also used in the core algorithm.
//
// At the end, aom_codec_destroy should be called for each initialized
// aom_codec_ctx_t.

/*!\defgroup codec Common Algorithm Interface
 * This abstraction allows applications to easily support multiple video
 * formats with minimal code duplication. This section describes the interface
 * common to all codecs (both encoders and decoders).
 * @{
 */

/*!\file
 * \brief Describes the codec algorithm interface to applications.
 *
 * This file describes the interface between an application and a
 * video codec algorithm.
 *
 * An application instantiates a specific codec instance by using
 * aom_codec_dec_init() or aom_codec_enc_init() and a pointer to the
 * algorithm's interface structure:
 *     <pre>
 *     my_app.c:
 *       extern aom_codec_iface_t my_codec;
 *       {
 *           aom_codec_ctx_t algo;
 *           int threads = 4;
 *           aom_codec_dec_cfg_t cfg = { threads, 0, 0, 1 };
 *           res = aom_codec_dec_init(&algo, &my_codec, &cfg, 0);
 *       }
 *     </pre>
 *
 * Once initialized, the instance is managed using other functions from
 * the aom_codec_* family.
 */
#ifndef AOM_AOM_AOM_CODEC_H_
#define AOM_AOM_AOM_CODEC_H_

#ifdef __cplusplus
extern "C" {
#endif

#include "aom/aom_image.h"
#include "aom/aom_integer.h"

/*!\brief Decorator indicating a function is deprecated */
#ifndef AOM_DEPRECATED
#if defined(__GNUC__)
#define AOM_DEPRECATED __attribute__((deprecated))
#elif defined(_MSC_VER)
#define AOM_DEPRECATED
#else
#define AOM_DEPRECATED
#endif
#endif /* AOM_DEPRECATED */

#ifndef AOM_DECLSPEC_DEPRECATED
#if defined(__GNUC__)
#define AOM_DECLSPEC_DEPRECATED /**< \copydoc #AOM_DEPRECATED */
#elif defined(_MSC_VER)
/*!\brief \copydoc #AOM_DEPRECATED */
#define AOM_DECLSPEC_DEPRECATED __declspec(deprecated)
#else
#define AOM_DECLSPEC_DEPRECATED /**< \copydoc #AOM_DEPRECATED */
#endif
#endif /* AOM_DECLSPEC_DEPRECATED */

/*!\brief Decorator indicating a function is potentially unused */
#ifdef AOM_UNUSED
#elif defined(__GNUC__) || defined(__clang__)
#define AOM_UNUSED __attribute__((unused))
#else
#define AOM_UNUSED
#endif

/*!\brief Decorator indicating that given struct/union/enum is packed */
#ifndef ATTRIBUTE_PACKED
#if defined(__GNUC__)
#define ATTRIBUTE_PACKED __attribute__((packed))
#elif defined(_MSC_VER)
#define ATTRIBUTE_PACKED
#else
#define ATTRIBUTE_PACKED
#endif
#endif /* ATTRIBUTE_PACKED */

/*!\brief Current ABI version number
 *
 * \internal
 * If this file is altered in any way that changes the ABI, this value
 * must be bumped.  Examples include, but are not limited to, changing
 * types, removing or reassigning enums, adding/removing/rearranging
 * fields to structures
 */
#define AOM_CODEC_ABI_VERSION (7 + AOM_IMAGE_ABI_VERSION) /**<\hideinitializer*/

/*!\brief Algorithm return codes */
typedef enum {
  /*!\brief Operation completed without error */
  AOM_CODEC_OK,

  /*!\brief Unspecified error */
  AOM_CODEC_ERROR,

  /*!\brief Memory operation failed */
  AOM_CODEC_MEM_ERROR,

  /*!\brief ABI version mismatch */
  AOM_CODEC_ABI_MISMATCH,

  /*!\brief Algorithm does not have required capability */
  AOM_CODEC_INCAPABLE,

  /*!\brief The given bitstream is not supported.
   *
   * The bitstream was unable to be parsed at the highest level. The decoder
   * is unable to proceed. This error \ref SHOULD be treated as fatal to the
   * stream. */
  AOM_CODEC_UNSUP_BITSTREAM,

  /*!\brief Encoded bitstream uses an unsupported feature
   *
   * The decoder does not implement a feature required by the encoder. This
   * return code should only be used for features that prevent future
   * pictures from being properly decoded. This error \ref MAY be treated as
   * fatal to the stream or \ref MAY be treated as fatal to the current GOP.
   */
  AOM_CODEC_UNSUP_FEATURE,

  /*!\brief The coded data for this stream is corrupt or incomplete
   *
   * There was a problem decoding the current frame.  This return code
   * should only be used for failures that prevent future pictures from
   * being properly decoded. This error \ref MAY be treated as fatal to the
   * stream or \ref MAY be treated as fatal to the current GOP. If decoding
   * is continued for the current GOP, artifacts may be present.
   */
  AOM_CODEC_CORRUPT_FRAME,

  /*!\brief An application-supplied parameter is not valid.
   *
   */
  AOM_CODEC_INVALID_PARAM,

  /*!\brief An iterator reached the end of list.
   *
   */
  AOM_CODEC_LIST_END

} aom_codec_err_t;

/*! \brief Codec capabilities bitfield
 *
 *  Each codec advertises the capabilities it supports as part of its
 *  ::aom_codec_iface_t interface structure. Capabilities are extra interfaces
 *  or functionality, and are not required to be supported.
 *
 *  The available flags are specified by AOM_CODEC_CAP_* defines.
 */
typedef long aom_codec_caps_t;
#define AOM_CODEC_CAP_DECODER 0x1 /**< Is a decoder */
#define AOM_CODEC_CAP_ENCODER 0x2 /**< Is an encoder */

/*! \brief Initialization-time Feature Enabling
 *
 *  Certain codec features must be known at initialization time, to allow for
 *  proper memory allocation.
 *
 *  The available flags are specified by AOM_CODEC_USE_* defines. The bits are
 *  allocated as follows:
 *      0x1 -     0x80: codec (common to decoder and encoder)
 *    0x100 -   0x8000: decoder
 *  0x10000 - 0x800000: encoder
 */
typedef long aom_codec_flags_t;

// Experimental feature policy
//
// New features may be marked as experimental. Experimental features are not
// part of the stable API and may be modified or removed in a future release.
// Experimental features are made available only if you pass the
// AOM_CODEC_USE_EXPERIMENTAL flag to the codec init function.
//
// If you use experimental features, you must rebuild your code whenever you
// update to a new libaom release, and you must be prepared to modify your code
// when an experimental feature you use is modified or removed. If you are not
// sure, DO NOT use experimental features.
#define AOM_CODEC_USE_EXPERIMENTAL 0x1 /**< Enables experimental features */

/*!\brief Time Stamp Type
 *
 * An integer, which when multiplied by the stream's time base, provides
 * the absolute time of a sample.
 */
typedef int64_t aom_codec_pts_t;

/*!\brief Codec interface structure.
 *
 * Contains function pointers and other data private to the codec
 * implementation. This structure is opaque to the application. Common
 * functions used with this structure:
 *   - aom_codec_iface_name(aom_codec_iface_t *iface): get the
 *     name of the codec
 *   - aom_codec_get_caps(aom_codec_iface_t *iface): returns
 *     the capabilities of the codec
 *   - aom_codec_enc_config_default: generate the default config for
 *     initializing the encoder (see documentation in aom_encoder.h)
 *   - aom_codec_dec_init, aom_codec_enc_init: initialize the codec context
 *     structure (see documentation on aom_codec_ctx).
 *
 * To get access to the AV1 encoder and decoder, use aom_codec_av1_cx() and
 *  aom_codec_av1_dx().
 */
typedef const struct aom_codec_iface aom_codec_iface_t;

/*!\brief Codec private data structure.
 *
 * Contains data private to the codec implementation. This structure is opaque
 * to the application.
 */
typedef struct aom_codec_priv aom_codec_priv_t;

/*!\brief Compressed Frame Flags
 *
 * This type represents a bitfield containing information about a compressed
 * frame that may be useful to an application. The most significant 16 bits
 * can be used by an algorithm to provide additional detail, for example to
 * support frame types that are codec specific (MPEG-1 D-frames for example)
 */
typedef uint32_t aom_codec_frame_flags_t;
#define AOM_FRAME_IS_KEY 0x1u /**< frame is the start of a GOP */
/*!\brief frame can be dropped without affecting the stream (no future frame
 * depends on this one) */
#define AOM_FRAME_IS_DROPPABLE 0x2u
/*!\brief this is an INTRA_ONLY frame */
#define AOM_FRAME_IS_INTRAONLY 0x10u
/*!\brief this is an S-frame */
#define AOM_FRAME_IS_SWITCH 0x20u
/*!\brief this is an error-resilient frame */
#define AOM_FRAME_IS_ERROR_RESILIENT 0x40u
/*!\brief this is a key-frame dependent recovery-point frame */
#define AOM_FRAME_IS_DELAYED_RANDOM_ACCESS_POINT 0x80u

/*!\brief Iterator
 *
 * Opaque storage used for iterating over lists.
 */
typedef const void *aom_codec_iter_t;

/*!\brief Codec context structure
 *
 * All codecs \ref MUST support this context structure fully. In general,
 * this data should be considered private to the codec algorithm, and
 * not be manipulated or examined by the calling application. Applications
 * may reference the 'name' member to get a printable description of the
 * algorithm.
 */
typedef struct aom_codec_ctx {
  const char *name;             /**< Printable interface name */
  aom_codec_iface_t *iface;     /**< Interface pointers */
  aom_codec_err_t err;          /**< Last returned error */
  const char *err_detail;       /**< Detailed info, if available */
  aom_codec_flags_t init_flags; /**< Flags passed at init time */
  union {
    /**< Decoder Configuration Pointer */
    const struct aom_codec_dec_cfg *dec;
    /**< Encoder Configuration Pointer */
    const struct aom_codec_enc_cfg *enc;
    const void *raw;
  } config;               /**< Configuration pointer aliasing union */
  aom_codec_priv_t *priv; /**< Algorithm private storage */
} aom_codec_ctx_t;

/*!\brief Bit depth for codec
 * *
 * This enumeration determines the bit depth of the codec.
 */
typedef enum aom_bit_depth {
  AOM_BITS_8 = 8,   /**<  8 bits */
  AOM_BITS_10 = 10, /**< 10 bits */
  AOM_BITS_12 = 12, /**< 12 bits */
} aom_bit_depth_t;

/*!\brief Superblock size selection.
 *
 * Defines the superblock size used for encoding. The superblock size can
 * either be fixed at 64x64 or 128x128 pixels, or it can be dynamically
 * selected by the encoder for each frame.
 */
typedef enum aom_superblock_size {
  AOM_SUPERBLOCK_SIZE_64X64,   /**< Always use 64x64 superblocks. */
  AOM_SUPERBLOCK_SIZE_128X128, /**< Always use 128x128 superblocks. */
  AOM_SUPERBLOCK_SIZE_DYNAMIC  /**< Select superblock size dynamically. */
} aom_superblock_size_t;

/*
 * Library Version Number Interface
 *
 * For example, see the following sample return values:
 *     aom_codec_version()           (1<<16 | 2<<8 | 3)
 *     aom_codec_version_str()       "v1.2.3-rc1-16-gec6a1ba"
 *     aom_codec_version_extra_str() "rc1-16-gec6a1ba"
 */

/*!\brief Return the version information (as an integer)
 *
 * Returns a packed encoding of the library version number. This will only
 * include the major.minor.patch component of the version number. Note that this
 * encoded value should be accessed through the macros provided, as the encoding
 * may change in the future.
 *
 */
int aom_codec_version(void);

/*!\brief Return the major version number */
#define aom_codec_version_major() ((aom_codec_version() >> 16) & 0xff)

/*!\brief Return the minor version number */
#define aom_codec_version_minor() ((aom_codec_version() >> 8) & 0xff)

/*!\brief Return the patch version number */
#define aom_codec_version_patch() ((aom_codec_version() >> 0) & 0xff)

/*!\brief Return the version information (as a string)
 *
 * Returns a printable string containing the full library version number. This
 * may contain additional text following the three digit version number, as to
 * indicate release candidates, pre-release versions, etc.
 *
 */
const char *aom_codec_version_str(void);

/*!\brief Return the version information (as a string)
 *
 * Returns a printable "extra string". This is the component of the string
 * returned by aom_codec_version_str() following the three digit version number.
 *
 */
const char *aom_codec_version_extra_str(void);

/*!\brief Return the build configuration
 *
 * Returns a printable string containing an encoded version of the build
 * configuration. This may be useful to aom support.
 *
 */
const char *aom_codec_build_config(void);

/*!\brief Return the name for a given interface
 *
 * Returns a human readable string for name of the given codec interface.
 *
 * \param[in]    iface     Interface pointer
 *
 */
const char *aom_codec_iface_name(aom_codec_iface_t *iface);

/*!\brief Convert error number to printable string
 *
 * Returns a human readable string for the last error returned by the
 * algorithm. The returned error will be one line and will not contain
 * any newline characters.
 *
 *
 * \param[in]    err     Error number.
 *
 */
const char *aom_codec_err_to_string(aom_codec_err_t err);

/*!\brief Retrieve error synopsis for codec context
 *
 * Returns a human readable string for the last error returned by the
 * algorithm. The returned error will be one line and will not contain
 * any newline characters.
 *
 *
 * \param[in]    ctx     Pointer to this instance's context.
 *
 */
const char *aom_codec_error(const aom_codec_ctx_t *ctx);

/*!\brief Retrieve detailed error information for codec context
 *
 * Returns a human readable string providing detailed information about
 * the last error. The returned string is only valid until the next
 * aom_codec_* function call (except aom_codec_error and
 * aom_codec_error_detail) on the codec context.
 *
 * \param[in]    ctx     Pointer to this instance's context.
 *
 * \retval NULL
 *     No detailed information is available.
 */
const char *aom_codec_error_detail(const aom_codec_ctx_t *ctx);

/* REQUIRED FUNCTIONS
 *
 * The following functions are required to be implemented for all codecs.
 * They represent the base case functionality expected of all codecs.
 */

/*!\brief Destroy a codec instance
 *
 * Destroys a codec context, freeing any associated memory buffers.
 *
 * \param[in] ctx   Pointer to this instance's context
 *
 * \retval #AOM_CODEC_OK
 *     The codec instance has been destroyed.
 * \retval #AOM_CODEC_INVALID_PARAM
 *     ctx is a null pointer.
 * \retval #AOM_CODEC_ERROR
 *     Codec context not initialized.
 */
aom_codec_err_t aom_codec_destroy(aom_codec_ctx_t *ctx);

/*!\brief Get the capabilities of an algorithm.
 *
 * Retrieves the capabilities bitfield from the algorithm's interface.
 *
 * \param[in] iface   Pointer to the algorithm interface
 *
 */
aom_codec_caps_t aom_codec_get_caps(aom_codec_iface_t *iface);

/*!\name Codec Control
 *
 * The aom_codec_control function exchanges algorithm specific data with the
 * codec instance. Additionally, the macro AOM_CODEC_CONTROL_TYPECHECKED is
 * provided, which will type-check the parameter against the control ID before
 * calling aom_codec_control - note that this macro requires the control ID
 * to be directly encoded in it, e.g.,
 * AOM_CODEC_CONTROL_TYPECHECKED(&ctx, AOME_SET_CPUUSED, 8).
 *
 * The codec control IDs can be found in aom.h, aomcx.h, and aomdx.h
 * (defined as aom_com_control_id, aome_enc_control_id, and aom_dec_control_id).
 * @{
 */
/*!\brief Algorithm Control
 *
 * aom_codec_control takes a context, a control ID, and a third parameter
 * (with varying type). If the context is non-null and an error occurs,
 * ctx->err will be set to the same value as the return value.
 *
 * \param[in]     ctx              Pointer to this instance's context
 * \param[in]     ctrl_id          Algorithm specific control identifier.
 *                                 Must be nonzero.
 *
 * \retval #AOM_CODEC_OK
 *     The control request was processed.
 * \retval #AOM_CODEC_ERROR
 *     The control request was not processed.
 * \retval #AOM_CODEC_INVALID_PARAM
 *     The control ID was zero, or the data was not valid.
 */
aom_codec_err_t aom_codec_control(aom_codec_ctx_t *ctx, int ctrl_id, ...);

/*!\brief Key & Value API
 *
 * aom_codec_set_option() takes a context, a key (option name) and a value. If
 * the context is non-null and an error occurs, ctx->err will be set to the same
 * value as the return value.
 *
 * \param[in]     ctx              Pointer to this instance's context
 * \param[in]     name             The name of the option (key)
 * \param[in]     value            The value of the option
 *
 * \retval #AOM_CODEC_OK
 *     The value of the option was set.
 * \retval #AOM_CODEC_INVALID_PARAM
 *     The data was not valid.
 * \retval #AOM_CODEC_ERROR
 *     The option was not successfully set.
 */
aom_codec_err_t aom_codec_set_option(aom_codec_ctx_t *ctx, const char *name,
                                     const char *value);

/*!\brief aom_codec_control wrapper macro (adds type-checking, less flexible)
 *
 * This macro allows for type safe conversions across the variadic parameter
 * to aom_codec_control(). However, it requires the explicit control ID
 * be passed in (it cannot be passed in via a variable) -- otherwise a compiler
 * error will occur. After the type checking, it calls aom_codec_control.
 */
#define AOM_CODEC_CONTROL_TYPECHECKED(ctx, id, data) \
  aom_codec_control_typechecked_##id(ctx, id, data) /**<\hideinitializer*/

/*!\brief Creates type checking mechanisms for aom_codec_control
 *
 * It defines a static function with the correctly typed arguments as a wrapper
 * to the type-unsafe aom_codec_control function. It also creates a typedef
 * for each type.
 */
#define AOM_CTRL_USE_TYPE(id, typ)                           \
  static aom_codec_err_t aom_codec_control_typechecked_##id( \
      aom_codec_ctx_t *, int, typ) AOM_UNUSED;               \
  static aom_codec_err_t aom_codec_control_typechecked_##id( \
      aom_codec_ctx_t *ctx, int ctrl, typ data) {            \
    return aom_codec_control(ctx, ctrl, data);               \
  } /**<\hideinitializer*/                                   \
  typedef typ aom_codec_control_type_##id;
/*!@} end Codec Control group */

/*!\brief OBU types. */
typedef enum ATTRIBUTE_PACKED {
  OBU_SEQUENCE_HEADER = 1,
  OBU_TEMPORAL_DELIMITER = 2,
  OBU_FRAME_HEADER = 3,
  OBU_TILE_GROUP = 4,
  OBU_METADATA = 5,
  OBU_FRAME = 6,
  OBU_REDUNDANT_FRAME_HEADER = 7,
  OBU_TILE_LIST = 8,
  OBU_PADDING = 15,
} OBU_TYPE;

/*!\brief OBU metadata types. */
typedef enum {
  OBU_METADATA_TYPE_AOM_RESERVED_0 = 0,
  OBU_METADATA_TYPE_HDR_CLL = 1,
  OBU_METADATA_TYPE_HDR_MDCV = 2,
  OBU_METADATA_TYPE_SCALABILITY = 3,
  OBU_METADATA_TYPE_ITUT_T35 = 4,
  OBU_METADATA_TYPE_TIMECODE = 5,
} OBU_METADATA_TYPE;

/*!\brief Returns string representation of OBU_TYPE.
 *
 * \param[in]     type            The OBU_TYPE to convert to string.
 */
const char *aom_obu_type_to_string(OBU_TYPE type);

/*!@} - end defgroup codec*/
#ifdef __cplusplus
}
#endif
#endif  // AOM_AOM_AOM_CODEC_H_
