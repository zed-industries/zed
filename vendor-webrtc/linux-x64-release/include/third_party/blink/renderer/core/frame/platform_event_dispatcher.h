// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_PLATFORM_EVENT_DISPATCHER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_PLATFORM_EVENT_DISPATCHER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {
class PlatformEventController;
class LocalDOMWindow;

class CORE_EXPORT PlatformEventDispatcher : public GarbageCollectedMixin {
 public:
  // Adds a controller to be notified when a change event occurs and starts
  // listening for change events. |frame| is the frame that will be passed to
  // the dispatcher's StartListening method. The caller must provide a valid,
  // non-nullptr frame.
  //
  // Note that the frame associated with the controller's document may be
  // nullptr if the document was shut down, which can occur when a frame
  // navigates from an initial empty document to another same-origin document.
  // If the controller was initialized with the initial empty document, it may
  // need to provide a valid frame from another source, for instance the
  // DOMWindow.
  // TODO(crbug.com/850619): fix all the callsites, currently not all of them
  // (and unittests) are guaranteed to pass a non-nullptr frame.
  void AddController(PlatformEventController*, LocalDOMWindow*);

  // Removes a controller from |controllers_| and stops listening if there are
  // no more registered controllers.
  void RemoveController(PlatformEventController*);

  void Trace(Visitor*) const override;

 protected:
  PlatformEventDispatcher();

  void NotifyControllers();

  virtual void StartListening(LocalDOMWindow*) = 0;
  virtual void StopListening() = 0;

 private:
  void PurgeControllers();

  HeapHashSet<WeakMember<PlatformEventController>> controllers_;
  bool is_dispatching_;
  bool is_listening_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAME_PLATFORM_EVENT_DISPATCHER_H_
