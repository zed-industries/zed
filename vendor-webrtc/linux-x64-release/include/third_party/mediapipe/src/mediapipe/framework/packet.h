// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Defines Packet, a container capable of holding an object of any type.

#ifndef MEDIAPIPE_FRAMEWORK_PACKET_H_
#define MEDIAPIPE_FRAMEWORK_PACKET_H_

#include <cstddef>
#include <cstdint>
#include <memory>
#include <ostream>
#include <string>
#include <type_traits>
#include <utility>
#include <vector>

#include "absl/base/attributes.h"
#include "absl/base/macros.h"
#include "absl/base/optimization.h"
#include "absl/functional/any_invocable.h"
#include "absl/log/absl_check.h"
#include "absl/log/absl_log.h"
#include "absl/memory/memory.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/str_cat.h"
#include "absl/synchronization/mutex.h"
#include "google/protobuf/message.h"
#include "google/protobuf/message_lite.h"
#include "mediapipe/framework/deps/no_destructor.h"
#include "mediapipe/framework/deps/registration.h"
#include "mediapipe/framework/port.h"
#include "mediapipe/framework/port/canonical_errors.h"
#include "mediapipe/framework/port/logging.h"
#include "mediapipe/framework/port/proto_ns.h"
#include "mediapipe/framework/port/ret_check.h"
#include "mediapipe/framework/port/status.h"
#include "mediapipe/framework/port/status_builder.h"
#include "mediapipe/framework/port/status_macros.h"
#include "mediapipe/framework/port/statusor.h"
#include "mediapipe/framework/timestamp.h"
#include "mediapipe/framework/tool/type_util.h"
#include "mediapipe/framework/type_map.h"

namespace mediapipe {

class Packet;

namespace packet_internal {
class HolderBase;

Packet Create(HolderBase* holder);
Packet Create(HolderBase* holder, Timestamp timestamp);
Packet Create(std::shared_ptr<const HolderBase> holder, Timestamp timestamp);
const HolderBase* GetHolder(const Packet& packet);
const std::shared_ptr<const HolderBase>& GetHolderShared(const Packet& packet);
std::shared_ptr<const HolderBase> GetHolderShared(Packet&& packet);
absl::StatusOr<Packet> PacketFromDynamicProto(const std::string& type_name,
                                              const std::string& serialized);
}  // namespace packet_internal

// A generic container class which can hold data of any type.  The type of
// the data is specified when accessing the data (using Packet::Get<T>()).
//
// The Packet is implemented as a reference-counted pointer.  This means
// that copying Packets creates a fast, shallow copy.  Packets are
// copyable, movable, and assignable.  Packets can be stored in STL
// containers.  A Packet may optionally contain a timestamp.
//
// The preferred method of creating a Packet is with MakePacket<T>().
// The Packet typically owns the object that it contains, but
// PointToForeign allows a Packet to be constructed which does not
// own its data.
//
// This class is thread compatible.
class Packet {
 public:
  // Creates an empty Packet, for which IsEmpty()==true and
  // Timestamp()==Timestamp::Unset(). Calling Get() on this Packet leads
  // to CHECK-failure.
  Packet() = default;

  // Copy constructor and assignment operator.
  Packet(const Packet&);
  Packet& operator=(const Packet&);

  // Move constructor and assignment operator that take non-const rvalue
  // reference.
  Packet(Packet&&);
  Packet& operator=(Packet&&);

  // Returns a Packet that contains the same data as *this, and has the
  // given timestamp. Does not modify *this.
  Packet At(class Timestamp timestamp) const&;

  // The rvalue reference overload of Packet's member function
  // Packet::At(class Timestamp). Moves *this to a new Packet and returns
  // the new Packet with the given timestamp.
  Packet At(class Timestamp timestamp) &&;

  // Returns true iff the Packet has been created using the default
  // constructor Packet(), or is a copy of such a Packet.
  bool IsEmpty() const;

