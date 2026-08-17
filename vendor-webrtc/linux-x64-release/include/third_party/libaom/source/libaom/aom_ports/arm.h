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

#ifndef AOM_AOM_PORTS_ARM_H_
#define AOM_AOM_PORTS_ARM_H_
#include <stdlib.h>

#include "config/aom_config.h"

#ifdef __cplusplus
extern "C" {
#endif

// Armv7-A optional Neon instructions, mandatory from Armv8.0-A.
#define HAS_NEON (1 << 0)
// Armv8.0-A optional CRC32 instructions, mandatory from Armv8.1-A.
#define HAS_ARM_CRC32 (1 << 1)
// Armv8.2-A optional Neon dot-product instructions, mandatory from Armv8.4-A.
#define HAS_NEON_DOTPROD (1 << 2)
// Armv8.2-A optional Neon i8mm instructions, mandatory from Armv8.6-A.
#define HAS_NEON_I8MM (1 << 3)
// Armv8.2-A optional SVE instructions, mandatory from Armv9.0-A.
#define HAS_SVE (1 << 4)
// Armv9.0-A SVE2 instructions.
#define HAS_SVE2 (1 << 5)

int aom_arm_cpu_caps(void);

// Earlier gcc compilers have issues with some neon intrinsics
#if !defined(__clang__) && defined(__GNUC__) && __GNUC__ == 4 && \
    __GNUC_MINOR__ <= 6
#define AOM_INCOMPATIBLE_GCC
#endif

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  // AOM_AOM_PORTS_ARM_H_
