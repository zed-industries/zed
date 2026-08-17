/*
 * Copyright (C) 2023 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_TRACE_PROCESSOR_PERFETTO_SQL_PARSER_PERFETTO_SQL_TEST_UTILS_H_
#define SRC_TRACE_PROCESSOR_PERFETTO_SQL_PARSER_PERFETTO_SQL_TEST_UTILS_H_

#include <cstddef>
#include <cstdint>
#include <ostream>
#include <string>
#include <tuple>
#include <variant>

#include "perfetto/base/logging.h"
#include "perfetto/ext/base/status_or.h"
#include "src/trace_processor/perfetto_sql/parser/function_util.h"
#include "src/trace_processor/perfetto_sql/parser/perfetto_sql_parser.h"
#include "src/trace_processor/sqlite/sql_source.h"
#include "src/trace_processor/util/sql_argument.h"
#include "test/gtest_and_gmock.h"

namespace perfetto::trace_processor {

namespace sql_argument {

inline std::ostream& operator<<(std::ostream& stream, const Type& type) {
  return stream << TypeToHumanFriendlyString(type);
}

}  // namespace sql_argument

inline bool operator==(const SqlSource& a, const SqlSource& b) {
  return a.sql() == b.sql();
}

inline bool operator==(const PerfettoSqlParser::SqliteSql&,
                       const PerfettoSqlParser::SqliteSql&) {
  return true;
}

inline bool operator==(const PerfettoSqlParser::CreateFunction::Returns& a,
                       const PerfettoSqlParser::CreateFunction::Returns& b) {
  return std::tie(a.is_table, a.scalar_type, a.table_columns) ==
         std::tie(b.is_table, b.scalar_type, b.table_columns);
}

inline bool operator==(const PerfettoSqlParser::CreateFunction& a,
                       const PerfettoSqlParser::CreateFunction& b) {
  return std::tie(a.returns, a.prototype, a.replace, a.sql, a.description) ==
         std::tie(b.returns, b.prototype, b.replace, b.sql, b.description);
}

inline bool operator==(const PerfettoSqlParser::CreateTable& a,
                       const PerfettoSqlParser::CreateTable& b) {
  return std::tie(a.name, a.sql) == std::tie(b.name, b.sql);
}

inline bool operator==(const PerfettoSqlParser::CreateView& a,
                       const PerfettoSqlParser::CreateView& b) {
  return std::tie(a.name, a.create_view_sql) ==
         std::tie(b.name, b.create_view_sql);
}

inline bool operator==(const PerfettoSqlParser::Include& a,
                       const PerfettoSqlParser::Include& b) {
  return std::tie(a.key) == std::tie(b.key);
}

constexpr bool operator==(const PerfettoSqlParser::CreateMacro& a,
                          const PerfettoSqlParser::CreateMacro& b) {
  return std::tie(a.replace, a.name, a.sql, a.args) ==
         std::tie(b.replace, b.name, b.sql, b.args);
}

constexpr bool operator==(const PerfettoSqlParser::CreateIndex& a,
                          const PerfettoSqlParser::CreateIndex& b) {
  return std::tie(a.replace, a.name, a.table_name, a.col_names) ==
         std::tie(b.replace, b.name, b.table_name, b.col_names);
}

constexpr bool operator==(const PerfettoSqlParser::DropIndex& a,
                          const PerfettoSqlParser::DropIndex& b) {
  return std::tie(a.name, a.table_name) == std::tie(b.name, b.table_name);
}

inline std::ostream& operator<<(std::ostream& stream, const SqlSource& sql) {
  return stream << "SqlSource(sql=" << testing::PrintToString(sql.sql()) << ")";
}

inline std::ostream& operator<<(std::ostream& stream,
                                const FunctionPrototype& ret) {
  return stream << "Prototype(name="
                << testing::PrintToString(ret.function_name)
                << ", arguments=" << testing::PrintToString(ret.arguments)
                << ")";
}

inline std::ostream& operator<<(
    std::ostream& stream,
    const PerfettoSqlParser::CreateFunction::Returns& ret) {
  return stream << "Returns(is_table=" << testing::PrintToString(ret.is_table)
                << ", scalar_type=" << testing::PrintToString(ret.scalar_type)
                << ", table_columns="
                << testing::PrintToString(ret.table_columns) << ")";
}

inline std::ostream& operator<<(std::ostream& stream,
                                const PerfettoSqlParser::Statement& line) {
  if (std::get_if<PerfettoSqlParser::SqliteSql>(&line)) {
    return stream << "SqliteSql()";
  }
  if (const auto* fn = std::get_if<PerfettoSqlParser::CreateFunction>(&line)) {
    return stream << "CreateFn(sql=" << testing::PrintToString(fn->sql)
                  << ", prototype=" << testing::PrintToString(fn->prototype)
                  << ", returns=" << testing::PrintToString(fn->returns)
                  << ", replace=" << testing::PrintToString(fn->replace) << ")";
  }
  if (const auto* tab = std::get_if<PerfettoSqlParser::CreateTable>(&line)) {
    return stream << "CreateTable(name=" << testing::PrintToString(tab->name)
                  << ", sql=" << testing::PrintToString(tab->sql) << ")";
  }
  if (const auto* tab = std::get_if<PerfettoSqlParser::CreateView>(&line)) {
    return stream << "CreateView(name=" << testing::PrintToString(tab->name)
                  << ", sql=" << testing::PrintToString(tab->create_view_sql)
                  << ")";
  }
  if (const auto* macro = std::get_if<PerfettoSqlParser::CreateMacro>(&line)) {
    return stream << "CreateTable(name=" << testing::PrintToString(macro->name)
                  << ", args=" << testing::PrintToString(macro->args)
                  << ", replace=" << testing::PrintToString(macro->replace)
                  << ", sql=" << testing::PrintToString(macro->sql) << ")";
  }
  PERFETTO_FATAL("Unknown type");
}

template <typename T>
inline bool operator==(const base::StatusOr<T>& a, const base::StatusOr<T>& b) {
  return a.status().ok() == b.ok() &&
         a.status().message() == b.status().message() &&
         (!a.ok() || a.value() == b.value());
}

inline std::ostream& operator<<(std::ostream& stream, const base::Status& a) {
  return stream << "base::Status(ok=" << a.ok()
                << ", message=" << testing::PrintToString(a.message()) << ")";
}

template <typename T>
inline std::ostream& operator<<(std::ostream& stream,
                                const base::StatusOr<T>& a) {
  std::string val = a.ok() ? testing::PrintToString(a.value()) : "";
  return stream << "base::StatusOr(status="
                << testing::PrintToString(a.status()) << ", value=" << val
                << ")";
}

inline SqlSource FindSubstr(const SqlSource& source,
                            const std::string& needle) {
  size_t off = source.sql().find(needle);
  PERFETTO_CHECK(off != std::string::npos);
  return source.Substr(static_cast<uint32_t>(off),
                       static_cast<uint32_t>(needle.size()));
}

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_PERFETTO_SQL_PARSER_PERFETTO_SQL_TEST_UTILS_H_
