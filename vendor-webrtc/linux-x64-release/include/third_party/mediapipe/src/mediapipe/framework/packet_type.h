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

#ifndef MEDIAPIPE_FRAMEWORK_PACKET_TYPE_H_
#define MEDIAPIPE_FRAMEWORK_PACKET_TYPE_H_

#include <map>
#include <memory>
#include <set>
#include <string>
#include <typeinfo>
#include <vector>

#include "absl/base/macros.h"
#include "absl/log/absl_check.h"
#include "absl/log/absl_log.h"
#include "absl/status/status.h"
#include "absl/strings/str_split.h"
#include "absl/strings/string_view.h"
#include "absl/types/span.h"
#include "mediapipe/framework/collection.h"
#include "mediapipe/framework/deps/no_destructor.h"
#include "mediapipe/framework/packet.h"
#include "mediapipe/framework/packet_set.h"
#include "mediapipe/framework/port/status.h"
#include "mediapipe/framework/tool/type_util.h"
#include "mediapipe/framework/tool/validate_name.h"
#include "mediapipe/framework/type_map.h"

namespace mediapipe {

// Encapsulates the type and description of an input stream, output
// stream, or input side packet.  The type to expect is set with Set<type>()
// and a packet is validated with Validate().  The PacketType can be
// set to indicate the same type as some other PacketType.
class PacketType {
 public:
  // Creates an uninitialized PacketType.
  PacketType() = default;

  // PacketType can be passed by value.
  PacketType(const PacketType&) = default;
  PacketType& operator=(const PacketType&) = default;

  // False for a PacketType that has not had any Set*() function called.
  bool IsInitialized() const;

  // The following "Set*" functions initialize the PacketType.  They can
  // be called any number of times; the PacketType is initialized to
  // whatever the last call is.

  // Sets the packet type to accept the provided type.
  template <typename T>
  PacketType& Set();
  // Sets the packet type to accept packets of any type.  If an input or
  // output stream is set to this type then the framework tries to infer
  // the stream type based on the packet types of other Calculators.
  // Specifically, using SetAny() still means that the stream has a type
  // but this particular calculator just doesn't care what it is.
  PacketType& SetAny();
  // Sets the packet type to accept any of the provided types.
  template <typename... T>
  PacketType& SetOneOf();
  // Sets the packet type to not accept any packets.
  PacketType& SetNone();
  // Sets the PacketType to be the same as type.  This actually stores
  // the pointer to the other PacketType but does not acquire ownership.
  // "type" must outlive this object.
  PacketType& SetSameAs(const PacketType* type);
  // Marks this port as optional.
  PacketType& Optional();

  // Returns a pointer to the root PacketType of the "same as" equivalence
  // class.
  const PacketType* GetSameAs() const;
  PacketType* GetSameAs();
  // Returns true if this PacketType allows anything.
  bool IsAny() const;
  // Returns true if this PacketType allows nothing.
  bool IsNone() const;
  // Returns true if this PacketType allows a set of types.
  bool IsOneOf() const;
  // Returns true if this PacketType allows one specific type.
  bool IsExactType() const;
  // Returns true if this port has been marked as optional.
  bool IsOptional() const { return optional_; }

  // Returns true iff this and other are consistent, meaning they do
  // not expect different types.  IsAny() is consistent with anything.
  // IsNone() is only consistent with IsNone() and IsAny().
  // Note: this is definied as a symmetric relationship, but within the
  // framework, it is consistently invoked as:
  //   input_port_type.IsConsistentWith(connected_output_port_type)
  // TODO: consider making this explicitly directional, and
  // sharing some logic with the packet validation check.
  bool IsConsistentWith(const PacketType& other) const;

  // Returns OK if the packet contains an object of the appropriate type.
  absl::Status Validate(const Packet& packet) const;

  // Returns a pointer to the Registered type name, or nullptr if the type
  // is not registered.  Do not use this for validation, use Validate()
  // instead.
  const std::string* RegisteredTypeName() const;
  // Returns the type name.  Do not use this for validation, use
  // Validate() instead.
  std::string DebugTypeName() const;

 private:
  struct SameAs {
    // This PacketType is the same as other.
    // We don't do union-find optimizations in order to avoid a mutex.
    const PacketType* other;
  };
  using TypeIdSpan = absl::Span<const TypeId>;
  struct MultiType {
    TypeIdSpan types;
    // TODO: refactor RegisteredTypeName, remove.
    const std::string* registered_type_name;
  };
  struct SpecialType;
  using TypeSpec =
      absl::variant<absl::monostate, TypeId, MultiType, SameAs, SpecialType>;
  typedef absl::Status (*AcceptsTypeFn)(const TypeSpec& type);
  struct SpecialType {
    std::string name_;
    AcceptsTypeFn accept_fn_;
  };

  static absl::Status AcceptAny(const TypeSpec& type);
  static absl::Status AcceptNone(const TypeSpec& type);

