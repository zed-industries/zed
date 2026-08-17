// This file defines a typed Packet type. It fully interoperates with the older
// mediapipe::Packet; creating an api::Packet<T> that refers to an existing
// Packet (or vice versa) is cheap, just like copying a Packet. Ownership of
// the payload is shared. Consider this as a typed view into the same data.
//
// Conversion is currently done explicitly with the FromOldPacket and
// ToOldPacket functions, but calculator code does not need to concern itself
// with it.

#ifndef MEDIAPIPE_FRAMEWORK_API2_PACKET_H_
#define MEDIAPIPE_FRAMEWORK_API2_PACKET_H_

#include <memory>
#include <string>
#include <type_traits>
#include <utility>

#include "absl/base/attributes.h"
#include "absl/base/optimization.h"
#include "absl/log/absl_check.h"
#include "absl/log/absl_log.h"
#include "absl/meta/type_traits.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/str_cat.h"
#include "mediapipe/framework/api2/tuple.h"
#include "mediapipe/framework/legacy_calculator_support.h"
#include "mediapipe/framework/packet.h"
#include "mediapipe/framework/port/logging.h"
#include "mediapipe/framework/port/status_macros.h"
#include "mediapipe/framework/timestamp.h"

namespace mediapipe {
namespace api2 {

using Timestamp = mediapipe::Timestamp;
using HolderBase = mediapipe::packet_internal::HolderBase;

template <typename T>
class Packet;

struct AnyType {
  AnyType() = delete;
};

// Type-erased packet.
class PacketBase {
 public:
  // Empty.
  PacketBase() = default;
  // Copy.
  PacketBase(const PacketBase&) = default;
  PacketBase& operator=(const PacketBase&) = default;
  // Move.
  PacketBase(PacketBase&&) = default;
  PacketBase& operator=(PacketBase&&) = default;

  // Get timestamp.
  Timestamp timestamp() const { return timestamp_; }
  // The original API has a Timestamp method, but it shadows the Timestamp
  // type within this class, which is annoying.
  // Timestamp Timestamp() const { return timestamp_; }

  PacketBase At(Timestamp timestamp) const&;
  PacketBase At(Timestamp timestamp) &&;

  bool IsEmpty() const { return payload_ == nullptr; }

  template <typename T>
  Packet<T> As() const;

  template <typename T>
  bool Has() const {
    return payload_->As<T>() != nullptr;
  }

  // Returns the reference to the object of type T if it contains
  // one, crashes otherwise.
  template <typename T>
  const T& Get() const;

  // Return OK if the packet is not empty and holds and object of the given
  // type.
  template <typename T>
  absl::Status ValidateAsType() const;

  // Returns the shared pointer to the object of type T if it contains
  // one, an error otherwise.
  template <typename T>
  absl::StatusOr<std::shared_ptr<const T>> Share() const;

  // Conversion to old Packet type.
  operator mediapipe::Packet() const& { return ToOldPacket(*this); }
  operator mediapipe::Packet() && { return ToOldPacket(std::move(*this)); }

  // DEPRECATED
  //
  // Note: Consume is included for compatibility with the old Packet; however,
  // it relies on shared_ptr.use_count(), which is deprecated and is not
  // guaranteed to give exact results.
  template <typename T>
  ABSL_DEPRECATED(
      "Avoid Consume* functions usage as in most cases it's hard to ensure "
      "the proper usage (taken the nature of calculators not knowing where "
      "packets are received from and sent to) and leads to races. Consider "
      "SharedPtrWithPacket instead to get a shared_ptr<T> if applicable.")
  absl::StatusOr<std::unique_ptr<T>> Consume() {
    // Using the implementation in the old Packet for now.
    mediapipe::Packet old =
        packet_internal::Create(std::move(payload_), timestamp_);
    auto result = old.Consume<T>();
    if (!result.ok())
      payload_ = packet_internal::GetHolderShared(std::move(old));
    return result;
  }

 protected:
  explicit PacketBase(std::shared_ptr<const HolderBase> payload)
      : payload_(std::move(payload)) {}

