// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_BLOB_BLOB_BYTES_PROVIDER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_BLOB_BLOB_BYTES_PROVIDER_H_

#include "base/gtest_prod_util.h"
#include "base/sequence_checker.h"
#include "base/task/sequenced_task_runner.h"
#include "third_party/blink/public/mojom/blob/blob_registry.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/blob/data_element.mojom-blink.h"
#include "third_party/blink/renderer/platform/blob/blob_data.h"

namespace blink {

// Implementation of the BytesProvider mojo interface, used to transport bytes
// making up a blob to the browser process, at the request of the blob service.
//
// Typical usage of this class creates and calls AppendData on one thread, and
// then transfers ownership of the class to a different thread using the `Bind`
// method. This ensures that the various Request* methods are called on a
// thread that is allowed to do File IO.
class PLATFORM_EXPORT BlobBytesProvider : public mojom::blink::BytesProvider {
 public:
  // All consecutive items that are accumulate to < this number will have the
  // data appended to the same item.
  static constexpr size_t kMaxConsolidatedItemSizeInBytes = 15 * 1024;

  BlobBytesProvider();
  ~BlobBytesProvider() override;

  void AppendData(scoped_refptr<RawData>);
  void AppendData(base::span<const char>);

  // Binds `provider` to `receiver` on a threadpool task runner, transferring
  // ownership.
  static void Bind(std::unique_ptr<BlobBytesProvider> provider,
                   mojo::PendingReceiver<mojom::blink::BytesProvider> receiver);

  // BytesProvider implementation:
  void RequestAsReply(RequestAsReplyCallback) override;
  void RequestAsStream(mojo::ScopedDataPipeProducerHandle) override;
  void RequestAsFile(uint64_t source_offset,
                     uint64_t source_size,
                     base::File,
                     uint64_t file_offset,
                     RequestAsFileCallback) override;

 private:
  FRIEND_TEST_ALL_PREFIXES(BlobBytesProviderTest, Consolidation);

  static void IncreaseChildProcessRefCount();
  static void DecreaseChildProcessRefCount();

  Vector<scoped_refptr<RawData>> data_ GUARDED_BY_CONTEXT(sequence_checker_);
  // |offsets_| always contains exactly one fewer item than |data_| (except when
  // |data_| itself is empty).
  // offsets_[x] is equal to the sum of data_[i].length for all i <= x.
  Vector<uint64_t> offsets_ GUARDED_BY_CONTEXT(sequence_checker_);

  SEQUENCE_CHECKER(sequence_checker_);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_BLOB_BLOB_BYTES_PROVIDER_H_
