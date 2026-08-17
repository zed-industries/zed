// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_IME_TEXT_FORMAT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_IME_TEXT_FORMAT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class TextFormatInit;
class TextFormat;

// The TextFormat describes how the texts in an active composition should be
// styled.
// spec:
// https://w3c.github.io/editing/docs/EditContext/index.html#textformatupdateevent
class CORE_EXPORT TextFormat final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static TextFormat* Create(const TextFormatInit* dict);
  static TextFormat* Create(wtf_size_t range_start,
                            wtf_size_t range_end,
                            const String& underline_style,
                            const String& underline_thickness);
  explicit TextFormat(const TextFormatInit* dict);
  TextFormat(wtf_size_t range_start,
             wtf_size_t range_end,
             const String& underline_style,
             const String& underline_thickness);

  wtf_size_t rangeStart() const;
  wtf_size_t rangeEnd() const;
  String underlineStyle() const;
  String underlineThickness() const;

 private:
  wtf_size_t range_start_ = 0;
  wtf_size_t range_end_ = 0;
  String underline_style_;
  String underline_thickness_;
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_IME_TEXT_FORMAT_H_
