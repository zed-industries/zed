// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_FILE_BACKED_BLOB_FACTORY_TEST_HELPER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_FILE_BACKED_BLOB_FACTORY_TEST_HELPER_H_

#include "mojo/public/cpp/bindings/associated_receiver.h"
#include "third_party/blink/public/mojom/blob/file_backed_blob_factory.mojom-blink.h"
#include "third_party/blink/renderer/platform/blob/testing/fake_file_backed_blob_factory.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"

namespace blink {

class ExecutionContext;

// Helper class setting up FileBackedBlobFactory for tests.
class FileBackedBlobFactoryTestHelper {
 public:
  explicit FileBackedBlobFactoryTestHelper(ExecutionContext* context);
  ~FileBackedBlobFactoryTestHelper();
  FileBackedBlobFactoryTestHelper(const FileBackedBlobFactoryTestHelper&) =
      delete;
  FileBackedBlobFactoryTestHelper& operator=(
      const FileBackedBlobFactoryTestHelper&) = delete;

  void FlushForTesting();

 private:
  Persistent<ExecutionContext> context_;
  FakeFileBackedBlobFactory factory_;
  mojo::AssociatedReceiver<mojom::blink::FileBackedBlobFactory> receiver_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TESTING_FILE_BACKED_BLOB_FACTORY_TEST_HELPER_H_
