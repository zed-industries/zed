// This file defines an API to define a node's ports in a concise, type-safe
// way. Example usage in a node:
//
//   static constexpr Input<int> kBase("IN");
//   static constexpr Output<float> kOut("OUT");
//   static constexpr SideInput<float>::Optional kDelta("DELTA");
//   static constexpr SideOutput<float> kForward("FORWARD");
//
// Pass a CalculatorContext to a port to access the inputs or outputs in the
// context. For example:
//
//   kBase(cc) yields an InputShardAccess<int>
//   kOut(cc) yields an OutputShardAccess<float>
//   kDelta(cc) yields an InputSidePacketAccess<float>
//   kForward(cc) yields an OutputSidePacketAccess<float>

#ifndef MEDIAPIPE_FRAMEWORK_API2_PORT_H_
#define MEDIAPIPE_FRAMEWORK_API2_PORT_H_

#include <type_traits>
#include <utility>

#include "absl/log/absl_check.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/string_view.h"
#include "mediapipe/framework/api2/const_str.h"
#include "mediapipe/framework/api2/packet.h"
#include "mediapipe/framework/calculator_context.h"
#include "mediapipe/framework/calculator_contract.h"
#include "mediapipe/framework/output_side_packet.h"
#include "mediapipe/framework/port/logging.h"
#include "mediapipe/framework/tool/type_util.h"

namespace mediapipe {
namespace api2 {

// This is a base class for various types of port. It is not meant to be used
// directly by node code.
class PortBase {
 public:
  constexpr PortBase(absl::string_view tag, TypeId type_id, bool optional,
                     bool multiple)
      : tag_(tag.size(), tag.data()),
        optional_(optional),
        multiple_(multiple),
        type_id_(type_id) {}

  constexpr PortBase(std::size_t tag_size, const char* tag, TypeId type_id,
                     bool optional, bool multiple)
      : tag_(tag_size, tag),
        optional_(optional),
        multiple_(multiple),
        type_id_(type_id) {}

  bool IsOptional() const { return optional_; }
  bool IsMultiple() const { return multiple_; }
  const char* Tag() const { return tag_.data(); }

  TypeId type_id() const { return type_id_; }

  const const_str tag_;
  const bool optional_;
  const bool multiple_;

