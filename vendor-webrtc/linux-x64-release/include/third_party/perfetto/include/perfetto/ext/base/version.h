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

#ifndef INCLUDE_PERFETTO_EXT_BASE_VERSION_H_
#define INCLUDE_PERFETTO_EXT_BASE_VERSION_H_

namespace perfetto {
namespace base {

// The returned pointer is a static string and safe to pass around.
// Returns a human readable string currently of the approximate form:
// Perfetto v42.1-deadbeef0 (deadbeef03c641e4b4ea9cf38e9b5696670175a9)
// However you should not depend on the format of this string.
// It maybe not be possible to determine the version. In which case the
// string will be of the approximate form:
// Perfetto v0.0 (unknown)
const char* GetVersionString();

// The returned pointer is a static string and safe to pass around.
// Returns the short code used to identity the version:
// v42.1-deadbeef0
// It maybe not be possible to determine the version. In which case
// this returns nullptr.
// This can be compared with equality to other
// version codes to detect matched builds (for example to see if
// trace_processor_shell and the UI were built at the same revision)
// but you should not attempt to parse it as the format may change
// without warning.
const char* GetVersionCode();

}  // namespace base
}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_EXT_BASE_VERSION_H_
