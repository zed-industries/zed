#ifndef GOOGLE_PROTOBUF_COMPILER_JAVA_LITE_FIELD_GENERATOR_H__
#define GOOGLE_PROTOBUF_COMPILER_JAVA_LITE_FIELD_GENERATOR_H__

#include <string>

#include "absl/log/absl_log.h"
#include "google/protobuf/compiler/java/generator_common.h"
#include "google/protobuf/io/printer.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace java {

class ImmutableFieldLiteGenerator : public FieldGenerator {
 public:
  ImmutableFieldLiteGenerator() = default;
  ImmutableFieldLiteGenerator(const ImmutableFieldLiteGenerator&) = delete;
  ImmutableFieldLiteGenerator& operator=(const ImmutableFieldLiteGenerator&) =
      delete;
  virtual ~ImmutableFieldLiteGenerator() = default;

  virtual int GetNumBitsForMessage() const = 0;
  virtual void GenerateInterfaceMembers(io::Printer* printer) const = 0;
  virtual void GenerateMembers(io::Printer* printer) const = 0;
  virtual void GenerateBuilderMembers(io::Printer* printer) const = 0;
  virtual void GenerateInitializationCode(io::Printer* printer) const = 0;
  virtual void GenerateFieldInfo(io::Printer* printer,
                                 std::vector<uint16_t>* output) const = 0;

  void GenerateSerializationCode(io::Printer* printer) const override {
    ABSL_LOG(FATAL) << "GenerateSerializationCode is unsupported for lite.";
  }

  virtual std::string GetBoxedType() const = 0;
};

}  // namespace java
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_JAVA_LITE_FIELD_GENERATOR_H__
