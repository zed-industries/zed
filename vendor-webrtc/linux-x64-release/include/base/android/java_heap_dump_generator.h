// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_JAVA_HEAP_DUMP_GENERATOR_H_
#define BASE_ANDROID_JAVA_HEAP_DUMP_GENERATOR_H_

#include <string_view>

#include "base/android/scoped_java_ref.h"
#include "base/base_export.h"

namespace base {
namespace android {

// Generates heap dump and writes it to a file at |file_path|. Returns true on
// success. The heap dump is generated through the Android Java system API
// android.os.Debug#dumpHprofData(...)
BASE_EXPORT bool WriteJavaHeapDumpToPath(std::string_view file_path);

}  // namespace android
}  // namespace base

#endif  // BASE_ANDROID_JAVA_HEAP_DUMP_GENERATOR_H_
