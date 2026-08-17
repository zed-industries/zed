// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_GEOLOCATION_GEOLOCATION_WATCHERS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_GEOLOCATION_GEOLOCATION_WATCHERS_H_

#include "third_party/blink/renderer/platform/bindings/name_client.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class GeoNotifier;

class GeolocationWatchers final : public GarbageCollected<GeolocationWatchers>,
                                  public NameClient {
 public:
  GeolocationWatchers() = default;
  ~GeolocationWatchers() override = default;
  void Trace(Visitor*) const;
  const char* NameInHeapSnapshot() const override {
    return "GeolocationWatchers";
  }

  bool Add(int id, GeoNotifier*);
  GeoNotifier* Find(int id) const;
  void Remove(int id);
  void Remove(GeoNotifier*);
  bool Contains(GeoNotifier*) const;
  void Clear();
  bool IsEmpty() const;
  void Swap(GeolocationWatchers& other);

  auto& Notifiers() { return id_to_notifier_map_.Values(); }

  void CopyNotifiersToVector(HeapVector<Member<GeoNotifier>>&) const;

 private:
  typedef HeapHashMap<int, Member<GeoNotifier>> IdToNotifierMap;
  typedef HeapHashMap<Member<GeoNotifier>, int> NotifierToIdMap;

  IdToNotifierMap id_to_notifier_map_;
  NotifierToIdMap notifier_to_id_map_;
};

inline void swap(GeolocationWatchers& a, GeolocationWatchers& b) {
  a.Swap(b);
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_GEOLOCATION_GEOLOCATION_WATCHERS_H_
