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
// Defines CalculatorBase, the base class for feature computation.

#ifndef MEDIAPIPE_FRAMEWORK_CALCULATOR_BASE_H_
#define MEDIAPIPE_FRAMEWORK_CALCULATOR_BASE_H_

#include <memory>
#include <string>
#include <type_traits>

#include "absl/memory/memory.h"
#include "absl/status/status.h"
#include "mediapipe/framework/calculator_context.h"
#include "mediapipe/framework/calculator_contract.h"
#include "mediapipe/framework/deps/registration.h"
#include "mediapipe/framework/port.h"
#include "mediapipe/framework/timestamp.h"

namespace mediapipe {

// SOFT DEPRECATION: use mediapipe::api2::NodeIntf/Impl instead.
//
// Experimental: CalculatorBase will eventually replace Calculator as the
// base class of leaf (non-subgraph) nodes in a CalculatorGraph.
//
// The base calculator class.  A subclass must, at a minimum, provide the
// implementation of GetContract(), Process(), and register the calculator
// using REGISTER_CALCULATOR(MyClass).
//
// The framework calls four primary functions on a calculator.
// On initialization of the graph, a static function is called.
//   GetContract()
// Then, for each run of the graph on a set of input side packets, the
// following sequence will occur.
//   Open()
//   Process() (repeatedly)
//   Close()
//
// The entire calculator is constructed and destroyed for each graph run
// (set of input side packets, which could mean once per video, or once
// per image).  Any expensive operations and large objects should be
// input side packets.
//
// The framework calls Open() to initialize the calculator.
// If appropriate, Open() should call cc->SetOffset() or
// cc->Outputs().Get(id)->SetNextTimestampBound() to allow the framework to
// better optimize packet queueing.
//
// The framework calls Process() for every packet received on the input
// streams.  The framework guarantees that cc->InputTimestamp() will
// increase with every call to Process().  An empty packet will be on the
// input stream if there is no packet on a particular input stream (but
// some other input stream has a packet).
//
// The framework calls Close() after all calls to Process().
//
// Calculators with no inputs are referred to as "sources" and are handled
// slightly differently than non-sources (see the function comments for
// Process() for more details).
//
// Calculators must be thread-compatible.
// The framework does not call the non-const methods of a calculator from
// multiple threads at the same time.  However, the thread that calls the
// methods of a calculator is not fixed.  Therefore, calculators should not
// use ThreadLocal objects.
class CalculatorBase {
 public:
  CalculatorBase();
  virtual ~CalculatorBase();

  // The subclasses of CalculatorBase must implement GetContract.
  // The calculator cannot be registered without it.  Notice that although
  // this function is static the registration macro provides access to
  // each subclass' GetContract function.
  //
  // static absl::Status GetContract(CalculatorContract* cc);
  //
  // GetContract fills in the calculator's contract with the framework, such
  // as its expectations of what packets it will receive.  When this function
  // is called, the numbers of inputs, outputs, and input side packets will
  // have already been determined by the calculator graph.  You can use
  // indexes, tags, or tag:index to access input streams, output streams,
  // or input side packets.
  //
  // Example (uses tags for inputs and indexes for outputs and input side
  // packets):
  //   cc->Inputs().Tag("VIDEO").Set<ImageFrame>("Input Image Frames.");
  //   cc->Inputs().Tag("AUDIO").Set<Matrix>("Input Audio Frames.");
  //   cc->Outputs().Index(0).Set<Matrix>("Output FooBar feature.");
  //   cc->InputSidePackets().Index(0).Set<MyModel>(
  //       "Model used for FooBar feature extraction.");
  //
  // Example (same number and type of outputs as inputs):
  //   for (int i = 0; i < cc->Inputs().NumEntries(); ++i) {
  //     // SetAny() is used to specify that whatever the type of the
  //     // stream is, it's acceptable.  This does not mean that any
  //     // packet is acceptable.  Packets in the stream still have a
  //     // particular type.  SetAny() has the same effect as explicitly
  //     // setting the type to be the stream's type.
  //     cc->Inputs().Index(i).SetAny(StrCat("Generic Input Stream ", i));
  //     // Set each output to accept the same specific type as the
  //     // corresponding input.
  //     cc->Outputs().Index(i).SetSameAs(
  //         &cc->Inputs().Index(i), StrCat("Generic Output Stream ", i));
  //   }

  // Open is called before any Process() calls, on a freshly constructed
  // calculator.  Subclasses may override this method to perform necessary
  // setup, and possibly output Packets and/or set output streams' headers.
  // Must return absl::OkStatus() to indicate success. On failure any
  // other status code can be returned. If failure is returned then the
  // framework will call neither Process() nor Close() on the calculator (so any
  // necessary cleanup should be done before returning failure or in the
  // destructor).
  virtual absl::Status Open(CalculatorContext* cc) { return absl::OkStatus(); }