  // Returns the reference to the object of typename T if it contains
  // one, crashes otherwise. It is safe to concurrently call Get()
  // on the same packet from multiple threads.
  template <typename T>
  const T& Get() const;

  // Returns a shared pointer to the object of typename T if it contains
  // one, an error otherwise (if the packet is empty or of another type). It is
  // safe to concurrently call Share() on the same packet from multiple threads.
  template <typename T>
  absl::StatusOr<std::shared_ptr<const T>> Share() const;

  // DEPRECATED
  //
  // Transfers the ownership of holder's data to a unique pointer
  // of the object if the packet is the sole owner of a non-foreign
  // holder. Otherwise, returns error when the packet can't be consumed.
  //
  // --- WARNING ---
  // Packet is thread-compatible and this member function is non-const. Hence,
  // calling it requires exclusive access to the object - callers are
  // responsible for ensuring that no other thread is doing anything with the
  // packet.
  //
  // For example, if a node/calculator calls this function, then no other
  // calculator should be processing the same packet. Nodes/calculators cannot
  // enforce/guarantee this as they don't know of each other, which means graph
  // must be written in a special way to account for that. It's error-prone and
  // general recommendation is to avoid calling this function.
  template <typename T>
  ABSL_DEPRECATED(
      "Avoid Consume* functions usage as in most cases it's hard to ensure "
      "the proper usage (taken the nature of calculators not knowing where "
      "packets are received from and sent to) and leads to races. Consider "
      "SharedPtrWithPacket instead to get a shared_ptr<T> if applicable.")
  absl::StatusOr<std::unique_ptr<T>> Consume();

  // DEPRECATED
  //
  // Consumes the packet and transfers the ownership of the data to a
  // unique pointer if the packet is the sole owner of a non-foreign
  // holder. Otherwise, the unique pointer holds a copy of the original
  // data. In either case, the original packet is set to empty. The
  // function returns error when the packet can't be consumed or copied. If
  // was_copied is not nullptr, it is set to indicate whether the packet
  // data was copied.
  //
  // --- WARNING ---
  // Packet is thread-compatible and this member function is non-const. Hence,
  // calling it requires exclusive access to the object - callers are
  // responsible for ensuring that no other thread is doing anything with the
  // packet.
  //
  // For example, if a node/calculator calls this function, then no other
  // calculator should be processing the same packet. Nodes/calculators cannot
  // enforce/guarantee this as they don't know of each other, which means graph
  // must be written in a special way to account for that. It's error-prone and
  // general recommendation is to avoid calling this function.
  //
  // Example usage:
  //   MP_ASSIGN_OR_RETURN(std::unique_ptr<Detection> detection,
  //                    p.ConsumeOrCopy<Detection>());
  //   // The unique_ptr type can be omitted with auto.
  //   MP_ASSIGN_OR_RETURN(auto detection, p.ConsumeOrCopy<Detection>());
  //   If you would like to crash on failure (prefer MP_ASSIGN_OR_RETURN):
  //   auto detection = p.ConsumeOrCopy<Detection>().value();
  //   // In functions which do not return absl::Status use an adaptor
  //   // function as the third argument to MP_ASSIGN_OR_RETURN.  In tests,
  //   // use an adaptor which returns void.
  //   MP_ASSIGN_OR_RETURN(auto detection, p.ConsumeOrCopy<Detection>(),
  //                    _.With([](const absl::Status& status) {
  //                      MP_EXPECT_OK(status);
  //                      // Use CHECK_OK to crash and report a usable line
  //                      // number (which the value() alternative does not).
  //                      // Include a return statement if the return value is
  //                      // non-void.  For example: return 1;
  //                    }));
  //
  // Version for non-arrays.
  template <typename T>
  ABSL_DEPRECATED(
      "Avoid Consume* functions usage as in most cases it's hard to ensure "
      "the proper usage (taken the nature of calculators not knowing where "
      "packets are received from and sent to) and leads to races. Consider "
      "SharedPtrWithPacket instead to get a shared_ptr<T> if applicable.")
  absl::StatusOr<std::unique_ptr<T>> ConsumeOrCopy(
      bool* was_copied = nullptr,
      typename std::enable_if<!std::is_array<T>::value>::type* = nullptr);

