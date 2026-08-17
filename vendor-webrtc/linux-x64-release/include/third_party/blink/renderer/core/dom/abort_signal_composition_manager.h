// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ABORT_SIGNAL_COMPOSITION_MANAGER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ABORT_SIGNAL_COMPOSITION_MANAGER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/abort_signal_composition_type.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_linked_hash_set.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class AbortSignal;

// `AbortSignalCompositionManager` maintains the relationships between source
// and dependent signals for AbortSignal.any() and TaskSignal.any(). The abort
// and priority components of a signal can be propagated separately and with
// different relationships, but the logic for maintaining the relationships is
// the same.
//
// There are two types of `AbortSignalManager`: one for source signals and one
// for dependents. New sources cannot be added to dependent signals after signal
// construction. When constructing a new composite signal that depends on
// another composite signal, this means the new signal can depend on the other
// composite signal's sources rather than directly on the directly on the
// composite signal itself. We can then represent each signal exclusively as a
// source or dependent, with composite signals being dependents and
// non-composite signals being sources.
//
// A signal is "settled" for a given event type (abort or prioritychange) when
// it can no longer emit that event, e.g. after abort or if a signal's
// controller is GCed. When all of a composite signal's sources are settled, it
// can be settled as well.
//
// Source signals are stored weakly and can be either associated with a
// controller or timeout. Sources are removed when they're settled.
//
// Dependent signals are stored weakly, with `AbortSignalRegistry` used to store
// strong references when needed. This, along with detecting settled signals,
// ensures we only hold strong references to signals when necessary.
class CORE_EXPORT AbortSignalCompositionManager
    : public GarbageCollected<AbortSignalCompositionManager> {
 public:
  AbortSignalCompositionManager(AbortSignal&, AbortSignalCompositionType);
  virtual ~AbortSignalCompositionManager();

  AbortSignalCompositionManager(const AbortSignalCompositionManager&) = delete;
  AbortSignalCompositionManager& operator=(
      const AbortSignalCompositionManager&) = delete;

  virtual void Trace(Visitor*) const;

  // Settle `signal_`. This can be called by the signal or composition manager.
  virtual void Settle();

  // Returns true if `signal_` is settled for `composition_type_`.
  bool IsSettled() const { return is_settled_; }

  // Used for casting.
  virtual bool IsSourceSignalManager() const { return false; }
  virtual bool IsDependentSignalManager() const { return false; }

  AbortSignal& GetSignal() { return *signal_.Get(); }

 protected:
  AbortSignalCompositionType GetCompositionType() const {
    return composition_type_;
  }

 private:
  Member<AbortSignal> signal_;
  AbortSignalCompositionType composition_type_;
  bool is_settled_ = false;
};

class DependentSignalCompositionManager;

// Manages composition for an `AbortSignal` that is a source for dependent
// signals.
class CORE_EXPORT SourceSignalCompositionManager
    : public AbortSignalCompositionManager {
 public:
  SourceSignalCompositionManager(AbortSignal&, AbortSignalCompositionType);
  ~SourceSignalCompositionManager() override;
  SourceSignalCompositionManager(const SourceSignalCompositionManager&) =
      delete;
  SourceSignalCompositionManager& operator=(
      const SourceSignalCompositionManager&) = delete;

  void Trace(Visitor*) const override;
  void Settle() override;

  bool IsSourceSignalManager() const override { return true; }

  void AddDependentSignal(DependentSignalCompositionManager&);

  const HeapLinkedHashSet<WeakMember<AbortSignal>>& GetDependentSignals() {
    return dependent_signals_;
  }

 private:
  HeapLinkedHashSet<WeakMember<AbortSignal>> dependent_signals_;
};

// Manages composition for an `AbortSignal` that is dependent on zero or more
// source signals.
class CORE_EXPORT DependentSignalCompositionManager
    : public AbortSignalCompositionManager {
 public:
  DependentSignalCompositionManager(
      AbortSignal&,
      AbortSignalCompositionType,
      const HeapVector<Member<AbortSignal>>& source_signals);
  ~DependentSignalCompositionManager() override;
  DependentSignalCompositionManager(const DependentSignalCompositionManager&) =
      delete;
  DependentSignalCompositionManager& operator=(
      const DependentSignalCompositionManager&) = delete;

  void Trace(Visitor*) const override;
  void Settle() override;

  bool IsDependentSignalManager() const override { return true; }

  const HeapLinkedHashSet<WeakMember<AbortSignal>>& GetSourceSignals() {
    return source_signals_;
  }

  // Callback invoked on this signal when `source` is settled.
  void OnSourceSettled(SourceSignalCompositionManager& source);

 private:
  void AddSourceSignal(AbortSignal&);

  HeapLinkedHashSet<WeakMember<AbortSignal>> source_signals_;
};

template <>
struct DowncastTraits<DependentSignalCompositionManager> {
  static bool AllowFrom(const AbortSignalCompositionManager& manager) {
    return manager.IsDependentSignalManager();
  }
};

template <>
struct DowncastTraits<SourceSignalCompositionManager> {
  static bool AllowFrom(const AbortSignalCompositionManager& manager) {
    return manager.IsSourceSignalManager();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ABORT_SIGNAL_COMPOSITION_MANAGER_H_
