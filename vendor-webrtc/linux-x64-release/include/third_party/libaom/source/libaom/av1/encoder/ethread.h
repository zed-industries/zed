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

#ifndef AOM_AV1_ENCODER_ETHREAD_H_
#define AOM_AV1_ENCODER_ETHREAD_H_

#ifdef __cplusplus
extern "C" {
#endif

struct AV1_COMP;
struct ThreadData;

typedef struct EncWorkerData {
  struct AV1_COMP *cpi;
  struct ThreadData *td;
  struct ThreadData *original_td;
  struct aom_internal_error_info error_info;
  AV1LfSync *lf_sync;
  LFWorkerData *lf_data;
  int start;
  int thread_id;
} EncWorkerData;

void av1_row_mt_sync_read(AV1EncRowMultiThreadSync *row_mt_sync, int r, int c);
void av1_row_mt_sync_write(AV1EncRowMultiThreadSync *row_mt_sync, int r, int c,
                           int cols);

void av1_row_mt_sync_read_dummy(AV1EncRowMultiThreadSync *row_mt_sync, int r,
                                int c);
void av1_row_mt_sync_write_dummy(AV1EncRowMultiThreadSync *row_mt_sync, int r,
                                 int c, int cols);

void av1_encode_tiles_mt(struct AV1_COMP *cpi);
void av1_encode_tiles_row_mt(struct AV1_COMP *cpi);

#if !CONFIG_REALTIME_ONLY
void av1_fp_encode_tiles_row_mt(AV1_COMP *cpi);

int av1_fp_compute_num_enc_workers(AV1_COMP *cpi);
#endif

void av1_accumulate_frame_counts(struct FRAME_COUNTS *acc_counts,
                                 const struct FRAME_COUNTS *counts);

void av1_row_mt_mem_dealloc(AV1_COMP *cpi);

void av1_row_mt_sync_mem_dealloc(AV1EncRowMultiThreadSync *row_mt_sync);

void av1_global_motion_estimation_mt(AV1_COMP *cpi);

#if !CONFIG_REALTIME_ONLY
void av1_tpl_row_mt_sync_read_dummy(AV1TplRowMultiThreadSync *tpl_mt_sync,
                                    int r, int c);
void av1_tpl_row_mt_sync_write_dummy(AV1TplRowMultiThreadSync *tpl_mt_sync,
                                     int r, int c, int cols);

void av1_tpl_row_mt_sync_read(AV1TplRowMultiThreadSync *tpl_mt_sync, int r,
                              int c);
void av1_tpl_row_mt_sync_write(AV1TplRowMultiThreadSync *tpl_mt_sync, int r,
                               int c, int cols);

void av1_mc_flow_dispenser_mt(AV1_COMP *cpi);

void av1_tpl_dealloc(AV1TplRowMultiThreadSync *tpl_sync);

#endif  // !CONFIG_REALTIME_ONLY

void av1_calc_mb_wiener_var_mt(AV1_COMP *cpi, int num_workers,
                               double *sum_rec_distortion,
                               double *sum_est_rate);

void av1_tf_do_filtering_mt(AV1_COMP *cpi);

void av1_tf_mt_dealloc(AV1TemporalFilterSync *tf_sync);

void av1_compute_num_workers_for_mt(AV1_COMP *cpi);

int av1_get_max_num_workers(const AV1_COMP *cpi);

void av1_create_workers(AV1_PRIMARY *ppi, int num_workers);

void av1_terminate_workers(AV1_PRIMARY *ppi);

void av1_init_frame_mt(AV1_PRIMARY *ppi, AV1_COMP *cpi);

void av1_init_cdef_worker(AV1_COMP *cpi);

#if !CONFIG_REALTIME_ONLY
void av1_init_lr_mt_buffers(AV1_COMP *cpi);
#endif

#if CONFIG_MULTITHREAD
void av1_init_mt_sync(AV1_COMP *cpi, int is_first_pass);
#endif  // CONFIG_MULTITHREAD

int av1_get_num_mod_workers_for_alloc(const PrimaryMultiThreadInfo *p_mt_info,
                                      MULTI_THREADED_MODULES mod_name);

void av1_init_tile_thread_data(AV1_PRIMARY *ppi, int is_first_pass);

void av1_cdef_mse_calc_frame_mt(AV1_COMP *cpi);

void av1_cdef_mt_dealloc(AV1CdefSync *cdef_sync);

void av1_write_tile_obu_mt(
    AV1_COMP *const cpi, uint8_t *const dst, uint32_t *total_size,
    struct aom_write_bit_buffer *saved_wb, uint8_t obu_extn_header,
    const FrameHeaderInfo *fh_info, int *const largest_tile_id,
    unsigned int *max_tile_size, uint32_t *const obu_header_size,
    uint8_t **tile_data_start, const int num_workers);

int av1_compute_num_fp_contexts(AV1_PRIMARY *ppi, AV1EncoderConfig *oxcf);

int av1_check_fpmt_config(AV1_PRIMARY *const ppi,
                          const AV1EncoderConfig *const oxcf);

void av1_compress_parallel_frames(AV1_PRIMARY *const ppi,
                                  AV1_COMP_DATA *const first_cpi_data);
#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_ENCODER_ETHREAD_H_
