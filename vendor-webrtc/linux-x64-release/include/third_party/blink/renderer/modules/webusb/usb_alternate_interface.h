// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBUSB_USB_ALTERNATE_INTERFACE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBUSB_USB_ALTERNATE_INTERFACE_H_

#include "services/device/public/mojom/usb_device.mojom-blink.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class ExceptionState;
class USBEndpoint;
class USBInterface;

class USBAlternateInterface : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static USBAlternateInterface* Create(const USBInterface*,
                                       wtf_size_t alternate_index);
  static USBAlternateInterface* Create(const USBInterface*,
                                       uint8_t alternate_setting,
                                       ExceptionState&);

  USBAlternateInterface(const USBInterface*, wtf_size_t alternate_index);

  const device::mojom::blink::UsbAlternateInterfaceInfo& Info() const;

  uint8_t alternateSetting() const { return Info().alternate_setting; }
  uint8_t interfaceClass() const { return Info().class_code; }
  uint8_t interfaceSubclass() const { return Info().subclass_code; }
  uint8_t interfaceProtocol() const { return Info().protocol_code; }
  String interfaceName() const { return Info().interface_name; }
  HeapVector<Member<USBEndpoint>> endpoints() const;

  void Trace(Visitor*) const override;

 private:
  Member<const USBInterface> interface_;
  HeapVector<Member<USBEndpoint>> endpoints_;
  const wtf_size_t alternate_index_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBUSB_USB_ALTERNATE_INTERFACE_H_
