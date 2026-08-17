// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ALLOCATOR_DISPATCHER_INITIALIZER_H_
#define BASE_ALLOCATOR_DISPATCHER_INITIALIZER_H_

#include <tuple>
#include <utility>

#include "base/allocator/dispatcher/configuration.h"
#include "base/allocator/dispatcher/dispatcher.h"
#include "base/allocator/dispatcher/internal/tools.h"

namespace base::allocator::dispatcher {
namespace internal {

// Filter the passed observers and perform initialization of the passed
// dispatcher.
template <size_t CurrentIndex,
          typename DispatcherType,
          typename CheckObserverPredicate,
          typename VerifiedObservers,
          typename UnverifiedObservers,
          size_t... IndicesToSelect>
inline void DoInitialize(DispatcherType& dispatcher,
                         CheckObserverPredicate check_observer,
                         const VerifiedObservers& verified_observers,
                         const UnverifiedObservers& unverified_observers,
                         std::index_sequence<IndicesToSelect...> indices) {
  if constexpr (CurrentIndex < std::tuple_size_v<UnverifiedObservers>) {
    // We still have some items left to handle.
    if (check_observer(std::get<CurrentIndex>(unverified_observers))) {
      // The current observer is valid. Hence, append the index of the current
      // item to the set of indices and head on to the next item.
      DoInitialize<CurrentIndex + 1>(
          dispatcher, check_observer, verified_observers, unverified_observers,
          std::index_sequence<IndicesToSelect..., CurrentIndex>{});
    } else {
      // The current observer is not valid. Hence, head on to the next item with
      // an unaltered list of indices.
      DoInitialize<CurrentIndex + 1>(dispatcher, check_observer,
                                     verified_observers, unverified_observers,
                                     indices);
    }
  } else if constexpr (CurrentIndex == std::tuple_size_v<UnverifiedObservers>) {
    // So we have met the end of the tuple of observers to verify.
    // Hence, we extract the additional valid observers, append to the tuple of
    // already verified observers and hand over to the dispatcher.
    auto observers = std::tuple_cat(
        verified_observers,
        std::make_tuple(std::get<IndicesToSelect>(unverified_observers)...));

    // Do a final check that neither the maximum total number of observers nor
    // the maximum number of optional observers is exceeded.
    static_assert(std::tuple_size_v<decltype(observers)> <=
                  configuration::kMaximumNumberOfObservers);
    static_assert(sizeof...(IndicesToSelect) <=
                  configuration::kMaximumNumberOfOptionalObservers);

    dispatcher.Initialize(std::move(observers));
  }
}

}  // namespace internal

// The result of concatenating two tuple-types.
template <typename... tuples>
using TupleCat = decltype(std::tuple_cat(std::declval<tuples>()...));

// Initializer collects mandatory and optional observers and initializes the
// passed Dispatcher with only the enabled observers.
//
// In some situations, presence of observers depends on runtime. i.e. command
// line parameters or CPU features. With 3 optional observers we already have 8
// different combinations. Initializer takes the job of dealing with all
// combinations from the user. It allows users to pass all observers (including
// nullptr for disabled optional observers) and initializes the Dispatcher with
// only the enabled observers.
//
// Since this process results in a combinatoric explosion, Initializer
// distinguishes between optional and mandatory observers. Mandatory observers
// are not included in the filtering process and must always be enabled (not
// nullptr).
//
// To allow the Initializer to track the number and exact type of observers, it
// is implemented as a templated class which holds information on the types in
// the std::tuples passed as template parameters. Therefore, whenever any type
// observer it set, the initializer changes its type to reflect this.
template <typename MandatoryObservers = std::tuple<>,
          typename OptionalObservers = std::tuple<>>
struct BASE_EXPORT Initializer {
  Initializer() = default;
  Initializer(MandatoryObservers mandatory_observers,
              OptionalObservers optional_observers)
      : mandatory_observers_(std::move(mandatory_observers)),
        optional_observers_(std::move(optional_observers)) {}

