// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_STORAGE_ACCESS_DOCUMENT_STORAGE_ACCESS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_STORAGE_ACCESS_DOCUMENT_STORAGE_ACCESS_H_

#include "third_party/blink/public/mojom/permissions/permission_status.mojom-blink-forward.h"
#include "third_party/blink/renderer/bindings/core/v8/idl_types.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class Document;
class ScriptState;
class StorageAccessHandle;
class StorageAccessTypes;

class DocumentStorageAccess final
    : public GarbageCollected<DocumentStorageAccess>,
      public Supplement<Document> {
 public:
  static const char kSupplementName[];
  static const char kNoAccessRequested[];
  static DocumentStorageAccess& From(Document& document);
  static ScriptPromise<IDLBoolean> hasStorageAccess(ScriptState* script_state,
                                                    Document& document);
  static ScriptPromise<IDLUndefined> requestStorageAccess(
      ScriptState* script_state,
      Document& document);
  static ScriptPromise<StorageAccessHandle> requestStorageAccess(
      ScriptState* script_state,
      Document& document,
      const StorageAccessTypes* storage_access_types);
  static ScriptPromise<IDLBoolean> hasUnpartitionedCookieAccess(
      ScriptState* script_state,
      Document& document);
  static ScriptPromise<IDLUndefined> requestStorageAccessFor(
      ScriptState* script_state,
      Document& document,
      const AtomicString& site);

  explicit DocumentStorageAccess(Document& document);
  void Trace(Visitor*) const override;

 private:
  ScriptPromise<IDLBoolean> hasStorageAccess(ScriptState* script_state);
  ScriptPromise<IDLUndefined> requestStorageAccess(ScriptState* script_state);
  ScriptPromise<StorageAccessHandle> requestStorageAccess(
      ScriptState* script_state,
      const StorageAccessTypes* storage_access_types);
  ScriptPromise<IDLBoolean> hasUnpartitionedCookieAccess(
      ScriptState* script_state);
  ScriptPromise<IDLUndefined> requestStorageAccessFor(ScriptState* script_state,
                                                      const AtomicString& site);

  // Attempt permission checks for unpartitioned storage access and enable
  // unpartitioned cookie access based on success if
  // `request_unpartitioned_cookie_access` is true. On success, resolved with
  // `on_resolve`.
  template <typename T>
  ScriptPromise<T> RequestStorageAccessImpl(
      ScriptState* script_state,
      bool request_unpartitioned_cookie_access,
      base::OnceCallback<void(ScriptPromiseResolver<T>*)> on_resolve);

  // Resolves the promise if the `status` can approve using `on_resolve`;
  // rejects the promise otherwise, and consumes user activation. Enables
  // unpartitioned cookie access if `request_unpartitioned_cookie_access` is
  // true.
  template <typename T>
  void ProcessStorageAccessPermissionState(
      ScriptPromiseResolver<T>* resolver,
      bool request_unpartitioned_cookie_access,
      base::OnceCallback<void(ScriptPromiseResolver<T>*)> on_resolve,
      mojom::blink::PermissionStatus status);

  // Resolves the promise if the `status` can approve; rejects the promise
  // otherwise, and consumes user activation.  Notably, does not modify the
  // per-frame storage access bit.
  void ProcessTopLevelStorageAccessPermissionState(
      ScriptPromiseResolver<IDLUndefined>* resolver,
      mojom::blink::PermissionStatus status);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_STORAGE_ACCESS_DOCUMENT_STORAGE_ACCESS_H_
