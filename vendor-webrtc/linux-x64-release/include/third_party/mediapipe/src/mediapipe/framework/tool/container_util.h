#ifndef MEDIAPIPE_FRAMEWORK_TOOL_CONTAINER_UTIL_H_
#define MEDIAPIPE_FRAMEWORK_TOOL_CONTAINER_UTIL_H_

#include "mediapipe/framework/calculator_framework.h"

namespace mediapipe {
namespace tool {

// Returns a tag name for one of the demux output channels.
// This is the channel number followed by the stream name separated by "__".
// For example, the channel-name for stream "FRAME" on channel 1 is "C1__FRAME".
std::string ChannelTag(const std::string& tag, int channel);

// Returns the set of tags directed to demux output channels.
// Each demux output tag is named using function ChannelTag().
// This function returns the demux input tags without the channel numbers.
std::set<std::string> ChannelTags(const std::shared_ptr<tool::TagMap>& map);

// Returns the number of demux output channels.
int ChannelCount(const std::shared_ptr<tool::TagMap>& map);

// Copies packet or timestamp bound from input to output stream.
void Relay(const InputStreamShard& input, OutputStreamShard* output);

// Returns the most recent specified channel index.
int GetChannelIndex(const CalculatorContext& cc, int previous_index);

}  // namespace tool
}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_TOOL_CONTAINER_UTIL_H_
