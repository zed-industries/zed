// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_NFC_NDEF_MESSAGE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_NFC_NDEF_MESSAGE_H_

#include <stdint.h>

#include "services/device/public/mojom/nfc.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_typedefs.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class ExceptionState;
class NDEFMessageInit;
class NDEFRecord;
class ScriptState;

class MODULES_EXPORT NDEFMessage final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // |is_embedded| indicates if this message serves as payload for a parent
  // record.
  static NDEFMessage* Create(const ScriptState*,
                             const NDEFMessageInit*,
                             ExceptionState&,
                             uint8_t records_depth = 0U,
                             bool is_embedded = false);
  static NDEFMessage* Create(const ScriptState*,
                             const V8NDEFMessageSource* source,
                             ExceptionState& exception_state);
  static NDEFMessage* CreateAsPayloadOfSmartPoster(const ScriptState*,
                                                   const NDEFMessageInit*,
                                                   ExceptionState&,
                                                   uint8_t records_depth);

  NDEFMessage();
  explicit NDEFMessage(const device::mojom::blink::NDEFMessage&);

  const HeapVector<Member<NDEFRecord>>& records() const;

  void Trace(Visitor*) const override;

 private:
  HeapVector<Member<NDEFRecord>> records_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_NFC_NDEF_MESSAGE_H_
