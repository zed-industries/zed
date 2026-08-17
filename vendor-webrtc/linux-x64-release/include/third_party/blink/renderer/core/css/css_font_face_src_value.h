/*
 * Copyright (C) 2007, 2008 Apple Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_FONT_FACE_SRC_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_FONT_FACE_SRC_VALUE_H_

#include "base/memory/scoped_refptr.h"
#include "base/task/single_thread_task_runner.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/core/loader/resource/font_resource.h"
#include "third_party/blink/renderer/platform/bindings/dom_wrapper_world.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

namespace cssvalue {
class CSSURIValue;
}  // namespace cssvalue

class ExecutionContext;

class CORE_EXPORT CSSFontFaceSrcValue : public CSSValue {
 public:
  static CSSFontFaceSrcValue* Create(cssvalue::CSSURIValue* src_value,
                                     const DOMWrapperWorld* world) {
    return MakeGarbageCollected<CSSFontFaceSrcValue>(src_value, world);
  }
  static CSSFontFaceSrcValue* CreateLocal(const String& local_resource) {
    return MakeGarbageCollected<CSSFontFaceSrcValue>(local_resource);
  }

  explicit CSSFontFaceSrcValue(const String& local_resource)
      : CSSValue(kFontFaceSrcClass), local_resource_(local_resource) {}
  CSSFontFaceSrcValue(cssvalue::CSSURIValue* src_value,
                      const DOMWrapperWorld* world)
      : CSSValue(kFontFaceSrcClass), src_value_(src_value), world_(world) {}

  // Returns the local() resource name. Only usable if IsLocal() returns true.
  const String& LocalResource() const { return local_resource_; }
  bool IsLocal() const { return !src_value_; }

  /* Format is serialized as string, so we can set this to string internally. It
   * does not affect functionality downstream - i.e. the font face is handled
   * the same way whatsoever, if the format is supported. */
  void SetFormat(const String& format) { format_ = format; }

  /* Only supported technologies need to be listed here, as we can reject other
   * font face source component values, hence remove SVG and incremental for
   * now, compare https://drafts.csswg.org/css-fonts-4/#font-face-src-parsing */
  enum class FontTechnology {
    kTechnologyFeaturesAAT,
    kTechnologyFeaturesOT,
    kTechnologyCOLRv0,
    kTechnologyCOLRv1,
    kTechnologySBIX,
    kTechnologyCDBT,
    kTechnologyVariations,
    kTechnologyPalettes,
    kTechnologyUnknown
  };
  void AppendTechnology(FontTechnology technology);

  bool IsSupportedFormat() const;

  String CustomCSSText() const;

  bool HasFailedOrCanceledSubresources() const;

  FontResource& Fetch(ExecutionContext*, FontResourceClient*) const;

  bool Equals(const CSSFontFaceSrcValue&) const;

  void TraceAfterDispatch(Visitor* visitor) const;

 private:
  void RestoreCachedResourceIfNeeded(ExecutionContext*) const;

  Vector<FontTechnology> technologies_;
  Member<cssvalue::CSSURIValue> src_value_;  // Non-null if remote (src()).
  String local_resource_;                    // Non-null if local (local()).
  String format_;
  const Member<const DOMWrapperWorld> world_;
  mutable Member<FontResource> fetched_;
};

template <>
struct DowncastTraits<CSSFontFaceSrcValue> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsFontFaceSrcValue();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_FONT_FACE_SRC_VALUE_H_
