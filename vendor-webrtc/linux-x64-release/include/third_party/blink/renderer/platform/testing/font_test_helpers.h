// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_FONT_TEST_HELPERS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_FONT_TEST_HELPERS_H_

#include "base/memory/raw_ptr.h"
#include "build/build_config.h"
#include "third_party/blink/public/platform/web_font_prewarmer.h"
#include "third_party/blink/renderer/platform/fonts/font_description.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class Font;

namespace test {

// Reads a font from a specified path, for use in unit tests only.
Font* CreateTestFont(const AtomicString& family_name,
                     const String& font_path,
                     float size,
                     const FontDescription::VariantLigatures* = nullptr,
                     const FontVariantEmoji variant_emoji = kNormalVariantEmoji,
                     void (*init_font_description)(FontDescription*) = nullptr);

// Reads a font from raw font data, for use in fuzzing test only.
Font* CreateTestFont(const AtomicString& family_name,
                     base::span<const uint8_t> data,
                     float size,
                     const FontDescription::VariantLigatures* = nullptr);

Font* CreateAhemFont(float size);

#if BUILDFLAG(IS_WIN)
class TestFontPrewarmer : public WebFontPrewarmer {
 public:
  void PrewarmFamily(const WebString& family_name) override;

  const Vector<String>& PrewarmedFamilyNames() const { return family_names_; }

 private:
  Vector<String> family_names_;
};

class ScopedTestFontPrewarmer {
 public:
  ScopedTestFontPrewarmer();
  ~ScopedTestFontPrewarmer();

  const Vector<String>& PrewarmedFamilyNames() const {
    return current_.PrewarmedFamilyNames();
  }

 private:
  TestFontPrewarmer current_;
  raw_ptr<WebFontPrewarmer> saved_;
};
#endif

}  // namespace test
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TESTING_FONT_TEST_HELPERS_H_