  // Version for bounded array.
  template <typename T>
  ABSL_DEPRECATED(
      "Avoid Consume* functions usage as in most cases it's hard to ensure "
      "the proper usage (taken the nature of calculators not knowing where "
      "packets are received from and sent to) and leads to races. Consider "
      "SharedPtrWithPacket instead to get a shared_ptr<T> if applicable.")
  absl::StatusOr<std::unique_ptr<T>> ConsumeOrCopy(
      bool* was_copied = nullptr,
      typename std::enable_if<std::is_array<T>::value &&
                              std::extent<T>::value != 0>::type* = nullptr);

  // DEPRECATED
  //
  // TODO: Support unbounded array after fixing the bug in holder's
  // delete helper.
  // Version for unbounded array.
  template <typename T>
  ABSL_DEPRECATED(
      "Avoid Consume* functions usage as in most cases it's hard to ensure "
      "the proper usage (taken the nature of calculators not knowing where "
      "packets are received from and sent to) and leads to races. Consider "
      "SharedPtrWithPacket instead to get a shared_ptr<T> if applicable.")
  absl::StatusOr<std::unique_ptr<T>> ConsumeOrCopy(
      bool* was_copied = nullptr,
      typename std::enable_if<std::is_array<T>::value &&
                              std::extent<T>::value == 0>::type* = nullptr);

  // Returns the reference to type MessageLite data, if the underlying
  // object type is protocol buffer, crashes otherwise.
  const proto_ns::MessageLite& GetProtoMessageLite() const;

  // Returns a vector of pointers to MessageLite data, if the underlying
  // object type is a vector of MessageLite data, returns an error otherwise.
  // Note: This function is meant to be used internally within the MediaPipe
  // framework only.
  absl::StatusOr<std::vector<const proto_ns::MessageLite*>>
  GetVectorOfProtoMessageLitePtrs() const;

  // Returns an error if the packet does not contain data of type T.
  template <typename T>
  absl::Status ValidateAsType() const {
    return ValidateAsType(kTypeId<T>);
  }

  // Returns an error if the packet is not an instance of
  // a protocol buffer message.
  absl::Status ValidateAsProtoMessageLite() const;

  // Get the type id for the underlying type stored in the Packet.
  // Crashes if IsEmpty() == true.
  TypeId GetTypeId() const;

  // Returns the timestamp.
  class Timestamp Timestamp() const;

  std::string DebugString() const;
  friend std::ostream& operator<<(std::ostream& stream, const Packet& p) {
    return stream << p.DebugString();
  }

  // Returns the type name.  If the packet is empty or the type is not
  // registered (with MEDIAPIPE_REGISTER_TYPE or companion macros) then
  // the empty string is returned.
  std::string RegisteredTypeName() const;
  // Returns a string with the best guess at the type name.
  std::string DebugTypeName() const;

 private:
  friend Packet packet_internal::Create(packet_internal::HolderBase* holder);
  friend Packet packet_internal::Create(packet_internal::HolderBase* holder,
                                        class Timestamp timestamp);
  friend Packet packet_internal::Create(
      std::shared_ptr<const packet_internal::HolderBase> holder,
      class Timestamp timestamp);
  friend const packet_internal::HolderBase* packet_internal::GetHolder(
      const Packet& packet);
  friend const std::shared_ptr<const packet_internal::HolderBase>&
  packet_internal::GetHolderShared(const Packet& packet);
  friend std::shared_ptr<const packet_internal::HolderBase>
  packet_internal::GetHolderShared(Packet&& packet);

  friend class PacketType;
  absl::Status ValidateAsType(TypeId type_id) const;

