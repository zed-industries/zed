/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_FILESYSTEM_LOCAL_FILE_SYSTEM_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_FILESYSTEM_LOCAL_FILE_SYSTEM_H_

#include <memory>

#include "base/functional/callback.h"
#include "third_party/blink/public/mojom/filesystem/file_system.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/platform/bindings/name_client.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"

namespace blink {

class ExecutionContext;
class FileSystemCallbacks;
class KURL;
class ResolveURICallbacks;

class LocalFileSystem final : public GarbageCollected<LocalFileSystem>,
                              public Supplement<ExecutionContext>,
                              public NameClient {
 public:
  enum SynchronousType { kAsynchronous, kSynchronous };

  static const char kSupplementName[];

  explicit LocalFileSystem(ExecutionContext&);

  LocalFileSystem(const LocalFileSystem&) = delete;
  LocalFileSystem& operator=(const LocalFileSystem&) = delete;

  ~LocalFileSystem() final = default;

  void ResolveURL(const KURL&,
                  std::unique_ptr<ResolveURICallbacks>,
                  SynchronousType sync_type);
  void RequestFileSystem(mojom::blink::FileSystemType,
                         int64_t size,
                         std::unique_ptr<FileSystemCallbacks>,
                         SynchronousType sync_type);

  static LocalFileSystem* From(ExecutionContext&);

  const char* NameInHeapSnapshot() const override { return "LocalFileSystem"; }

 private:
  void ResolveURLCallback(const KURL& file_system_url,
                          std::unique_ptr<ResolveURICallbacks> callbacks,
                          SynchronousType sync_type,
                          bool allowed);
  void RequestFileSystemCallback(mojom::blink::FileSystemType type,
                                 std::unique_ptr<FileSystemCallbacks> callbacks,
                                 SynchronousType sync_type,
                                 bool allowed);
  void RequestFileSystemAccessInternal(base::OnceCallback<void(bool)> callback);
  void FileSystemNotAllowedInternal(std::unique_ptr<FileSystemCallbacks>);
  void FileSystemNotAllowedInternal(std::unique_ptr<ResolveURICallbacks>);
  void FileSystemAllowedInternal(mojom::blink::FileSystemType,
                                 std::unique_ptr<FileSystemCallbacks> callbacks,
                                 SynchronousType sync_type);
  void ResolveURLInternal(const KURL&,
                          std::unique_ptr<ResolveURICallbacks>,
                          SynchronousType sync_type);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_FILESYSTEM_LOCAL_FILE_SYSTEM_H_
