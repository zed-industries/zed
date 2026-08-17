// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MOJO_MOJO_FILE_SYSTEM_ACCESS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MOJO_MOJO_FILE_SYSTEM_ACCESS_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/core/mojo/mojo.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class MojoHandle;
class FileSystemFileHandle;

class MojoFileSystemAccess final
    : public GarbageCollected<MojoFileSystemAccess>,
      public Supplement<Mojo> {
 public:
  static const char kSupplementName[];
  explicit MojoFileSystemAccess(Mojo&);
  static MojoFileSystemAccess& From(Mojo&);

  // IDL interface methods:
  static MojoHandle* getFileSystemAccessTransferToken(
      FileSystemFileHandle* fs_handle);

  void Trace(Visitor* visitor) const override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MOJO_MOJO_FILE_SYSTEM_ACCESS_H_
