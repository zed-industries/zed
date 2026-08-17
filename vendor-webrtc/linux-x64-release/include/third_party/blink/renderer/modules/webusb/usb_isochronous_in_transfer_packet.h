// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBUSB_USB_ISOCHRONOUS_IN_TRANSFER_PACKET_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBUSB_USB_ISOCHRONOUS_IN_TRANSFER_PACKET_H_

#include "third_party/blink/renderer/bindings/modules/v8/v8_usb_transfer_status.h"
#include "third_party/blink/renderer/core/typed_arrays/array_buffer_view_helpers.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_data_view.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class USBIsochronousInTransferPacket final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static USBIsochronousInTransferPacket* Create(
      const V8USBTransferStatus& status) {
    return MakeGarbageCollected<USBIsochronousInTransferPacket>(
        status, NotShared<DOMDataView>());
  }

  static USBIsochronousInTransferPacket* Create(
      const V8USBTransferStatus& status,
      NotShared<DOMDataView> data) {
    return MakeGarbageCollected<USBIsochronousInTransferPacket>(status, data);
  }

  USBIsochronousInTransferPacket(const V8USBTransferStatus& status,
                                 NotShared<DOMDataView> data)
      : status_(status), data_(data) {}
  ~USBIsochronousInTransferPacket() override = default;

  V8USBTransferStatus status() const { return status_; }
  NotShared<DOMDataView> data() const { return data_; }

  void Trace(Visitor* visitor) const override {
    visitor->Trace(data_);
    ScriptWrappable::Trace(visitor);
  }

 private:
  const V8USBTransferStatus status_;
  const NotShared<DOMDataView> data_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBUSB_USB_ISOCHRONOUS_IN_TRANSFER_PACKET_H_
