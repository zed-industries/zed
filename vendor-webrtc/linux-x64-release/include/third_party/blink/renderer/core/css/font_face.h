/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_FONT_FACE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_FONT_FACE_H_

#include "third_party/blink/renderer/bindings/core/v8/active_script_wrappable.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_property.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/core/css/font_display.h"
#include "third_party/blink/renderer/core/css/parser/at_rule_descriptors.h"
#include "third_party/blink/renderer/core/dom/dom_exception.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/fonts/font_selection_types.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class CSSFontFace;
class CSSFontFamilyValue;
class CSSPropertyValueSet;
class CSSValue;
class Document;
class CSSLengthResolver;
class ExceptionState;
class MediaValues;
class FontFaceDescriptors;
class StyleRuleFontFace;
class V8FontFaceLoadStatus;
class V8UnionArrayBufferOrArrayBufferViewOrString;
struct FontMetricsOverride;

class CORE_EXPORT FontFace : public ScriptWrappable,
                             public ActiveScriptWrappable<FontFace>,
                             public ExecutionContextClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  enum LoadStatusType : uint8_t { kUnloaded, kLoading, kLoaded, kError };

  static FontFace* Create(
      ExecutionContext* execution_context,
      const AtomicString& family,
      const V8UnionArrayBufferOrArrayBufferViewOrString* source,
      const FontFaceDescriptors* descriptors);
  static FontFace* Create(Document*,
                          const StyleRuleFontFace*,
                          bool is_user_style);

  FontFace(ExecutionContext*, const StyleRuleFontFace*, bool is_user_style);
  FontFace(ExecutionContext*,
           const AtomicString& family,
           const FontFaceDescriptors*);
  FontFace(const FontFace&) = delete;
  FontFace& operator=(const FontFace&) = delete;
  ~FontFace() override;

  const AtomicString& family() const { return family_; }
  String style() const;
  String weight() const;
  String stretch() const;
  String unicodeRange() const;
  String variant() const;
  String featureSettings() const;
  String display() const;
  String ascentOverride() const;
  String descentOverride() const;
  String lineGapOverride() const;
  String sizeAdjust() const;

  // FIXME: Changing these attributes should affect font matching.
  void setFamily(ExecutionContext*, const AtomicString& s, ExceptionState&) {
    family_ = s;
  }
  void setStyle(ExecutionContext*, const String&, ExceptionState&);
  void setWeight(ExecutionContext*, const String&, ExceptionState&);
  void setStretch(ExecutionContext*, const String&, ExceptionState&);
  void setUnicodeRange(ExecutionContext*, const String&, ExceptionState&);
  void setVariant(ExecutionContext*, const String&, ExceptionState&);
  void setFeatureSettings(ExecutionContext*, const String&, ExceptionState&);
  void setDisplay(ExecutionContext*, const String&, ExceptionState&);
  void setAscentOverride(ExecutionContext*, const String&, ExceptionState&);
  void setDescentOverride(ExecutionContext*, const String&, ExceptionState&);
  void setLineGapOverride(ExecutionContext*, const String&, ExceptionState&);
  void setSizeAdjust(ExecutionContext*, const String&, ExceptionState&);

  V8FontFaceLoadStatus status() const;
  ScriptPromise<FontFace> loaded(ScriptState* script_state) {
    return FontStatusPromise(script_state);
  }

  ScriptPromise<FontFace> load(ScriptState*);

  LoadStatusType LoadStatus() const { return status_; }
  void SetLoadStatus(LoadStatusType);
  void SetError(DOMException* = nullptr);
  DOMException* GetError() const { return error_.Get(); }
  FontSelectionCapabilities GetFontSelectionCapabilities() const;
  CSSFontFace* CssFontFace() { return css_font_face_.Get(); }
  size_t ApproximateBlankCharacterCount() const;
  FontDisplay GetFontDisplay() const;

  void Trace(Visitor*) const override;

  bool HadBlankText() const;

  class CORE_EXPORT LoadFontCallback : public GarbageCollectedMixin {
   public:
    virtual ~LoadFontCallback() = default;
    virtual void NotifyLoaded(FontFace*) = 0;
    virtual void NotifyError(FontFace*) = 0;
    void Trace(Visitor* visitor) const override {}
  };
  void LoadWithCallback(LoadFontCallback*);
  void AddCallback(LoadFontCallback*);

  void DidBeginImperativeLoad();

  // ScriptWrappable:
  bool HasPendingActivity() const final;

  bool HasFontMetricsOverride() const {
    return ascent_override_ || descent_override_ || line_gap_override_ ||
           advance_override_;
  }
  FontMetricsOverride GetFontMetricsOverride() const;

  bool HasSizeAdjust() const { return size_adjust_ != nullptr; }
  float GetSizeAdjust() const;

  Document* GetDocument() const;

  const StyleRuleFontFace* GetStyleRule() const { return style_rule_.Get(); }
  bool IsUserStyle() const { return is_user_style_; }

  const CSSLengthResolver& EnsureLengthResolver() const;

 private:
  static FontFace* Create(ExecutionContext*,
                          const AtomicString& family,
                          base::span<const uint8_t> data,
                          const FontFaceDescriptors*);
  static FontFace* Create(ExecutionContext*,
                          const AtomicString& family,
                          const String& source,
                          const FontFaceDescriptors*);

  void InitCSSFontFace(ExecutionContext*, const CSSValue& src);
  void InitCSSFontFace(ExecutionContext*, base::span<const uint8_t> data);
  void SetPropertyFromString(const ExecutionContext*,
                             const String&,
                             AtRuleDescriptorID,
                             ExceptionState* = nullptr);
  bool SetPropertyFromStyle(const CSSPropertyValueSet&, AtRuleDescriptorID);
  bool SetPropertyValue(const CSSValue*, AtRuleDescriptorID);
  void SetFamilyValue(const CSSFontFamilyValue&);
  ScriptPromise<FontFace> FontStatusPromise(ScriptState*);
  void RunCallbacks();

  using LoadedProperty = ScriptPromiseProperty<FontFace, DOMException>;

  HeapVector<Member<LoadFontCallback>> callbacks_;
  AtomicString family_;
  String ots_parse_message_;
  Member<const CSSValue> style_;
  Member<const CSSValue> weight_;
  Member<const CSSValue> stretch_;
  Member<const CSSValue> unicode_range_;
  Member<const CSSValue> variant_;
  Member<const CSSValue> feature_settings_;
  Member<const CSSValue> display_;
  Member<const CSSValue> ascent_override_;
  Member<const CSSValue> descent_override_;
  Member<const CSSValue> line_gap_override_;
  Member<const CSSValue> advance_override_;
  Member<const CSSValue> size_adjust_;
  Member<DOMException> error_;

  Member<LoadedProperty> loaded_property_;
  Member<CSSFontFace> css_font_face_;
  Member<const StyleRuleFontFace> style_rule_;

  LoadStatusType status_;
  // Note that we will also need to distinguish font faces in different tree
  // scopes when we allow @font-face in shadow DOM. See crbug.com/336876.
  bool is_user_style_ = false;

  // Global media values to resolve calc().
  mutable Member<const MediaValues> media_values_;
};

using FontFaceArray = HeapVector<Member<FontFace>>;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_FONT_FACE_H_
