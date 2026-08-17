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

#ifndef MEDIAPIPE_FRAMEWORK_PACKET_GENERATOR_H_
#define MEDIAPIPE_FRAMEWORK_PACKET_GENERATOR_H_

#include <memory>
#include <string>
#include <type_traits>

#include "absl/base/attributes.h"
#include "absl/base/macros.h"
#include "mediapipe/framework/deps/registration.h"
#include "mediapipe/framework/packet_generator.pb.h"
#include "mediapipe/framework/packet_set.h"
#include "mediapipe/framework/packet_type.h"
#include "mediapipe/framework/port.h"
#include "mediapipe/framework/port/status.h"

namespace mediapipe {

// Pure virtual base class for packet generators.  These classes take any
// number of input side packet packets and produce some number of external
// output packets.  Those packets then become input side packets to other
// PacketGenerator's or to Calculators within the calculator graph.
//
// ***NOTE*** It is vital that the public interfaces for all classes
// included in packets be thread safe if the packet is meant
// to be used concurrently (e.g., with the PacketManager).
class PacketGenerator {
 public:
  PacketGenerator(const PacketGenerator&) = delete;
  PacketGenerator& operator=(const PacketGenerator&) = delete;
  virtual ~PacketGenerator() = 0;

  // All subclasses of PacketGenerator must implement two static functions with
  // the following signatures.  See FillExpectations() in calculator.h for an
  // explanation of that function.  Generate() must take the input side packets
  // and
  // produce output side packets.
  //
  // static absl::Status FillExpectations(
  //     const PacketGeneratorOptions& extendable_options,
  //     PacketTypeSet* input_side_packets,
  //     PacketTypeSet* output_side_packets);
  //
  // static absl::Status Generate(
  //     const PacketGeneratorOptions& extendable_options,
  //     const PacketSet& input_side_packets,
  //     PacketSet* output_side_packets);
};

// Details for the registration of a PacketGenerator follow.  A user of
// PacketGenerator does not need to know about the following code.
namespace internal {

// Gives access to the static functions within subclasses of PacketGenerator.
// This adds functionality akin to virtual static functions.
class StaticAccessToGenerator {
 public:
  virtual ~StaticAccessToGenerator() {}
  virtual absl::Status FillExpectations(
      const PacketGeneratorOptions& extendable_options,  //
      PacketTypeSet* input_side_packets,                 //
      PacketTypeSet* output_side_packets) = 0;
  virtual absl::Status Generate(
      const PacketGeneratorOptions& extendable_options,  //
      const PacketSet& input_side_packets,               //
      PacketSet* output_side_packets) = 0;
};

using StaticAccessToGeneratorRegistry =
    GlobalFactoryRegistry<std::unique_ptr<StaticAccessToGenerator>>;

// Functions for checking that the PacketGenerator has the proper
// functions defined.
template <class T>
constexpr bool PacketGeneratorHasFillExpectations(
    decltype(&T::FillExpectations) /*unused*/) {
  typedef absl::Status (*FillExpectationsType)(
      const PacketGeneratorOptions& extendable_options,  //
      PacketTypeSet* input_side_packets,                 //
      PacketTypeSet* output_side_packets);
  return std::is_same<decltype(&T::FillExpectations),
                      FillExpectationsType>::value;
}
template <class T>
constexpr bool PacketGeneratorHasFillExpectations(...) {
  return false;
}
template <class T>
constexpr bool PacketGeneratorHasGenerate(decltype(&T::Generate) /*unused*/) {
  typedef absl::Status (*GenerateType)(
      const PacketGeneratorOptions& extendable_options,  //
      const PacketSet& input_side_packets,               //
      PacketSet* output_side_packets);
  return std::is_same<decltype(&T::Generate), GenerateType>::value;
}
template <class T>
constexpr bool PacketGeneratorHasGenerate(...) {
  return false;
}

// Provides access to the static functions within a specific subclass
// of PacketGenerator.  See thee same mechanism in calculator.h for a
// more detailed explanation.
template <typename PacketGeneratorSubclass>
class StaticAccessToGeneratorTyped : public StaticAccessToGenerator {
 public:
  static_assert(std::is_base_of<mediapipe::PacketGenerator,
                                PacketGeneratorSubclass>::value,
                "Classes registered with REGISTER_PACKET_GENERATOR must be "
                "subclasses of mediapipe::PacketGenerator.");
  static_assert(
      PacketGeneratorHasFillExpectations<PacketGeneratorSubclass>(nullptr),
      "FillExpectations() must be defined with the correct signature in "
      "every PacketGenerator.");
  static_assert(PacketGeneratorHasGenerate<PacketGeneratorSubclass>(nullptr),
                "Generate() must be defined with the correct signature in "
                "every PacketGenerator.");

  absl::Status FillExpectations(
      const PacketGeneratorOptions& extendable_options,  //
      PacketTypeSet* input_side_packets,                 //
      PacketTypeSet* output_side_packets) final {
    return PacketGeneratorSubclass::FillExpectations(
        extendable_options, input_side_packets, output_side_packets);
  }

  absl::Status Generate(const PacketGeneratorOptions& extendable_options,  //
                        const PacketSet& input_side_packets,               //
                        PacketSet* output_side_packets) final {
    return PacketGeneratorSubclass::Generate(
        extendable_options, input_side_packets, output_side_packets);
  }
};

}  // namespace internal

// Macro for registering PacketGenerators.  It actually just registers
// the StaticAccessToGeneratorTyped class.
#define REGISTER_PACKET_GENERATOR(name)                     \
  REGISTER_FACTORY_FUNCTION_QUALIFIED(                      \
      mediapipe::internal::StaticAccessToGeneratorRegistry, \
      generator_registration, name,                         \
      std::make_unique<                                     \
          mediapipe::internal::StaticAccessToGeneratorTyped<name>>)

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_PACKET_GENERATOR_H_
