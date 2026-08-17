#ifndef MEDIAPIPE_FRAMEWORK_TOOL_OPTIONS_SYNTAX_UTIL_H_
#define MEDIAPIPE_FRAMEWORK_TOOL_OPTIONS_SYNTAX_UTIL_H_

#include <memory>
#include <string>
#include <vector>

#include "mediapipe/framework/calculator.pb.h"
#include "mediapipe/framework/packet.h"
#include "mediapipe/framework/packet_type.h"
#include "mediapipe/framework/port/advanced_proto_inc.h"
#include "mediapipe/framework/port/proto_ns.h"
#include "mediapipe/framework/tool/options_field_util.h"
#include "mediapipe/framework/tool/options_registry.h"

namespace mediapipe {
namespace tool {

// Utility to parse the graph options syntax used in "option_value",
// "side_packet", and "stream".
class OptionsSyntaxUtil {
 public:
  using FieldPath = options_field_util::FieldPath;
  OptionsSyntaxUtil();
  OptionsSyntaxUtil(const std::string& tag_name);
  OptionsSyntaxUtil(const std::string& tag_name, const std::string& packet_name,
                    const std::string& separator);
  ~OptionsSyntaxUtil();

  // Converts slash-separated field names into a tag name.
  std::string OptionFieldsTag(absl::string_view option_names);

  // Returns the field-path for an option stream-tag.
  FieldPath OptionFieldPath(absl::string_view tag,
                            const Descriptor* descriptor);

  // Splits a string into "tag" and "name" delimited by a single colon.
  std::vector<absl::string_view> StrSplitTags(absl::string_view tag_and_name);

 private:
  class OptionsSyntaxHelper;
  std::unique_ptr<OptionsSyntaxHelper> syntax_helper_;
};

}  // namespace tool
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_TOOL_OPTIONS_SYNTAX_UTIL_H_
