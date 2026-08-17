// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBUSB_USB_INTERFACE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBUSB_USB_INTERFACE_H_

#include "services/device/public/mojom/usb_device.mojom-blink.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class ExceptionState;
class USBAlternateInterface;
class USBConfiguration;
class USBDevice;

class USBInterface : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static USBInterface* Create(const USBConfiguration*,
                              wtf_size_t interface_index);
  static USBInterface* Create(const USBConfiguration*,
                              uint8_t interface_number,
                              ExceptionState&);

  USBInterface(const USBDevice*,
               wtf_size_t configuration_index,
               wtf_size_t interface_index);

  const device::mojom::blink::UsbInterfaceInfo& Info() const;

  uint8_t interfaceNumber() const { return Info().interface_number; }
  USBAlternateInterface* alternate() const;
  HeapVector<Member<USBAlternateInterface>> alternates() const;
  bool claimed() const;

  void Trace(Visitor*) const override;

 private:
  Member<const USBDevice> device_;
  HeapVector<Member<USBAlternateInterface>> alternates_;
  const wtf_size_t configuration_index_;
  const wtf_size_t interface_index_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBUSB_USB_INTERFACE_H_
