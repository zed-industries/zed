#ifndef GOOGLE_PROTOBUF_UNREDACTED_DEBUG_FORMAT_FOR_TEST_H__
#define GOOGLE_PROTOBUF_UNREDACTED_DEBUG_FORMAT_FOR_TEST_H__

#include "google/protobuf/message.h"
#include "google/protobuf/message_lite.h"

namespace google {
namespace protobuf {
namespace util {

// Generates a human-readable form of this message for debugging purposes in
// test-only code. This API does not redact any fields in the message.
std::string UnredactedDebugFormatForTest(const google::protobuf::Message& message);
// Like UnredactedDebugFormatForTest(), but prints the message in a single line.
std::string UnredactedShortDebugFormatForTest(const google::protobuf::Message& message);
// Like UnredactedDebugFormatForTest(), but does not escape UTF-8 byte
// sequences.
std::string UnredactedUtf8DebugFormatForTest(const google::protobuf::Message& message);

// The following APIs are added just to work with code that interoperates with
// `Message` and `MessageLite`.

std::string UnredactedDebugFormatForTest(const google::protobuf::MessageLite& message);
std::string UnredactedShortDebugFormatForTest(
    const google::protobuf::MessageLite& message);
std::string UnredactedUtf8DebugFormatForTest(
    const google::protobuf::MessageLite& message);

}  // namespace util
}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_UNREDACTED_DEBUG_FORMAT_FOR_TEST_H__
