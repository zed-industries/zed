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

#ifndef SRC_TRACE_PROCESSOR_PERFETTO_SQL_PARSER_PERFETTO_SQL_PARSER_H_
#define SRC_TRACE_PROCESSOR_PERFETTO_SQL_PARSER_PERFETTO_SQL_PARSER_H_

#include <cstdint>
#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <variant>
#include <vector>

#include "perfetto/base/logging.h"
#include "perfetto/ext/base/flat_hash_map.h"
#include "src/trace_processor/perfetto_sql/grammar/perfettosql_grammar_interface.h"
#include "src/trace_processor/perfetto_sql/parser/function_util.h"
#include "src/trace_processor/perfetto_sql/preprocessor/perfetto_sql_preprocessor.h"
#include "src/trace_processor/sqlite/sql_source.h"
#include "src/trace_processor/util/sql_argument.h"

namespace perfetto::trace_processor {

// Parser for PerfettoSQL statements. This class provides an iterator-style
// interface for reading all PerfettoSQL statements from a block of SQL.
//
// Usage:
// PerfettoSqlParser parser(my_sql_string.c_str());
// while (parser.Next()) {
//   auto& stmt = parser.statement();
//   // Handle |stmt| here
// }
// RETURN_IF_ERROR(r.status());
class PerfettoSqlParser {
 public:
  // Indicates that the specified SQLite SQL was extracted directly from a
  // PerfettoSQL statement and should be directly executed with SQLite.
  struct SqliteSql {};
  // Indicates that the specified SQL was a CREATE PERFETTO FUNCTION statement
  // with the following parameters.
  struct CreateFunction {
    struct Returns {
      bool is_table;
      // Only set when `is_table` is false.
      sql_argument::Type scalar_type;
      // Only set when `is_table` is true.
      std::vector<sql_argument::ArgumentDefinition> table_columns;
    };
    bool replace;
    FunctionPrototype prototype;
    Returns returns;
    SqlSource sql;
    std::string description;
  };
  // Indicates that the specified SQL was a CREATE PERFETTO TABLE statement
  // with the following parameters.
  struct CreateTable {
    enum Implementation : uint8_t {
      kRuntimeTable,
      kDataframe,
    };
    bool replace;
    std::string name;
    Implementation implementation;
    std::vector<sql_argument::ArgumentDefinition> schema;
    // SQL source for the select statement.
    SqlSource sql;
  };
  // Indicates that the specified SQL was a CREATE PERFETTO VIEW statement
  // with the following parameters.
  struct CreateView {
    bool replace;
    std::string name;
    std::vector<sql_argument::ArgumentDefinition> schema;
    // SQL source for the select statement.
    SqlSource sql;
    // SQL source for the CREATE VIEW statement.
    SqlSource create_view_sql;
  };
  // Indicates that the specified SQL was a CREATE PERFETTO INDEX statement
  // with the following parameters.
  struct CreateIndex {
    bool replace;
    std::string name;
    std::string table_name;
    std::vector<std::string> col_names;
  };
  // Indicates that the specified SQL was a DROP PERFETTO INDEX statement
  // with the following parameters.
  struct DropIndex {
    std::string name;
    std::string table_name;
  };
  // Indicates that the specified SQL was a INCLUDE PERFETTO MODULE statement
  // with the following parameter.
  struct Include {
    std::string key;
  };
  // Indicates that the specified SQL was a CREATE PERFETTO MACRO statement
  // with the following parameter.
  struct CreateMacro {
    bool replace;
    SqlSource name;
    std::vector<std::pair<SqlSource, SqlSource>> args;
    SqlSource returns;
    SqlSource sql;
  };

  using Statement = std::variant<CreateFunction,
                                 CreateIndex,
                                 CreateMacro,
                                 CreateTable,
                                 CreateView,
                                 DropIndex,
                                 Include,
                                 SqliteSql>;

  // Creates a new SQL parser with the a block of PerfettoSQL statements.
  // Concretely, the passed string can contain >1 statement.
  explicit PerfettoSqlParser(
      SqlSource,
      const base::FlatHashMap<std::string, PerfettoSqlPreprocessor::Macro>&);

  ~PerfettoSqlParser();

  PerfettoSqlParser(const PerfettoSqlParser&) = delete;
  PerfettoSqlParser& operator=(const PerfettoSqlParser&) = delete;

  PerfettoSqlParser(PerfettoSqlParser&&) = delete;
  PerfettoSqlParser& operator=(PerfettoSqlParser&&) = delete;

  // Attempts to parse to the next statement in the SQL. Returns true if
  // a statement was successfully parsed and false if EOF was reached or the
  // statement was not parsed correctly.
  //
  // Note: if this function returns false, callers *must* call |status()|: it
  // is undefined behaviour to not do so.
  bool Next();

  // Returns the current statement which was parsed. This function *must not* be
  // called unless |Next()| returned true.
  const Statement& statement() const;

  // Returns the full statement which was parsed. This should return
  // |statement()| and Perfetto SQL code that's in front. This function *must
  // not* be called unless |Next()| returned true.
  const SqlSource& statement_sql() const {
    PERFETTO_CHECK(statement_sql_);
    return *statement_sql_;
  }

  // Returns the error status for the parser. This will be |base::OkStatus()|
  // until an unrecoverable error is encountered.
  const base::Status& status() const;

 private:
  std::unique_ptr<PerfettoSqlParserState> parser_state_;
  std::optional<SqlSource> statement_sql_;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_PERFETTO_SQL_PARSER_PERFETTO_SQL_PARSER_H_