  std::shared_ptr<const HolderBase> payload_;
  Timestamp timestamp_;

  template <typename T>
  friend PacketBase PacketBaseAdopting(const T* ptr);
  friend PacketBase FromOldPacket(const mediapipe::Packet& op);
  friend PacketBase FromOldPacket(mediapipe::Packet&& op);
  friend mediapipe::Packet ToOldPacket(const PacketBase& p);
  friend mediapipe::Packet ToOldPacket(PacketBase&& p);
};

PacketBase FromOldPacket(const mediapipe::Packet& op);
PacketBase FromOldPacket(mediapipe::Packet&& op);
mediapipe::Packet ToOldPacket(const PacketBase& p);
mediapipe::Packet ToOldPacket(PacketBase&& p);

template <typename T>
inline const T& PacketBase::Get() const {
  ABSL_CHECK(payload_);
  const packet_internal::Holder<T>* typed_payload = payload_->As<T>();
  ABSL_CHECK(typed_payload) << absl::StrCat(
      "The Packet stores \"", payload_->DebugTypeName(), "\", but \"",
      MediaPipeTypeStringOrDemangled<T>(), "\" was requested.");
  return typed_payload->data();
}

template <class T>
absl::Status PacketBase::ValidateAsType() const {
  if (ABSL_PREDICT_FALSE(payload_ == nullptr)) {
    return absl::FailedPreconditionError("Empty Packet");
  }
  const packet_internal::Holder<T>* const typed_payload = payload_->As<T>();
  if (ABSL_PREDICT_FALSE(typed_payload == nullptr)) {
    return absl::InvalidArgumentError(absl::StrCat(
        "The Packet stores \"", payload_->DebugTypeName(), "\", but \"",
        MediaPipeTypeStringOrDemangled<T>(), "\" was requested"));
  }
  return absl::OkStatus();
}

template <class T>
absl::StatusOr<std::shared_ptr<const T>> PacketBase::Share() const {
  MP_RETURN_IF_ERROR(ValidateAsType<T>());
  const T* ptr = &Get<T>();
  return std::shared_ptr<const T>(
      ptr, [packet = *this](const T* ptr) mutable { packet = {}; });
}

// This is used to indicate that the packet could be holding one of a set of
// types, e.g. Packet<OneOf<A, B>>.
//
// A Packet<OneOf<T...>> has an interface similar to std::variant<T...>.
// However, we cannot use std::variant directly, since it requires that the
// contained object be stored in place within the variant.
// Suppose we have a stream that accepts an Image or an ImageFrame, and it
// receives a Packet<ImageFrame>. To present it as a
// std::variant<Image, ImageFrame> we would have to move the ImageFrame into
// the variant (or copy it), but that is not compatible with Packet's existing
// ownership model.
// We could have Get() return a std::variant<std::reference_wrapper<Image>,
// std::reference_wrapper<ImageFrame>>, but that would just make user code more
// convoluted.
//
// TODO: should we just use Packet<T...>?
template <class... T>
struct OneOf {};

namespace internal {

template <class T>
inline void CheckCompatibleType(const HolderBase& holder, internal::Wrap<T>) {
  const packet_internal::Holder<T>* typed_payload = holder.As<T>();
  ABSL_CHECK(typed_payload) << absl::StrCat(
      "The Packet stores \"", holder.DebugTypeName(), "\", but \"",
      MediaPipeTypeStringOrDemangled<T>(), "\" was requested.");
  //  ABSL_CHECK(payload_->has_type<T>());
}

template <class... T>
inline void CheckCompatibleType(const HolderBase& holder,
                                internal::Wrap<OneOf<T...>>) {
  bool compatible = (holder.As<T>() || ...);
  ABSL_CHECK(compatible)
      << "The Packet stores \"" << holder.DebugTypeName() << "\", but one of "
      << absl::StrJoin(
             {absl::StrCat("\"", MediaPipeTypeStringOrDemangled<T>(), "\"")...},
             ", ")
      << " was requested.";
}

// TODO: remove usage of internal::Generic and simply use AnyType.
using Generic = ::mediapipe::api2::AnyType;

template <class V, class U>
struct IsCompatibleType : std::false_type {};
template <class V>
struct IsCompatibleType<V, V> : std::true_type {};
template <class V>
struct IsCompatibleType<V, internal::Generic> : std::true_type {};
template <class V, class... U>
struct IsCompatibleType<V, OneOf<U...>>
    : std::integral_constant<bool, (std::is_same_v<V, U> || ...)> {};

}  // namespace internal

template <typename T>
inline Packet<T> PacketBase::As() const {
  if (!payload_) return Packet<T>().At(timestamp_);
  internal::CheckCompatibleType(*payload_, internal::Wrap<T>{});
  return Packet<T>(payload_).At(timestamp_);
}

template <>
inline Packet<internal::Generic> PacketBase::As<internal::Generic>() const;

template <typename T = internal::Generic>
class Packet;
#if __cplusplus >= 201703L
// Deduction guide to silence -Wctad-maybe-unsupported.
explicit Packet() -> Packet<internal::Generic>;
#endif  // C++17

template <>
class Packet<internal::Generic> : public PacketBase {
 public:
  Packet() = default;

