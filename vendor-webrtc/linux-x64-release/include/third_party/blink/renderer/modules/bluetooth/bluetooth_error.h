// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BLUETOOTH_BLUETOOTH_ERROR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BLUETOOTH_BLUETOOTH_ERROR_H_

#include "third_party/blink/public/mojom/bluetooth/web_bluetooth.mojom-blink-forward.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

// Used when generating DOMExceptions specific to each operation.
enum class BluetoothOperation {
  kServicesRetrieval,
  kCharacteristicsRetrieval,
  kDescriptorsRetrieval,
  kGATT,
};

// These error codes requires detailed error messages.
enum class BluetoothErrorCode {
  kInvalidService,
  kInvalidCharacteristic,
  kInvalidDescriptor,
  kServiceNotFound,
  kCharacteristicNotFound,
  kDescriptorNotFound
};

class DOMException;

// BluetoothError is used with CallbackPromiseAdapter to receive
// WebBluetoothResult responses. See CallbackPromiseAdapter class comments.
class BluetoothError {
  STATIC_ONLY(BluetoothError);

 public:
  static String CreateNotConnectedExceptionMessage(
      BluetoothOperation operation);
  static DOMException* CreateNotConnectedException(BluetoothOperation);
  static DOMException* CreateDOMException(BluetoothErrorCode,
                                          const String& detailed_message);

  static DOMException* CreateDOMException(
      mojom::blink::WebBluetoothResult error);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BLUETOOTH_BLUETOOTH_ERROR_H_