  std::shared_ptr<const packet_internal::HolderBase> holder_;
  class Timestamp timestamp_;
};

// Factory functions for creating Packets. Non-members as opposed to static
// methods, to prevent users from mistakenly calling them on Packet instances.

// Returns a Packet that adopts the object; the Packet assumes the ownership.
// The timestamp of the returned Packet is Timestamp::Unset(). To set the
// timestamp, the caller should do Adopt(...).At(...).
//
// Generally prefer MakePacket<T>().
template <typename T>
Packet Adopt(const T* ptr);

// Returns a Packet that does not own its data. The data pointed to by *ptr
// remains owned by the caller, who must ensure that it outlives not only the
// returned Packet but also all of its copies.
//
// Optionally, `cleanup` object can be specified to invoke when all copies of
// the packet are destroyed (can be used to capture the foreign owner if
// possible and ensure the lifetime).
//
// The timestamp of the returned Packet is Timestamp::Unset(). To set the
// timestamp, the caller should do PointToForeign(...).At(...).
template <typename T>
Packet PointToForeign(const T* ptr,
                      absl::AnyInvocable<void()> cleanup = nullptr);

// Adopts the data but places it in a std::unique_ptr inside the
// resulting Packet, leaving the timestamp unset. This allows the
// adopted data to be mutated, with the mutable data accessible as
// packet.Get<std::unique_ptr<T>>().get(). GetFromUniquePtr below provides
// a more syntactically-pleasing way of accomplishing that.
template <typename T>
Packet AdoptAsUniquePtr(T* ptr) {
  static_assert(
      !std::is_const<T>::value,
      "AdoptAsUniquePtr should not be called with a pointer-to-const.");
  return Adopt(new std::unique_ptr<T>(ptr));
}

// A SyncedPacket is a packet containing a reference to another packet, and the
// reference can be updated.
// SyncedPacket is thread-safe.
class SyncedPacket {
 public:
  explicit SyncedPacket(const Packet& packet) : packet_(packet) {}
  void UpdatePacket(const Packet& packet) {
    absl::WriterMutexLock writer_lock(&mutex_);
    packet_ = packet;
  }
  Packet Get() {
    absl::ReaderMutexLock reader_lock(&mutex_);
    return packet_;
  }

