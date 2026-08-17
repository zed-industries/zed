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

#ifndef SRC_TRACE_PROCESSOR_PERFETTO_SQL_PREPROCESSOR_PERFETTO_SQL_PREPROCESSOR_H_
#define SRC_TRACE_PROCESSOR_PERFETTO_SQL_PREPROCESSOR_PERFETTO_SQL_PREPROCESSOR_H_

#include <optional>
#include <string>
#include <unordered_set>
#include <vector>

#include "perfetto/base/status.h"
#include "perfetto/ext/base/flat_hash_map.h"
#include "src/trace_processor/perfetto_sql/tokenizer/sqlite_tokenizer.h"
#include "src/trace_processor/sqlite/sql_source.h"

namespace perfetto::trace_processor {

// Preprocessor for PerfettoSQL statements. The main responsibility of this
// class is to perform similar functions to the C/C++ preprocessor (e.g.
// expanding macros). It is also responsible for splitting the given SQL into
// statements.
class PerfettoSqlPreprocessor {
 public:
  struct Macro {
    bool replace;
    std::string name;
    std::vector<std::string> args;
    SqlSource sql;
  };

  // Creates a preprocessor acting on the given SqlSource.
  explicit PerfettoSqlPreprocessor(
      SqlSource,
      const base::FlatHashMap<std::string, Macro>&);

  // Preprocesses the next SQL statement. Returns true if a statement was
  // successfully preprocessed and false if EOF was reached or the statement was
  // not preprocessed correctly.
  //
  // Note: if this function returns false, callers *must* call |status()|: it
  // is undefined behaviour to not do so.
  bool NextStatement();

  // Returns the error status for the parser. This will be |base::OkStatus()|
  // until an unrecoverable error is encountered.
  const base::Status& status() const { return status_; }

  // Returns the most-recent preprocessed SQL statement.
  //
  // Note: this function must not be called unless |NextStatement()| returned
  // true.
  SqlSource& statement() { return *statement_; }

 private:
  SqliteTokenizer global_tokenizer_;
  const base::FlatHashMap<std::string, Macro>* macros_ = nullptr;
  std::unordered_set<std::string> seen_macros_;
  std::optional<SqlSource> statement_;
  base::Status status_;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_PERFETTO_SQL_PREPROCESSOR_PERFETTO_SQL_PREPROCESSOR_H_
