// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_SCOPED_MULTI_SOURCE_OBSERVATION_H_
#define BASE_SCOPED_MULTI_SOURCE_OBSERVATION_H_

#include <stddef.h>

#include <algorithm>
#include <vector>

#include "base/check.h"
#include "base/containers/contains.h"
#include "base/memory/raw_ptr.h"
#include "base/scoped_observation_traits.h"

namespace base {

// ScopedMultiSourceObservation is used to keep track of plural observation,
// e.g. where an observer observes more than a single source.
//
// Use base::ScopedObservation for objects that observe only a single source.
//
// When ScopedMultiSourceObservation is destroyed, it removes the object as an
// observer from all sources it has been added to.
// Basic example (as a member variable):
//
//   class MyFooObserver : public FooObserver {
//     ...
//    private:
//     ScopedMultiSourceObservation<Foo, FooObserver> foo_observations_{this};
//   };
//
//   MyFooObserver::OnFooCreated(Foo* foo) {
//     foo_observations_.AddObservation(foo);
//   }
//
////////////////////////////////////////////////////////////////////////////////
//
// By default `ScopedMultiSourceObservation` only works with sources that expose
// `AddObserver` and `RemoveObserver`. However, it's also possible to
// adapt it to custom function names (say `AddFoo` and `RemoveFoo` accordingly)
// by tailoring ScopedObservationTraits<> for the given Source and Observer --
// see `base/scoped_observation_traits.h` for details.
//

template <class Source, class Observer>
class ScopedMultiSourceObservation {
 public:
  explicit ScopedMultiSourceObservation(Observer* observer)
      : observer_(observer) {}
  ScopedMultiSourceObservation(const ScopedMultiSourceObservation&) = delete;
  ScopedMultiSourceObservation& operator=(const ScopedMultiSourceObservation&) =
      delete;
  ~ScopedMultiSourceObservation() { RemoveAllObservations(); }

  // Adds the object passed to the constructor as an observer on |source|.
  void AddObservation(Source* source) {
    CHECK(!IsObservingSource(source));
    sources_.push_back(source);
    Traits::AddObserver(source, observer_);
  }

  // Remove the object passed to the constructor as an observer from |source|.
  void RemoveObservation(Source* source) {
    auto it = std::ranges::find(sources_, source);
    CHECK(it != sources_.end());
    sources_.erase(it);
    Traits::RemoveObserver(source, observer_);
  }

  // Remove the object passed to the constructor as an observer from all sources
  // it's observing.
  void RemoveAllObservations() {
    for (Source* source : sources_) {
      Traits::RemoveObserver(source, observer_);
    }
    sources_.clear();
  }

  // Returns true if any source is being observed.
  bool IsObservingAnySource() const { return !sources_.empty(); }

  // Returns true if |source| is being observed.
  bool IsObservingSource(Source* source) const {
    DCHECK(source);
    return base::Contains(sources_, source);
  }

  // Returns the number of sources being observed.
  size_t GetSourcesCount() const { return sources_.size(); }

  // Returns a pointer to the observer that observes the sources.
  Observer* observer() { return observer_; }
  const Observer* observer() const { return observer_; }

  // Returns the sources being observed. Note: It is invalid to add or remove
  // sources while iterating on it.
  const std::vector<raw_ptr<Source>>& sources() const { return sources_; }

 private:
  using Traits = ScopedObservationTraits<Source, Observer>;

  const raw_ptr<Observer> observer_;

  std::vector<raw_ptr<Source>> sources_;
};

}  // namespace base

#endif  // BASE_SCOPED_MULTI_SOURCE_OBSERVATION_H_
