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

#ifndef AOM_AV1_ENCODER_GOP_STRUCTURE_H_
#define AOM_AV1_ENCODER_GOP_STRUCTURE_H_

#include "av1/common/av1_common_int.h"
#include "av1/encoder/ratectrl.h"

#ifdef __cplusplus
extern "C" {
#endif
/*!\cond */
struct AV1_COMP;
struct EncodeFrameParams;

#define MIN_ARF_GF_BOOST 240
#define NORMAL_BOOST 100

/*!\endcond */

/*!\brief Set up the Group-Of-Pictures structure for this GF_GROUP.
 *
 *\ingroup rate_control
 *
 * This function defines the Group-Of-Pictures structure for this GF_GROUP.
 * This involves deciding where to place the various FRAME_UPDATE_TYPEs in
 * the group. It does this primarily by updateing entries in
 * cpi->twopass.gf_group.update_type[].
 *
 * \param[in]    cpi          Top - level encoder instance structure
 *
 * \remark No return value but this function updates group data structures.
 */
void av1_gop_setup_structure(struct AV1_COMP *cpi);

/*!\brief Check whether a frame in the GOP is a forward key frame
 *
 *\ingroup rate_control
 *
 * \param[in]   gf_group       GF/ARF group data structure
 * \param[in]   gf_frame_index GOP index
 *
 * \return Return 1 if it is a forward key frame, otherwise return 0
 */
int av1_gop_check_forward_keyframe(const GF_GROUP *gf_group,
                                   int gf_frame_index);

/*!\brief Check whether a frame in the GOP is the second arf
 *
 *\ingroup rate_control
 *
 * \param[in]   gf_group       GF/ARF group data structure
 * \param[in]   gf_frame_index GOP index
 *
 * \return Return 1 if it is the second arf
 */
int av1_gop_is_second_arf(const GF_GROUP *gf_group, int gf_frame_index);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AV1_ENCODER_GOP_STRUCTURE_H_
