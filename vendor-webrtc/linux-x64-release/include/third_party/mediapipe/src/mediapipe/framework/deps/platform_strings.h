// Copyright 2024 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef MEDIAPIPE_DEPS_PLATFORM_STRINGS_H_
#define MEDIAPIPE_DEPS_PLATFORM_STRINGS_H_
#include <codecvt>
#include <locale>
#include <string>

namespace mediapipe {
// `PlatformString` represents a native string type on the platform.
// `Utf8ToNative`/`NativeToUtf8` convert between UTF-8 and that type.
#if defined(_WIN32) && defined(UNICODE)
using PlatformString = std::wstring;

inline PlatformString Utf8ToNative(const std::string& string) {
  std::wstring_convert<std::codecvt_utf8_utf16<wchar_t>, wchar_t> converter;
  return converter.from_bytes(string.data(), string.data() + string.size());
}
inline std::string NativeToUtf8(const PlatformString& string) {
  std::wstring_convert<std::codecvt_utf8_utf16<wchar_t>, wchar_t> converter;
  return converter.to_bytes(string.data(), string.data() + string.size());
}
#define PLATFORM_STRING_LITERAL_INTERNAL(x) L##x
#define PLATFORM_STRING_LITERAL(x) PLATFORM_STRING_LITERAL_INTERNAL(x)
#else
using PlatformString = std::string;

inline PlatformString Utf8ToNative(const std::string& string) { return string; }
inline std::string NativeToUtf8(const PlatformString& string) { return string; }
#define PLATFORM_STRING_LITERAL(x) x
#endif

// Produces a human-readable message about the last OS error.
std::string FormatLastError();
}  // namespace mediapipe
#endif  // MEDIAPIPE_DEPS_PLATFORM_STRINGS_H_