 protected:
  TypeId type_id_;
};

// These four base classes are used to distinguish between ports of different
// kinds. They are not meant to be used directly by node code.
class InputBase : public PortBase {
  using PortBase::PortBase;
};
class OutputBase : public PortBase {
  using PortBase::PortBase;
};
class SideInputBase : public PortBase {
  using PortBase::PortBase;
};
class SideOutputBase : public PortBase {
  using PortBase::PortBase;
};

struct NoneType {
  NoneType() = delete;
};

template <auto& kP>
struct SameType {
  static constexpr const decltype(kP) & kPort = kP;
};

class PacketTypeAccess;
class PacketTypeAccessFallback;
template <typename T>
class InputShardAccess;
template <typename T>
class OutputShardAccess;
template <typename T>
class InputSidePacketAccess;
template <typename T>
class OutputSidePacketAccess;
template <typename T>
class InputShardOrSideAccess;

namespace internal {

// Forward declaration for AddToContract friend.
template <typename...>
class Contract;

template <class CC>
auto GetCollection(CC* cc, const InputBase& port) -> decltype(cc->Inputs()) {
  return cc->Inputs();
}

template <class CC>
auto GetCollection(CC* cc, const SideInputBase& port)
    -> decltype(cc->InputSidePackets()) {
  return cc->InputSidePackets();
}

template <class CC>
auto GetCollection(CC* cc, const OutputBase& port) -> decltype(cc->Outputs()) {
  return cc->Outputs();
}

template <class CC>
auto GetCollection(CC* cc, const SideOutputBase& port)
    -> decltype(cc->OutputSidePackets()) {
  return cc->OutputSidePackets();
}

template <class Collection>
auto GetOrNull(Collection& collection, const absl::string_view& tag, int index)
    -> decltype(&collection.Get(std::declval<CollectionItemId>())) {
  CollectionItemId id = collection.GetId(tag, index);
  return id.IsValid() ? &collection.Get(id) : nullptr;
}

template <class T>
struct IsOneOf : std::false_type {};

template <class... T>
struct IsOneOf<OneOf<T...>> : std::true_type {};

template <class T>
struct IsSameType : std::false_type {};

template <class P, P& kP>
struct IsSameType<SameType<kP>> : std::true_type {};

template <typename T,
          typename std::enable_if<!std::is_same<T, AnyType>{} &&
                                      !IsOneOf<T>{} && !IsSameType<T>{},
                                  int>::type = 0>
inline void SetType(CalculatorContract* cc, PacketType& pt) {
  pt.Set<T>();
}

template <typename T, typename std::enable_if<IsSameType<T>{}, int>::type = 0>
inline void SetType(CalculatorContract* cc, PacketType& pt) {
  pt.SetSameAs(&internal::GetCollection(cc, T::kPort).Tag(T::kPort.Tag()));
}

template <typename T,
          typename std::enable_if<std::is_same<T, AnyType>{}, int>::type = 0>
inline void SetType(CalculatorContract* cc, PacketType& pt) {
  pt.SetAny();
}

template <>
inline void SetType<NoneType>(CalculatorContract* cc, PacketType& pt) {
  // This is used for header-only streams. Should it be removed?
  pt.SetNone();
}

template <typename... T>
inline void SetTypeOneOf(OneOf<T...>, CalculatorContract* cc, PacketType& pt) {
  pt.SetOneOf<T...>();
}

template <typename T, typename std::enable_if<IsOneOf<T>{}, int>::type = 0>
inline void SetType(CalculatorContract* cc, PacketType& pt) {
  SetTypeOneOf(T{}, cc, pt);
}

template <typename ValueT>
InputShardAccess<ValueT> SinglePortAccess(mediapipe::CalculatorContext* cc,
                                          InputStreamShard* stream) {
  return InputShardAccess<ValueT>(*cc, stream);
}

template <typename ValueT>
OutputShardAccess<ValueT> SinglePortAccess(mediapipe::CalculatorContext* cc,
                                           OutputStreamShard* stream) {
  return OutputShardAccess<ValueT>(*cc, stream);
}

template <typename ValueT>
InputSidePacketAccess<ValueT> SinglePortAccess(
    mediapipe::CalculatorContext* cc, const mediapipe::Packet* packet) {
  return InputSidePacketAccess<ValueT>(packet);
}

template <typename ValueT>
OutputSidePacketAccess<ValueT> SinglePortAccess(
    mediapipe::CalculatorContext* cc, OutputSidePacket* osp) {
  return OutputSidePacketAccess<ValueT>(osp);
}

template <typename ValueT>
InputShardOrSideAccess<ValueT> SinglePortAccess(
    mediapipe::CalculatorContext* cc, InputStreamShard* stream,
    const mediapipe::Packet* packet) {
  return InputShardOrSideAccess<ValueT>(*cc, stream, packet);
}

template <typename ValueT>
PacketTypeAccess SinglePortAccess(mediapipe::CalculatorContract* cc,
                                  PacketType* pt);

template <typename ValueT>
PacketTypeAccessFallback SinglePortAccess(mediapipe::CalculatorContract* cc,
                                          PacketType* pt, bool is_stream);

template <typename ValueT, typename PortT, class CC>
auto AccessPort(std::false_type, const PortT& port, CC* cc) {
  auto& collection = GetCollection(cc, port);
  return SinglePortAccess<ValueT>(
      cc, internal::GetOrNull(collection, port.Tag(), 0));
}

template <typename ValueT, typename X, typename CollectionT, class CC>
class MultiplePortAccess {
 public:
  using AccessT = decltype(SinglePortAccess<ValueT>(std::declval<CC*>(),
                                                    std::declval<X*>()));

  MultiplePortAccess(CC* cc, CollectionT* collection, const char* tag,
                     int count)
      : cc_(cc), collection_(collection), tag_(tag), count_(count) {}

  // TODO: maybe this should be size(), like in a standard C++
  // container?
  int Count() const { return count_; }
  AccessT operator[](int pos) const {
    ABSL_CHECK_GE(pos, 0);
    ABSL_CHECK_LT(pos, count_);
    return SinglePortAccess<ValueT>(
        cc_, internal::GetOrNull(*collection_, tag_, pos));
  }

  class Iterator {
   public:
    using iterator_category = std::input_iterator_tag;
    using value_type = AccessT;
    using difference_type = std::ptrdiff_t;
    using pointer = AccessT*;
    using reference = AccessT;  // allowed; see e.g. std::istreambuf_iterator

