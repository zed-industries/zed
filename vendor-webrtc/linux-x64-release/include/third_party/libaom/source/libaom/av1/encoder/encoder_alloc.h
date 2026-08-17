/*
 * Copyright (c) 2020, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_AV1_ENCODER_ENCODER_ALLOC_H_
#define AOM_AV1_ENCODER_ENCODER_ALLOC_H_

#include "av1/encoder/block.h"
#include "av1/encoder/encodeframe_utils.h"
#include "av1/encoder/encoder.h"
#include "av1/encoder/encodetxb.h"
#include "av1/encoder/ethread.h"
#include "av1/encoder/global_motion_facade.h"
#include "av1/encoder/intra_mode_search_utils.h"
#include "av1/encoder/pickcdef.h"

#ifdef __cplusplus
extern "C" {
#endif

static inline void dealloc_context_buffers_ext(
    MBMIExtFrameBufferInfo *mbmi_ext_info) {
  aom_free(mbmi_ext_info->frame_base);
  mbmi_ext_info->frame_base = NULL;
  mbmi_ext_info->alloc_size = 0;
}

static inline void alloc_context_buffers_ext(
    AV1_COMMON *cm, MBMIExtFrameBufferInfo *mbmi_ext_info) {
  const CommonModeInfoParams *const mi_params = &cm->mi_params;

  const int mi_alloc_size_1d = mi_size_wide[mi_params->mi_alloc_bsize];
  const int mi_alloc_rows =
      (mi_params->mi_rows + mi_alloc_size_1d - 1) / mi_alloc_size_1d;
  const int mi_alloc_cols =
      (mi_params->mi_cols + mi_alloc_size_1d - 1) / mi_alloc_size_1d;
  const int new_ext_mi_size = mi_alloc_rows * mi_alloc_cols;

  if (new_ext_mi_size > mbmi_ext_info->alloc_size) {
    dealloc_context_buffers_ext(mbmi_ext_info);
    CHECK_MEM_ERROR(
        cm, mbmi_ext_info->frame_base,
        aom_malloc(new_ext_mi_size * sizeof(*mbmi_ext_info->frame_base)));
    mbmi_ext_info->alloc_size = new_ext_mi_size;
  }
  // The stride needs to be updated regardless of whether new allocation
  // happened or not.
  mbmi_ext_info->stride = mi_alloc_cols;
}

static inline void alloc_compressor_data(AV1_COMP *cpi) {
  AV1_COMMON *cm = &cpi->common;
  CommonModeInfoParams *const mi_params = &cm->mi_params;

  // Setup mi_params
  mi_params->set_mb_mi(mi_params, cm->width, cm->height,
                       cpi->sf.part_sf.default_min_partition_size);

  if (!is_stat_generation_stage(cpi)) av1_alloc_txb_buf(cpi);

  aom_free(cpi->td.mv_costs_alloc);
  cpi->td.mv_costs_alloc = NULL;
  // Avoid the memory allocation of 'mv_costs_alloc' for allintra encoding
  // mode.
  if (cpi->oxcf.kf_cfg.key_freq_max != 0) {
    CHECK_MEM_ERROR(cm, cpi->td.mv_costs_alloc,
                    (MvCosts *)aom_calloc(1, sizeof(*cpi->td.mv_costs_alloc)));
    cpi->td.mb.mv_costs = cpi->td.mv_costs_alloc;
  }

  av1_setup_shared_coeff_buffer(cm->seq_params, &cpi->td.shared_coeff_buf,
                                cm->error);
  if (av1_setup_sms_tree(cpi, &cpi->td)) {
    aom_internal_error(cm->error, AOM_CODEC_MEM_ERROR,
                       "Failed to allocate SMS tree");
  }
  cpi->td.firstpass_ctx =
      av1_alloc_pmc(cpi, BLOCK_16X16, &cpi->td.shared_coeff_buf);
  if (!cpi->td.firstpass_ctx)
    aom_internal_error(cm->error, AOM_CODEC_MEM_ERROR,
                       "Failed to allocate PICK_MODE_CONTEXT");
}

// Allocate mbmi buffers which are used to store mode information at block
// level.
static inline void alloc_mb_mode_info_buffers(AV1_COMP *const cpi) {
  AV1_COMMON *const cm = &cpi->common;
  if (av1_alloc_context_buffers(cm, cm->width, cm->height,
                                cpi->sf.part_sf.default_min_partition_size)) {
    aom_internal_error(cm->error, AOM_CODEC_MEM_ERROR,
                       "Failed to allocate context buffers");
  }

  if (!is_stat_generation_stage(cpi))
    alloc_context_buffers_ext(cm, &cpi->mbmi_ext_info);
}

static inline void realloc_segmentation_maps(AV1_COMP *cpi) {
  AV1_COMMON *const cm = &cpi->common;
  CommonModeInfoParams *const mi_params = &cm->mi_params;

  // Create the encoder segmentation map and set all entries to 0
  aom_free(cpi->enc_seg.map);
  CHECK_MEM_ERROR(cm, cpi->enc_seg.map,
                  aom_calloc(mi_params->mi_rows * mi_params->mi_cols, 1));

  // Create a map used for cyclic background refresh.
  if (cpi->cyclic_refresh) av1_cyclic_refresh_free(cpi->cyclic_refresh);
  CHECK_MEM_ERROR(
      cm, cpi->cyclic_refresh,
      av1_cyclic_refresh_alloc(mi_params->mi_rows, mi_params->mi_cols));

  // Create a map used to mark inactive areas.
  aom_free(cpi->active_map.map);
  CHECK_MEM_ERROR(cm, cpi->active_map.map,
                  aom_calloc(mi_params->mi_rows * mi_params->mi_cols, 1));
}

static inline void alloc_obmc_buffers(OBMCBuffer *obmc_buffer,
                                      struct aom_internal_error_info *error) {
  AOM_CHECK_MEM_ERROR(
      error, obmc_buffer->wsrc,
      (int32_t *)aom_memalign(16, MAX_SB_SQUARE * sizeof(*obmc_buffer->wsrc)));
  AOM_CHECK_MEM_ERROR(
      error, obmc_buffer->mask,
      (int32_t *)aom_memalign(16, MAX_SB_SQUARE * sizeof(*obmc_buffer->mask)));
  AOM_CHECK_MEM_ERROR(
      error, obmc_buffer->above_pred,
      (uint8_t *)aom_memalign(
          16, MAX_MB_PLANE * MAX_SB_SQUARE * sizeof(*obmc_buffer->above_pred)));
  AOM_CHECK_MEM_ERROR(
      error, obmc_buffer->left_pred,
      (uint8_t *)aom_memalign(
          16, MAX_MB_PLANE * MAX_SB_SQUARE * sizeof(*obmc_buffer->left_pred)));
}

static inline void release_obmc_buffers(OBMCBuffer *obmc_buffer) {
  aom_free(obmc_buffer->mask);
  aom_free(obmc_buffer->above_pred);
  aom_free(obmc_buffer->left_pred);
  aom_free(obmc_buffer->wsrc);

  obmc_buffer->mask = NULL;
  obmc_buffer->above_pred = NULL;
  obmc_buffer->left_pred = NULL;
  obmc_buffer->wsrc = NULL;
}

static inline void alloc_compound_type_rd_buffers(
    struct aom_internal_error_info *error, CompoundTypeRdBuffers *const bufs) {
  AOM_CHECK_MEM_ERROR(
      error, bufs->pred0,
      (uint8_t *)aom_memalign(16, 2 * MAX_SB_SQUARE * sizeof(*bufs->pred0)));
  AOM_CHECK_MEM_ERROR(
      error, bufs->pred1,
      (uint8_t *)aom_memalign(16, 2 * MAX_SB_SQUARE * sizeof(*bufs->pred1)));
  AOM_CHECK_MEM_ERROR(
      error, bufs->residual1,
      (int16_t *)aom_memalign(32, MAX_SB_SQUARE * sizeof(*bufs->residual1)));
  AOM_CHECK_MEM_ERROR(
      error, bufs->diff10,
      (int16_t *)aom_memalign(32, MAX_SB_SQUARE * sizeof(*bufs->diff10)));
  AOM_CHECK_MEM_ERROR(error, bufs->tmp_best_mask_buf,
                      (uint8_t *)aom_malloc(2 * MAX_SB_SQUARE *
                                            sizeof(*bufs->tmp_best_mask_buf)));
}

static inline void release_compound_type_rd_buffers(
    CompoundTypeRdBuffers *const bufs) {
  aom_free(bufs->pred0);
  aom_free(bufs->pred1);
  aom_free(bufs->residual1);
  aom_free(bufs->diff10);
  aom_free(bufs->tmp_best_mask_buf);
  av1_zero(*bufs);  // Set all pointers to NULL for safety.
}

static inline void dealloc_compressor_data(AV1_COMP *cpi) {
  AV1_COMMON *const cm = &cpi->common;
  TokenInfo *token_info = &cpi->token_info;
  AV1EncRowMultiThreadInfo *const enc_row_mt = &cpi->mt_info.enc_row_mt;
  const int num_planes = av1_num_planes(cm);
  dealloc_context_buffers_ext(&cpi->mbmi_ext_info);

  aom_free(cpi->tile_data);
  cpi->tile_data = NULL;
  cpi->allocated_tiles = 0;
  enc_row_mt->allocated_tile_cols = 0;
  enc_row_mt->allocated_tile_rows = 0;

  // Delete sementation map
  aom_free(cpi->enc_seg.map);
  cpi->enc_seg.map = NULL;

  av1_cyclic_refresh_free(cpi->cyclic_refresh);
  cpi->cyclic_refresh = NULL;

  aom_free(cpi->active_map.map);
  cpi->active_map.map = NULL;

  aom_free(cpi->ssim_rdmult_scaling_factors);
  cpi->ssim_rdmult_scaling_factors = NULL;

  aom_free(cpi->tpl_rdmult_scaling_factors);
  cpi->tpl_rdmult_scaling_factors = NULL;

#if CONFIG_TUNE_VMAF
  aom_free(cpi->vmaf_info.rdmult_scaling_factors);
  cpi->vmaf_info.rdmult_scaling_factors = NULL;
  aom_close_vmaf_model(cpi->vmaf_info.vmaf_model);
#endif

#if CONFIG_TUNE_BUTTERAUGLI
  aom_free(cpi->butteraugli_info.rdmult_scaling_factors);
  cpi->butteraugli_info.rdmult_scaling_factors = NULL;
  aom_free_frame_buffer(&cpi->butteraugli_info.source);
  aom_free_frame_buffer(&cpi->butteraugli_info.resized_source);
#endif

#if CONFIG_SALIENCY_MAP
  aom_free(cpi->saliency_map);
  aom_free(cpi->sm_scaling_factor);
#endif

  release_obmc_buffers(&cpi->td.mb.obmc_buffer);

  aom_free(cpi->td.mv_costs_alloc);
  cpi->td.mv_costs_alloc = NULL;
  aom_free(cpi->td.dv_costs_alloc);
  cpi->td.dv_costs_alloc = NULL;

  aom_free(cpi->td.mb.sb_stats_cache);
  cpi->td.mb.sb_stats_cache = NULL;

  aom_free(cpi->td.mb.sb_fp_stats);
  cpi->td.mb.sb_fp_stats = NULL;

#if CONFIG_PARTITION_SEARCH_ORDER
  aom_free(cpi->td.mb.rdcost);
  cpi->td.mb.rdcost = NULL;
#endif

  av1_free_pc_tree_recursive(cpi->td.pc_root, num_planes, 0, 0,
                             cpi->sf.part_sf.partition_search_type);
  cpi->td.pc_root = NULL;

  for (int i = 0; i < 2; i++)
    for (int j = 0; j < 2; j++) {
      aom_free(cpi->td.mb.intrabc_hash_info.hash_value_buffer[i][j]);
      cpi->td.mb.intrabc_hash_info.hash_value_buffer[i][j] = NULL;
    }

  av1_hash_table_destroy(&cpi->td.mb.intrabc_hash_info.intrabc_hash_table);

  aom_free(cm->tpl_mvs);
  cm->tpl_mvs = NULL;

  aom_free(cpi->td.pixel_gradient_info);
  cpi->td.pixel_gradient_info = NULL;

  aom_free(cpi->td.src_var_info_of_4x4_sub_blocks);
  cpi->td.src_var_info_of_4x4_sub_blocks = NULL;

  aom_free(cpi->td.vt64x64);
  cpi->td.vt64x64 = NULL;

  av1_free_pmc(cpi->td.firstpass_ctx, num_planes);
  cpi->td.firstpass_ctx = NULL;

  const int is_highbitdepth = cpi->tf_ctx.is_highbitdepth;
  // This call ensures that the buffers allocated by tf_alloc_and_reset_data()
  // in av1_temporal_filter() for single-threaded encode are freed in case an
  // error is encountered during temporal filtering (due to early termination
  // tf_dealloc_data() in av1_temporal_filter() would not be invoked).
  tf_dealloc_data(&cpi->td.tf_data, is_highbitdepth);

  // This call ensures that tpl_tmp_buffers for single-threaded encode are freed
  // in case of an error during tpl.
  tpl_dealloc_temp_buffers(&cpi->td.tpl_tmp_buffers);

  // This call ensures that the global motion (gm) data buffers for
  // single-threaded encode are freed in case of an error during gm.
  gm_dealloc_data(&cpi->td.gm_data);

  // This call ensures that CDEF search context buffers are deallocated in case
  // of an error during cdef search.
  av1_cdef_dealloc_data(cpi->cdef_search_ctx);
  aom_free(cpi->cdef_search_ctx);
  cpi->cdef_search_ctx = NULL;

  av1_dealloc_mb_data(&cpi->td.mb, num_planes);

  av1_dealloc_mb_wiener_var_pred_buf(&cpi->td);

  av1_free_txb_buf(cpi);
  av1_free_context_buffers(cm);

  aom_free_frame_buffer(&cpi->last_frame_uf);
#if !CONFIG_REALTIME_ONLY
  av1_free_restoration_buffers(cm);
  av1_free_firstpass_data(&cpi->firstpass_data);
#endif

  if (!is_stat_generation_stage(cpi)) {
    av1_free_cdef_buffers(cm, &cpi->ppi->p_mt_info.cdef_worker,
                          &cpi->mt_info.cdef_sync);
  }

  for (int plane = 0; plane < num_planes; plane++) {
    aom_free(cpi->pick_lr_ctxt.rusi[plane]);
    cpi->pick_lr_ctxt.rusi[plane] = NULL;
  }
  aom_free(cpi->pick_lr_ctxt.dgd_avg);
  cpi->pick_lr_ctxt.dgd_avg = NULL;

  aom_free_frame_buffer(&cpi->trial_frame_rst);
  aom_free_frame_buffer(&cpi->scaled_source);
  aom_free_frame_buffer(&cpi->scaled_last_source);
  aom_free_frame_buffer(&cpi->orig_source);
  aom_free_frame_buffer(&cpi->svc.source_last_TL0);

  free_token_info(token_info);

  av1_free_shared_coeff_buffer(&cpi->td.shared_coeff_buf);
  av1_free_sms_tree(&cpi->td);

  aom_free(cpi->td.mb.palette_buffer);
  release_compound_type_rd_buffers(&cpi->td.mb.comp_rd_buffer);
  aom_free(cpi->td.mb.tmp_conv_dst);
  for (int j = 0; j < 2; ++j) {
    aom_free(cpi->td.mb.tmp_pred_bufs[j]);
  }

#if CONFIG_DENOISE && !CONFIG_REALTIME_ONLY
  if (cpi->denoise_and_model) {
    aom_denoise_and_model_free(cpi->denoise_and_model);
    cpi->denoise_and_model = NULL;
  }
#endif
#if !CONFIG_REALTIME_ONLY
  if (cpi->film_grain_table) {
    aom_film_grain_table_free(cpi->film_grain_table);
    aom_free(cpi->film_grain_table);
    cpi->film_grain_table = NULL;
  }
#endif

  if (cpi->ppi->use_svc) av1_free_svc_cyclic_refresh(cpi);
  aom_free(cpi->svc.layer_context);
  cpi->svc.layer_context = NULL;

  aom_free(cpi->consec_zero_mv);
  cpi->consec_zero_mv = NULL;
  cpi->consec_zero_mv_alloc_size = 0;

  aom_free(cpi->src_sad_blk_64x64);
  cpi->src_sad_blk_64x64 = NULL;

  aom_free(cpi->mb_weber_stats);
  cpi->mb_weber_stats = NULL;

  if (cpi->oxcf.enable_rate_guide_deltaq) {
    aom_free(cpi->prep_rate_estimates);
    cpi->prep_rate_estimates = NULL;

    aom_free(cpi->ext_rate_distribution);
    cpi->ext_rate_distribution = NULL;
  }

  aom_free(cpi->mb_delta_q);
  cpi->mb_delta_q = NULL;
}

static inline void allocate_gradient_info_for_hog(AV1_COMP *cpi) {
  if (!is_gradient_caching_for_hog_enabled(cpi)) return;

  PixelLevelGradientInfo *pixel_gradient_info = cpi->td.pixel_gradient_info;
  if (!pixel_gradient_info) {
    const AV1_COMMON *const cm = &cpi->common;
    const int plane_types = PLANE_TYPES >> cm->seq_params->monochrome;
    CHECK_MEM_ERROR(
        cm, pixel_gradient_info,
        aom_malloc(sizeof(*pixel_gradient_info) * plane_types * MAX_SB_SQUARE));
    cpi->td.pixel_gradient_info = pixel_gradient_info;
  }

  cpi->td.mb.pixel_gradient_info = pixel_gradient_info;
}

static inline void allocate_src_var_of_4x4_sub_block_buf(AV1_COMP *cpi) {
  if (!is_src_var_for_4x4_sub_blocks_caching_enabled(cpi)) return;

  Block4x4VarInfo *source_variance_info =
      cpi->td.src_var_info_of_4x4_sub_blocks;
  if (!source_variance_info) {
    const AV1_COMMON *const cm = &cpi->common;
    const BLOCK_SIZE sb_size = cm->seq_params->sb_size;
    const int mi_count_in_sb = mi_size_wide[sb_size] * mi_size_high[sb_size];
    CHECK_MEM_ERROR(cm, source_variance_info,
                    aom_malloc(sizeof(*source_variance_info) * mi_count_in_sb));
    cpi->td.src_var_info_of_4x4_sub_blocks = source_variance_info;
  }

  cpi->td.mb.src_var_info_of_4x4_sub_blocks = source_variance_info;
}

static inline void variance_partition_alloc(AV1_COMP *cpi) {
  AV1_COMMON *const cm = &cpi->common;
  const int num_64x64_blocks = (cm->seq_params->sb_size == BLOCK_64X64) ? 1 : 4;
  if (cpi->td.vt64x64) {
    if (num_64x64_blocks != cpi->td.num_64x64_blocks) {
      aom_free(cpi->td.vt64x64);
      cpi->td.vt64x64 = NULL;
    }
  }
  if (!cpi->td.vt64x64) {
    CHECK_MEM_ERROR(cm, cpi->td.vt64x64,
                    aom_malloc(sizeof(*cpi->td.vt64x64) * num_64x64_blocks));
    cpi->td.num_64x64_blocks = num_64x64_blocks;
  }
}

static inline YV12_BUFFER_CONFIG *realloc_and_scale_source(AV1_COMP *cpi,
                                                           int scaled_width,
                                                           int scaled_height) {
  AV1_COMMON *cm = &cpi->common;
  const int num_planes = av1_num_planes(cm);

  if (scaled_width == cpi->unscaled_source->y_crop_width &&
      scaled_height == cpi->unscaled_source->y_crop_height) {
    return cpi->unscaled_source;
  }

  if (aom_realloc_frame_buffer(
          &cpi->scaled_source, scaled_width, scaled_height,
          cm->seq_params->subsampling_x, cm->seq_params->subsampling_y,
          cm->seq_params->use_highbitdepth, AOM_BORDER_IN_PIXELS,
          cm->features.byte_alignment, NULL, NULL, NULL, cpi->alloc_pyramid, 0))
    aom_internal_error(cm->error, AOM_CODEC_MEM_ERROR,
                       "Failed to reallocate scaled source buffer");
  assert(cpi->scaled_source.y_crop_width == scaled_width);
  assert(cpi->scaled_source.y_crop_height == scaled_height);
  if (!av1_resize_and_extend_frame_nonnormative(
          cpi->unscaled_source, &cpi->scaled_source,
          (int)cm->seq_params->bit_depth, num_planes))
    aom_internal_error(cm->error, AOM_CODEC_MEM_ERROR,
                       "Failed to reallocate buffers during resize");
  return &cpi->scaled_source;
}

// Deallocate allocated thread_data.
static inline void free_thread_data(AV1_PRIMARY *ppi) {
  PrimaryMultiThreadInfo *const p_mt_info = &ppi->p_mt_info;
  const int num_tf_workers =
      AOMMIN(p_mt_info->num_mod_workers[MOD_TF], p_mt_info->num_workers);
  const int num_tpl_workers =
      AOMMIN(p_mt_info->num_mod_workers[MOD_TPL], p_mt_info->num_workers);
  const int is_highbitdepth = ppi->seq_params.use_highbitdepth;
  const int num_planes = ppi->seq_params.monochrome ? 1 : MAX_MB_PLANE;
  for (int t = 1; t < p_mt_info->num_workers; ++t) {
    EncWorkerData *const thread_data = &p_mt_info->tile_thr_data[t];
    thread_data->td = thread_data->original_td;
    ThreadData *const td = thread_data->td;
    if (!td) continue;
    aom_free(td->tctx);
    aom_free(td->palette_buffer);
    aom_free(td->tmp_conv_dst);
    release_compound_type_rd_buffers(&td->comp_rd_buffer);
    for (int j = 0; j < 2; ++j) {
      aom_free(td->tmp_pred_bufs[j]);
    }
    aom_free(td->pixel_gradient_info);
    aom_free(td->src_var_info_of_4x4_sub_blocks);
    release_obmc_buffers(&td->obmc_buffer);
    aom_free(td->vt64x64);

    for (int x = 0; x < 2; x++) {
      for (int y = 0; y < 2; y++) {
        aom_free(td->hash_value_buffer[x][y]);
        td->hash_value_buffer[x][y] = NULL;
      }
    }
    aom_free(td->mv_costs_alloc);
    td->mv_costs_alloc = NULL;
    aom_free(td->dv_costs_alloc);
    td->dv_costs_alloc = NULL;
    aom_free(td->counts);
    av1_free_pmc(td->firstpass_ctx, num_planes);
    td->firstpass_ctx = NULL;
    av1_free_shared_coeff_buffer(&td->shared_coeff_buf);
    av1_free_sms_tree(td);
    // This call ensures that the buffers allocated by tf_alloc_and_reset_data()
    // in prepare_tf_workers() for MT encode are freed in case an error is
    // encountered during temporal filtering (due to early termination
    // tf_dealloc_thread_data() in av1_tf_do_filtering_mt() would not be
    // invoked).
    if (t < num_tf_workers) tf_dealloc_data(&td->tf_data, is_highbitdepth);
    // This call ensures that tpl_tmp_buffers for MT encode are freed in case of
    // an error during tpl.
    if (t < num_tpl_workers) tpl_dealloc_temp_buffers(&td->tpl_tmp_buffers);
    // This call ensures that the buffers in gm_data for MT encode are freed in
    // case of an error during gm.
    gm_dealloc_data(&td->gm_data);
    av1_dealloc_mb_data(&td->mb, num_planes);
    aom_free(td->mb.sb_stats_cache);
    td->mb.sb_stats_cache = NULL;
    aom_free(td->mb.sb_fp_stats);
    td->mb.sb_fp_stats = NULL;
#if CONFIG_PARTITION_SEARCH_ORDER
    aom_free(td->mb.rdcost);
    td->mb.rdcost = NULL;
#endif
    av1_free_pc_tree_recursive(td->pc_root, num_planes, 0, 0, SEARCH_PARTITION);
    td->pc_root = NULL;
    av1_dealloc_mb_wiener_var_pred_buf(td);
    aom_free(td);
    thread_data->td = NULL;
    thread_data->original_td = NULL;
  }
}

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_ENCODER_ENCODER_ALLOC_H_
