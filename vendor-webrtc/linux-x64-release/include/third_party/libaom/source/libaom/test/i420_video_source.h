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
#ifndef AOM_TEST_I420_VIDEO_SOURCE_H_
#define AOM_TEST_I420_VIDEO_SOURCE_H_
#include <cstdio>
#include <cstdlib>
#include <string>

#include "test/yuv_video_source.h"

namespace libaom_test {

// This class extends VideoSource to allow parsing of raw yv12
// so that we can do actual file encodes.
class I420VideoSource : public YUVVideoSource {
 public:
  I420VideoSource(const std::string &file_name, unsigned int width,
                  unsigned int height, int rate_numerator, int rate_denominator,
                  unsigned int start, int limit)
      : YUVVideoSource(file_name, AOM_IMG_FMT_I420, width, height,
                       rate_numerator, rate_denominator, start, limit) {}
};

}  // namespace libaom_test

#endif  // AOM_TEST_I420_VIDEO_SOURCE_H_
