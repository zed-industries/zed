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
#ifndef AOM_AOM_AOM_ENCODER_H_
#define AOM_AOM_AOM_ENCODER_H_

/*!\defgroup encoder Encoder Algorithm Interface
 * \ingroup codec
 * This abstraction allows applications using this encoder to easily support
 * multiple video formats with minimal code duplication. This section describes
 * the interface common to all encoders.
 * @{
 */

/*!\file
 * \brief Describes the encoder algorithm interface to applications.
 *
 * This file describes the interface between an application and a
 * video encoder algorithm.
 *
 */
#ifdef __cplusplus
extern "C" {
#endif

#include "aom/aom_codec.h"  // IWYU pragma: export
#include "aom/aom_external_partition.h"

/*!\brief Current ABI version number
 *
 * \hideinitializer
 * \internal
 * If this file is altered in any way that changes the ABI, this value
 * must be bumped.  Examples include, but are not limited to, changing
 * types, removing or reassigning enums, adding/removing/rearranging
 * fields to structures
 *
 * Note: In the definition of AOM_ENCODER_ABI_VERSION, 3 is the value of
 * AOM_EXT_PART_ABI_VERSION in libaom v3.2.0. The old value of
 * AOM_EXT_PART_ABI_VERSION is used so as to not break the ABI version check in
 * aom_codec_enc_init_ver() when an application compiled against libaom v3.2.0
 * passes the old value of AOM_ENCODER_ABI_VERSION to aom_codec_enc_init_ver().
 * The external partition API is still experimental. When it is declared stable,
 * we will replace 3 with AOM_EXT_PART_ABI_VERSION in the definition of
 * AOM_ENCODER_ABI_VERSION.
 */
#define AOM_ENCODER_ABI_VERSION \
  (10 + AOM_CODEC_ABI_VERSION + /*AOM_EXT_PART_ABI_VERSION=*/3)

/*! \brief Encoder capabilities bitfield
 *
 *  Each encoder advertises the capabilities it supports as part of its
 *  ::aom_codec_iface_t interface structure. Capabilities are extra
 *  interfaces or functionality, and are not required to be supported
 *  by an encoder.
 *
 *  The available flags are specified by AOM_CODEC_CAP_* defines.
 */
#define AOM_CODEC_CAP_PSNR 0x10000 /**< Can issue PSNR packets */

/*! Can support input images at greater than 8 bitdepth.
 */
#define AOM_CODEC_CAP_HIGHBITDEPTH 0x40000

/*! \brief Initialization-time Feature Enabling
 *
 *  Certain codec features must be known at initialization time, to allow
 *  for proper memory allocation.
 *
 *  The available flags are specified by AOM_CODEC_USE_* defines.
 */
#define AOM_CODEC_USE_PSNR 0x10000         /**< Calculate PSNR on each frame */
#define AOM_CODEC_USE_HIGHBITDEPTH 0x40000 /**< Use high bitdepth */
// 0x80000 was used for the experimental feature AOM_CODEC_USE_PRESET during
// libaom v3.11.0 development but was removed before the release.

/*!\brief Generic fixed size buffer structure
 *
 * This structure is able to hold a reference to any fixed size buffer.
 */
typedef struct aom_fixed_buf {
  void *buf;       /**< Pointer to the data. Does NOT own the data! */
  size_t sz;       /**< Length of the buffer, in chars */
} aom_fixed_buf_t; /**< alias for struct aom_fixed_buf */

/*!\brief Error Resilient flags
 *
 * These flags define which error resilient features to enable in the
 * encoder. The flags are specified through the
 * aom_codec_enc_cfg::g_error_resilient variable.
 */
typedef uint32_t aom_codec_er_flags_t;
/*!\brief Improve resiliency against losses of whole frames */
#define AOM_ERROR_RESILIENT_DEFAULT 0x1

/*!\brief Encoder output packet variants
 *
 * This enumeration lists the different kinds of data packets that can be
 * returned by calls to aom_codec_get_cx_data(). Algorithms \ref MAY
 * extend this list to provide additional functionality.
 */
enum aom_codec_cx_pkt_kind {
  AOM_CODEC_CX_FRAME_PKT,    /**< Compressed video frame */
  AOM_CODEC_STATS_PKT,       /**< Two-pass statistics for this frame */
  AOM_CODEC_FPMB_STATS_PKT,  /**< first pass mb statistics for this frame */
  AOM_CODEC_PSNR_PKT,        /**< PSNR statistics for this frame */
  AOM_CODEC_CUSTOM_PKT = 256 /**< Algorithm extensions  */
};

/*!\brief Encoder output packet
 *
 * This structure contains the different kinds of output data the encoder
 * may produce while compressing a frame.
 */
typedef struct aom_codec_cx_pkt {
  enum aom_codec_cx_pkt_kind kind; /**< packet variant */
  union {
    struct {
      void *buf; /**< compressed data buffer */
      size_t sz; /**< length of compressed data */
      /*!\brief time stamp to show frame (in timebase units) */
      aom_codec_pts_t pts;
      /*!\brief duration to show frame (in timebase units) */
      unsigned long duration;
      aom_codec_frame_flags_t flags; /**< flags for this frame */
      /*!\brief the partition id defines the decoding order of the partitions.
       * Only applicable when "output partition" mode is enabled. First
       * partition has id 0.*/
      int partition_id;
      /*!\brief size of the visible frame in this packet */
      size_t vis_frame_size;
    } frame;                            /**< data for compressed frame packet */
    aom_fixed_buf_t twopass_stats;      /**< data for two-pass packet */
    aom_fixed_buf_t firstpass_mb_stats; /**< first pass mb packet */
    struct aom_psnr_pkt {
      unsigned int samples[4]; /**< Number of samples, total/y/u/v */
      uint64_t sse[4];         /**< sum squared error, total/y/u/v */
      double psnr[4];          /**< PSNR, total/y/u/v */
      /*!\brief Number of samples, total/y/u/v when
       * input bit-depth < stream bit-depth.*/
      unsigned int samples_hbd[4];
      /*!\brief sum squared error, total/y/u/v when
       * input bit-depth < stream bit-depth.*/
      uint64_t sse_hbd[4];
      /*!\brief PSNR, total/y/u/v when
       * input bit-depth < stream bit-depth.*/
      double psnr_hbd[4];
    } psnr;              /**< data for PSNR packet */
    aom_fixed_buf_t raw; /**< data for arbitrary packets */
  } data;                /**< packet data */
} aom_codec_cx_pkt_t;    /**< alias for struct aom_codec_cx_pkt */

/*!\brief Rational Number
 *
 * This structure holds a fractional value.
 */
typedef struct aom_rational {
  int num;        /**< fraction numerator */
  int den;        /**< fraction denominator */
} aom_rational_t; /**< alias for struct aom_rational */

/*!\brief Multi-pass Encoding Pass
 *
 * AOM_RC_LAST_PASS is kept for backward compatibility.
 * If passes is not given and pass==2, the codec will assume passes=2.
 * For new code, it is recommended to use AOM_RC_SECOND_PASS and set
 * the "passes" member to 2 via the key & val API for two-pass encoding.
 */
enum aom_enc_pass {
  AOM_RC_ONE_PASS = 0,    /**< Single pass mode */
  AOM_RC_FIRST_PASS = 1,  /**< First pass of multi-pass mode */
  AOM_RC_SECOND_PASS = 2, /**< Second pass of multi-pass mode */
  AOM_RC_THIRD_PASS = 3,  /**< Third pass of multi-pass mode */
  AOM_RC_LAST_PASS = 2,   /**< Final pass of two-pass mode */
};

/*!\brief Rate control mode */
enum aom_rc_mode {
  AOM_VBR, /**< Variable Bit Rate (VBR) mode */
  AOM_CBR, /**< Constant Bit Rate (CBR) mode */
  AOM_CQ,  /**< Constrained Quality (CQ)  mode */
  AOM_Q,   /**< Constant Quality (Q) mode */
};

/*!\brief Keyframe placement mode.
 *
 * This enumeration determines whether keyframes are placed automatically by
 * the encoder or whether this behavior is disabled. Older releases of this
 * SDK were implemented such that AOM_KF_FIXED meant keyframes were disabled.
 * This name is confusing for this behavior, so the new symbols to be used
 * are AOM_KF_AUTO and AOM_KF_DISABLED.
 */
enum aom_kf_mode {
  AOM_KF_FIXED,       /**< deprecated, implies AOM_KF_DISABLED */
  AOM_KF_AUTO,        /**< Encoder determines optimal placement automatically */
  AOM_KF_DISABLED = 0 /**< Encoder does not place keyframes. */
};

/*!\brief Frame super-resolution mode. */
typedef enum {
  /**< Frame super-resolution is disabled for all frames. */
  AOM_SUPERRES_NONE,
  /**< All frames are coded at the specified scale and super-resolved. */
  AOM_SUPERRES_FIXED,
  /**< All frames are coded at a random scale and super-resolved. */
  AOM_SUPERRES_RANDOM,
  /**< Super-resolution scale for each frame is determined based on the q index
     of that frame. */
  AOM_SUPERRES_QTHRESH,
  /**< Full-resolution or super-resolution and the scale (in case of
     super-resolution) are automatically selected for each frame. */
  AOM_SUPERRES_AUTO,
} aom_superres_mode;

/*!\brief Encoder Config Options
 *
 * This type allows to enumerate and control flags defined for encoder control
 * via config file at runtime.
 */
typedef struct cfg_options {
  /*!\brief Indicate init by cfg file
   * 0 or 1
   */
  unsigned int init_by_cfg_file;
  /*!\brief Superblock size
   * 0, 64 or 128
   */
  unsigned int super_block_size;
  /*!\brief max partition size
   * 8, 16, 32, 64, 128
   */
  unsigned int max_partition_size;
  /*!\brief min partition size
   * 8, 16, 32, 64, 128
   */
  unsigned int min_partition_size;
  /*!\brief disable AB Shape partition type
   *
   */
  unsigned int disable_ab_partition_type;
  /*!\brief disable rectangular partition type
   *
   */
  unsigned int disable_rect_partition_type;
  /*!\brief disable 1:4/4:1 partition type
   *
   */
  unsigned int disable_1to4_partition_type;
  /*!\brief disable flip and identity transform type
   *
   */
  unsigned int disable_flip_idtx;
  /*!\brief disable CDEF filter
   *
   */
  unsigned int disable_cdef;
  /*!\brief disable Loop Restoration Filter
   *
   */
  unsigned int disable_lr;
  /*!\brief disable OBMC
   *
   */
  unsigned int disable_obmc;
  /*!\brief disable Warped Motion
   *
   */
  unsigned int disable_warp_motion;
  /*!\brief disable global motion
   *
   */
  unsigned int disable_global_motion;
  /*!\brief disable dist weighted compound
   *
   */
  unsigned int disable_dist_wtd_comp;
  /*!\brief disable diff weighted compound
   *
   */
  unsigned int disable_diff_wtd_comp;
  /*!\brief disable inter/intra compound
   *
   */
  unsigned int disable_inter_intra_comp;
  /*!\brief disable masked compound
   *
   */
  unsigned int disable_masked_comp;
  /*!\brief disable one sided compound
   *
   */
  unsigned int disable_one_sided_comp;
  /*!\brief disable Palette
   *
   */
  unsigned int disable_palette;
  /*!\brief disable Intra Block Copy
   *
   */
  unsigned int disable_intrabc;
  /*!\brief disable chroma from luma
   *
   */
  unsigned int disable_cfl;
  /*!\brief disable intra smooth mode
   *
   */
  unsigned int disable_smooth_intra;
  /*!\brief disable filter intra
   *
   */
  unsigned int disable_filter_intra;
  /*!\brief disable dual filter
   *
   */
  unsigned int disable_dual_filter;
  /*!\brief disable intra angle delta
   *
   */
  unsigned int disable_intra_angle_delta;
  /*!\brief disable intra edge filter
   *
   */
  unsigned int disable_intra_edge_filter;
  /*!\brief disable 64x64 transform
   *
   */
  unsigned int disable_tx_64x64;
  /*!\brief disable smooth inter/intra
   *
   */
  unsigned int disable_smooth_inter_intra;
  /*!\brief disable inter/inter wedge comp
   *
   */
  unsigned int disable_inter_inter_wedge;
  /*!\brief disable inter/intra wedge comp
   *
   */
  unsigned int disable_inter_intra_wedge;
  /*!\brief disable paeth intra
   *
   */
  unsigned int disable_paeth_intra;
  /*!\brief disable trellis quantization
   *
   */
  unsigned int disable_trellis_quant;
  /*!\brief disable ref frame MV
   *
   */
  unsigned int disable_ref_frame_mv;
  /*!\brief use reduced reference frame set
   *
   */
  unsigned int reduced_reference_set;
  /*!\brief use reduced transform type set
   *
   */
  unsigned int reduced_tx_type_set;
} cfg_options_t;

/*!\brief Encoded Frame Flags
 *
 * This type indicates a bitfield to be passed to aom_codec_encode(), defining
 * per-frame boolean values. By convention, bits common to all codecs will be
 * named AOM_EFLAG_*, and bits specific to an algorithm will be named
 * /algo/_eflag_*. The lower order 16 bits are reserved for common use.
 */
typedef long aom_enc_frame_flags_t;
/*!\brief Force this frame to be a keyframe */
#define AOM_EFLAG_FORCE_KF (1 << 0)

/*!\brief Encoder configuration structure
 *
 * This structure contains the encoder settings that have common representations
 * across all codecs. This doesn't imply that all codecs support all features,
 * however.
 */
typedef struct aom_codec_enc_cfg {
  /*
   * generic settings (g)
   */

  /*!\brief Algorithm specific "usage" value
   *
   * Algorithms may define multiple values for usage, which may convey the
   * intent of how the application intends to use the stream. If this value
   * is non-zero, consult the documentation for the codec to determine its
   * meaning.
   */
  unsigned int g_usage;

  /*!\brief Maximum number of threads to use
   *
   * For multi-threaded implementations, use no more than this number of
   * threads. The codec may use fewer threads than allowed. The value
   * 0 is equivalent to the value 1.
   */
  unsigned int g_threads;

  /*!\brief Bitstream profile to use
   *
   * Some codecs support a notion of multiple bitstream profiles. Typically
   * this maps to a set of features that are turned on or off. Often the
   * profile to use is determined by the features of the intended decoder.
   * Consult the documentation for the codec to determine the valid values
   * for this parameter, or set to zero for a sane default.
   */
  unsigned int g_profile; /**< profile of bitstream to use */

  /*!\brief Width of the frame
   *
   * This value identifies the presentation resolution of the frame,
   * in pixels. Note that the frames passed as input to the encoder must
   * have this resolution. Frames will be presented by the decoder in this
   * resolution, independent of any spatial resampling the encoder may do.
   */
  unsigned int g_w;

  /*!\brief Height of the frame
   *
   * This value identifies the presentation resolution of the frame,
   * in pixels. Note that the frames passed as input to the encoder must
   * have this resolution. Frames will be presented by the decoder in this
   * resolution, independent of any spatial resampling the encoder may do.
   */
  unsigned int g_h;

  /*!\brief Max number of frames to encode
   *
   * If force video mode is off (the default) and g_limit is 1, the encoder
   * will encode a still picture (still_picture is set to 1 in the sequence
   * header OBU). If in addition full_still_picture_hdr is 0 (the default),
   * the encoder will use a reduced header (reduced_still_picture_header is
   * set to 1 in the sequence header OBU) for the still picture.
   */
  unsigned int g_limit;

  /*!\brief Forced maximum width of the frame
   *
   * If this value is non-zero then it is used to force the maximum frame
   * width written in write_sequence_header().
   */
  unsigned int g_forced_max_frame_width;

  /*!\brief Forced maximum height of the frame
   *
   * If this value is non-zero then it is used to force the maximum frame
   * height written in write_sequence_header().
   */
  unsigned int g_forced_max_frame_height;

  /*!\brief Bit-depth of the codec
   *
   * This value identifies the bit_depth of the codec,
   * Only certain bit-depths are supported as identified in the
   * aom_bit_depth_t enum.
   */
  aom_bit_depth_t g_bit_depth;

  /*!\brief Bit-depth of the input frames
   *
   * This value identifies the bit_depth of the input frames in bits.
   * Note that the frames passed as input to the encoder must have
   * this bit-depth.
   */
  unsigned int g_input_bit_depth;

  /*!\brief Stream timebase units
   *
   * Indicates the smallest interval of time, in seconds, used by the stream.
   * For fixed frame rate material, or variable frame rate material where
   * frames are timed at a multiple of a given clock (ex: video capture),
   * the \ref RECOMMENDED method is to set the timebase to the reciprocal
   * of the frame rate (ex: 1001/30000 for 29.970 Hz NTSC). This allows the
   * pts to correspond to the frame number, which can be handy. For
   * re-encoding video from containers with absolute time timestamps, the
   * \ref RECOMMENDED method is to set the timebase to that of the parent
   * container or multimedia framework (ex: 1/1000 for ms, as in FLV).
   */
  struct aom_rational g_timebase;

  /*!\brief Enable error resilient modes.
   *
   * The error resilient bitfield indicates to the encoder which features
   * it should enable to take measures for streaming over lossy or noisy
   * links.
   */
  aom_codec_er_flags_t g_error_resilient;

  /*!\brief Multi-pass Encoding Mode
   *
   * This value should be set to the current phase for multi-pass encoding.
   * For single pass, set to #AOM_RC_ONE_PASS.
   */
  enum aom_enc_pass g_pass;

  /*!\brief Allow lagged encoding
   *
   * If set, this value allows the encoder to consume a number of input
   * frames before producing output frames. This allows the encoder to
   * base decisions for the current frame on future frames. This does
   * increase the latency of the encoding pipeline, so it is not appropriate
   * in all situations (ex: realtime encoding).
   *
   * Note that this is a maximum value -- the encoder may produce frames
   * sooner than the given limit. Set this value to 0 to disable this
   * feature.
   */
  unsigned int g_lag_in_frames;

  /*
   * rate control settings (rc)
   */

  /*!\brief Temporal resampling configuration, if supported by the codec.
   *
   * Temporal resampling allows the codec to "drop" frames as a strategy to
   * meet its target data rate. This can cause temporal discontinuities in
   * the encoded video, which may appear as stuttering during playback. This
   * trade-off is often acceptable, but for many applications is not. It can
   * be disabled in these cases.
   *
   * Note that not all codecs support this feature. All aom AVx codecs do.
   * For other codecs, consult the documentation for that algorithm.
   *
   * This threshold is described as a percentage of the target data buffer.
   * When the data buffer falls below this percentage of fullness, a
   * dropped frame is indicated. Set the threshold to zero (0) to disable
   * this feature.
   */
  unsigned int rc_dropframe_thresh;

  /*!\brief Mode for spatial resampling, if supported by the codec.
   *
   * Spatial resampling allows the codec to compress a lower resolution
   * version of the frame, which is then upscaled by the decoder to the
   * correct presentation resolution. This increases visual quality at
   * low data rates, at the expense of CPU time on the encoder/decoder.
   */
  unsigned int rc_resize_mode;

  /*!\brief Frame resize denominator.
   *
   * The denominator for resize to use, assuming 8 as the numerator.
   *
   * Valid denominators are  8 - 16 for now.
   */
  unsigned int rc_resize_denominator;

  /*!\brief Keyframe resize denominator.
   *
   * The denominator for resize to use, assuming 8 as the numerator.
   *
   * Valid denominators are  8 - 16 for now.
   */
  unsigned int rc_resize_kf_denominator;

  /*!\brief Frame super-resolution scaling mode.
   *
   * Similar to spatial resampling, frame super-resolution integrates
   * upscaling after the encode/decode process. Taking control of upscaling and
   * using restoration filters should allow it to outperform normal resizing.
   */
  aom_superres_mode rc_superres_mode;

  /*!\brief Frame super-resolution denominator.
   *
   * The denominator for superres to use. If fixed it will only change if the
   * cumulative scale change over resizing and superres is greater than 1/2;
   * this forces superres to reduce scaling.
   *
   * Valid denominators are 8 to 16.
   *
   * Used only by AOM_SUPERRES_FIXED.
   */
  unsigned int rc_superres_denominator;

  /*!\brief Keyframe super-resolution denominator.
   *
   * The denominator for superres to use. If fixed it will only change if the
   * cumulative scale change over resizing and superres is greater than 1/2;
   * this forces superres to reduce scaling.
   *
   * Valid denominators are 8 - 16 for now.
   */
  unsigned int rc_superres_kf_denominator;

  /*!\brief Frame super-resolution q threshold.
   *
   * The q level threshold after which superres is used.
   * Valid values are 1 to 63.
   *
   * Used only by AOM_SUPERRES_QTHRESH
   */
  unsigned int rc_superres_qthresh;

  /*!\brief Keyframe super-resolution q threshold.
   *
   * The q level threshold after which superres is used for key frames.
   * Valid values are 1 to 63.
   *
   * Used only by AOM_SUPERRES_QTHRESH
   */
  unsigned int rc_superres_kf_qthresh;

  /*!\brief Rate control algorithm to use.
   *
   * Indicates whether the end usage of this stream is to be streamed over
   * a bandwidth constrained link, indicating that Constant Bit Rate (CBR)
   * mode should be used, or whether it will be played back on a high
   * bandwidth link, as from a local disk, where higher variations in
   * bitrate are acceptable.
   */
  enum aom_rc_mode rc_end_usage;

  /*!\brief Two-pass stats buffer.
   *
   * A buffer containing all of the stats packets produced in the first
   * pass, concatenated.
   */
  aom_fixed_buf_t rc_twopass_stats_in;

  /*!\brief first pass mb stats buffer.
   *
   * A buffer containing all of the first pass mb stats packets produced
   * in the first pass, concatenated.
   */
  aom_fixed_buf_t rc_firstpass_mb_stats_in;

  /*!\brief Target data rate
   *
   * Target bitrate to use for this stream, in kilobits per second.
   * Max allowed value is 2000000
   */
  unsigned int rc_target_bitrate;

  /*
   * quantizer settings
   */

  /*!\brief Minimum (Best Quality) Quantizer
   *
   * The quantizer is the most direct control over the quality of the
   * encoded image. The range of valid values for the quantizer is codec
   * specific. Consult the documentation for the codec to determine the
   * values to use. To determine the range programmatically, call
   * aom_codec_enc_config_default() with a usage value of 0.
   */
  unsigned int rc_min_quantizer;

  /*!\brief Maximum (Worst Quality) Quantizer
   *
   * The quantizer is the most direct control over the quality of the
   * encoded image. The range of valid values for the quantizer is codec
   * specific. Consult the documentation for the codec to determine the
   * values to use. To determine the range programmatically, call
   * aom_codec_enc_config_default() with a usage value of 0.
   */
  unsigned int rc_max_quantizer;

  /*
   * bitrate tolerance
   */

  /*!\brief Rate control adaptation undershoot control
   *
   * This value, controls the tolerance of the VBR algorithm to undershoot
   * and is used as a trigger threshold for more aggressive adaptation of Q.
   *
   * Valid values in the range 0-100.
   */
  unsigned int rc_undershoot_pct;

  /*!\brief Rate control adaptation overshoot control
   *
   * This value, controls the tolerance of the VBR algorithm to overshoot
   * and is used as a trigger threshold for more aggressive adaptation of Q.
   *
   * Valid values in the range 0-100.
   */
  unsigned int rc_overshoot_pct;

  /*
   * decoder buffer model parameters
   */

  /*!\brief Decoder Buffer Size
   *
   * This value indicates the amount of data that may be buffered by the
   * decoding application. Note that this value is expressed in units of
   * time (milliseconds). For example, a value of 5000 indicates that the
   * client will buffer (at least) 5000ms worth of encoded data. Use the
   * target bitrate (#rc_target_bitrate) to convert to bits/bytes, if
   * necessary.
   */
  unsigned int rc_buf_sz;

  /*!\brief Decoder Buffer Initial Size
   *
   * This value indicates the amount of data that will be buffered by the
   * decoding application prior to beginning playback. This value is
   * expressed in units of time (milliseconds). Use the target bitrate
   * (#rc_target_bitrate) to convert to bits/bytes, if necessary.
   */
  unsigned int rc_buf_initial_sz;

  /*!\brief Decoder Buffer Optimal Size
   *
   * This value indicates the amount of data that the encoder should try
   * to maintain in the decoder's buffer. This value is expressed in units
   * of time (milliseconds). Use the target bitrate (#rc_target_bitrate)
   * to convert to bits/bytes, if necessary.
   */
  unsigned int rc_buf_optimal_sz;

  /*
   * 2 pass rate control parameters
   */

  /*!\brief Two-pass mode CBR/VBR bias
   *
   * Bias, expressed on a scale of 0 to 100, for determining target size
   * for the current frame. The value 0 indicates the optimal CBR mode
   * value should be used. The value 100 indicates the optimal VBR mode
   * value should be used. Values in between indicate which way the
   * encoder should "lean."
   */
  unsigned int rc_2pass_vbr_bias_pct;

  /*!\brief Two-pass mode per-GOP minimum bitrate
   *
   * This value, expressed as a percentage of the target bitrate, indicates
   * the minimum bitrate to be used for a single GOP (aka "section")
   */
  unsigned int rc_2pass_vbr_minsection_pct;

  /*!\brief Two-pass mode per-GOP maximum bitrate
   *
   * This value, expressed as a percentage of the target bitrate, indicates
   * the maximum bitrate to be used for a single GOP (aka "section")
   */
  unsigned int rc_2pass_vbr_maxsection_pct;

  /*
   * keyframing settings (kf)
   */

  /*!\brief Option to enable forward reference key frame
   *
   */
  int fwd_kf_enabled;

  /*!\brief Keyframe placement mode
   *
   * This value indicates whether the encoder should place keyframes at a
   * fixed interval, or determine the optimal placement automatically
   * (as governed by the #kf_min_dist and #kf_max_dist parameters)
   */
  enum aom_kf_mode kf_mode;

  /*!\brief Keyframe minimum interval
   *
   * This value, expressed as a number of frames, prevents the encoder from
   * placing a keyframe nearer than kf_min_dist to the previous keyframe. At
   * least kf_min_dist frames non-keyframes will be coded before the next
   * keyframe. Set kf_min_dist equal to kf_max_dist for a fixed interval.
   */
  unsigned int kf_min_dist;

  /*!\brief Keyframe maximum interval
   *
   * This value, expressed as a number of frames, forces the encoder to code
   * a keyframe if one has not been coded in the last kf_max_dist frames.
   * A value of 0 implies all frames will be keyframes. Set kf_min_dist
   * equal to kf_max_dist for a fixed interval.
   */
  unsigned int kf_max_dist;

  /*!\brief sframe interval
   *
   * This value, expressed as a number of frames, forces the encoder to code
   * an S-Frame every sframe_dist frames.
   */
  unsigned int sframe_dist;

  /*!\brief sframe insertion mode
   *
   * This value must be set to 1 or 2, and tells the encoder how to insert
   * S-Frames. It will only have an effect if sframe_dist != 0.
   *
   * If altref is enabled:
   *   - if sframe_mode == 1, the considered frame will be made into an
   *     S-Frame only if it is an altref frame
   *   - if sframe_mode == 2, the next altref frame will be made into an
   *     S-Frame.
   *
   * Otherwise: the considered frame will be made into an S-Frame.
   *
   * \attention Not implemented.
   */
  unsigned int sframe_mode;

  /*!\brief Tile coding mode
   *
   * This value indicates the tile coding mode.
   * A value of 0 implies a normal non-large-scale tile coding. A value of 1
   * implies a large-scale tile coding.
   */
  unsigned int large_scale_tile;

  /*!\brief Monochrome mode
   *
   * If this is nonzero, the encoder will generate a monochrome stream
   * with no chroma planes.
   */
  unsigned int monochrome;

  /*!\brief full_still_picture_hdr
   *
   * If this is nonzero, the encoder will generate a full header
   * (reduced_still_picture_header is set to 0 in the sequence header OBU) even
   * for still picture encoding. If this is zero (the default), a reduced
   * header (reduced_still_picture_header is set to 1 in the sequence header
   * OBU) is used for still picture encoding. This flag has no effect when a
   * regular video with more than a single frame is encoded.
   */
  unsigned int full_still_picture_hdr;

  /*!\brief Bitstream syntax mode
   *
   * This value indicates the bitstream syntax mode.
   * A value of 0 indicates bitstream is saved as Section 5 bitstream. A value
   * of 1 indicates the bitstream is saved in Annex-B format
   */
  unsigned int save_as_annexb;

  /*!\brief Number of explicit tile widths specified
   *
   * This value indicates the number of tile widths specified
   * A value of 0 implies no tile widths are specified.
   * Tile widths are given in the array tile_widths[]
   */
  int tile_width_count;

  /*!\brief Number of explicit tile heights specified
   *
   * This value indicates the number of tile heights specified
   * A value of 0 implies no tile heights are specified.
   * Tile heights are given in the array tile_heights[]
   */
  int tile_height_count;

/*!\brief Maximum number of tile widths in tile widths array
 *
 * This define gives the maximum number of elements in the tile_widths array.
 */
#define MAX_TILE_WIDTHS 64  // maximum tile width array length

  /*!\brief Array of specified tile widths
   *
   * This array specifies tile widths (and may be empty)
   * The number of widths specified is given by tile_width_count
   */
  int tile_widths[MAX_TILE_WIDTHS];

/*!\brief Maximum number of tile heights in tile heights array.
 *
 * This define gives the maximum number of elements in the tile_heights array.
 */
#define MAX_TILE_HEIGHTS 64  // maximum tile height array length

  /*!\brief Array of specified tile heights
   *
   * This array specifies tile heights (and may be empty)
   * The number of heights specified is given by tile_height_count
   */
  int tile_heights[MAX_TILE_HEIGHTS];

  /*!\brief Whether encoder should use fixed QP offsets.
   *
   * If a value of 1 is provided, encoder will use fixed QP offsets for frames
   * at different levels of the pyramid.
   * If a value of 0 is provided, encoder will NOT use fixed QP offsets.
   * Note: This option is only relevant for --end-usage=q.
   */
  unsigned int use_fixed_qp_offsets;

  /*!\brief Deprecated and ignored. DO NOT USE.
   *
   * TODO(aomedia:3269): Remove fixed_qp_offsets in libaom v4.0.0.
   */
  int fixed_qp_offsets[5];

  /*!\brief Options defined per config file
   *
   */
  cfg_options_t encoder_cfg;
} aom_codec_enc_cfg_t; /**< alias for struct aom_codec_enc_cfg */

/*!\brief Initialize an encoder instance
 *
 * Initializes an encoder context using the given interface. Applications
 * should call the aom_codec_enc_init convenience macro instead of this
 * function directly, to ensure that the ABI version number parameter
 * is properly initialized.
 *
 * If the library was configured with -DCONFIG_MULTITHREAD=0, this call
 * is not thread safe and should be guarded with a lock if being used
 * in a multithreaded context.
 *
 * If aom_codec_enc_init_ver() fails, it is not necessary to call
 * aom_codec_destroy() on the encoder context.
 *
 * \param[in]    ctx     Pointer to this instance's context.
 * \param[in]    iface   Pointer to the algorithm interface to use.
 * \param[in]    cfg     Configuration to use, if known.
 * \param[in]    flags   Bitfield of AOM_CODEC_USE_* flags
 * \param[in]    ver     ABI version number. Must be set to
 *                       AOM_ENCODER_ABI_VERSION
 * \retval #AOM_CODEC_OK
 *     The encoder algorithm has been initialized.
 * \retval #AOM_CODEC_MEM_ERROR
 *     Memory allocation failed.
 */
aom_codec_err_t aom_codec_enc_init_ver(aom_codec_ctx_t *ctx,
                                       aom_codec_iface_t *iface,
                                       const aom_codec_enc_cfg_t *cfg,
                                       aom_codec_flags_t flags, int ver);

/*!\brief Convenience macro for aom_codec_enc_init_ver()
 *
 * Ensures the ABI version parameter is properly set.
 */
#define aom_codec_enc_init(ctx, iface, cfg, flags) \
  aom_codec_enc_init_ver(ctx, iface, cfg, flags, AOM_ENCODER_ABI_VERSION)

/*!\brief Get the default configuration for a usage.
 *
 * Initializes an encoder configuration structure with default values. Supports
 * the notion of "usages" so that an algorithm may offer different default
 * settings depending on the user's intended goal. This function \ref SHOULD
 * be called by all applications to initialize the configuration structure
 * before specializing the configuration with application specific values.
 *
 * \param[in]    iface     Pointer to the algorithm interface to use.
 * \param[out]   cfg       Configuration buffer to populate.
 * \param[in]    usage     Algorithm specific usage value. For AV1, must be
 *                         set to AOM_USAGE_GOOD_QUALITY (0),
 *                         AOM_USAGE_REALTIME (1), or AOM_USAGE_ALL_INTRA (2).
 *
 * \retval #AOM_CODEC_OK
 *     The configuration was populated.
 * \retval #AOM_CODEC_INCAPABLE
 *     Interface is not an encoder interface.
 * \retval #AOM_CODEC_INVALID_PARAM
 *     A parameter was NULL, or the usage value was not recognized.
 */
aom_codec_err_t aom_codec_enc_config_default(aom_codec_iface_t *iface,
                                             aom_codec_enc_cfg_t *cfg,
                                             unsigned int usage);

/*!\brief Set or change configuration
 *
 * Reconfigures an encoder instance according to the given configuration.
 *
 * \param[in]    ctx     Pointer to this instance's context
 * \param[in]    cfg     Configuration buffer to use
 *
 * \retval #AOM_CODEC_OK
 *     The configuration was populated.
 * \retval #AOM_CODEC_INCAPABLE
 *     Interface is not an encoder interface.
 * \retval #AOM_CODEC_INVALID_PARAM
 *     A parameter was NULL, or the usage value was not recognized.
 */
aom_codec_err_t aom_codec_enc_config_set(aom_codec_ctx_t *ctx,
                                         const aom_codec_enc_cfg_t *cfg);

/*!\brief Get global stream headers
 *
 * Retrieves a stream level global header packet, if supported by the codec.
 * Calls to this function should be deferred until all configuration information
 * has been passed to libaom. Otherwise the global header data may be
 * invalidated by additional configuration changes.
 *
 * The AV1 implementation of this function returns an OBU. The OBU returned is
 * in Low Overhead Bitstream Format. Specifically, the obu_has_size_field bit is
 * set, and the buffer contains the obu_size field for the returned OBU.
 *
 * \param[in]    ctx     Pointer to this instance's context
 *
 * \retval NULL
 *     Encoder does not support global header, or an error occurred while
 *     generating the global header.
 *
 * \retval Non-NULL
 *     Pointer to buffer containing global header packet. The caller owns the
 *     memory associated with this buffer, and must free the 'buf' member of the
 *     aom_fixed_buf_t as well as the aom_fixed_buf_t pointer. Memory returned
 *     must be freed via call to free().
 */
aom_fixed_buf_t *aom_codec_get_global_headers(aom_codec_ctx_t *ctx);

/*!\brief usage parameter analogous to AV1 GOOD QUALITY mode. */
#define AOM_USAGE_GOOD_QUALITY 0u
/*!\brief usage parameter analogous to AV1 REALTIME mode. */
#define AOM_USAGE_REALTIME 1u
/*!\brief usage parameter analogous to AV1 all intra mode. */
#define AOM_USAGE_ALL_INTRA 2u

/*!\brief Encode a frame
 *
 * Encodes a video frame at the given "presentation time." The presentation
 * time stamp (PTS) \ref MUST be strictly increasing.
 *
 * When the last frame has been passed to the encoder, this function should
 * continue to be called in a loop, with the img parameter set to NULL. This
 * will signal the end-of-stream condition to the encoder and allow it to
 * encode any held buffers. Encoding is complete when aom_codec_encode() is
 * called with img set to NULL and aom_codec_get_cx_data() returns no data.
 *
 * \param[in]    ctx       Pointer to this instance's context
 * \param[in]    img       Image data to encode, NULL to flush.
 *                         Encoding sample values outside the range
 *                         [0..(1<<img->bit_depth)-1] is undefined behavior.
 *                         Note: Although img is declared as a const pointer,
 *                         if AV1E_SET_DENOISE_NOISE_LEVEL is set to a nonzero
 *                         value aom_codec_encode() modifies (denoises) the
 *                         samples in img->planes[i] .
 * \param[in]    pts       Presentation time stamp, in timebase units. If img
 *                         is NULL, pts is ignored.
 * \param[in]    duration  Duration to show frame, in timebase units. If img
 *                         is not NULL, duration must be nonzero. If img is
 *                         NULL, duration is ignored.
 * \param[in]    flags     Flags to use for encoding this frame.
 *
 * \retval #AOM_CODEC_OK
 *     The configuration was populated.
 * \retval #AOM_CODEC_INCAPABLE
 *     Interface is not an encoder interface.
 * \retval #AOM_CODEC_INVALID_PARAM
 *     A parameter was NULL, the image format is unsupported, etc.
 *
 * \note
 * `duration` is of the unsigned long type, which can be 32 or 64 bits.
 * `duration` must be less than or equal to UINT32_MAX so that its range is
 * independent of the size of unsigned long.
 */
aom_codec_err_t aom_codec_encode(aom_codec_ctx_t *ctx, const aom_image_t *img,
                                 aom_codec_pts_t pts, unsigned long duration,
                                 aom_enc_frame_flags_t flags);

/*!\brief Set compressed data output buffer
 *
 * Sets the buffer that the codec should output the compressed data
 * into. This call effectively sets the buffer pointer returned in the
 * next AOM_CODEC_CX_FRAME_PKT packet. Subsequent packets will be
 * appended into this buffer. The buffer is preserved across frames,
 * so applications must periodically call this function after flushing
 * the accumulated compressed data to disk or to the network to reset
 * the pointer to the buffer's head.
 *
 * `pad_before` bytes will be skipped before writing the compressed
 * data, and `pad_after` bytes will be appended to the packet. The size
 * of the packet will be the sum of the size of the actual compressed
 * data, pad_before, and pad_after. The padding bytes will be preserved
 * (not overwritten).
 *
 * Note that calling this function does not guarantee that the returned
 * compressed data will be placed into the specified buffer. In the
 * event that the encoded data will not fit into the buffer provided,
 * the returned packet \ref MAY point to an internal buffer, as it would
 * if this call were never used. In this event, the output packet will
 * NOT have any padding, and the application must free space and copy it
 * to the proper place. This is of particular note in configurations
 * that may output multiple packets for a single encoded frame (e.g., lagged
 * encoding) or if the application does not reset the buffer periodically.
 *
 * Applications may restore the default behavior of the codec providing
 * the compressed data buffer by calling this function with a NULL
 * buffer.
 *
 * Applications \ref MUSTNOT call this function during iteration of
 * aom_codec_get_cx_data().
 *
 * \param[in]    ctx         Pointer to this instance's context
 * \param[in]    buf         Buffer to store compressed data into
 * \param[in]    pad_before  Bytes to skip before writing compressed data
 * \param[in]    pad_after   Bytes to skip after writing compressed data
 *
 * \retval #AOM_CODEC_OK
 *     The buffer was set successfully.
 * \retval #AOM_CODEC_INVALID_PARAM
 *     A parameter was NULL, the image format is unsupported, etc.
 */
aom_codec_err_t aom_codec_set_cx_data_buf(aom_codec_ctx_t *ctx,
                                          const aom_fixed_buf_t *buf,
                                          unsigned int pad_before,
                                          unsigned int pad_after);

/*!\brief Encoded data iterator
 *
 * Iterates over a list of data packets to be passed from the encoder to the
 * application. The different kinds of packets available are enumerated in
 * #aom_codec_cx_pkt_kind.
 *
 * #AOM_CODEC_CX_FRAME_PKT packets should be passed to the application's
 * muxer. Multiple compressed frames may be in the list.
 * #AOM_CODEC_STATS_PKT packets should be appended to a global buffer.
 *
 * The application \ref MUST silently ignore any packet kinds that it does
 * not recognize or support.
 *
 * The data buffers returned from this function are only guaranteed to be
 * valid until the application makes another call to any aom_codec_* function.
 *
 * \param[in]     ctx      Pointer to this instance's context
 * \param[in,out] iter     Iterator storage, initialized to NULL
 *
 * \return Returns a pointer to an output data packet (compressed frame data,
 *         two-pass statistics, etc.) or NULL to signal end-of-list.
 *
 */
const aom_codec_cx_pkt_t *aom_codec_get_cx_data(aom_codec_ctx_t *ctx,
                                                aom_codec_iter_t *iter);

/*!\brief Get Preview Frame
 *
 * Returns an image that can be used as a preview. Shows the image as it would
 * exist at the decompressor. The application \ref MUST NOT write into this
 * image buffer.
 *
 * \param[in]     ctx      Pointer to this instance's context
 *
 * \return Returns a pointer to a preview image, or NULL if no image is
 *         available.
 *
 */
const aom_image_t *aom_codec_get_preview_frame(aom_codec_ctx_t *ctx);

/*!@} - end defgroup encoder*/
#ifdef __cplusplus
}
#endif
#endif  // AOM_AOM_AOM_ENCODER_H_
