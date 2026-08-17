// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_FILE_SYSTEM_ACCESS_BUCKET_FILE_SYSTEM_AGENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_FILE_SYSTEM_ACCESS_BUCKET_FILE_SYSTEM_AGENT_H_

#include "third_party/blink/renderer/core/frame/local_dom_window.h"
#include "third_party/blink/renderer/core/frame/navigator.h"
#include "third_party/blink/renderer/core/inspector/inspector_page_agent.h"
#include "third_party/blink/renderer/core/inspector/protocol/file_system.h"
#include "third_party/blink/renderer/modules/file_system_access/file_system_directory_handle.h"

namespace blink {

class StorageBucket;

const char kDefaultBucketName[] = "_default";

// This class implements the `protocol::FileSystem::Metainfo` which will
// receive call from DevTools.
class MODULES_EXPORT BucketFileSystemAgent final
    : public InspectorBaseAgent<protocol::FileSystem::Metainfo> {
 public:
  explicit BucketFileSystemAgent(InspectedFrames*);
  ~BucketFileSystemAgent() override;

  // Converts a file system access error to a protocol::Response.
  static protocol::Response HandleError(
      mojom::blink::FileSystemAccessErrorPtr error);

  // This method is called by DevTools to retrieve all nested files and
  // directories for a given `file_system_locator`.
  void getDirectory(
      std::unique_ptr<protocol::FileSystem::BucketFileSystemLocator>
          file_system_locator,
      std::unique_ptr<protocol::FileSystem::Backend::GetDirectoryCallback>
          callback) override;

  void Trace(Visitor*) const override;

 private:
  void DidGetDirectoryHandle(
      ExecutionContext* execution_context,
      const String& storage_key,
      const String& directory_name,
      std::unique_ptr<protocol::FileSystem::Backend::GetDirectoryCallback>
          callback,
      mojom::blink::FileSystemAccessErrorPtr result,
      FileSystemDirectoryHandle* handle);

  StorageBucket* GetStorageBucket(const String& storage_key,
                                  const String& bucket_name);

  Member<InspectedFrames> inspected_frames_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_FILE_SYSTEM_ACCESS_BUCKET_FILE_SYSTEM_AGENT_H_
