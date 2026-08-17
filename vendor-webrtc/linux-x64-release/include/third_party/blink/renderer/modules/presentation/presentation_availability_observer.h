// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PRESENTATION_PRESENTATION_AVAILABILITY_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PRESENTATION_PRESENTATION_AVAILABILITY_OBSERVER_H_

#include "third_party/blink/public/mojom/presentation/presentation.mojom-blink-forward.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class KURL;

// PresentationAvailabilityObserver is an interface that is implemented by
// objects that wish to be notified when there is a presentation display
// availability change for given URLs.
class PresentationAvailabilityObserver : public GarbageCollectedMixin {
 public:
  virtual ~PresentationAvailabilityObserver() = default;

  virtual void AvailabilityChanged(mojom::blink::ScreenAvailability) = 0;

  virtual const Vector<KURL>& Urls() const = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PRESENTATION_PRESENTATION_AVAILABILITY_OBSERVER_H_
