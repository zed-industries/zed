// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBUSB_USB_ISOCHRONOUS_OUT_TRANSFER_RESULT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBUSB_USB_ISOCHRONOUS_OUT_TRANSFER_RESULT_H_

#include "third_party/blink/renderer/core/typed_arrays/dom_data_view.h"
#include "third_party/blink/renderer/modules/webusb/usb_isochronous_out_transfer_packet.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class USBIsochronousOutTransferResult final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static USBIsochronousOutTransferResult* Create(
      const HeapVector<Member<USBIsochronousOutTransferPacket>>& packets) {
    return MakeGarbageCollected<USBIsochronousOutTransferResult>(packets);
  }

  USBIsochronousOutTransferResult(
      const HeapVector<Member<USBIsochronousOutTransferPacket>>& packets)
      : packets_(packets) {}

  ~USBIsochronousOutTransferResult() override = default;

  const HeapVector<Member<USBIsochronousOutTransferPacket>>& packets() const {
    return packets_;
  }

  void Trace(Visitor* visitor) const override {
    visitor->Trace(packets_);
    ScriptWrappable::Trace(visitor);
  }

 private:
  const HeapVector<Member<USBIsochronousOutTransferPacket>> packets_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBUSB_USB_ISOCHRONOUS_OUT_TRANSFER_RESULT_H_
