// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_STORAGE_DOM_WINDOW_STORAGE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_STORAGE_DOM_WINDOW_STORAGE_H_

#include "mojo/public/cpp/bindings/pending_remote.h"
#include "third_party/blink/public/mojom/dom_storage/storage_area.mojom-blink-forward.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class ExceptionState;
class LocalDOMWindow;
class StorageArea;

class DOMWindowStorage final : public GarbageCollected<DOMWindowStorage>,
                               public Supplement<LocalDOMWindow> {
 public:
  static const char kSupplementName[];

  static DOMWindowStorage& From(LocalDOMWindow&);
  static StorageArea* sessionStorage(LocalDOMWindow&, ExceptionState&);
  static StorageArea* localStorage(LocalDOMWindow&, ExceptionState&);

  explicit DOMWindowStorage(LocalDOMWindow&);

  StorageArea* sessionStorage(ExceptionState&) const;
  StorageArea* localStorage(ExceptionState&) const;
  StorageArea* OptionalSessionStorage() const { return session_storage_.Get(); }
  StorageArea* OptionalLocalStorage() const { return local_storage_.Get(); }

  // These Init* methods allow initializing the StorageArea as an optimization
  // to avoid it being requested from the browser process, which can be slow.
  // These storage areas are ignored if a cached storage area already exists for
  // this storage key/namespace.
  void InitLocalStorage(
      mojo::PendingRemote<mojom::blink::StorageArea> local_storage_area) const;
  void InitSessionStorage(mojo::PendingRemote<mojom::blink::StorageArea>
                              session_storage_area) const;

  void Trace(Visitor*) const override;

 private:
  StorageArea* GetOrCreateSessionStorage(
      ExceptionState& exception_state,
      mojo::PendingRemote<mojom::blink::StorageArea> storage_area_for_init)
      const;
  StorageArea* GetOrCreateLocalStorage(
      ExceptionState& exception_state,
      mojo::PendingRemote<mojom::blink::StorageArea> storage_area_for_init)
      const;

  mutable Member<StorageArea> session_storage_;
  mutable Member<StorageArea> local_storage_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_STORAGE_DOM_WINDOW_STORAGE_H_
