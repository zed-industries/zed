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

#ifndef AOM_AV1_COMMON_THREAD_COMMON_H_
#define AOM_AV1_COMMON_THREAD_COMMON_H_

#include "config/aom_config.h"

#include "av1/common/av1_loopfilter.h"
#include "av1/common/cdef.h"
#include "aom_util/aom_pthread.h"
#include "aom_util/aom_thread.h"

#ifdef __cplusplus
extern "C" {
#endif

struct AV1Common;

typedef struct AV1LfMTInfo {
  int mi_row;
  int plane;
  int dir;
  int lpf_opt_level;
} AV1LfMTInfo;

// Loopfilter row synchronization
typedef struct AV1LfSyncData {
#if CONFIG_MULTITHREAD
  pthread_mutex_t *mutex_[MAX_MB_PLANE];
  pthread_cond_t *cond_[MAX_MB_PLANE];
#endif
  // Allocate memory to store the loop-filtered superblock index in each row.
  int *cur_sb_col[MAX_MB_PLANE];
  // The optimal sync_range for different resolution and platform should be
  // determined by testing. Currently, it is chosen to be a power-of-2 number.
  int sync_range;
  int rows;

  // Row-based parallel loopfilter data
  LFWorkerData *lfdata;
  int num_workers;

#if CONFIG_MULTITHREAD
  pthread_mutex_t *job_mutex;
#endif
  AV1LfMTInfo *job_queue;
  int jobs_enqueued;
  int jobs_dequeued;

  // Initialized to false, set to true by the worker thread that encounters an
  // error in order to abort the processing of other worker threads.
  bool lf_mt_exit;
} AV1LfSync;

typedef struct AV1LrMTInfo {
  int v_start;
  int v_end;
  int lr_unit_row;
  int plane;
  int sync_mode;
  int v_copy_start;
  int v_copy_end;
} AV1LrMTInfo;

typedef struct LoopRestorationWorkerData {
  int32_t *rst_tmpbuf;
  void *rlbs;
  void *lr_ctxt;
  int do_extend_border;
  struct aom_internal_error_info error_info;
} LRWorkerData;

// Looprestoration row synchronization
typedef struct AV1LrSyncData {
#if CONFIG_MULTITHREAD
  pthread_mutex_t *mutex_[MAX_MB_PLANE];
  pthread_cond_t *cond_[MAX_MB_PLANE];
#endif
  // Allocate memory to store the loop-restoration block index in each row.
  int *cur_sb_col[MAX_MB_PLANE];
  // The optimal sync_range for different resolution and platform should be
  // determined by testing. Currently, it is chosen to be a power-of-2 number.
  int sync_range;
  int rows;
  int num_planes;

  int num_workers;

#if CONFIG_MULTITHREAD
  pthread_mutex_t *job_mutex;
#endif
  // Row-based parallel loopfilter data
  LRWorkerData *lrworkerdata;

  AV1LrMTInfo *job_queue;
  int jobs_enqueued;
  int jobs_dequeued;
  // Initialized to false, set to true by the worker thread that encounters
  // an error in order to abort the processing of other worker threads.
  bool lr_mt_exit;
} AV1LrSync;

typedef struct AV1CdefWorker {
  AV1_COMMON *cm;
  MACROBLOCKD *xd;
  uint16_t *colbuf[MAX_MB_PLANE];
  uint16_t *srcbuf;
  uint16_t *linebuf[MAX_MB_PLANE];
  cdef_init_fb_row_t cdef_init_fb_row_fn;
  int do_extend_border;
  struct aom_internal_error_info error_info;
} AV1CdefWorkerData;

typedef struct AV1CdefRowSync {
#if CONFIG_MULTITHREAD
  pthread_mutex_t *row_mutex_;
  pthread_cond_t *row_cond_;
#endif  // CONFIG_MULTITHREAD
  int is_row_done;
} AV1CdefRowSync;

// Data related to CDEF search multi-thread synchronization.
typedef struct AV1CdefSyncData {
#if CONFIG_MULTITHREAD
  // Mutex lock used while dispatching jobs.
  pthread_mutex_t *mutex_;
#endif  // CONFIG_MULTITHREAD
  // Data related to CDEF row mt sync information
  AV1CdefRowSync *cdef_row_mt;
  // Flag to indicate all blocks are processed and end of frame is reached
  int end_of_frame;
  // Row index in units of 64x64 block
  int fbr;
  // Column index in units of 64x64 block
  int fbc;
  // Initialized to false, set to true by the worker thread that encounters
  // an error in order to abort the processing of other worker threads.
  bool cdef_mt_exit;
} AV1CdefSync;

void av1_cdef_frame_mt(AV1_COMMON *const cm, MACROBLOCKD *const xd,
                       AV1CdefWorkerData *const cdef_worker,
                       AVxWorker *const workers, AV1CdefSync *const cdef_sync,
                       int num_workers, cdef_init_fb_row_t cdef_init_fb_row_fn,
                       int do_extend_border);
void av1_cdef_init_fb_row_mt(const AV1_COMMON *const cm,
                             const MACROBLOCKD *const xd,
                             CdefBlockInfo *const fb_info,
                             uint16_t **const linebuf, uint16_t *const src,
                             struct AV1CdefSyncData *const cdef_sync, int fbr);
void av1_cdef_copy_sb8_16(const AV1_COMMON *const cm, uint16_t *const dst,
                          int dstride, const uint8_t *src, int src_voffset,
                          int src_hoffset, int sstride, int vsize, int hsize);
void av1_cdef_copy_sb8_16_lowbd(uint16_t *const dst, int dstride,
                                const uint8_t *src, int src_voffset,
                                int src_hoffset, int sstride, int vsize,
                                int hsize);
#if CONFIG_AV1_HIGHBITDEPTH
void av1_cdef_copy_sb8_16_highbd(uint16_t *const dst, int dstride,
                                 const uint8_t *src, int src_voffset,
                                 int src_hoffset, int sstride, int vsize,
                                 int hsize);
#endif  // CONFIG_AV1_HIGHBITDEPTH
void av1_alloc_cdef_sync(AV1_COMMON *const cm, AV1CdefSync *cdef_sync,
                         int num_workers);
void av1_free_cdef_sync(AV1CdefSync *cdef_sync);

// Deallocate loopfilter synchronization related mutex and data.
void av1_loop_filter_dealloc(AV1LfSync *lf_sync);
void av1_loop_filter_alloc(AV1LfSync *lf_sync, AV1_COMMON *cm, int rows,
                           int width, int num_workers);

void av1_set_vert_loop_filter_done(AV1_COMMON *cm, AV1LfSync *lf_sync,
                                   int num_mis_in_lpf_unit_height_log2);

void av1_loop_filter_frame_mt(YV12_BUFFER_CONFIG *frame, struct AV1Common *cm,
                              struct macroblockd *xd, int plane_start,
                              int plane_end, int partial_frame,
                              AVxWorker *workers, int num_workers,
                              AV1LfSync *lf_sync, int lpf_opt_level);

#if !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER
void av1_loop_restoration_filter_frame_mt(YV12_BUFFER_CONFIG *frame,
                                          struct AV1Common *cm,
                                          int optimized_lr, AVxWorker *workers,
                                          int num_workers, AV1LrSync *lr_sync,
                                          void *lr_ctxt, int do_extend_border);
void av1_loop_restoration_dealloc(AV1LrSync *lr_sync);
void av1_loop_restoration_alloc(AV1LrSync *lr_sync, AV1_COMMON *cm,
                                int num_workers, int num_rows_lr,
                                int num_planes, int width);
#endif  // !CONFIG_REALTIME_ONLY || CONFIG_AV1_DECODER

int av1_get_intrabc_extra_top_right_sb_delay(const AV1_COMMON *cm);

void av1_thread_loop_filter_rows(
    const YV12_BUFFER_CONFIG *const frame_buffer, AV1_COMMON *const cm,
    struct macroblockd_plane *planes, MACROBLOCKD *xd, int mi_row, int plane,
    int dir, int lpf_opt_level, AV1LfSync *const lf_sync,
    struct aom_internal_error_info *error_info,
    AV1_DEBLOCKING_PARAMETERS *params_buf, TX_SIZE *tx_buf, int mib_size_log2);

static AOM_FORCE_INLINE bool skip_loop_filter_plane(
    const int planes_to_lf[MAX_MB_PLANE], int plane, int lpf_opt_level) {
  // If LPF_PICK_METHOD is LPF_PICK_FROM_Q, we have the option to filter both
  // chroma planes together
  if (lpf_opt_level == 2) {
    if (plane == AOM_PLANE_Y) {
      return !planes_to_lf[plane];
    }
    if (plane == AOM_PLANE_U) {
      // U and V are handled together
      return !planes_to_lf[1] && !planes_to_lf[2];
    }
    assert(plane == AOM_PLANE_V);
    if (plane == AOM_PLANE_V) {
      // V is handled when u is filtered
      return true;
    }
  }

  // Normal operation mode
  return !planes_to_lf[plane];
}

static inline void enqueue_lf_jobs(AV1LfSync *lf_sync, int start, int stop,
                                   const int planes_to_lf[MAX_MB_PLANE],
                                   int lpf_opt_level,
                                   int num_mis_in_lpf_unit_height) {
  int mi_row, plane, dir;
  AV1LfMTInfo *lf_job_queue = lf_sync->job_queue;
  lf_sync->jobs_enqueued = 0;
  lf_sync->jobs_dequeued = 0;

  // Launch all vertical jobs first, as they are blocking the horizontal ones.
  // Launch top row jobs for all planes first, in case the output can be
  // partially reconstructed row by row.
  for (dir = 0; dir < 2; ++dir) {
    for (mi_row = start; mi_row < stop; mi_row += num_mis_in_lpf_unit_height) {
      for (plane = 0; plane < MAX_MB_PLANE; ++plane) {
        if (skip_loop_filter_plane(planes_to_lf, plane, lpf_opt_level)) {
          continue;
        }
        if (!planes_to_lf[plane]) continue;
        lf_job_queue->mi_row = mi_row;
        lf_job_queue->plane = plane;
        lf_job_queue->dir = dir;
        lf_job_queue->lpf_opt_level = lpf_opt_level;
        lf_job_queue++;
        lf_sync->jobs_enqueued++;
      }
    }
  }
}

static inline void loop_filter_frame_mt_init(
    AV1_COMMON *cm, int start_mi_row, int end_mi_row,
    const int planes_to_lf[MAX_MB_PLANE], int num_workers, AV1LfSync *lf_sync,
    int lpf_opt_level, int num_mis_in_lpf_unit_height_log2) {
  // Number of superblock rows
  const int sb_rows =
      CEIL_POWER_OF_TWO(cm->mi_params.mi_rows, num_mis_in_lpf_unit_height_log2);

  if (!lf_sync->sync_range || sb_rows != lf_sync->rows ||
      num_workers > lf_sync->num_workers) {
    av1_loop_filter_dealloc(lf_sync);
    av1_loop_filter_alloc(lf_sync, cm, sb_rows, cm->width, num_workers);
  }
  lf_sync->lf_mt_exit = false;

  // Initialize cur_sb_col to -1 for all SB rows.
  for (int i = 0; i < MAX_MB_PLANE; i++) {
    memset(lf_sync->cur_sb_col[i], -1,
           sizeof(*(lf_sync->cur_sb_col[i])) * sb_rows);
  }

  enqueue_lf_jobs(lf_sync, start_mi_row, end_mi_row, planes_to_lf,
                  lpf_opt_level, (1 << num_mis_in_lpf_unit_height_log2));
}

static inline AV1LfMTInfo *get_lf_job_info(AV1LfSync *lf_sync) {
  AV1LfMTInfo *cur_job_info = NULL;

#if CONFIG_MULTITHREAD
  pthread_mutex_lock(lf_sync->job_mutex);

  if (!lf_sync->lf_mt_exit && lf_sync->jobs_dequeued < lf_sync->jobs_enqueued) {
    cur_job_info = lf_sync->job_queue + lf_sync->jobs_dequeued;
    lf_sync->jobs_dequeued++;
  }

  pthread_mutex_unlock(lf_sync->job_mutex);
#else
  (void)lf_sync;
#endif

  return cur_job_info;
}

static inline void loop_filter_data_reset(LFWorkerData *lf_data,
                                          YV12_BUFFER_CONFIG *frame_buffer,
                                          struct AV1Common *cm,
                                          MACROBLOCKD *xd) {
  struct macroblockd_plane *pd = xd->plane;
  lf_data->frame_buffer = frame_buffer;
  lf_data->cm = cm;
  lf_data->xd = xd;
  for (int i = 0; i < MAX_MB_PLANE; i++) {
    memcpy(&lf_data->planes[i].dst, &pd[i].dst, sizeof(lf_data->planes[i].dst));
    lf_data->planes[i].subsampling_x = pd[i].subsampling_x;
    lf_data->planes[i].subsampling_y = pd[i].subsampling_y;
  }
}

static inline void set_planes_to_loop_filter(const struct loopfilter *lf,
                                             int planes_to_lf[MAX_MB_PLANE],
                                             int plane_start, int plane_end) {
  // For each luma and chroma plane, whether to filter it or not.
  planes_to_lf[0] = (lf->filter_level[0] || lf->filter_level[1]) &&
                    plane_start <= 0 && 0 < plane_end;
  planes_to_lf[1] = lf->filter_level_u && plane_start <= 1 && 1 < plane_end;
  planes_to_lf[2] = lf->filter_level_v && plane_start <= 2 && 2 < plane_end;
}

static inline int check_planes_to_loop_filter(const struct loopfilter *lf,
                                              int planes_to_lf[MAX_MB_PLANE],
                                              int plane_start, int plane_end) {
  set_planes_to_loop_filter(lf, planes_to_lf, plane_start, plane_end);
  // If the luma plane is purposely not filtered, neither are the chroma
  // planes.
  if (!planes_to_lf[0] && plane_start <= 0 && 0 < plane_end) return 0;
  // Early exit.
  if (!planes_to_lf[0] && !planes_to_lf[1] && !planes_to_lf[2]) return 0;
  return 1;
}

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_COMMON_THREAD_COMMON_H_
