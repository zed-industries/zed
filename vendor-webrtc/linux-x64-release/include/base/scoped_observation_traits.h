// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_SCOPED_OBSERVATION_TRAITS_H_
#define BASE_SCOPED_OBSERVATION_TRAITS_H_

namespace base {

// `ScopedObservationTraits` is used to control the behavior of
// `ScopedObservation` on sources without AddObserver()/RemoveObserver()
// methods.
//
// The implementation of `ScopedObservation<Source, Observer>` will look up the
// most specialized version of `ScopedObservationTraits<Source, Observer>` and
// use the corresponding `Traits::AddObserver` and `Traits::RemoveObserver`.
//
// The default specialization takes care of any Source that exposes
// `AddObserver(Observer*)` and `RemoveObserver(Observer*)` methods -- if that's
// the case, then `ScopedObservation<Source, Observer>` will work out of the
// box.
//
// However, if your `CustomSource` features custom method names -- say,
// `AddFoo(FooObserver*)` and `RemoveFoo(FooObserver*)`, then you'll have to
// define a new traits specialization like this:
//
// `custom_source.h`:
//    #include "base/scoped_observation_traits.h"
//
//    class FooObserver;
//    class CustomSource {
//     public:
//      void AddFoo(FooObserver*);
//      void RemoveFoo(FooObserver*);
//    };
//
//    namespace base {
//
//    template<>
//    struct ScopedObservationTraits<CustomSource, FooObserver> {
//      static void AddObserver(CustomSource* source,
//                              FooObserver* observer) {
//        source->AddFoo(observer);
//      }
//      static void RemoveObserver(CustomSource* source,
//                                 FooObserver* observer) {
//        source->RemoveFoo(observer);
//      }
//    };
//
//    }  // namespace base
//
// `some_important_file.cc`:
//    // Now this works out of the box.
//    base::ScopedObservation<CustomSource, FooObserver> obs...
//

template <class Source, class Observer>
struct ScopedObservationTraits {
  static_assert(
      requires(Source& source, Observer* observer) {
        source.AddObserver(observer);
        source.RemoveObserver(observer);
      },
      "The given Source is missing "
      "AddObserver(Observer*) and/or RemoveObserver(Observer*) "
      "methods. Please provide a custom specialization of "
      "ScopedObservationTraits<> for this Source/Observer pair.");

  static void AddObserver(Source* source, Observer* observer) {
    source->AddObserver(observer);
  }
  static void RemoveObserver(Source* source, Observer* observer) {
    source->RemoveObserver(observer);
  }
};

}  // namespace base

#endif  // BASE_SCOPED_OBSERVATION_TRAITS_H_
