#ifndef GOOGLE_PROTOBUF_COMPILER_JAVA_IMMUTABLE_GENERATOR_FACTORY_H__
#define GOOGLE_PROTOBUF_COMPILER_JAVA_IMMUTABLE_GENERATOR_FACTORY_H__

#include "google/protobuf/compiler/java/generator_factory.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace java {

std::unique_ptr<GeneratorFactory> MakeImmutableGeneratorFactory(
    Context* context);

}  // namespace java
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_JAVA_IMMUTABLE_GENERATOR_FACTORY_H__