    Iterator(CC* cc, CollectionT* collection, const char* tag, int pos)
        : cc_(cc), collection_(collection), tag_(tag), pos_(pos) {}
    Iterator& operator++() {
      ++pos_;
      return *this;
    }
    Iterator operator++(int) {
      Iterator res = *this;
      ++(*this);
      return res;
    }
    bool operator==(const Iterator& other) const { return pos_ == other.pos_; }
    bool operator!=(const Iterator& other) const { return !(*this == other); }
    AccessT operator*() const {
      return SinglePortAccess<ValueT>(
          cc_, internal::GetOrNull(*collection_, tag_, pos_));
    }

   private:
    CC* cc_;
    CollectionT* collection_;
    const char* tag_;
    int pos_ = 0;
  };

  Iterator begin() const { return Iterator(cc_, collection_, tag_, 0); }
  Iterator end() const { return Iterator(cc_, collection_, tag_, count_); }

 private:
  CC* cc_;
  CollectionT* collection_;
  const char* tag_;
  int count_;
};

template <typename ValueT, typename PortT, class CC>
auto AccessPort(std::true_type, const PortT& port, CC* cc) {
  auto& collection = GetCollection(cc, port);
  auto* first = internal::GetOrNull(collection, port.Tag(), 0);
  using EntryT = typename std::remove_pointer_t<decltype(first)>;
  using CollectionT = typename std::remove_reference_t<
      std::remove_const_t<decltype(collection)>>;
  return MultiplePortAccess<ValueT, EntryT, CollectionT, CC>(
      cc, &collection, port.Tag(), collection.NumEntries(port.Tag()));
}

template <class Base>
struct SideBase;

template <>
struct SideBase<InputBase> {
  using type = SideInputBase;
};

// TODO: maybe return a PacketBase instead of a Packet<internal::Generic>?
template <typename T, typename = void>
struct ActualPayloadType {
  using type = T;
};

template <typename T>
struct ActualPayloadType<T, std::enable_if_t<IsSameType<T>{}, void>> {
  using type = typename ActualPayloadType<
      typename std::decay_t<decltype(T::kPort)>::value_t>::type;
};

}  // namespace internal

// Maps special port value types, such as AnyType, to internal::Generic.
template <typename T>
using ActualPayloadT = typename internal::ActualPayloadType<T>::type;

static_assert(std::is_same_v<ActualPayloadT<int>, int>, "");
static_assert(std::is_same_v<ActualPayloadT<AnyType>, internal::Generic>, "");

template <typename Base, typename ValueT, bool IsOptional = false,
          bool IsMultiple = false>
class SideFallbackT;

// This template is used to define a port. Nodes should use it through one
// of the aliases below (Input, Output, SideInput, SideOutput).
template <typename Base, typename ValueT, bool IsOptionalV = false,
          bool IsMultipleV = false>
class PortCommon : public Base {
 public:
  using value_t = ValueT;
  static constexpr bool kOptional = IsOptionalV;
  static constexpr bool kMultiple = IsMultipleV;

  using Optional = PortCommon<Base, ValueT, true, IsMultipleV>;
  using Multiple = PortCommon<Base, ValueT, IsOptionalV, true>;
  using SideFallback = SideFallbackT<Base, ValueT, IsOptionalV, IsMultipleV>;

  explicit constexpr PortCommon(absl::string_view tag)
      : Base(tag, kTypeId<ValueT>, IsOptionalV, IsMultipleV) {}

  template <std::size_t N>
  explicit constexpr PortCommon(const char (&tag)[N])
      : Base(N, tag, kTypeId<ValueT>, IsOptionalV, IsMultipleV) {}

  using PayloadT = ActualPayloadT<ValueT>;

  auto operator()(CalculatorContext* cc) const {
    return internal::AccessPort<PayloadT>(
        std::integral_constant<bool, IsMultipleV>{}, *this, cc);
  }

  auto operator()(CalculatorContract* cc) const {
    return internal::AccessPort<PayloadT>(
        std::integral_constant<bool, IsMultipleV>{}, *this, cc);
  }

 private:
  absl::Status AddToContract(CalculatorContract* cc) const {
    if (kMultiple) {
      AddMultiple(cc);
    } else {
      auto& pt = internal::GetCollection(cc, *this).Tag(this->Tag());
      internal::SetType<value_t>(cc, pt);
      if (kOptional) {
        pt.Optional();
      }
    }
    return {};
  }