  // Processes the incoming inputs. May call the methods on cc to access
  // inputs and produce outputs.
  //
  // Process() called on a non-source node must return
  // absl::OkStatus() to indicate that all went well, or any other
  // status code to signal an error.
  // For example:
  //     absl::UnknownError("Failure Message");
  // Notice the convenience functions in util/task/canonical_errors.h .
  // If a non-source Calculator returns tool::StatusStop(), then this
  // signals the graph is being cancelled early.  In this case, all
  // source Calculators and graph input streams will be closed (and
  // remaining Packets will propagate through the graph).
  //
  // A source node will continue to have Process() called on it as long
  // as it returns absl::OkStatus().  To indicate that there is
  // no more data to be generated return tool::StatusStop().  Any other
  // status indicates an error has occurred.
  virtual absl::Status Process(CalculatorContext* cc) = 0;

  // Is called if Open() was called and succeeded.  Is called either
  // immediately after processing is complete or after a graph run has ended
  // (if an error occurred in the graph).  Must return absl::OkStatus()
  // to indicate success.  On failure any other status code can be returned.
  // Packets may be output during a call to Close().  However, output packets
  // are silently discarded if Close() is called after a graph run has ended.
  //
  // NOTE: Do not call cc->GraphStatus() in Close() if you need to check if the
  // processing is complete. Please, see CalculatorContext::GraphStatus
  // documentation for the suggested solution.
  virtual absl::Status Close(CalculatorContext* cc) { return absl::OkStatus(); }

  // Returns a value according to which the framework selects
  // the next source calculator to Process(); smaller value means
  // Process() first. The default implementation returns the smallest
  // NextTimestampBound value over all the output streams, but subclasses
  // may override this. If a calculator is not a source, this method is
  // not called.
  // TODO: Does this method need to be virtual? No Calculator
  // subclasses override the SourceProcessOrder method.
  virtual Timestamp SourceProcessOrder(const CalculatorContext* cc) const;
};

namespace api2 {
class Node;
}  // namespace api2

namespace internal {

// Gives access to the static functions within subclasses of CalculatorBase.
// This adds functionality akin to virtual static functions.
class CalculatorBaseFactory {
 public:
  virtual ~CalculatorBaseFactory() {}
  virtual absl::Status GetContract(CalculatorContract* cc) = 0;
  virtual std::unique_ptr<CalculatorBase> CreateCalculator(
      CalculatorContext* calculator_context) = 0;
  virtual std::string ContractMethodName() { return "GetContract"; }
};

// Functions for checking that the calculator has the required GetContract.
template <class T>
constexpr bool CalculatorHasGetContract(decltype(&T::GetContract) /*unused*/) {
  typedef absl::Status (*GetContractType)(CalculatorContract* cc);
  return std::is_same<decltype(&T::GetContract), GetContractType>::value;
}
template <class T>
constexpr bool CalculatorHasGetContract(...) {
  return false;
}

// Provides access to the static functions within a specific subclass
// of CalculatorBase.
template <class T, class Enable = void>
class CalculatorBaseFactoryFor : public CalculatorBaseFactory {
  static_assert(std::is_base_of<mediapipe::CalculatorBase, T>::value,
                "Classes registered with REGISTER_CALCULATOR must be "
                "subclasses of mediapipe::CalculatorBase.");
};

template <class T>
class CalculatorBaseFactoryFor<
    T,
    typename std::enable_if<std::is_base_of<mediapipe::CalculatorBase, T>{} &&
                            !std::is_base_of<mediapipe::api2::Node, T>{}>::type>
    : public CalculatorBaseFactory {
 public:
  static_assert(CalculatorHasGetContract<T>(nullptr),
                "GetContract() must be defined with the correct signature in "
                "every calculator.");

  // Provides access to the static function GetContract within a specific
  // subclass of CalculatorBase.
  absl::Status GetContract(CalculatorContract* cc) final {
    // CalculatorBaseSubclass must implement this function, since it is not
    // implemented in the parent class.
    return T::GetContract(cc);
  }

  std::unique_ptr<CalculatorBase> CreateCalculator(
      CalculatorContext* calculator_context) final {
    return absl::make_unique<T>();
  }
};

}  // namespace internal

using CalculatorBaseRegistry =
    GlobalFactoryRegistry<std::unique_ptr<internal::CalculatorBaseFactory>>;

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_CALCULATOR_BASE_H_
