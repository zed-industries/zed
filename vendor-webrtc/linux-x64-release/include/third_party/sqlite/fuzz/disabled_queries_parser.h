// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_SQLITE_FUZZ_DISABLED_QUERIES_PARSER_H_
#define THIRD_PARTY_SQLITE_FUZZ_DISABLED_QUERIES_PARSER_H_

#include <set>
#include <string>

namespace sql_fuzzer {
// |query_list| should be a list of disabled queries separated only by commas.
std::set<std::string> ParseDisabledQueries(std::string query_list);
}  // namespace sql_fuzzer

#endif  // THIRD_PARTY_SQLITE_FUZZ_DISABLED_QUERIES_PARSER_H_