  void AddMultiple(CalculatorContract* cc) const {
    auto& collection = internal::GetCollection(cc, *this);
    int count = collection.NumEntries(this->Tag());
    for (int i = 0; i < count; ++i) {
      internal::SetType<value_t>(cc, collection.Get(this->Tag(), i));
    }
  }

  template <typename...>
  friend class internal::Contract;
  template <typename B, typename VT, bool, bool>
  friend class mediapipe::api2::SideFallbackT;
};

// Use one of these templates to define a port in node code.
template <typename T = internal::Generic>
using Input = PortCommon<InputBase, T>;

template <typename T = internal::Generic>
using Output = PortCommon<OutputBase, T>;

template <typename T = internal::Generic>
using SideInput = PortCommon<SideInputBase, T>;

template <typename T = internal::Generic>
using SideOutput = PortCommon<SideOutputBase, T>;

template <typename Base, typename ValueT, bool IsOptionalV, bool IsMultipleV>
class SideFallbackT : public Base {
 public:
  using value_t = ValueT;
  static constexpr bool kOptional = IsOptionalV;
  static constexpr bool kMultiple = IsMultipleV;
  using Optional = SideFallbackT<Base, ValueT, true, IsMultipleV>;
  using PayloadT = ActualPayloadT<ValueT>;

  const char* Tag() const { return stream_port.Tag(); }

  auto operator()(CalculatorContract* cc) const {
    bool is_stream = true;
    auto& stream_collection = internal::GetCollection(cc, stream_port);
    auto* packet_type = internal::GetOrNull(stream_collection, Tag(), 0);
    if (packet_type == nullptr) {
      auto& side_collection = internal::GetCollection(cc, side_port);
      packet_type = internal::GetOrNull(side_collection, Tag(), 0);
      is_stream = false;
    }
    return internal::SinglePortAccess<PayloadT>(cc, packet_type, is_stream);
  }

  auto operator()(CalculatorContext* cc) const {
    auto& stream_collection = internal::GetCollection(cc, stream_port);
    auto& side_collection = internal::GetCollection(cc, side_port);
    return internal::SinglePortAccess<PayloadT>(
        cc, internal::GetOrNull(stream_collection, Tag(), 0),
        internal::GetOrNull(side_collection, Tag(), 0));
  }

  template <std::size_t N>
  explicit constexpr SideFallbackT(const char (&tag)[N])
      : Base(N, tag, kTypeId<ValueT>, IsOptionalV, IsMultipleV),
        stream_port(tag),
        side_port(tag) {}

 protected:
  absl::Status AddToContract(CalculatorContract* cc) const {
    stream_port.AddToContract(cc);
    side_port.AddToContract(cc);
    int connected_count =
        stream_port(cc).IsConnected() + side_port(cc).IsConnected();
    if (connected_count > 1)
      return absl::InvalidArgumentError(absl::StrCat(
          Tag(),
          " can be connected as a stream or as a side packet, but not both"));
    if (!IsOptionalV && connected_count == 0)
      return absl::InvalidArgumentError(
          absl::StrCat(Tag(), " must be connected"));
    return {};
  }

  using StreamPort = PortCommon<Base, ValueT, true, IsMultipleV>;
  using SidePort = PortCommon<typename internal::SideBase<Base>::type, ValueT,
                              true, IsMultipleV>;
  StreamPort stream_port;
  SidePort side_port;

  template <typename...>
  friend class internal::Contract;
};

// An OutputShardAccess is returned when accessing an output stream within a
// CalculatorContext (e.g. kOut(cc)), and provides a type-safe interface to
// OutputStreamShard. Like that class, this class will not be usually named in
// calculator code, but used as a temporary object (e.g. kOut(cc).Send(...)).
//
// If not connected (!IsConnected()) SetNextTimestampBound is safe to call and
// does nothing.
// All the sub-classes that define Send should implement it to be safe to to
// call if not connected and do nothing in such case.
class OutputShardAccessBase {
 public:
  OutputShardAccessBase(const CalculatorContext& cc, OutputStreamShard* output)
      : context_(cc), output_(output) {}

  Timestamp NextTimestampBound() const {
    return (output_) ? output_->NextTimestampBound() : Timestamp::Unset();
  }
  void SetNextTimestampBound(Timestamp timestamp) {
    if (output_) output_->SetNextTimestampBound(timestamp);
  }

