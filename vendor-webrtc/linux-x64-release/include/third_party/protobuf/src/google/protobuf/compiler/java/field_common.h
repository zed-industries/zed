#ifndef GOOGLE_PROTOBUF_COMPILER_JAVA_FIELD_COMMON_H__
#define GOOGLE_PROTOBUF_COMPILER_JAVA_FIELD_COMMON_H__

#include <string>

#include "google/protobuf/compiler/java/options.h"
#include "google/protobuf/descriptor.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace java {

// Field information used in FieldGenerators.
struct FieldGeneratorInfo {
  std::string name;
  std::string capitalized_name;
  std::string disambiguated_reason;
  Options options;
};

// Oneof information used in OneofFieldGenerators.
struct OneofGeneratorInfo {
  std::string name;
  std::string capitalized_name;
};

// Set some common variables used in variable FieldGenerators.
void SetCommonFieldVariables(
    const FieldDescriptor* descriptor, const FieldGeneratorInfo* info,
    absl::flat_hash_map<absl::string_view, std::string>* variables);

// Set some common oneof variables used in OneofFieldGenerators.
void SetCommonOneofVariables(
    const FieldDescriptor* descriptor, const OneofGeneratorInfo* info,
    absl::flat_hash_map<absl::string_view, std::string>* variables);

// Print useful comments before a field's accessors.
void PrintExtraFieldInfo(
    const absl::flat_hash_map<absl::string_view, std::string>& variables,
    io::Printer* printer);

// Returns the name by which the generated Java getters and setters should be
// referenced from Kotlin as properties. In the simplest case, the original name
// is something like `foo_bar`, which gets translated into `getFooBar()` etc,
// and that in turn can be referenced from Kotlin as `fooBar`.
//
// The algorithm for translating proto names into Java getters and setters is
// straightforward. The first letter of each underscore-separated word gets
// uppercased and the underscores are deleted. There are no other changes, so in
// particular if the proto name has a string of capitals then those remain
// as-is.
//
// The algorithm that the Kotlin compiler uses to derive the property name is
// slightly more complicated. If the first character after `get` (etc) is a
// capital and the second isn't, then the property name is just that string with
// its first letter lowercased. So `getFoo` becomes `foo` and `getX` becomes
// `x`. But if there is more than one capital, then all but the last get
// lowercased. So `getHTMLPage` becomes `htmlPage`. If there are only capitals
// then they all get lowercased, so `getID` becomes `id`.
// TODO: move this to a Kotlin-specific location
std::string GetKotlinPropertyName(std::string capitalized_name);

}  // namespace java
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_JAVA_FIELD_COMMON_H__
