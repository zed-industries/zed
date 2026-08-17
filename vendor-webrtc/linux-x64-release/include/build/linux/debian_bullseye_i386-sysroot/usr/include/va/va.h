/*
 * Copyright (c) 2007-2009 Intel Corporation. All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sub license, and/or sell copies of the Software, and to
 * permit persons to whom the Software is furnished to do so, subject to
 * the following conditions:
 *
 * The above copyright notice and this permission notice (including the
 * next paragraph) shall be included in all copies or substantial portions
 * of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT.
 * IN NO EVENT SHALL INTEL AND/OR ITS SUPPLIERS BE LIABLE FOR
 * ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
 * TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
 * SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */
/*
 * Video Acceleration (VA) API Specification
 *
 * Rev. 0.30
 * <jonathan.bian@intel.com>
 *
 * Revision History:
 * rev 0.10 (12/10/2006 Jonathan Bian) - Initial draft
 * rev 0.11 (12/15/2006 Jonathan Bian) - Fixed some errors
 * rev 0.12 (02/05/2007 Jonathan Bian) - Added VC-1 data structures for slice level decode
 * rev 0.13 (02/28/2007 Jonathan Bian) - Added GetDisplay()
 * rev 0.14 (04/13/2007 Jonathan Bian) - Fixed MPEG-2 PictureParameter structure, cleaned up a few funcs.
 * rev 0.15 (04/20/2007 Jonathan Bian) - Overhauled buffer management
 * rev 0.16 (05/02/2007 Jonathan Bian) - Added error codes and fixed some issues with configuration
 * rev 0.17 (05/07/2007 Jonathan Bian) - Added H.264/AVC data structures for slice level decode.
 * rev 0.18 (05/14/2007 Jonathan Bian) - Added data structures for MPEG-4 slice level decode
 *                                       and MPEG-2 motion compensation.
 * rev 0.19 (08/06/2007 Jonathan Bian) - Removed extra type for bitplane data.
 * rev 0.20 (08/08/2007 Jonathan Bian) - Added missing fields to VC-1 PictureParameter structure.
 * rev 0.21 (08/20/2007 Jonathan Bian) - Added image and subpicture support.
 * rev 0.22 (08/27/2007 Jonathan Bian) - Added support for chroma-keying and global alpha.
 * rev 0.23 (09/11/2007 Jonathan Bian) - Fixed some issues with images and subpictures.
 * rev 0.24 (09/18/2007 Jonathan Bian) - Added display attributes.
 * rev 0.25 (10/18/2007 Jonathan Bian) - Changed to use IDs only for some types.
 * rev 0.26 (11/07/2007 Waldo Bastian) - Change vaCreateBuffer semantics
 * rev 0.27 (11/19/2007 Matt Sottek)   - Added DeriveImage
 * rev 0.28 (12/06/2007 Jonathan Bian) - Added new versions of PutImage and AssociateSubpicture
 *                                       to enable scaling
 * rev 0.29 (02/07/2008 Jonathan Bian) - VC1 parameter fixes,
 *                                       added VA_STATUS_ERROR_RESOLUTION_NOT_SUPPORTED
 * rev 0.30 (03/01/2009 Jonathan Bian) - Added encoding support for H.264 BP and MPEG-4 SP and fixes
 *                                       for ISO C conformance.
 * rev 0.31 (09/02/2009 Gwenole Beauchesne) - VC-1/H264 fields change for VDPAU and XvBA backend
 *                                       Application needs to relink with the new library.
 *
 * rev 0.31.1 (03/29/2009)              - Data structure for JPEG encode
 * rev 0.31.2 (01/13/2011 Anthony Pabon)- Added a flag to indicate Subpicture coordinates are screen
 *                                        screen relative rather than source video relative.
 * rev 0.32.0 (01/13/2011 Xiang Haihao) - Add profile into VAPictureParameterBufferVC1
 *                                        update VAAPI to 0.32.0
 *
 * Acknowledgements:
 *  Some concepts borrowed from XvMC and XvImage.
 *  Waldo Bastian (Intel), Matt Sottek (Intel),  Austin Yuan (Intel), and Gwenole Beauchesne (SDS)
 *  contributed to various aspects of the API.
 */

/**
 * \file va.h
 * \brief The Core API
 *
 * This file contains the \ref api_core "Core API".
 */

#ifndef _VA_H_
#define _VA_H_

#include <stddef.h>
#include <stdint.h>
#include <va/va_version.h>

