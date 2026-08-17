// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FILEAPI_URL_FILE_API_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FILEAPI_URL_FILE_API_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class ExceptionState;
class ExecutionContext;
class ScriptState;
class Blob;

class CORE_EXPORT URLFileAPI {
  STATIC_ONLY(URLFileAPI);

 public:
  static WTF::String createObjectURL(ScriptState*, Blob*, ExceptionState&);
  static void revokeObjectURL(ScriptState*, const WTF::String&);
  static void revokeObjectURL(ExecutionContext*, const WTF::String&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FILEAPI_URL_FILE_API_H_
