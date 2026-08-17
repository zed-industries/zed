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
 * \brief Declares frame encoding functions.
 */
#ifndef AOM_AV1_ENCODER_ENCODE_STRATEGY_H_
#define AOM_AV1_ENCODER_ENCODE_STRATEGY_H_

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>

#include "aom/aom_encoder.h"

#include "av1/encoder/encoder.h"
#include "av1/encoder/firstpass.h"

/*!\brief Implement high-level encode strategy
 *
 * \ingroup high_level_algo
 * \callgraph
 * \callergraph
 * This function will implement high-level encode strategy, choosing frame type,
 * frame placement, etc. It populates an EncodeFrameParams struct with the
 * results of these decisions and then encodes the frame. The caller should use
 * the output parameters *time_stamp and *time_end only when this function
 * returns AOM_CODEC_OK.
 *
 * \param[in]    cpi         Top-level encoder structure
 * \param[out]   size        Bitstream size
 * \param[out]   dest        Bitstream output buffer
 * \param[in]    dest_size   Bitstream output buffer size
 * \param[in]    frame_flags Flags to decide how to encoding the frame
 * \param[out]   time_stamp  Time stamp of the frame
 * \param[out]   time_end    Time end
 * \param[in]    timestamp_ratio Time base
 * \param[in]    pop_lookahead Decide to pop the source frame from queue
 * \param[in]    flush       Decide to encode one frame or the rest of frames
 *
 * \return Returns a value to indicate if the encoding is done successfully.
 * \retval #AOM_CODEC_OK
 * \retval -1
 * \retval #AOM_CODEC_ERROR
 */
int av1_encode_strategy(AV1_COMP *const cpi, size_t *const size,
                        uint8_t *const dest, size_t dest_size,
                        unsigned int *frame_flags, int64_t *const time_stamp,
                        int64_t *const time_end,
                        const aom_rational64_t *const timestamp_ratio,
                        int *const pop_lookahead, int flush);

/*!\cond */
// Set individual buffer update flags based on frame reference type.
// force_refresh_all is used when we have a KEY_FRAME or S_FRAME.  It forces all
// refresh_*_frame flags to be set, because we refresh all buffers in this case.
void av1_configure_buffer_updates(AV1_COMP *const cpi,
                                  RefreshFrameInfo *const refresh_frame,
                                  const FRAME_UPDATE_TYPE type,
                                  const REFBUF_STATE refbuf_state,
                                  int force_refresh_all);

int av1_get_refresh_frame_flags(
    const AV1_COMP *const cpi, const EncodeFrameParams *const frame_params,
    FRAME_UPDATE_TYPE frame_update_type, int gf_index, int cur_disp_order,
    RefFrameMapPair ref_frame_map_pairs[REF_FRAMES]);

int av1_get_refresh_ref_frame_map(int refresh_frame_flags);

/*!\brief Obtain indices of reference frames in ref_frame_map
 *
 * \callgraph
 * \callergraph
 *
 * \param[out]   remapped_ref_idx  An array for storing indices of reference
 *                                 frames. The index is used to retrieve a
 *                                 reference frame buffer from ref_frame_map
 *                                 in AV1Common.
 */
void av1_get_ref_frames(RefFrameMapPair ref_frame_map_pairs[REF_FRAMES],
                        int cur_frame_disp, const AV1_COMP *cpi, int gf_index,
                        int is_parallel_encode,
                        int remapped_ref_idx[REF_FRAMES]);

int is_forced_keyframe_pending(struct lookahead_ctx *lookahead,
                               const int up_to_index,
                               const COMPRESSOR_STAGE compressor_stage);

static inline int is_frame_droppable(
    const RTC_REF *const rtc_ref,
    const ExtRefreshFrameFlagsInfo *const ext_refresh_frame_flags) {
  // Droppable frame is only used by external refresh flags. VoD setting won't
  // trigger its use case.
  if (rtc_ref->set_ref_frame_config)
    return rtc_ref->non_reference_frame;
  else if (ext_refresh_frame_flags->update_pending)
    return !(ext_refresh_frame_flags->alt_ref_frame ||
             ext_refresh_frame_flags->alt2_ref_frame ||
             ext_refresh_frame_flags->bwd_ref_frame ||
             ext_refresh_frame_flags->golden_frame ||
             ext_refresh_frame_flags->last_frame);
  else
    return 0;
}

static inline int get_current_frame_ref_type(const AV1_COMP *const cpi) {
  // We choose the reference "type" of this frame from the flags which indicate
  // which reference frames will be refreshed by it. More than one of these
  // flags may be set, so the order here implies an order of precedence. This is
  // just used to choose the primary_ref_frame (as the most recent reference
  // buffer of the same reference-type as the current frame).

  switch (cpi->ppi->gf_group.layer_depth[cpi->gf_frame_index]) {
    case 0: return 0;
    case 1: return 1;
    case MAX_ARF_LAYERS:
    case MAX_ARF_LAYERS + 1: return 4;
    default: return 7;
  }
}

int av1_calc_refresh_idx_for_intnl_arf(
    AV1_COMP *cpi, RefFrameMapPair ref_frame_map_pairs[REF_FRAMES],
    int gf_index);
/*!\endcond */
#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_ENCODER_ENCODE_STRATEGY_H_
