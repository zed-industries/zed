// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TESTING_LIBFUZZER_FUZZERS_JAVASCRIPT_PARSER_PROTO_TO_STRING_H_
#define TESTING_LIBFUZZER_FUZZERS_JAVASCRIPT_PARSER_PROTO_TO_STRING_H_

#include "javascript_parser.pb.h"  // from out/gen

#include <string>

std::string token_to_string(const javascript_parser_proto_fuzzer::Token& token,
                            int depth);

#endif  // TESTING_LIBFUZZER_FUZZERS_JAVASCRIPT_PARSER_PROTO_TO_STRING_H_
