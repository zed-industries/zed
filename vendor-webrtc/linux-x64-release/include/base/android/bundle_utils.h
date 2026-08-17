// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_BUNDLE_UTILS_H_
#define BASE_ANDROID_BUNDLE_UTILS_H_

#include <string>

#include "base/base_export.h"

namespace base {
namespace android {

// Utils to help working with android app bundles.
class BASE_EXPORT BundleUtils {
 public:
  // Returns true if the current build is a bundle.
  static bool IsBundle();

  // Helper function asking Java to resolve a library path. This is required for
  // resolving a module library made available via SplitCompat, rather than in
  // its eventual fully-installed state.
  static std::string ResolveLibraryPath(const std::string& library_name,
                                        const std::string& split_name);

  // dlopen wrapper that works for partitioned native libraries in dynamic
  // feature modules. This routine looks up the partition's address space in a
  // table of main library symbols, and uses it when loading the feature
  // library. It requires |library_name| (eg. chrome_foo) to resolve the file
  // path (which may be in an interesting location due to SplitCompat) and
  // |partition_name| to look up the load parameters in the main library. These
  // two values may be identical, but since the partition name is set at compile
  // time, and the code is linked into multiple libraries (eg. Chrome vs
  // Monochrome), they may not be.
  static void* DlOpenModuleLibraryPartition(const std::string& library_name,
                                            const std::string& partition,
                                            const std::string& split_name);
};

}  // namespace android
}  // namespace base

#endif  // BASE_ANDROID_BUNDLE_UTILS_H_
