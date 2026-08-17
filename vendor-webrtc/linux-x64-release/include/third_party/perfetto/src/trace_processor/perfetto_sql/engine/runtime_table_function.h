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

#ifndef SRC_TRACE_PROCESSOR_PERFETTO_SQL_ENGINE_RUNTIME_TABLE_FUNCTION_H_
#define SRC_TRACE_PROCESSOR_PERFETTO_SQL_ENGINE_RUNTIME_TABLE_FUNCTION_H_

#include <cstddef>
#include <cstdint>
#include <memory>
#include <optional>
#include <vector>

#include "perfetto/base/logging.h"
#include "src/trace_processor/perfetto_sql/parser/function_util.h"
#include "src/trace_processor/sqlite/bindings/sqlite_module.h"
#include "src/trace_processor/sqlite/module_state_manager.h"
#include "src/trace_processor/sqlite/sql_source.h"
#include "src/trace_processor/sqlite/sqlite_engine.h"
#include "src/trace_processor/util/sql_argument.h"

namespace perfetto::trace_processor {

class PerfettoSqlEngine;

// The implementation of the SqliteTableLegacy interface for table functions
// defined at runtime using SQL.
struct RuntimeTableFunctionModule
    : public sqlite::Module<RuntimeTableFunctionModule> {
  struct State {
    PerfettoSqlEngine* engine;
    SqlSource sql_defn_str;

    FunctionPrototype prototype;
    std::vector<sql_argument::ArgumentDefinition> return_values;

    std::optional<SqliteEngine::PreparedStatement> temporary_create_stmt;

    bool IsReturnValueColumn(size_t i) const {
      PERFETTO_DCHECK(i < TotalColumnCount());
      return i < return_values.size();
    }

    bool IsArgumentColumn(size_t i) const {
      PERFETTO_DCHECK(i < TotalColumnCount());
      return i >= return_values.size() &&
             (i - return_values.size()) < prototype.arguments.size();
    }

    bool IsPrimaryKeyColumn(size_t i) const {
      PERFETTO_DCHECK(i < TotalColumnCount());
      return i == (return_values.size() + prototype.arguments.size());
    }

    size_t TotalColumnCount() const {
      static constexpr uint32_t kPrimaryKeyColumns = 1;
      return prototype.arguments.size() + return_values.size() +
             kPrimaryKeyColumns;
    }
  };
  struct Context : sqlite::ModuleStateManager<RuntimeTableFunctionModule> {
    std::unique_ptr<State> temporary_create_state;
  };
  struct Vtab : sqlite::Module<RuntimeTableFunctionModule>::Vtab {
    sqlite::ModuleStateManager<RuntimeTableFunctionModule>::PerVtabState* state;
    std::optional<SqliteEngine::PreparedStatement> reusable_stmt;
  };
  struct Cursor : sqlite::Module<RuntimeTableFunctionModule>::Cursor {
    std::optional<SqliteEngine::PreparedStatement> stmt;
    bool is_eof = false;
    int next_call_count = 0;
  };

  static constexpr bool kSupportsWrites = false;
  static constexpr bool kDoesOverloadFunctions = false;

  static int Create(sqlite3*,
                    void*,
                    int,
                    const char* const*,
                    sqlite3_vtab**,
                    char**);
  static int Destroy(sqlite3_vtab*);

  static int Connect(sqlite3*,
                     void*,
                     int,
                     const char* const*,
                     sqlite3_vtab**,
                     char**);
  static int Disconnect(sqlite3_vtab*);

  static int BestIndex(sqlite3_vtab*, sqlite3_index_info*);

  static int Open(sqlite3_vtab*, sqlite3_vtab_cursor**);
  static int Close(sqlite3_vtab_cursor*);

  static int Filter(sqlite3_vtab_cursor*,
                    int,
                    const char*,
                    int,
                    sqlite3_value**);
  static int Next(sqlite3_vtab_cursor*);
  static int Eof(sqlite3_vtab_cursor*);
  static int Column(sqlite3_vtab_cursor*, sqlite3_context*, int);
  static int Rowid(sqlite3_vtab_cursor*, sqlite_int64*);

  // This needs to happen at the end as it depends on the functions
  // defined above.
  static constexpr sqlite3_module kModule = CreateModule();
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_PERFETTO_SQL_ENGINE_RUNTIME_TABLE_FUNCTION_H_
