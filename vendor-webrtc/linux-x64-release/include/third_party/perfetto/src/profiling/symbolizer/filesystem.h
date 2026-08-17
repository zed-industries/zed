/*
 * Copyright (C) 2020 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_PROFILING_SYMBOLIZER_FILESYSTEM_H_
#define SRC_PROFILING_SYMBOLIZER_FILESYSTEM_H_

#include "src/profiling/symbolizer/local_symbolizer.h"

namespace perfetto {
namespace profiling {

using FileCallback = std::function<void(const char*, size_t)>;
bool WalkDirectories(std::vector<std::string> dirs, FileCallback fn);

}  // namespace profiling
}  // namespace perfetto

#endif  // SRC_PROFILING_SYMBOLIZER_FILESYSTEM_H_
