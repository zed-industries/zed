/*
 * Copyright (C) 2006, 2007, 2008 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_IME_IME_TEXT_SPAN_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_IME_IME_TEXT_SPAN_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/graphics/color.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "ui/base/ime/ime_text_span.h"
#include "ui/base/ime/mojom/ime_types.mojom-blink-forward.h"

namespace blink {

class CORE_EXPORT ImeTextSpan {
  DISALLOW_NEW();

 public:
  enum class Type {
    kComposition,
    kSuggestion,
    kMisspellingSuggestion,
    kAutocorrect,
    kGrammarSuggestion,
  };

  ImeTextSpan(Type,
              wtf_size_t start_offset,
              wtf_size_t end_offset,
              const Color& underline_color,
              ui::mojom::ImeTextSpanThickness,
              ui::mojom::ImeTextSpanUnderlineStyle,
              const Color& text_color,
              const Color& background_color,
              const Color& suggestion_highlight_color = Color::kTransparent,
              bool remove_on_finish_composing = false,
              bool interim_char_selection_ = false,
              const Vector<String>& suggestions = Vector<String>());

  explicit ImeTextSpan(const ui::ImeTextSpan&);

  Type GetType() const { return type_; }
  wtf_size_t StartOffset() const { return start_offset_; }
  wtf_size_t EndOffset() const { return end_offset_; }
  const Color& UnderlineColor() const { return underline_color_; }
  ui::mojom::ImeTextSpanThickness Thickness() const { return thickness_; }
  ui::mojom::ImeTextSpanUnderlineStyle UnderlineStyle() const {
    return underline_style_;
  }
  const Color& TextColor() const { return text_color_; }
  const Color& BackgroundColor() const { return background_color_; }
  const Color& SuggestionHighlightColor() const {
    return suggestion_highlight_color_;
  }
  bool NeedsRemovalOnFinishComposing() const {
    return remove_on_finish_composing_;
  }
  bool InterimCharSelection() const { return interim_char_selection_; }
  const Vector<String>& Suggestions() const { return suggestions_; }

  ui::ImeTextSpan ToUiImeTextSpan();

 private:
  Type type_;
  wtf_size_t start_offset_;
  wtf_size_t end_offset_;
  Color underline_color_;
  ui::mojom::ImeTextSpanThickness thickness_;
  ui::mojom::ImeTextSpanUnderlineStyle underline_style_;
  Color text_color_;
  Color background_color_;
  Color suggestion_highlight_color_;
  bool remove_on_finish_composing_;
  bool interim_char_selection_;
  Vector<String> suggestions_;
};

ImeTextSpan::Type ConvertUiTypeToType(ui::ImeTextSpan::Type type);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_IME_IME_TEXT_SPAN_H_
