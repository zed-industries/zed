#ifndef MEDIAPIPE_TASKS_CC_TEXT_UTILS_VOCAB_CONVERT_UTILS_H_
#define MEDIAPIPE_TASKS_CC_TEXT_UTILS_VOCAB_CONVERT_UTILS_H_

#include <cstddef>
#include <string>
#include <vector>

#include "absl/container/node_hash_map.h"
#include "absl/status/status.h"

namespace mediapipe {
namespace tasks {
namespace text {

// Converts a HF tokenizer to SentencePiece model that can be loaded by internal
// SentencePiece library. Note that this script currently only works with BPE
// tokenizer and includes a unicode normalization.
//   hf_tokenizer: a directory that contains 'tokenizer.json' and
//     'tokenizer_config.json' files in it.
//   output_vocab_path: the path to the output vocabulary file.
absl::Status ConvertHfTokenizer(const std::string& hf_tokenizer,
                                const std::string& output_vocab_path);

}  // namespace text
}  // namespace tasks
}  // namespace mediapipe

#endif  // MEDIAPIPE_TASKS_CC_TEXT_UTILS_VOCAB_CONVERT_UTILS_H_
