// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_NFC_NFC_TYPE_CONVERTERS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_NFC_NFC_TYPE_CONVERTERS_H_

#include "mojo/public/cpp/bindings/type_converter.h"
#include "services/device/public/mojom/nfc.mojom-blink-forward.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class NDEFRecord;
class NDEFMessage;
class NDEFWriteOptions;

}  // namespace blink

// Mojo type converters
namespace mojo {

template <>
struct TypeConverter<device::mojom::blink::NDEFRecordPtr, blink::NDEFRecord*> {
  static device::mojom::blink::NDEFRecordPtr Convert(
      const blink::NDEFRecord* record);
};

template <>
struct TypeConverter<device::mojom::blink::NDEFMessagePtr,
                     blink::NDEFMessage*> {
  static device::mojom::blink::NDEFMessagePtr Convert(
      const blink::NDEFMessage* message);
};

template <>
struct TypeConverter<device::mojom::blink::NDEFWriteOptionsPtr,
                     const ::blink::NDEFWriteOptions*> {
  static device::mojom::blink::NDEFWriteOptionsPtr Convert(
      const ::blink::NDEFWriteOptions* writeOptions);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_NFC_NFC_TYPE_CONVERTERS_H_
