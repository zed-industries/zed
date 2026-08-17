/*
 * Copyright (c) 2023, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#include "aom_ports/arm.h"
#include "config/aom_config.h"

#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#if defined(_WIN32)
#undef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#undef WIN32_EXTRA_LEAN
#define WIN32_EXTRA_LEAN
#include <windows.h>
#endif

#ifdef WINAPI_FAMILY
#include <winapifamily.h>
#if !WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)
#define getenv(x) NULL
#endif
#endif

#if defined(__ANDROID__) && (__ANDROID_API__ < 18)
#define AOM_USE_ANDROID_CPU_FEATURES 1
// Use getauxval() when targeting (64-bit) Android with API level >= 18.
// getauxval() is supported since Android API level 18 (Android 4.3.)
// First Android version with 64-bit support was Android 5.x (API level 21).
#include <cpu-features.h>
#endif

static bool arm_cpu_env_flags(int *flags) {
  const char *env = getenv("AOM_SIMD_CAPS");
  if (env && *env) {
    *flags = (int)strtol(env, NULL, 0);
    return true;
  }
  return false;
}

static int arm_cpu_env_mask(void) {
  const char *env = getenv("AOM_SIMD_CAPS_MASK");
  return env && *env ? (int)strtol(env, NULL, 0) : ~0;
}
