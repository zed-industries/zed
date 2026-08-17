/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_UTILITY_VP9_CONSTANTS_H_
#define MODULES_VIDEO_CODING_UTILITY_VP9_CONSTANTS_H_

#include <stddef.h>
#include <stdint.h>


namespace webrtc {

// Number of frames that can be stored for future reference.
constexpr size_t kVp9NumRefFrames = 8;
// Number of frame contexts that can be store for future reference.
constexpr size_t kVp9NumFrameContexts = 4;
// Each inter frame can use up to 3 frames for reference.
constexpr size_t kVp9RefsPerFrame = 3;
// Number of values that can be decoded for mv_fr.
constexpr size_t kVp9MvFrSize = 4;
// Number of positions to search in motion vector prediction.
constexpr size_t kVp9MvrefNeighbours = 8;
// Number of contexts when decoding intra_mode .
constexpr size_t kVp9BlockSizeGroups = 4;
// Number of different block sizes used.
constexpr size_t kVp9BlockSizes = 13;
// Sentinel value to mark partition choices that are illegal.
constexpr size_t kVp9BlockInvalid = 14;
// Number of contexts when decoding partition.
constexpr size_t kVp9PartitionContexts = 16;
// Smallest size of a mode info block.
constexpr size_t kVp9MiSize = 8;
// Minimum  width  of a  tile  in  units  of  superblocks  (although tiles on
// the right hand edge can be narrower).
constexpr size_t kVp9MinTileWidth_B64 = 4;
// Maximum width of a tile in units of superblocks.
constexpr size_t kVp9MaxTileWidth_B64 = 64;
// Number of motion vectors returned by find_mv_refs process.
constexpr size_t kVp9MaxMvRefCandidates = 2;
// Number of values that can be derived for ref_frame.
constexpr size_t kVp9MaxRefFrames = 4;
// Number of contexts for is_inter.
constexpr size_t kVp9IsInterContexts = 4;
// Number of contexts for comp_mode.
constexpr size_t kVp9CompModeContexts = 5;
// Number of contexts for single_ref and comp_ref.
constexpr size_t kVp9RefContexts = 5;
// Number of segments allowed in segmentation map.
constexpr size_t kVp9MaxSegments = 8;
// Index for quantizer segment feature.
constexpr size_t kVp9SegLvlAlt_Q = 0;
// Index for loop filter segment feature.
constexpr size_t kVp9SegLvlAlt_L = 1;
// Index for reference frame segment feature.
constexpr size_t kVp9SegLvlRefFrame = 2;
// Index for skip segment feature.
constexpr size_t kVp9SegLvlSkip = 3;
// Number of segment features.
constexpr size_t kVp9SegLvlMax = 4;
// Number of different plane types (Y or UV).
constexpr size_t kVp9BlockTypes = 2;
// Number of different prediction types (intra or inter).
constexpr size_t kVp9RefTypes = 2;
// Number of coefficient bands.
constexpr size_t kVp9CoefBands = 6;
// Number of contexts for decoding coefficients.
constexpr size_t kVp9PrevCoefContexts = 6;
// Number  of  coefficient  probabilities  that  are  directly transmitted.
constexpr size_t kVp9UnconstrainedNodes = 3;
// Number of contexts for transform size.
constexpr size_t kVp9TxSizeContexts = 2;
// Number of values for interp_filter.
constexpr size_t kVp9SwitchableFilters = 3;
// Number of contexts for interp_filter.
constexpr size_t kVp9InterpFilterContexts = 4;
// Number of contexts for decoding skip.
constexpr size_t kVp9SkipContexts = 3;
// Number of values for partition.
constexpr size_t kVp9PartitionTypes = 4;
// Number of values for tx_size.
constexpr size_t kVp9TxSizes = 4;
// Number of values for tx_mode.
constexpr size_t kVp9TxModes = 5;
// Inverse transform rows with DCT and columns with DCT.
constexpr size_t kVp9DctDct = 0;
// Inverse transform rows with DCT and columns with ADST.
constexpr size_t kVp9AdstDct = 1;
// Inverse transform rows with ADST and columns with DCT.
constexpr size_t kVp9DctAdst = 2;
// Inverse transform rows with ADST and columns with ADST.
constexpr size_t kVp9AdstAdst = 3;
// Number of values for y_mode.
constexpr size_t kVp9MbModeCount = 14;
// Number of values for intra_mode.
constexpr size_t kVp9IntraModes = 10;
// Number of values for inter_mode.
constexpr size_t kVp9InterModes = 4;
// Number of contexts for inter_mode.
constexpr size_t kVp9InterModeContexts = 7;
// Number of values for mv_joint.
constexpr size_t kVp9MvJoints = 4;
// Number of values for mv_class.
constexpr size_t kVp9MvClasses = 11;
// Number of values for mv_class0_bit.
constexpr size_t kVp9Class0Size = 2;
// Maximum number of bits for decoding motion vectors.
constexpr size_t kVp9MvOffsetBits = 10;
// Number of values allowed for a probability adjustment.
constexpr size_t kVp9MaxProb = 255;
// Number of different mode types for loop filtering.
constexpr size_t kVp9MaxModeLfDeltas = 2;
// Threshold at which motion vectors are considered large.
constexpr size_t kVp9CompandedMvrefThresh = 8;
// Maximum value used for loop filtering.
constexpr size_t kVp9MaxLoopFilter = 63;
// Number of bits of precision when scaling reference frames.
constexpr size_t kVp9RefScaleShift = 14;
// Number of bits of precision when performing inter prediction.
constexpr size_t kVp9SubpelBits = 4;
// 1 << kVp9SubpelBits.
constexpr size_t kVp9SubpelShifts = 16;
// kVp9SubpelShifts - 1.
constexpr size_t kVp9SubpelMask = 15;
// Value used when clipping motion vectors.
constexpr size_t kVp9MvBorder = 128;
// Value used when clipping motion vectors.
constexpr size_t kVp9InterpExtend = 4;
// Value used when clipping motion vectors.
constexpr size_t kVp9Borderinpixels = 160;
// Value used in adapting probabilities.
constexpr size_t kVp9MaxUpdateFactor = 128;
// Value used in adapting probabilities.
constexpr size_t kVp9CountSat = 20;
// Both candidates use ZEROMV.
constexpr size_t kVp9BothZero = 0;
// One  candidate uses ZEROMV, one uses NEARMV or NEARESTMV.
constexpr size_t kVp9ZeroPlusPredicted = 1;
// Both candidates use NEARMV or NEARESTMV.
constexpr size_t kVp9BothPredicted = 2;
// One candidate uses NEWMV, one uses ZEROMV.
constexpr size_t kVp9NewPlusNonIntra = 3;
// Both candidates use NEWMV.
constexpr size_t kVp9BothNew = 4;
// One candidate uses intra prediction, one uses inter prediction.
constexpr size_t kVp9IntraPlusNonIntra = 5;
// Both candidates use intra prediction.
constexpr size_t kVp9BothIntra = 6;
// Sentinel value marking a case that can never occur.
constexpr size_t kVp9InvalidCase = 9;

enum class Vp9TxMode : uint8_t {
  kOnly4X4 = 0,
  kAllow8X8 = 1,
  kAllow16x16 = 2,
  kAllow32x32 = 3,
  kTxModeSelect = 4
};

enum Vp9BlockSize : uint8_t {
  kBlock4X4 = 0,
  kBlock4X8 = 1,
  kBlock8X4 = 2,
  kBlock8X8 = 3,
  kBlock8X16 = 4,
  kBlock16X8 = 5,
  kBlock16X16 = 6,
  kBlock16X32 = 7,
  kBlock32X16 = 8,
  kBlock32X32 = 9,
  kBlock32X64 = 10,
  kBlock64X32 = 11,
  kBlock64X64 = 12
};

enum Vp9Partition : uint8_t {
  kPartitionNone = 0,
  kPartitionHorizontal = 1,
  kPartitionVertical = 2,
  kPartitionSplit = 3
};

enum class Vp9ReferenceMode : uint8_t {
  kSingleReference = 0,
  kCompoundReference = 1,
  kReferenceModeSelect = 2,
};

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_UTILITY_VP9_CONSTANTS_H_
