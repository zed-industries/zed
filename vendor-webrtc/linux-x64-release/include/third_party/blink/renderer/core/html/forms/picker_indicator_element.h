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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_PICKER_INDICATOR_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_PICKER_INDICATOR_ELEMENT_H_

#include "third_party/blink/renderer/core/html/forms/date_time_chooser.h"
#include "third_party/blink/renderer/core/html/forms/date_time_chooser_client.h"
#include "third_party/blink/renderer/core/html/html_div_element.h"

namespace blink {

class PickerIndicatorElement final : public HTMLDivElement,
                                     public DateTimeChooserClient {
 public:
  // PickerIndicatorOwner implementer must call removePickerIndicatorOwner when
  // it doesn't handle event, e.g. at destruction.
  class PickerIndicatorOwner : public GarbageCollectedMixin {
   public:
    virtual ~PickerIndicatorOwner() = default;
    virtual bool IsPickerIndicatorOwnerDisabledOrReadOnly() const = 0;
    // FIXME: Remove. Deprecated in favor of double version.
    virtual void PickerIndicatorChooseValue(const String&) = 0;
    virtual void PickerIndicatorChooseValue(double) = 0;
    virtual Element& PickerOwnerElement() const = 0;
    virtual bool SetupDateTimeChooserParameters(DateTimeChooserParameters&) = 0;
    virtual void DidEndChooser() = 0;
    virtual String AriaLabelForPickerIndicator() const = 0;
  };

  PickerIndicatorElement(Document&, PickerIndicatorOwner&);
  ~PickerIndicatorElement() override;
  void Trace(Visitor*) const override;

  void OpenPopup();
  void ClosePopup();
  bool HasOpenedPopup() const;
  bool IsPickerVisible() const;
  bool WillRespondToMouseClickEvents() override;
  void RemovePickerIndicatorOwner() { picker_indicator_owner_ = nullptr; }
  AXObject* PopupRootAXObject() const;

  // DateTimeChooserClient implementation.
  Element& OwnerElement() const override;
  void DidChooseValue(const String&) override;
  void DidChooseValue(double) override;
  void DidEndChooser() override;

 private:
  void DefaultEventHandler(Event&) override;
  void DetachLayoutTree(bool performing_reattach) override;
  bool IsPickerIndicatorElement() const override;
  InsertionNotificationRequest InsertedInto(ContainerNode&) override;
  void DidNotifySubtreeInsertionsToDocument() override;
  void SetAXProperties();

  Member<PickerIndicatorOwner> picker_indicator_owner_;
  Member<DateTimeChooser> chooser_;
};

template <>
struct DowncastTraits<PickerIndicatorElement> {
  static bool AllowFrom(const Element& element) {
    return element.IsPickerIndicatorElement();
  }
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_PICKER_INDICATOR_ELEMENT_H_
