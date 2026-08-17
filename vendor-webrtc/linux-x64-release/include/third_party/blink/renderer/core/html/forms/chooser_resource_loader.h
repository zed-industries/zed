// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_CHOOSER_RESOURCE_LOADER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_CHOOSER_RESOURCE_LOADER_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class ChooserResourceLoader {
  STATIC_ONLY(ChooserResourceLoader);

 public:
  // Returns the picker common stylesheet as a string.
  static Vector<char> GetPickerCommonStyleSheet();

  // Returns the picker common javascript as a string.
  static Vector<char> GetPickerCommonJS();

  // Returns the suggestion picker stylesheet as a string.
  static Vector<char> GetSuggestionPickerStyleSheet();

  // Returns the suggestion picker javascript as a string.
  static Vector<char> GetSuggestionPickerJS();

  // Returns the suggestion picker stylesheet as a string.
  static Vector<char> GetCalendarPickerStyleSheet();

  // Returns the suggestion picker javascript as a string.
  static Vector<char> GetCalendarPickerJS();

  // Returns the month picker javascript as a string.
  static Vector<char> GetMonthPickerJS();

  // Returns the time picker stylesheet as a string.
  static Vector<char> GetTimePickerStyleSheet();

  // Returns the time picker javascript as a string.
  static Vector<char> GetTimePickerJS();

  // Returns the datetimelocal picker javascript as a string.
  static Vector<char> GetDateTimeLocalPickerJS();

  // Returns the color suggestion picker stylesheet as a string.
  static Vector<char> GetColorSuggestionPickerStyleSheet();

  // Returns the color suggestion picker javascript as a string.
  static Vector<char> GetColorSuggestionPickerJS();

  // Returns the color picker stylesheet as a string.
  static Vector<char> GetColorPickerStyleSheet();

  // Returns the color picker javascript as a string.
  static Vector<char> GetColorPickerJS();

  // Returns the color picker common javascript as a string.
  static Vector<char> GetColorPickerCommonJS();

  // Returns the list picker stylesheet as a string.
  static Vector<char> GetListPickerStyleSheet();

  // Returns the list picker javascript as a string.
  static Vector<char> GetListPickerJS();
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_CHOOSER_RESOURCE_LOADER_H_
