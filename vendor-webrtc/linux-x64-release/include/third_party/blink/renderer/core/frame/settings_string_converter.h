// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_SETTINGS_STRING_CONVERTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_SETTINGS_STRING_CONVERTER_H_

#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

// Converter from String to Type for the generated Settings::SetFromStrings().

namespace blink {

template <typename T>
struct FromString {
  T operator()(const String& s) { return static_cast<T>(s.ToInt()); }
};

template <>
struct FromString<String> {
  const String& operator()(const String& s) { return s; }
};

template <>
struct FromString<bool> {
  bool operator()(const String& s) { return s.empty() || s == "true"; }
};

template <>
struct FromString<float> {
  float operator()(const String& s) { return s.ToFloat(); }
};

template <>
struct FromString<double> {
  double operator()(const String& s) { return s.ToDouble(); }
};

template <>
struct FromString<gfx::Size> {
  gfx::Size operator()(const String& s) {
    Vector<String> fields;
    s.Split(',', fields);
    return gfx::Size(fields.size() > 0 ? fields[0].ToInt() : 0,
                     fields.size() > 1 ? fields[1].ToInt() : 0);
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_SETTINGS_STRING_CONVERTER_H_
