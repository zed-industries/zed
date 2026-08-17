#ifndef GOOGLE_PROTOBUF_COMPILER_KOTLIN_FIELD_H__
#define GOOGLE_PROTOBUF_COMPILER_KOTLIN_FIELD_H__

#include <string>

#include "absl/container/flat_hash_map.h"
#include "absl/strings/string_view.h"
#include "google/protobuf/compiler/java/context.h"
#include "google/protobuf/descriptor.h"
#include "google/protobuf/io/printer.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace kotlin {

class FieldGenerator {
 public:
  FieldGenerator(const FieldDescriptor* descriptor, java::Context* context,
                 bool lite);
  FieldGenerator(const FieldGenerator&) = delete;
  FieldGenerator& operator=(const FieldGenerator&) = delete;
  ~FieldGenerator() = default;

  void Generate(io::Printer* printer) const;

 private:
  const FieldDescriptor* descriptor_;
  absl::flat_hash_map<absl::string_view, std::string> variables_;
  java::Context* context_;
  bool lite_;

  void GeneratePritimiveField(io::Printer* printer) const;
  void GenerateMessageField(io::Printer* printer) const;
  void GenerateMapField(io::Printer* printer) const;
  void GenerateStringField(io::Printer* printer) const;
  void GenerateEnumField(io::Printer* printer) const;

  void GenerateRepeatedPritimiveField(io::Printer* printer) const;
  void GenerateRepeatedMessageField(io::Printer* printer) const;
  void GenerateRepeatedStringField(io::Printer* printer) const;
  void GenerateRepeatedEnumField(io::Printer* printer) const;
};

}  // namespace kotlin
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_KOTLIN_FIELD_H__