 private:
  absl::Mutex mutex_;
  Packet packet_;
};

// Adopt the data as SyncedPacket, so that the content of the packet can be
// updated in a thread-safe way.
// Usage:
//   Packet synced_packet = AdoptAsSyncedPacket(new int(100));
//   Packet value_packet =
//       synced_packet.Get<std::unique_ptr<SyncedPacket>>()->Get();
//   EXPECT_EQ(100, value_packet.Get<int>());
//   // update the value.
//   Packet new_value_packet = Adopt(new int(999));
//   synced_packet.Get<std::unique_ptr<SyncedPacket>>()
//       ->UpdatePacket(new_value_packet);
//   Packet packet_new =
//       synced_packet.Get<std::unique_ptr<SyncedPacket>>()->Get();
//   EXPECT_EQ(999, packet_new.Get<int>());

template <typename T>
Packet AdoptAsSyncedPacket(const T* ptr) {
  Packet packet = Adopt(ptr);
  return AdoptAsUniquePtr(new SyncedPacket(packet));
}

// Create a packet containing an object of type T initialized with the
// provided arguments. Similar to MakeUnique. Especially convenient for arrays,
// since it ensures the packet gets the right type (see below).
//
// Version for scalars.
template <typename T,
          typename std::enable_if<!std::is_array<T>::value>::type* = nullptr,
          typename... Args>
Packet MakePacket(Args&&... args) {  // NOLINT(build/c++11)
  return Adopt(new T(std::forward<Args>(args)...));
}

// Version for arrays. We have to use reinterpret_cast because new T[N]
// returns a T* instead of a T(*)[N] (i.e. a pointer to the first element
// instead of a pointer to the array itself - they have the same value, but
// different types), which would prevent Adopt from seeing the array's type
// if we did not have the cast.
template <typename T,
          typename std::enable_if<std::is_array<T>::value>::type* = nullptr,
          typename... Args>
Packet MakePacket(Args&&... args) {  // NOLINT(build/c++11)
  return Adopt(reinterpret_cast<T*>(
      new T{std::forward<typename std::remove_extent<T>::type>(args)...}));
}

// Returns a mutable pointer to the data in a unique_ptr in a packet. This
// is useful in combination with AdoptAsUniquePtr.  The caller must
// exercise caution when mutating the retrieved data, since the data
// may be accessible from other locations.
template <typename T>
T* GetFromUniquePtr(const Packet& packet) {
  return packet.Get<std::unique_ptr<T>>().get();
}

// Returns a shared_ptr to the payload of the packet which retains its object
// through a copy of the packet.
// Use std::const_pointer_cast if you need a shared_ptr<T>, but remember that
// you must not change the payload if the packet has other owners.
template <typename T>
ABSL_DEPRECATED("Use Packet::Share<T>() instead.")
std::shared_ptr<const T> SharedPtrWithPacket(const Packet& packet) {
  absl::StatusOr<std::shared_ptr<const T>> shared_ptr = packet.Share<T>();
  ABSL_CHECK_OK(shared_ptr.status());
  return *std::move(shared_ptr);
}

//// Implementation details.
namespace packet_internal {

template <typename T>
class Holder;
template <typename T>
class ForeignHolder;

class HolderBase {
 public:
  HolderBase() {}
  HolderBase(const HolderBase&) = delete;
  HolderBase& operator=(const HolderBase&) = delete;
  virtual ~HolderBase();
  template <typename T>
  bool PayloadIsOfType() const {
    return GetTypeId() == kTypeId<T>;
  }
  // Returns a printable string identifying the type stored in the holder.
  virtual const std::string DebugTypeName() const = 0;
  // Returns debug data id.
  virtual int64_t DebugDataId() const = 0;
  // Returns the registered type name if it's available, otherwise the
  // empty string.
  virtual const std::string RegisteredTypeName() const = 0;
  // Get the type id of the underlying data type.
  virtual TypeId GetTypeId() const = 0;

  // Downcasts this to Holder<T>.  Returns nullptr if deserialization
  // failed or if the requested type is not what is stored.
  template <typename T>
  const Holder<T>* As() const;

  // Same as As() but returns this as a mutable Holder<T>.
  // Dangerous! Only use in contexts when we are guaranteed the holder is not
  // shared.
  template <typename T>
  Holder<T>* AsMutable() const;

  // Returns the pointer to MessageLite type for the data in holder, if
  // underlying object is protocol buffer type, otherwise, nullptr is returned.
  virtual const proto_ns::MessageLite* GetProtoMessageLite() const = 0;

  // Returns a vector<MessageLite*> for the data in the holder, if the
  // underlying object is a vector of protocol buffer objects, otherwise,
  // returns an error.
  virtual absl::StatusOr<std::vector<const proto_ns::MessageLite*>>
  GetVectorOfProtoMessageLite() const = 0;

