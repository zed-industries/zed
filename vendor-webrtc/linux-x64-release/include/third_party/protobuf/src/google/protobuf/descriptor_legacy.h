#ifndef GOOGLE_PROTOBUF_DESCRIPTOR_EDITION_H__
#define GOOGLE_PROTOBUF_DESCRIPTOR_EDITION_H__

#include "google/protobuf/descriptor.h"
#include "google/protobuf/descriptor.pb.h"

namespace google {
namespace protobuf {

// TODO Remove this deprecated API entirely.
class FileDescriptorLegacy {
 public:
  explicit FileDescriptorLegacy(const FileDescriptor* file) : file_(file) {}

  // Edition shouldn't be depended on unless dealing with raw unbuilt
  // descriptors, which will expose it via FileDescriptorProto.edition.
  Edition edition() const { return file_->edition(); }

 private:
  const FileDescriptor* file_;
};
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_DESCRIPTOR_EDITION_H__
