#ifndef GOOGLE_PROTOBUF_COMPILER_JAVA_SERVICE_COMMON_H__
#define GOOGLE_PROTOBUF_COMPILER_JAVA_SERVICE_COMMON_H__

#include <cstddef>
#include <memory>
#include <vector>

#include "absl/log/absl_check.h"
#include "absl/log/absl_log.h"
#include "google/protobuf/compiler/java/context.h"
#include "google/protobuf/compiler/java/name_resolver.h"
#include "google/protobuf/descriptor.h"
#include "google/protobuf/io/printer.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace java {

static const int kMaxStaticSize = 1 << 15;  // aka 32k

class FieldGenerator {
 public:
  virtual ~FieldGenerator() = default;
  virtual void GenerateSerializationCode(io::Printer* printer) const = 0;
};

// Convenience class which constructs FieldGenerators for a Descriptor.
template <typename FieldGeneratorType>
class FieldGeneratorMap {
 public:
  explicit FieldGeneratorMap(const Descriptor* descriptor)
      : descriptor_(descriptor) {
    field_generators_.reserve(static_cast<size_t>(descriptor->field_count()));
  }

  ~FieldGeneratorMap() {
    for (const auto* g : field_generators_) {
      delete g;
    }
  }

  FieldGeneratorMap(FieldGeneratorMap&&) = default;
  FieldGeneratorMap& operator=(FieldGeneratorMap&&) = default;

  FieldGeneratorMap(const FieldGeneratorMap&) = delete;
  FieldGeneratorMap& operator=(const FieldGeneratorMap&) = delete;

  void Add(const FieldDescriptor* field,
           std::unique_ptr<FieldGeneratorType> field_generator) {
    ABSL_CHECK_EQ(field->containing_type(), descriptor_);
    field_generators_.push_back(field_generator.release());
  }

  const FieldGeneratorType& get(const FieldDescriptor* field) const {
    ABSL_CHECK_EQ(field->containing_type(), descriptor_);
    return *field_generators_[static_cast<size_t>(field->index())];
  }

  std::vector<const FieldGenerator*> field_generators() const {
    std::vector<const FieldGenerator*> field_generators;
    field_generators.reserve(field_generators_.size());
    for (const auto* g : field_generators_) {
      field_generators.push_back(g);
    }
    return field_generators;
  }

 private:
  const Descriptor* descriptor_;
  std::vector<const FieldGeneratorType*> field_generators_;
};

inline void ReportUnexpectedPackedFieldsCall() {
  // Reaching here indicates a bug. Cases are:
  //   - This FieldGenerator should support packing,
  //     but this method should be overridden.
  //   - This FieldGenerator doesn't support packing, and this method
  //     should never have been called.
  ABSL_LOG(FATAL) << "GenerateBuilderParsingCodeFromPacked() "
                  << "called on field generator that does not support packing.";
}

}  // namespace java
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_JAVA_SERVICE_COMMON_H__
