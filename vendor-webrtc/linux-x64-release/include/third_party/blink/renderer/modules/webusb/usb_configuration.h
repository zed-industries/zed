// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBUSB_USB_CONFIGURATION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBUSB_USB_CONFIGURATION_H_

#include "services/device/public/mojom/usb_device.mojom-blink.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class ExceptionState;
class USBDevice;
class USBInterface;

class USBConfiguration : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static USBConfiguration* Create(const USBDevice*,
                                  wtf_size_t configuration_index);
  static USBConfiguration* Create(const USBDevice*,
                                  uint8_t configuration_value,
                                  ExceptionState&);

  USBConfiguration(const USBDevice*, wtf_size_t configuration_index);

  const USBDevice* Device() const;
  wtf_size_t Index() const;
  const device::mojom::blink::UsbConfigurationInfo& Info() const;

  uint8_t configurationValue() const { return Info().configuration_value; }
  String configurationName() const { return Info().configuration_name; }
  HeapVector<Member<USBInterface>> interfaces() const;

  void Trace(Visitor*) const override;

 private:
  Member<const USBDevice> device_;
  HeapVector<Member<USBInterface>> interfaces_;
  const wtf_size_t configuration_index_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBUSB_USB_CONFIGURATION_H_
