// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_BLOB_BYTES_CONSUMER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_BLOB_BYTES_CONSUMER_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/loader/fetch/bytes_consumer.h"

namespace blink {

class BlobDataHandle;
class ExecutionContext;

// A BlobBytesConsumer is created from a blob handle and it will
// return a valid handle from drainAsBlobDataHandle as much as possible.
class CORE_EXPORT BlobBytesConsumer final : public BytesConsumer {
 public:
  // |handle| can be null. In that case this consumer gets closed.
  BlobBytesConsumer(ExecutionContext*,
                    scoped_refptr<BlobDataHandle> /* handle */);
  ~BlobBytesConsumer() override;

  // BytesConsumer implementation
  Result BeginRead(base::span<const char>& buffer) override;
  Result EndRead(size_t read_size) override;
  scoped_refptr<BlobDataHandle> DrainAsBlobDataHandle(BlobSizePolicy) override;
  scoped_refptr<EncodedFormData> DrainAsFormData() override;
  void SetClient(BytesConsumer::Client*) override;
  void ClearClient() override;
  void Cancel() override;
  PublicState GetPublicState() const override;
  Error GetError() const override;
  String DebugName() const override { return "BlobBytesConsumer"; }

  void Trace(Visitor*) const override;

 private:
  class BlobClient;
  Member<ExecutionContext> execution_context_;
  scoped_refptr<BlobDataHandle> blob_data_handle_;
  Member<BlobBytesConsumer::BlobClient> blob_client_;
  Member<BytesConsumer> nested_consumer_;
  Member<BytesConsumer::Client> client_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_BLOB_BYTES_CONSUMER_H_
