/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_QUOTA_NAVIGATOR_STORAGE_QUOTA_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_QUOTA_NAVIGATOR_STORAGE_QUOTA_H_

#include "third_party/blink/renderer/modules/quota/deprecated_storage_quota.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class Navigator;
class NavigatorBase;
class StorageManager;

class NavigatorStorageQuota final
    : public GarbageCollected<NavigatorStorageQuota>,
      public Supplement<NavigatorBase> {
 public:
  static const char kSupplementName[];

  // Web-exposed on window only.
  static DeprecatedStorageQuota* webkitTemporaryStorage(Navigator&);
  static DeprecatedStorageQuota* webkitPersistentStorage(Navigator&);

  // Web-exposed on both window and worker.
  static StorageManager* storage(NavigatorBase&);

  explicit NavigatorStorageQuota(NavigatorBase&);

  void Trace(Visitor*) const override;

 private:
  static NavigatorStorageQuota& From(NavigatorBase&);

  mutable Member<DeprecatedStorageQuota> temporary_storage_;
  mutable Member<StorageManager> storage_manager_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_QUOTA_NAVIGATOR_STORAGE_QUOTA_H_