  bool IsClosed() const { return output_ ? output_->IsClosed() : true; }
  void Close() {
    if (output_) output_->Close();
  }

  bool IsConnected() const { return output_ != nullptr; }

 protected:
  const CalculatorContext& context_;
  OutputStreamShard* output_;
};

template <typename T>
class OutputShardAccess : public OutputShardAccessBase {
 public:
  void Send(Packet<T>&& packet) {
    if (output_) output_->AddPacket(ToOldPacket(std::move(packet)));
  }

  void Send(const Packet<T>& packet) {
    if (output_) output_->AddPacket(ToOldPacket(packet));
  }

  void Send(const T& payload, Timestamp time) {
    Send(api2::MakePacket<T>(payload).At(time));
  }

  void Send(const T& payload) { Send(payload, context_.InputTimestamp()); }

  void Send(T&& payload, Timestamp time) {
    Send(api2::MakePacket<T>(std::move(payload)).At(time));
  }

  void Send(T&& payload) {
    Send(std::move(payload), context_.InputTimestamp());
  }

  void Send(std::unique_ptr<T> payload, Timestamp time) {
    Send(api2::PacketAdopting(std::move(payload)).At(time));
  }

  void Send(std::unique_ptr<T> payload) {
    Send(std::move(payload), context_.InputTimestamp());
  }

  void SetHeader(const PacketBase& header) {
    if (output_) output_->SetHeader(ToOldPacket(header));
  }

 private:
  OutputShardAccess(const CalculatorContext& cc, OutputStreamShard* output)
      : OutputShardAccessBase(cc, output) {}

  friend OutputShardAccess<T> internal::SinglePortAccess<T>(
      mediapipe::CalculatorContext*, OutputStreamShard*);
};

template <>
class OutputShardAccess<internal::Generic> : public OutputShardAccessBase {
 public:
  void Send(PacketBase&& packet) {
    if (output_) output_->AddPacket(ToOldPacket(std::move(packet)));
  }

  void Send(const PacketBase& packet) {
    if (output_) output_->AddPacket(ToOldPacket(packet));
  }

  void SetHeader(const PacketBase& header) {
    if (output_) output_->SetHeader(ToOldPacket(header));
  }

 private:
  OutputShardAccess(const CalculatorContext& cc, OutputStreamShard* output)
      : OutputShardAccessBase(cc, output) {}

  friend OutputShardAccess<internal::Generic>
  internal::SinglePortAccess<internal::Generic>(mediapipe::CalculatorContext*,
                                                OutputStreamShard*);
};

// Equivalent of OutputShardAccess, but for side packets.
template <typename T>
class OutputSidePacketAccess {
 public:
  void Set(Packet<T> packet) {
    if (output_) output_->Set(ToOldPacket(std::move(packet)));
  }

  void Set(const T& payload) { Set(api2::MakePacket<T>(payload)); }
  void Set(T&& payload) { Set(api2::MakePacket<T>(std::move(payload))); }

  bool IsConnected() const { return output_ != nullptr; }

 private:
  explicit OutputSidePacketAccess(OutputSidePacket* output) : output_(output) {}
  OutputSidePacket* output_;

  friend OutputSidePacketAccess<T> internal::SinglePortAccess<T>(
      mediapipe::CalculatorContext*, OutputSidePacket*);
};

template <typename T>
class InputShardAccess : public Packet<T> {
 public:
  const PacketBase& packet() const& { return *this; }
  // Since InputShardAccess is currently created as a temporary, this avoids
  // easy mistakes with dangling references.
  PacketBase packet() const&& { return *this; }

  bool IsDone() const { return stream_->IsDone(); }
  bool IsConnected() const { return stream_ != nullptr; }

  PacketBase Header() const { return FromOldPacket(stream_->Header()); }

  // "Consume" requires exclusive ownership of the packet's payload. In the
  // current interim implementation, InputShardAccess creates a new reference to
  // the payload (as a Packet<T> instead of a type-erased Packet), which means
  // the conditions for Consume would never be satisfied. This helper class
  // defines wrappers for the Consume methods in Packet which temporarily erase
  // the reference held by the underlying InputStreamShard.
  // Note that we cannot simply take over the reference when InputShardAccess is
  // created, because it is currently created as a temporary and we might create
  // more than one instance for the same stream.
  template <class U = T,
            class = std::enable_if_t<std::is_same<U, T>{},
                                     decltype(&Packet<U>::Consume)>>
  absl::StatusOr<std::unique_ptr<U>> Consume() {
    return WrapConsumeCall(&Packet<T>::Consume);
  }

