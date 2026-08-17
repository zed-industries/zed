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

#ifndef AOM_AOM_MEM_INCLUDE_AOM_MEM_INTRNL_H_
#define AOM_AOM_MEM_INCLUDE_AOM_MEM_INTRNL_H_

#include "config/aom_config.h"

#define ADDRESS_STORAGE_SIZE sizeof(size_t)

#ifndef DEFAULT_ALIGNMENT
#if defined(VXWORKS)
/*default addr alignment to use in calls to aom_* functions other than
  aom_memalign*/
#define DEFAULT_ALIGNMENT 32
#else
#define DEFAULT_ALIGNMENT (2 * sizeof(void *)) /* NOLINT */
#endif
#endif

#endif  // AOM_AOM_MEM_INCLUDE_AOM_MEM_INTRNL_H_
