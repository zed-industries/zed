// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBUSB_USB_OUT_TRANSFER_RESULT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBUSB_USB_OUT_TRANSFER_RESULT_H_

#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class USBOutTransferResult final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static USBOutTransferResult* Create(const V8USBTransferStatus& status) {
    return MakeGarbageCollected<USBOutTransferResult>(status, 0);
  }

  static USBOutTransferResult* Create(const V8USBTransferStatus& status,
                                      uint32_t bytes_written) {
    return MakeGarbageCollected<USBOutTransferResult>(status, bytes_written);
  }

  USBOutTransferResult(const V8USBTransferStatus& status,
                       uint32_t bytes_written)
      : status_(status), bytes_written_(bytes_written) {}

  ~USBOutTransferResult() override = default;

  V8USBTransferStatus status() const { return status_; }
  uint32_t bytesWritten() const { return bytes_written_; }

 private:
  const V8USBTransferStatus status_;
  const uint32_t bytes_written_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBUSB_USB_OUT_TRANSFER_RESULT_H_
