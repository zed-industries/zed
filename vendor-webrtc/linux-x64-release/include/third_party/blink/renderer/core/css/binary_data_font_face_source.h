// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_BINARY_DATA_FONT_FACE_SOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_BINARY_DATA_FONT_FACE_SOURCE_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/css/css_font_face_source.h"

namespace blink {

class CSSFontFace;
class FontCustomPlatformData;

class BinaryDataFontFaceSource final : public CSSFontFaceSource {
 public:
  BinaryDataFontFaceSource(CSSFontFace*, SharedBuffer*, String&);
  void Trace(Visitor*) const override;
  bool IsValid() const override;

 private:
  SimpleFontData* CreateFontData(const FontDescription&,
                                 const FontSelectionCapabilities&) override;

  Member<const FontCustomPlatformData> custom_platform_data_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_BINARY_DATA_FONT_FACE_SOURCE_H_
