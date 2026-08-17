// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_BINARY_SIZE_LIBSUPERSIZE_VIEWER_CASPIAN_DIFF_H_
#define TOOLS_BINARY_SIZE_LIBSUPERSIZE_VIEWER_CASPIAN_DIFF_H_

#include <string>
#include <vector>

#include "tools/binary_size/libsupersize/viewer/caspian/model.h"

namespace caspian {
DeltaSizeInfo Diff(const SizeInfo* before,
                   const SizeInfo* after,
                   const std::vector<std::string>* removed_sources,
                   const std::vector<std::string>* added_sources);
}

#endif  // TOOLS_BINARY_SIZE_LIBSUPERSIZE_VIEWER_CASPIAN_DIFF_H_
