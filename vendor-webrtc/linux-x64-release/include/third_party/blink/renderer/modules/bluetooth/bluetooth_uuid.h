// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_BLUETOOTH_BLUETOOTH_UUID_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_BLUETOOTH_BLUETOOTH_UUID_H_

#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_typedefs.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class ExceptionState;

// This class provides a way for script to look up UUIDs by name so they don't
// need to be replicated in each application.
class BluetoothUUID final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // IDL exposed interface:
  static WTF::String getService(const V8BluetoothServiceUUID* name,
                                ExceptionState& exception_state);
  static WTF::String getCharacteristic(
      const V8BluetoothCharacteristicUUID* name,
      ExceptionState& exception_state);
  static WTF::String getDescriptor(const V8BluetoothDescriptorUUID* name,
                                   ExceptionState& exception_state);
  static WTF::String canonicalUUID(unsigned alias);
};

// Helper function to retrieve the UUID (as a string) from the V8 value.
// The value may be a string or 16-bit unsigned integer. If the value cannot
// be interpreted as a valid UUID then an empty string will be returned.
BLINK_EXPORT WTF::String GetBluetoothUUIDFromV8Value(
    const V8UnionStringOrUnsignedLong* value);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_BLUETOOTH_BLUETOOTH_UUID_H_
