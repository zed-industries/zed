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

#ifndef AOM_AV1_ENCODER_SVC_LAYERCONTEXT_H_
#define AOM_AV1_ENCODER_SVC_LAYERCONTEXT_H_

#include "aom_scale/yv12config.h"
#include "av1/encoder/aq_cyclicrefresh.h"
#include "av1/encoder/encoder.h"
#include "av1/encoder/ratectrl.h"

#ifdef __cplusplus
extern "C" {
#endif

/*!
 * \brief The stucture of quantities related to each spatial and temporal layer.
 * \ingroup SVC
 */
typedef struct {
  /*!\cond */
  RATE_CONTROL rc;
  PRIMARY_RATE_CONTROL p_rc;
  int framerate_factor;
  int64_t layer_target_bitrate;  // In bits per second.
  int scaling_factor_num;
  int scaling_factor_den;
  int64_t target_bandwidth;
  int64_t spatial_layer_target_bandwidth;
  double framerate;
  int avg_frame_size;
  int max_q;
  int min_q;
  int frames_from_key_frame;
  /*!\endcond */

  /*!
   * Cyclic refresh parameters (aq-mode=3), that need to be updated per-frame.
   */
  int sb_index;
  /*!
   * Segmentation map
   */
  int8_t *map;
  /*!
   * Number of blocks on segment 1
   */
  int actual_num_seg1_blocks;

  /*!
   * Number of blocks on segment 2
   */
  int actual_num_seg2_blocks;
  /*!
   * Counter used to detect scene change.
   */
  int counter_encode_maxq_scene_change;

  /*!
   * Speed settings for each layer.
   */
  uint8_t speed;
  /*!
   * GF group index.
   */
  unsigned char group_index;
  /*!
   * If current layer is key frame.
   */
  int is_key_frame;
  /*!
   * Maximum motion magnitude of previous encoded layer.
   */
  int max_mv_magnitude;
} LAYER_CONTEXT;

/*!
 * \brief The stucture of SVC.
 * \ingroup SVC
 */
typedef struct SVC {
  /*!\cond */
  int spatial_layer_id;
  int temporal_layer_id;
  int number_spatial_layers;
  int number_temporal_layers;
  int prev_number_spatial_layers;
  int prev_number_temporal_layers;
  int use_flexible_mode;
  int ksvc_fixed_mode;
  /*!\endcond */

  /*!\cond */
  double base_framerate;
  unsigned int current_superframe;
  int skip_mvsearch_last;
  int skip_mvsearch_gf;
  int skip_mvsearch_altref;
  int spatial_layer_fb[REF_FRAMES];
  int temporal_layer_fb[REF_FRAMES];
  int num_encoded_top_layer;
  int first_layer_denoise;
  YV12_BUFFER_CONFIG source_last_TL0;
  int mi_cols_full_resoln;
  int mi_rows_full_resoln;
  /*!\endcond */

  /*!
   * Layer context used for rate control in CBR mode.
   * An array. The index for spatial layer `sl` and temporal layer `tl` is
   * sl * number_temporal_layers + tl.
   */
  LAYER_CONTEXT *layer_context;

  /*!
   * Number of layers allocated for layer_context. If nonzero, must be greater
   * than or equal to number_spatial_layers * number_temporal_layers.
   */
  int num_allocated_layers;

  /*!
   * EIGHTTAP_SMOOTH or BILINEAR
   */
  InterpFilter downsample_filter_type[AOM_MAX_SS_LAYERS];

  /*!
   * Downsample_filter_phase: = 0 will do sub-sampling (no weighted average),
   * = 8 will center the target pixel and get a symmetric averaging filter.
   */
  int downsample_filter_phase[AOM_MAX_SS_LAYERS];

  /*!
   * Force zero-mv in mode search for the spatial/inter-layer reference.
   */
  int force_zero_mode_spatial_ref;

  /*!
   * Flag to indicate that current spatial layer has a lower quality layer
   * (at the same timestamp) that can be used as a reference.
   * Lower quality layer refers to the same resolution but encoded at
   * different/lower bitrate.
   */
  int has_lower_quality_layer;

  /*!
   * Flag to indicate the frame drop mode for SVC: one of the two settings:
   * AOM_LAYER_DROP (default) or AOM_FULL_SUPERFRAME_DROP.
   */
  AOM_SVC_FRAME_DROP_MODE framedrop_mode;

  /*!
   * Flag to indicate if frame was dropped for a given spatial_layer_id on
   * previous superframe.
   */
  bool last_layer_dropped[AOM_MAX_SS_LAYERS];

  /*!
   * Flag to indicate if a previous spatial was dropped for the same superframe.
   */
  bool drop_spatial_layer[AOM_MAX_SS_LAYERS];
} SVC;

struct AV1_COMP;
struct EncodeFrameInput;

/*!\brief Initialize layer context data from init_config().
 *
 * \ingroup SVC
 * \callgraph
 * \callergraph
 *
 * \param[in]       cpi  Top level encoder structure
 *
 * \remark  Nothing returned. Set cpi->svc.
 */
void av1_init_layer_context(struct AV1_COMP *const cpi);

/*!\brief Allocate layer context data.
 *
 * \ingroup SVC
 * \callgraph
 * \callergraph
 *
 * \param[in]       cpi  Top level encoder structure
 * \param[in]       num_layers  Number of layers to be allocated
 *
 * \remark  Allocates memory for cpi->svc.layer_context.
 * \return  True on success, false on allocation failure.
 */
bool av1_alloc_layer_context(struct AV1_COMP *cpi, int num_layers);

/*!\brief Update the layer context from a change_config() call.
 *
 * \ingroup SVC
 * \callgraph
 * \callergraph
 *
 * \param[in]       cpi  Top level encoder structure
 * \param[in]       target_bandwidth  Total target bandwidth
 *
 * \remark  Nothing returned. Buffer level for each layer is set.
 */
void av1_update_layer_context_change_config(struct AV1_COMP *const cpi,
                                            const int64_t target_bandwidth);

/*!\brief Prior to encoding the frame, update framerate-related quantities
          for the current temporal layer.
 *
 * \ingroup SVC
 * \callgraph
 * \callergraph
 *
 * \param[in]       cpi  Top level encoder structure
 *
 * \remark  Nothing returned. Frame related quantities for current temporal
 layer are updated.
 */
void av1_update_temporal_layer_framerate(struct AV1_COMP *const cpi);

/*!\brief Prior to check if reference is lower spatial layer at the same
 *        timestamp/superframe.
 *
 * \ingroup SVC
 * \callgraph
 * \callergraph
 *
 * \param[in]       cpi  Top level encoder structure
 * \param[in]       ref_frame Reference frame
 *
 * \return  True if the ref_frame if lower spatial layer, otherwise false.
 */
bool av1_check_ref_is_low_spatial_res_super_frame(struct AV1_COMP *const cpi,
                                                  int ref_frame);

/*!\brief Prior to encoding the frame, set the layer context, for the current
 layer to be encoded, to the cpi struct.
 *
 * \ingroup SVC
 * \callgraph
 * \callergraph
 *
 * \param[in]       cpi  Top level encoder structure
 *
 * \remark  Nothing returned. Layer context for current layer is set.
 */
void av1_restore_layer_context(struct AV1_COMP *const cpi);

/*!\brief Save the layer context after encoding the frame.
 *
 * \ingroup SVC
 * \callgraph
 * \callergraph
 *
 * \param[in]       cpi  Top level encoder structure
 */
void av1_save_layer_context(struct AV1_COMP *const cpi);

/*!\brief Free the memory used for cyclic refresh in layer context.
 *
 * \ingroup SVC
 * \callgraph
 * \callergraph
 *
 * \param[in]       cpi  Top level encoder structure
 */
void av1_free_svc_cyclic_refresh(struct AV1_COMP *const cpi);

/*!\brief Reset on key frame: reset counters, references and buffer updates.
 *
 * \ingroup SVC
 * \callgraph
 * \callergraph
 *
 * \param[in]       cpi  Top level encoder structure
 * \param[in]       is_key  Whether current layer is key frame
 */
void av1_svc_reset_temporal_layers(struct AV1_COMP *const cpi, int is_key);

/*!\brief Before encoding, set resolutions and allocate compressor data.
 *
 * \ingroup SVC
 * \callgraph
 * \callergraph
 *
 * \param[in]       cpi  Top level encoder structure
 */
void av1_one_pass_cbr_svc_start_layer(struct AV1_COMP *const cpi);

/*!\brief Get primary reference frame for current layer
 *
 * \ingroup SVC
 * \callgraph
 * \callergraph
 *
 * \param[in]       cpi  Top level encoder structure
 *
 * \return  The primary reference frame for current layer.
 */
int av1_svc_primary_ref_frame(const struct AV1_COMP *const cpi);

/*!\brief Get resolution for current layer.
 *
 * \ingroup SVC
 * \param[in]       width_org    Original width, unscaled
 * \param[in]       height_org   Original height, unscaled
 * \param[in]       num          Numerator for the scale ratio
 * \param[in]       den          Denominator for the scale ratio
 * \param[in]       width_out    Output width, scaled for current layer
 * \param[in]       height_out   Output height, scaled for current layer
 *
 * \remark Nothing is returned. Instead the scaled width and height are set.
 */
void av1_get_layer_resolution(const int width_org, const int height_org,
                              const int num, const int den, int *width_out,
                              int *height_out);

void av1_set_svc_fixed_mode(struct AV1_COMP *const cpi);

void av1_svc_check_reset_layer_rc_flag(struct AV1_COMP *const cpi);

void av1_svc_set_last_source(struct AV1_COMP *const cpi,
                             struct EncodeFrameInput *frame_input,
                             YV12_BUFFER_CONFIG *prev_source);

void av1_svc_update_buffer_slot_refreshed(struct AV1_COMP *const cpi);

int av1_svc_get_min_ref_dist(const struct AV1_COMP *cpi);

void av1_svc_set_reference_was_previous(struct AV1_COMP *cpi);
#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_ENCODER_SVC_LAYERCONTEXT_H_
