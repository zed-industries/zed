/*
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with FFmpeg; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

#ifndef AVCODEC_SEI_H
#define AVCODEC_SEI_H

// SEI payload types form a common namespace between the H.264, H.265
// and H.266 standards.  A given payload type always has the same
// meaning, but some names have different payload types in different
// standards (e.g. scalable-nesting is 30 in H.264 but 133 in H.265).
// The content of the payload data depends on the standard, though
// many generic parts have the same interpretation everywhere (such as
// mastering-display-colour-volume and user-data-unregistered).
enum SEIType {
    SEI_TYPE_BUFFERING_PERIOD                            = 0,
    SEI_TYPE_PIC_TIMING                                  = 1,
    SEI_TYPE_PAN_SCAN_RECT                               = 2,
    SEI_TYPE_FILLER_PAYLOAD                              = 3,
    SEI_TYPE_USER_DATA_REGISTERED_ITU_T_T35              = 4,
    SEI_TYPE_USER_DATA_UNREGISTERED                      = 5,
    SEI_TYPE_RECOVERY_POINT                              = 6,
    SEI_TYPE_DEC_REF_PIC_MARKING_REPETITION              = 7,
    SEI_TYPE_SPARE_PIC                                   = 8,
    SEI_TYPE_SCENE_INFO                                  = 9,
    SEI_TYPE_SUB_SEQ_INFO                                = 10,
    SEI_TYPE_SUB_SEQ_LAYER_CHARACTERISTICS               = 11,
    SEI_TYPE_SUB_SEQ_CHARACTERISTICS                     = 12,
    SEI_TYPE_FULL_FRAME_FREEZE                           = 13,
    SEI_TYPE_FULL_FRAME_FREEZE_RELEASE                   = 14,
    SEI_TYPE_FULL_FRAME_SNAPSHOT                         = 15,
    SEI_TYPE_PROGRESSIVE_REFINEMENT_SEGMENT_START        = 16,
    SEI_TYPE_PROGRESSIVE_REFINEMENT_SEGMENT_END          = 17,
    SEI_TYPE_MOTION_CONSTRAINED_SLICE_GROUP_SET          = 18,
    SEI_TYPE_FILM_GRAIN_CHARACTERISTICS                  = 19,
    SEI_TYPE_DEBLOCKING_FILTER_DISPLAY_PREFERENCE        = 20,
    SEI_TYPE_STEREO_VIDEO_INFO                           = 21,
    SEI_TYPE_POST_FILTER_HINT                            = 22,
    SEI_TYPE_TONE_MAPPING_INFO                           = 23,
    SEI_TYPE_SCALABILITY_INFO                            = 24,
    SEI_TYPE_SUB_PIC_SCALABLE_LAYER                      = 25,
    SEI_TYPE_NON_REQUIRED_LAYER_REP                      = 26,
    SEI_TYPE_PRIORITY_LAYER_INFO                         = 27,
    SEI_TYPE_LAYERS_NOT_PRESENT_4                        = 28,
    SEI_TYPE_LAYER_DEPENDENCY_CHANGE                     = 29,
    SEI_TYPE_SCALABLE_NESTING_4                          = 30,
    SEI_TYPE_BASE_LAYER_TEMPORAL_HRD                     = 31,
    SEI_TYPE_QUALITY_LAYER_INTEGRITY_CHECK               = 32,
    SEI_TYPE_REDUNDANT_PIC_PROPERTY                      = 33,
    SEI_TYPE_TL0_DEP_REP_INDEX                           = 34,
    SEI_TYPE_TL_SWITCHING_POINT                          = 35,
    SEI_TYPE_PARALLEL_DECODING_INFO                      = 36,
    SEI_TYPE_MVC_SCALABLE_NESTING                        = 37,
    SEI_TYPE_VIEW_SCALABILITY_INFO                       = 38,
    SEI_TYPE_MULTIVIEW_SCENE_INFO_4                      = 39,
    SEI_TYPE_MULTIVIEW_ACQUISITION_INFO_4                = 40,
    SEI_TYPE_NON_REQUIRED_VIEW_COMPONENT                 = 41,
    SEI_TYPE_VIEW_DEPENDENCY_CHANGE                      = 42,
    SEI_TYPE_OPERATION_POINTS_NOT_PRESENT                = 43,
    SEI_TYPE_BASE_VIEW_TEMPORAL_HRD                      = 44,
    SEI_TYPE_FRAME_PACKING_ARRANGEMENT                   = 45,
    SEI_TYPE_MULTIVIEW_VIEW_POSITION_4                   = 46,
    SEI_TYPE_DISPLAY_ORIENTATION                         = 47,
    SEI_TYPE_MVCD_SCALABLE_NESTING                       = 48,
    SEI_TYPE_MVCD_VIEW_SCALABILITY_INFO                  = 49,
    SEI_TYPE_DEPTH_REPRESENTATION_INFO_4                 = 50,
    SEI_TYPE_THREE_DIMENSIONAL_REFERENCE_DISPLAYS_INFO_4 = 51,
    SEI_TYPE_DEPTH_TIMING                                = 52,
    SEI_TYPE_DEPTH_SAMPLING_INFO                         = 53,
    SEI_TYPE_CONSTRAINED_DEPTH_PARAMETER_SET_IDENTIFIER  = 54,
    SEI_TYPE_GREEN_METADATA                              = 56,
    SEI_TYPE_STRUCTURE_OF_PICTURES_INFO                  = 128,
    SEI_TYPE_ACTIVE_PARAMETER_SETS                       = 129,
    SEI_TYPE_PARAMETER_SETS_INCLUSION_INDICATION         = SEI_TYPE_ACTIVE_PARAMETER_SETS,
    SEI_TYPE_DECODING_UNIT_INFO                          = 130,
    SEI_TYPE_TEMPORAL_SUB_LAYER_ZERO_IDX                 = 131,
    SEI_TYPE_DECODED_PICTURE_HASH                        = 132,
    SEI_TYPE_SCALABLE_NESTING_5                          = 133,
    SEI_TYPE_REGION_REFRESH_INFO                         = 134,
    SEI_TYPE_NO_DISPLAY                                  = 135,
    SEI_TYPE_TIME_CODE                                   = 136,
    SEI_TYPE_MASTERING_DISPLAY_COLOUR_VOLUME             = 137,
    SEI_TYPE_SEGMENTED_RECT_FRAME_PACKING_ARRANGEMENT    = 138,
    SEI_TYPE_TEMPORAL_MOTION_CONSTRAINED_TILE_SETS       = 139,
    SEI_TYPE_CHROMA_RESAMPLING_FILTER_HINT               = 140,
    SEI_TYPE_KNEE_FUNCTION_INFO                          = 141,
    SEI_TYPE_COLOUR_REMAPPING_INFO                       = 142,
    SEI_TYPE_DEINTERLACED_FIELD_IDENTIFICATION           = 143,
    SEI_TYPE_CONTENT_LIGHT_LEVEL_INFO                    = 144,
    SEI_TYPE_DEPENDENT_RAP_INDICATION                    = 145,
    SEI_TYPE_CODED_REGION_COMPLETION                     = 146,
    SEI_TYPE_ALTERNATIVE_TRANSFER_CHARACTERISTICS        = 147,
    SEI_TYPE_AMBIENT_VIEWING_ENVIRONMENT                 = 148,
    SEI_TYPE_CONTENT_COLOUR_VOLUME                       = 149,
    SEI_TYPE_EQUIRECTANGULAR_PROJECTION                  = 150,
    SEI_TYPE_CUBEMAP_PROJECTION                          = 151,
    SEI_TYPE_FISHEYE_VIDEO_INFO                          = 152,
    SEI_TYPE_SPHERE_ROTATION                             = 154,
    SEI_TYPE_REGIONWISE_PACKING                          = 155,
    SEI_TYPE_OMNI_VIEWPORT                               = 156,
    SEI_TYPE_REGIONAL_NESTING                            = 157,
    SEI_TYPE_MCTS_EXTRACTION_INFO_SETS                   = 158,
    SEI_TYPE_MCTS_EXTRACTION_INFO_NESTING                = 159,
    SEI_TYPE_LAYERS_NOT_PRESENT_5                        = 160,
    SEI_TYPE_INTER_LAYER_CONSTRAINED_TILE_SETS           = 161,
    SEI_TYPE_BSP_NESTING                                 = 162,
    SEI_TYPE_BSP_INITIAL_ARRIVAL_TIME                    = 163,
    SEI_TYPE_SUB_BITSTREAM_PROPERTY                      = 164,
    SEI_TYPE_ALPHA_CHANNEL_INFO                          = 165,
    SEI_TYPE_OVERLAY_INFO                                = 166,
    SEI_TYPE_TEMPORAL_MV_PREDICTION_CONSTRAINTS          = 167,
    SEI_TYPE_FRAME_FIELD_INFO                            = 168,
    SEI_TYPE_THREE_DIMENSIONAL_REFERENCE_DISPLAYS_INFO   = 176,
    SEI_TYPE_DEPTH_REPRESENTATION_INFO_5                 = 177,
    SEI_TYPE_MULTIVIEW_SCENE_INFO_5                      = 178,
    SEI_TYPE_MULTIVIEW_ACQUISITION_INFO_5                = 179,
    SEI_TYPE_MULTIVIEW_VIEW_POSITION_5                   = 180,
    SEI_TYPE_ALTERNATIVE_DEPTH_INFO                      = 181,
    SEI_TYPE_SEI_MANIFEST                                = 200,
    SEI_TYPE_SEI_PREFIX_INDICATION                       = 201,
    SEI_TYPE_ANNOTATED_REGIONS                           = 202,
    SEI_TYPE_SUBPIC_LEVEL_INFO                           = 203,
    SEI_TYPE_SAMPLE_ASPECT_RATIO_INFO                    = 204,
};

/**
 * frame_packing_arrangement types. H.265 and H.274 use only 3..5
 * with all the other values being reserved. H.264 uses a few more values
 * that are prefixed with SEI_FPA_H264 in the enum below.
 *
 * The semantics of the common values are the same for all standards.
 */
typedef enum {
    SEI_FPA_H264_TYPE_CHECKERBOARD        = 0,
    SEI_FPA_H264_TYPE_INTERLEAVE_COLUMN   = 1,
    SEI_FPA_H264_TYPE_INTERLEAVE_ROW      = 2,
    SEI_FPA_TYPE_SIDE_BY_SIDE             = 3,
    SEI_FPA_TYPE_TOP_BOTTOM               = 4,
    SEI_FPA_TYPE_INTERLEAVE_TEMPORAL      = 5,
    SEI_FPA_H264_TYPE_2D                  = 6,
} SEIFpaType;

#endif /* AVCODEC_SEI_H */