  const PacketType* SameAsPtr() const;
  static TypeIdSpan GetTypeSpan(const TypeSpec& type_spec);
  static std::string TypeNameForOneOf(TypeIdSpan types);

  TypeSpec type_spec_;

  // Whether the corresponding port is optional.
  bool optional_ = false;
};

// An error handler class which allows a PacketTypeSet to return valid
// results while deferring errors.
//
// This class is thread compatible.
class PacketTypeSetErrorHandler {
 public:
  // Returns a usable PacketType.  A different PacketType object is
  // returned for each different invalid location and the same object
  // is returned for multiple accesses to the same invalid location.
  PacketType& GetFallback(const absl::string_view tag, int index) {
    if (!missing_) {
      missing_ = absl::make_unique<Missing>();
    }
    ABSL_CHECK(!missing_->initialized_errors);
    std::string key = absl::StrCat(tag, ":", index);
    return missing_->entries[key];
  }

  // In the const setting produce a FATAL error.
  const PacketType& GetFallback(const absl::string_view tag, int index) const {
    ABSL_LOG(FATAL) << "Failed to get tag \"" << tag << "\" index " << index
                    << ".  Unable to defer error due to const specifier.";
    std::abort();
  }

  // Returns true if a (deferred) error has been recorded by the
  // PacketTypeSet.
  bool HasError() const { return missing_ != nullptr; }

  // Get the error messages that have been deferred.
  // This function can only be called if HasError() is true.
  const std::vector<std::string>& ErrorMessages() const {
    ABSL_CHECK(missing_) << "ErrorMessages() can only be called if errors have "
                            "occurred.  Call HasError() before calling this "
                            "function.";
    if (!missing_->initialized_errors) {
      for (const auto& entry : missing_->entries) {
        // Optional entries that were missing are not considered errors.
        if (!entry.second.IsOptional()) {
          // Split them to keep the error string unchanged.
          std::pair<std::string, std::string> tag_idx =
              absl::StrSplit(entry.first, ':');
          missing_->errors.push_back(absl::StrCat("Failed to get tag \"",
                                                  tag_idx.first, "\" index ",
                                                  tag_idx.second));
        }
      }
      missing_->initialized_errors = true;
    }
    return missing_->errors;
  }

 private:
  struct Missing {
    // Mapping from TAG:index to PacketType objects, one for each invalid
    // location that has been accessed.
    std::map<std::string, PacketType> entries;
    // The list of errors is only computed at the end.
    std::vector<std::string> errors;
    bool initialized_errors = false;
  };

  // Initialize lazily to save space in the common no-error case.
  std::unique_ptr<Missing> missing_;
};

// A collection of PacketTypes.  The types are either retrieved by index
// or by tag.  The calculator must set a type for every input stream and
// input side packet that it accepts and every output stream it produces.
// Every (non-const) function in this class always returns valid values
// that can be used directly without error checking.  If the types don't
// match what the user provided then an error will be triggered later
// (but the program will not crash).
//
// For example, a calculator can just call
//   inputs->Tag("VIDEO").Set<ImageFrame>("Annotated Video Frames.");
// Without checking that "VIDEO" is a valid tag.
//
// Similarly if the following is specified:
//   inputs->Index(0).Set<int>("Some Integer.");
//   inputs->Index(1).Set<std::string>("Some String.");
//   inputs->Index(2).Set<float>("Some Float.");
// then it is not necessary to check that NumEntries() == 3. An error
// is triggered if there aren't exactly 3 inputs or they don't have the
// proper types.
//
// For a const PacketTypeSet, invalid access is a fatal error.
//
// This class is thread compatible.
using PacketTypeSet =
    internal::Collection<PacketType, internal::CollectionStorage::kStoreValue,
                         PacketTypeSetErrorHandler>;

// Returns OK if the packets in the PacketSet are of the appropriate type.
// packet_type_set must be valid before this is called (but packet_set
// may be in any state).
absl::Status ValidatePacketSet(const PacketTypeSet& packet_type_set,
                               const PacketSet& packet_set);

// Validates that the PacketTypeSet was initialized properly.
// An error is returned if
// 1) Tag() or Index() is called with an invalid argument (however,
//    a valid PacketType is still returned by the function).
// 2) Any PacketType is not initialized.
absl::Status ValidatePacketTypeSet(const PacketTypeSet& packet_type_set);

// Templated function definitions.

template <typename T>
PacketType& PacketType::Set() {
  type_spec_ = kTypeId<T>;
  return *this;
}

template <typename... T>
PacketType& PacketType::SetOneOf() {
  static const NoDestructor<std::vector<TypeId>> types{{kTypeId<T>...}};
  static const NoDestructor<std::string> name{TypeNameForOneOf(*types)};
  type_spec_ = MultiType{*types, &*name};
  return *this;
}

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_PACKET_TYPE_H_
