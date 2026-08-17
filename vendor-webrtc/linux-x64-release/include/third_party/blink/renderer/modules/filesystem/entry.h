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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_FILESYSTEM_ENTRY_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_FILESYSTEM_ENTRY_H_

#include "third_party/blink/renderer/modules/filesystem/dom_file_system.h"
#include "third_party/blink/renderer/modules/filesystem/entry_base.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class DirectoryEntry;
class V8EntryCallback;
class V8ErrorCallback;
class V8MetadataCallback;
class V8VoidCallback;

class MODULES_EXPORT Entry : public EntryBase {
  DEFINE_WRAPPERTYPEINFO();

 public:
  Entry(DOMFileSystemBase*, const String& full_path);

  DOMFileSystem* filesystem() const {
    return static_cast<DOMFileSystem*>(file_system_.Get());
  }
  DOMFileSystem* filesystem(ScriptState*) const;

  void getMetadata(ScriptState*,
                   V8MetadataCallback* success_callback = nullptr,
                   V8ErrorCallback* = nullptr);
  void moveTo(ScriptState*,
              DirectoryEntry* parent,
              const String& name = String(),
              V8EntryCallback* success_callback = nullptr,
              V8ErrorCallback* = nullptr) const;
  void copyTo(ScriptState*,
              DirectoryEntry* parent,
              const String& name = String(),
              V8EntryCallback* success_callback = nullptr,
              V8ErrorCallback* = nullptr) const;
  void remove(ScriptState*,
              V8VoidCallback* success_callback = nullptr,
              V8ErrorCallback* = nullptr) const;
  void getParent(ScriptState*,
                 V8EntryCallback* success_callback = nullptr,
                 V8ErrorCallback* = nullptr) const;
  String toURL(ScriptState*) const;

  void Trace(Visitor*) const override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_FILESYSTEM_ENTRY_H_
