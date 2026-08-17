// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_COMPUTE_PRESSURE_PRESSURE_CLIENT_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_COMPUTE_PRESSURE_PRESSURE_CLIENT_IMPL_H_

#include "base/time/time.h"
#include "mojo/public/cpp/bindings/associated_receiver.h"
#include "services/device/public/mojom/pressure_manager.mojom-blink-forward.h"
#include "services/device/public/mojom/pressure_update.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/dom/dom_high_res_time_stamp.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/compute_pressure/pressure_observer.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_associated_receiver.h"

namespace blink {

class ExecutionContext;
class PressureObserverManager;

// This class implements the "device::mojom::blink::PressureClient"
// interface to receive "device::mojom::blink::PressureUpdate" from
// "content::PressureClientImpl" and broadcasts the information to active
// PressureObservers.
//
// This class keeps track of State and active PressureObservers per source type.
class MODULES_EXPORT PressureClientImpl final
    : public GarbageCollected<PressureClientImpl>,
      public ExecutionContextClient,
      public device::mojom::blink::PressureClient {
 public:
  // kUninitialized: receiver_ is not bound.
  // kInitializing: receiver_ is bound but the remote side is not bound.
  // kInitialized: receiver_ and the remote side are both bound.
  enum class State { kUninitialized, kInitializing, kInitialized };

  PressureClientImpl(ExecutionContext*, PressureObserverManager*);
  ~PressureClientImpl() override;

  PressureClientImpl(const PressureClientImpl&) = delete;
  PressureClientImpl& operator=(const PressureClientImpl&) = delete;

  // device::mojom::blink::PressureClient implementation.
  void OnPressureUpdated(device::mojom::blink::PressureUpdatePtr) override;

  State state() const { return state_; }
  void set_state(State state) { state_ = state; }

  void AddObserver(PressureObserver*);
  void RemoveObserver(PressureObserver*);
  const HeapHashSet<Member<PressureObserver>>& observers() const {
    return observers_;
  }

  mojo::PendingAssociatedRemote<device::mojom::blink::PressureClient>
      BindNewEndpointAndPassRemote(scoped_refptr<base::SequencedTaskRunner>);

  void Reset();

  void Trace(Visitor*) const override;

 private:
  DOMHighResTimeStamp CalculateTimestamp(base::TimeTicks timestamp) const;

  WeakMember<PressureObserverManager> manager_;

  HeapMojoAssociatedReceiver<device::mojom::blink::PressureClient,
                             PressureClientImpl>
      associated_receiver_;

  HeapHashSet<Member<PressureObserver>> observers_;

  State state_ = State::kUninitialized;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_COMPUTE_PRESSURE_PRESSURE_CLIENT_IMPL_H_