  virtual bool HasForeignOwner() const { return false; }
};

// Two helper functions to get the proto base pointers.
template <typename T>
const proto_ns::MessageLite* ConvertToProtoMessageLite(const T* data,
                                                       std::false_type) {
  return nullptr;
}

template <typename T>
const proto_ns::MessageLite* ConvertToProtoMessageLite(const T* data,
                                                       std::true_type) {
  return data;
}

// Helper structs for determining if a type is an std::vector<Proto>.
template <typename Type>
struct is_proto_vector : public std::false_type {};

template <typename ItemT, typename Allocator>
struct is_proto_vector<std::vector<ItemT, Allocator>>
    : public std::is_base_of<proto_ns::MessageLite, ItemT>::type {};

// Helper function to create and return a vector of pointers to proto message
// elements of the vector passed into the function.
template <typename T>
absl::StatusOr<std::vector<const proto_ns::MessageLite*>>
ConvertToVectorOfProtoMessageLitePtrs(const T* data,
                                      /*is_proto_vector=*/std::false_type) {
  return absl::InvalidArgumentError(absl::StrCat(
      "The Packet stores \"", kTypeId<T>.name(), "\"",
      "which is not convertible to vector<proto_ns::MessageLite*>."));
}

template <typename T>
absl::StatusOr<std::vector<const proto_ns::MessageLite*>>
ConvertToVectorOfProtoMessageLitePtrs(const T* data,
                                      /*is_proto_vector=*/std::true_type) {
  std::vector<const proto_ns::MessageLite*> result;
  for (auto it = data->begin(); it != data->end(); ++it) {
    const proto_ns::MessageLite* element = &(*it);
    result.push_back(element);
  }
  return result;
}

// This registry is used to create Holders of the right concrete C++ type given
// a proto type string (which is used as the registration key).
class MessageHolderRegistry
    : public GlobalFactoryRegistry<std::unique_ptr<HolderBase>> {};

template <typename T>
struct is_concrete_proto_t
    : public std::integral_constant<
          bool, std::is_base_of<proto_ns::MessageLite, T>{} &&
                    !std::is_same<proto_ns::MessageLite, T>{} &&
                    !std::is_same<proto_ns::Message, T>{}> {};

template <typename T>
std::unique_ptr<HolderBase> CreateMessageHolder() {
  return absl::make_unique<Holder<T>>(new T);
}

// Registers a message type. T must be a non-cv-qualified concrete proto type.
MEDIAPIPE_STATIC_REGISTRATOR_TEMPLATE(MessageRegistrator, MessageHolderRegistry,
                                      T{}.GetTypeName(), CreateMessageHolder<T>)

// For non-Message payloads, this does nothing.
template <typename T, typename Enable = void>
struct HolderPayloadRegistrator {};

// This template ensures that, for each concrete MessageLite subclass that is
// stored in a Packet, we register a function that allows us to create a
// Holder with the correct payload type from the proto's type name.
//
// We must use std::remove_cv to ensure we don't try to register Foo twice if
// there are Holder<Foo> and Holder<const Foo>. TODO: lift this
// up to Holder?
template <typename T>
struct HolderPayloadRegistrator<
    T, typename std::enable_if<is_concrete_proto_t<T>{}>::type>
    : private MessageRegistrator<typename std::remove_cv<T>::type> {};

template <typename T>
class Holder : public HolderBase, private HolderPayloadRegistrator<T> {
 public:
  explicit Holder(const T* ptr) : ptr_(ptr) {}
  ~Holder() override { delete_helper(); }
  const T& data() const { return *ptr_; }
  TypeId GetTypeId() const final { return kTypeId<T>; }
  // Releases the underlying data pointer and transfers the ownership to a
  // unique pointer.
  // This method is dangerous and is only used by Packet::Consume() if the
  // packet is the only owner of the holder.
  template <typename U = T>
  absl::StatusOr<std::unique_ptr<T>> Release(
      typename std::enable_if<!std::is_array<U>::value ||
                              std::extent<U>::value != 0>::type* = 0) {
    if (HasForeignOwner()) {
      return InternalError(
          "Foreign holder can't release data ptr without ownership.");
    }
    // Casts away constness to make the data mutable after the release.
    std::unique_ptr<T> data_ptr(const_cast<T*>(ptr_));
    ptr_ = nullptr;
    return std::move(data_ptr);
  }
  // TODO: support unbounded array after fixing the bug in holder's
  // delete helper.
  template <typename U = T>
  absl::StatusOr<std::unique_ptr<T>> Release(
      typename std::enable_if<std::is_array<U>::value &&
                              std::extent<U>::value == 0>::type* = 0) {
    return absl::InternalError("Release T[] isn't supported.");
  }
  const std::string DebugTypeName() const final {
    return MediaPipeTypeStringOrDemangled<T>();
  }
  int64_t DebugDataId() const final { return reinterpret_cast<int64_t>(ptr_); }
  const std::string RegisteredTypeName() const final {
    const std::string* type_string = MediaPipeTypeString<T>();
    if (type_string) {
      return *type_string;
    }
    return "";
  }

