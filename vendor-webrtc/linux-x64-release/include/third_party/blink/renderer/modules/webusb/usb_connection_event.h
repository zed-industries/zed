// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBUSB_USB_CONNECTION_EVENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBUSB_USB_CONNECTION_EVENT_H_

#include "third_party/blink/renderer/modules/event_modules.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class USBConnectionEventInit;
class USBDevice;

class USBConnectionEvent final : public Event {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static USBConnectionEvent* Create(const AtomicString& type,
                                    const USBConnectionEventInit*);
  static USBConnectionEvent* Create(const AtomicString& type, USBDevice*);

  USBConnectionEvent(const AtomicString& type, const USBConnectionEventInit*);
  USBConnectionEvent(const AtomicString& type, USBDevice*);

  USBDevice* device() const { return device_.Get(); }

  void Trace(Visitor*) const override;

 private:
  Member<USBDevice> device_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBUSB_USB_CONNECTION_EVENT_H_
