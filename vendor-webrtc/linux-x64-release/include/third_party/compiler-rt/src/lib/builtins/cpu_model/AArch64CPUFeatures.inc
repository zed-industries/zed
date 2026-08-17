//===- AArch64CPUFeatures.inc - AArch64 CPU Features enum -------*- C++ -*-===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
//
// This file defines the CPUFeatures enum for AArch64 to facilitate better
// testing of this code between LLVM and compiler-rt, primarily that the files
// are an exact match.
//
// This file has two identical copies. The primary copy lives in LLVM and
// the other one sits in compiler-rt/lib/builtins/cpu_model directory. To make
// changes in this file, first modify the primary copy and copy it over to
// compiler-rt. compiler-rt tests will fail if the two files are not synced up.
//
//===----------------------------------------------------------------------===//

#ifndef AARCH64_CPU_FEATURS_INC_H
#define AARCH64_CPU_FEATURS_INC_H

// Function Multi Versioning CPU features.
enum CPUFeatures {
  FEAT_RNG,
  FEAT_FLAGM,
  FEAT_FLAGM2,
  FEAT_FP16FML,
  FEAT_DOTPROD,
  FEAT_SM4,
  FEAT_RDM,
  FEAT_LSE,
  FEAT_FP,
  FEAT_SIMD,
  FEAT_CRC,
  FEAT_CSSC,
  FEAT_SHA2,
  FEAT_SHA3,
  RESERVED_FEAT_AES, // previously used and now ABI legacy
  FEAT_PMULL,
  FEAT_FP16,
  FEAT_DIT,
  FEAT_DPB,
  FEAT_DPB2,
  FEAT_JSCVT,
  FEAT_FCMA,
  FEAT_RCPC,
  FEAT_RCPC2,
  FEAT_FRINTTS,
  RESERVED_FEAT_DGH, // previously used and now ABI legacy
  FEAT_I8MM,
  FEAT_BF16,
  RESERVED_FEAT_EBF16, // previously used and now ABI legacy
  RESERVED_FEAT_RPRES, // previously used and now ABI legacy
  FEAT_SVE,
  RESERVED_FEAT_SVE_BF16,  // previously used and now ABI legacy
  RESERVED_FEAT_SVE_EBF16, // previously used and now ABI legacy
  RESERVED_FEAT_SVE_I8MM,  // previously used and now ABI legacy
  FEAT_SVE_F32MM,
  FEAT_SVE_F64MM,
  FEAT_SVE2,
  RESERVED_FEAT_SVE_AES, // previously used and now ABI legacy
  FEAT_SVE_PMULL128,
  FEAT_SVE_BITPERM,
  FEAT_SVE_SHA3,
  FEAT_SVE_SM4,
  FEAT_SME,
  RESERVED_FEAT_MEMTAG, // previously used and now ABI legacy
  FEAT_MEMTAG2,
  RESERVED_FEAT_MEMTAG3, // previously used and now ABI legacy
  FEAT_SB,
  RESERVED_FEAT_PREDRES, // previously used and now ABI legacy
  RESERVED_FEAT_SSBS,    // previously used and now ABI legacy
  FEAT_SSBS2,
  FEAT_BTI,
  RESERVED_FEAT_LS64,         // previously used and now ABI legacy
  RESERVED_FEAT_LS64_V,       // previously used and now ABI legacy
  RESERVED_FEAT_LS64_ACCDATA, // previously used and now ABI legacy
  FEAT_WFXT,
  FEAT_SME_F64,
  FEAT_SME_I64,
  FEAT_SME2,
  FEAT_RCPC3,
  FEAT_MOPS,
  FEAT_MAX,
  FEAT_EXT = 62, // Reserved to indicate presence of additional features field
                 // in __aarch64_cpu_features
  FEAT_INIT      // Used as flag of features initialization completion
};

#endif
