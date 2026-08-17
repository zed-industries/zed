/*
 * This file is part of the WebKit project.
 *
 * Copyright (C) 2009 Michelangelo De Simone <micdesim@gmail.com>
 * Copyright (C) 2010 Apple Inc. All rights reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_VALIDITY_STATE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_VALIDITY_STATE_H_

#include "third_party/blink/renderer/core/html/forms/listed_element.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class ValidityState final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit ValidityState(ListedElement* control) : control_(control) {}
  ValidityState(const ValidityState&) = delete;
  ValidityState& operator=(const ValidityState&) = delete;

  void Trace(Visitor* visitor) const override {
    visitor->Trace(control_);
    ScriptWrappable::Trace(visitor);
  }

  String ValidationMessage() const;

  void SetCustomErrorMessage(const String&);

  bool valueMissing() const;
  bool typeMismatch() const;
  bool patternMismatch() const;
  bool tooLong() const;
  bool tooShort() const;
  bool rangeUnderflow() const;
  bool rangeOverflow() const;
  bool stepMismatch() const;
  bool badInput() const;
  bool customError() const;
  bool valid() const;

 private:
  Member<ListedElement> control_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_VALIDITY_STATE_H_
