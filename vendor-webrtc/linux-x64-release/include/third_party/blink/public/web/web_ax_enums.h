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

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_AX_ENUMS_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_AX_ENUMS_H_

namespace blink {

// Expanded State.
// These values must match blink::AccessibilityExpanded values.
// Enforced in AssertMatchingEnums.cpp.
enum WebAXExpanded {
  kWebAXExpandedUndefined = 0,
  kWebAXExpandedCollapsed,
  kWebAXExpandedExpanded
};

// Grabbed State.
// These values must match blink::AccessibilityGrabbedState values.
enum WebAXGrabbedState {
  kWebAXGrabbedStateUndefined = 0,
  kWebAXGrabbedStateFalse,
  kWebAXGrabbedStateTrue
};

// Selected State.
// These values must match blink::AccessibilitySelectedState values.
enum WebAXSelectedState {
  kWebAXSelectedStateUndefined = 0,
  kWebAXSelectedStateFalse,
  kWebAXSelectedStateTrue
};

// These values must match blink::AccessibilityOrientation values.
// Enforced in AssertMatchingEnums.cpp.
enum WebAXOrientation {
  kWebAXOrientationUndefined = 0,
  kWebAXOrientationVertical,
  kWebAXOrientationHorizontal,
};

// State of a form control or editors
enum WebAXRestriction {
  kWebAXRestrictionNone = 0,  // Enabled control or other object not disabled
  kWebAXRestrictionReadOnly,
  kWebAXRestrictionDisabled,
};

// Availability of Autofill/Autocomplete suggestions.
enum class WebAXAutofillSuggestionAvailability {
  kNoSuggestions = 0,
  kAutofillAvailable,
  kAutocompleteAvailable,
};

//
// Sparse accessibility attributes
//
// The following enums represent accessibility attributes that apply
// to only a small fraction of WebAXObjects. Rather than the client
// asking each WebAXObject for the value of each accessibility
// attribute, it can call a single function to query for all
// sparse attributes at the same time. Any sparse attributes that
// are present are returned via a callback consisting of an attribute
// key enum and an attribute value.
//

// Sparse attributes of a WebAXObject whose value is either true or
// false. In order for it to be a sparse attribute the default value
// must be false.
enum class WebAXBoolAttribute {
  kAriaBusy,
};

enum class WebAXIntAttribute {
  kAriaColumnCount,
  kAriaRowCount,
};

enum class WebAXUIntAttribute {
  kAriaColumnIndex,
  kAriaColumnSpan,
  kAriaRowIndex,
  kAriaRowSpan,
};

// Sparse attributes of a WebAXObject whose value is a string.
// In order for it to be a sparse attribute the default value
// must be "".
enum class WebAXStringAttribute {
  kAriaBrailleLabel,
  kAriaBrailleRoleDescription,
  kAriaKeyShortcuts,
  kAriaRoleDescription,
};

// Sparse attributes of a WebAXObject whose value is a reference to
// another WebAXObject within the same frame. In order for it to be a
// sparse attribute the default value must be the null WebAXObject.
enum class WebAXObjectAttribute {
  kAriaActiveDescendant,
  kAriaErrorMessage,
};

// Sparse attributes of a WebAXObject whose value is a vector of
// references to other WebAXObjects within the same frame. In order
// for it to be a sparse attribute the default value must be the
// empty vector.
enum class WebAXObjectVectorAttribute {
  kAriaControls,
  kAriaDetails,
  kAriaFlowTo,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_AX_ENUMS_H_
