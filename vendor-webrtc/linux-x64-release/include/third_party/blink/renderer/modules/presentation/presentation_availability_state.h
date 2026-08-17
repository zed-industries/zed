// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PRESENTATION_PRESENTATION_AVAILABILITY_STATE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PRESENTATION_PRESENTATION_AVAILABILITY_STATE_H_

#include <memory>

#include "base/memory/raw_ptr.h"
#include "third_party/blink/public/mojom/presentation/presentation.mojom-blink.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/presentation/presentation_availability.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class PresentationAvailabilityObserver;

// Maintains the states of PresentationAvailability objects in a frame. It is
// also responsible for querying the availability values from
// PresentationService for the URLs given in a PresentationRequest. As an
// optimization, the result will be cached and shared with other
// PresentationRequests containing the same URL. PresentationAvailabilityState
// is owned by PresentationController in the same frame.
// TODO(crbug.com/780109): Improve encapsulation of PresentationAvailability and
// this class by moving the multiple URL tracking logic to the former, and
// consolidating this class's APIs to take repeating callbacks.
class MODULES_EXPORT PresentationAvailabilityState final
    : public GarbageCollected<PresentationAvailabilityState> {
 public:
  explicit PresentationAvailabilityState(mojom::blink::PresentationService*);

  PresentationAvailabilityState(const PresentationAvailabilityState&) = delete;
  PresentationAvailabilityState& operator=(
      const PresentationAvailabilityState&) = delete;

  ~PresentationAvailabilityState();

  // Requests availability for the given URLs and resolves the Promise
  // maintained by `availability` when the availability is known, or rejects
  // the Promise if the availability cannot be determined.
  void RequestAvailability(PresentationAvailability* availability);

  // Starts/stops listening for availability with the given observer.
  void AddObserver(PresentationAvailabilityObserver*);
  void RemoveObserver(PresentationAvailabilityObserver*);

  // Updates the availability value for a given URL, and invoking any affected
  // callbacks and observers.
  void UpdateAvailability(const KURL&, mojom::blink::ScreenAvailability);

  // Returns AVAILABLE if any url in |urls| has screen availability AVAILABLE;
  // otherwise returns DISABLED if at least one url in |urls| has screen
  // availability DISABLED;
  // otherwise, returns SOURCE_NOT_SUPPORTED if any url in |urls| has screen
  // availability SOURCE_NOT_SUPPORTED;
  // otherwise, returns UNAVAILABLE if any url in |urls| has screen
  // availability UNAVAILABLE;
  // otherwise returns UNKNOWN.
  mojom::blink::ScreenAvailability GetScreenAvailability(
      const Vector<KURL>&) const;

  void Trace(Visitor*) const;

 private:
  enum class ListeningState {
    kInactive,
    kWaiting,
    kActive,
  };

  // Tracks listeners of presentation displays availability for
  // |availability_urls|. Shared with PresentationRequest objects with the same
  // set of URLs.
  class AvailabilityListener final
      : public GarbageCollected<AvailabilityListener> {
   public:
    explicit AvailabilityListener(const Vector<KURL>& availability_urls);

    AvailabilityListener(const AvailabilityListener&) = delete;
    AvailabilityListener& operator=(const AvailabilityListener&) = delete;

    ~AvailabilityListener();

    const WTF::Vector<KURL> urls;
    HeapHashSet<WeakMember<PresentationAvailability>> availabilities;
    HeapVector<Member<PresentationAvailabilityObserver>> availability_observers;

    void Trace(Visitor*) const;
  };

  // Tracks listening status and screen availability of |availability_url|.
  struct ListeningStatus {
    explicit ListeningStatus(const KURL& availability_url);
    ~ListeningStatus();

    const KURL url;
    mojom::blink::ScreenAvailability last_known_availability;
    ListeningState listening_state;
  };

  // Starts listening for availability for the given URL, and calls
  // PresentationService if needed.
  void StartListeningToURL(const KURL&);

  // Stops listening for availability for the given URL if there are no
  // remaining callbacks or observers registered to it, and calls
  // PresentationService if needed.
  void MaybeStopListeningToURL(const KURL&);

  // Returns nullptr if there is no AvailabilityListener for the given URLs.
  AvailabilityListener* GetAvailabilityListener(const Vector<KURL>&);

  // Removes the given listener from |availability_set_| if it has no callbacks
  // and no observers.
  void TryRemoveAvailabilityListener(AvailabilityListener*);

  // Returns nullptr if there is no status for the given URL.
  ListeningStatus* GetListeningStatus(const KURL&) const;

  // ListeningStatus for known URLs.
  WTF::Vector<std::unique_ptr<ListeningStatus>> availability_listening_status_;

  // Set of AvailabilityListener for known PresentationRequests.
  HeapVector<Member<AvailabilityListener>> availability_listeners_;

  // A pointer to PresentationService owned by PresentationController.
  const raw_ptr<mojom::blink::PresentationService, DanglingUntriaged>
      presentation_service_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PRESENTATION_PRESENTATION_AVAILABILITY_STATE_H_
