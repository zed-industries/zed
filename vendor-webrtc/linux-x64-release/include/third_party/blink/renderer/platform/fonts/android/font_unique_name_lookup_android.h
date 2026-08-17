// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_ANDROID_FONT_UNIQUE_NAME_LOOKUP_ANDROID_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_ANDROID_FONT_UNIQUE_NAME_LOOKUP_ANDROID_H_

#include "base/sequence_checker.h"
#include "mojo/public/cpp/bindings/remote.h"
#include "third_party/blink/public/common/font_unique_name_lookup/font_table_matcher.h"
#include "third_party/blink/public/mojom/android_font_lookup/android_font_lookup.mojom-blink.h"
#include "third_party/blink/public/mojom/font_unique_name_lookup/font_unique_name_lookup.mojom-blink.h"
#include "third_party/blink/renderer/platform/fonts/font_unique_name_lookup.h"
#include "third_party/blink/renderer/platform/wtf/deque.h"

namespace blink {

// Unique font lookup implementation for Android, uses two backends: Fonts from
// the firmware in the font directories indexed by
// content/browser/font_unique_name_lookup/font_unique_name_lookup.cc as well
// as the Mojo IPC connection to a java implementation that fetches fonts from
// GMSCore, see
// content/public/android/java/src/org/chromium/content/browser/font/AndroidFontLookupImpl.java
class FontUniqueNameLookupAndroid : public FontUniqueNameLookup {
 public:
  FontUniqueNameLookupAndroid() = default;
  FontUniqueNameLookupAndroid(const FontUniqueNameLookupAndroid&) = delete;
  FontUniqueNameLookupAndroid& operator=(const FontUniqueNameLookupAndroid&) =
      delete;
  ~FontUniqueNameLookupAndroid() override;

  bool IsFontUniqueNameLookupReadyForSyncLookup() override;

  void PrepareFontUniqueNameLookup(
      NotifyFontUniqueNameLookupReady callback) override;

  sk_sp<SkTypeface> MatchUniqueName(const String& font_unique_name) override;

  void Init() override;

 private:
  void EnsureServiceConnected();

  void ReceiveReadOnlySharedMemoryRegion(
      base::ReadOnlySharedMemoryRegion shared_memory_region);

  sk_sp<SkTypeface> MatchUniqueNameFromFirmwareFonts(
      const String& font_unique_name);

  bool RequestedNameInQueryableFonts(const String& font_unique_name);
  sk_sp<SkTypeface> MatchUniqueNameFromDownloadableFonts(
      const String& font_unique_name);

  void FontsPrefetched(HashMap<String, base::File> font_files);

  mojo::Remote<mojom::blink::FontUniqueNameLookup>
      firmware_font_lookup_service_;
  mojo::Remote<mojom::blink::AndroidFontLookup> android_font_lookup_service_;
  WTF::Deque<NotifyFontUniqueNameLookupReady> pending_callbacks_;
  std::optional<bool> sync_available_;
  std::optional<Vector<String>> queryable_fonts_;
  HashMap<String, base::File> prefetched_font_map_;

  SEQUENCE_CHECKER(sequence_checker_);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_ANDROID_FONT_UNIQUE_NAME_LOOKUP_ANDROID_H_
