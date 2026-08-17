// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_FORM_DATA_BYTES_CONSUMER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_FORM_DATA_BYTES_CONSUMER_H_

#include "base/containers/span.h"
#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "third_party/blink/renderer/platform/loader/fetch/bytes_consumer.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class DOMArrayBuffer;
class DOMArrayBufferView;
class EncodedFormData;
class ExecutionContext;

class FormDataBytesConsumer final : public BytesConsumer {
 public:
  explicit CORE_EXPORT FormDataBytesConsumer(const String&);
  explicit CORE_EXPORT FormDataBytesConsumer(DOMArrayBuffer*);
  explicit CORE_EXPORT FormDataBytesConsumer(DOMArrayBufferView*);
  explicit CORE_EXPORT FormDataBytesConsumer(SegmentedBuffer&&);
  explicit CORE_EXPORT FormDataBytesConsumer(base::span<const uint8_t>);
  CORE_EXPORT FormDataBytesConsumer(ExecutionContext*,
                                    scoped_refptr<EncodedFormData>);
  CORE_EXPORT FormDataBytesConsumer(ExecutionContext*,
                                    scoped_refptr<EncodedFormData>,
                                    BytesConsumer* consumer_for_testing);

  // BytesConsumer implementation
  Result BeginRead(base::span<const char>& buffer) override {
    return impl_->BeginRead(buffer);
  }
  Result EndRead(size_t read_size) override {
    return impl_->EndRead(read_size);
  }
  scoped_refptr<BlobDataHandle> DrainAsBlobDataHandle(
      BlobSizePolicy policy) override {
    return impl_->DrainAsBlobDataHandle(policy);
  }
  scoped_refptr<EncodedFormData> DrainAsFormData() override {
    return impl_->DrainAsFormData();
  }
  void SetClient(BytesConsumer::Client* client) override {
    impl_->SetClient(client);
  }
  void ClearClient() override { impl_->ClearClient(); }
  void Cancel() override { impl_->Cancel(); }
  PublicState GetPublicState() const override {
    return impl_->GetPublicState();
  }
  Error GetError() const override { return impl_->GetError(); }
  String DebugName() const override { return impl_->DebugName(); }

  void Trace(Visitor* visitor) const override {
    visitor->Trace(impl_);
    BytesConsumer::Trace(visitor);
  }

 private:
  static BytesConsumer* GetImpl(ExecutionContext*,
                                scoped_refptr<EncodedFormData>,
                                BytesConsumer* consumer_for_testing);

  const Member<BytesConsumer> impl_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_FORM_DATA_BYTES_CONSUMER_H_
