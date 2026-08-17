// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBUSB_USB_ISOCHRONOUS_IN_TRANSFER_RESULT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBUSB_USB_ISOCHRONOUS_IN_TRANSFER_RESULT_H_

#include "third_party/blink/renderer/core/typed_arrays/dom_array_buffer.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_data_view.h"
#include "third_party/blink/renderer/modules/webusb/usb_isochronous_in_transfer_packet.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class USBIsochronousInTransferResult final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static USBIsochronousInTransferResult* Create(
      DOMArrayBuffer* data,
      const HeapVector<Member<USBIsochronousInTransferPacket>>& packets) {
    auto data_view =
        NotShared(DOMDataView::Create(data, 0, data->ByteLength()));
    return MakeGarbageCollected<USBIsochronousInTransferResult>(data_view,
                                                                packets);
  }

  static USBIsochronousInTransferResult* Create(
      const HeapVector<Member<USBIsochronousInTransferPacket>>& packets,
      NotShared<DOMDataView> data) {
    return MakeGarbageCollected<USBIsochronousInTransferResult>(data, packets);
  }

  static USBIsochronousInTransferResult* Create(
      const HeapVector<Member<USBIsochronousInTransferPacket>>& packets) {
    return MakeGarbageCollected<USBIsochronousInTransferResult>(
        NotShared<DOMDataView>(), packets);
  }

  USBIsochronousInTransferResult(
      NotShared<DOMDataView> data,
      const HeapVector<Member<USBIsochronousInTransferPacket>>& packets)
      : data_(data), packets_(packets) {}

  ~USBIsochronousInTransferResult() override = default;

  NotShared<DOMDataView> data() const { return data_; }
  const HeapVector<Member<USBIsochronousInTransferPacket>>& packets() const {
    return packets_;
  }

  void Trace(Visitor* visitor) const override {
    visitor->Trace(data_);
    visitor->Trace(packets_);
    ScriptWrappable::Trace(visitor);
  }

 private:
  NotShared<DOMDataView> data_;
  const HeapVector<Member<USBIsochronousInTransferPacket>> packets_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBUSB_USB_ISOCHRONOUS_IN_TRANSFER_RESULT_H_
