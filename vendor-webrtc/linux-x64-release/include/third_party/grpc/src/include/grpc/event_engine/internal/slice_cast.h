// Copyright 2022 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef GRPC_EVENT_ENGINE_INTERNAL_SLICE_CAST_H
#define GRPC_EVENT_ENGINE_INTERNAL_SLICE_CAST_H

namespace grpc_event_engine {
namespace experimental {
namespace internal {

// Opt-in trait class for slice conversions.
// Declare a specialization of this class for any types that are compatible
// with `SliceCast`. Both ways need to be declared (i.e. if
// SliceCastable<A,B> exists, you should declare
// SliceCastable<B,A> too).
// The type has no members, it's just the existence of the specialization that
// unlocks SliceCast usage for a type pair.
template <typename Result, typename T>
struct SliceCastable;

// This is strictly too wide, but consider all types to be SliceCast-able to
// themselves.
// Unfortunately this allows `const int& x = SliceCast<int>(x);` which is kind
// of bogus.
template <typename A>
struct SliceCastable<A, A> {};

// Cast to `const Result&` from `const T&` without any runtime checks.
// This is only valid if `sizeof(Result) == sizeof(T)`, and if `Result`, `T` are
// opted in as compatible via `SliceCastable`.
template <typename Result, typename T>
const Result& SliceCast(const T& value, SliceCastable<Result, T> = {}) {
  // Insist upon sizes being equal to catch mismatches.
  // We assume if sizes are opted in and sizes are equal then yes, these two
  // types are expected to be layout compatible and actually appear to be.
  static_assert(sizeof(Result) == sizeof(T), "size mismatch");
  return reinterpret_cast<const Result&>(value);
}

// Cast to `Result&` from `T&` without any runtime checks.
// This is only valid if `sizeof(Result) == sizeof(T)`, and if `Result`, `T` are
// opted in as compatible via `SliceCastable`.
template <typename Result, typename T>
Result& SliceCast(T& value, SliceCastable<Result, T> = {}) {
  // Insist upon sizes being equal to catch mismatches.
  // We assume if sizes are opted in and sizes are equal then yes, these two
  // types are expected to be layout compatible and actually appear to be.
  static_assert(sizeof(Result) == sizeof(T), "size mismatch");
  return reinterpret_cast<Result&>(value);
}

// Cast to `Result&&` from `T&&` without any runtime checks.
// This is only valid if `sizeof(Result) == sizeof(T)`, and if `Result`, `T` are
// opted in as compatible via `SliceCastable`.
template <typename Result, typename T>
Result&& SliceCast(T&& value, SliceCastable<Result, T> = {}) {
  // Insist upon sizes being equal to catch mismatches.
  // We assume if sizes are opted in and sizes are equal then yes, these two
  // types are expected to be layout compatible and actually appear to be.
  static_assert(sizeof(Result) == sizeof(T), "size mismatch");
  return reinterpret_cast<Result&&>(value);
}

}  // namespace internal
}  // namespace experimental
}  // namespace grpc_event_engine

#endif  // GRPC_EVENT_ENGINE_INTERNAL_SLICE_CAST_H
