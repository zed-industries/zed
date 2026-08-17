#ifndef MEDIAPIPE_CALCULATORS_UTIL_RESOURCE_PROVIDER_CALCULATOR_H_
#define MEDIAPIPE_CALCULATORS_UTIL_RESOURCE_PROVIDER_CALCULATOR_H_

#include <string>

#include "absl/status/status.h"
#include "mediapipe/framework/api2/node.h"
#include "mediapipe/framework/api2/port.h"
#include "mediapipe/framework/calculator_framework.h"
#include "mediapipe/framework/resources.h"
namespace mediapipe::api2 {

// The calculator takes resource id (e.g. file path) as input side packet or
// calculator options and provides the corresponding resource.
//
// NOTE: calculator supports loading multiple resources.
//
// Example config:
//
// node {
//   calculator: "ResourceProviderCalculator"
//   output_side_packet: "RESOURCE:0:resource0"
//   output_side_packet: "RESOURCE:1:resource1"
//   node_options {
//     [type.googleapis.com/mediapipe.ResourceProviderCalculatorOptions]: {
//        resource_id: "path/to/resource0"
//        resource_id: "path/to/resource1"
//     }
//   }
// }
//
// node {
//   calculator: "ResourceProviderCalculator"
//   input_side_packet: "RESOURCE_ID:resource_id"
//   output_side_packet: "RESOURCE:resource"
// }
//
// node {
//   calculator: "ResourceProviderCalculator"
//   input_side_packet: "RESOURCE_ID:0:resource_id0"
//   input_side_packet: "RESOURCE_ID:1:resource_id1"
//   ...
//   output_side_packet: "RESOURCE:0:resource0"
//   output_side_packet: "RESOURCE:1:resource1"
//   ...
// }
//
class ResourceProviderCalculator : public mediapipe::api2::Node {
 public:
  static constexpr api2::SideInput<std::string>::Multiple kIds{"RESOURCE_ID"};
  static constexpr api2::SideOutput<Resource>::Multiple kResources{"RESOURCE"};

  MEDIAPIPE_NODE_INTERFACE(ResourceProviderCalculator, kIds, kResources);

  static absl::Status UpdateContract(CalculatorContext* cc);

  absl::Status Open(CalculatorContext* cc) override;

  absl::Status Process(CalculatorContext* cc) override {
    return absl::OkStatus();
  }
};

}  // namespace mediapipe::api2

#endif  // MEDIAPIPE_CALCULATORS_UTIL_RESOURCE_PROVIDER_CALCULATOR_H_
