/*
 * Copyright (c) 2021, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */
#ifndef AOM_AOM_AOM_EXTERNAL_PARTITION_H_
#define AOM_AOM_AOM_EXTERNAL_PARTITION_H_

/*!\defgroup aom_encoder AOMedia AOM/AV1 Encoder
 * \ingroup aom
 *
 * @{
 */
#include <stdint.h>

/*!\file
 * \brief Provides function pointer definitions for the external partition.
 *
 * \note The external partition API should be considered experimental. Until the
 * external partition API is declared stable, breaking changes may be made to
 * this API in a future libaom release.
 */

/*!\brief Current ABI version number
 *
 * \internal
 * If this file is altered in any way that changes the ABI, this value
 * must be bumped. Examples include, but are not limited to, changing
 * types, removing or reassigning enums, adding/removing/rearranging
 * fields to structures.
 */
#define AOM_EXT_PART_ABI_VERSION 8

#ifdef __cplusplus
extern "C" {
#endif

/*!\brief Abstract external partition model handler
 */
typedef void *aom_ext_part_model_t;

/*!\brief Number of features to determine whether to skip partition none and
 * do partition split directly. The same as "FEATURE_SIZE_SMS_SPLIT".
 */
#define AOM_EXT_PART_SIZE_DIRECT_SPLIT 17

/*!\brief Number of features to use simple motion search to prune out
 * rectangular partition in some direction. The same as
 * "FEATURE_SIZE_SMS_PRUNE_PART".
 */
#define AOM_EXT_PART_SIZE_PRUNE_PART 25

/*!\brief Number of features to prune split and rectangular partition
 * after PARTITION_NONE.
 */
#define AOM_EXT_PART_SIZE_PRUNE_NONE 4

/*!\brief Number of features to terminates partition after partition none using
 * simple_motion_search features and the rate, distortion, and rdcost of
 * PARTITION_NONE. The same as "FEATURE_SIZE_SMS_TERM_NONE".
 */
#define AOM_EXT_PART_SIZE_TERM_NONE 28

/*!\brief Number of features to terminates partition after partition split.
 */
#define AOM_EXT_PART_SIZE_TERM_SPLIT 31

/*!\brief Number of features to prune rectangular partition using stats
 * collected after partition split.
 */
#define AOM_EXT_PART_SIZE_PRUNE_RECT 9

/*!\brief Number of features to prune AB partition using stats
 * collected after rectangular partition..
 */
#define AOM_EXT_PART_SIZE_PRUNE_AB 10

/*!\brief Number of features to prune 4-way partition using stats
 * collected after AB partition.
 */
#define AOM_EXT_PART_SIZE_PRUNE_4_WAY 18

/*!\brief Decision mode of the external partition model.
 * AOM_EXT_PART_WHOLE_TREE: the external partition model should provide the
 * whole partition tree for the superblock.
 *
 * AOM_EXT_PART_RECURSIVE: the external partition model provides the partition
 * decision of the current block only. The decision process starts from
 * the superblock size, down to the smallest block size (4x4) recursively.
 */
typedef enum aom_ext_part_decision_mode {
  AOM_EXT_PART_WHOLE_TREE = 0,
  AOM_EXT_PART_RECURSIVE = 1,
} aom_ext_part_decision_mode_t;

/*!\brief Config information sent to the external partition model.
 *
 * For example, the maximum superblock size determined by the sequence header.
 */
typedef struct aom_ext_part_config {
  int superblock_size;  ///< super block size (either 64x64 or 128x128)
} aom_ext_part_config_t;

/*!\brief Features pass to the external model to make partition decisions.
 * Specifically, features collected before NONE partition.
 * Features "f" are used to determine:
 * partition_none_allowed, partition_horz_allowed, partition_vert_allowed,
 * do_rectangular_split, do_square_split
 * Features "f_part2" are used to determine:
 * prune_horz, prune_vert.
 */
typedef struct aom_partition_features_before_none {
  /*! features to determine whether skip partition none and do split directly */
  float f[AOM_EXT_PART_SIZE_DIRECT_SPLIT];
  /*! features to determine whether to prune rectangular partition */
  float f_part2[AOM_EXT_PART_SIZE_PRUNE_PART];
} aom_partition_features_before_none_t;

/*!\brief Features pass to the external model to make partition decisions.
 * Specifically, features collected after NONE partition.
 */
typedef struct aom_partition_features_none {
  /*! features to prune split and rectangular partition */
  float f[AOM_EXT_PART_SIZE_PRUNE_NONE];
  /*! features to determine termination of partition */
  float f_terminate[AOM_EXT_PART_SIZE_TERM_NONE];
} aom_partition_features_none_t;

/*!\brief Features pass to the external model to make partition decisions.
 * Specifically, features collected after SPLIT partition.
 */
typedef struct aom_partition_features_split {
  /*! features to determine termination of  partition */
  float f_terminate[AOM_EXT_PART_SIZE_TERM_SPLIT];
  /*! features to determine pruning rect partition */
  float f_prune_rect[AOM_EXT_PART_SIZE_PRUNE_RECT];
} aom_partition_features_split_t;

/*!\brief Features pass to the external model to make partition decisions.
 * Specifically, features collected after RECTANGULAR partition.
 */
typedef struct aom_partition_features_rect {
  /*! features to determine pruning AB partition */
  float f[AOM_EXT_PART_SIZE_PRUNE_AB];
} aom_partition_features_rect_t;

/*!\brief Features pass to the external model to make partition decisions.
 * Specifically, features collected after AB partition: HORZ_A, HORZ_B, VERT_A,
 * VERT_B.
 */
typedef struct aom_partition_features_ab {
  /*! features to determine pruning 4-way partition */
  float f[AOM_EXT_PART_SIZE_PRUNE_4_WAY];
} aom_partition_features_ab_t;

/*!\brief Feature id to tell the external model the current stage in partition
 * pruning and what features to use to make decisions accordingly.
 */
typedef enum {
  AOM_EXT_PART_FEATURE_BEFORE_NONE,
  AOM_EXT_PART_FEATURE_BEFORE_NONE_PART2,
  AOM_EXT_PART_FEATURE_AFTER_NONE,
  AOM_EXT_PART_FEATURE_AFTER_NONE_PART2,
  AOM_EXT_PART_FEATURE_AFTER_SPLIT,
  AOM_EXT_PART_FEATURE_AFTER_SPLIT_PART2,
  AOM_EXT_PART_FEATURE_AFTER_RECT,
  AOM_EXT_PART_FEATURE_AFTER_AB
} AOM_EXT_PART_FEATURE_ID;

/*!\brief Features collected from the tpl process.
 *
 * The tpl process collects information that help measure the inter-frame
 * dependency.
 * The tpl process is computed in the unit of tpl_bsize_1d (16x16).
 * Therefore, the max number of units inside a superblock is
 * 128x128 / (16x16) = 64. Change it if the tpl process changes.
 */
typedef struct aom_sb_tpl_features {
  int available;        ///< If tpl stats are available
  int tpl_unit_length;  ///< The block length of tpl process
  int num_units;        ///< The number of units inside the current superblock
  int64_t intra_cost[64];   ///< The intra cost of each unit
  int64_t inter_cost[64];   ///< The inter cost of each unit
  int64_t mc_dep_cost[64];  ///< The motion compensated dependency cost
} aom_sb_tpl_features_t;

/*!\brief Features collected from the simple motion process.
 *
 * The simple motion process collects information by applying motion compensated
 * prediction on each block.
 * The block size is 16x16, which could be changed. If it is changed, update
 * comments and the array size here.
 */
typedef struct aom_sb_simple_motion_features {
  int unit_length;    ///< The block length of the simple motion process
  int num_units;      ///< The number of units inside the current superblock
  int block_sse[64];  ///< Sum of squared error of each unit
  int block_var[64];  ///< Variance of each unit
} aom_sb_simple_motion_features_t;

/*!\brief Features of each super block.
 *
 * Features collected for each super block before partition search.
 */
typedef struct aom_sb_features {
  /*! Features from motion search */
  aom_sb_simple_motion_features_t motion_features;
  /*! Features from tpl process */
  aom_sb_tpl_features_t tpl_features;
} aom_sb_features_t;

/*!\brief Features pass to the external model to make partition decisions.
 *
 * The encoder sends these features to the external model through
 * "func()" defined in .....
 *
 * NOTE: new member variables may be added to this structure in the future.
 * Once new features are finalized, bump the major version of libaom.
 */
typedef struct aom_partition_features {
  // Features for the current supervised multi-stage ML model.
  /*! Feature ID to indicate active features */
  AOM_EXT_PART_FEATURE_ID id;
  /*! Features collected before NONE partition */
  aom_partition_features_before_none_t before_part_none;
  /*! Features collected after NONE partition */
  aom_partition_features_none_t after_part_none;
  /*! Features collected after SPLIT partition */
  aom_partition_features_split_t after_part_split;
  /*! Features collected after RECTANGULAR partition */
  aom_partition_features_rect_t after_part_rect;
  /*! Features collected after AB partition */
  aom_partition_features_ab_t after_part_ab;

  // Features for a new ML model.
  aom_sb_features_t sb_features;  ///< Features collected for the super block
  int mi_row;                     ///< Mi_row position of the block
  int mi_col;                     ///< Mi_col position of the block
  int frame_width;                ///< Frame width
  int frame_height;               ///< Frame height
  int block_size;                 ///< As "BLOCK_SIZE" in av1/common/enums.h
  /*!
   * Valid partition types. A bitmask is used.  "1" represents the
   * corresponding type is valid. The bitmask follows the enum order for
   * PARTITION_TYPE in "enums.h" to represent one partition type at a bit.
   * For example, 0x01 stands for only PARTITION_NONE is valid,
   * 0x09 (00...001001) stands for PARTITION_NONE and PARTITION_SPLIT are valid.
   */
  int valid_partition_types;
  int update_type;    ///< Frame update type, defined in ratectrl.h
  int qindex;         ///< Quantization index, range: [0, 255]
  int rdmult;         ///< Rate-distortion multiplier
  int pyramid_level;  ///< The level of this frame in the hierarchical structure
  int has_above_block;     ///< Has above neighbor block
  int above_block_width;   ///< Width of the above block, -1 if not exist
  int above_block_height;  ///< Height of the above block, -1 if not exist
  int has_left_block;      ///< Has left neighbor block
  int left_block_width;    ///< Width of the left block, -1 if not exist
  int left_block_height;   ///< Height of the left block, -1 if not exist
  /*!
   * The following parameters are collected from applying simple motion search.
   * Sum of squared error (SSE) and variance of motion compensated residual
   * are good indicators of block partitioning.
   * If a block is a square, we also apply motion search for its 4 sub blocks.
   * If not a square, their values are -1.
   * If a block is able to split horizontally, we apply motion search and get
   * stats for horizontal blocks. If not, their values are -1.
   * If a block is able to split vertically, we apply motion search and get
   * stats for vertical blocks. If not, their values are -1.
   */
  unsigned int block_sse;          ///< SSE of motion compensated residual
  unsigned int block_var;          ///< Variance of motion compensated residual
  unsigned int sub_block_sse[4];   ///< SSE of sub blocks.
  unsigned int sub_block_var[4];   ///< Variance of sub blocks.
  unsigned int horz_block_sse[2];  ///< SSE of horz sub blocks
  unsigned int horz_block_var[2];  ///< Variance of horz sub blocks
  unsigned int vert_block_sse[2];  ///< SSE of vert sub blocks
  unsigned int vert_block_var[2];  ///< Variance of vert sub blocks
  /*!
   * The following parameters are calculated from tpl model.
   * If tpl model is not available, their values are -1.
   */
  int64_t tpl_intra_cost;   ///< Intra cost, ref to "TplDepStats" in tpl_model.h
  int64_t tpl_inter_cost;   ///< Inter cost in tpl model
  int64_t tpl_mc_dep_cost;  ///< Motion compensated dependency cost in tpl model
} aom_partition_features_t;

/*!\brief Partition decisions received from the external model.
 *
 * The encoder receives partition decisions and encodes the superblock
 * with the given partition type.
 * The encoder receives it from "func()" define in ....
 *
 * NOTE: new member variables may be added to this structure in the future.
 * Once new features are finalized, bump the major version of libaom.
 */
typedef struct aom_partition_decision {
  // Decisions for directly set partition types
  int is_final_decision;         ///< The flag whether it's the final decision
  int num_nodes;                 ///< The number of leaf nodes
  int partition_decision[2048];  ///< Partition decisions
  int current_decision;          ///< Partition decision for the current block

  // Decisions for partition type pruning
  int terminate_partition_search;  ///< Terminate further partition search
  int partition_none_allowed;      ///< Allow partition none type
  int partition_rect_allowed[2];   ///< Allow rectangular partitions
  int do_rectangular_split;        ///< Try rectangular split partition
  int do_square_split;             ///< Try square split partition
  int prune_rect_part[2];          ///< Prune rectangular partition
  int horza_partition_allowed;     ///< Allow HORZ_A partition
  int horzb_partition_allowed;     ///< Allow HORZ_B partition
  int verta_partition_allowed;     ///< Allow VERT_A partition
  int vertb_partition_allowed;     ///< Allow VERT_B partition
  int partition_horz4_allowed;     ///< Allow HORZ4 partition
  int partition_vert4_allowed;     ///< Allow VERT4 partition
} aom_partition_decision_t;

/*!\brief Encoding stats for the given partition decision.
 *
 * The encoding stats collected by encoding the superblock with the
 * given partition types.
 * The encoder sends the stats to the external model for training
 * or inference through "func()" defined in ....
 */
typedef struct aom_partition_stats {
  int rate;        ///< Rate cost of the block
  int64_t dist;    ///< Distortion of the block
  int64_t rdcost;  ///< Rate-distortion cost of the block
} aom_partition_stats_t;

/*!\brief Enum for return status.
 */
typedef enum aom_ext_part_status {
  AOM_EXT_PART_OK = 0,     ///< Status of success
  AOM_EXT_PART_ERROR = 1,  ///< Status of failure
  AOM_EXT_PART_TEST = 2,   ///< Status used for tests
} aom_ext_part_status_t;

/*!\brief Callback of creating an external partition model.
 *
 * The callback is invoked by the encoder to create an external partition
 * model.
 *
 * \param[in] priv Callback's private data
 * \param[in] part_config Config information pointer for model creation
 * \param[out] ext_part_model Pointer to the model
 */
typedef aom_ext_part_status_t (*aom_ext_part_create_model_fn_t)(
    void *priv, const aom_ext_part_config_t *part_config,
    aom_ext_part_model_t *ext_part_model);

/*!\brief Callback of sending features to the external partition model.
 *
 * The callback is invoked by the encoder to send features to the external
 * partition model.
 *
 * \param[in] ext_part_model The external model
 * \param[in] part_features Pointer to the features
 */
typedef aom_ext_part_status_t (*aom_ext_part_send_features_fn_t)(
    aom_ext_part_model_t ext_part_model,
    const aom_partition_features_t *part_features);

/*!\brief Callback of receiving partition decisions from the external
 * partition model.
 *
 * The callback is invoked by the encoder to receive partition decisions from
 * the external partition model.
 *
 * \param[in] ext_part_model The external model
 * \param[in] ext_part_decision Pointer to the partition decisions
 */
typedef aom_ext_part_status_t (*aom_ext_part_get_decision_fn_t)(
    aom_ext_part_model_t ext_part_model,
    aom_partition_decision_t *ext_part_decision);

/*!\brief Callback of sending stats to the external partition model.
 *
 * The callback is invoked by the encoder to send encoding stats to
 * the external partition model.
 *
 * \param[in] ext_part_model The external model
 * \param[in] ext_part_stats Pointer to the encoding stats
 */
typedef aom_ext_part_status_t (*aom_ext_part_send_partition_stats_fn_t)(
    aom_ext_part_model_t ext_part_model,
    const aom_partition_stats_t *ext_part_stats);

/*!\brief Callback of deleting the external partition model.
 *
 * The callback is invoked by the encoder to delete the external partition
 * model.
 *
 * \param[in] ext_part_model The external model
 */
typedef aom_ext_part_status_t (*aom_ext_part_delete_model_fn_t)(
    aom_ext_part_model_t ext_part_model);

/*!\brief Callback function set for external partition model.
 *
 * Uses can enable external partition model by registering a set of
 * callback functions with the flag: AV1E_SET_EXTERNAL_PARTITION_MODEL
 */
typedef struct aom_ext_part_funcs {
  /*!
   * Create an external partition model.
   */
  aom_ext_part_create_model_fn_t create_model;

  /*!
   * Send features to the external partition model to make partition decisions.
   */
  aom_ext_part_send_features_fn_t send_features;

  /*!
   * Get partition decisions from the external partition model.
   */
  aom_ext_part_get_decision_fn_t get_partition_decision;

  /*!
   * Send stats of the current partition to the external model.
   */
  aom_ext_part_send_partition_stats_fn_t send_partition_stats;

  /*!
   * Delete the external partition model.
   */
  aom_ext_part_delete_model_fn_t delete_model;

  /*!
   * The decision mode of the model.
   */
  aom_ext_part_decision_mode_t decision_mode;

  /*!
   * Private data for the external partition model.
   */
  void *priv;
} aom_ext_part_funcs_t;

/*!@} - end defgroup aom_encoder*/
#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AOM_AOM_EXTERNAL_PARTITION_H_
