// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_UNIQUE_NAME_LOOKUP_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_UNIQUE_NAME_LOOKUP_H_

#include "base/functional/callback.h"
#include "base/notreached.h"
#include "build/build_config.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/skia/include/core/SkRefCnt.h"
#include "third_party/skia/include/core/SkTypeface.h"

#if BUILDFLAG(IS_ANDROID) || BUILDFLAG(IS_WIN)
#include "third_party/blink/public/common/font_unique_name_lookup/font_table_matcher.h"
#endif

#include <memory>

namespace blink {

class FontTableMatcher;

class FontUniqueNameLookup {
  USING_FAST_MALLOC(FontUniqueNameLookup);

 public:
  // Factory function to construct a platform specific font unique name lookup
  // instance. Client must not use this directly as it is thread
  // specific. Retrieve it from FontGlobalContext instead.
  static std::unique_ptr<FontUniqueNameLookup> GetPlatformUniqueNameLookup();

  virtual sk_sp<SkTypeface> MatchUniqueName(const String& font_unique_name) = 0;

  FontUniqueNameLookup(const FontUniqueNameLookup&) = delete;
  FontUniqueNameLookup& operator=(const FontUniqueNameLookup&) = delete;
  virtual ~FontUniqueNameLookup() = default;

  // Below: Methods for asynchronously retrieving the FontUniqueNameLookup
  // table. Currently needed on Windows, on other platforms the implementation
  // is synchronous.

  // Determines whether fonts can be uniquely matched synchronously.
  virtual bool IsFontUniqueNameLookupReadyForSyncLookup() { return true; }

  // If fonts cannot be uniquely matched synchronously, send a Mojo IPC call to
  // prepare the lookup table, and wait for the callback. Once the callback has
  // been called, IsFontUniqueNameLookupReadyForSyncLookup() will become true.
  // PrepareFontUniqueNameLookup() must not be called if
  // IsFontUniqueNameLookupReadyForSyncLookup() is true already.
  using NotifyFontUniqueNameLookupReady = base::OnceCallback<void()>;
  virtual void PrepareFontUniqueNameLookup(
      NotifyFontUniqueNameLookupReady callback) {
    NOTREACHED();
  }

  // Performs any global initialization needed for this renderer. This is called
  // early in renderer startup, as opposed to PrepareFontUniqueNameLookup()
  // which is called when sync lookup is first needed.
  virtual void Init() {}

 protected:
  FontUniqueNameLookup();

  // Windows and Android share the concept of connecting to a Mojo service for
  // retrieving a ReadOnlySharedMemoryRegion with the lookup table in it.
#if BUILDFLAG(IS_WIN) || BUILDFLAG(IS_ANDROID)
  std::unique_ptr<FontTableMatcher> font_table_matcher_;
#endif
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_FONT_UNIQUE_NAME_LOOKUP_H_