  template <class V, class U = T,
            std::enable_if_t<internal::IsCompatibleType<V, U>{}, int> = 0>
  absl::StatusOr<std::unique_ptr<V>> Consume() {
    return WrapConsumeCall(&Packet<T>::template Consume<V>);
  }

  template <class... F>
  auto ConsumeAndVisit(F&&... args) {
    auto f = &Packet<T>::template ConsumeAndVisit<F...>;
    return WrapConsumeCall(f, std::forward<F>(args)...);
  }

 private:
  InputShardAccess(const CalculatorContext&, InputStreamShard* stream)
      : Packet<T>(stream ? FromOldPacket(stream->Value()).template As<T>()
                         : Packet<T>()),
        stream_(stream) {}

  template <class F, class... A>
  auto WrapConsumeCall(F f, A&&... args) {
    stream_->Value() = {};
    auto result = (this->*f)(std::forward<A>(args)...);
    if (!result.ok()) {
      stream_->Value() = ToOldPacket(*this);
    }
    return result;
  }

  InputStreamShard* stream_;

  friend InputShardAccess<T> internal::SinglePortAccess<T>(
      mediapipe::CalculatorContext*, InputStreamShard*);
};

template <typename T>
class InputSidePacketAccess : public Packet<T> {
 public:
  const PacketBase& packet() const& { return *this; }
  PacketBase packet() const&& { return *this; }

  bool IsConnected() const { return connected_; }

 private:
  InputSidePacketAccess(const mediapipe::Packet* packet)
      : Packet<T>(packet ? FromOldPacket(*packet).template As<T>()
                         : Packet<T>()),
        connected_(packet != nullptr) {}
  const bool connected_;

  friend InputSidePacketAccess<T> internal::SinglePortAccess<T>(
      mediapipe::CalculatorContext*, const mediapipe::Packet*);
};

template <typename T>
class InputShardOrSideAccess : public Packet<T> {
 public:
  const PacketBase& packet() const& { return *this; }
  PacketBase packet() const&& { return *this; }

  bool IsDone() const { return stream_->IsDone(); }
  bool IsConnected() const { return connected_; }
  bool IsStream() const { return stream_ != nullptr; }

  PacketBase Header() const { return FromOldPacket(stream_->Header()); }

 private:
  InputShardOrSideAccess(const CalculatorContext&, InputStreamShard* stream,
                         const mediapipe::Packet* packet)
      : Packet<T>(stream   ? FromOldPacket(stream->Value()).template As<T>()
                  : packet ? FromOldPacket(*packet).template As<T>()
                           : Packet<T>()),
        stream_(stream),
        connected_(stream_ != nullptr || packet != nullptr) {}
  InputStreamShard* stream_;
  bool connected_;

  friend InputShardOrSideAccess<T> internal::SinglePortAccess<T>(
      mediapipe::CalculatorContext*, InputStreamShard*,
      const mediapipe::Packet*);
};

class PacketTypeAccess {
 public:
  bool IsConnected() const { return packet_type_ != nullptr; }

 protected:
  PacketTypeAccess(PacketType* pt) : packet_type_(pt) {}
  PacketType* packet_type_;

  template <typename T>
  friend PacketTypeAccess internal::SinglePortAccess(
      mediapipe::CalculatorContract*, PacketType*);
};

class PacketTypeAccessFallback : public PacketTypeAccess {
 public:
  bool IsStream() const { return is_stream_; }

 private:
  PacketTypeAccessFallback(PacketType* pt, bool is_stream)
      : PacketTypeAccess(pt), is_stream_(is_stream) {}
  bool is_stream_;

  template <typename T>
  friend PacketTypeAccessFallback internal::SinglePortAccess(
      mediapipe::CalculatorContract*, PacketType*, bool);
};

namespace internal {
template <typename ValueT>
PacketTypeAccess SinglePortAccess(mediapipe::CalculatorContract* cc,
                                  PacketType* pt) {
  return PacketTypeAccess(pt);
}
template <typename ValueT>
PacketTypeAccessFallback SinglePortAccess(mediapipe::CalculatorContract* cc,
                                          PacketType* pt, bool is_stream) {
  return PacketTypeAccessFallback(pt, is_stream);
}
}  // namespace internal

}  // namespace api2
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_API2_PORT_H_
