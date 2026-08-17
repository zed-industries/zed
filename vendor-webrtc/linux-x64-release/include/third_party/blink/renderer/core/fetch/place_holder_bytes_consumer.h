// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_PLACE_HOLDER_BYTES_CONSUMER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_PLACE_HOLDER_BYTES_CONSUMER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/loader/fetch/bytes_consumer.h"

namespace blink {

// A BytesConsumer implementation that acts as a place holder. The actual
// BytesConsumer can be provided later by calling "Update()".
class CORE_EXPORT PlaceHolderBytesConsumer final : public BytesConsumer {
 public:
  // BytesConsumer implementation
  Result BeginRead(base::span<const char>& buffer) override;
  Result EndRead(size_t read_size) override;
  scoped_refptr<BlobDataHandle> DrainAsBlobDataHandle(
      BlobSizePolicy policy) override;
  scoped_refptr<EncodedFormData> DrainAsFormData() override;
  mojo::ScopedDataPipeConsumerHandle DrainAsDataPipe() override;
  void SetClient(BytesConsumer::Client* client) override;
  void ClearClient() override;
  void Cancel() override;
  PublicState GetPublicState() const override;
  Error GetError() const override;
  String DebugName() const override;

  // This function can be called at most once.
  void Update(BytesConsumer* consumer);

  void Trace(Visitor* visitor) const override;

 private:
  Member<BytesConsumer> underlying_;
  Member<Client> client_;
  bool is_cancelled_ = false;
  SEQUENCE_CHECKER(sequence_checker_);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_PLACE_HOLDER_BYTES_CONSUMER_H_
