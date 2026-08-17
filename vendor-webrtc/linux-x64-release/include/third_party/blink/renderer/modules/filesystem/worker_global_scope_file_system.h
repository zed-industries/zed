/*
 * Copyright (C) 2008, 2009 Apple Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_FILESYSTEM_WORKER_GLOBAL_SCOPE_FILE_SYSTEM_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_FILESYSTEM_WORKER_GLOBAL_SCOPE_FILE_SYSTEM_H_

#include "third_party/blink/renderer/modules/filesystem/dom_file_system_sync.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class EntrySync;
class ExceptionState;
class V8EntryCallback;
class V8ErrorCallback;
class V8FileSystemCallback;
class WorkerGlobalScope;

class WorkerGlobalScopeFileSystem {
  STATIC_ONLY(WorkerGlobalScopeFileSystem);

 public:
  enum {
    kTemporary,
    kPersistent,
  };

  static void webkitRequestFileSystem(WorkerGlobalScope&,
                                      int type,
                                      int64_t size,
                                      V8FileSystemCallback* success_callback,
                                      V8ErrorCallback*);
  static DOMFileSystemSync* webkitRequestFileSystemSync(WorkerGlobalScope&,
                                                        int type,
                                                        int64_t size,
                                                        ExceptionState&);
  static void webkitResolveLocalFileSystemURL(WorkerGlobalScope&,
                                              const String& url,
                                              V8EntryCallback* success_callback,
                                              V8ErrorCallback*);
  static EntrySync* webkitResolveLocalFileSystemSyncURL(WorkerGlobalScope&,
                                                        const String& url,
                                                        ExceptionState&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_FILESYSTEM_WORKER_GLOBAL_SCOPE_FILE_SYSTEM_H_
