/*
 * Copyright (C) 2024 The Android Open Source Project
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

#ifndef SRC_TRACECONV_TRACE_TO_FIREFOX_H_
#define SRC_TRACECONV_TRACE_TO_FIREFOX_H_

#include <iostream>

namespace perfetto {
namespace trace_to_text {

// Exports trace as as Firefox Profile. More details here:
// https://firefox-source-docs.mozilla.org/tools/profiler/code-overview.html
// https://github.com/firefox-devtools/profiler/blob/main/src/types/profile.js
bool TraceToFirefoxProfile(std::istream* input, std::ostream* output);

}  // namespace trace_to_text
}  // namespace perfetto

#endif  // SRC_TRACECONV_TRACE_TO_FIREFOX_H_
