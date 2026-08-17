// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_LOCAL_FONT_FACE_SOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_LOCAL_FONT_FACE_SOURCE_H_

#include "base/memory/weak_ptr.h"
#include "third_party/blink/renderer/core/css/css_font_face_source.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

class CSSFontFace;
class FontSelector;

// LocalFontFaceSource represents a src: local(<unique_name>) font face
// source. It operates in one of two ways: Synchronous or asynchronous,
// depending on what the platform implementation of FontUniqueNameLookup
// supports. If it operates in synchronous mode, IsLocalNonBlocking() return
// true and lookups are performed immediately. If IsLocalNonBlocking() returns
// false, a fallback font is returned from GetFontData() until
// FontUniqueNameLookup is ready (which is signalled by a callback). When
// FontUniqueNameLookup becomes ready, LocalFontFaceSource can lookup fonts
// synchronously and a relayout is triggered.
class LocalFontFaceSource final : public CSSFontFaceSource,
                                  public GarbageCollectedMixin {
 public:
  LocalFontFaceSource(CSSFontFace*, FontSelector*, const String& font_name);
  ~LocalFontFaceSource() override;

  // Returns whether this font can be immediately retrieved using a non-blocking
  // font lookup, or whether it may need to be retrieved asynchronously,
  // behaving similar to a RemoteFontFaceSource. This is needed on Windows 7 and
  // 8 where the font lookup map needs to be built first.
  bool IsLocalNonBlocking() const override;
  bool IsLocalFontAvailable(const FontDescription&) const override;
  bool IsLoaded() const override;
  bool IsLoading() const override;
  bool IsValid() const override;

  void BeginLoadIfNeeded() override;

  void Trace(Visitor* visitor) const override;

  void NotifyFontUniqueNameLookupReady();

 protected:
  const SimpleFontData* CreateLoadingFallbackFontData(const FontDescription&);

 private:
  const SimpleFontData* CreateFontData(
      const FontDescription&,
      const FontSelectionCapabilities&) override;

  class LocalFontHistograms {
    DISALLOW_NEW();

   public:
    LocalFontHistograms() : reported_(false) {}
    void Record(bool load_success);

   private:
    bool reported_;
  };

  Member<CSSFontFace> face_;
  Member<FontSelector> font_selector_;

  AtomicString font_name_;
  LocalFontHistograms histograms_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_LOCAL_FONT_FACE_SOURCE_H_
