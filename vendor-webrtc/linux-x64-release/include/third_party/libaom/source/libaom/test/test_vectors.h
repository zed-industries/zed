/*
 * Copyright (c) 2018, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

#ifndef AOM_TEST_TEST_VECTORS_H_
#define AOM_TEST_TEST_VECTORS_H_

#include "config/aom_config.h"

namespace libaom_test {

#if CONFIG_AV1_DECODER
extern const int kNumAV1TestVectors;
extern const char *const kAV1TestVectors[];
#endif

}  // namespace libaom_test

#endif  // AOM_TEST_TEST_VECTORS_H_