 protected:
  // The pointer that uniquely owns the data. However, the ownership of the
  // Holder itself may be shared by several Packets.
  const T* ptr_;

  // Returns the MessageLite pointer to the data, if the underlying object type
  // is protocol buffer, otherwise, nullptr is returned.
  const proto_ns::MessageLite* GetProtoMessageLite() const override {
    return ConvertToProtoMessageLite(
        ptr_, std::is_base_of<proto_ns::MessageLite, T>());
  }

  // Returns a vector<MessageLite*> for the data in the holder, if the
  // underlying object is a vector of protocol buffer objects, otherwise,
  // returns an error.
  absl::StatusOr<std::vector<const proto_ns::MessageLite*>>
  GetVectorOfProtoMessageLite() const override {
    return ConvertToVectorOfProtoMessageLitePtrs(ptr_, is_proto_vector<T>());
  }

 private:
  // Call delete[] if T is an array, delete otherwise.
  template <typename U = T>
  inline void delete_helper(
      typename std::enable_if<!std::is_array<U>::value>::type* = 0) {
    delete ptr_;
  }
  template <typename U = T>
  inline void delete_helper(
      typename std::enable_if<std::is_array<U>::value>::type* = 0) {
    // Casts ptr_ from const Type(*)[] or const Type(*)[N] to const Type*.
    // Deleting a pointer to incomplete type (Type(*)[]) causes compile error.
    delete[] reinterpret_cast<const typename std::remove_extent<U>::type*>(
        ptr_);
  }
};

// Like Holder, but does not own its data.
template <typename T>
class ForeignHolder : public Holder<T> {
 public:
  explicit ForeignHolder(const T* ptr,
                         absl::AnyInvocable<void()> cleanup = nullptr)
      : Holder<T>::Holder(ptr), cleanup_(std::move(cleanup)) {}

  ~ForeignHolder() override {
    // Null out ptr_ so it doesn't get deleted by ~Holder.
    // Note that ~Holder cannot call HasForeignOwner because the subclass's
    // destructor runs first.
    this->ptr_ = nullptr;
    if (cleanup_) {
      cleanup_();
    }
  }

  bool HasForeignOwner() const final { return true; }