  Packet<internal::Generic> At(Timestamp timestamp) const&;
  Packet<internal::Generic> At(Timestamp timestamp) &&;

 protected:
  explicit Packet(std::shared_ptr<const HolderBase> payload)
      : PacketBase(std::move(payload)) {}

  friend PacketBase;
};

// Having Packet<T> subclass Packet<Generic> will require hiding some methods
// like As. May be better not to subclass, and allow implicit conversion
// instead.
template <typename T>
class Packet : public Packet<internal::Generic> {
 public:
  Packet() = default;

  Packet<T> At(Timestamp timestamp) const&;
  Packet<T> At(Timestamp timestamp) &&;

  const T& Get() const {
    if (!payload_) {
      // TODO - Remove this check once stack trace symbolization
      // works on Android non-apk execution.
      const CalculatorContext* calculator_context =
          LegacyCalculatorSupport::Scoped<CalculatorContext>::current();
      if (calculator_context) {
        ABSL_LOG(FATAL) << absl::StrCat("Get() called for type ",
                                        MediaPipeTypeStringOrDemangled<T>(),
                                        " on empty packet during execution of ",
                                        calculator_context->NodeName(), ".");
      }
      ABSL_LOG(FATAL) << absl::StrCat("Get() called for type ",
                                      MediaPipeTypeStringOrDemangled<T>(),
                                      " on empty packet.");
    }
    const packet_internal::Holder<T>* typed_payload = payload_->As<T>();
    ABSL_CHECK(typed_payload);
    return typed_payload->data();
  }
  const T& operator*() const { return Get(); }
  const T* operator->() const { return &Get(); }

  template <typename U, typename TT = T>
  std::enable_if_t<!std::is_abstract_v<TT>, TT> GetOr(U&& v) const {
    return IsEmpty() ? static_cast<T>(std::forward<U>(v)) : **this;
  }

  // DEPRECATED
  //
  // Note: Consume is included for compatibility with the old Packet; however,
  // it relies on shared_ptr.unique(), which is deprecated and is not guaranteed
  // to give exact results.
  ABSL_DEPRECATED(
      "Avoid Consume* functions usage as in most cases it's hard to ensure "
      "the proper usage (taken the nature of calculators not knowing where "
      "packets are received from and sent to) and leads to races. Consider "
      "SharedPtrWithPacket instead to get a shared_ptr<T> if applicable.")
  absl::StatusOr<std::unique_ptr<T>> Consume() {
    return PacketBase::Consume<T>();
  }

  absl::StatusOr<std::shared_ptr<const T>> Share() const {
    return PacketBase::Share<T>();
  }

 private:
  explicit Packet(std::shared_ptr<const HolderBase> payload)
      : Packet<internal::Generic>(std::move(payload)) {}

