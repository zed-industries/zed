// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_FILE_SYSTEM_ACCESS_GLOBAL_FILE_SYSTEM_ACCESS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_FILE_SYSTEM_ACCESS_GLOBAL_FILE_SYSTEM_ACCESS_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class OpenFilePickerOptions;
class SaveFilePickerOptions;
class DirectoryPickerOptions;
class ExceptionState;
class FileSystemDirectoryHandle;
class FileSystemFileHandle;
class LocalDOMWindow;
class ScriptState;

class GlobalFileSystemAccess {
  STATIC_ONLY(GlobalFileSystemAccess);

 public:
  static ScriptPromise<IDLSequence<FileSystemFileHandle>> showOpenFilePicker(
      ScriptState*,
      LocalDOMWindow&,
      const OpenFilePickerOptions*,
      ExceptionState&);
  static ScriptPromise<FileSystemFileHandle> showSaveFilePicker(
      ScriptState*,
      LocalDOMWindow&,
      const SaveFilePickerOptions*,
      ExceptionState&);
  static ScriptPromise<FileSystemDirectoryHandle> showDirectoryPicker(
      ScriptState*,
      LocalDOMWindow&,
      const DirectoryPickerOptions*,
      ExceptionState&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_FILE_SYSTEM_ACCESS_GLOBAL_FILE_SYSTEM_ACCESS_H_
