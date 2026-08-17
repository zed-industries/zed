/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_DATE_TIME_CHOOSER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_DATE_TIME_CHOOSER_H_

#include "third_party/blink/public/mojom/choosers/date_time_chooser.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/forms/input_type.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "ui/gfx/geometry/rect.h"

namespace blink {

class AXObject;
class Element;

struct DateTimeChooserParameters {
  DISALLOW_NEW();
  CORE_EXPORT DateTimeChooserParameters();
  // DateTimeSuggestionPtr is not copyable.
  DateTimeChooserParameters(const DateTimeChooserParameters&) = delete;
  DateTimeChooserParameters& operator=(const DateTimeChooserParameters&) =
      delete;
  CORE_EXPORT ~DateTimeChooserParameters();

  // InputType::Type is a subset of FormControlType. InputType::Type is
  // sufficient because DateTimeChooser only deals with HTMLInputElements. It's
  // preferable over FormControlType because with InputType::TypeToString() a
  // string conversion is already available.
  InputType::Type type;
  gfx::Rect anchor_rect_in_screen;
  // Locale name for which the chooser should be localized. This
  // might be an invalid name because it comes from HTML lang
  // attributes.
  AtomicString locale;
  double double_value = 0;
  Vector<mojom::blink::DateTimeSuggestionPtr> suggestions;
  double minimum = 0;
  double maximum = 0;
  double step = 1.0;
  double step_base = 0;
  bool required = false;
  bool is_anchor_element_rtl = false;
  // The fields below are used for type="time".
  // For some locales the am/pm is the first field, so is_ampm_first informs
  // the time popup when the am/pm column should be the first one.
  bool is_ampm_first = false;
  bool has_ampm = false;
  bool has_second = false;
  bool has_millisecond = false;
  int focused_field_index = 0;
};

// For pickers like color pickers and date pickers.
class CORE_EXPORT DateTimeChooser : public GarbageCollected<DateTimeChooser> {
 public:
  virtual ~DateTimeChooser();

  virtual void EndChooser() = 0;
  // Returns a root AXObject in the DateTimeChooser if it's available.
  virtual AXObject* RootAXObject(Element* popup_owner) = 0;
  // Returns true if picker UI is currently showing.
  virtual bool IsPickerVisible() const = 0;

  virtual void Trace(Visitor* visitor) const {}
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_DATE_TIME_CHOOSER_H_
