// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_LAUNCH_LAUNCH_PARAMS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_LAUNCH_LAUNCH_PARAMS_H_

#include "third_party/blink/renderer/modules/file_system_access/file_system_handle.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"

namespace blink {

class LaunchParams final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit LaunchParams(KURL target_url);
  explicit LaunchParams(HeapVector<Member<FileSystemHandle>> files);
  ~LaunchParams() override;

  // LaunchParams IDL interface.
  const HeapVector<Member<FileSystemHandle>>& files() { return files_; }
  const String& targetURL() { return target_url_.GetString(); }

  void Trace(Visitor*) const override;

 private:
  KURL target_url_;
  HeapVector<Member<FileSystemHandle>> files_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_LAUNCH_LAUNCH_PARAMS_H_
