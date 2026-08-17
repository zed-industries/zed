// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_CODEC_TRACE_NAMES_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_CODEC_TRACE_NAMES_H_

#include <string>

namespace blink {

// Holds the names of various codec operations for tracing. Uses std::string
// instead of WTF::String for ease of use with the tracing macros.
struct CodecTraceNames {
  explicit CodecTraceNames(const std::string& codec_name) {
    configure = codec_name + "::Configure";
    encode = codec_name + "::Encode";
    decode = codec_name + "::Decode";
    flush = codec_name + "::Flush";
    handle_error = codec_name + "::HandleError";
    output = codec_name + "::Ouput";
    reset = codec_name + "::OnCodecReclaimed";
    reset = codec_name + "::Reset";
    reconfigure = codec_name + "::Reconfigure";
    requests_counter = codec_name + " requests";
    shutdown = codec_name + "::Shutdown";
  }

  // Disallow copy and assign, because we should be using CodecTraceNames with
  // DEFINE_THREAD_SAFE_STATIC_LOCAL, and never accidentally copy it. Otherwise,
  // we end up with trace names pointing to destroyed strings.
  CodecTraceNames(const CodecTraceNames&) = delete;
  CodecTraceNames& operator=(const CodecTraceNames&) = delete;

  std::string configure;
  std::string encode;
  std::string decode;
  std::string flush;
  std::string handle_error;
  std::string output;
  std::string reclaimed;
  std::string reset;
  std::string reconfigure;
  std::string requests_counter;
  std::string shutdown;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBCODECS_CODEC_TRACE_NAMES_H_
