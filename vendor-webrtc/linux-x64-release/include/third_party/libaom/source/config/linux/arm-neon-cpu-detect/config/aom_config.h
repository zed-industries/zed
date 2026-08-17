/*
 * Copyright (c) 2025, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */
#ifndef AOM_CONFIG_H_
#define AOM_CONFIG_H_
#define AOM_ARCH_AARCH64 0
#define AOM_ARCH_ARM 1
#define AOM_ARCH_PPC 0
#define AOM_ARCH_RISCV 0
#define AOM_ARCH_X86 0
#define AOM_ARCH_X86_64 0
#define CONFIG_ACCOUNTING 0
#define CONFIG_ANALYZER 0
#define CONFIG_AV1_DECODER 0
#define CONFIG_AV1_ENCODER 1
#define CONFIG_AV1_HIGHBITDEPTH 0
#define CONFIG_AV1_TEMPORAL_DENOISING 1
#define CONFIG_BIG_ENDIAN 0
#define CONFIG_BITRATE_ACCURACY 0
#define CONFIG_BITRATE_ACCURACY_BL 0
#define CONFIG_BITSTREAM_DEBUG 0
#define CONFIG_COEFFICIENT_RANGE_CHECKING 0
#define CONFIG_COLLECT_COMPONENT_TIMING 0
#define CONFIG_COLLECT_PARTITION_STATS 0
#define CONFIG_COLLECT_RD_STATS 0
#define CONFIG_CWG_C013 0
#define CONFIG_CWG_E050 0
#define CONFIG_DEBUG 0
#define CONFIG_DENOISE 1
#define CONFIG_DISABLE_FULL_PIXEL_SPLIT_8X8 1
#define CONFIG_ENTROPY_STATS 0
#define CONFIG_EXCLUDE_SIMD_MISMATCH 0
#define CONFIG_FPMT_TEST 0
#define CONFIG_GCC 1
#define CONFIG_GCOV 0
#define CONFIG_GPROF 0
#define CONFIG_HIGHWAY 0
#define CONFIG_INSPECTION 0
#define CONFIG_INTERNAL_STATS 0
#define CONFIG_INTER_STATS_ONLY 0
#define CONFIG_LIBVMAF_PSNR_PEAK 1
#define CONFIG_LIBYUV 1
#define CONFIG_MAX_DECODE_PROFILE 2
#define CONFIG_MISMATCH_DEBUG 0
#define CONFIG_MULTITHREAD 1
#define CONFIG_NN_V2 0
#define CONFIG_NORMAL_TILE_MODE 0
#define CONFIG_OPTICAL_FLOW_API 0
#define CONFIG_OS_SUPPORT 1
#define CONFIG_OUTPUT_FRAME_SIZE 0
#define CONFIG_PARTITION_SEARCH_ORDER 0
#define CONFIG_PIC 1
#define CONFIG_QUANT_MATRIX 0
#define CONFIG_RATECTRL_LOG 0
#define CONFIG_RD_COMMAND 0
#define CONFIG_RD_DEBUG 0
#define CONFIG_REALTIME_ONLY 1
#define CONFIG_RT_ML_PARTITIONING 0
#define CONFIG_RUNTIME_CPU_DETECT 1
#define CONFIG_SALIENCY_MAP 0
#define CONFIG_SHARED 0
#define CONFIG_SIZE_LIMIT 1
#define CONFIG_SPEED_STATS 0
#define CONFIG_SVT_AV1 1
#define CONFIG_TFLITE 0
#define CONFIG_THREE_PASS 0
#define CONFIG_TUNE_BUTTERAUGLI 0
#define CONFIG_TUNE_VMAF 0
#define CONFIG_WEBM_IO 1
#define DECODE_HEIGHT_LIMIT 16384
#define DECODE_WIDTH_LIMIT 16384
#define FORCE_HIGHBITDEPTH_DECODING 0
#define HAVE_ARM_CRC32 0
#define HAVE_AVX 0
#define HAVE_AVX2 0
#define HAVE_FEXCEPT 1
#define HAVE_MMX 0
#define HAVE_NEON 1
#define HAVE_NEON_DOTPROD 0
#define HAVE_NEON_I8MM 0
#define HAVE_PTHREAD_H 1
#define HAVE_RVV 0
#define HAVE_SSE 0
#define HAVE_SSE2 0
#define HAVE_SSE3 0
#define HAVE_SSE4_1 0
#define HAVE_SSE4_2 0
#define HAVE_SSSE3 0
#define HAVE_SVE 0
#define HAVE_SVE2 0
#define HAVE_UNISTD_H 1
#define HAVE_VSX 0
#define HAVE_WXWIDGETS 0
#define STATIC_LINK_JXL 0
#endif  // AOM_CONFIG_H_