 private:
  absl::AnyInvocable<void()> cleanup_;
};

template <typename T>
Holder<T>* HolderBase::AsMutable() const {
  if (PayloadIsOfType<T>()) {
    return static_cast<Holder<T>*>(const_cast<HolderBase*>(this));
  }
  // Does not hold a T.
  return nullptr;
}

template <typename T>
const Holder<T>* HolderBase::As() const {
  if (PayloadIsOfType<T>()) {
    return static_cast<const Holder<T>*>(this);
  }
  // Does not hold a T.
  return nullptr;
}

}  // namespace packet_internal

inline Packet::Packet(const Packet& packet)
    : holder_(packet.holder_), timestamp_(packet.timestamp_) {
  VLOG(4) << "Using copy constructor of " << packet.DebugString();
}

inline Packet& Packet::operator=(const Packet& packet) {
  VLOG(4) << "Using copy assignment operator of " << packet.DebugString();
  if (this != &packet) {
    holder_ = packet.holder_;
    timestamp_ = packet.timestamp_;
  }
  return *this;
}

template <typename T>
inline absl::StatusOr<std::unique_ptr<T>> Packet::Consume() {
  return absl::InternalError("Consume isn't supported.");
}

template <typename T>
inline absl::StatusOr<std::unique_ptr<T>> Packet::ConsumeOrCopy(
    bool* was_copied,
    typename std::enable_if<!std::is_array<T>::value>::type*) {
  return absl::InternalError("ConsumeOrCopy isn't supported.");
}

template <typename T>
inline absl::StatusOr<std::unique_ptr<T>> Packet::ConsumeOrCopy(
    bool* was_copied,
    typename std::enable_if<std::is_array<T>::value &&
                            std::extent<T>::value != 0>::type*) {
  return absl::InternalError("ConsumeOrCopy isn't supported.");
}

template <typename T>
inline absl::StatusOr<std::unique_ptr<T>> Packet::ConsumeOrCopy(
    bool* was_copied,
    typename std::enable_if<std::is_array<T>::value &&
                            std::extent<T>::value == 0>::type*) {
  return absl::InternalError("Unbounded array isn't supported.");
}

inline Packet::Packet(Packet&& packet) {
  VLOG(4) << "Using move constructor of " << packet.DebugString();
  holder_ = std::move(packet.holder_);
  timestamp_ = packet.timestamp_;
  packet.timestamp_ = Timestamp::Unset();
}

inline Packet& Packet::operator=(Packet&& packet) {
  VLOG(4) << "Using move assignment operator of " << packet.DebugString();
  if (this != &packet) {
    holder_ = std::move(packet.holder_);
    timestamp_ = packet.timestamp_;
    packet.timestamp_ = Timestamp::Unset();
  }
  return *this;
}

inline bool Packet::IsEmpty() const { return holder_ == nullptr; }

inline TypeId Packet::GetTypeId() const {
  ABSL_CHECK(holder_);
  return holder_->GetTypeId();
}

template <typename T>
inline const T& Packet::Get() const {
  const packet_internal::Holder<T>* holder =
      IsEmpty() ? nullptr : holder_->As<T>();
  if (holder == nullptr) {
    // Produce a good error message.
    absl::Status status = ValidateAsType<T>();
    ABSL_LOG(FATAL) << "Packet::Get() failed: " << status.message();
  }
  return holder->data();
}

// Returns a shared pointer to the object of typename T if it contains
// one, an error otherwise (if the packet is empty or of another type). It is
// safe to concurrently call Share() on the same packet from multiple threads.
template <typename T>
inline absl::StatusOr<std::shared_ptr<const T>> Packet::Share() const {
  MP_RETURN_IF_ERROR(ValidateAsType<T>());
  const T* ptr = &Get<T>();
  return std::shared_ptr<const T>(
      ptr, [packet = *this](const T* ptr) mutable { packet = {}; });
}

inline Timestamp Packet::Timestamp() const { return timestamp_; }

template <typename T>
Packet Adopt(const T* ptr) {
  ABSL_CHECK(ptr != nullptr);
  return packet_internal::Create(new packet_internal::Holder<T>(ptr));
}

template <typename T>
Packet PointToForeign(const T* ptr, absl::AnyInvocable<void()> cleanup) {
  ABSL_CHECK(ptr != nullptr);
  return packet_internal::Create(
      new packet_internal::ForeignHolder<T>(ptr, std::move(cleanup)));
}

// Equal Packets refer to the same memory contents, like equal pointers.
inline bool operator==(const Packet& p1, const Packet& p2) {
  return packet_internal::GetHolder(p1) == packet_internal::GetHolder(p2);
}
inline bool operator!=(const Packet& p1, const Packet& p2) {
  return !(p1 == p2);
}

namespace packet_internal {

inline const std::shared_ptr<const HolderBase>& GetHolderShared(
    const Packet& packet) {
  return packet.holder_;
}

inline std::shared_ptr<const HolderBase> GetHolderShared(Packet&& packet) {
  return std::move(packet.holder_);
}

}  // namespace packet_internal

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_PACKET_H_
