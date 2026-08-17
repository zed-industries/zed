// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_FINISH_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_FINISH_OBSERVER_H_

#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

// ResourceFinishObserver is different from ResourceClient in several ways.
//  - NotifyFinished is dispatched asynchronously.
//  - ResourceFinishObservers will be removed from Resource when the load
//  finishes.
//  - This class is not intended to be "subclassed" per each Resource subclass.
//    There is no ImageResourceFinishObserver, for example.
// ResourceFinishObserver should be quite simple. All notifications must be
// notified AFTER the loading finishes.
class PLATFORM_EXPORT ResourceFinishObserver
    : public GarbageCollected<ResourceFinishObserver> {
 public:
  virtual ~ResourceFinishObserver() = default;

  // Called asynchronously when loading finishes.
  // Note that this can be dispatched after removing |this| client from a
  // Resource, because of the asynchronicity.
  virtual void NotifyFinished() = 0;
  // Name for debugging
  virtual String DebugName() const = 0;

  virtual void Trace(Visitor* visitor) const {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_RESOURCE_FINISH_OBSERVER_H_
