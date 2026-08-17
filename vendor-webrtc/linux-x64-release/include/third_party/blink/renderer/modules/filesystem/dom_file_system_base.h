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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_FILESYSTEM_DOM_FILE_SYSTEM_BASE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_FILESYSTEM_DOM_FILE_SYSTEM_BASE_H_

#include "third_party/blink/public/mojom/filesystem/file_system.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_file_system_flags.h"
#include "third_party/blink/renderer/core/fileapi/file_error.h"
#include "third_party/blink/renderer/modules/filesystem/file_system_callbacks.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class DirectoryReaderBase;
class EntryBase;
class ExecutionContext;
class File;
class FileMetadata;
class SecurityOrigin;

// A common base class for DOMFileSystem and DOMFileSystemSync.
class MODULES_EXPORT DOMFileSystemBase : public ScriptWrappable {
 public:
  enum SynchronousType {
    kSynchronous,
    kAsynchronous,
  };

  // Path prefixes that are used in the filesystem URLs (that can be obtained by
  // toURL()).  http://www.w3.org/TR/file-system-api/#widl-Entry-toURL
  static const char kPersistentPathPrefix[];
  static const char kTemporaryPathPrefix[];
  static const char kIsolatedPathPrefix[];
  static const char kExternalPathPrefix[];

  ~DOMFileSystemBase() override;

  // These are called when a new callback is created and resolved in
  // FileSystem API, so that subclasses can track the number of pending
  // callbacks if necessary.
  virtual void AddPendingCallbacks() {}
  virtual void RemovePendingCallbacks() {}

  // Overridden by subclasses to handle sync vs async error-handling.
  using ErrorCallback = base::OnceCallback<void(base::File::Error)>;
  virtual void ReportError(ErrorCallback, base::File::Error error) = 0;

  const String& name() const { return name_; }
  mojom::blink::FileSystemType GetType() const { return type_; }
  KURL RootURL() const { return filesystem_root_url_; }
  const SecurityOrigin* GetSecurityOrigin() const;

  // The clonable flag is used in the structured clone algorithm to test
  // whether the FileSystem API object is permitted to be cloned. It defaults
  // to false, and must be explicitly set by internal code permit cloning.
  void MakeClonable() { clonable_ = true; }
  bool Clonable() const { return clonable_; }

  static bool IsValidType(mojom::blink::FileSystemType);
  static KURL CreateFileSystemRootURL(const String& origin,
                                      mojom::blink::FileSystemType);
  bool SupportsToURL() const;
  KURL CreateFileSystemURL(const EntryBase*) const;
  KURL CreateFileSystemURL(const String& full_path) const;
  static bool PathToAbsolutePath(mojom::blink::FileSystemType,
                                 const EntryBase*,
                                 String path,
                                 String& absolute_path);
  static bool PathPrefixToFileSystemType(const String& path_prefix,
                                         mojom::blink::FileSystemType&);
  static File* CreateFile(ExecutionContext* context,
                          const FileMetadata&,
                          const KURL& file_system_url,
                          mojom::blink::FileSystemType,
                          const String name);

  // Actual FileSystem API implementations. All the validity checks on virtual
  // paths are done at this level.
  void GetMetadata(const EntryBase*,
                   MetadataCallbacks::SuccessCallback,
                   MetadataCallbacks::ErrorCallback,
                   SynchronousType = kAsynchronous);
  void Move(const EntryBase* source,
            EntryBase* parent,
            const String& name,
            EntryCallbacks::SuccessCallback success_callback,
            EntryCallbacks::ErrorCallback error_callback,
            SynchronousType = kAsynchronous);
  void Copy(const EntryBase* source,
            EntryBase* parent,
            const String& name,
            EntryCallbacks::SuccessCallback,
            EntryCallbacks::ErrorCallback,
            SynchronousType = kAsynchronous);
  void Remove(const EntryBase*,
              VoidCallbacks::SuccessCallback,
              VoidCallbacks::ErrorCallback,
              SynchronousType = kAsynchronous);
  void RemoveRecursively(const EntryBase*,
                         VoidCallbacks::SuccessCallback,
                         VoidCallbacks::ErrorCallback,
                         SynchronousType = kAsynchronous);
  void GetParent(const EntryBase*,
                 EntryCallbacks::SuccessCallback,
                 EntryCallbacks::ErrorCallback);
  void GetFile(const EntryBase*,
               const String& path,
               const FileSystemFlags*,
               EntryCallbacks::SuccessCallback,
               EntryCallbacks::ErrorCallback,
               SynchronousType = kAsynchronous);
  void GetDirectory(const EntryBase*,
                    const String& path,
                    const FileSystemFlags*,
                    EntryCallbacks::SuccessCallback,
                    EntryCallbacks::ErrorCallback,
                    SynchronousType = kAsynchronous);
  void ReadDirectory(DirectoryReaderBase*,
                     const String& path,
                     const EntriesCallbacks::SuccessCallback&,
                     EntriesCallbacks::ErrorCallback,
                     SynchronousType = kAsynchronous);

  void Trace(Visitor*) const override;

 protected:
  DOMFileSystemBase(ExecutionContext*,
                    const String& name,
                    mojom::blink::FileSystemType,
                    const KURL& root_url);

  friend class DOMFileSystemBaseTest;
  friend class DOMFileSystemSync;

  Member<ExecutionContext> context_;
  String name_;
  mojom::blink::FileSystemType type_;
  KURL filesystem_root_url_;
  bool clonable_;
};

inline bool operator==(const DOMFileSystemBase& a, const DOMFileSystemBase& b) {
  return a.name() == b.name() && a.GetType() == b.GetType() &&
         a.RootURL() == b.RootURL();
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_FILESYSTEM_DOM_FILE_SYSTEM_BASE_H_
