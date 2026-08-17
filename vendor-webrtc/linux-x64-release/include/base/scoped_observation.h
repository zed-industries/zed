// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_SCOPED_OBSERVATION_H_
#define BASE_SCOPED_OBSERVATION_H_

#include <utility>

#include "base/check.h"
#include "base/check_op.h"
#include "base/memory/raw_ptr.h"
#include "base/scoped_observation_traits.h"

namespace base {

// `ScopedObservation` is used to keep track of a singular observation, i.e.,
// where an observer observes a single source only. Use
// `base::ScopedMultiSourceObservation` for objects that observe multiple
// sources.
//
// When a `ScopedObservation` is destroyed, it unregisters the observer from the
// observable if it was currently observing something. Otherwise it does
// nothing.
//
// Using a `ScopedObservation` instead of manually observing and unobserving has
// the following benefits:
// - The observer cannot accidentally forget to stop observing when it is
//   destroyed.
// - By calling `Reset`, an ongoing observation can be stopped before the
//   `ScopedObservation` is destroyed. If nothing was currently observed, then
//   calling `Reset` does nothing. This can be useful for when the observable is
//   destroyed before the observer is destroyed, because it prevents the
//   observer from accidentally unregistering itself from the destroyed
//   observable a second time when it itself is destroyed. Without
//   `ScopedObservation`, one might need to keep track of whether one has
//   already stopped observing in a separate boolean.
//
// A complete usage example can be found below.
//
// `observer.h`:
//   class Observer {
//    public:
//     virtual ~Observer() = default;
//
//     virtual void OnEvent() {}
//   };
//
// `source.h`:
//   class Observer;
//   class Source {
//    public:
//     void AddObserver(Observer* observer);
//     void RemoveObserver(Observer* observer);
//   };
//
// `observer_impl.h`:
//   #include "observer.h"
//
//   class Source;
//
//   class ObserverImpl: public Observer {
//    public:
//     ObserverImpl(Source* source);
//     // Note how there is no need to stop observing in the destructor.
//     ~ObserverImpl() override {}
//
//     void OnEvent() override {
//       ...
//     }
//
//    private:
//     // Note that |obs_| can be instantiated with forward-declared Source.
//     base::ScopedObservation<Source, Observer> obs_{this};
//   };
//
// `observer_impl.cc`:
//   #include "observer_impl.h"
//   #include "source.h"
//
//   ObserverImpl::ObserverImpl(Source* source) {
//     // After the call |this| starts listening to events from |source|.
//     obs_.Observe(source);
//   }
//
////////////////////////////////////////////////////////////////////////////////
//
// By default `ScopedObservation` only works with sources that expose
// `AddObserver` and `RemoveObserver`. However, it's also possible to
// adapt it to custom function names (say `AddFoo` and `RemoveFoo` accordingly)
// by tailoring ScopedObservationTraits<> for the given Source and Observer --
// see `base/scoped_observation_traits.h` for details.
//

template <class Source, class Observer>
class ScopedObservation {
 public:
  explicit ScopedObservation(Observer* observer) : observer_(observer) {}
  ScopedObservation(const ScopedObservation&) = delete;
  ScopedObservation& operator=(const ScopedObservation&) = delete;
  ~ScopedObservation() { Reset(); }

  // Adds the object passed to the constructor as an observer on |source|.
  // IsObserving() must be false.
  void Observe(Source* source) {
    DCHECK_EQ(source_, nullptr);
    source_ = source;
    Traits::AddObserver(source_, observer_);
  }

  // Remove the object passed to the constructor as an observer from |source_|
  // if currently observing. Does nothing otherwise.
  void Reset() {
    if (source_) {
      Traits::RemoveObserver(std::exchange(source_, nullptr), observer_);
    }
  }

  // Returns true if any source is being observed.
  bool IsObserving() const { return source_ != nullptr; }

  // Returns true if |source| is being observed.
  bool IsObservingSource(Source* source) const {
    DCHECK(source);
    return source_ == source;
  }

  // Gets a pointer to the observer that observes the source.
  Observer* GetObserver() { return observer_; }
  const Observer* GetObserver() const { return observer_; }

  // Gets a pointer to the observed source, or nullptr if no source is being
  // observed.
  Source* GetSource() { return source_; }
  const Source* GetSource() const { return source_; }

 private:
  using Traits = ScopedObservationTraits<Source, Observer>;

  const raw_ptr<Observer> observer_;

  // The observed source, if any.
  raw_ptr<Source, LeakedDanglingUntriaged> source_ = nullptr;
};

}  // namespace base

#endif  // BASE_SCOPED_OBSERVATION_H_