#ifdef __cplusplus
extern "C" {
#endif

#if defined(__GNUC__) && !defined(__COVERITY__)
#define va_deprecated __attribute__((deprecated))
#if __GNUC__ >= 6
#define va_deprecated_enum va_deprecated
#else
#define va_deprecated_enum
#endif
#else
#define va_deprecated
#define va_deprecated_enum
#endif

/**
 * \mainpage Video Acceleration (VA) API
 *
 * \section intro Introduction
 *
 * The main motivation for VA-API (Video Acceleration API) is to
 * enable hardware accelerated video decode and encode at various
 * entry-points (VLD, IDCT, Motion Compensation etc.) for the
 * prevailing coding standards today (MPEG-2, MPEG-4 ASP/H.263, MPEG-4
 * AVC/H.264, VC-1/VMW3, and JPEG, HEVC/H265, VP8, VP9) and video pre/post
 * processing
 *
 * VA-API is split into several modules:
 * - \ref api_core
 * - Encoder (H264, HEVC, JPEG, MPEG2, VP8, VP9)
 *  - \ref api_enc_h264
 *  - \ref api_enc_hevc
 *  - \ref api_enc_jpeg
 *  - \ref api_enc_mpeg2
 *  - \ref api_enc_vp8
 *  - \ref api_enc_vp9
 *  - \ref api_enc_av1
 * - Decoder (HEVC, JPEG, VP8, VP9, AV1)
 *      - \ref api_dec_hevc
 *      - \ref api_dec_jpeg
 *      - \ref api_dec_vp8
 *      - \ref api_dec_vp9
 *      - \ref api_dec_av1
 * - \ref api_vpp
 * - \ref api_prot
 * - FEI (H264, HEVC)
 *  - \ref api_fei
 *  - \ref api_fei_h264
 *  - \ref api_fei_hevc
 *
 * \section threading Multithreading Guide
 * All VAAPI functions implemented in libva are thread-safe. For any VAAPI
 * function that requires the implementation of a backend (e.g. hardware driver),
 * the backend must ensure that its implementation is also thread-safe. If the
 * backend implementation of a VAAPI function is not thread-safe then this should
 * be considered as a bug against the backend implementation.
 *
 * It is assumed that none of the VAAPI functions will be called from signal
 * handlers.
 *
 * Thread-safety in this context means that when VAAPI is being called by multiple
 * concurrent threads, it will not crash or hang the OS, and VAAPI internal
 * data structures will not be corrupted. When multiple threads are operating on
 * the same VAAPI objects, it is the application's responsibility to synchronize
 * these operations in order to generate the expected results. For example, using
 * a single VAContext from multiple threads may generate unexpected results.
 *
 * Following pseudo code illustrates a multithreaded transcoding scenario, where
 * one thread is handling the decoding operation and another thread is handling
 * the encoding operation, while synchronizing the use of a common pool of
 * surfaces.
 *
 * \code
 * // Initialization
 * dpy = vaGetDisplayDRM(fd);
 * vaInitialize(dpy, ...);
 *
 * // Create surfaces required for decoding and subsequence encoding
 * vaCreateSurfaces(dpy, VA_RT_FORMAT_YUV420, width, height, &surfaces[0], ...);
 *
 * // Set up a queue for the surfaces shared between decode and encode threads
 * surface_queue = queue_create();
 *
 * // Create decode_thread
 * pthread_create(&decode_thread, NULL, decode, ...);
 *
 * // Create encode_thread
 * pthread_create(&encode_thread, NULL, encode, ...);
 *
 * // Decode thread function
 * decode() {
 *   // Find the decode entrypoint for H.264
 *   vaQueryConfigEntrypoints(dpy, h264_profile, entrypoints, ...);
 *
 *   // Create a config for H.264 decode
 *   vaCreateConfig(dpy, h264_profile, VAEntrypointVLD, ...);
 *
 *   // Create a context for decode
 *   vaCreateContext(dpy, config, width, height, VA_PROGRESSIVE, surfaces,
 *     num_surfaces, &decode_context);
 *
 *   // Decode frames in the bitstream
 *   for (;;) {
 *     // Parse one frame and decode
 *     vaBeginPicture(dpy, decode_context, surfaces[surface_index]);
 *     vaRenderPicture(dpy, decode_context, buf, ...);
 *     vaEndPicture(dpy, decode_context);
 *     // Poll the decoding status and enqueue the surface in display order after
 *     // decoding is complete
 *     vaQuerySurfaceStatus();
 *     enqueue(surface_queue, surface_index);
 *   }
 * }
 *
 * // Encode thread function
 * encode() {
 *   // Find the encode entrypoint for HEVC
 *   vaQueryConfigEntrypoints(dpy, hevc_profile, entrypoints, ...);
 *
 *   // Create a config for HEVC encode
 *   vaCreateConfig(dpy, hevc_profile, VAEntrypointEncSlice, ...);
 *
 *   // Create a context for encode
 *   vaCreateContext(dpy, config, width, height, VA_PROGRESSIVE, surfaces,
 *     num_surfaces, &encode_context);
 *
 *   // Encode frames produced by the decoder
 *   for (;;) {
 *     // Dequeue the surface enqueued by the decoder
 *     surface_index = dequeue(surface_queue);
 *     // Encode using this surface as the source
 *     vaBeginPicture(dpy, encode_context, surfaces[surface_index]);
 *     vaRenderPicture(dpy, encode_context, buf, ...);
 *     vaEndPicture(dpy, encode_context);
 *   }
 * }
 * \endcode
 */

/**
 * \defgroup api_core Core API
 *
 * @{
 */

/**
Overview

The VA API is intended to provide an interface between a video decode/encode/processing
application (client) and a hardware accelerator (server), to off-load
video decode/encode/processing operations from the host to the hardware accelerator at various
entry-points.

The basic operation steps are:

- Negotiate a mutually acceptable configuration with the server to lock
  down profile, entrypoints, and other attributes that will not change on
  a frame-by-frame basis.
- Create a video decode, encode or processing context which represents a
  "virtualized" hardware device
- Get and fill the render buffers with the corresponding data (depending on
  profiles and entrypoints)
- Pass the render buffers to the server to handle the current frame

Initialization & Configuration Management

- Find out supported profiles
- Find out entrypoints for a given profile
- Find out configuration attributes for a given profile/entrypoint pair
- Create a configuration for use by the application

*/

typedef void* VADisplay;    /* window system dependent */

typedef int VAStatus;   /** Return status type from functions */
/** Values for the return status */
#define VA_STATUS_SUCCESS           0x00000000
#define VA_STATUS_ERROR_OPERATION_FAILED    0x00000001
#define VA_STATUS_ERROR_ALLOCATION_FAILED   0x00000002
#define VA_STATUS_ERROR_INVALID_DISPLAY     0x00000003
#define VA_STATUS_ERROR_INVALID_CONFIG      0x00000004
#define VA_STATUS_ERROR_INVALID_CONTEXT     0x00000005
#define VA_STATUS_ERROR_INVALID_SURFACE     0x00000006
#define VA_STATUS_ERROR_INVALID_BUFFER      0x00000007
#define VA_STATUS_ERROR_INVALID_IMAGE       0x00000008
#define VA_STATUS_ERROR_INVALID_SUBPICTURE  0x00000009
#define VA_STATUS_ERROR_ATTR_NOT_SUPPORTED  0x0000000a
#define VA_STATUS_ERROR_MAX_NUM_EXCEEDED    0x0000000b
#define VA_STATUS_ERROR_UNSUPPORTED_PROFILE 0x0000000c
#define VA_STATUS_ERROR_UNSUPPORTED_ENTRYPOINT  0x0000000d
#define VA_STATUS_ERROR_UNSUPPORTED_RT_FORMAT   0x0000000e
#define VA_STATUS_ERROR_UNSUPPORTED_BUFFERTYPE  0x0000000f
#define VA_STATUS_ERROR_SURFACE_BUSY        0x00000010
#define VA_STATUS_ERROR_FLAG_NOT_SUPPORTED      0x00000011
#define VA_STATUS_ERROR_INVALID_PARAMETER   0x00000012
#define VA_STATUS_ERROR_RESOLUTION_NOT_SUPPORTED 0x00000013
#define VA_STATUS_ERROR_UNIMPLEMENTED           0x00000014
#define VA_STATUS_ERROR_SURFACE_IN_DISPLAYING   0x00000015
#define VA_STATUS_ERROR_INVALID_IMAGE_FORMAT    0x00000016
#define VA_STATUS_ERROR_DECODING_ERROR          0x00000017
#define VA_STATUS_ERROR_ENCODING_ERROR          0x00000018
/**
 * \brief An invalid/unsupported value was supplied.
 *
 * This is a catch-all error code for invalid or unsupported values.
 * e.g. value exceeding the valid range, invalid type in the context
 * of generic attribute values.
 */
#define VA_STATUS_ERROR_INVALID_VALUE           0x00000019
/** \brief An unsupported filter was supplied. */
#define VA_STATUS_ERROR_UNSUPPORTED_FILTER      0x00000020
/** \brief An invalid filter chain was supplied. */
#define VA_STATUS_ERROR_INVALID_FILTER_CHAIN    0x00000021
/** \brief Indicate HW busy (e.g. run multiple encoding simultaneously). */
#define VA_STATUS_ERROR_HW_BUSY                 0x00000022
/** \brief An unsupported memory type was supplied. */
#define VA_STATUS_ERROR_UNSUPPORTED_MEMORY_TYPE 0x00000024
/** \brief Indicate allocated buffer size is not enough for input or output. */
#define VA_STATUS_ERROR_NOT_ENOUGH_BUFFER       0x00000025
/** \brief Indicate an operation isn't completed because time-out interval elapsed. */
#define VA_STATUS_ERROR_TIMEDOUT                0x00000026
#define VA_STATUS_ERROR_UNKNOWN         0xFFFFFFFF

/**
 * 1. De-interlacing flags for vaPutSurface()
 * 2. Surface sample type for input/output surface flag
 *    - Progressive: VA_FRAME_PICTURE
 *    - Interleaved: VA_TOP_FIELD_FIRST, VA_BOTTOM_FIELD_FIRST
 *    - Field: VA_TOP_FIELD, VA_BOTTOM_FIELD
*/
#define VA_FRAME_PICTURE        0x00000000
#define VA_TOP_FIELD            0x00000001
#define VA_BOTTOM_FIELD         0x00000002
#define VA_TOP_FIELD_FIRST      0x00000004
#define VA_BOTTOM_FIELD_FIRST   0x00000008

/**
 * Enabled the positioning/cropping/blending feature:
 * 1, specify the video playback position in the isurface
 * 2, specify the cropping info for video playback
 * 3, encoded video will blend with background color
 */
#define VA_ENABLE_BLEND         0x00000004 /* video area blend with the constant color */

/**
 * Clears the drawable with background color.
 * for hardware overlay based implementation this flag
 * can be used to turn off the overlay
 */
#define VA_CLEAR_DRAWABLE       0x00000008

/** Color space conversion flags for vaPutSurface() */
#define VA_SRC_COLOR_MASK       0x000000f0
#define VA_SRC_BT601            0x00000010
#define VA_SRC_BT709            0x00000020
#define VA_SRC_SMPTE_240        0x00000040

/** Scaling flags for vaPutSurface() */
#define VA_FILTER_SCALING_DEFAULT       0x00000000
#define VA_FILTER_SCALING_FAST          0x00000100
#define VA_FILTER_SCALING_HQ            0x00000200
#define VA_FILTER_SCALING_NL_ANAMORPHIC 0x00000300
#define VA_FILTER_SCALING_MASK          0x00000f00

/** Interpolation method for scaling */
#define VA_FILTER_INTERPOLATION_DEFAULT                    0x00000000
#define VA_FILTER_INTERPOLATION_NEAREST_NEIGHBOR           0x00001000
#define VA_FILTER_INTERPOLATION_BILINEAR                   0x00002000
#define VA_FILTER_INTERPOLATION_ADVANCED                   0x00003000
#define VA_FILTER_INTERPOLATION_MASK                       0x0000f000

/** Padding size in 4-bytes */
#define VA_PADDING_LOW          4
#define VA_PADDING_MEDIUM       8
#define VA_PADDING_HIGH         16
#define VA_PADDING_LARGE        32

/** operation options */
/** synchronization, block call, output should be ready after execution function return*/
#define VA_EXEC_SYNC              0x0
/** asynchronization,application should call additonal sync operation to access output */
#define VA_EXEC_ASYNC             0x1

/** operation mode */
#define VA_EXEC_MODE_DEFAULT      0x0
#define VA_EXEC_MODE_POWER_SAVING 0x1
#define VA_EXEC_MODE_PERFORMANCE  0x2

/* Values used to describe device features. */
/** The feature is not supported by the device.
 *
 * Any corresponding feature flag must not be set.
 */
#define VA_FEATURE_NOT_SUPPORTED  0
/** The feature is supported by the device.
 *
 * The user may decide whether or not to use this feature.
 *
 * Note that support for a feature only indicates that the hardware
 * is able to use it; whether it is actually a positive change to
 * enable it in a given situation will depend on other factors
 * including the input provided by the user.
 */
#define VA_FEATURE_SUPPORTED      1
/** The feature is required by the device.
 *
 * The device does not support not enabling this feature, so any
 * corresponding feature flag must be set and any additional
 * configuration needed by the feature must be supplied.
 */
#define VA_FEATURE_REQUIRED       2

/**
 * Returns a short english description of error_status
 */
const char *vaErrorStr(VAStatus error_status);

typedef struct _VARectangle {
    int16_t x;
    int16_t y;
    uint16_t width;
    uint16_t height;
} VARectangle;

/** \brief Generic motion vector data structure. */
typedef struct _VAMotionVector {
    /** \mv0[0]: horizontal motion vector for past reference */
    /** \mv0[1]: vertical motion vector for past reference */
    /** \mv1[0]: horizontal motion vector for future reference */
    /** \mv1[1]: vertical motion vector for future reference */
    int16_t  mv0[2];  /* past reference */
    int16_t  mv1[2];  /* future reference */
} VAMotionVector;

/** Type of a message callback, used for both error and info log. */
typedef void (*VAMessageCallback)(void *user_context, const char *message);

/**
 * Set the callback for error messages, or NULL for no logging.
 * Returns the previous one, or NULL if it was disabled.
 */
VAMessageCallback vaSetErrorCallback(VADisplay dpy, VAMessageCallback callback, void *user_context);

/**
 * Set the callback for info messages, or NULL for no logging.
 * Returns the previous one, or NULL if it was disabled.
 */
VAMessageCallback vaSetInfoCallback(VADisplay dpy, VAMessageCallback callback, void *user_context);

/**
 * Initialization:
 * A display must be obtained by calling vaGetDisplay() before calling
 * vaInitialize() and other functions. This connects the API to the
 * native window system.
 * For X Windows, native_dpy would be from XOpenDisplay()
 */
typedef void* VANativeDisplay;  /* window system dependent */

int vaDisplayIsValid(VADisplay dpy);

/**
 *  Set the override driver name instead of queried driver driver.
 */
VAStatus vaSetDriverName(VADisplay dpy,
                         char *driver_name
                        );

/**
 * Initialize the library
 */
VAStatus vaInitialize(
    VADisplay dpy,
    int *major_version,  /* out */
    int *minor_version   /* out */
);

/**
 * After this call, all library internal resources will be cleaned up
 */
VAStatus vaTerminate(
    VADisplay dpy
);

/**
 * vaQueryVendorString returns a pointer to a zero-terminated string
 * describing some aspects of the VA implemenation on a specific
 * hardware accelerator. The format of the returned string is vendor
 * specific and at the discretion of the implementer.
 * e.g. for the Intel GMA500 implementation, an example would be:
 * "Intel GMA500 - 2.0.0.32L.0005"
 */
const char *vaQueryVendorString(
    VADisplay dpy
);

typedef int (*VAPrivFunc)(void);

/**
 * Return a function pointer given a function name in the library.
 * This allows private interfaces into the library
 */
VAPrivFunc vaGetLibFunc(
    VADisplay dpy,
    const char *func
);

/** Currently defined profiles */
typedef enum {
    /** \brief Profile ID used for video processing. */
    VAProfileNone                       = -1,
    VAProfileMPEG2Simple        = 0,
    VAProfileMPEG2Main          = 1,
    VAProfileMPEG4Simple        = 2,
    VAProfileMPEG4AdvancedSimple    = 3,
    VAProfileMPEG4Main          = 4,
    VAProfileH264Baseline va_deprecated_enum = 5,
    VAProfileH264Main           = 6,
    VAProfileH264High           = 7,
    VAProfileVC1Simple          = 8,
    VAProfileVC1Main            = 9,
    VAProfileVC1Advanced        = 10,
    VAProfileH263Baseline       = 11,
    VAProfileJPEGBaseline               = 12,
    VAProfileH264ConstrainedBaseline    = 13,
    VAProfileVP8Version0_3              = 14,
    VAProfileH264MultiviewHigh          = 15,
    VAProfileH264StereoHigh             = 16,
    VAProfileHEVCMain                   = 17,
    VAProfileHEVCMain10                 = 18,
    VAProfileVP9Profile0                = 19,
    VAProfileVP9Profile1                = 20,
    VAProfileVP9Profile2                = 21,
    VAProfileVP9Profile3                = 22,
    VAProfileHEVCMain12                 = 23,
    VAProfileHEVCMain422_10             = 24,
    VAProfileHEVCMain422_12             = 25,
    VAProfileHEVCMain444                = 26,
    VAProfileHEVCMain444_10             = 27,
    VAProfileHEVCMain444_12             = 28,
    VAProfileHEVCSccMain                = 29,
    VAProfileHEVCSccMain10              = 30,
    VAProfileHEVCSccMain444             = 31,
    VAProfileAV1Profile0                = 32,
    VAProfileAV1Profile1                = 33,
    VAProfileHEVCSccMain444_10          = 34,
    /** \brief Profile ID used for protected video playback. */
    VAProfileProtected                  = 35
} VAProfile;

/**
 *  Currently defined entrypoints
 */
typedef enum {
    VAEntrypointVLD     = 1,
    VAEntrypointIZZ     = 2,
    VAEntrypointIDCT        = 3,
    VAEntrypointMoComp      = 4,
    VAEntrypointDeblocking  = 5,
    VAEntrypointEncSlice    = 6,    /* slice level encode */
    VAEntrypointEncPicture  = 7,    /* pictuer encode, JPEG, etc */
    /*
     * For an implementation that supports a low power/high performance variant
     * for slice level encode, it can choose to expose the
     * VAEntrypointEncSliceLP entrypoint. Certain encoding tools may not be
     * available with this entrypoint (e.g. interlace, MBAFF) and the
     * application can query the encoding configuration attributes to find
     * out more details if this entrypoint is supported.
     */
    VAEntrypointEncSliceLP  = 8,
    VAEntrypointVideoProc       = 10,   /**< Video pre/post-processing. */
    /**
     * \brief VAEntrypointFEI
     *
     * The purpose of FEI (Flexible Encoding Infrastructure) is to allow applications to
     * have more controls and trade off quality for speed with their own IPs.
     * The application can optionally provide input to ENC for extra encode control
     * and get the output from ENC. Application can chose to modify the ENC
     * output/PAK input during encoding, but the performance impact is significant.
     *
     * On top of the existing buffers for normal encode, there will be
     * one extra input buffer (VAEncMiscParameterFEIFrameControl) and
     * three extra output buffers (VAEncFEIMVBufferType, VAEncFEIMBModeBufferType
     * and VAEncFEIDistortionBufferType) for VAEntrypointFEI entry function.
     * If separate PAK is set, two extra input buffers
     * (VAEncFEIMVBufferType, VAEncFEIMBModeBufferType) are needed for PAK input.
     **/
    VAEntrypointFEI         = 11,
    /**
     * \brief VAEntrypointStats
     *
     * A pre-processing function for getting some statistics and motion vectors is added,
     * and some extra controls for Encode pipeline are provided. The application can
     * optionally call the statistics function to get motion vectors and statistics like
     * variances, distortions before calling Encode function via this entry point.
     *
     * Checking whether Statistics is supported can be performed with vaQueryConfigEntrypoints().
     * If Statistics entry point is supported, then the list of returned entry-points will
     * include #VAEntrypointStats. Supported pixel format, maximum resolution and statistics
     * specific attributes can be obtained via normal attribute query. One input buffer
     * (VAStatsStatisticsParameterBufferType) and one or two output buffers
     * (VAStatsStatisticsBufferType, VAStatsStatisticsBottomFieldBufferType (for interlace only)
     * and VAStatsMVBufferType) are needed for this entry point.
     **/
    VAEntrypointStats       = 12,
    /**
     * \brief VAEntrypointProtectedTEEComm
     *
     * A function for communicating with TEE (Trusted Execution Environment).
     **/
    VAEntrypointProtectedTEEComm       = 13,
    /**
     * \brief VAEntrypointProtectedContent
     *
     * A function for protected content to decrypt encrypted content.
     **/
    VAEntrypointProtectedContent       = 14,
} VAEntrypoint;

/** Currently defined configuration attribute types */
typedef enum {
    VAConfigAttribRTFormat      = 0,
    VAConfigAttribSpatialResidual   = 1,
    VAConfigAttribSpatialClipping   = 2,
    VAConfigAttribIntraResidual     = 3,
    VAConfigAttribEncryption        = 4,
    VAConfigAttribRateControl       = 5,

    /** @name Attributes for decoding */
    /**@{*/
    /**
     * \brief Slice Decoding mode. Read/write.
     *
     * This attribute determines what mode the driver supports for slice
     * decoding, through vaGetConfigAttributes(); and what mode the user
     * will be providing to the driver, through vaCreateConfig(), if the
     * driver supports those. If this attribute is not set by the user then
     * it is assumed that VA_DEC_SLICE_MODE_NORMAL mode is used.
     *
     * See \c VA_DEC_SLICE_MODE_xxx for the list of slice decoding modes.
     */
    VAConfigAttribDecSliceMode      = 6,
    /**
      * \brief JPEG decoding attribute. Read-only.
      *
      * This attribute exposes a number of capabilities of the underlying
      * JPEG implementation. The attribute value is partitioned into fields as defined in the
      * VAConfigAttribValDecJPEG union.
      */
    VAConfigAttribDecJPEG             = 7,
    /**
     * \brief Decode processing support. Read/write.
     *
     * This attribute determines if the driver supports video processing
     * with decoding using the decoding context in a single call, through
     * vaGetConfigAttributes(); and if the user may use this feature,
     * through vaCreateConfig(), if the driver supports the user scenario.
     * The user will essentially create a regular decode VAContext.  Therefore,
     * the parameters of vaCreateContext() such as picture_width, picture_height
     * and render_targets are in relation to the decode output parameters
     * (not processing output parameters) as normal.
     * If this attribute is not set by the user then it is assumed that no
     * extra processing is done after decoding for this decode context.
     *
     * Since essentially the application is creating a decoder config and context,
     * all function calls that take in the config (e.g. vaQuerySurfaceAttributes())
     * or context are in relation to the decoder, except those video processing
     * function specified in the next paragraph.
     *
     * Once the decode config and context are created, the user must further
     * query the supported processing filters using vaQueryVideoProcFilters(),
     * vaQueryVideoProcFilterCaps(), vaQueryVideoProcPipelineCaps() by specifying
     * the created decode context.  The user must provide processing information
     * and extra processing output surfaces as "additional_outputs" to the driver
     * through VAProcPipelineParameterBufferType.  The render_target specified
     * at vaBeginPicture() time refers to the decode output surface.  The
     * target surface for the output of processing needs to be a different
     * surface since the decode process requires the original reconstructed buffer.
     * The “surface” member of VAProcPipelineParameterBuffer should be set to the
     * same as “render_target” set in vaBeginPicture(), but the driver may choose
     * to ignore this parameter.
     */
    VAConfigAttribDecProcessing     = 8,
    /** @name Attributes for encoding */
    /**@{*/
    /**
     * \brief Packed headers mode. Read/write.
     *
     * This attribute determines what packed headers the driver supports,
     * through vaGetConfigAttributes(); and what packed headers the user
     * will be providing to the driver, through vaCreateConfig(), if the
     * driver supports those.
     *
     * See \c VA_ENC_PACKED_HEADER_xxx for the list of packed headers.
     */
    VAConfigAttribEncPackedHeaders      = 10,
    /**
     * \brief Interlaced mode. Read/write.
     *
     * This attribute determines what kind of interlaced encoding mode
     * the driver supports.
     *
     * See \c VA_ENC_INTERLACED_xxx for the list of interlaced modes.
     */
    VAConfigAttribEncInterlaced         = 11,
    /**
     * \brief Maximum number of reference frames. Read-only.
     *
     * This attribute determines the maximum number of reference
     * frames supported for encoding.
     *
     * Note: for H.264 encoding, the value represents the maximum number
     * of reference frames for both the reference picture list 0 (bottom
     * 16 bits) and the reference picture list 1 (top 16 bits).
     */
    VAConfigAttribEncMaxRefFrames       = 13,
    /**
     * \brief Maximum number of slices per frame. Read-only.
     *
     * This attribute determines the maximum number of slices the
     * driver can support to encode a single frame.
     */
    VAConfigAttribEncMaxSlices          = 14,
    /**
     * \brief Slice structure. Read-only.
     *
     * This attribute determines slice structures supported by the
     * driver for encoding. This attribute is a hint to the user so
     * that he can choose a suitable surface size and how to arrange
     * the encoding process of multiple slices per frame.
     *
     * More specifically, for H.264 encoding, this attribute
     * determines the range of accepted values to
     * VAEncSliceParameterBufferH264::macroblock_address and
     * VAEncSliceParameterBufferH264::num_macroblocks.
     *
     * See \c VA_ENC_SLICE_STRUCTURE_xxx for the supported slice
     * structure types.
     */
    VAConfigAttribEncSliceStructure     = 15,
    /**
     * \brief Macroblock information. Read-only.
     *
     * This attribute determines whether the driver supports extra
     * encoding information per-macroblock. e.g. QP.
     *
     * More specifically, for H.264 encoding, if the driver returns a non-zero
     * value for this attribute, this means the application can create
     * additional #VAEncMacroblockParameterBufferH264 buffers referenced
     * through VAEncSliceParameterBufferH264::macroblock_info.
     */
    VAConfigAttribEncMacroblockInfo     = 16,
    /**
     * \brief Maximum picture width. Read-only.
     *
     * This attribute determines the maximum picture width the driver supports
     * for a given configuration.
     */
    VAConfigAttribMaxPictureWidth     = 18,
    /**
     * \brief Maximum picture height. Read-only.
     *
     * This attribute determines the maximum picture height the driver supports
     * for a given configuration.
     */
    VAConfigAttribMaxPictureHeight    = 19,
    /**
     * \brief JPEG encoding attribute. Read-only.
     *
     * This attribute exposes a number of capabilities of the underlying
     * JPEG implementation. The attribute value is partitioned into fields as defined in the
     * VAConfigAttribValEncJPEG union.
     */
    VAConfigAttribEncJPEG             = 20,
    /**
     * \brief Encoding quality range attribute. Read-only.
     *
     * This attribute conveys whether the driver supports different quality level settings
     * for encoding. A value less than or equal to 1 means that the encoder only has a single
     * quality setting, and a value greater than 1 represents the number of quality levels
     * that can be configured. e.g. a value of 2 means there are two distinct quality levels.
     */
    VAConfigAttribEncQualityRange     = 21,
    /**
     * \brief Encoding quantization attribute. Read-only.
     *
     * This attribute conveys whether the driver supports certain types of quantization methods
     * for encoding (e.g. trellis). See \c VA_ENC_QUANTIZATION_xxx for the list of quantization methods
     */
    VAConfigAttribEncQuantization     = 22,
    /**
     * \brief Encoding intra refresh attribute. Read-only.
     *
     * This attribute conveys whether the driver supports certain types of intra refresh methods
     * for encoding (e.g. adaptive intra refresh or rolling intra refresh).
     * See \c VA_ENC_INTRA_REFRESH_xxx for intra refresh methods
     */
    VAConfigAttribEncIntraRefresh     = 23,
    /**
     * \brief Encoding skip frame attribute. Read-only.
     *
     * This attribute conveys whether the driver supports sending skip frame parameters
     * (VAEncMiscParameterTypeSkipFrame) to the encoder's rate control, when the user has
     * externally skipped frames.
     */
    VAConfigAttribEncSkipFrame        = 24,
    /**
     * \brief Encoding region-of-interest (ROI) attribute. Read-only.
     *
     * This attribute conveys whether the driver supports region-of-interest (ROI) encoding,
     * based on user provided ROI rectangles.  The attribute value is partitioned into fields
     * as defined in the VAConfigAttribValEncROI union.
     *
     * If ROI encoding is supported, the ROI information is passed to the driver using
     * VAEncMiscParameterTypeROI.
     */
    VAConfigAttribEncROI              = 25,
    /**
     * \brief Encoding extended rate control attribute. Read-only.
     *
     * This attribute conveys whether the driver supports any extended rate control features
     * The attribute value is partitioned into fields as defined in the
     * VAConfigAttribValEncRateControlExt union.
     */
    VAConfigAttribEncRateControlExt   = 26,
    /**
     * \brief Processing rate reporting attribute. Read-only.
     *
     * This attribute conveys whether the driver supports reporting of
     * encode/decode processing rate based on certain set of parameters
     * (i.e. levels, I frame internvals) for a given configuration.
     * If this is supported, vaQueryProcessingRate() can be used to get
     * encode or decode processing rate.
     * See \c VA_PROCESSING_RATE_xxx for encode/decode processing rate
     */
    VAConfigAttribProcessingRate    = 27,
    /**
     * \brief Encoding dirty rectangle. Read-only.
     *
     * This attribute conveys whether the driver supports dirty rectangle.
     * encoding, based on user provided ROI rectangles which indicate the rectangular areas
     * where the content has changed as compared to the previous picture.  The regions of the
     * picture that are not covered by dirty rect rectangles are assumed to have not changed
     * compared to the previous picture.  The encoder may do some optimizations based on
     * this information.  The attribute value returned indicates the number of regions that
     * are supported.  e.g. A value of 0 means dirty rect encoding is not supported.  If dirty
     * rect encoding is supported, the ROI information is passed to the driver using
     * VAEncMiscParameterTypeDirtyRect.
     */
    VAConfigAttribEncDirtyRect       = 28,
    /**
     * \brief Parallel Rate Control (hierachical B) attribute. Read-only.
     *
     * This attribute conveys whether the encoder supports parallel rate control.
     * It is a integer value 0 - unsupported, > 0 - maximum layer supported.
     * This is the way when hireachical B frames are encoded, multiple independent B frames
     * on the same layer may be processed at same time. If supported, app may enable it by
     * setting enable_parallel_brc in VAEncMiscParameterRateControl,and the number of B frames
     * per layer per GOP will be passed to driver through VAEncMiscParameterParallelRateControl
     * structure.Currently three layers are defined.
     */
    VAConfigAttribEncParallelRateControl   = 29,
    /**
    * \brief Dynamic Scaling Attribute. Read-only.
    *
    * This attribute conveys whether encoder is capable to determine dynamic frame
    * resolutions adaptive to bandwidth utilization and processing power, etc.
    * It is a boolean value 0 - unsupported, 1 - supported.
    * If it is supported,for VP9, suggested frame resolution can be retrieved from VACodedBufferVP9Status.
    */
    VAConfigAttribEncDynamicScaling        = 30,
    /**
    * \brief frame size tolerance support
    * it indicates the tolerance of frame size
    */
    VAConfigAttribFrameSizeToleranceSupport = 31,
    /**
    * \brief Encode function type for FEI.
    *
    * This attribute conveys whether the driver supports different function types for encode.
    * It can be VA_FEI_FUNCTION_ENC, VA_FEI_FUNCTION_PAK, or VA_FEI_FUNCTION_ENC_PAK. Currently
    * it is for FEI entry point only.
    * Default is VA_FEI_FUNCTION_ENC_PAK.
    */
    VAConfigAttribFEIFunctionType     = 32,
    /**
     * \brief Maximum number of FEI MV predictors. Read-only.
     *
     * This attribute determines the maximum number of MV predictors the driver
     * can support to encode a single frame. 0 means no MV predictor is supported.
     * Currently it is for FEI entry point only.
     */
    VAConfigAttribFEIMVPredictors     = 33,
    /**
     * \brief Statistics attribute. Read-only.
     *
     * This attribute exposes a number of capabilities of the VAEntrypointStats entry
     * point. The attribute value is partitioned into fields as defined in the
     * VAConfigAttribValStats union. Currently it is for VAEntrypointStats only.
     */
    VAConfigAttribStats               = 34,
    /**
    * \brief Tile Support Attribute. Read-only.
    *
    * This attribute conveys whether encoder is capable to support tiles.
    * If not supported, the tile related parameters sent to encoder, such as
    * tiling structure, should be ignored. 0 - unsupported, 1 - supported.
    */
    VAConfigAttribEncTileSupport        = 35,
    /**
     * \brief whether accept rouding setting from application. Read-only.
     * This attribute is for encode quality, if it is report,
     * application can change the rounding setting by VAEncMiscParameterTypeCustomRoundingControl
     */
    VAConfigAttribCustomRoundingControl = 36,
    /**
     * \brief Encoding QP info block size attribute. Read-only.
     * This attribute conveys the block sizes that underlying driver
     * support for QP info for buffer #VAEncQpBuffer.
     */
    VAConfigAttribQPBlockSize            = 37,
    /**
     * \brief encode max frame size attribute. Read-only
     * attribute value \c VAConfigAttribValMaxFrameSize represent max frame size support
     */
    VAConfigAttribMaxFrameSize           = 38,
    /** \brief inter frame prediction directrion attribute. Read-only.
     * this attribute conveys the prediction direction (backward or forword) for specific config
     * the value could be  VA_PREDICTION_DIRECTION_XXXX. it can be combined with VAConfigAttribEncMaxRefFrames
     * to describe reference list , and the prediction direction. if this attrib is not present,both direction
     * should be supported, no restriction.
     * for example: normal HEVC encoding , maximum reference frame number in reflist 0 and reflist 1 is deduced
     * by  VAConfigAttribEncMaxRefFrames. so there are typical P frame, B frame,
     * if VAConfigAttribPredictionDirection is also present. it will stipulate prediction direction in both
     * reference list. if only one prediction direction present(such as PREVIOUS),all reference frame should be
     *  previous frame (PoC < current).
     */
    VAConfigAttribPredictionDirection   = 39,
    /** \brief combined submission of multiple frames from different streams, it is optimization for different HW
     * implementation, multiple frames encode/decode can improve HW concurrency
     */
    VAConfigAttribMultipleFrame         = 40,
    /** \brief priority setting for the context. Read-Write
     *  attribute value is \c VAConfigAttribValContextPriority
     *  this setting also could be update by \c VAContextParameterUpdateBuffer
     */
    VAConfigAttribContextPriority       = 41,
    /** \brief AV1 decoding features.  Read-only.
     *
     * This attribute describes the supported features of an
     * AV1 decoder configuration.  The value returned uses the
     * VAConfigAttribValDecAV1Features type.
     */
    VAConfigAttribDecAV1Features    = 42,
    /** \brief TEE could be any HW secure device. Read-only */
    VAConfigAttribTEEType               = 43,
    /** \brief TEE type client is a specific module supporting specific functions in TEE. Read-only*/
    VAConfigAttribTEETypeClient         = 44,
    /**
     * \brief Cipher algorithm of the protected content session.
     *
     * This attribute specifies the cipher algorithm of the protected content session. It
     * could be \c VA_PC_CIPHER_AES, etc....
     */
    VAConfigAttribProtectedContentCipherAlgorithm = 45,
    /**
     * \brief Cipher block size of the protected content session.
     *
     * This attribute specifies the block size of the protected content session. It could be
     * \c VA_PC_BLOCK_SIZE_128, \c VA_PC_BLOCK_SIZE_192, or \c VA_PC_BLOCK_SIZE_256, etc....
     */
    VAConfigAttribProtectedContentCipherBlockSize = 46,
    /**
     * \brief Cipher mode of the protected content session.
     *
     * This attribute specifies the cipher mode of the protected content session. It could
     * be \c VA_PC_CIPHER_MODE_ECB, \c VA_PC_CIPHER_MODE_CBC, \c VA_PC_CIPHER_MODE_CTR, etc...
     */
    VAConfigAttribProtectedContentCipherMode = 47,
    /**
     * \brief Decryption sample type of the protected content session.
     *
     * This attribute specifies the decryption sample type of the protected content session.
     * It could be \c VA_PC_SAMPLE_TYPE_FULLSAMPLE or \c VA_PC_SAMPLE_TYPE_SUBSAMPLE.
     */
    VAConfigAttribProtectedContentCipherSampleType = 48,
    /**
     * \brief Special usage attribute of the protected session.
     *
     * The attribute specifies the flow for the protected session could be used. For
     * example, it could be \c VA_PC_USAGE_DEFAULT, \c VA_PC_USAGE_WIDEVINE, etc....
     */
    VAConfigAttribProtectedContentUsage = 49,

    /** \brief HEVC/H.265 encoding features.  Read-only.
     *
     * This attribute describes the supported features of an
     * HEVC/H.265 encoder configuration.  The value returned uses the
     * VAConfigAttribValEncHEVCFeatures type.
     *
     * If this attribute is supported by a driver then it must also
     * support the VAConfigAttribEncHEVCBlockSizes attribute.
     */
    VAConfigAttribEncHEVCFeatures       = 50,
    /** \brief HEVC/H.265 encoding block sizes.  Read-only.
     *
     * This attribute describes the supported coding tree and transform
     * block sizes of an HEVC/H.265 encoder configuration.  The value
     * returned uses the VAConfigAttribValEncHEVCBlockSizes type.
     *
     * If this attribute is supported by a driver then it must also
     * support the VAConfigAttribEncHEVCFeatures attribute.
     */
    VAConfigAttribEncHEVCBlockSizes     = 51,
    /**
     * \brief AV1 encoding attribute. Read-only.
     *
     * This attribute exposes a number of capabilities of the underlying
     * AV1 implementation. The attribute value is partitioned into fields as defined in the
     * VAConfigAttribValEncAV1 union.
     */
    VAConfigAttribEncAV1                = 52,
    /**
     * \brief AV1 encoding attribute extend1. Read-only.
     *
     * This attribute exposes a number of capabilities of the underlying
     * AV1 implementation. The attribute value is partitioned into fields as defined in the
     * VAConfigAttribValEncAV1Ext1 union.
     */
    VAConfigAttribEncAV1Ext1            = 53,
    /**
     * \brief AV1 encoding attribute extend2. Read-only.
     *
     * This attribute exposes a number of capabilities of the underlying
     * AV1 implementation. The attribute value is partitioned into fields as defined in the
     * VAConfigAttribValEncAV1Ext2 union.
     */
    VAConfigAttribEncAV1Ext2            = 54,
    /** \brief Settings per block attribute for Encoding.  Read-only.
     *
     * This attribute describes whether to support delta qp per block,
     * the supported size of delta qp block and the size of delta QP in bytes.
     * The value returned uses the VAConfigAttribValEncPerBlockControl type.
     */
    VAConfigAttribEncPerBlockControl    = 55,
    /**@}*/
    VAConfigAttribTypeMax
} VAConfigAttribType;

/**
 * Configuration attributes
 * If there is more than one value for an attribute, a default
 * value will be assigned to the attribute if the client does not
 * specify the attribute when creating a configuration
 */
typedef struct _VAConfigAttrib {
    VAConfigAttribType type;
    uint32_t value; /* OR'd flags (bits) for this attribute */
} VAConfigAttrib;

/* Attribute values for VAConfigAttribRTFormat. */

#define VA_RT_FORMAT_YUV420 0x00000001  ///< YUV 4:2:0 8-bit.
#define VA_RT_FORMAT_YUV422 0x00000002  ///< YUV 4:2:2 8-bit.
#define VA_RT_FORMAT_YUV444 0x00000004  ///< YUV 4:4:4 8-bit.
#define VA_RT_FORMAT_YUV411 0x00000008  ///< YUV 4:1:1 8-bit.
#define VA_RT_FORMAT_YUV400 0x00000010  ///< Greyscale 8-bit.
#define VA_RT_FORMAT_YUV420_10  0x00000100  ///< YUV 4:2:0 10-bit.
#define VA_RT_FORMAT_YUV422_10  0x00000200  ///< YUV 4:2:2 10-bit.
#define VA_RT_FORMAT_YUV444_10  0x00000400  ///< YUV 4:4:4 10-bit.
#define VA_RT_FORMAT_YUV420_12  0x00001000  ///< YUV 4:2:0 12-bit.
#define VA_RT_FORMAT_YUV422_12  0x00002000  ///< YUV 4:2:2 12-bit.
#define VA_RT_FORMAT_YUV444_12  0x00004000  ///< YUV 4:4:4 12-bit.

#define VA_RT_FORMAT_RGB16  0x00010000  ///< Packed RGB, 16 bits per pixel.
#define VA_RT_FORMAT_RGB32  0x00020000  ///< Packed RGB, 32 bits per pixel, 8 bits per colour sample.
#define VA_RT_FORMAT_RGBP   0x00100000  ///< Planar RGB, 8 bits per sample.
#define VA_RT_FORMAT_RGB32_10   0x00200000  ///< Packed RGB, 32 bits per pixel, 10 bits per colour sample.

#define VA_RT_FORMAT_PROTECTED  0x80000000

#define VA_RT_FORMAT_RGB32_10BPP    VA_RT_FORMAT_RGB32_10   ///< @deprecated use VA_RT_FORMAT_RGB32_10 instead.
#define VA_RT_FORMAT_YUV420_10BPP   VA_RT_FORMAT_YUV420_10  ///< @deprecated use VA_RT_FORMAT_YUV420_10 instead.

/** @name Attribute values for VAConfigAttribRateControl */
/**@{*/
/** \brief Driver does not support any form of rate control. */
#define VA_RC_NONE                      0x00000001
/** \brief Constant bitrate. */
#define VA_RC_CBR                       0x00000002
/** \brief Variable bitrate. */
#define VA_RC_VBR                       0x00000004
/** \brief Video conference mode. */
#define VA_RC_VCM                       0x00000008
/** \brief Constant QP. */
#define VA_RC_CQP                       0x00000010
/** \brief Variable bitrate with peak rate higher than average bitrate. */
#define VA_RC_VBR_CONSTRAINED           0x00000020
/** \brief Intelligent Constant Quality. Provided an initial ICQ_quality_factor,
 *  adjusts QP at a frame and MB level based on motion to improve subjective quality. */
#define VA_RC_ICQ           0x00000040
/** \brief Macroblock based rate control.  Per MB control is decided
 *  internally in the encoder. It may be combined with other RC modes, except CQP. */
#define VA_RC_MB                        0x00000080
/** \brief Constant Frame Size, it is used for small tolerent  */
#define VA_RC_CFS                       0x00000100
/** \brief Parallel BRC, for hierachical B.
 *
 *  For hierachical B, B frames can be refered by other B frames.
 *  Currently three layers of hierachy are defined:
 *  B0 - regular B, no reference to other B frames.
 *  B1 - reference to only I, P and regular B0 frames.
 *  B2 - reference to any other frames, including B1.
 *  In Hierachical B structure, B frames on the same layer can be processed
 *  simultaneously. And BRC would adjust accordingly. This is so called
 *  Parallel BRC. */
#define VA_RC_PARALLEL                  0x00000200
/** \brief Quality defined VBR
 * Use Quality factor to determine the good enough QP for each MB such that
 * good enough quality can be obtained without waste of bits
 * for this BRC mode, you must set all legacy VBR parameters
 * and reuse quality_factor in \c VAEncMiscParameterRateControl
 * */
#define VA_RC_QVBR                      0x00000400
/** \brief Average VBR
 *  Average variable bitrate control algorithm focuses on overall encoding
 *  quality while meeting the specified target bitrate, within the accuracy
 *  range, after a convergence period.
 *  bits_per_second in VAEncMiscParameterRateControl is target bitrate for AVBR.
 *  Convergence is specified in the unit of frame.
 *  window_size in VAEncMiscParameterRateControl is equal to convergence for AVBR.
 *  Accuracy is in the range of [1,100], 1 means one percent, and so on.
 *  target_percentage in VAEncMiscParameterRateControl is equal to accuracy for AVBR.
 * */
#define VA_RC_AVBR                      0x00000800
/** \brief Transport Controlled BRC
 *  Specific bitrate control for real time streaming.
 *  TCBRC can instantly react to channel change to remove or significantly reduce the delay.
 *  Application (transport) provides channel feedback to driver through TargetFrameSize.
 *  When channel condition is very good (almost no constraint on instant frame size),
 *  the app should set target frame size as zero. Otherwise, channel capacity divided by fps
 *  should be used.
 * */
#define VA_RC_TCBRC                     0x00001000

/**@}*/

/** @name Attribute values for VAConfigAttribDecSliceMode */
/**@{*/
/** \brief Driver supports normal mode for slice decoding */
#define VA_DEC_SLICE_MODE_NORMAL       0x00000001
/** \brief Driver supports base mode for slice decoding */
#define VA_DEC_SLICE_MODE_BASE         0x00000002

/** @name Attribute values for VAConfigAttribDecJPEG */
/**@{*/
typedef union _VAConfigAttribValDecJPEG {
    struct {
        /** \brief Set to (1 << VA_ROTATION_xxx) for supported rotation angles. */
        uint32_t rotation : 4;
        /** \brief Reserved for future use. */
        uint32_t reserved : 28;
    } bits;
    uint32_t value;
} VAConfigAttribValDecJPEG;
/** @name Attribute values for VAConfigAttribDecProcessing */
/**@{*/
/** \brief No decoding + processing in a single decoding call. */
#define VA_DEC_PROCESSING_NONE     0x00000000
/** \brief Decode + processing in a single decoding call. */
#define VA_DEC_PROCESSING          0x00000001
/**@}*/

/** @name Attribute values for VAConfigAttribEncPackedHeaders */
/**@{*/
/** \brief Driver does not support any packed headers mode. */
#define VA_ENC_PACKED_HEADER_NONE       0x00000000
/**
 * \brief Driver supports packed sequence headers. e.g. SPS for H.264.
 *
 * Application must provide it to driver once this flag is returned through
 * vaGetConfigAttributes()
 */
#define VA_ENC_PACKED_HEADER_SEQUENCE   0x00000001
/**
 * \brief Driver supports packed picture headers. e.g. PPS for H.264.
 *
 * Application must provide it to driver once this falg is returned through
 * vaGetConfigAttributes()
 */
#define VA_ENC_PACKED_HEADER_PICTURE    0x00000002
/**
 * \brief Driver supports packed slice headers. e.g. slice_header() for H.264.
 *
 * Application must provide it to driver once this flag is returned through
 * vaGetConfigAttributes()
 */
#define VA_ENC_PACKED_HEADER_SLICE      0x00000004
/**
 * \brief Driver supports misc packed headers. e.g. SEI for H.264.
 *
 * @deprecated
 * This is a deprecated packed header flag, All applications can use
 * \c VA_ENC_PACKED_HEADER_RAW_DATA to pass the corresponding packed
 * header data buffer to the driver
 */
#define VA_ENC_PACKED_HEADER_MISC       0x00000008
/** \brief Driver supports raw packed header, see VAEncPackedHeaderRawData */
#define VA_ENC_PACKED_HEADER_RAW_DATA   0x00000010
/**@}*/

/** @name Attribute values for VAConfigAttribEncInterlaced */
/**@{*/
/** \brief Driver does not support interlaced coding. */
#define VA_ENC_INTERLACED_NONE          0x00000000
/** \brief Driver supports interlaced frame coding. */
#define VA_ENC_INTERLACED_FRAME         0x00000001
/** \brief Driver supports interlaced field coding. */
#define VA_ENC_INTERLACED_FIELD         0x00000002
/** \brief Driver supports macroblock adaptive frame field coding. */
#define VA_ENC_INTERLACED_MBAFF         0x00000004
/** \brief Driver supports picture adaptive frame field coding. */
#define VA_ENC_INTERLACED_PAFF          0x00000008
/**@}*/

/** @name Attribute values for VAConfigAttribEncSliceStructure */
/**@{*/
/** \brief Driver supports a power-of-two number of rows per slice. */
#define VA_ENC_SLICE_STRUCTURE_POWER_OF_TWO_ROWS        0x00000001
/** \brief Driver supports an arbitrary number of macroblocks per slice. */
#define VA_ENC_SLICE_STRUCTURE_ARBITRARY_MACROBLOCKS    0x00000002
/** \brief Driver support 1 row per slice */
#define VA_ENC_SLICE_STRUCTURE_EQUAL_ROWS               0x00000004
/** \brief Driver support max encoded slice size per slice */
#define VA_ENC_SLICE_STRUCTURE_MAX_SLICE_SIZE           0x00000008
/** \brief Driver supports an arbitrary number of rows per slice. */
#define VA_ENC_SLICE_STRUCTURE_ARBITRARY_ROWS           0x00000010
/** \brief Driver supports any number of rows per slice but they must be the same
*       for all slices except for the last one, which must be equal or smaller
*       to the previous slices. */
#define VA_ENC_SLICE_STRUCTURE_EQUAL_MULTI_ROWS         0x00000020
/**@}*/

/** \brief Attribute value for VAConfigAttribMaxFrameSize */
typedef union _VAConfigAttribValMaxFrameSize {
    struct {
        /** \brief support max frame size
          * if max_frame_size == 1, VAEncMiscParameterTypeMaxFrameSize/VAEncMiscParameterBufferMaxFrameSize
          * could be used to set the frame size, if multiple_pass also equal 1, VAEncMiscParameterTypeMultiPassFrameSize
          * VAEncMiscParameterBufferMultiPassFrameSize could be used to set frame size and pass information
          */
        uint32_t max_frame_size : 1;
        /** \brief multiple_pass support */
        uint32_t multiple_pass  : 1;
        /** \brief reserved bits for future, must be zero*/
        uint32_t reserved       : 30;
    } bits;
    uint32_t value;
} VAConfigAttribValMaxFrameSize;

/** \brief Attribute value for VAConfigAttribEncJPEG */
typedef union _VAConfigAttribValEncJPEG {
    struct {
        /** \brief set to 1 for arithmatic coding. */
        uint32_t arithmatic_coding_mode : 1;
        /** \brief set to 1 for progressive dct. */
        uint32_t progressive_dct_mode : 1;
        /** \brief set to 1 for non-interleaved. */
        uint32_t non_interleaved_mode : 1;
        /** \brief set to 1 for differential. */
        uint32_t differential_mode : 1;
        uint32_t max_num_components : 3;
        uint32_t max_num_scans : 4;
        uint32_t max_num_huffman_tables : 3;
        uint32_t max_num_quantization_tables : 3;
    } bits;
    uint32_t value;
} VAConfigAttribValEncJPEG;

/** @name Attribute values for VAConfigAttribEncQuantization */
/**@{*/
/** \brief Driver does not support special types of quantization */
#define VA_ENC_QUANTIZATION_NONE                        0x00000000
/** \brief Driver supports trellis quantization */
#define VA_ENC_QUANTIZATION_TRELLIS_SUPPORTED           0x00000001
/**@}*/

/** @name Attribute values for VAConfigAttribPredictionDirection */
/**@{*/
/** \brief Driver support forward reference frame (inter frame for vpx, P frame for H26x MPEG)
 * can work with the VAConfigAttribEncMaxRefFrames. for example: low delay B frame of HEVC.
 * these value can be OR'd together. typical value should be VA_PREDICTION_DIRECTION_PREVIOUS
 * or VA_PREDICTION_DIRECTION_PREVIOUS | VA_PREDICTION_DIRECTION_FUTURE, theoretically, there
 * are no stream only include future reference frame.
 */
#define VA_PREDICTION_DIRECTION_PREVIOUS                0x00000001
/** \brief Driver support backward prediction frame/slice */
#define VA_PREDICTION_DIRECTION_FUTURE                  0x00000002
/** \brief Dirver require both reference list must be not empty for inter frame */
#define VA_PREDICTION_DIRECTION_BI_NOT_EMPTY            0x00000004
/**@}*/

/** @name Attribute values for VAConfigAttribEncIntraRefresh */
/**@{*/
/** \brief Driver does not support intra refresh */
#define VA_ENC_INTRA_REFRESH_NONE                       0x00000000
/** \brief Driver supports column based rolling intra refresh */
#define VA_ENC_INTRA_REFRESH_ROLLING_COLUMN             0x00000001
/** \brief Driver supports row based rolling intra refresh */
#define VA_ENC_INTRA_REFRESH_ROLLING_ROW                0x00000002
/** \brief Driver supports adaptive intra refresh */
#define VA_ENC_INTRA_REFRESH_ADAPTIVE                   0x00000010
/** \brief Driver supports cyclic intra refresh */
#define VA_ENC_INTRA_REFRESH_CYCLIC                     0x00000020
/** \brief Driver supports intra refresh of P frame*/
#define VA_ENC_INTRA_REFRESH_P_FRAME                    0x00010000
/** \brief Driver supports intra refresh of B frame */
#define VA_ENC_INTRA_REFRESH_B_FRAME                    0x00020000
/** \brief Driver supports intra refresh of multiple reference encoder */
#define VA_ENC_INTRA_REFRESH_MULTI_REF                  0x00040000

/**@}*/

/** \brief Attribute value for VAConfigAttribEncROI */
typedef union _VAConfigAttribValEncROI {
    struct {
        /** \brief The number of ROI regions supported, 0 if ROI is not supported. */
        uint32_t num_roi_regions        : 8;
        /**
         * \brief A flag indicates whether ROI priority is supported
         *
         * \ref roi_rc_priority_support equal to 1 specifies the underlying driver supports
         * ROI priority when VAConfigAttribRateControl != VA_RC_CQP, user can use \c roi_value
         * in #VAEncROI to set ROI priority. \ref roi_rc_priority_support equal to 0 specifies
         * the underlying driver doesn't support ROI priority.
         *
         * User should ignore \ref roi_rc_priority_support when VAConfigAttribRateControl == VA_RC_CQP
         * because ROI delta QP is always required when VAConfigAttribRateControl == VA_RC_CQP.
         */
        uint32_t roi_rc_priority_support    : 1;
        /**
         * \brief A flag indicates whether ROI delta QP is supported
         *
         * \ref roi_rc_qp_delta_support equal to 1 specifies the underlying driver supports
         * ROI delta QP when VAConfigAttribRateControl != VA_RC_CQP, user can use \c roi_value
         * in #VAEncROI to set ROI delta QP. \ref roi_rc_qp_delta_support equal to 0 specifies
         * the underlying driver doesn't support ROI delta QP.
         *
         * User should ignore \ref roi_rc_qp_delta_support when VAConfigAttribRateControl == VA_RC_CQP
         * because ROI delta QP is always required when VAConfigAttribRateControl == VA_RC_CQP.
         */
        uint32_t roi_rc_qp_delta_support    : 1;
        uint32_t reserved                   : 22;
    } bits;
    uint32_t value;
} VAConfigAttribValEncROI;

/** \brief Attribute value for VAConfigAttribEncRateControlExt */
typedef union _VAConfigAttribValEncRateControlExt {
    struct {
        /**
         * \brief The maximum number of temporal layers minus 1
         *
         * \ref max_num_temporal_layers_minus1 plus 1 specifies the maximum number of temporal
         * layers that supported by the underlying driver. \ref max_num_temporal_layers_minus1
         * equal to 0 implies the underlying driver doesn't support encoding with temporal layer.
         */
        uint32_t max_num_temporal_layers_minus1      : 8;

        /**
         * /brief support temporal layer bit-rate control flag
         *
         * \ref temporal_layer_bitrate_control_flag equal to 1 specifies the underlying driver
         * can support bit-rate control per temporal layer when (#VAConfigAttribRateControl == #VA_RC_CBR ||
         * #VAConfigAttribRateControl == #VA_RC_VBR).
         *
         * The underlying driver must set \ref temporal_layer_bitrate_control_flag to 0 when
         * \c max_num_temporal_layers_minus1 is equal to 0
         *
         * To use bit-rate control per temporal layer, an application must send the right layer
         * structure via #VAEncMiscParameterTemporalLayerStructure at the beginning of a coded sequence
         * and then followed by #VAEncMiscParameterRateControl and #VAEncMiscParameterFrameRate structures
         * for each layer, using the \c temporal_id field as the layer identifier. Otherwise
         * the driver doesn't use bitrate control per temporal layer if an application doesn't send the
         * layer structure via #VAEncMiscParameterTemporalLayerStructure to the driver. The driver returns
         * VA_STATUS_ERROR_INVALID_PARAMETER if an application sends a wrong layer structure or doesn't send
         * #VAEncMiscParameterRateControl and #VAEncMiscParameterFrameRate for each layer.
         *
         * The driver will ignore #VAEncMiscParameterTemporalLayerStructure and the \c temporal_id field
         * in #VAEncMiscParameterRateControl and #VAEncMiscParameterFrameRate if
         * \ref temporal_layer_bitrate_control_flag is equal to 0 or #VAConfigAttribRateControl == #VA_RC_CQP
         */
        uint32_t temporal_layer_bitrate_control_flag : 1;
        uint32_t reserved                            : 23;
    } bits;
    uint32_t value;
} VAConfigAttribValEncRateControlExt;

/** \brief Attribute value for VAConfigAttribMultipleFrame*/
typedef union _VAConfigAttribValMultipleFrame {
    struct {
        /** \brief max num of concurrent frames from different stream */
        uint32_t max_num_concurrent_frames      : 8;
        /** \brief indicate whether all stream must support same quality level
         *  if mixed_quality_level == 0, same quality level setting for multple streams is required
         *  if mixed_quality_level == 1, different stream can have different quality level*/
        uint32_t mixed_quality_level            : 1;
        /** \brief reserved bit for future, must be zero */
        uint32_t reserved                       : 23;
    } bits;
    uint32_t value;
} VAConfigAttribValMultipleFrame;

/** brief Attribute value VAConfigAttribValContextPriority */
typedef union _VAConfigAttribValContextPriority {
    struct {
        /** \brief the priority , for the Query operation (read) it represents highest priority
         * for the set operation (write), value should be [0~highest priority] , 0 is lowest priority*/
        uint32_t priority     : 16;
        /** \brief reserved bits for future, must be zero*/
        uint32_t reserved     : 16;
    } bits;
    uint32_t value;
} VAConfigAttribValContextPriority;

/** brief Attribute value VAConfigAttribEncPerBlockControl */
typedef union _VAConfigAttribValEncPerBlockControl {
    struct {
        /** \brief whether to support dela qp per block */
        uint32_t delta_qp_support         : 1;
        /** \brief supported size of delta qp block */
        uint32_t log2_delta_qp_block_size : 4;
        /** \brief size of delta qp per block in bytes*/
        uint32_t delta_qp_size_in_bytes   : 3;
        /** \brief reserved bit for future, must be zero */
        uint32_t reserved                 : 24;
    } bits;
    uint32_t value;
} VAConfigAttribValEncPerBlockControl;

/** @name Attribute values for VAConfigAttribProtectedContentCipherAlgorithm */
/** \brief AES cipher */
#define VA_PC_CIPHER_AES                    0x00000001

/** @name Attribute values for VAConfigAttribProtectedContentCipherBlockSize */
/** \brief 128 bits block size */
#define VA_PC_BLOCK_SIZE_128                0x00000001
/** \brief 192 bits block size */
#define VA_PC_BLOCK_SIZE_192                0x00000002
/** \brief 256 bits block size */
#define VA_PC_BLOCK_SIZE_256                0x00000004

/** @name Attribute values for VAConfigAttribProtectedContentCipherMode */
/** \brief AES ECB */
#define VA_PC_CIPHER_MODE_ECB               0x00000001
/** \brief AES CBC */
#define VA_PC_CIPHER_MODE_CBC               0x00000002
/** \brief AES CTR */
#define VA_PC_CIPHER_MODE_CTR               0x00000004

/** @name Attribute values for VAConfigAttribProtectedContentCipherSampleType */
/** \brief Full sample */
#define VA_PC_SAMPLE_TYPE_FULLSAMPLE        0x00000001
/** \brief Sub sample */
#define VA_PC_SAMPLE_TYPE_SUBSAMPLE         0x00000002

/** @name Attribute values for VAConfigAttribProtectedContentUsage */
/** \brief Default usage */
#define VA_PC_USAGE_DEFAULT                 0x00000000
/** \brief Widevine */
#define VA_PC_USAGE_WIDEVINE                0x00000001

/** @name Attribute values for VAConfigAttribProcessingRate. */
/**@{*/
/** \brief Driver does not support processing rate report */
#define VA_PROCESSING_RATE_NONE                       0x00000000
/** \brief Driver supports encode processing rate report  */
#define VA_PROCESSING_RATE_ENCODE                     0x00000001
/** \brief Driver supports decode processing rate report  */
#define VA_PROCESSING_RATE_DECODE                     0x00000002
/**@}*/
/**
 * if an attribute is not applicable for a given
 * profile/entrypoint pair, then set the value to the following
 */
#define VA_ATTRIB_NOT_SUPPORTED 0x80000000

/** Get maximum number of profiles supported by the implementation */
int vaMaxNumProfiles(
    VADisplay dpy
);

/** Get maximum number of entrypoints supported by the implementation */
int vaMaxNumEntrypoints(
    VADisplay dpy
);

/** Get maximum number of attributs supported by the implementation */
int vaMaxNumConfigAttributes(
    VADisplay dpy
);

/**
 * Query supported profiles
 * The caller must provide a "profile_list" array that can hold at
 * least vaMaxNumProfile() entries. The actual number of profiles
 * returned in "profile_list" is returned in "num_profile".
 */
VAStatus vaQueryConfigProfiles(
    VADisplay dpy,
    VAProfile *profile_list,    /* out */
    int *num_profiles       /* out */
);

/**
 * Query supported entrypoints for a given profile
 * The caller must provide an "entrypoint_list" array that can hold at
 * least vaMaxNumEntrypoints() entries. The actual number of entrypoints
 * returned in "entrypoint_list" is returned in "num_entrypoints".
 */
VAStatus vaQueryConfigEntrypoints(
    VADisplay dpy,
    VAProfile profile,
    VAEntrypoint *entrypoint_list,  /* out */
    int *num_entrypoints        /* out */
);

/**
 * Get attributes for a given profile/entrypoint pair
 * The caller must provide an "attrib_list" with all attributes to be
 * retrieved.  Upon return, the attributes in "attrib_list" have been
 * updated with their value.  Unknown attributes or attributes that are
 * not supported for the given profile/entrypoint pair will have their
 * value set to VA_ATTRIB_NOT_SUPPORTED
 */
VAStatus vaGetConfigAttributes(
    VADisplay dpy,
    VAProfile profile,
    VAEntrypoint entrypoint,
    VAConfigAttrib *attrib_list, /* in/out */
    int num_attribs
);

/** Generic ID type, can be re-typed for specific implementation */
typedef unsigned int VAGenericID;

typedef VAGenericID VAConfigID;

/**
 * Create a configuration for the video decode/encode/processing pipeline
 * it passes in the attribute list that specifies the attributes it cares
 * about, with the rest taking default values.
 */
VAStatus vaCreateConfig(
    VADisplay dpy,
    VAProfile profile,
    VAEntrypoint entrypoint,
    VAConfigAttrib *attrib_list,
    int num_attribs,
    VAConfigID *config_id /* out */
);

/**
 * Free resources associdated with a given config
 */
VAStatus vaDestroyConfig(
    VADisplay dpy,
    VAConfigID config_id
);

/**
 * Query all attributes for a given configuration
 * The profile of the configuration is returned in "profile"
 * The entrypoint of the configuration is returned in "entrypoint"
 * The caller must provide an "attrib_list" array that can hold at least
 * vaMaxNumConfigAttributes() entries. The actual number of attributes
 * returned in "attrib_list" is returned in "num_attribs"
 */
VAStatus vaQueryConfigAttributes(
    VADisplay dpy,
    VAConfigID config_id,
    VAProfile *profile,     /* out */
    VAEntrypoint *entrypoint,   /* out */
    VAConfigAttrib *attrib_list,/* out */
    int *num_attribs        /* out */
);


/**
 * Contexts and Surfaces
 *
 * Context represents a "virtual" video decode, encode or video processing
 * pipeline. Surfaces are render targets for a given context. The data in the
 * surfaces are not accessible to the client except if derived image is supported
 * and the internal data format of the surface is implementation specific.
 *
 * Surfaces are provided as a hint of what surfaces will be used when the context
 * is created through vaCreateContext(). A surface may be used by different contexts
 * at the same time as soon as application can make sure the operations are synchronized
 * between different contexts, e.g. a surface is used as the output of a decode context
 * and the input of a video process context. Surfaces can only be destroyed after all
 * contexts using these surfaces have been destroyed.
 *
 * Both contexts and surfaces are identified by unique IDs and its
 * implementation specific internals are kept opaque to the clients
 */

typedef VAGenericID VAContextID;

typedef VAGenericID VASurfaceID;

#define VA_INVALID_ID       0xffffffff
#define VA_INVALID_SURFACE  VA_INVALID_ID

/** \brief Generic value types. */
typedef enum  {
    VAGenericValueTypeInteger = 1,      /**< 32-bit signed integer. */
    VAGenericValueTypeFloat,            /**< 32-bit floating-point value. */
    VAGenericValueTypePointer,          /**< Generic pointer type */
    VAGenericValueTypeFunc              /**< Pointer to function */
} VAGenericValueType;

/** \brief Generic function type. */
typedef void (*VAGenericFunc)(void);

/** \brief Generic value. */
typedef struct _VAGenericValue {
    /** \brief Value type. See #VAGenericValueType. */
    VAGenericValueType  type;
    /** \brief Value holder. */
    union {
        /** \brief 32-bit signed integer. */
        int32_t             i;
        /** \brief 32-bit float. */
        float           f;
        /** \brief Generic pointer. */
        void           *p;
        /** \brief Pointer to function. */
        VAGenericFunc   fn;
    }                   value;
} VAGenericValue;

/** @name Surface attribute flags */
/**@{*/
/** \brief Surface attribute is not supported. */
#define VA_SURFACE_ATTRIB_NOT_SUPPORTED 0x00000000
/** \brief Surface attribute can be got through vaQuerySurfaceAttributes(). */
#define VA_SURFACE_ATTRIB_GETTABLE      0x00000001
/** \brief Surface attribute can be set through vaCreateSurfaces(). */
#define VA_SURFACE_ATTRIB_SETTABLE      0x00000002
/**@}*/

/** \brief Surface attribute types. */
typedef enum {
    VASurfaceAttribNone = 0,
    /**
     * \brief Pixel format as a FOURCC (int, read/write).
     *
     * When vaQuerySurfaceAttributes() is called, the driver will return one
     * PixelFormat attribute per supported pixel format.
     *
     * When provided as an input to vaCreateSurfaces(), the driver will
     * allocate a surface with the provided pixel format.
     */
    VASurfaceAttribPixelFormat,
    /** \brief Minimal width in pixels (int, read-only). */
    VASurfaceAttribMinWidth,
    /** \brief Maximal width in pixels (int, read-only). */
    VASurfaceAttribMaxWidth,
    /** \brief Minimal height in pixels (int, read-only). */
    VASurfaceAttribMinHeight,
    /** \brief Maximal height in pixels (int, read-only). */
    VASurfaceAttribMaxHeight,
    /** \brief Surface memory type expressed in bit fields (int, read/write). */
    VASurfaceAttribMemoryType,
    /** \brief External buffer descriptor (pointer, write).
     *
     * Refer to the documentation for the memory type being created to
     * determine what descriptor structure to pass here.  If not otherwise
     * stated, the common VASurfaceAttribExternalBuffers should be used.
     */
    VASurfaceAttribExternalBufferDescriptor,
    /** \brief Surface usage hint, gives the driver a hint of intended usage
     *  to optimize allocation (e.g. tiling) (int, read/write). */
    VASurfaceAttribUsageHint,
    /** \brief List of possible DRM format modifiers (pointer, write).
     *
     * The value must be a pointer to a VADRMFormatModifierList. This can only
     * be used when allocating a new buffer, it's invalid to use this attribute
     * when importing an existing buffer.
     */
    VASurfaceAttribDRMFormatModifiers,
    /** \brief Number of surface attributes. */
    VASurfaceAttribCount
} VASurfaceAttribType;

/** \brief Surface attribute. */
typedef struct _VASurfaceAttrib {
    /** \brief Type. */
    VASurfaceAttribType type;
    /** \brief Flags. See "Surface attribute flags". */
    uint32_t        flags;
    /** \brief Value. See "Surface attribute types" for the expected types. */
    VAGenericValue      value;
} VASurfaceAttrib;

/**
 * @name VASurfaceAttribMemoryType values in bit fields.
 * Bits 0:7 are reserved for generic types. Bits 31:28 are reserved for
 * Linux DRM. Bits 23:20 are reserved for Android. Bits 19:16 are reserved for Win32.
 * DRM, Android and Win32 specific types are defined in respective va_*.h header files.
 */
/**@{*/
/** \brief VA memory type (default) is supported. */
#define VA_SURFACE_ATTRIB_MEM_TYPE_VA           0x00000001
/** \brief V4L2 buffer memory type is supported. */
#define VA_SURFACE_ATTRIB_MEM_TYPE_V4L2         0x00000002
/** \brief User pointer memory type is supported. */
#define VA_SURFACE_ATTRIB_MEM_TYPE_USER_PTR     0x00000004
/**@}*/

/**
 * \brief VASurfaceAttribExternalBuffers structure for
 * the VASurfaceAttribExternalBufferDescriptor attribute.
 */
typedef struct _VASurfaceAttribExternalBuffers {
    /** \brief pixel format in fourcc. */
    uint32_t pixel_format;
    /** \brief width in pixels. */
    uint32_t width;
    /** \brief height in pixels. */
    uint32_t height;
    /** \brief total size of the buffer in bytes. */
    uint32_t data_size;
    /** \brief number of planes for planar layout */
    uint32_t num_planes;
    /** \brief pitch for each plane in bytes */
    uint32_t pitches[4];
    /** \brief offset for each plane in bytes */
    uint32_t offsets[4];
    /** \brief buffer handles or user pointers */
    uintptr_t *buffers;
    /** \brief number of elements in the "buffers" array */
    uint32_t num_buffers;
    /** \brief flags. See "Surface external buffer descriptor flags". */
    uint32_t flags;
    /** \brief reserved for passing private data */
    void *private_data;
} VASurfaceAttribExternalBuffers;

/** @name VASurfaceAttribExternalBuffers flags */
/**@{*/
/** \brief Enable memory tiling */
#define VA_SURFACE_EXTBUF_DESC_ENABLE_TILING    0x00000001
/** \brief Memory is cacheable */
#define VA_SURFACE_EXTBUF_DESC_CACHED       0x00000002
/** \brief Memory is non-cacheable */
#define VA_SURFACE_EXTBUF_DESC_UNCACHED     0x00000004
/** \brief Memory is write-combined */
#define VA_SURFACE_EXTBUF_DESC_WC       0x00000008
/** \brief Memory is protected */
#define VA_SURFACE_EXTBUF_DESC_PROTECTED        0x80000000

/** @name VASurfaceAttribUsageHint attribute usage hint flags */
/**@{*/
/** \brief Surface usage not indicated. */
#define VA_SURFACE_ATTRIB_USAGE_HINT_GENERIC    0x00000000
/** \brief Surface used by video decoder. */
#define VA_SURFACE_ATTRIB_USAGE_HINT_DECODER    0x00000001
/** \brief Surface used by video encoder. */
#define VA_SURFACE_ATTRIB_USAGE_HINT_ENCODER    0x00000002
/** \brief Surface read by video post-processing. */
#define VA_SURFACE_ATTRIB_USAGE_HINT_VPP_READ   0x00000004
/** \brief Surface written by video post-processing. */
#define VA_SURFACE_ATTRIB_USAGE_HINT_VPP_WRITE  0x00000008
/** \brief Surface used for display. */
#define VA_SURFACE_ATTRIB_USAGE_HINT_DISPLAY    0x00000010
/** \brief Surface used for export to third-party APIs, e.g. via
 *  vaExportSurfaceHandle(). */
#define VA_SURFACE_ATTRIB_USAGE_HINT_EXPORT     0x00000020

/**@}*/

/**
 * \brief Queries surface attributes for the supplied config.
 *
 * This function queries for all supported attributes for the
 * supplied VA @config. In particular, if the underlying hardware
 * supports the creation of VA surfaces in various formats, then
 * this function will enumerate all pixel formats that are supported.
 *
 * The \c attrib_list array is allocated by the user and \c
 * num_attribs shall be initialized to the number of allocated
 * elements in that array. Upon successful return, the actual number
 * of attributes will be overwritten into \c num_attribs. Otherwise,
 * \c VA_STATUS_ERROR_MAX_NUM_EXCEEDED is returned and \c num_attribs
 * is adjusted to the number of elements that would be returned if
 * enough space was available.
 *
 * Note: it is perfectly valid to pass NULL to the \c attrib_list
 * argument when vaQuerySurfaceAttributes() is used to determine the
 * actual number of elements that need to be allocated.
 *
 * @param[in] dpy               the VA display
 * @param[in] config            the config identifying a codec or a video
 *     processing pipeline
 * @param[out] attrib_list      the output array of #VASurfaceAttrib elements
 * @param[in,out] num_attribs   the number of elements allocated on
 *      input, the number of elements actually filled in output
 */
VAStatus
vaQuerySurfaceAttributes(
    VADisplay           dpy,
    VAConfigID          config,
    VASurfaceAttrib    *attrib_list,
    unsigned int       *num_attribs
);

/**
 * \brief Creates an array of surfaces
 *
 * Creates an array of surfaces. The optional list of attributes shall
 * be constructed based on what the underlying hardware could expose
 * through vaQuerySurfaceAttributes().
 *
 * @param[in] dpy               the VA display
 * @param[in] format            the desired surface format. See \c VA_RT_FORMAT_*
 * @param[in] width             the surface width
 * @param[in] height            the surface height
 * @param[out] surfaces         the array of newly created surfaces
 * @param[in] num_surfaces      the number of surfaces to create
 * @param[in] attrib_list       the list of (optional) attributes, or \c NULL
 * @param[in] num_attribs       the number of attributes supplied in
 *     \c attrib_list, or zero
 */
VAStatus
vaCreateSurfaces(
    VADisplay           dpy,
    unsigned int        format,
    unsigned int        width,
    unsigned int        height,
    VASurfaceID        *surfaces,
    unsigned int        num_surfaces,
    VASurfaceAttrib    *attrib_list,
    unsigned int        num_attribs
);

/**
 * vaDestroySurfaces - Destroy resources associated with surfaces.
 *  Surfaces can only be destroyed after all contexts using these surfaces have been
 *  destroyed.
 *  dpy: display
 *  surfaces: array of surfaces to destroy
 *  num_surfaces: number of surfaces in the array to be destroyed.
 */
VAStatus vaDestroySurfaces(
    VADisplay dpy,
    VASurfaceID *surfaces,
    int num_surfaces
);

#define VA_PROGRESSIVE 0x1
/**
 * vaCreateContext - Create a context
 *  dpy: display
 *  config_id: configuration for the context
 *  picture_width: coded picture width
 *  picture_height: coded picture height
 *  flag: any combination of the following:
 *    VA_PROGRESSIVE (only progressive frame pictures in the sequence when set)
 *  render_targets: a hint for render targets (surfaces) tied to the context
 *  num_render_targets: number of render targets in the above array
 *  context: created context id upon return
 */
VAStatus vaCreateContext(
    VADisplay dpy,
    VAConfigID config_id,
    int picture_width,
    int picture_height,
    int flag,
    VASurfaceID *render_targets,
    int num_render_targets,
    VAContextID *context        /* out */
);

/**
 * vaDestroyContext - Destroy a context
 *  dpy: display
 *  context: context to be destroyed
 */
VAStatus vaDestroyContext(
    VADisplay dpy,
    VAContextID context
);

//Multi-frame context
typedef VAGenericID VAMFContextID;
/**
 * vaCreateMFContext - Create a multi-frame context
 *  interface encapsulating common for all streams memory objects and structures
 *  required for single GPU task submission from several VAContextID's.
 *  Allocation: This call only creates an instance, doesn't allocate any additional memory.
 *  Support identification: Application can identify multi-frame feature support by ability
 *  to create multi-frame context. If driver supports multi-frame - call successful,
 *  mf_context != NULL and VAStatus = VA_STATUS_SUCCESS, otherwise if multi-frame processing
 *  not supported driver returns VA_STATUS_ERROR_UNIMPLEMENTED and mf_context = NULL.
 *  return values:
 *  VA_STATUS_SUCCESS - operation successful.
 *  VA_STATUS_ERROR_UNIMPLEMENTED - no support for multi-frame.
 *  dpy: display adapter.
 *  mf_context: Multi-Frame context encapsulating all associated context
 *  for multi-frame submission.
 */
VAStatus vaCreateMFContext(
    VADisplay dpy,
    VAMFContextID *mf_context    /* out */
);

/**
 * vaMFAddContext - Provide ability to associate each context used for
 *  Multi-Frame submission and common Multi-Frame context.
 *  Try to add context to understand if it is supported.
 *  Allocation: this call allocates and/or reallocates all memory objects
 *  common for all contexts associated with particular Multi-Frame context.
 *  All memory required for each context(pixel buffers, internal driver
 *  buffers required for processing) allocated during standard vaCreateContext call for each context.
 *  Runtime dependency - if current implementation doesn't allow to run different entry points/profile,
 *  first context added will set entry point/profile for whole Multi-Frame context,
 *  all other entry points and profiles can be rejected to be added.
 *  Return values:
 *  VA_STATUS_SUCCESS - operation successful, context was added.
 *  VA_STATUS_ERROR_OPERATION_FAILED - something unexpected happened - application have to close
 *  current mf_context and associated contexts and start working with new ones.
 *  VA_STATUS_ERROR_INVALID_CONTEXT - ContextID is invalid, means:
 *  1 - mf_context is not valid context or
 *  2 - driver can't suport different VAEntrypoint or VAProfile simultaneosly
 *  and current context contradicts with previously added, application can continue with current mf_context
 *  and other contexts passed this call, rejected context can continue work in stand-alone
 *  mode or other mf_context.
 *  VA_STATUS_ERROR_UNSUPPORTED_ENTRYPOINT - particular context being added was created with with
 *  unsupported VAEntrypoint. Application can continue with current mf_context
 *  and other contexts passed this call, rejected context can continue work in stand-alone
 *  mode.
 *  VA_STATUS_ERROR_UNSUPPORTED_PROFILE - Current context with Particular VAEntrypoint is supported
 *  but VAProfile is not supported. Application can continue with current mf_context
 *  and other contexts passed this call, rejected context can continue work in stand-alone
 *  mode.
 *  dpy: display adapter.
 *  context: context being associated with Multi-Frame context.
 *  mf_context: - multi-frame context used to associate contexts for multi-frame submission.
 */
VAStatus vaMFAddContext(
    VADisplay dpy,
    VAMFContextID mf_context,
    VAContextID context
);

/**
 * vaMFReleaseContext - Removes context from multi-frame and
 *  association with multi-frame context.
 *  After association removed vaEndPicture will submit tasks, but not vaMFSubmit.
 *  Return values:
 *  VA_STATUS_SUCCESS - operation successful, context was removed.
 *  VA_STATUS_ERROR_OPERATION_FAILED - something unexpected happened.
 *  application need to destroy this VAMFContextID and all assotiated VAContextID
 *  dpy: display
 *  mf_context: VAMFContextID where context is added
 *  context: VAContextID to be added
 */
VAStatus vaMFReleaseContext(
    VADisplay dpy,
    VAMFContextID mf_context,
    VAContextID context
);

/**
 * Buffers
 * Buffers are used to pass various types of data from the
 * client to the server. The server maintains a data store
 * for each buffer created, and the client idenfies a buffer
 * through a unique buffer id assigned by the server.
 */

typedef VAGenericID VABufferID;

typedef enum {
    VAPictureParameterBufferType    = 0,
    VAIQMatrixBufferType        = 1,
    VABitPlaneBufferType        = 2,
    VASliceGroupMapBufferType       = 3,
    VASliceParameterBufferType      = 4,
    VASliceDataBufferType       = 5,
    VAMacroblockParameterBufferType = 6,
    VAResidualDataBufferType        = 7,
    VADeblockingParameterBufferType = 8,
    VAImageBufferType           = 9,
    VAProtectedSliceDataBufferType  = 10,
    VAQMatrixBufferType                 = 11,
    VAHuffmanTableBufferType            = 12,
    VAProbabilityBufferType             = 13,

    /* Following are encode buffer types */
    VAEncCodedBufferType        = 21,
    VAEncSequenceParameterBufferType    = 22,
    VAEncPictureParameterBufferType = 23,
    VAEncSliceParameterBufferType   = 24,
    VAEncPackedHeaderParameterBufferType = 25,
    VAEncPackedHeaderDataBufferType     = 26,
    VAEncMiscParameterBufferType    = 27,
    VAEncMacroblockParameterBufferType  = 28,
    VAEncMacroblockMapBufferType        = 29,

    /**
     * \brief Encoding QP buffer
     *
     * This buffer contains QP per MB for encoding. Currently
     * VAEncQPBufferH264 is defined for H.264 encoding, see
     * #VAEncQPBufferH264 for details
     */
    VAEncQPBufferType                   = 30,
    /* Following are video processing buffer types */
    /**
     * \brief Video processing pipeline parameter buffer.
     *
     * This buffer describes the video processing pipeline. See
     * #VAProcPipelineParameterBuffer for details.
     */
    VAProcPipelineParameterBufferType   = 41,
    /**
     * \brief Video filter parameter buffer.
     *
     * This buffer describes the video filter parameters. All buffers
     * inherit from #VAProcFilterParameterBufferBase, thus including
     * a unique filter buffer type.
     *
     * The default buffer used by most filters is #VAProcFilterParameterBuffer.
     * Filters requiring advanced parameters include, but are not limited to,
     * deinterlacing (#VAProcFilterParameterBufferDeinterlacing),
     * color balance (#VAProcFilterParameterBufferColorBalance), etc.
     */
    VAProcFilterParameterBufferType     = 42,
    /**
     * \brief FEI specific buffer types
     */
    VAEncFEIMVBufferType                = 43,
    VAEncFEIMBCodeBufferType            = 44,
    VAEncFEIDistortionBufferType        = 45,
    VAEncFEIMBControlBufferType         = 46,
    VAEncFEIMVPredictorBufferType       = 47,
    VAStatsStatisticsParameterBufferType = 48,
    /** \brief Statistics output for VAEntrypointStats progressive and top field of interlaced case*/
    VAStatsStatisticsBufferType         = 49,
    /** \brief Statistics output for VAEntrypointStats bottom field of interlaced case*/
    VAStatsStatisticsBottomFieldBufferType = 50,
    VAStatsMVBufferType                 = 51,
    VAStatsMVPredictorBufferType        = 52,
    /** Force MB's to be non skip for encode.it's per-mb control buffer, The width of the MB map
     * Surface is (width of the Picture in MB unit) * 1 byte, multiple of 64 bytes.
     * The height is (height of the picture in MB unit). The picture is either
     * frame or non-interleaved top or bottom field.  If the application provides this
     *surface, it will override the "skipCheckDisable" setting in VAEncMiscParameterEncQuality.
     */
    VAEncMacroblockDisableSkipMapBufferType = 53,
    /**
     * \brief HEVC FEI CTB level cmd buffer
     * it is CTB level information for future usage.
     */
    VAEncFEICTBCmdBufferType            = 54,
    /**
     * \brief HEVC FEI CU level data buffer
     * it's CTB level information for future usage
     */
    VAEncFEICURecordBufferType          = 55,
    /** decode stream out buffer, intermedia data of decode, it may include MV, MB mode etc.
      * it can be used to detect motion and analyze the frame contain  */
    VADecodeStreamoutBufferType             = 56,

    /** \brief HEVC Decoding Subset Parameter buffer type
     *
     * The subsets parameter buffer is concatenation with one or multiple
     * subset entry point offsets. All the offset values are layed out one
     * by one according to slice order with first slice segment first, second
     * slice segment second, etc... The entry number is indicated by parameter
     * \ref num_entry_point_offsets. And the first entry position of the entry
     * point offsets for any slice segment is indicated by parameter
     * entry_offset_to_subset_array in VAPictureParameterBufferHEVC data structure.
     */
    VASubsetsParameterBufferType        = 57,
    /** \brief adjust context parameters dynamically
     *
     * this parameter is used to update context parameters, detail parameter is in
     *  \c VAContextParameterUpdateBuffer
     */
    VAContextParameterUpdateBufferType  = 58,
    /**
     * \brief Protected session execution buffer type
     *
     * It's for TEE execution usage (vaProtectedSessionExecute()). The buffer structure is in
     * \c VAProtectedSessionExecuteBuffer
     */
    VAProtectedSessionExecuteBufferType = 59,

    /** \brief Encryption parameters buffer for protected content session.
     *
     * Refer to \c VAEncryptionParameters
    */
    VAEncryptionParameterBufferType = 60,

    /**
     * \brief Encoding delta QP per block buffer
     *
     * This buffer only could be created and accepted
     * when \c VAConfigAttribValEncPerBlockControl delta_qp_support == 1.
     * This input buffer contains delta QP per block for encoding.
     * The supported size of delta QP block and the size of delta QP
     * must be quried from \c VAConfigAttribValEncPerBlockControl.
     */
    VAEncDeltaQpPerBlockBufferType   = 61,

    VABufferTypeMax
} VABufferType;

/** \brief update the context parameter
 * this structure is used to update context parameters, such as priority of the context
 * backend driver should keep the parameter unchanged if there no new
 * parameter updated.
 */
typedef struct _VAContextParameterUpdateBuffer {
    union {
        struct {
            /** \brief indicate whether context priority changed */
            uint32_t context_priority_update : 1;
            /** \brief Reserved bits for future use, must be zero */
            uint32_t reserved                : 31;
        } bits;
        uint32_t value;
    } flags;
    /** \brief task/context priority */
    VAConfigAttribValContextPriority context_priority;
    /** \brief Reserved bytes for future use, must be zero */
    uint32_t reserved[VA_PADDING_MEDIUM];
} VAContextParameterUpdateBuffer;

/**
 * These ENCRYPTION_TYPEs are used for the attribute values for
 * \c VAConfigAttribEncryption and for encryption_type in
 * VAEncryptionParameters.
 *
 * When used for \c VAConfigAttribEncryption, it be used via
 * vaQueryConfigEntrypoints to check which type are supported for specific
 * profile or not.
 *
 * When used for encryption_type in VAEncryptionParameters, it tells driver
 * the parameters in VAEncryptionParameters are used for which encryption type.
 */
#define VA_ENCRYPTION_TYPE_FULLSAMPLE_CTR       0x00000001  /* AES CTR fullsample */
#define VA_ENCRYPTION_TYPE_FULLSAMPLE_CBC       0x00000002  /* AES CBC fullsample */
#define VA_ENCRYPTION_TYPE_SUBSAMPLE_CTR        0x00000004  /* AES CTR fullsample */
#define VA_ENCRYPTION_TYPE_SUBSAMPLE_CBC        0x00000008  /* AES CBC fullsample */

/** \brief structure for encrypted segment info. */
typedef struct _VAEncryptionSegmentInfo {
    /** \brief  The offset relative to the start of the bitstream input in
     *  bytes of the start of the segment */
    uint32_t segment_start_offset;
    /** \brief  The length of the segments in bytes */
    uint32_t segment_length;
    /** \brief  The length in bytes of the remainder of an incomplete block
     *  from a previous segment*/
    uint32_t partial_aes_block_size;
    /** \brief  The length in bytes of the initial clear data */
    uint32_t init_byte_length;
    /** \brief  This will be AES counter for secure decode and secure encode
     *  when numSegments equals 1, valid size is specified by
     * \c key_blob_size */
    uint8_t aes_cbc_iv_or_ctr[64];
    /** \brief Reserved bytes for future use, must be zero */
    uint32_t va_reserved[VA_PADDING_MEDIUM];
} VAEncryptionSegmentInfo;

/** \brief Encryption parameters buffer for VAEncryptionParameterBufferType */
typedef struct _VAEncryptionParameters {
    /** \brief Encryption type, refer to \c VA_ENCRYPTION_TYPE_FULLSAMPLE_CTR,
     * \c VA_ENCRYPTION_TYPE_FULLSAMPLE_CBC, \c VA_ENCRYPTION_TYPE_SUBSAMPLE_CTR,
     * or \c VA_ENCRYPTION_TYPE_SUBSAMPLE_CBC */
    uint32_t encryption_type;
    /** \brief The number of sengments */
    uint32_t num_segments;
    /** \brief Pointer of segments */
    VAEncryptionSegmentInfo *segment_info;
    /** \brief The status report index reserved for CENC fullsample workload.
     * The related structures and definitions are vendor specific.
    */
    uint32_t status_report_index;
    /** \brief CENC counter length */
    uint32_t size_of_length;
    /** \brief Wrapped decrypt blob (Snd)kb, valid size is specified by
     * \c key_blob_size */
    uint8_t wrapped_decrypt_blob[64];
    /** \brief Wrapped Key blob info (Sne)kb, valid size is specified by
     * \c key_blob_size */
    uint8_t wrapped_encrypt_blob[64];
    /** \brief key blob size
     * It could be \c VA_PC_BLOCK_SIZE_128, \c VA_PC_BLOCK_SIZE_192, or
     * \c VA_PC_BLOCK_SIZE_256
     */
    uint32_t key_blob_size;
    /** \brief Indicates the number of 16-byte BLOCKS that are encrypted in any
     *  given encrypted region of segments.
     *  If this value is zero:
     *    1. All bytes in encrypted region of segments are encrypted, i.e. the
     *       CENC or CBC1 scheme is being used
     *    2. blocks_stripe_clear must also be zero.
     *  If this value is non-zero, blocks_stripe_clear must also be non-zero.
     */
    uint32_t blocks_stripe_encrypted;
    /** \brief Indicates the number of 16-byte BLOCKS that are clear in any given
     *  encrypted region of segments, as defined by the CENS and CBCS schemes in
     *  the common encryption spec.
     *  If this value is zero, all bytes in encrypted region of segments are
     *  encrypted, i.e. the CENC or CBC1 scheme is being used.
     */
    uint32_t blocks_stripe_clear;
    /** \brief Reserved bytes for future use, must be zero */
    uint32_t va_reserved[VA_PADDING_MEDIUM];
} VAEncryptionParameters;

/**
 * Processing rate parameter for encode.
 */
typedef struct _VAProcessingRateParameterEnc {
    /** \brief Profile level */
    uint8_t         level_idc;
    uint8_t         reserved[3];
    /** \brief quality level. When set to 0, default quality
     * level is used.
     */
    uint32_t        quality_level;
    /** \brief Period between I frames. */
    uint32_t        intra_period;
    /** \brief Period between I/P frames. */
    uint32_t        ip_period;
} VAProcessingRateParameterEnc;

/**
 * Processing rate parameter for decode.
 */
typedef struct _VAProcessingRateParameterDec {
    /** \brief Profile level */
    uint8_t         level_idc;
    uint8_t         reserved0[3];
    uint32_t        reserved;
} VAProcessingRateParameterDec;

typedef struct _VAProcessingRateParameter {
    union {
        VAProcessingRateParameterEnc proc_buf_enc;
        VAProcessingRateParameterDec proc_buf_dec;
    };
} VAProcessingRateParameter;

/**
 * \brief Queries processing rate for the supplied config.
 *
 * This function queries the processing rate based on parameters in
 * \c proc_buf for the given \c config. Upon successful return, the processing
 * rate value will be stored in \c processing_rate. Processing rate is
 * specified as the number of macroblocks/CTU per second.
 *
 * If NULL is passed to the \c proc_buf, the default processing rate for the
 * given configuration will be returned.
 *
 * @param[in] dpy               the VA display
 * @param[in] config            the config identifying a codec or a video
 *     processing pipeline
 * @param[in] proc_buf       the buffer that contains the parameters for
        either the encode or decode processing rate
 * @param[out] processing_rate  processing rate in number of macroblocks per
        second constrained by parameters specified in proc_buf
 *
 */
VAStatus
vaQueryProcessingRate(
    VADisplay           dpy,
    VAConfigID          config,
    VAProcessingRateParameter *proc_buf,
    unsigned int       *processing_rate
);

typedef enum {
    VAEncMiscParameterTypeFrameRate     = 0,
    VAEncMiscParameterTypeRateControl   = 1,
    VAEncMiscParameterTypeMaxSliceSize  = 2,
    VAEncMiscParameterTypeAIR       = 3,
    /** \brief Buffer type used to express a maximum frame size (in bits). */
    VAEncMiscParameterTypeMaxFrameSize  = 4,
    /** \brief Buffer type used for HRD parameters. */
    VAEncMiscParameterTypeHRD           = 5,
    VAEncMiscParameterTypeQualityLevel  = 6,
    /** \brief Buffer type used for Rolling intra refresh */
    VAEncMiscParameterTypeRIR           = 7,
    /** \brief Buffer type used for quantization parameters, it's per-sequence parameter*/
    VAEncMiscParameterTypeQuantization  = 8,
    /** \brief Buffer type used for sending skip frame parameters to the encoder's
      * rate control, when the user has externally skipped frames. */
    VAEncMiscParameterTypeSkipFrame     = 9,
    /** \brief Buffer type used for region-of-interest (ROI) parameters. */
    VAEncMiscParameterTypeROI           = 10,
    /** \brief Buffer type used to express a maximum frame size (in bytes) settings for multiple pass. */
    VAEncMiscParameterTypeMultiPassFrameSize       = 11,
    /** \brief Buffer type used for temporal layer structure */
    VAEncMiscParameterTypeTemporalLayerStructure   = 12,
    /** \brief Buffer type used for dirty region-of-interest (ROI) parameters. */
    VAEncMiscParameterTypeDirtyRect      = 13,
    /** \brief Buffer type used for parallel BRC parameters. */
    VAEncMiscParameterTypeParallelBRC   = 14,
    /** \brief Set MB partion mode mask and Half-pel/Quant-pel motion search */
    VAEncMiscParameterTypeSubMbPartPel = 15,
    /** \brief set encode quality tuning */
    VAEncMiscParameterTypeEncQuality = 16,
    /** \brief Buffer type used for encoder rounding offset parameters. */
    VAEncMiscParameterTypeCustomRoundingControl = 17,
    /** \brief Buffer type used for FEI input frame level parameters */
    VAEncMiscParameterTypeFEIFrameControl = 18,
    /** \brief encode extension buffer, ect. MPEG2 Sequence extenstion data */
    VAEncMiscParameterTypeExtensionData = 19
} VAEncMiscParameterType;

/** \brief Packed header type. */
typedef enum {
    /** \brief Packed sequence header. */
    VAEncPackedHeaderSequence   = 1,
    /** \brief Packed picture header. */
    VAEncPackedHeaderPicture    = 2,
    /** \brief Packed slice header. */
    VAEncPackedHeaderSlice      = 3,
    /**
     * \brief Packed raw header.
     *
     * Packed raw data header can be used by the client to insert a header
     * into the bitstream data buffer at the point it is passed, the driver
     * will handle the raw packed header based on "has_emulation_bytes" field
     * in the packed header parameter structure.
     */
    VAEncPackedHeaderRawData    = 4,
    /**
     * \brief Misc packed header. See codec-specific definitions.
     *
     * @deprecated
     * This is a deprecated packed header type. All applications can use
     * \c VAEncPackedHeaderRawData to insert a codec-specific packed header
     */
    VAEncPackedHeaderMiscMask va_deprecated_enum  = 0x80000000,
} VAEncPackedHeaderType;

/** \brief Packed header parameter. */
typedef struct _VAEncPackedHeaderParameterBuffer {
    /** Type of the packed header buffer. See #VAEncPackedHeaderType. */
    uint32_t                type;
    /** \brief Size of the #VAEncPackedHeaderDataBuffer in bits. */
    uint32_t                bit_length;
    /** \brief Flag: buffer contains start code emulation prevention bytes? */
    uint8_t               has_emulation_bytes;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAEncPackedHeaderParameterBuffer;

/**
 *  For application, e.g. set a new bitrate
 *    VABufferID buf_id;
 *    VAEncMiscParameterBuffer *misc_param;
 *    VAEncMiscParameterRateControl *misc_rate_ctrl;
 *
 *    vaCreateBuffer(dpy, context, VAEncMiscParameterBufferType,
 *              sizeof(VAEncMiscParameterBuffer) + sizeof(VAEncMiscParameterRateControl),
 *              1, NULL, &buf_id);
 *
 *    vaMapBuffer(dpy,buf_id,(void **)&misc_param);
 *    misc_param->type = VAEncMiscParameterTypeRateControl;
 *    misc_rate_ctrl= (VAEncMiscParameterRateControl *)misc_param->data;
 *    misc_rate_ctrl->bits_per_second = 6400000;
 *    vaUnmapBuffer(dpy, buf_id);
 *    vaRenderPicture(dpy, context, &buf_id, 1);
 */
typedef struct _VAEncMiscParameterBuffer {
    VAEncMiscParameterType type;
    uint32_t data[];
} VAEncMiscParameterBuffer;

/** \brief Temporal layer Structure*/
typedef struct _VAEncMiscParameterTemporalLayerStructure {
    /** \brief The number of temporal layers */
    uint32_t number_of_layers;
    /** \brief The length of the array defining frame layer membership. Should be 1-32 */
    uint32_t periodicity;
    /**
     * \brief The array indicating the layer id for each frame
     *
     * The layer id for the first frame in a coded sequence is always 0, so layer_id[] specifies the layer
     * ids for frames starting from the 2nd frame.
     */
    uint32_t layer_id[32];

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAEncMiscParameterTemporalLayerStructure;


/** \brief Rate control parameters */
typedef struct _VAEncMiscParameterRateControl {
    /** The maximum bit-rate which the the rate controller should generate. */
    uint32_t bits_per_second;
    /** The target bit-rate which the rate controller should generate, as a percentage of the
     * maximum bit-rate.
     *
     * In CBR mode this value is ignored (treated as 100%).
     */
    uint32_t target_percentage;
    /** Rate control window size in milliseconds.
     *
     * The rate controller will attempt to guarantee that the target and maximum bit-rates are
     * correct over this window.
     */
    uint32_t window_size;
    /** Initial quantiser value used at the start of the stream.
     *
     * Ignored if set to zero.
     */
    uint32_t initial_qp;
    /** Minimum quantiser value to use.
     *
     * The quantiser will not go below the value - if this limit is hit, the output bitrate may
     * be lower than the target.  Ignored if set to zero.
     */
    uint32_t min_qp;
    /** Basic unit size.
     *
     * Only used by some drivers - see driver documentation for details.  Set to zero if unused.
     */
    uint32_t basic_unit_size;
    union {
        struct {
            /** Force rate controller reset.
             *
             * The next frame will be treated as the start of a new stream, with all rate
             * controller state reset to its initial values.
             */
            uint32_t reset : 1;
            /** Disable frame skip in rate control mode. */
            uint32_t disable_frame_skip : 1;
            /** Disable bit stuffing in rate control mode. */
            uint32_t disable_bit_stuffing : 1;
            /** Macroblock-level rate control.
             *
             * 0: use default, 1: always enable, 2: always disable, other: reserved.
             *
             * This feature is only available if VAConfigAttribRateControl has the
             * \ref VA_RC_MB bit set.
             */
            uint32_t mb_rate_control : 4;
            /** The temporal layer that these rate control parameters apply to. */
            uint32_t temporal_id : 8;
            /** Ensure that intra frames also conform to the constant frame size. */
            uint32_t cfs_I_frames : 1;
            /** Enable parallel rate control for hierarchical B frames.
             *
             * See \ref VA_RC_PARALLEL.
             */
            uint32_t enable_parallel_brc    : 1;
            uint32_t enable_dynamic_scaling : 1;
            /** Frame tolerance mode.
             *
             *  Indicates the tolerance the application has to variations in the frame size.
             *  For example, wireless display scenarios may require very steady bit rate to
             *  reduce buffering time. It affects the rate control algorithm used,
             *  but may or may not have an effect based on the combination of other BRC
             *  parameters.  Only valid when the driver reports support for
             *  #VAConfigAttribFrameSizeToleranceSupport.
             *
             *  equals 0    -- normal mode;
             *  equals 1    -- maps to sliding window;
             *  equals 2    -- maps to low delay mode;
             *  other       -- invalid.
             */
            uint32_t frame_tolerance_mode   : 2;
            /** Reserved for future use, must be zero. */
            uint32_t reserved               : 12;
        } bits;
        uint32_t value;
    } rc_flags;
    /** Initial quality factor used in ICQ mode.
     *
     * This value must be between 1 and 51.
     * this value will be deprecated in future, to use quality_factor instead of it.
     */
    uint32_t ICQ_quality_factor;
    /** Maximum quantiser value to use.
     *
     * The quantiser will not go above this value - if this limit is hit, the output bitrate
     * may exceed the target.  Ignored if set to zero.
     */
    uint32_t max_qp;
    /** Quality factor
     *
     *  the range will be different for different codec
     */
    uint32_t quality_factor;
    /** Target frame size
     *
     *  Desired frame size in bytes.
     *  This parameter can be used in some RC modes (like Transport Controlled BRC)
     *  where feedback from the app is required.
     *  Zero value means no limits.
     *
     */
    uint32_t target_frame_size;
    /** Reserved bytes for future use, must be zero. */
    uint32_t va_reserved[VA_PADDING_LOW];
} VAEncMiscParameterRateControl;

/** Encode framerate parameters.
 *
 * Sets the encode framerate used by the rate controller.  This should be
 * provided in all modes using a bitrate target (variable framerate is not
 * supported).
 */
typedef struct _VAEncMiscParameterFrameRate {
    /** Encode framerate.
     *
     * The framerate is specified as a number of frames per second, as a
     * fraction.  The denominator of the fraction is given in the top half
     * (the high two bytes) of the framerate field, and the numerator is
     * given in the bottom half (the low two bytes).
     *
     * That is:
     * denominator = framerate >> 16 & 0xffff;
     * numerator   = framerate & 0xffff;
     * fps         = numerator / denominator;
     *
     * For example, if framerate is set to (100 << 16 | 750), this is
     * 750 / 100, hence 7.5fps.
     *
     * If the denominator is zero (the high two bytes are both zero) then
     * it takes the value one instead, so the framerate is just the integer
     * in the low 2 bytes.
     */
    uint32_t framerate;
    union {
        struct {
            /** The temporal layer that these framerate parameters apply to. */
            uint32_t temporal_id : 8;
            /** Reserved for future use, must be zero. */
            uint32_t reserved : 24;
        } bits;
        uint32_t value;
    } framerate_flags;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAEncMiscParameterFrameRate;

/**
 * Allow a maximum slice size to be specified (in bits).
 * The encoder will attempt to make sure that individual slices do not exceed this size
 * Or to signal applicate if the slice size exceed this size, see "status" of VACodedBufferSegment
 */
typedef struct _VAEncMiscParameterMaxSliceSize {
    uint32_t max_slice_size;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAEncMiscParameterMaxSliceSize;

typedef struct _VAEncMiscParameterAIR {
    uint32_t air_num_mbs;
    uint32_t air_threshold;
    uint32_t air_auto; /* if set to 1 then hardware auto-tune the AIR threshold */

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAEncMiscParameterAIR;

/*
 * \brief Rolling intra refresh data structure for encoding.
 */
typedef struct _VAEncMiscParameterRIR {
    union {
        struct
        /**
         * \brief Indicate if intra refresh is enabled in column/row.
         *
         * App should query VAConfigAttribEncIntraRefresh to confirm RIR support
         * by the driver before sending this structure.
             */
        {
            /* \brief enable RIR in column */
            uint32_t enable_rir_column : 1;
            /* \brief enable RIR in row */
            uint32_t enable_rir_row : 1;
            uint32_t reserved : 30;
        } bits;
        uint32_t value;
    } rir_flags;
    /**
     * \brief Indicates the column or row location in MB. It is ignored if
     * rir_flags is 0.
     */
    uint16_t intra_insertion_location;
    /**
     * \brief Indicates the number of columns or rows in MB. It is ignored if
     * rir_flags is 0.
     */
    uint16_t intra_insert_size;
    /**
     * \brief indicates the Qp difference for inserted intra columns or rows.
     * App can use this to adjust intra Qp based on bitrate & max frame size.
     */
    uint8_t  qp_delta_for_inserted_intra;
    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAEncMiscParameterRIR;

/** HRD / VBV buffering parameters for encoding.
 *
 * This sets the HRD / VBV parameters which will be used by the rate
 * controller for encoding.  It should be specified in modes using a bitrate
 * target when the buffering of the output stream needs to be constrained.
 *
 * If not provided, the encoder may use arbitrary amounts of buffering.
 */
typedef struct _VAEncMiscParameterHRD {
    /** The initial fullness of the HRD coded picture buffer, in bits.
     *
     * This sets how full the CPB is when encoding begins - that is, how much
     * buffering will happen on the decoder side before the first frame.
     * The CPB fullness will be reset to this value after any rate control
     * reset (a change in parameters or an explicit reset).
     *
     * For H.264, it should match the value of initial_cpb_removal_delay in
     * buffering_period SEI messages.
     */
    uint32_t initial_buffer_fullness;
    /** The HRD coded picture buffer size, in bits.
     *
     * For H.264, it should match the value of cpb_size_value_minus1 in the VUI
     * parameters.
     */
    uint32_t buffer_size;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAEncMiscParameterHRD;

/**
 * \brief Defines a maximum frame size (in bits).
 *
 * This misc parameter buffer defines the maximum size of a frame (in
 * bits). The encoder will try to make sure that each frame does not
 * exceed this size. Otherwise, if the frame size exceeds this size,
 * the \c status flag of #VACodedBufferSegment will contain
 * #VA_CODED_BUF_STATUS_FRAME_SIZE_OVERFLOW.
 */
typedef struct _VAEncMiscParameterBufferMaxFrameSize {
    /** \brief Type. Shall be set to #VAEncMiscParameterTypeMaxFrameSize. */
    /** duplicated with VAEncMiscParameterBuffer, should be deprecated*/
    va_deprecated VAEncMiscParameterType      type;
    /** \brief Maximum size of a frame (in bits). */
    uint32_t                max_frame_size;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAEncMiscParameterBufferMaxFrameSize;

/**
 * \brief Maximum frame size (in bytes) settings for multiple pass.
 *
 * This misc parameter buffer defines the maximum size of a frame (in
 * bytes) settings for multiple pass. currently only AVC encoder can
 * support this settings in multiple pass case. If the frame size exceeds
 * this size, the encoder will do more pak passes to adjust the QP value
 * to control the frame size.
 */
typedef struct _VAEncMiscParameterBufferMultiPassFrameSize {
    /** \brief Type. Shall be set to #VAEncMiscParameterTypeMultiPassMaxFrameSize. */
    /** duplicated with VAEncMiscParameterBuffer, should be deprecated*/
    va_deprecated VAEncMiscParameterType      type;
    /** \brief Maximum size of a frame (in byte) */
    uint32_t                max_frame_size;
    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                reserved;
    /** \brief number of passes, every pass has different QP, currently AVC encoder can support up to 4 passes */
    uint8_t                 num_passes;
    /** \brief delta QP list for every pass */
    uint8_t                *delta_qp;

    /** \brief Reserved bytes for future use, must be zero */
    unsigned long           va_reserved[VA_PADDING_LOW];
} VAEncMiscParameterBufferMultiPassFrameSize;

/**
 * \brief Encoding quality level.
 *
 * The encoding quality could be set through this structure, if the implementation
 * supports multiple quality levels. The quality level set through this structure is
 * persistent over the entire coded sequence, or until a new structure is being sent.
 * The quality level range can be queried through the VAConfigAttribEncQualityRange
 * attribute. A lower value means higher quality, and a value of 1 represents the highest
 * quality. The quality level setting is used as a trade-off between quality and speed/power
 * consumption, with higher quality corresponds to lower speed and higher power consumption.
 */
typedef struct _VAEncMiscParameterBufferQualityLevel {
    /** \brief Encoding quality level setting. When set to 0, default quality
     * level is used.
     */
    uint32_t                quality_level;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAEncMiscParameterBufferQualityLevel;

/**
 * \brief Quantization settings for encoding.
 *
 * Some encoders support special types of quantization such as trellis, and this structure
 * can be used by the app to control these special types of quantization by the encoder.
 */
typedef struct _VAEncMiscParameterQuantization {
    union {
        /* if no flags is set then quantization is determined by the driver */
        struct {
            /* \brief disable trellis for all frames/fields */
            uint32_t disable_trellis : 1;
            /* \brief enable trellis for I frames/fields */
            uint32_t enable_trellis_I : 1;
            /* \brief enable trellis for P frames/fields */
            uint32_t enable_trellis_P : 1;
            /* \brief enable trellis for B frames/fields */
            uint32_t enable_trellis_B : 1;
            uint32_t reserved : 28;
        } bits;
        uint32_t value;
    } quantization_flags;
    uint32_t va_reserved;
} VAEncMiscParameterQuantization;

/**
 * \brief Encoding skip frame.
 *
 * The application may choose to skip frames externally to the encoder (e.g. drop completely or
 * code as all skip's). For rate control purposes the encoder will need to know the size and number
 * of skipped frames.  Skip frame(s) indicated through this structure is applicable only to the
 * current frame.  It is allowed for the application to still send in packed headers for the driver to
 * pack, although no frame will be encoded (e.g. for HW to encrypt the frame).
 */
typedef struct _VAEncMiscParameterSkipFrame {
    /** \brief Indicates skip frames as below.
      * 0: Encode as normal, no skip.
      * 1: One or more frames were skipped prior to the current frame, encode the current frame as normal.
      * 2: The current frame is to be skipped, do not encode it but pack/encrypt the packed header contents
      *    (all except VAEncPackedHeaderSlice) which could contain actual frame contents (e.g. pack the frame
      *    in VAEncPackedHeaderPicture).  */
    uint8_t               skip_frame_flag;
    /** \brief The number of frames skipped prior to the current frame.  Valid when skip_frame_flag = 1. */
    uint8_t               num_skip_frames;
    /** \brief When skip_frame_flag = 1, the size of the skipped frames in bits.   When skip_frame_flag = 2,
      * the size of the current skipped frame that is to be packed/encrypted in bits. */
    uint32_t                size_skip_frames;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAEncMiscParameterSkipFrame;

/**
 * \brief Encoding region-of-interest (ROI).
 *
 * The encoding ROI can be set through VAEncMiscParameterBufferROI, if the implementation
 * supports ROI input. The ROI set through this structure is applicable only to the
 * current frame or field, so must be sent every frame or field to be applied.  The number of
 * supported ROIs can be queried through the VAConfigAttribEncROI.  The encoder will use the
 * ROI information to adjust the QP values of the MB's that fall within the ROIs.
 */
typedef struct _VAEncROI {
    /** \brief Defines the ROI boundary in pixels, the driver will map it to appropriate
     *  codec coding units.  It is relative to frame coordinates for the frame case and
     *  to field coordinates for the field case. */
    VARectangle     roi_rectangle;
    /**
     * \brief ROI value
     *
     * \ref roi_value specifies ROI delta QP or ROI priority.
     * --  ROI delta QP is the value that will be added on top of the frame level QP.
     * --  ROI priority specifies the priority of a region, it can be positive (more important)
     * or negative (less important) values and is compared with non-ROI region (taken as value 0),
     * E.g. ROI region with \ref roi_value -3 is less important than the non-ROI region (\ref roi_value
     * implied to be 0) which is less important than ROI region with roi_value +2. For overlapping
     * regions, the roi_value that is first in the ROI array will have priority.
     *
     * \ref roi_value always specifes ROI delta QP when VAConfigAttribRateControl == VA_RC_CQP, no matter
     * the value of \c roi_value_is_qp_delta in #VAEncMiscParameterBufferROI.
     *
     * \ref roi_value depends on \c roi_value_is_qp_delta in #VAEncMiscParameterBufferROI when
     * VAConfigAttribRateControl != VA_RC_CQP. \ref roi_value specifies ROI_delta QP if \c roi_value_is_qp_delta
     * in VAEncMiscParameterBufferROI is 1, otherwise \ref roi_value specifies ROI priority.
     */
    int8_t            roi_value;
} VAEncROI;

typedef struct _VAEncMiscParameterBufferROI {
    /** \brief Number of ROIs being sent.*/
    uint32_t        num_roi;

    /** \brief Valid when VAConfigAttribRateControl != VA_RC_CQP, then the encoder's
     *  rate control will determine actual delta QPs.  Specifies the max/min allowed delta
     *  QPs. */
    int8_t                max_delta_qp;
    int8_t                min_delta_qp;

    /** \brief Pointer to a VAEncROI array with num_roi elements.  It is relative to frame
      *  coordinates for the frame case and to field coordinates for the field case.*/
    VAEncROI            *roi;
    union {
        struct {
            /**
             * \brief An indication for roi value.
             *
             * \ref roi_value_is_qp_delta equal to 1 indicates \c roi_value in #VAEncROI should
             * be used as ROI delta QP. \ref roi_value_is_qp_delta equal to 0 indicates \c roi_value
             * in #VAEncROI should be used as ROI priority.
             *
             * \ref roi_value_is_qp_delta is only available when VAConfigAttribRateControl != VA_RC_CQP,
             * the setting must comply with \c roi_rc_priority_support and \c roi_rc_qp_delta_support in
             * #VAConfigAttribValEncROI. The underlying driver should ignore this field
             * when VAConfigAttribRateControl == VA_RC_CQP.
             */
            uint32_t  roi_value_is_qp_delta    : 1;
            uint32_t  reserved                 : 31;
        } bits;
        uint32_t value;
    } roi_flags;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAEncMiscParameterBufferROI;
/*
 * \brief Dirty rectangle data structure for encoding.
 *
 * The encoding dirty rect can be set through VAEncMiscParameterBufferDirtyRect, if the
 * implementation supports dirty rect input. The rect set through this structure is applicable
 * only to the current frame or field, so must be sent every frame or field to be applied.
 * The number of supported rects can be queried through the VAConfigAttribEncDirtyRect.  The
 * encoder will use the rect information to know those rectangle areas have changed while the
 * areas not covered by dirty rect rectangles are assumed to have not changed compared to the
 * previous picture.  The encoder may do some internal optimizations.
 */
typedef struct _VAEncMiscParameterBufferDirtyRect {
    /** \brief Number of Rectangle being sent.*/
    uint32_t    num_roi_rectangle;

    /** \brief Pointer to a VARectangle array with num_roi_rectangle elements.*/
    VARectangle    *roi_rectangle;
} VAEncMiscParameterBufferDirtyRect;

/** \brief Attribute value for VAConfigAttribEncParallelRateControl */
typedef struct _VAEncMiscParameterParallelRateControl {
    /** brief Number of layers*/
    uint32_t num_layers;
    /** brief Number of B frames per layer per GOP.
     *
     * it should be allocated by application, and the is num_layers.
     *  num_b_in_gop[0] is the number of regular B which refers to only I or P frames. */
    uint32_t *num_b_in_gop;
} VAEncMiscParameterParallelRateControl;

/** per frame encoder quality controls, once set they will persist for all future frames
  *till it is updated again. */
typedef struct _VAEncMiscParameterEncQuality {
    union {
        struct {
            /** Use raw frames for reference instead of reconstructed frames.
              * it only impact motion estimation (ME)  stage, and will not impact MC stage
              * so the reconstruct picture will can match with decode side */
            uint32_t useRawPicForRef                    : 1;
            /**  Disables skip check for ME stage, it will increase the bistream size
              * but will improve the qulity */
            uint32_t skipCheckDisable                   : 1;
            /**  Indicates app will override default driver FTQ settings using FTQEnable.
              *  FTQ is forward transform quantization */
            uint32_t FTQOverride                        : 1;
            /** Enables/disables FTQ. */
            uint32_t FTQEnable                          : 1;
            /** Indicates the app will provide the Skip Threshold LUT to use when FTQ is
              * enabled (FTQSkipThresholdLUT), else default driver thresholds will be used. */
            uint32_t FTQSkipThresholdLUTInput           : 1;
            /** Indicates the app will provide the Skip Threshold LUT to use when FTQ is
              * disabled (NonFTQSkipThresholdLUT), else default driver thresholds will be used. */
            uint32_t NonFTQSkipThresholdLUTInput        : 1;
            uint32_t ReservedBit                        : 1;
            /** Control to enable the ME mode decision algorithm to bias to fewer B Direct/Skip types.
              * Applies only to B frames, all other frames will ignore this setting.  */
            uint32_t directBiasAdjustmentEnable         : 1;
            /** Enables global motion bias. global motion also is called HME (Heirarchical Motion Estimation )
              * HME is used to handle large motions and avoiding local minima in the video encoding process
              * down scaled the input and reference picture, then do ME. the result will be a predictor to next level HME or ME
              * current interface divide the HME to 3 level. UltraHME , SuperHME, and HME, result of UltraHME will be input of SurperHME,
              * result of superHME will be a input for HME. HME result will be input of ME. it is a switch for HMEMVCostScalingFactor
              * can change the HME bias inside RDO stage*/
            uint32_t globalMotionBiasAdjustmentEnable   : 1;
            /** MV cost scaling ratio for HME ( predictors.  It is used when
              * globalMotionBiasAdjustmentEnable == 1, else it is ignored.  Values are:
              *     0: set MV cost to be 0 for HME predictor.
              *     1: scale MV cost to be 1/2 of the default value for HME predictor.
              *     2: scale MV cost to be 1/4 of the default value for HME predictor.
              *     3: scale MV cost to be 1/8 of the default value for HME predictor. */
            uint32_t HMEMVCostScalingFactor             : 2;
            /**disable HME, if it is disabled. Super*ultraHME should also be disabled  */
            uint32_t HMEDisable                         : 1;
            /**disable Super HME, if it is disabled, ultraHME should be disabled */
            uint32_t SuperHMEDisable                    : 1;
            /** disable Ultra HME */
            uint32_t UltraHMEDisable                    : 1;
            /** disable panic mode. Panic mode happened when there are extreme BRC (bit rate control) requirement
              * frame size cant achieve the target of BRC.  when Panic mode is triggered, Coefficients will
              *  be set to zero. disable panic mode will improve quality but will impact BRC */
            uint32_t PanicModeDisable                   : 1;
            /** Force RepartitionCheck
             *  0: DEFAULT - follow driver default settings.
             *  1: FORCE_ENABLE - enable this feature totally for all cases.
             *  2: FORCE_DISABLE - disable this feature totally for all cases. */
            uint32_t ForceRepartitionCheck              : 2;

        };
        uint32_t encControls;
    };

    /** Maps QP to skip thresholds when FTQ is enabled.  Valid range is 0-255. */
    uint8_t FTQSkipThresholdLUT[52];
    /** Maps QP to skip thresholds when FTQ is disabled.  Valid range is 0-65535. */
    uint16_t NonFTQSkipThresholdLUT[52];

    uint32_t reserved[VA_PADDING_HIGH];  // Reserved for future use.

} VAEncMiscParameterEncQuality;

/**
 *  \brief Custom Encoder Rounding Offset Control.
 *  Application may use this structure to set customized rounding
 *  offset parameters for quantization.
 *  Valid when \c VAConfigAttribCustomRoundingControl equals 1.
 */
typedef struct _VAEncMiscParameterCustomRoundingControl {
    union {
        struct {
            /** \brief Enable customized rounding offset for intra blocks.
             *  If 0, default value would be taken by driver for intra
             *  rounding offset.
             */
            uint32_t    enable_custom_rouding_intra     : 1 ;

            /** \brief Intra rounding offset
             *  Ignored if \c enable_custom_rouding_intra equals 0.
             */
            uint32_t    rounding_offset_intra           : 7;

            /** \brief Enable customized rounding offset for inter blocks.
             *  If 0, default value would be taken by driver for inter
             *  rounding offset.
             */
            uint32_t    enable_custom_rounding_inter    : 1 ;

            /** \brief Inter rounding offset
             *  Ignored if \c enable_custom_rouding_inter equals 0.
             */
            uint32_t    rounding_offset_inter           : 7;

            /* Reserved */
            uint32_t    reserved                        : 16;
        }  bits;
        uint32_t    value;
    }   rounding_offset_setting;
} VAEncMiscParameterCustomRoundingControl;

/**
 * There will be cases where the bitstream buffer will not have enough room to hold
 * the data for the entire slice, and the following flags will be used in the slice
 * parameter to signal to the server for the possible cases.
 * If a slice parameter buffer and slice data buffer pair is sent to the server with
 * the slice data partially in the slice data buffer (BEGIN and MIDDLE cases below),
 * then a slice parameter and data buffer needs to be sent again to complete this slice.
 */
#define VA_SLICE_DATA_FLAG_ALL      0x00    /* whole slice is in the buffer */
#define VA_SLICE_DATA_FLAG_BEGIN    0x01    /* The beginning of the slice is in the buffer but the end if not */
#define VA_SLICE_DATA_FLAG_MIDDLE   0x02    /* Neither beginning nor end of the slice is in the buffer */
#define VA_SLICE_DATA_FLAG_END      0x04    /* end of the slice is in the buffer */

/* Codec-independent Slice Parameter Buffer base */
typedef struct _VASliceParameterBufferBase {
    uint32_t slice_data_size;   /* number of bytes in the slice data buffer for this slice */
    uint32_t slice_data_offset; /* the offset to the first byte of slice data */
    uint32_t slice_data_flag;   /* see VA_SLICE_DATA_FLAG_XXX definitions */
} VASliceParameterBufferBase;

/**********************************
 * JPEG common  data structures
 **********************************/
/**
 * \brief Huffman table for JPEG decoding.
 *
 * This structure holds the complete Huffman tables. This is an
 * aggregation of all Huffman table (DHT) segments maintained by the
 * application. i.e. up to 2 Huffman tables are stored in there for
 * baseline profile.
 *
 * The #load_huffman_table array can be used as a hint to notify the
 * VA driver implementation about which table(s) actually changed
 * since the last submission of this buffer.
 */
typedef struct _VAHuffmanTableBufferJPEGBaseline {
    /** \brief Specifies which #huffman_table is valid. */
    uint8_t       load_huffman_table[2];
    /** \brief Huffman tables indexed by table identifier (Th). */
    struct {
        /** @name DC table (up to 12 categories) */
        /**@{*/
        /** \brief Number of Huffman codes of length i + 1 (Li). */
        uint8_t   num_dc_codes[16];
        /** \brief Value associated with each Huffman code (Vij). */
        uint8_t   dc_values[12];
        /**@}*/
        /** @name AC table (2 special codes + up to 16 * 10 codes) */
        /**@{*/
        /** \brief Number of Huffman codes of length i + 1 (Li). */
        uint8_t   num_ac_codes[16];
        /** \brief Value associated with each Huffman code (Vij). */
        uint8_t   ac_values[162];
        /** \brief Padding to 4-byte boundaries. Must be set to zero. */
        uint8_t   pad[2];
        /**@}*/
    }                   huffman_table[2];

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAHuffmanTableBufferJPEGBaseline;

/****************************
 * MPEG-2 data structures
 ****************************/

/* MPEG-2 Picture Parameter Buffer */
/*
 * For each frame or field, and before any slice data, a single
 * picture parameter buffer must be send.
 */
typedef struct _VAPictureParameterBufferMPEG2 {
    uint16_t horizontal_size;
    uint16_t vertical_size;
    VASurfaceID forward_reference_picture;
    VASurfaceID backward_reference_picture;
    /* meanings of the following fields are the same as in the standard */
    int32_t picture_coding_type;
    int32_t f_code; /* pack all four fcode into this */
    union {
        struct {
            uint32_t intra_dc_precision     : 2;
            uint32_t picture_structure      : 2;
            uint32_t top_field_first        : 1;
            uint32_t frame_pred_frame_dct       : 1;
            uint32_t concealment_motion_vectors : 1;
            uint32_t q_scale_type           : 1;
            uint32_t intra_vlc_format       : 1;
            uint32_t alternate_scan         : 1;
            uint32_t repeat_first_field     : 1;
            uint32_t progressive_frame      : 1;
            uint32_t is_first_field         : 1; /* indicate whether the current field
                                                              * is the first field for field picture
                                                              */
        } bits;
        uint32_t value;
    } picture_coding_extension;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAPictureParameterBufferMPEG2;

/** MPEG-2 Inverse Quantization Matrix Buffer */
typedef struct _VAIQMatrixBufferMPEG2 {
    /** \brief Same as the MPEG-2 bitstream syntax element. */
    int32_t load_intra_quantiser_matrix;
    /** \brief Same as the MPEG-2 bitstream syntax element. */
    int32_t load_non_intra_quantiser_matrix;
    /** \brief Same as the MPEG-2 bitstream syntax element. */
    int32_t load_chroma_intra_quantiser_matrix;
    /** \brief Same as the MPEG-2 bitstream syntax element. */
    int32_t load_chroma_non_intra_quantiser_matrix;
    /** \brief Luminance intra matrix, in zig-zag scan order. */
    uint8_t intra_quantiser_matrix[64];
    /** \brief Luminance non-intra matrix, in zig-zag scan order. */
    uint8_t non_intra_quantiser_matrix[64];
    /** \brief Chroma intra matrix, in zig-zag scan order. */
    uint8_t chroma_intra_quantiser_matrix[64];
    /** \brief Chroma non-intra matrix, in zig-zag scan order. */
    uint8_t chroma_non_intra_quantiser_matrix[64];

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAIQMatrixBufferMPEG2;

/** MPEG-2 Slice Parameter Buffer */
typedef struct _VASliceParameterBufferMPEG2 {
    uint32_t slice_data_size;/* number of bytes in the slice data buffer for this slice */
    uint32_t slice_data_offset;/* the offset to the first byte of slice data */
    uint32_t slice_data_flag; /* see VA_SLICE_DATA_FLAG_XXX defintions */
    uint32_t macroblock_offset;/* the offset to the first bit of MB from the first byte of slice data */
    uint32_t slice_horizontal_position;
    uint32_t slice_vertical_position;
    int32_t quantiser_scale_code;
    int32_t intra_slice_flag;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VASliceParameterBufferMPEG2;

/** MPEG-2 Macroblock Parameter Buffer */
typedef struct _VAMacroblockParameterBufferMPEG2 {
    uint16_t macroblock_address;
    /*
     * macroblock_address (in raster scan order)
     * top-left: 0
     * bottom-right: picture-height-in-mb*picture-width-in-mb - 1
     */
    uint8_t macroblock_type;  /* see definition below */
    union {
        struct {
            uint32_t frame_motion_type      : 2;
            uint32_t field_motion_type      : 2;
            uint32_t dct_type           : 1;
        } bits;
        uint32_t value;
    } macroblock_modes;
    uint8_t motion_vertical_field_select;
    /*
     * motion_vertical_field_select:
     * see section 6.3.17.2 in the spec
     * only the lower 4 bits are used
     * bit 0: first vector forward
     * bit 1: first vector backward
     * bit 2: second vector forward
     * bit 3: second vector backward
     */
    int16_t PMV[2][2][2]; /* see Table 7-7 in the spec */
    uint16_t coded_block_pattern;
    /*
     * The bitplanes for coded_block_pattern are described
     * in Figure 6.10-12 in the spec
     */

    /* Number of skipped macroblocks after this macroblock */
    uint16_t num_skipped_macroblocks;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAMacroblockParameterBufferMPEG2;

/*
 * OR'd flags for macroblock_type (section 6.3.17.1 in the spec)
 */
#define VA_MB_TYPE_MOTION_FORWARD   0x02
#define VA_MB_TYPE_MOTION_BACKWARD  0x04
#define VA_MB_TYPE_MOTION_PATTERN   0x08
#define VA_MB_TYPE_MOTION_INTRA     0x10

/**
 * MPEG-2 Residual Data Buffer
 * For each macroblock, there wil be 64 shorts (16-bit) in the
 * residual data buffer
 */

/****************************
 * MPEG-4 Part 2 data structures
 ****************************/

/* MPEG-4 Picture Parameter Buffer */
/*
 * For each frame or field, and before any slice data, a single
 * picture parameter buffer must be send.
 */
typedef struct _VAPictureParameterBufferMPEG4 {
    uint16_t vop_width;
    uint16_t vop_height;
    VASurfaceID forward_reference_picture;
    VASurfaceID backward_reference_picture;
    union {
        struct {
            uint32_t short_video_header     : 1;
            uint32_t chroma_format          : 2;
            uint32_t interlaced         : 1;
            uint32_t obmc_disable           : 1;
            uint32_t sprite_enable          : 2;
            uint32_t sprite_warping_accuracy    : 2;
            uint32_t quant_type         : 1;
            uint32_t quarter_sample         : 1;
            uint32_t data_partitioned       : 1;
            uint32_t reversible_vlc         : 1;
            uint32_t resync_marker_disable      : 1;
        } bits;
        uint32_t value;
    } vol_fields;
    uint8_t no_of_sprite_warping_points;
    int16_t sprite_trajectory_du[3];
    int16_t sprite_trajectory_dv[3];
    uint8_t quant_precision;
    union {
        struct {
            uint32_t vop_coding_type        : 2;
            uint32_t backward_reference_vop_coding_type : 2;
            uint32_t vop_rounding_type      : 1;
            uint32_t intra_dc_vlc_thr       : 3;
            uint32_t top_field_first        : 1;
            uint32_t alternate_vertical_scan_flag   : 1;
        } bits;
        uint32_t value;
    } vop_fields;
    uint8_t vop_fcode_forward;
    uint8_t vop_fcode_backward;
    uint16_t vop_time_increment_resolution;
    /* short header related */
    uint8_t num_gobs_in_vop;
    uint8_t num_macroblocks_in_gob;
    /* for direct mode prediction */
    int16_t TRB;
    int16_t TRD;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAPictureParameterBufferMPEG4;

/** MPEG-4 Inverse Quantization Matrix Buffer */
typedef struct _VAIQMatrixBufferMPEG4 {
    /** Same as the MPEG-4:2 bitstream syntax element. */
    int32_t load_intra_quant_mat;
    /** Same as the MPEG-4:2 bitstream syntax element. */
    int32_t load_non_intra_quant_mat;
    /** The matrix for intra blocks, in zig-zag scan order. */
    uint8_t intra_quant_mat[64];
    /** The matrix for non-intra blocks, in zig-zag scan order. */
    uint8_t non_intra_quant_mat[64];

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAIQMatrixBufferMPEG4;

/** MPEG-4 Slice Parameter Buffer */
typedef struct _VASliceParameterBufferMPEG4 {
    uint32_t slice_data_size;/* number of bytes in the slice data buffer for this slice */
    uint32_t slice_data_offset;/* the offset to the first byte of slice data */
    uint32_t slice_data_flag; /* see VA_SLICE_DATA_FLAG_XXX defintions */
    uint32_t macroblock_offset;/* the offset to the first bit of MB from the first byte of slice data */
    uint32_t macroblock_number;
    int32_t quant_scale;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VASliceParameterBufferMPEG4;

/**
 VC-1 data structures
*/

typedef enum   /* see 7.1.1.32 */
{
    VAMvMode1Mv                        = 0,
    VAMvMode1MvHalfPel                 = 1,
    VAMvMode1MvHalfPelBilinear         = 2,
    VAMvModeMixedMv                    = 3,
    VAMvModeIntensityCompensation      = 4
} VAMvModeVC1;

/** VC-1 Picture Parameter Buffer */
/*
 * For each picture, and before any slice data, a picture parameter
 * buffer must be send. Multiple picture parameter buffers may be
 * sent for a single picture. In that case picture parameters will
 * apply to all slice data that follow it until a new picture
 * parameter buffer is sent.
 *
 * Notes:
 *   pic_quantizer_type should be set to the applicable quantizer
 *   type as defined by QUANTIZER (J.1.19) and either
 *   PQUANTIZER (7.1.1.8) or PQINDEX (7.1.1.6)
 */
typedef struct _VAPictureParameterBufferVC1 {
    VASurfaceID forward_reference_picture;
    VASurfaceID backward_reference_picture;
    /* if out-of-loop post-processing is done on the render
       target, then we need to keep the in-loop decoded
       picture as a reference picture */
    VASurfaceID inloop_decoded_picture;

    /* sequence layer for AP or meta data for SP and MP */
    union {
        struct {
            uint32_t pulldown   : 1; /* SEQUENCE_LAYER::PULLDOWN */
            uint32_t interlace  : 1; /* SEQUENCE_LAYER::INTERLACE */
            uint32_t tfcntrflag : 1; /* SEQUENCE_LAYER::TFCNTRFLAG */
            uint32_t finterpflag    : 1; /* SEQUENCE_LAYER::FINTERPFLAG */
            uint32_t psf        : 1; /* SEQUENCE_LAYER::PSF */
            uint32_t multires   : 1; /* METADATA::MULTIRES */
            uint32_t overlap    : 1; /* METADATA::OVERLAP */
            uint32_t syncmarker : 1; /* METADATA::SYNCMARKER */
            uint32_t rangered   : 1; /* METADATA::RANGERED */
            uint32_t max_b_frames   : 3; /* METADATA::MAXBFRAMES */
            uint32_t profile    : 2; /* SEQUENCE_LAYER::PROFILE or The MSB of METADATA::PROFILE */
        } bits;
        uint32_t value;
    } sequence_fields;

    uint16_t coded_width;       /* ENTRY_POINT_LAYER::CODED_WIDTH */
    uint16_t coded_height;  /* ENTRY_POINT_LAYER::CODED_HEIGHT */
    union {
        struct {
            uint32_t broken_link    : 1; /* ENTRY_POINT_LAYER::BROKEN_LINK */
            uint32_t closed_entry   : 1; /* ENTRY_POINT_LAYER::CLOSED_ENTRY */
            uint32_t panscan_flag   : 1; /* ENTRY_POINT_LAYER::PANSCAN_FLAG */
            uint32_t loopfilter : 1; /* ENTRY_POINT_LAYER::LOOPFILTER */
        } bits;
        uint32_t value;
    } entrypoint_fields;
    uint8_t conditional_overlap_flag; /* ENTRY_POINT_LAYER::CONDOVER */
    uint8_t fast_uvmc_flag; /* ENTRY_POINT_LAYER::FASTUVMC */
    union {
        struct {
            uint32_t luma_flag  : 1; /* ENTRY_POINT_LAYER::RANGE_MAPY_FLAG */
            uint32_t luma       : 3; /* ENTRY_POINT_LAYER::RANGE_MAPY */
            uint32_t chroma_flag    : 1; /* ENTRY_POINT_LAYER::RANGE_MAPUV_FLAG */
            uint32_t chroma     : 3; /* ENTRY_POINT_LAYER::RANGE_MAPUV */
        } bits;
        uint32_t value;
    } range_mapping_fields;

    uint8_t b_picture_fraction; /* Index for PICTURE_LAYER::BFRACTION value in Table 40 (7.1.1.14) */
    uint8_t cbp_table;      /* PICTURE_LAYER::CBPTAB/ICBPTAB */
    uint8_t mb_mode_table;  /* PICTURE_LAYER::MBMODETAB */
    uint8_t range_reduction_frame;/* PICTURE_LAYER::RANGEREDFRM */
    uint8_t rounding_control;   /* PICTURE_LAYER::RNDCTRL */
    uint8_t post_processing;    /* PICTURE_LAYER::POSTPROC */
    uint8_t picture_resolution_index;   /* PICTURE_LAYER::RESPIC */
    uint8_t luma_scale;     /* PICTURE_LAYER::LUMSCALE */
    uint8_t luma_shift;     /* PICTURE_LAYER::LUMSHIFT */

    union {
        struct {
            uint32_t picture_type       : 3; /* PICTURE_LAYER::PTYPE */
            uint32_t frame_coding_mode  : 3; /* PICTURE_LAYER::FCM */
            uint32_t top_field_first    : 1; /* PICTURE_LAYER::TFF */
            uint32_t is_first_field     : 1; /* set to 1 if it is the first field */
            uint32_t intensity_compensation : 1; /* PICTURE_LAYER::INTCOMP */
        } bits;
        uint32_t value;
    } picture_fields;
    union {
        struct {
            uint32_t mv_type_mb : 1;    /* PICTURE::MVTYPEMB */
            uint32_t direct_mb  : 1;    /* PICTURE::DIRECTMB */
            uint32_t skip_mb    : 1;    /* PICTURE::SKIPMB */
            uint32_t field_tx   : 1;    /* PICTURE::FIELDTX */
            uint32_t forward_mb : 1;    /* PICTURE::FORWARDMB */
            uint32_t ac_pred    : 1;    /* PICTURE::ACPRED */
            uint32_t overflags  : 1;    /* PICTURE::OVERFLAGS */
        } flags;
        uint32_t value;
    } raw_coding;
    union {
        struct {
            uint32_t bp_mv_type_mb   : 1;    /* PICTURE::MVTYPEMB */
            uint32_t bp_direct_mb    : 1;    /* PICTURE::DIRECTMB */
            uint32_t bp_skip_mb      : 1;    /* PICTURE::SKIPMB */
            uint32_t bp_field_tx     : 1;    /* PICTURE::FIELDTX */
            uint32_t bp_forward_mb   : 1;    /* PICTURE::FORWARDMB */
            uint32_t bp_ac_pred      : 1;    /* PICTURE::ACPRED */
            uint32_t bp_overflags    : 1;    /* PICTURE::OVERFLAGS */
        } flags;
        uint32_t value;
    } bitplane_present; /* signal what bitplane is being passed via the bitplane buffer */
    union {
        struct {
            uint32_t reference_distance_flag : 1;/* PICTURE_LAYER::REFDIST_FLAG */
            uint32_t reference_distance : 5;/* PICTURE_LAYER::REFDIST */
            uint32_t num_reference_pictures: 1;/* PICTURE_LAYER::NUMREF */
            uint32_t reference_field_pic_indicator  : 1;/* PICTURE_LAYER::REFFIELD */
        } bits;
        uint32_t value;
    } reference_fields;
    union {
        struct {
            uint32_t mv_mode        : 3; /* PICTURE_LAYER::MVMODE */
            uint32_t mv_mode2       : 3; /* PICTURE_LAYER::MVMODE2 */
            uint32_t mv_table       : 3; /* PICTURE_LAYER::MVTAB/IMVTAB */
            uint32_t two_mv_block_pattern_table: 2; /* PICTURE_LAYER::2MVBPTAB */
            uint32_t four_mv_switch     : 1; /* PICTURE_LAYER::4MVSWITCH */
            uint32_t four_mv_block_pattern_table : 2; /* PICTURE_LAYER::4MVBPTAB */
            uint32_t extended_mv_flag   : 1; /* ENTRY_POINT_LAYER::EXTENDED_MV */
            uint32_t extended_mv_range  : 2; /* PICTURE_LAYER::MVRANGE */
            uint32_t extended_dmv_flag  : 1; /* ENTRY_POINT_LAYER::EXTENDED_DMV */
            uint32_t extended_dmv_range : 2; /* PICTURE_LAYER::DMVRANGE */
        } bits;
        uint32_t value;
    } mv_fields;
    union {
        struct {
            uint32_t dquant : 2;    /* ENTRY_POINT_LAYER::DQUANT */
            uint32_t quantizer     : 2;     /* ENTRY_POINT_LAYER::QUANTIZER */
            uint32_t half_qp    : 1;    /* PICTURE_LAYER::HALFQP */
            uint32_t pic_quantizer_scale : 5;/* PICTURE_LAYER::PQUANT */
            uint32_t pic_quantizer_type : 1;/* PICTURE_LAYER::PQUANTIZER */
            uint32_t dq_frame   : 1;    /* VOPDQUANT::DQUANTFRM */
            uint32_t dq_profile : 2;    /* VOPDQUANT::DQPROFILE */
            uint32_t dq_sb_edge : 2;    /* VOPDQUANT::DQSBEDGE */
            uint32_t dq_db_edge     : 2;    /* VOPDQUANT::DQDBEDGE */
            uint32_t dq_binary_level : 1;   /* VOPDQUANT::DQBILEVEL */
            uint32_t alt_pic_quantizer : 5;/* VOPDQUANT::ALTPQUANT */
        } bits;
        uint32_t value;
    } pic_quantizer_fields;
    union {
        struct {
            uint32_t variable_sized_transform_flag  : 1;/* ENTRY_POINT_LAYER::VSTRANSFORM */
            uint32_t mb_level_transform_type_flag   : 1;/* PICTURE_LAYER::TTMBF */
            uint32_t frame_level_transform_type : 2;/* PICTURE_LAYER::TTFRM */
            uint32_t transform_ac_codingset_idx1    : 2;/* PICTURE_LAYER::TRANSACFRM */
            uint32_t transform_ac_codingset_idx2    : 2;/* PICTURE_LAYER::TRANSACFRM2 */
            uint32_t intra_transform_dc_table   : 1;/* PICTURE_LAYER::TRANSDCTAB */
        } bits;
        uint32_t value;
    } transform_fields;

    uint8_t luma_scale2;                  /* PICTURE_LAYER::LUMSCALE2 */
    uint8_t luma_shift2;                  /* PICTURE_LAYER::LUMSHIFT2 */
    uint8_t intensity_compensation_field; /* Index for PICTURE_LAYER::INTCOMPFIELD value in Table 109 (9.1.1.48) */

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_MEDIUM - 1];
} VAPictureParameterBufferVC1;

/** VC-1 Bitplane Buffer
There will be at most three bitplanes coded in any picture header. To send
the bitplane data more efficiently, each byte is divided in two nibbles, with
each nibble carrying three bitplanes for one macroblock.  The following table
shows the bitplane data arrangement within each nibble based on the picture
type.

Picture Type    Bit3        Bit2        Bit1        Bit0
I or BI             OVERFLAGS   ACPRED      FIELDTX
P               MYTYPEMB    SKIPMB      DIRECTMB
B               FORWARDMB   SKIPMB      DIRECTMB

Within each byte, the lower nibble is for the first MB and the upper nibble is
for the second MB.  E.g. the lower nibble of the first byte in the bitplane
buffer is for Macroblock #1 and the upper nibble of the first byte is for
Macroblock #2 in the first row.
*/

/* VC-1 Slice Parameter Buffer */
typedef struct _VASliceParameterBufferVC1 {
    uint32_t slice_data_size;/* number of bytes in the slice data buffer for this slice */
    uint32_t slice_data_offset;/* the offset to the first byte of slice data */
    uint32_t slice_data_flag; /* see VA_SLICE_DATA_FLAG_XXX defintions */
    uint32_t macroblock_offset;/* the offset to the first bit of MB from the first byte of slice data */
    uint32_t slice_vertical_position;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VASliceParameterBufferVC1;

/* VC-1 Slice Data Buffer */
/*
This is simplely a buffer containing raw bit-stream bytes
*/

/****************************
 * H.264/AVC data structures
 ****************************/

typedef struct _VAPictureH264 {
    VASurfaceID picture_id;
    uint32_t frame_idx;
    uint32_t flags;
    int32_t TopFieldOrderCnt;
    int32_t BottomFieldOrderCnt;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAPictureH264;
/* flags in VAPictureH264 could be OR of the following */
#define VA_PICTURE_H264_INVALID         0x00000001
#define VA_PICTURE_H264_TOP_FIELD       0x00000002
#define VA_PICTURE_H264_BOTTOM_FIELD        0x00000004
#define VA_PICTURE_H264_SHORT_TERM_REFERENCE    0x00000008
#define VA_PICTURE_H264_LONG_TERM_REFERENCE 0x00000010

/** H.264 Picture Parameter Buffer */
/*
 * For each picture, and before any slice data, a single
 * picture parameter buffer must be send.
 */
typedef struct _VAPictureParameterBufferH264 {
    VAPictureH264 CurrPic;
    VAPictureH264 ReferenceFrames[16];  /* in DPB */
    uint16_t picture_width_in_mbs_minus1;
    uint16_t picture_height_in_mbs_minus1;
    uint8_t bit_depth_luma_minus8;
    uint8_t bit_depth_chroma_minus8;
    uint8_t num_ref_frames;
    union {
        struct {
            uint32_t chroma_format_idc          : 2;
            uint32_t residual_colour_transform_flag     : 1; /* Renamed to separate_colour_plane_flag in newer standard versions. */
            uint32_t gaps_in_frame_num_value_allowed_flag   : 1;
            uint32_t frame_mbs_only_flag            : 1;
            uint32_t mb_adaptive_frame_field_flag       : 1;
            uint32_t direct_8x8_inference_flag      : 1;
            uint32_t MinLumaBiPredSize8x8           : 1; /* see A.3.3.2 */
            uint32_t log2_max_frame_num_minus4      : 4;
            uint32_t pic_order_cnt_type         : 2;
            uint32_t log2_max_pic_order_cnt_lsb_minus4  : 4;
            uint32_t delta_pic_order_always_zero_flag   : 1;
        } bits;
        uint32_t value;
    } seq_fields;
    // FMO is not supported.
    va_deprecated uint8_t num_slice_groups_minus1;
    va_deprecated uint8_t slice_group_map_type;
    va_deprecated uint16_t slice_group_change_rate_minus1;
    int8_t pic_init_qp_minus26;
    int8_t pic_init_qs_minus26;
    int8_t chroma_qp_index_offset;
    int8_t second_chroma_qp_index_offset;
    union {
        struct {
            uint32_t entropy_coding_mode_flag   : 1;
            uint32_t weighted_pred_flag     : 1;
            uint32_t weighted_bipred_idc        : 2;
            uint32_t transform_8x8_mode_flag    : 1;
            uint32_t field_pic_flag         : 1;
            uint32_t constrained_intra_pred_flag    : 1;
            uint32_t pic_order_present_flag         : 1; /* Renamed to bottom_field_pic_order_in_frame_present_flag in newer standard versions. */
            uint32_t deblocking_filter_control_present_flag : 1;
            uint32_t redundant_pic_cnt_present_flag     : 1;
            uint32_t reference_pic_flag         : 1; /* nal_ref_idc != 0 */
        } bits;
        uint32_t value;
    } pic_fields;
    uint16_t frame_num;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_MEDIUM];
} VAPictureParameterBufferH264;

/** H.264 Inverse Quantization Matrix Buffer */
typedef struct _VAIQMatrixBufferH264 {
    /** \brief 4x4 scaling list, in raster scan order. */
    uint8_t ScalingList4x4[6][16];
    /** \brief 8x8 scaling list, in raster scan order. */
    uint8_t ScalingList8x8[2][64];

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAIQMatrixBufferH264;

/** H.264 Slice Parameter Buffer */
typedef struct _VASliceParameterBufferH264 {
    uint32_t slice_data_size;/* number of bytes in the slice data buffer for this slice */
    /** \brief Byte offset to the NAL Header Unit for this slice. */
    uint32_t slice_data_offset;
    uint32_t slice_data_flag; /* see VA_SLICE_DATA_FLAG_XXX defintions */
    /**
     * \brief Bit offset from NAL Header Unit to the begining of slice_data().
     *
     * This bit offset is relative to and includes the NAL unit byte
     * and represents the number of bits parsed in the slice_header()
     * after the removal of any emulation prevention bytes in
     * there. However, the slice data buffer passed to the hardware is
     * the original bitstream, thus including any emulation prevention
     * bytes.
     */
    uint16_t slice_data_bit_offset;
    uint16_t first_mb_in_slice;
    uint8_t slice_type;
    uint8_t direct_spatial_mv_pred_flag;
    /**
     * H264/AVC syntax element
     *
     * if num_ref_idx_active_override_flag equals 0, host decoder should
     * set its value to num_ref_idx_l0_default_active_minus1.
     */
    uint8_t num_ref_idx_l0_active_minus1;
    /**
     * H264/AVC syntax element
     *
     * if num_ref_idx_active_override_flag equals 0, host decoder should
     * set its value to num_ref_idx_l1_default_active_minus1.
     */
    uint8_t num_ref_idx_l1_active_minus1;
    uint8_t cabac_init_idc;
    int8_t slice_qp_delta;
    uint8_t disable_deblocking_filter_idc;
    int8_t slice_alpha_c0_offset_div2;
    int8_t slice_beta_offset_div2;
    VAPictureH264 RefPicList0[32];  /* See 8.2.4.2 */
    VAPictureH264 RefPicList1[32];  /* See 8.2.4.2 */
    uint8_t luma_log2_weight_denom;
    uint8_t chroma_log2_weight_denom;
    uint8_t luma_weight_l0_flag;
    int16_t luma_weight_l0[32];
    int16_t luma_offset_l0[32];
    uint8_t chroma_weight_l0_flag;
    int16_t chroma_weight_l0[32][2];
    int16_t chroma_offset_l0[32][2];
    uint8_t luma_weight_l1_flag;
    int16_t luma_weight_l1[32];
    int16_t luma_offset_l1[32];
    uint8_t chroma_weight_l1_flag;
    int16_t chroma_weight_l1[32][2];
    int16_t chroma_offset_l1[32][2];

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VASliceParameterBufferH264;

/****************************
 * Common encode data structures
 ****************************/
typedef enum {
    VAEncPictureTypeIntra       = 0,
    VAEncPictureTypePredictive      = 1,
    VAEncPictureTypeBidirectional   = 2,
} VAEncPictureType;

/**
 * \brief Encode Slice Parameter Buffer.
 *
 * @deprecated
 * This is a deprecated encode slice parameter buffer, All applications
 * \c can use VAEncSliceParameterBufferXXX (XXX = MPEG2, HEVC, H264, JPEG)
 */
typedef struct _VAEncSliceParameterBuffer {
    uint32_t start_row_number;  /* starting MB row number for this slice */
    uint32_t slice_height;  /* slice height measured in MB */
    union {
        struct {
            uint32_t is_intra   : 1;
            uint32_t disable_deblocking_filter_idc : 2;
            uint32_t uses_long_term_ref     : 1;
            uint32_t is_long_term_ref       : 1;
        } bits;
        uint32_t value;
    } slice_flags;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAEncSliceParameterBuffer;


/****************************
 * H.263 specific encode data structures
 ****************************/

typedef struct _VAEncSequenceParameterBufferH263 {
    uint32_t intra_period;
    uint32_t bits_per_second;
    uint32_t frame_rate;
    uint32_t initial_qp;
    uint32_t min_qp;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAEncSequenceParameterBufferH263;

typedef struct _VAEncPictureParameterBufferH263 {
    VASurfaceID reference_picture;
    VASurfaceID reconstructed_picture;
    VABufferID coded_buf;
    uint16_t picture_width;
    uint16_t picture_height;
    VAEncPictureType picture_type;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAEncPictureParameterBufferH263;

/****************************
 * MPEG-4 specific encode data structures
 ****************************/

typedef struct _VAEncSequenceParameterBufferMPEG4 {
    uint8_t profile_and_level_indication;
    uint32_t intra_period;
    uint32_t video_object_layer_width;
    uint32_t video_object_layer_height;
    uint32_t vop_time_increment_resolution;
    uint32_t fixed_vop_rate;
    uint32_t fixed_vop_time_increment;
    uint32_t bits_per_second;
    uint32_t frame_rate;
    uint32_t initial_qp;
    uint32_t min_qp;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAEncSequenceParameterBufferMPEG4;

typedef struct _VAEncPictureParameterBufferMPEG4 {
    VASurfaceID reference_picture;
    VASurfaceID reconstructed_picture;
    VABufferID coded_buf;
    uint16_t picture_width;
    uint16_t picture_height;
    uint32_t modulo_time_base; /* number of 1s */
    uint32_t vop_time_increment;
    VAEncPictureType picture_type;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAEncPictureParameterBufferMPEG4;



/** Buffer functions */

/**
 * Creates a buffer for "num_elements" elements of "size" bytes and
 * initalize with "data".
 * if "data" is null, then the contents of the buffer data store
 * are undefined.
 * Basically there are two ways to get buffer data to the server side. One is
 * to call vaCreateBuffer() with a non-null "data", which results the data being
 * copied to the data store on the server side.  A different method that
 * eliminates this copy is to pass null as "data" when calling vaCreateBuffer(),
 * and then use vaMapBuffer() to map the data store from the server side to the
 * client address space for access.
 * The user must call vaDestroyBuffer() to destroy a buffer.
 *  Note: image buffers are created by the library, not the client. Please see
 *        vaCreateImage on how image buffers are managed.
 */
VAStatus vaCreateBuffer(
    VADisplay dpy,
    VAContextID context,
    VABufferType type,  /* in */
    unsigned int size,  /* in */
    unsigned int num_elements, /* in */
    void *data,     /* in */
    VABufferID *buf_id  /* out */
);

/**
 * Create a buffer for given width & height get unit_size, pitch, buf_id for 2D buffer
 * for permb qp buffer, it will return unit_size for one MB or LCU and the pitch for alignments
 * can call vaMapBuffer with this Buffer ID to get virtual address.
 * e.g. AVC 1080P encode, 1920x1088, the size in MB is 120x68,but inside driver,
 * maybe it should align with 256, and one byte present one Qp.so, call the function.
 * then get unit_size = 1, pitch = 256. call vaMapBuffer to get the virtual address (pBuf).
 * then read write the memory like 2D. the size is 256x68, application can only use 120x68
 * pBuf + 256 is the start of next line.
 * different driver implementation maybe return different unit_size and pitch
 */
VAStatus vaCreateBuffer2(
    VADisplay dpy,
    VAContextID context,
    VABufferType type,
    unsigned int width,
    unsigned int height,
    unsigned int *unit_size,
    unsigned int *pitch,
    VABufferID *buf_id
);

/**
 * Convey to the server how many valid elements are in the buffer.
 * e.g. if multiple slice parameters are being held in a single buffer,
 * this will communicate to the server the number of slice parameters
 * that are valid in the buffer.
 */
VAStatus vaBufferSetNumElements(
    VADisplay dpy,
    VABufferID buf_id,  /* in */
    unsigned int num_elements /* in */
);


/**
 * device independent data structure for codedbuffer
 */

/*
 * FICTURE_AVE_QP(bit7-0): The average Qp value used during this frame
 * LARGE_SLICE(bit8):At least one slice in the current frame was large
 *              enough for the encoder to attempt to limit its size.
 * SLICE_OVERFLOW(bit9): At least one slice in the current frame has
 *              exceeded the maximum slice size specified.
 * BITRATE_OVERFLOW(bit10): The peak bitrate was exceeded for this frame.
 * BITRATE_HIGH(bit11): The frame size got within the safety margin of the maximum size (VCM only)
 * AIR_MB_OVER_THRESHOLD: the number of MBs adapted to Intra MB
 */
#define VA_CODED_BUF_STATUS_PICTURE_AVE_QP_MASK         0xff
#define VA_CODED_BUF_STATUS_LARGE_SLICE_MASK            0x100
#define VA_CODED_BUF_STATUS_SLICE_OVERFLOW_MASK         0x200
#define VA_CODED_BUF_STATUS_BITRATE_OVERFLOW        0x400
#define VA_CODED_BUF_STATUS_BITRATE_HIGH        0x800
/**
 * \brief The frame has exceeded the maximum requested size.
 *
 * This flag indicates that the encoded frame size exceeds the value
 * specified through a misc parameter buffer of type
 * #VAEncMiscParameterTypeMaxFrameSize.
 */
#define VA_CODED_BUF_STATUS_FRAME_SIZE_OVERFLOW         0x1000
/**
 * \brief the bitstream is bad or corrupt.
 */
#define VA_CODED_BUF_STATUS_BAD_BITSTREAM               0x8000
#define VA_CODED_BUF_STATUS_AIR_MB_OVER_THRESHOLD   0xff0000

/**
 * \brief The coded buffer segment status contains frame encoding passes number
 *
 * This is the mask to get the number of encoding passes from the coded
 * buffer segment status.
 * NUMBER_PASS(bit24~bit27): the number for encoding passes executed for the coded frame.
 *
 */
#define VA_CODED_BUF_STATUS_NUMBER_PASSES_MASK          0xf000000

/**
 * \brief The coded buffer segment contains a single NAL unit.
 *
 * This flag indicates that the coded buffer segment contains a
 * single NAL unit. This flag might be useful to the user for
 * processing the coded buffer.
 */
#define VA_CODED_BUF_STATUS_SINGLE_NALU                 0x10000000

/**
 * \brief Coded buffer segment.
 *
 * #VACodedBufferSegment is an element of a linked list describing
 * some information on the coded buffer. The coded buffer segment
 * could contain either a single NAL unit, or more than one NAL unit.
 * It is recommended (but not required) to return a single NAL unit
 * in a coded buffer segment, and the implementation should set the
 * VA_CODED_BUF_STATUS_SINGLE_NALU status flag if that is the case.
 */
typedef  struct _VACodedBufferSegment  {
    /**
     * \brief Size of the data buffer in this segment (in bytes).
     */
    uint32_t        size;
    /** \brief Bit offset into the data buffer where the video data starts. */
    uint32_t        bit_offset;
    /** \brief Status set by the driver. See \c VA_CODED_BUF_STATUS_*. */
    uint32_t        status;
    /** \brief Reserved for future use. */
    uint32_t        reserved;
    /** \brief Pointer to the start of the data buffer. */
    void               *buf;
    /**
     * \brief Pointer to the next #VACodedBufferSegment element,
     * or \c NULL if there is none.
     */
    void               *next;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VACodedBufferSegment;

/**
 * Map data store of the buffer into the client's address space
 * vaCreateBuffer() needs to be called with "data" set to NULL before
 * calling vaMapBuffer()
 *
 * if buffer type is VAEncCodedBufferType, pbuf points to link-list of
 * VACodedBufferSegment, and the list is terminated if "next" is NULL
 */
VAStatus vaMapBuffer(
    VADisplay dpy,
    VABufferID buf_id,  /* in */
    void **pbuf     /* out */
);

/**
 * After client making changes to a mapped data store, it needs to
 * "Unmap" it to let the server know that the data is ready to be
 * consumed by the server
 */
VAStatus vaUnmapBuffer(
    VADisplay dpy,
    VABufferID buf_id   /* in */
);

/**
 * After this call, the buffer is deleted and this buffer_id is no longer valid
 *
 * A buffer can be re-used and sent to the server by another Begin/Render/End
 * sequence if vaDestroyBuffer() is not called with this buffer.
 *
 * Note re-using a shared buffer (e.g. a slice data buffer) between the host and the
 * hardware accelerator can result in performance dropping.
 */
VAStatus vaDestroyBuffer(
    VADisplay dpy,
    VABufferID buffer_id
);

/** \brief VA buffer information */
typedef struct {
    /** \brief Buffer handle */
    uintptr_t           handle;
    /** \brief Buffer type (See \ref VABufferType). */
    uint32_t            type;
    /**
     * \brief Buffer memory type (See \ref VASurfaceAttribMemoryType).
     *
     * On input to vaAcquireBufferHandle(), this field can serve as a hint
     * to specify the set of memory types the caller is interested in.
     * On successful return from vaAcquireBufferHandle(), the field is
     * updated with the best matching memory type.
     */
    uint32_t            mem_type;
    /** \brief Size of the underlying buffer. */
    size_t              mem_size;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VABufferInfo;

/**
 * \brief Acquires buffer handle for external API usage
 *
 * Locks the VA buffer object \ref buf_id for external API usage like
 * EGL or OpenCL (OCL). This function is a synchronization point. This
 * means that any pending operation is guaranteed to be completed
 * prior to returning from the function.
 *
 * If the referenced VA buffer object is the backing store of a VA
 * surface, then this function acts as if vaSyncSurface() on the
 * parent surface was called first.
 *
 * The \ref VABufferInfo argument shall be zero'ed on input. On
 * successful output, the data structure is filled in with all the
 * necessary buffer level implementation details like handle, type,
 * memory type and memory size.
 *
 * Note: the external API implementation, or the application, can
 * express the memory types it is interested in by filling in the \ref
 * mem_type field accordingly. On successful output, the memory type
 * that fits best the request and that was used is updated in the \ref
 * VABufferInfo data structure. If none of the supplied memory types
 * is supported, then a \ref VA_STATUS_ERROR_UNSUPPORTED_MEMORY_TYPE
 * error is returned.
 *
 * The \ref VABufferInfo data is valid until vaReleaseBufferHandle()
 * is called. Besides, no additional operation is allowed on any of
 * the buffer parent object until vaReleaseBufferHandle() is called.
 * e.g. decoding into a VA surface backed with the supplied VA buffer
 * object \ref buf_id would fail with a \ref VA_STATUS_ERROR_SURFACE_BUSY
 * error.
 *
 * Possible errors:
 * - \ref VA_STATUS_ERROR_UNIMPLEMENTED: the VA driver implementation
 *   does not support this interface
 * - \ref VA_STATUS_ERROR_INVALID_DISPLAY: an invalid display was supplied
 * - \ref VA_STATUS_ERROR_INVALID_BUFFER: an invalid buffer was supplied
 * - \ref VA_STATUS_ERROR_UNSUPPORTED_BUFFERTYPE: the implementation
 *   does not support exporting buffers of the specified type
 * - \ref VA_STATUS_ERROR_UNSUPPORTED_MEMORY_TYPE: none of the requested
 *   memory types in \ref VABufferInfo.mem_type was supported
 *
 * @param[in] dpy               the VA display
 * @param[in] buf_id            the VA buffer
 * @param[in,out] buf_info      the associated VA buffer information
 * @return VA_STATUS_SUCCESS if successful
 */
VAStatus
vaAcquireBufferHandle(VADisplay dpy, VABufferID buf_id, VABufferInfo *buf_info);

/**
 * \brief Releases buffer after usage from external API
 *
 * Unlocks the VA buffer object \ref buf_id from external API usage like
 * EGL or OpenCL (OCL). This function is a synchronization point. This
 * means that any pending operation is guaranteed to be completed
 * prior to returning from the function.
 *
 * The \ref VABufferInfo argument shall point to the original data
 * structure that was obtained from vaAcquireBufferHandle(), unaltered.
 * This is necessary so that the VA driver implementation could
 * deallocate any resources that were needed.
 *
 * In any case, returning from this function invalidates any contents
 * in \ref VABufferInfo. i.e. the underlyng buffer handle is no longer
 * valid. Therefore, VA driver implementations are free to reset this
 * data structure to safe defaults.
 *
 * Possible errors:
 * - \ref VA_STATUS_ERROR_UNIMPLEMENTED: the VA driver implementation
 *   does not support this interface
 * - \ref VA_STATUS_ERROR_INVALID_DISPLAY: an invalid display was supplied
 * - \ref VA_STATUS_ERROR_INVALID_BUFFER: an invalid buffer was supplied
 * - \ref VA_STATUS_ERROR_UNSUPPORTED_BUFFERTYPE: the implementation
 *   does not support exporting buffers of the specified type
 *
 * @param[in] dpy               the VA display
 * @param[in] buf_id            the VA buffer
 * @return VA_STATUS_SUCCESS if successful
 */
VAStatus
vaReleaseBufferHandle(VADisplay dpy, VABufferID buf_id);

/** @name vaExportSurfaceHandle() flags
 *
 * @{
 */
/** Export surface to be read by external API. */
#define VA_EXPORT_SURFACE_READ_ONLY        0x0001
/** Export surface to be written by external API. */
#define VA_EXPORT_SURFACE_WRITE_ONLY       0x0002
/** Export surface to be both read and written by external API. */
#define VA_EXPORT_SURFACE_READ_WRITE       0x0003
/** Export surface with separate layers.
 *
 * For example, NV12 surfaces should be exported as two separate
 * planes for luma and chroma.
 */
#define VA_EXPORT_SURFACE_SEPARATE_LAYERS  0x0004
/** Export surface with composed layers.
 *
 * For example, NV12 surfaces should be exported as a single NV12
 * composed object.
 */
#define VA_EXPORT_SURFACE_COMPOSED_LAYERS  0x0008

/** @} */

/**
 * \brief Export a handle to a surface for use with an external API
 *
 * The exported handles are owned by the caller, and the caller is
 * responsible for freeing them when no longer needed (e.g. by closing
 * DRM PRIME file descriptors).
 *
 * This does not perform any synchronisation.  If the contents of the
 * surface will be read, vaSyncSurface() must be called before doing so.
 * If the contents of the surface are written, then all operations must
 * be completed externally before using the surface again by via VA-API
 * functions.
 *
 * @param[in] dpy          VA display.
 * @param[in] surface_id   Surface to export.
 * @param[in] mem_type     Memory type to export to.
 * @param[in] flags        Combination of flags to apply
 *   (VA_EXPORT_SURFACE_*).
 * @param[out] descriptor  Pointer to the descriptor structure to fill
 *   with the handle details.  The type of this structure depends on
 *   the value of mem_type.
 *
 * @return Status code:
 * - VA_STATUS_SUCCESS:    Success.
 * - VA_STATUS_ERROR_INVALID_DISPLAY:  The display is not valid.
 * - VA_STATUS_ERROR_UNIMPLEMENTED:  The driver does not implement
 *     this interface.
 * - VA_STATUS_ERROR_INVALID_SURFACE:  The surface is not valid, or
 *     the surface is not exportable in the specified way.
 * - VA_STATUS_ERROR_UNSUPPORTED_MEMORY_TYPE:  The driver does not
 *     support exporting surfaces to the specified memory type.
 */
VAStatus vaExportSurfaceHandle(VADisplay dpy,
                               VASurfaceID surface_id,
                               uint32_t mem_type, uint32_t flags,
                               void *descriptor);

/**
 * Render (Video Decode/Encode/Processing) Pictures
 *
 * A picture represents either a frame or a field.
 *
 * The Begin/Render/End sequence sends the video decode/encode/processing buffers
 * to the server
 */

/**
 * Get ready for a video pipeline
 * - decode a picture to a target surface
 * - encode a picture from a target surface
 * - process a picture to a target surface
 */
VAStatus vaBeginPicture(
    VADisplay dpy,
    VAContextID context,
    VASurfaceID render_target
);

/**
 * Send video decode, encode or processing buffers to the server.
 */
VAStatus vaRenderPicture(
    VADisplay dpy,
    VAContextID context,
    VABufferID *buffers,
    int num_buffers
);

/**
 * Make the end of rendering for a picture.
 * The server should start processing all pending operations for this
 * surface. This call is non-blocking. The client can start another
 * Begin/Render/End sequence on a different render target.
 * if VAContextID used in this function previously successfully passed
 * vaMFAddContext call, real processing will be started during vaMFSubmit
 */
VAStatus vaEndPicture(
    VADisplay dpy,
    VAContextID context
);

/**
 * Make the end of rendering for a pictures in contexts passed with submission.
 * The server should start processing all pending operations for contexts.
 * All contexts passed should be associated through vaMFAddContext
 * and call sequence Begin/Render/End performed.
 * This call is non-blocking. The client can start another
 * Begin/Render/End/vaMFSubmit sequence on a different render targets.
 * Return values:
 * VA_STATUS_SUCCESS - operation successful, context was removed.
 * VA_STATUS_ERROR_INVALID_CONTEXT - mf_context or one of contexts are invalid
 * due to mf_context not created or one of contexts not assotiated with mf_context
 * through vaAddContext.
 * VA_STATUS_ERROR_INVALID_PARAMETER - one of context has not submitted it's frame
 * through vaBeginPicture vaRenderPicture vaEndPicture call sequence.
 * dpy: display
 * mf_context: Multi-Frame context
 * contexts: list of contexts submitting their tasks for multi-frame operation.
 * num_contexts: number of passed contexts.
 */
VAStatus vaMFSubmit(
    VADisplay dpy,
    VAMFContextID mf_context,
    VAContextID * contexts,
    int num_contexts
);

/*

Synchronization

*/

/**
 * This function blocks until all pending operations on the render target
 * have been completed.  Upon return it is safe to use the render target for a
 * different picture.
 */
VAStatus vaSyncSurface(
    VADisplay dpy,
    VASurfaceID render_target
);

/** \brief Indicates an infinite timeout. */
#define VA_TIMEOUT_INFINITE 0xFFFFFFFFFFFFFFFF

/**
 * \brief Synchronizes pending operations associated with the supplied surface.
 *
 * This function blocks during specified timeout (in nanoseconds) until
 * all pending operations on the render target have been completed.
 * If timeout is zero, the function returns immediately.
 *
 * Possible errors:
 * - \ref VA_STATUS_ERROR_UNIMPLEMENTED: the VA driver implementation
 *   does not support this interface
 * - \ref VA_STATUS_ERROR_INVALID_DISPLAY: an invalid display was supplied
 * - \ref VA_STATUS_ERROR_INVALID_SURFACE: an invalid surface was supplied
 * - \ref VA_STATUS_ERROR_TIMEDOUT: synchronization is still in progress,
 *   client should call the function again to complete synchronization
 *
 * @param[in] dpy         the VA display
 * @param[in] surface     the surface for which synchronization is performed
 * @param[in] timeout_ns  the timeout in nanoseconds
 *
 */
VAStatus vaSyncSurface2(
    VADisplay dpy,
    VASurfaceID surface,
    uint64_t timeout_ns
);

typedef enum {
    VASurfaceRendering  = 1, /* Rendering in progress */
    VASurfaceDisplaying = 2, /* Displaying in progress (not safe to render into it) */
    /* this status is useful if surface is used as the source */
    /* of an overlay */
    VASurfaceReady  = 4, /* not being rendered or displayed */
    VASurfaceSkipped    = 8  /* Indicate a skipped frame during encode */
} VASurfaceStatus;

/**
 * Find out any pending ops on the render target
 */
VAStatus vaQuerySurfaceStatus(
    VADisplay dpy,
    VASurfaceID render_target,
    VASurfaceStatus *status /* out */
);

typedef enum {
    VADecodeSliceMissing            = 0,
    VADecodeMBError                 = 1,
} VADecodeErrorType;

/**
 * Client calls vaQuerySurfaceError with VA_STATUS_ERROR_DECODING_ERROR, server side returns
 * an array of structure VASurfaceDecodeMBErrors, and the array is terminated by setting status=-1
*/
typedef struct _VASurfaceDecodeMBErrors {
    int32_t status; /* 1 if hardware has returned detailed info below, -1 means this record is invalid */
    uint32_t start_mb; /* start mb address with errors */
    uint32_t end_mb;  /* end mb address with errors */
    VADecodeErrorType decode_error_type;
    uint32_t num_mb;   /* number of mbs with errors */
    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW - 1];
} VASurfaceDecodeMBErrors;

/**
 * After the application gets VA_STATUS_ERROR_DECODING_ERROR after calling vaSyncSurface(),
 * it can call vaQuerySurfaceError to find out further details on the particular error.
 * VA_STATUS_ERROR_DECODING_ERROR should be passed in as "error_status",
 * upon the return, error_info will point to an array of _VASurfaceDecodeMBErrors structure,
 * which is allocated and filled by libVA with detailed information on the missing or error macroblocks.
 * The array is terminated if "status==-1" is detected.
 */
VAStatus vaQuerySurfaceError(
    VADisplay dpy,
    VASurfaceID surface,
    VAStatus error_status,
    void **error_info
);

/**
 * \brief Synchronizes pending operations associated with the supplied buffer.
 *
 * This function blocks during specified timeout (in nanoseconds) until
 * all pending operations on the supplied buffer have been completed.
 * If timeout is zero, the function returns immediately.
 *
 * Possible errors:
 * - \ref VA_STATUS_ERROR_UNIMPLEMENTED: the VA driver implementation
 *   does not support this interface
 * - \ref VA_STATUS_ERROR_INVALID_DISPLAY: an invalid display was supplied
 * - \ref VA_STATUS_ERROR_INVALID_BUFFER: an invalid buffer was supplied
 * - \ref VA_STATUS_ERROR_TIMEDOUT: synchronization is still in progress,
 *   client should call the function again to complete synchronization
 *
 * @param[in] dpy         the VA display
 * @param[in] buf_id      the buffer for which synchronization is performed
 * @param[in] timeout_ns  the timeout in nanoseconds
 *
 */
VAStatus vaSyncBuffer(
    VADisplay dpy,
    VABufferID buf_id,
    uint64_t timeout_ns
);

/**
 * Notes about synchronization interfaces:
 * vaSyncSurface:
 * 1. Allows to synchronize output surface (i.e. from decoding or VP)
 * 2. Allows to synchronize all bitstreams being encoded from the given input surface (1->N pipelines).
 *
 * vaSyncSurface2:
 * 1. The same as vaSyncSurface but allows to specify a timeout
 *
 * vaSyncBuffer:
 * 1. Allows to synchronize output buffer (e.g. bitstream from encoding).
 *    Comparing to vaSyncSurface this function synchronizes given bitstream only.
 */

/**
 * Images and Subpictures
 * VAImage is used to either get the surface data to client memory, or
 * to copy image data in client memory to a surface.
 * Both images, subpictures and surfaces follow the same 2D coordinate system where origin
 * is at the upper left corner with positive X to the right and positive Y down
 */
#define VA_FOURCC(ch0, ch1, ch2, ch3) \
    ((unsigned long)(unsigned char) (ch0) | ((unsigned long)(unsigned char) (ch1) << 8) | \
    ((unsigned long)(unsigned char) (ch2) << 16) | ((unsigned long)(unsigned char) (ch3) << 24 ))

/* Pre-defined fourcc codes. */

/** NV12: two-plane 8-bit YUV 4:2:0.
 * The first plane contains Y, the second plane contains U and V in pairs of bytes.
 */
#define VA_FOURCC_NV12      0x3231564E
/** NV21: two-plane 8-bit YUV 4:2:0.
 * Same as NV12, but with U and V swapped.
 */
#define VA_FOURCC_NV21      0x3132564E

/** AI44: packed 4-bit YA.
 *
 * The bottom half of each byte contains luma, the top half contains alpha.
 */
#define VA_FOURCC_AI44      0x34344149

/** RGBA: packed 8-bit RGBA.
 *
 * Four bytes per pixel: red, green, blue, alpha.
 */
#define VA_FOURCC_RGBA      0x41424752
/** RGBX: packed 8-bit RGB.
 *
 * Four bytes per pixel: red, green, blue, unspecified.
 */
#define VA_FOURCC_RGBX      0x58424752
/** BGRA: packed 8-bit RGBA.
 *
 * Four bytes per pixel: blue, green, red, alpha.
 */
#define VA_FOURCC_BGRA      0x41524742
/** BGRX: packed 8-bit RGB.
 *
 * Four bytes per pixel: blue, green, red, unspecified.
 */
#define VA_FOURCC_BGRX      0x58524742
/** ARGB: packed 8-bit RGBA.
 *
 * Four bytes per pixel: alpha, red, green, blue.
 */
#define VA_FOURCC_ARGB      0x42475241
/** XRGB: packed 8-bit RGB.
 *
 * Four bytes per pixel: unspecified, red, green, blue.
 */
#define VA_FOURCC_XRGB      0x42475258
/** ABGR: packed 8-bit RGBA.
 *
 * Four bytes per pixel: alpha, blue, green, red.
 */
#define VA_FOURCC_ABGR          0x52474241
/** XBGR: packed 8-bit RGB.
 *
 * Four bytes per pixel: unspecified, blue, green, red.
 */
#define VA_FOURCC_XBGR          0x52474258

/** UYUV: packed 8-bit YUV 4:2:2.
 *
 * Four bytes per pair of pixels: U, Y, U, V.
 */
#define VA_FOURCC_UYVY          0x59565955
/** YUY2: packed 8-bit YUV 4:2:2.
 *
 * Four bytes per pair of pixels: Y, U, Y, V.
 */
#define VA_FOURCC_YUY2          0x32595559
/** AYUV: packed 8-bit YUVA 4:4:4.
 *
 * Four bytes per pixel: A, Y, U, V.
 */
#define VA_FOURCC_AYUV          0x56555941
/** NV11: two-plane 8-bit YUV 4:1:1.
 *
 * The first plane contains Y, the second plane contains U and V in pairs of bytes.
 */
#define VA_FOURCC_NV11          0x3131564e
/** YV12: three-plane 8-bit YUV 4:2:0.
 *
 * The three planes contain Y, V and U respectively.
 */
#define VA_FOURCC_YV12          0x32315659
/** P208: two-plane 8-bit YUV 4:2:2.
 *
 * The first plane contains Y, the second plane contains U and V in pairs of bytes.
 */
#define VA_FOURCC_P208          0x38303250
/** I420: three-plane 8-bit YUV 4:2:0.
 *
 * The three planes contain Y, U and V respectively.
 */
#define VA_FOURCC_I420          0x30323449
/** YV24: three-plane 8-bit YUV 4:4:4.
 *
 * The three planes contain Y, V and U respectively.
 */
#define VA_FOURCC_YV24          0x34325659
/** YV32: four-plane 8-bit YUVA 4:4:4
 *
 * The four planes contain Y, V, U and A respectively.
 */
#define VA_FOURCC_YV32          0x32335659
/** Y800: 8-bit greyscale.
 */
#define VA_FOURCC_Y800          0x30303859
/** IMC3: three-plane 8-bit YUV 4:2:0.
 *
 * Equivalent to YV12, but with the additional constraint that the pitch of all three planes
 * must be the same.
 */
#define VA_FOURCC_IMC3          0x33434D49
/** 411P: three-plane 8-bit YUV 4:1:1.
 *
 * The three planes contain Y, U and V respectively.
 */
#define VA_FOURCC_411P          0x50313134
/** 411R: three-plane 8-bit YUV.
 *
 * The subsampling is the transpose of 4:1:1 - full chroma appears on every fourth line.
 * The three planes contain Y, U and V respectively.
 */
#define VA_FOURCC_411R          0x52313134
/** 422H: three-plane 8-bit YUV 4:2:2.
 *
 * The three planes contain Y, U and V respectively.
 */
#define VA_FOURCC_422H          0x48323234
/** 422V: three-plane 8-bit YUV 4:4:0.
 *
 * The three planes contain Y, U and V respectively.
 */
#define VA_FOURCC_422V          0x56323234
/** 444P: three-plane 8-bit YUV 4:4:4.
 *
 * The three planes contain Y, U and V respectively.
 */
#define VA_FOURCC_444P          0x50343434

/** RGBP: three-plane 8-bit RGB.
 *
 * The three planes contain red, green and blue respectively.
 */
#define VA_FOURCC_RGBP          0x50424752
/** BGRP: three-plane 8-bit RGB.
 *
 * The three planes contain blue, green and red respectively.
 */
#define VA_FOURCC_BGRP          0x50524742
/** RG16: packed 5/6-bit RGB.
 *
 * Each pixel is a two-byte little-endian value.
 * Red, green and blue are found in bits 15:11, 10:5, 4:0 respectively.
 */
#define VA_FOURCC_RGB565        0x36314752
/** BG16: packed 5/6-bit RGB.
 *
 * Each pixel is a two-byte little-endian value.
 * Blue, green and red are found in bits 15:11, 10:5, 4:0 respectively.
 */
#define VA_FOURCC_BGR565        0x36314742

/** Y210: packed 10-bit YUV 4:2:2.
 *
 * Eight bytes represent a pair of pixels.  Each sample is a two-byte little-endian value,
 * with the bottom six bits ignored.  The samples are in the order Y, U, Y, V.
 */
#define VA_FOURCC_Y210          0x30313259
/** Y212: packed 12-bit YUV 4:2:2.
 *
 * Eight bytes represent a pair of pixels.  Each sample is a two-byte little-endian value.
 * The samples are in the order Y, U, Y, V.
 */
#define VA_FOURCC_Y212          0x32313259
/** Y216: packed 16-bit YUV 4:2:2.
 *
 * Eight bytes represent a pair of pixels.  Each sample is a two-byte little-endian value.
 * The samples are in the order Y, U, Y, V.
 */
#define VA_FOURCC_Y216          0x36313259
/** Y410: packed 10-bit YUVA 4:4:4.
 *
 * Each pixel is a four-byte little-endian value.
 * A, V, Y, U are found in bits 31:30, 29:20, 19:10, 9:0 respectively.
 */
#define VA_FOURCC_Y410          0x30313459
/** Y412 packed 12-bit YUVA 4:4:4.
 *
 * Each pixel is a set of four samples, each of which is a two-byte little-endian value.
 * The samples are in the order A, V, Y, U.
 */
#define VA_FOURCC_Y412          0x32313459
/** Y416: packed 16-bit YUVA 4:4:4.
 *
 * Each pixel is a set of four samples, each of which is a two-byte little-endian value.
 * The samples are in the order A, V, Y, U.
 */
#define VA_FOURCC_Y416          0x36313459

/** YV16: three-plane 8-bit YUV 4:2:2.
 *
 * The three planes contain Y, V and U respectively.
 */
#define VA_FOURCC_YV16          0x36315659
/** P010: two-plane 10-bit YUV 4:2:0.
 *
 * Each sample is a two-byte little-endian value with the bottom six bits ignored.
 * The first plane contains Y, the second plane contains U and V in pairs of samples.
 */
#define VA_FOURCC_P010          0x30313050
/** P012: two-plane 12-bit YUV 4:2:0.
 *
 * Each sample is a two-byte little-endian value with the bottom four bits ignored.
 * The first plane contains Y, the second plane contains U and V in pairs of samples.
 */
#define VA_FOURCC_P012          0x32313050
/** P016: two-plane 16-bit YUV 4:2:0.
 *
 * Each sample is a two-byte little-endian value.  The first plane contains Y, the second
 * plane contains U and V in pairs of samples.
 */
#define VA_FOURCC_P016          0x36313050

/** I010: three-plane 10-bit YUV 4:2:0.
 *
 * Each sample is a two-byte little-endian value with the top six bits ignored.
 * The three planes contain Y, V and U respectively.
 */
#define VA_FOURCC_I010          0x30313049

/** IYUV: three-plane 8-bit YUV 4:2:0.
 *
 * @deprecated Use I420 instead.
 */
#define VA_FOURCC_IYUV          0x56555949
/**
 * 10-bit Pixel RGB formats.
 */
#define VA_FOURCC_A2R10G10B10   0x30335241 /* VA_FOURCC('A','R','3','0') */
/**
 * 10-bit Pixel BGR formats.
 */
#define VA_FOURCC_A2B10G10R10   0x30334241 /* VA_FOURCC('A','B','3','0') */
/**
 * 10-bit Pixel RGB formats without alpha.
 */
#define VA_FOURCC_X2R10G10B10   0x30335258 /* VA_FOURCC('X','R','3','0') */
/**
 * 10-bit Pixel BGR formats without alpha.
 */
#define VA_FOURCC_X2B10G10R10   0x30334258 /* VA_FOURCC('X','B','3','0') */

/** Y8: 8-bit greyscale.
 *
 * Only a single sample, 8 bit Y plane for monochrome images
 */
#define VA_FOURCC_Y8            0x20203859
/** Y16: 16-bit greyscale.
 *
 * Only a single sample, 16 bit Y plane for monochrome images
 */
#define VA_FOURCC_Y16           0x20363159
/** VYUV: packed 8-bit YUV 4:2:2.
 *
 * Four bytes per pair of pixels: V, Y, U, V.
 */
#define VA_FOURCC_VYUY          0x59555956
/** YVYU: packed 8-bit YUV 4:2:2.
 *
 * Four bytes per pair of pixels: Y, V, Y, U.
 */
#define VA_FOURCC_YVYU          0x55595659
/** AGRB64: three-plane 16-bit ARGB 16:16:16:16
 *
 * The four planes contain: alpha, red, green, blue respectively.
 */
#define VA_FOURCC_ARGB64        0x34475241
/** ABGR64: three-plane 16-bit ABGR 16:16:16:16
 *
 * The four planes contain: alpha, blue, green, red respectively.
 */
#define VA_FOURCC_ABGR64        0x34474241
/** XYUV: packed 8-bit YUVX 4:4:4.
 *
 * Four bytes per pixel: X, Y, U, V.
 */
#define VA_FOURCC_XYUV          0x56555958

/* byte order */
#define VA_LSB_FIRST        1
#define VA_MSB_FIRST        2

typedef struct _VAImageFormat {
    uint32_t    fourcc;
    uint32_t    byte_order; /* VA_LSB_FIRST, VA_MSB_FIRST */
    uint32_t    bits_per_pixel;
    /* for RGB formats */
    uint32_t    depth; /* significant bits per pixel */
    uint32_t    red_mask;
    uint32_t    green_mask;
    uint32_t    blue_mask;
    uint32_t    alpha_mask;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAImageFormat;

typedef VAGenericID VAImageID;

typedef struct _VAImage {
    VAImageID       image_id; /* uniquely identify this image */
    VAImageFormat   format;
    VABufferID      buf;    /* image data buffer */
    /*
     * Image data will be stored in a buffer of type VAImageBufferType to facilitate
     * data store on the server side for optimal performance. The buffer will be
     * created by the CreateImage function, and proper storage allocated based on the image
     * size and format. This buffer is managed by the library implementation, and
     * accessed by the client through the buffer Map/Unmap functions.
     */
    uint16_t    width;
    uint16_t    height;
    uint32_t    data_size;
    uint32_t    num_planes; /* can not be greater than 3 */
    /*
     * An array indicating the scanline pitch in bytes for each plane.
     * Each plane may have a different pitch. Maximum 3 planes for planar formats
     */
    uint32_t    pitches[3];
    /*
     * An array indicating the byte offset from the beginning of the image data
     * to the start of each plane.
     */
    uint32_t    offsets[3];

    /* The following fields are only needed for paletted formats */
    int32_t num_palette_entries;   /* set to zero for non-palette images */
    /*
     * Each component is one byte and entry_bytes indicates the number of components in
     * each entry (eg. 3 for YUV palette entries). set to zero for non-palette images
     */
    int32_t entry_bytes;
    /*
     * An array of ascii characters describing the order of the components within the bytes.
     * Only entry_bytes characters of the string are used.
     */
    int8_t component_order[4];

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAImage;

/** Get maximum number of image formats supported by the implementation */
int vaMaxNumImageFormats(
    VADisplay dpy
);

/**
 * Query supported image formats
 * The caller must provide a "format_list" array that can hold at
 * least vaMaxNumImageFormats() entries. The actual number of formats
 * returned in "format_list" is returned in "num_formats".
 */
VAStatus vaQueryImageFormats(
    VADisplay dpy,
    VAImageFormat *format_list, /* out */
    int *num_formats        /* out */
);

/**
 * Create a VAImage structure
 * The width and height fields returned in the VAImage structure may get
 * enlarged for some YUV formats. Upon return from this function,
 * image->buf has been created and proper storage allocated by the library.
 * The client can access the image through the Map/Unmap calls.
 */
VAStatus vaCreateImage(
    VADisplay dpy,
    VAImageFormat *format,
    int width,
    int height,
    VAImage *image  /* out */
);

/**
 * Should call DestroyImage before destroying the surface it is bound to
 */
VAStatus vaDestroyImage(
    VADisplay dpy,
    VAImageID image
);

VAStatus vaSetImagePalette(
    VADisplay dpy,
    VAImageID image,
    /*
     * pointer to an array holding the palette data.  The size of the array is
     * num_palette_entries * entry_bytes in size.  The order of the components
     * in the palette is described by the component_order in VAImage struct
     */
    unsigned char *palette
);

/**
 * Retrive surface data into a VAImage
 * Image must be in a format supported by the implementation
 */
VAStatus vaGetImage(
    VADisplay dpy,
    VASurfaceID surface,
    int x,  /* coordinates of the upper left source pixel */
    int y,
    unsigned int width, /* width and height of the region */
    unsigned int height,
    VAImageID image
);

/**
 * Copy data from a VAImage to a surface
 * Image must be in a format supported by the implementation
 * Returns a VA_STATUS_ERROR_SURFACE_BUSY if the surface
 * shouldn't be rendered into when this is called
 */
VAStatus vaPutImage(
    VADisplay dpy,
    VASurfaceID surface,
    VAImageID image,
    int src_x,
    int src_y,
    unsigned int src_width,
    unsigned int src_height,
    int dest_x,
    int dest_y,
    unsigned int dest_width,
    unsigned int dest_height
);

/**
 * Derive an VAImage from an existing surface.
 * This interface will derive a VAImage and corresponding image buffer from
 * an existing VA Surface. The image buffer can then be mapped/unmapped for
 * direct CPU access. This operation is only possible on implementations with
 * direct rendering capabilities and internal surface formats that can be
 * represented with a VAImage. When the operation is not possible this interface
 * will return VA_STATUS_ERROR_OPERATION_FAILED. Clients should then fall back
 * to using vaCreateImage + vaPutImage to accomplish the same task in an
 * indirect manner.
 *
 * Implementations should only return success when the resulting image buffer
 * would be useable with vaMap/Unmap.
 *
 * When directly accessing a surface special care must be taken to insure
 * proper synchronization with the graphics hardware. Clients should call
 * vaQuerySurfaceStatus to insure that a surface is not the target of concurrent
 * rendering or currently being displayed by an overlay.
 *
 * Additionally nothing about the contents of a surface should be assumed
 * following a vaPutSurface. Implementations are free to modify the surface for
 * scaling or subpicture blending within a call to vaPutImage.
 *
 * Calls to vaPutImage or vaGetImage using the same surface from which the image
 * has been derived will return VA_STATUS_ERROR_SURFACE_BUSY. vaPutImage or
 * vaGetImage with other surfaces is supported.
 *
 * An image created with vaDeriveImage should be freed with vaDestroyImage. The
 * image and image buffer structures will be destroyed; however, the underlying
 * surface will remain unchanged until freed with vaDestroySurfaces.
 */
VAStatus vaDeriveImage(
    VADisplay dpy,
    VASurfaceID surface,
    VAImage *image  /* out */
);

/**
 * Subpictures
 * Subpicture is a special type of image that can be blended
 * with a surface during vaPutSurface(). Subpicture can be used to render
 * DVD sub-titles or closed captioning text etc.
 */

typedef VAGenericID VASubpictureID;

/** Get maximum number of subpicture formats supported by the implementation */
int vaMaxNumSubpictureFormats(
    VADisplay dpy
);

/** flags for subpictures */
#define VA_SUBPICTURE_CHROMA_KEYING         0x0001
#define VA_SUBPICTURE_GLOBAL_ALPHA          0x0002
#define VA_SUBPICTURE_DESTINATION_IS_SCREEN_COORD   0x0004
/**
 * Query supported subpicture formats
 * The caller must provide a "format_list" array that can hold at
 * least vaMaxNumSubpictureFormats() entries. The flags arrary holds the flag
 * for each format to indicate additional capabilities for that format. The actual
 * number of formats returned in "format_list" is returned in "num_formats".
 *  flags: returned value to indicate addtional capabilities
 *         VA_SUBPICTURE_CHROMA_KEYING - supports chroma-keying
 *         VA_SUBPICTURE_GLOBAL_ALPHA - supports global alpha
 *     VA_SUBPICTURE_DESTINATION_IS_SCREEN_COORD - supports unscaled screen relative subpictures for On Screen Display
 */

VAStatus vaQuerySubpictureFormats(
    VADisplay dpy,
    VAImageFormat *format_list, /* out */
    unsigned int *flags,    /* out */
    unsigned int *num_formats   /* out */
);

/**
 * Subpictures are created with an image associated.
 */
VAStatus vaCreateSubpicture(
    VADisplay dpy,
    VAImageID image,
    VASubpictureID *subpicture  /* out */
);

/**
 * Destroy the subpicture before destroying the image it is assocated to
 */
VAStatus vaDestroySubpicture(
    VADisplay dpy,
    VASubpictureID subpicture
);

/**
 * Bind an image to the subpicture. This image will now be associated with
 * the subpicture instead of the one at creation.
 */
VAStatus vaSetSubpictureImage(
    VADisplay dpy,
    VASubpictureID subpicture,
    VAImageID image
);

/**
 * If chromakey is enabled, then the area where the source value falls within
 * the chromakey [min, max] range is transparent
 * The chromakey component format is the following:
 *  For RGB: [0:7] Red [8:15] Blue [16:23] Green
 *  For YUV: [0:7] V [8:15] U [16:23] Y
 * The chromakey mask can be used to mask out certain components for chromakey
 * comparision
 */
VAStatus vaSetSubpictureChromakey(
    VADisplay dpy,
    VASubpictureID subpicture,
    unsigned int chromakey_min,
    unsigned int chromakey_max,
    unsigned int chromakey_mask
);

/**
 * Global alpha value is between 0 and 1. A value of 1 means fully opaque and
 * a value of 0 means fully transparent. If per-pixel alpha is also specified then
 * the overall alpha is per-pixel alpha multiplied by the global alpha
 */
VAStatus vaSetSubpictureGlobalAlpha(
    VADisplay dpy,
    VASubpictureID subpicture,
    float global_alpha
);

/**
 * vaAssociateSubpicture associates the subpicture with target_surfaces.
 * It defines the region mapping between the subpicture and the target
 * surfaces through source and destination rectangles (with the same width and height).
 * Both will be displayed at the next call to vaPutSurface.  Additional
 * associations before the call to vaPutSurface simply overrides the association.
 */
VAStatus vaAssociateSubpicture(
    VADisplay dpy,
    VASubpictureID subpicture,
    VASurfaceID *target_surfaces,
    int num_surfaces,
    int16_t src_x, /* upper left offset in subpicture */
    int16_t src_y,
    uint16_t src_width,
    uint16_t src_height,
    int16_t dest_x, /* upper left offset in surface */
    int16_t dest_y,
    uint16_t dest_width,
    uint16_t dest_height,
    /*
     * whether to enable chroma-keying, global-alpha, or screen relative mode
     * see VA_SUBPICTURE_XXX values
     */
    uint32_t flags
);

/**
 * vaDeassociateSubpicture removes the association of the subpicture with target_surfaces.
 */
VAStatus vaDeassociateSubpicture(
    VADisplay dpy,
    VASubpictureID subpicture,
    VASurfaceID *target_surfaces,
    int num_surfaces
);

/**
 * Display attributes
 * Display attributes are used to control things such as contrast, hue, saturation,
 * brightness etc. in the rendering process.  The application can query what
 * attributes are supported by the driver, and then set the appropriate attributes
 * before calling vaPutSurface()
 *
 * Display attributes can also be used to query/set platform or display adaptor (vaDisplay)
 * related information. These attributes do not depend on vaConfig, and could not be used
 * for vaPutSurface. Application can use vaQueryDisplayAttributes/vaGetDisplayAttributes
 * at anytime after vaInitialize, but (for settable attributes) vaSetDisplayAttributes should be
 * called after vaInitialize and before any other function call.
 *
 * To distinguish these two types of display attributes, display adaptor related attributes
 * should be marked as "HW attribute" in the description.
 */

/* PowerVR IEP Lite attributes */
typedef enum {
    VADISPLAYATTRIB_BLE_OFF              = 0x00,
    VADISPLAYATTRIB_BLE_LOW,
    VADISPLAYATTRIB_BLE_MEDIUM,
    VADISPLAYATTRIB_BLE_HIGH,
    VADISPLAYATTRIB_BLE_NONE,
} VADisplayAttribBLEMode;

/** attribute value for VADisplayAttribRotation   */
#define VA_ROTATION_NONE        0x00000000
#define VA_ROTATION_90          0x00000001
#define VA_ROTATION_180         0x00000002
#define VA_ROTATION_270         0x00000003
/**@}*/

/**
 * @name Mirroring directions
 *
 * Those values could be used for VADisplayAttribMirror attribute or
 * VAProcPipelineParameterBuffer::mirror_state.

 */
/**@{*/
/** \brief No Mirroring. */
#define VA_MIRROR_NONE              0x00000000
/** \brief Horizontal Mirroring. */
#define VA_MIRROR_HORIZONTAL        0x00000001
/** \brief Vertical Mirroring. */
#define VA_MIRROR_VERTICAL          0x00000002
/**@}*/

/** attribute value for VADisplayAttribOutOfLoopDeblock */
#define VA_OOL_DEBLOCKING_FALSE 0x00000000
#define VA_OOL_DEBLOCKING_TRUE  0x00000001

/** Render mode */
#define VA_RENDER_MODE_UNDEFINED           0
#define VA_RENDER_MODE_LOCAL_OVERLAY       1
#define VA_RENDER_MODE_LOCAL_GPU           2
#define VA_RENDER_MODE_EXTERNAL_OVERLAY    4
#define VA_RENDER_MODE_EXTERNAL_GPU        8

/** Render device */
#define VA_RENDER_DEVICE_UNDEFINED  0
#define VA_RENDER_DEVICE_LOCAL      1
#define VA_RENDER_DEVICE_EXTERNAL   2

/**\brief sub device info
 * Sub-device is the concept basing on the "device" behind "vaDisplay".
 * If a device could be divided to several sub devices, the task of
 * decode/encode/vpp could be assigned on one sub-device. So, application
 * could choose the sub device before any other operations. After that,
 * all of the task execution/resource allocation will be dispatched to
 * the sub device. If application does not choose the sub device, driver
 * will assign one as default.
 *
 * If the value == VA_ATTRIB_NOT_SUPPORTED, it mean that the attribute
 * is unsupport or UNKNOWN.
 */

typedef union _VADisplayAttribValSubDevice {
    struct {
        /** \brief current sub device index, read - write */
        uint32_t current_sub_device     : 4;
        /** \brief sub devices count, read - only */
        uint32_t sub_device_count       : 4;
        /** \brief reserved bits for future, must be zero*/
        uint32_t reserved               : 8;
        /** \brief bit mask to indicate which sub_device is available, read only
         * \code
         * VADisplayAttribValSubDevice reg;
         * VADisplayAttribute reg_attr;
         * reg_attr.type = VADisplayAttribSubDevice;
         * vaGetDisplayAttributes(dpy, &reg_attr, 1);
         * reg.value = reg_attr.value;
         *
         * for(int i = 0; i < reg.bits.sub_device_count; i ++ ){
         *    if((1<<i) & reg.bits.sub_device_mask){
         *        printf("sub device  %d can be selected", i);
         *    }
         *}
         * \endcode
         */
        uint32_t sub_device_mask       : 16;
    } bits;
    uint32_t value;
} VADisplayAttribValSubDevice;

/** Currently defined display attribute types */
typedef enum {
    VADisplayAttribBrightness       = 0,
    VADisplayAttribContrast     = 1,
    VADisplayAttribHue          = 2,
    VADisplayAttribSaturation       = 3,
    /* client can specifiy a background color for the target window
     * the new feature of video conference,
     * the uncovered area of the surface is filled by this color
     * also it will blend with the decoded video color
     */
    VADisplayAttribBackgroundColor      = 4,
    /*
     * this is a gettable only attribute. For some implementations that use the
     * hardware overlay, after PutSurface is called, the surface can not be
     * re-used until after the subsequent PutSurface call. If this is the case
     * then the value for this attribute will be set to 1 so that the client
     * will not attempt to re-use the surface right after returning from a call
     * to PutSurface.
     *
     * Don't use it, use flag VASurfaceDisplaying of vaQuerySurfaceStatus since
     * driver may use overlay or GPU alternatively
     */
    VADisplayAttribDirectSurface       = 5,
    VADisplayAttribRotation            = 6,
    VADisplayAttribOutofLoopDeblock    = 7,

    /* PowerVR IEP Lite specific attributes */
    VADisplayAttribBLEBlackMode        = 8,
    VADisplayAttribBLEWhiteMode        = 9,
    VADisplayAttribBlueStretch         = 10,
    VADisplayAttribSkinColorCorrection = 11,
    /*
     * For type VADisplayAttribCSCMatrix, "value" field is a pointer to the color
     * conversion matrix. Each element in the matrix is float-point
     */
    VADisplayAttribCSCMatrix           = 12,
    /* specify the constant color used to blend with video surface
     * Cd = Cv*Cc*Ac + Cb *(1 - Ac) C means the constant RGB
     *      d: the final color to overwrite into the frame buffer
     *      v: decoded video after color conversion,
     *      c: video color specified by VADisplayAttribBlendColor
     *      b: background color of the drawable
     */
    VADisplayAttribBlendColor          = 13,
    /*
     * Indicate driver to skip painting color key or not.
     * only applicable if the render is overlay
     */
    VADisplayAttribOverlayAutoPaintColorKey   = 14,
    /*
     * customized overlay color key, the format is RGB888
     * [23:16] = Red, [15:08] = Green, [07:00] = Blue.
     */
    VADisplayAttribOverlayColorKey  = 15,
    /*
     * The hint for the implementation of vaPutSurface
     * normally, the driver could use an overlay or GPU to render the surface on the screen
     * this flag provides APP the flexibity to switch the render dynamically
     */
    VADisplayAttribRenderMode           = 16,
    /*
     * specify if vaPutSurface needs to render into specified monitors
     * one example is that one external monitor (e.g. HDMI) is enabled,
     * but the window manager is not aware of it, and there is no associated drawable
     */
    VADisplayAttribRenderDevice        = 17,
    /*
     * specify vaPutSurface render area if there is no drawable on the monitor
     */
    VADisplayAttribRenderRect          = 18,
    /*
     * HW attribute, read/write, specify the sub device configure
     */
    VADisplayAttribSubDevice           = 19,
    /*
     * HW attribute. read only. specify whether vaCopy support on current HW
     * The value of each bit should equal to 1 << VA_EXEC_MODE_XXX to represent
     * modes of vaCopy
     */
    VADisplayAttribCopy                 = 20,
    /*
     * HW attribute. read only. retrieve the device information from backend driver
     * the value should be combined with vendor ID << 16 | device ID
     */
    VADisplayPCIID                      = 21,
} VADisplayAttribType;

/* flags for VADisplayAttribute */
#define VA_DISPLAY_ATTRIB_NOT_SUPPORTED 0x0000
#define VA_DISPLAY_ATTRIB_GETTABLE  0x0001
#define VA_DISPLAY_ATTRIB_SETTABLE  0x0002

typedef struct _VADisplayAttribute {
    VADisplayAttribType type;
    int32_t min_value;
    int32_t max_value;
    int32_t value;  /* used by the set/get attribute functions */
    /* flags can be VA_DISPLAY_ATTRIB_GETTABLE or VA_DISPLAY_ATTRIB_SETTABLE or OR'd together */
    uint32_t flags;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VADisplayAttribute;

/** Get maximum number of display attributs supported by the implementation */
int vaMaxNumDisplayAttributes(
    VADisplay dpy
);

/**
 * Query display attributes
 * The caller must provide a "attr_list" array that can hold at
 * least vaMaxNumDisplayAttributes() entries. The actual number of attributes
 * returned in "attr_list" is returned in "num_attributes".
 */
VAStatus vaQueryDisplayAttributes(
    VADisplay dpy,
    VADisplayAttribute *attr_list,  /* out */
    int *num_attributes         /* out */
);

/**
 * Get display attributes
 * This function returns the current attribute values in "attr_list".
 * Only attributes returned with VA_DISPLAY_ATTRIB_GETTABLE set in the "flags" field
 * from vaQueryDisplayAttributes() can have their values retrieved.
 */
VAStatus vaGetDisplayAttributes(
    VADisplay dpy,
    VADisplayAttribute *attr_list,  /* in/out */
    int num_attributes
);

/**
 * Set display attributes
 * Only attributes returned with VA_DISPLAY_ATTRIB_SETTABLE set in the "flags" field
 * from vaQueryDisplayAttributes() can be set.  If the attribute is not settable or
 * the value is out of range, the function returns VA_STATUS_ERROR_ATTR_NOT_SUPPORTED
 */
VAStatus vaSetDisplayAttributes(
    VADisplay dpy,
    VADisplayAttribute *attr_list,
    int num_attributes
);

/****************************
 * HEVC data structures
 ****************************/
/**
 * \brief Description of picture properties of those in DPB surfaces.
 *
 * If only progressive scan is supported, each surface contains one whole
 * frame picture.
 * Otherwise, each surface contains two fields of whole picture.
 * In this case, two entries of ReferenceFrames[] may share same picture_id
 * value.
 */
typedef struct _VAPictureHEVC {
    /** \brief reconstructed picture buffer surface index
     * invalid when taking value VA_INVALID_SURFACE.
     */
    VASurfaceID             picture_id;
    /** \brief picture order count.
     * in HEVC, POCs for top and bottom fields of same picture should
     * take different values.
     */
    int32_t                 pic_order_cnt;
    /* described below */
    uint32_t                flags;

    /** \brief Reserved bytes for future use, must be zero */
    uint32_t                va_reserved[VA_PADDING_LOW];
} VAPictureHEVC;

/* flags in VAPictureHEVC could be OR of the following */
#define VA_PICTURE_HEVC_INVALID                 0x00000001
/** \brief indication of interlace scan picture.
 * should take same value for all the pictures in sequence.
 */
#define VA_PICTURE_HEVC_FIELD_PIC               0x00000002
/** \brief polarity of the field picture.
 * top field takes even lines of buffer surface.
 * bottom field takes odd lines of buffer surface.
 */
#define VA_PICTURE_HEVC_BOTTOM_FIELD            0x00000004
/** \brief Long term reference picture */
#define VA_PICTURE_HEVC_LONG_TERM_REFERENCE     0x00000008
/**
 * VA_PICTURE_HEVC_RPS_ST_CURR_BEFORE, VA_PICTURE_HEVC_RPS_ST_CURR_AFTER
 * and VA_PICTURE_HEVC_RPS_LT_CURR of any picture in ReferenceFrames[] should
 * be exclusive. No more than one of them can be set for any picture.
 * Sum of NumPocStCurrBefore, NumPocStCurrAfter and NumPocLtCurr
 * equals NumPocTotalCurr, which should be equal to or smaller than 8.
 * Application should provide valid values for both short format and long format.
 * The pictures in DPB with any of these three flags turned on are referred by
 * the current picture.
 */
/** \brief RefPicSetStCurrBefore of HEVC spec variable
 * Number of ReferenceFrames[] entries with this bit set equals
 * NumPocStCurrBefore.
 */
#define VA_PICTURE_HEVC_RPS_ST_CURR_BEFORE      0x00000010
/** \brief RefPicSetStCurrAfter of HEVC spec variable
 * Number of ReferenceFrames[] entries with this bit set equals
 * NumPocStCurrAfter.
 */
#define VA_PICTURE_HEVC_RPS_ST_CURR_AFTER       0x00000020
/** \brief RefPicSetLtCurr of HEVC spec variable
 * Number of ReferenceFrames[] entries with this bit set equals
 * NumPocLtCurr.
 */
#define VA_PICTURE_HEVC_RPS_LT_CURR             0x00000040

typedef enum {
    VACopyObjectSurface = 0,
    VACopyObjectBuffer  = 1,
} VACopyObjectType;

typedef struct _VACopyObject {
    VACopyObjectType  obj_type;    // type of object.
    union {
        VASurfaceID surface_id;
        VABufferID  buffer_id;
    } object;

    uint32_t    va_reserved[VA_PADDING_MEDIUM];
} VACopyObject;

typedef union _VACopyOption {
    struct {
        /** \brief va copy synchronization, the value should be /c VA_EXEC_SYNC or /c VA_EXEC_ASYNC */
        uint32_t va_copy_sync : 2;
        /** \brief va copy mode, the value should be VA_EXEC_MODE_XXX */
        uint32_t va_copy_mode : 4;
        uint32_t reserved     : 26;
    } bits;
    uint32_t value;
} VACopyOption;

/** \brief Copies an object.
 *
 * Copies specified object (surface or buffer). If non-blocking copy
 * is requested (VA_COPY_NONBLOCK), then need vaSyncBuffer or vaSyncSurface/vaSyncSurface2
 * to sync the destination object.
 *
 * @param[in] dpy               the VA display
 * @param[in] dst               Destination object to copy to
 * @param[in] src               Source object to copy from
 * @param[in] option            VA copy option
 * @return VA_STATUS_SUCCESS if successful
 */
VAStatus vaCopy(VADisplay dpy, VACopyObject * dst, VACopyObject * src, VACopyOption option);

#include <va/va_dec_hevc.h>
#include <va/va_dec_jpeg.h>
#include <va/va_dec_vp8.h>
#include <va/va_dec_vp9.h>
#include <va/va_dec_av1.h>
#include <va/va_enc_hevc.h>
#include <va/va_fei_hevc.h>
#include <va/va_enc_h264.h>
#include <va/va_enc_jpeg.h>
#include <va/va_enc_mpeg2.h>
#include <va/va_enc_vp8.h>
#include <va/va_enc_vp9.h>
#include <va/va_enc_av1.h>
#include <va/va_fei.h>
#include <va/va_fei_h264.h>
#include <va/va_vpp.h>
#include <va/va_prot.h>

/**@}*/

#ifdef __cplusplus
}
#endif

#endif /* _VA_H_ */
