// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_POSITION_TRY_DESCRIPTORS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_POSITION_TRY_DESCRIPTORS_H_

#include "third_party/blink/renderer/core/css/style_rule_css_style_declaration.h"
#include "third_party/blink/renderer/platform/runtime_enabled_features.h"

namespace blink {

// https://drafts.csswg.org/css-anchor-position-1/#interfaces
class CORE_EXPORT CSSPositionTryDescriptors
    : public StyleRuleCSSStyleDeclaration {
  DEFINE_WRAPPERTYPEINFO();

 public:
  CSSPositionTryDescriptors(MutableCSSPropertyValueSet&, CSSRule*);

  bool IsPropertyValid(CSSPropertyID) const override;

  String margin() { return Get(CSSPropertyID::kMargin); }
  String marginTop() { return Get(CSSPropertyID::kMarginTop); }
  String marginRight() { return Get(CSSPropertyID::kMarginRight); }
  String marginBottom() { return Get(CSSPropertyID::kMarginBottom); }
  String marginLeft() { return Get(CSSPropertyID::kMarginLeft); }
  String marginBlock() { return Get(CSSPropertyID::kMarginBlock); }
  String marginBlockStart() { return Get(CSSPropertyID::kMarginBlockStart); }
  String marginBlockEnd() { return Get(CSSPropertyID::kMarginBlockEnd); }
  String marginInline() { return Get(CSSPropertyID::kMarginInline); }
  String marginInlineStart() { return Get(CSSPropertyID::kMarginInlineStart); }
  String marginInlineEnd() { return Get(CSSPropertyID::kMarginInlineEnd); }
  String inset() { return Get(CSSPropertyID::kInset); }
  String insetBlock() { return Get(CSSPropertyID::kInsetBlock); }
  String insetBlockStart() { return Get(CSSPropertyID::kInsetBlockStart); }
  String insetBlockEnd() { return Get(CSSPropertyID::kInsetBlockEnd); }
  String insetInline() { return Get(CSSPropertyID::kInsetInline); }
  String insetInlineStart() { return Get(CSSPropertyID::kInsetInlineStart); }
  String insetInlineEnd() { return Get(CSSPropertyID::kInsetInlineEnd); }
  String top() { return Get(CSSPropertyID::kTop); }
  String left() { return Get(CSSPropertyID::kLeft); }
  String right() { return Get(CSSPropertyID::kRight); }
  String bottom() { return Get(CSSPropertyID::kBottom); }
  String width() { return Get(CSSPropertyID::kWidth); }
  String minWidth() { return Get(CSSPropertyID::kMinWidth); }
  String maxWidth() { return Get(CSSPropertyID::kMaxWidth); }
  String height() { return Get(CSSPropertyID::kHeight); }
  String minHeight() { return Get(CSSPropertyID::kMinHeight); }
  String maxHeight() { return Get(CSSPropertyID::kMaxHeight); }
  String blockSize() { return Get(CSSPropertyID::kBlockSize); }
  String minBlockSize() { return Get(CSSPropertyID::kMinBlockSize); }
  String maxBlockSize() { return Get(CSSPropertyID::kMaxBlockSize); }
  String inlineSize() { return Get(CSSPropertyID::kInlineSize); }
  String minInlineSize() { return Get(CSSPropertyID::kMinInlineSize); }
  String maxInlineSize() { return Get(CSSPropertyID::kMaxInlineSize); }
  String placeSelf() { return Get(CSSPropertyID::kPlaceSelf); }
  String alignSelf() { return Get(CSSPropertyID::kAlignSelf); }
  String justifySelf() { return Get(CSSPropertyID::kJustifySelf); }
  String positionAnchor() { return Get(CSSPropertyID::kPositionAnchor); }
  String positionArea() { return Get(CSSPropertyID::kPositionArea); }

  void setMargin(const ExecutionContext* execution_context,
                 const String& value,
                 ExceptionState& exception_state) {
    Set(execution_context, CSSPropertyID::kMargin, value, exception_state);
  }
  void setMarginTop(const ExecutionContext* execution_context,
                    const String& value,
                    ExceptionState& exception_state) {
    Set(execution_context, CSSPropertyID::kMarginTop, value, exception_state);
  }
  void setMarginRight(const ExecutionContext* execution_context,
                      const String& value,
                      ExceptionState& exception_state) {
    Set(execution_context, CSSPropertyID::kMarginRight, value, exception_state);
  }
  void setMarginBottom(const ExecutionContext* execution_context,
                       const String& value,
                       ExceptionState& exception_state) {
    Set(execution_context, CSSPropertyID::kMarginBottom, value,
        exception_state);
  }
  void setMarginLeft(const ExecutionContext* execution_context,
                     const String& value,
                     ExceptionState& exception_state) {
    Set(execution_context, CSSPropertyID::kMarginLeft, value, exception_state);
  }
  void setMarginBlock(const ExecutionContext* execution_context,
                      const String& value,
                      ExceptionState& exception_state) {
    Set(execution_context, CSSPropertyID::kMarginBlock, value, exception_state);
  }
  void setMarginBlockStart(const ExecutionContext* execution_context,
                           const String& value,
                           ExceptionState& exception_state) {
    Set(execution_context, CSSPropertyID::kMarginBlockStart, value,
        exception_state);
  }
  void setMarginBlockEnd(const ExecutionContext* execution_context,
                         const String& value,
                         ExceptionState& exception_state) {
    Set(execution_context, CSSPropertyID::kMarginBlockEnd, value,
        exception_state);
  }
  void setMarginInline(const ExecutionContext* execution_context,
                       const String& value,
                       ExceptionState& exception_state) {
    Set(execution_context, CSSPropertyID::kMarginInline, value,
        exception_state);
  }
  void setMarginInlineStart(const ExecutionContext* execution_context,
                            const String& value,
                            ExceptionState& exception_state) {
    Set(execution_context, CSSPropertyID::kMarginInlineStart, value,
        exception_state);
  }
  void setMarginInlineEnd(const ExecutionContext* execution_context,
                          const String& value,
                          ExceptionState& exception_state) {
    Set(execution_context, CSSPropertyID::kMarginInlineEnd, value,
        exception_state);
  }
  void setInset(const ExecutionContext* execution_context,
                const String& value,
                ExceptionState& exception_state) {
    Set(execution_context, CSSPropertyID::kInset, value, exception_state);
  }
  void setInsetBlock(const ExecutionContext* execution_context,
                     const String& value,
                     ExceptionState& exception_state) {
    Set(execution_context, CSSPropertyID::kInsetBlock, value, exception_state);
  }
  void setInsetBlockStart(const ExecutionContext* execution_context,
                          const String& value,
                          ExceptionState& exception_state) {
    Set(execution_context, CSSPropertyID::kInsetBlockStart, value,
        exception_state);
  }
  void setInsetBlockEnd(const ExecutionContext* execution_context,
                        const String& value,
                        ExceptionState& exception_state) {
    Set(execution_context, CSSPropertyID::kInsetBlockEnd, value,
        exception_state);
  }
  void setInsetInline(const ExecutionContext* execution_context,
                      const String& value,
                      ExceptionState& exception_state) {
    Set(execution_context, CSSPropertyID::kInsetInline, value, exception_state);
  }
  void setInsetInlineStart(const ExecutionContext* execution_context,
                           const String& value,
                           ExceptionState& exception_state) {
    Set(execution_context, CSSPropertyID::kInsetInlineStart, value,
        exception_state);
  }
  void setInsetInlineEnd(const ExecutionContext* execution_context,
                         const String& value,
                         ExceptionState& exception_state) {
    Set(execution_context, CSSPropertyID::kInsetInlineEnd, value,
        exception_state);
  }
  void setTop(const ExecutionContext* execution_context,
              const String& value,
              ExceptionState& exception_state) {
    Set(execution_context, CSSPropertyID::kTop, value, exception_state);
  }
  void setLeft(const ExecutionContext* execution_context,
               const String& value,
               ExceptionState& exception_state) {
    Set(execution_context, CSSPropertyID::kLeft, value, exception_state);
  }
  void setRight(const ExecutionContext* execution_context,
                const String& value,
                ExceptionState& exception_state) {
    Set(execution_context, CSSPropertyID::kRight, value, exception_state);
  }
  void setBottom(const ExecutionContext* execution_context,
                 const String& value,
                 ExceptionState& exception_state) {
    Set(execution_context, CSSPropertyID::kBottom, value, exception_state);
  }
  void setWidth(const ExecutionContext* execution_context,
                const String& value,
                ExceptionState& exception_state) {
    Set(execution_context, CSSPropertyID::kWidth, value, exception_state);
  }
  void setMinWidth(const ExecutionContext* execution_context,
                   const String& value,
                   ExceptionState& exception_state) {
    Set(execution_context, CSSPropertyID::kMinWidth, value, exception_state);
  }
  void setMaxWidth(const ExecutionContext* execution_context,
                   const String& value,
                   ExceptionState& exception_state) {
    Set(execution_context, CSSPropertyID::kMaxWidth, value, exception_state);
  }
  void setHeight(const ExecutionContext* execution_context,
                 const String& value,
                 ExceptionState& exception_state) {
    Set(execution_context, CSSPropertyID::kHeight, value, exception_state);
  }
  void setMinHeight(const ExecutionContext* execution_context,
                    const String& value,
                    ExceptionState& exception_state) {
    Set(execution_context, CSSPropertyID::kMinHeight, value, exception_state);
  }
  void setMaxHeight(const ExecutionContext* execution_context,
                    const String& value,
                    ExceptionState& exception_state) {
    Set(execution_context, CSSPropertyID::kMaxHeight, value, exception_state);
  }
  void setBlockSize(const ExecutionContext* execution_context,
                    const String& value,
                    ExceptionState& exception_state) {
    Set(execution_context, CSSPropertyID::kBlockSize, value, exception_state);
  }
  void setMinBlockSize(const ExecutionContext* execution_context,
                       const String& value,
                       ExceptionState& exception_state) {
    Set(execution_context, CSSPropertyID::kMinBlockSize, value,
        exception_state);
  }
  void setMaxBlockSize(const ExecutionContext* execution_context,
                       const String& value,
                       ExceptionState& exception_state) {
    Set(execution_context, CSSPropertyID::kMaxBlockSize, value,
        exception_state);
  }
  void setInlineSize(const ExecutionContext* execution_context,
                     const String& value,
                     ExceptionState& exception_state) {
    Set(execution_context, CSSPropertyID::kInlineSize, value, exception_state);
  }
  void setMinInlineSize(const ExecutionContext* execution_context,
                        const String& value,
                        ExceptionState& exception_state) {
    Set(execution_context, CSSPropertyID::kMinInlineSize, value,
        exception_state);
  }
  void setMaxInlineSize(const ExecutionContext* execution_context,
                        const String& value,
                        ExceptionState& exception_state) {
    Set(execution_context, CSSPropertyID::kMaxInlineSize, value,
        exception_state);
  }
  void setPlaceSelf(const ExecutionContext* execution_context,
                    const String& value,
                    ExceptionState& exception_state) {
    Set(execution_context, CSSPropertyID::kPlaceSelf, value, exception_state);
  }
  void setAlignSelf(const ExecutionContext* execution_context,
                    const String& value,
                    ExceptionState& exception_state) {
    Set(execution_context, CSSPropertyID::kAlignSelf, value, exception_state);
  }
  void setJustifySelf(const ExecutionContext* execution_context,
                      const String& value,
                      ExceptionState& exception_state) {
    Set(execution_context, CSSPropertyID::kJustifySelf, value, exception_state);
  }
  void setPositionAnchor(const ExecutionContext* execution_context,
                         const String& value,
                         ExceptionState& exception_state) {
    Set(execution_context, CSSPropertyID::kPositionAnchor, value,
        exception_state);
  }
  void setPositionArea(const ExecutionContext* execution_context,
                       const String& value,
                       ExceptionState& exception_state) {
    Set(execution_context, CSSPropertyID::kPositionArea, value,
        exception_state);
  }
  void Trace(Visitor*) const override;

 private:
  String Get(CSSPropertyID);
  void Set(const ExecutionContext* execution_context,
           CSSPropertyID,
           const String& value,
           ExceptionState&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_POSITION_TRY_DESCRIPTORS_H_