  friend PacketBase;
  template <typename U, typename... Args>
  friend Packet<U> MakePacket(Args&&... args);
  template <typename U>
  friend Packet<U> PacketAdopting(const U* ptr);
  template <typename U>
  friend Packet<U> PacketAdopting(std::unique_ptr<U> ptr);
};

namespace internal {
template <class... F>
struct Overload : F... {
  using F::operator()...;
};
template <class... F>
explicit Overload(F...) -> Overload<F...>;

template <class T, class... U>
struct First {
  using type = T;
};

template <class T>
struct AddStatus {
  using type = absl::StatusOr<T>;
};
template <class T>
struct AddStatus<absl::StatusOr<T>> {
  using type = absl::StatusOr<T>;
};
template <>
struct AddStatus<absl::Status> {
  using type = absl::Status;
};
template <>
struct AddStatus<void> {
  using type = absl::Status;
};

template <class R, class F, class... A>
struct CallAndAddStatusImpl {
  typename AddStatus<R>::type operator()(const F& f, A&&... a) {
    return f(std::forward<A>(a)...);
  }
};
template <class F, class... A>
struct CallAndAddStatusImpl<void, F, A...> {
  absl::Status operator()(const F& f, A&&... a) {
    f(std::forward<A>(a)...);
    return {};
  }
};

template <class F, class... A>
auto CallAndAddStatus(const F& f, A&&... a) {
  return CallAndAddStatusImpl<absl::result_of_t<F(A...)>, F, A...>()(
      f, std::forward<A>(a)...);
}

}  // namespace internal

template <class... T>
class Packet<OneOf<T...>> : public PacketBase {
 public:
  Packet() = default;

  template <class U>
  using AllowedType = std::enable_if_t<(std::is_same_v<U, T> || ...)>;

  template <class U, class = AllowedType<U>>
  Packet(const Packet<U>& p) : PacketBase(p) {}
  template <class U, class = AllowedType<U>>
  Packet<OneOf<T...>>& operator=(const Packet<U>& p) {
    PacketBase::operator=(p);
    return *this;
  }

  template <class U, class = AllowedType<U>>
  Packet(Packet<U>&& p) : PacketBase(std::move(p)) {}
  template <class U, class = AllowedType<U>>
  Packet<OneOf<T...>>& operator=(Packet<U>&& p) {
    PacketBase::operator=(std::move(p));
    return *this;
  }

  Packet<OneOf<T...>> At(Timestamp timestamp) const& {
    return Packet<OneOf<T...>>(*this).At(timestamp);
  }
  Packet<OneOf<T...>> At(Timestamp timestamp) && {
    timestamp_ = timestamp;
    return std::move(*this);
  }

  template <class U, class = AllowedType<U>>
  const U& Get() const {
    ABSL_CHECK(payload_);
    const packet_internal::Holder<U>* typed_payload = payload_->As<U>();
    ABSL_CHECK(typed_payload);
    return typed_payload->data();
  }

  template <class U, class = AllowedType<U>>
  absl::StatusOr<std::shared_ptr<const U>> Share() const {
    return PacketBase::Share<U>();
  }

  template <class U, class = AllowedType<U>>
  bool Has() const {
    return payload_ && payload_->As<U>();
  }

  template <class... F>
  auto Visit(const F&... args) const {
    ABSL_CHECK(payload_);
    auto f = internal::Overload{args...};
    using FirstT = typename internal::First<T...>::type;
    using ResultType = absl::result_of_t<decltype(f)(const FirstT&)>;
    static_assert(
        (std::is_same_v<ResultType, absl::result_of_t<decltype(f)(const T&)>> &&
         ...),
        "All visitor overloads must have the same return type");
    return Invoke<decltype(f), T...>(f);
  }

  // DEPRECATED
  //
  // Note: Consume is included for compatibility with the old Packet; however,
  // it relies on shared_ptr.unique(), which is deprecated and is not guaranteed
  // to give exact results.
  template <class U, class = AllowedType<U>>
  ABSL_DEPRECATED(
      "Avoid Consume* functions usage as in most cases it's hard to ensure "
      "the proper usage (taken the nature of calculators not knowing where "
      "packets are received from and sent to) and leads to races. Consider "
      "SharedPtrWithPacket instead to get a shared_ptr<T> if applicable.")
  absl::StatusOr<std::unique_ptr<U>> Consume() {
    return PacketBase::Consume<U>();
  }

