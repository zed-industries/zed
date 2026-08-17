#ifndef GOOGLE_PROTOBUF_COMPILER_RUST_RUST_FIELD_TYPE_H__
#define GOOGLE_PROTOBUF_COMPILER_RUST_RUST_FIELD_TYPE_H__

#include "google/protobuf/descriptor.h"

namespace google {
namespace protobuf {
namespace compiler {
namespace rust {

// An enum of all of the singular types as they should be seen by Rust. This
// is parallel to FieldDescriptor::CppType with the main difference being that
// that String and Bytes are treated as different types.
enum class RustFieldType {
  INT32,
  INT64,
  UINT32,
  UINT64,
  DOUBLE,
  FLOAT,
  BOOL,
  ENUM,
  STRING,
  BYTES,
  MESSAGE,
};

// Note: for 'repeated X field' this returns the corresponding type of X.
// For map fields this returns MESSAGE.
RustFieldType GetRustFieldType(const FieldDescriptor& field);

RustFieldType GetRustFieldType(FieldDescriptor::Type type);

}  // namespace rust
}  // namespace compiler
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_COMPILER_RUST_RUST_FIELD_TYPE_H__
