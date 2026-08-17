// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_NFC_NDEF_RECORD_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_NFC_NDEF_RECORD_H_

#include <stdint.h>

#include <optional>

#include "services/device/public/mojom/nfc.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/typed_arrays/array_buffer_view_helpers.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class DOMDataView;
class ExceptionState;
class NDEFMessage;
class NDEFRecordInit;
class ScriptState;

class MODULES_EXPORT NDEFRecord final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  // This function is only supposed to be used by the automatically generated
  // bindings code. Use Create() instead.
  static NDEFRecord* CreateForBindings(const ScriptState*,
                                       const NDEFRecordInit*,
                                       ExceptionState&);

  // |is_embedded| indicates if this record is within the context of a parent
  // record.
  static NDEFRecord* Create(const ScriptState*,
                            const NDEFRecordInit*,
                            ExceptionState&,
                            uint8_t records_depth,
                            bool is_embedded);

  explicit NDEFRecord(device::mojom::NDEFRecordTypeCategory,
                      const String& record_type,
                      const String& id,
                      WTF::Vector<uint8_t>);

  // For constructing a "smart-poster", an external type or a local type record
  // whose payload is an NDEF message.
  explicit NDEFRecord(device::mojom::NDEFRecordTypeCategory,
                      const String& record_type,
                      const String& id,
                      NDEFMessage*);

  // Only for constructing "text" type record. The type category will be
  // device::mojom::NDEFRecordTypeCategory::kStandardized.
  explicit NDEFRecord(const String& id,
                      const String& encoding,
                      const String& lang,
                      WTF::Vector<uint8_t>);

  // Only for constructing "text" type record from just a text. The type
  // category will be device::mojom::NDEFRecordTypeCategory::kStandardized.
  // Called by NDEFMessage.
  explicit NDEFRecord(const ScriptState*, const String&);

  // Only for constructing "mime" type record. The type category will be
  // device::mojom::NDEFRecordTypeCategory::kStandardized.
  explicit NDEFRecord(const String& id,
                      const String& media_type,
                      WTF::Vector<uint8_t>);

  explicit NDEFRecord(const device::mojom::blink::NDEFRecord&);

  const String& recordType() const { return record_type_; }
  const String& mediaType() const;
  const String& id() const { return id_; }
  const String& encoding() const { return encoding_; }
  const String& lang() const { return lang_; }
  NotShared<DOMDataView> data() const;
  std::optional<HeapVector<Member<NDEFRecord>>> toRecords(
      ExceptionState& exception_state) const;

  device::mojom::NDEFRecordTypeCategory category() const { return category_; }
  const WTF::Vector<uint8_t>& payloadData() const { return payload_data_; }
  const NDEFMessage* payload_message() const { return payload_message_.Get(); }

  void Trace(Visitor*) const override;

 private:
  const device::mojom::NDEFRecordTypeCategory category_;
  const String record_type_;
  const String id_;
  const String media_type_;
  const String encoding_;
  const String lang_;
  // Holds the NDEFRecord.[[PayloadData]] bytes defined at
  // https://w3c.github.io/web-nfc/#the-ndefrecord-interface.
  const WTF::Vector<uint8_t> payload_data_;
  // |payload_data_| parsed as an NDEFMessage. This field will be set for some
  // "smart-poster", external, and local type records.
  const Member<NDEFMessage> payload_message_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_NFC_NDEF_RECORD_H_
