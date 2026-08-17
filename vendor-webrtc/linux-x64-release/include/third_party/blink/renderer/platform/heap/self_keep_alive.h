// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_SELF_KEEP_ALIVE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_SELF_KEEP_ALIVE_H_

#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/gc_plugin.h"

namespace blink {

// SelfKeepAlive<Object> is the idiom to use for objects that have to keep
// themselves temporarily alive and cannot rely on there being some
// external reference in that interval:
//
//  class Opener : public GarbageCollected<Opener> {
//   public:
//    ...
//    void Open() {
//      // Retain a self-reference while in an opened state:
//      keep_alive_ = this;
//      ....
//    }
//
//    void Close() {
//      // Clear self-reference that ensured we were kept alive while opened.
//      keep_alive_.Clear();
//      ....
//    }
//
//   private:
//    ...
//    SelfKeepAlive<Opener> keep_alive_;
//  };
//
// The responsibility to call Clear() in a timely fashion resides with the
// implementation of the object.
template <typename Self>
class SelfKeepAlive final {
  DISALLOW_NEW();

 public:
  explicit SelfKeepAlive(
      const PersistentLocation& loc = PERSISTENT_LOCATION_FROM_HERE)
      : keep_alive_(loc) {}
  explicit SelfKeepAlive(
      Self* self,
      const PersistentLocation& loc = PERSISTENT_LOCATION_FROM_HERE)
      : keep_alive_(self, loc) {}

  SelfKeepAlive& operator=(Self* self) {
    DCHECK(!keep_alive_ || keep_alive_.Get() == self);
    keep_alive_ = self;
    return *this;
  }

  void Clear() { keep_alive_.Clear(); }

  explicit operator bool() const { return keep_alive_; }

 private:
  GC_PLUGIN_IGNORE("Allowed to temporarily introduce non reclaimable memory.")
  Persistent<Self> keep_alive_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_HEAP_SELF_KEEP_ALIVE_H_