  template <class... F>
  ABSL_DEPRECATED(
      "Avoid Consume* functions usage as in most cases it's hard to ensure "
      "the proper usage (taken the nature of calculators not knowing where "
      "packets are received from and sent to) and leads to races. Consider "
      "SharedPtrWithPacket instead to get a shared_ptr<T> if applicable.")
  auto ConsumeAndVisit(const F&... args) {
    ABSL_CHECK(payload_);
    auto f = internal::Overload{args...};
    using FirstT = typename internal::First<T...>::type;
    using VisitorResultType =
        absl::result_of_t<decltype(f)(std::unique_ptr<FirstT>)>;
    static_assert(
        (std::is_same_v<VisitorResultType,
                        absl::result_of_t<decltype(f)(std::unique_ptr<T>)>> &&
         ...),
        "All visitor overloads must have the same return type");
    using ResultType = typename internal::AddStatus<VisitorResultType>::type;
    return InvokeConsuming<ResultType, decltype(f), T...>(f);
  }

 protected:
  explicit Packet(std::shared_ptr<const HolderBase> payload)
      : PacketBase(std::move(payload)) {}

  friend PacketBase;

 private:
  template <class F, class U>
  auto Invoke(const F& f) const {
    return f(Get<U>());
  }

  template <class F, class U, class V, class... W>
  auto Invoke(const F& f) const {
    return Has<U>() ? f(Get<U>()) : Invoke<F, V, W...>(f);
  }

  template <class R, class F, class U>
  auto InvokeConsuming(const F& f) -> R {
    auto maybe_value = Consume<U>();
    if (maybe_value.ok())
      return internal::CallAndAddStatus(f, std::move(maybe_value).value());
    else
      return maybe_value.status();
  }

  template <class R, class F, class U, class V, class... W>
  auto InvokeConsuming(const F& f) -> R {
    return Has<U>() ? InvokeConsuming<R, F, U>(f)
                    : InvokeConsuming<R, F, V, W...>(f);
  }
};

template <>
inline Packet<internal::Generic> PacketBase::As<internal::Generic>() const {
  if (!payload_) return Packet<internal::Generic>().At(timestamp_);
  return Packet<internal::Generic>(payload_).At(timestamp_);
}

inline PacketBase PacketBase::At(Timestamp timestamp) const& {
  return PacketBase(*this).At(timestamp);
}

inline PacketBase PacketBase::At(Timestamp timestamp) && {
  timestamp_ = timestamp;
  return std::move(*this);
}

template <typename T>
inline Packet<T> Packet<T>::At(Timestamp timestamp) const& {
  return Packet<T>(*this).At(timestamp);
}

template <typename T>
inline Packet<T> Packet<T>::At(Timestamp timestamp) && {
  timestamp_ = timestamp;
  return std::move(*this);
}

inline Packet<internal::Generic> Packet<internal::Generic>::At(
    Timestamp timestamp) const& {
  return Packet<internal::Generic>(*this).At(timestamp);
}

inline Packet<internal::Generic> Packet<internal::Generic>::At(
    Timestamp timestamp) && {
  timestamp_ = timestamp;
  return std::move(*this);
}

template <typename T, typename... Args>
Packet<T> MakePacket(Args&&... args) {
  return Packet<T>(std::make_shared<packet_internal::Holder<T>>(
      new T(std::forward<Args>(args)...)));
}

template <typename T>
Packet<T> PacketAdopting(const T* ptr) {
  return Packet<T>(std::make_shared<packet_internal::Holder<T>>(ptr));
}

template <typename T>
Packet<T> PacketAdopting(std::unique_ptr<T> ptr) {
  return Packet<T>(std::make_shared<packet_internal::Holder<T>>(ptr.release()));
}

}  // namespace api2
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_API2_PACKET_H_