  // Set the mandatory observers. The number of observers that can be set is
  // limited by configuration::maximum_number_of_observers.
  template <typename... NewMandatoryObservers,
            std::enable_if_t<
                internal::LessEqual((sizeof...(NewMandatoryObservers) +
                                     std::tuple_size_v<OptionalObservers>),
                                    configuration::kMaximumNumberOfObservers),
                bool> = true>
  Initializer<std::tuple<NewMandatoryObservers*...>, OptionalObservers>
  SetMandatoryObservers(NewMandatoryObservers*... mandatory_observers) const {
    return {std::make_tuple(mandatory_observers...), GetOptionalObservers()};
  }

  // Add mandatory observers. The number of observers that can be added is
  // limited by the current number of observers, see
  // configuration::maximum_number_of_observers.
  template <typename... AdditionalMandatoryObservers,
            std::enable_if_t<internal::LessEqual(
                                 std::tuple_size_v<MandatoryObservers> +
                                     sizeof...(AdditionalMandatoryObservers) +
                                     std::tuple_size_v<OptionalObservers>,
                                 configuration::kMaximumNumberOfObservers),
                             bool> = true>
  Initializer<TupleCat<MandatoryObservers,
                       std::tuple<AdditionalMandatoryObservers*...>>,
              OptionalObservers>
  AddMandatoryObservers(
      AdditionalMandatoryObservers*... additional_mandatory_observers) const {
    return {std::tuple_cat(GetMandatoryObservers(),
                           std::make_tuple(additional_mandatory_observers...)),
            GetOptionalObservers()};
  }

  // Set the optional observers. The number of observers that can be set is
  // limited by configuration::maximum_number_of_optional_observers as well as
  // configuration::maximum_number_of_observers.
  template <
      typename... NewOptionalObservers,
      std::enable_if_t<
          internal::LessEqual(
              sizeof...(NewOptionalObservers),
              configuration::kMaximumNumberOfOptionalObservers) &&
              internal::LessEqual((sizeof...(NewOptionalObservers) +
                                   std::tuple_size_v<MandatoryObservers>),
                                  configuration::kMaximumNumberOfObservers),
          bool> = true>
  Initializer<MandatoryObservers, std::tuple<NewOptionalObservers*...>>
  SetOptionalObservers(NewOptionalObservers*... optional_observers) const {
    return {GetMandatoryObservers(), std::make_tuple(optional_observers...)};
  }

  // Add optional observers. The number of observers that can be added is
  // limited by the current number of optional observers,
  // configuration::maximum_number_of_optional_observers as well as
  // configuration::maximum_number_of_observers.
  template <
      typename... AdditionalOptionalObservers,
      std::enable_if_t<
          internal::LessEqual(
              std::tuple_size_v<OptionalObservers> +
                  sizeof...(AdditionalOptionalObservers),
              configuration::kMaximumNumberOfOptionalObservers) &&
              internal::LessEqual((std::tuple_size_v<OptionalObservers> +
                                   sizeof...(AdditionalOptionalObservers) +
                                   std::tuple_size_v<MandatoryObservers>),
                                  configuration::kMaximumNumberOfObservers),
          bool> = true>
  Initializer<
      MandatoryObservers,
      TupleCat<OptionalObservers, std::tuple<AdditionalOptionalObservers*...>>>
  AddOptionalObservers(
      AdditionalOptionalObservers*... additional_optional_observers) const {
    return {GetMandatoryObservers(),
            std::tuple_cat(GetOptionalObservers(),
                           std::make_tuple(additional_optional_observers...))};
  }

  // Perform the actual initialization on the passed dispatcher.
  // The dispatcher is passed as a template only to provide better testability.
  template <typename DispatcherType>
  void DoInitialize(DispatcherType& dispatcher) const {
    internal::DoInitialize<0>(dispatcher, internal::IsValidObserver{},
                              GetMandatoryObservers(), GetOptionalObservers(),
                              {});
  }

  const MandatoryObservers& GetMandatoryObservers() const {
    return mandatory_observers_;
  }

  const OptionalObservers& GetOptionalObservers() const {
    return optional_observers_;
  }

 private:
  MandatoryObservers mandatory_observers_;
  OptionalObservers optional_observers_;
};

// Convenience function for creating an empty Initializer.
inline Initializer<> CreateInitializer() {
  return {};
}

}  // namespace base::allocator::dispatcher

#endif  // BASE_ALLOCATOR_DISPATCHER_INITIALIZER_H_
