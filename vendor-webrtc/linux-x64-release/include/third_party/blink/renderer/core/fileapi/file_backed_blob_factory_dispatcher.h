// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FILEAPI_FILE_BACKED_BLOB_FACTORY_DISPATCHER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FILEAPI_FILE_BACKED_BLOB_FACTORY_DISPATCHER_H_

#include "third_party/blink/public/common/associated_interfaces/associated_interface_provider.h"
#include "third_party/blink/public/mojom/blob/file_backed_blob_factory.mojom-blink.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/core/frame/local_frame.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_remote.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class CORE_EXPORT FileBackedBlobFactoryDispatcher
    : public GarbageCollected<FileBackedBlobFactoryDispatcher>,
      public Supplement<ExecutionContext>,
      public ExecutionContextClient {
 public:
  virtual ~FileBackedBlobFactoryDispatcher() = default;

  explicit FileBackedBlobFactoryDispatcher(ExecutionContext& context);

  static mojom::blink::FileBackedBlobFactory* GetFileBackedBlobFactory(
      ExecutionContext* context);

  void SetFileBackedBlobFactoryForTesting(
      mojo::PendingAssociatedRemote<mojom::blink::FileBackedBlobFactory>
          factory);

  void FlushForTesting();

  // GC
  void Trace(Visitor* visitor) const override;

  static const char kSupplementName[];

 private:
  friend class FileBackedBlobFactoryTestHelper;

  static FileBackedBlobFactoryDispatcher* From(ExecutionContext& context);

  mojom::blink::FileBackedBlobFactory* GetFileBackedBlobFactory();

  HeapMojoAssociatedRemote<mojom::blink::FileBackedBlobFactory> frame_remote_;

  HeapMojoRemote<mojom::blink::FileBackedBlobFactory> worker_remote_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FILEAPI_FILE_BACKED_BLOB_FACTORY_DISPATCHER_H_
