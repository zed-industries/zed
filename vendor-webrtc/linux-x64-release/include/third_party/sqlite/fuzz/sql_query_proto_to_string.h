// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_SQLITE_FUZZ_SQL_QUERY_PROTO_TO_STRING_H_
#define THIRD_PARTY_SQLITE_FUZZ_SQL_QUERY_PROTO_TO_STRING_H_

#include <string>
#include <vector>

#include "third_party/sqlite/fuzz/sql_queries.pb.h"  // IWYU pragma: export

namespace sql_fuzzer {

std::string SQLQueriesToString(
    const sql_query_grammar::SQLQueries& sql_queries);
std::vector<std::string> SQLQueriesToVec(
    const sql_query_grammar::SQLQueries& sql_queries);

std::string PrintfToString(const sql_query_grammar::Printf&);
std::string StrftimeFnToString(const sql_query_grammar::StrftimeFn&);
std::string ExprToString(const sql_query_grammar::Expr&);

std::string SQLQueryToString(const sql_query_grammar::SQLQuery&);

void SetDisabledQueries(std::set<std::string> disabled_queries);

}  // namespace sql_fuzzer

#endif  // THIRD_PARTY_SQLITE_FUZZ_SQL_QUERY_PROTO_TO_STRING_H_
