/*
 * Copyright (C) 2022 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_FUNCTIONS_UTILS_H_
#define SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_FUNCTIONS_UTILS_H_

#include <sqlite3.h>
#include <unordered_map>

#include "perfetto/base/compiler.h"
#include "perfetto/ext/base/base64.h"
#include "perfetto/ext/base/file_utils.h"
#include "perfetto/ext/base/string_utils.h"
#include "perfetto/ext/trace_processor/demangle.h"
#include "protos/perfetto/common/builtin_clock.pbzero.h"
#include "src/trace_processor/db/column/utils.h"
#include "src/trace_processor/export_json.h"
#include "src/trace_processor/importers/common/clock_tracker.h"
#include "src/trace_processor/perfetto_sql/intrinsics/functions/sql_function.h"
#include "src/trace_processor/sqlite/sqlite_utils.h"
#include "src/trace_processor/util/regex.h"
#include "src/trace_processor/util/status_macros.h"

namespace perfetto {
namespace trace_processor {

struct ExportJson : public SqlFunction {
  using Context = TraceStorage;
  static base::Status Run(TraceStorage* storage,
                          size_t /*argc*/,
                          sqlite3_value** argv,
                          SqlValue& /*out*/,
                          Destructors&);
};

base::Status ExportJson::Run(TraceStorage* storage,
                             size_t /*argc*/,
                             sqlite3_value** argv,
                             SqlValue& /*out*/,
                             Destructors&) {
  base::ScopedFstream output;
  if (sqlite3_value_type(argv[0]) == SQLITE_INTEGER) {
    // Assume input is an FD.
    output.reset(fdopen(sqlite3_value_int(argv[0]), "w"));
    if (!output) {
      return base::ErrStatus(
          "EXPORT_JSON: Couldn't open output file from given FD");
    }
  } else {
    const char* filename =
        reinterpret_cast<const char*>(sqlite3_value_text(argv[0]));
    output = base::OpenFstream(filename, "w");
    if (!output) {
      return base::ErrStatus("EXPORT_JSON: Couldn't open output file");
    }
  }
  return json::ExportJson(storage, output.get());
}

struct Hash : public SqlFunction {
  static base::Status Run(void*,
                          size_t argc,
                          sqlite3_value** argv,
                          SqlValue& out,
                          Destructors&);
};

base::Status Hash::Run(void*,
                       size_t argc,
                       sqlite3_value** argv,
                       SqlValue& out,
                       Destructors&) {
  base::Hasher hash;
  for (size_t i = 0; i < argc; ++i) {
    sqlite3_value* value = argv[i];
    int type = sqlite3_value_type(value);
    switch (type) {
      case SQLITE_INTEGER:
        hash.Update(sqlite3_value_int64(value));
        break;
      case SQLITE_TEXT: {
        const char* ptr =
            reinterpret_cast<const char*>(sqlite3_value_text(value));
        hash.Update(ptr, strlen(ptr));
        break;
      }
      default:
        return base::ErrStatus("HASH: arg %zu has unknown type %d", i, type);
    }
  }
  out = SqlValue::Long(static_cast<int64_t>(hash.digest()));
  return base::OkStatus();
}

struct Reverse : public SqlFunction {
  static base::Status Run(void*,
                          size_t argc,
                          sqlite3_value** argv,
                          SqlValue& out,
                          Destructors& destructors);
};

base::Status Reverse::Run(void*,
                          size_t argc,
                          sqlite3_value** argv,
                          SqlValue& out,
                          Destructors& destructors) {
  if (argc != 1) {
    return base::ErrStatus("REVERSE: expected one arg but got %zu", argc);
  }

  // If the string is null, just return null as the result.
  if (sqlite3_value_type(argv[0]) == SQLITE_NULL) {
    return base::OkStatus();
  }
  if (sqlite3_value_type(argv[0]) != SQLITE_TEXT) {
    return base::ErrStatus("REVERSE: argument should be string");
  }

  const char* in = reinterpret_cast<const char*>(sqlite3_value_text(argv[0]));
  std::string_view in_str = in;
  std::string reversed(in_str.rbegin(), in_str.rend());

  std::unique_ptr<char, base::FreeDeleter> s(
      static_cast<char*>(malloc(reversed.size() + 1)));
  memcpy(s.get(), reversed.c_str(), reversed.size() + 1);

  destructors.string_destructor = free;
  out = SqlValue::String(s.release());
  return base::OkStatus();
}

struct Base64Encode : public SqlFunction {
  static base::Status Run(void*,
                          size_t argc,
                          sqlite3_value** argv,
                          SqlValue& out,
                          Destructors&);
};

base::Status Base64Encode::Run(void*,
                               size_t argc,
                               sqlite3_value** argv,
                               SqlValue& out,
                               Destructors& destructors) {
  if (argc != 1)
    return base::ErrStatus("Unsupported number of arg passed to Base64Encode");

  sqlite3_value* value = argv[0];
  if (sqlite3_value_type(value) != SQLITE_BLOB)
    return base::ErrStatus("Base64Encode only supports bytes argument");

  size_t byte_count = static_cast<size_t>(sqlite3_value_bytes(value));
  std::string res = base::Base64Encode(sqlite3_value_blob(value), byte_count);

  std::unique_ptr<char, base::FreeDeleter> s(
      static_cast<char*>(malloc(res.size() + 1)));
  memcpy(s.get(), res.c_str(), res.size() + 1);

  out = SqlValue::String(s.release());
  destructors.string_destructor = free;

  return base::OkStatus();
}

struct Demangle : public SqlFunction {
  static base::Status Run(void*,
                          size_t argc,
                          sqlite3_value** argv,
                          SqlValue& out,
                          Destructors& destructors);
};

base::Status Demangle::Run(void*,
                           size_t argc,
                           sqlite3_value** argv,
                           SqlValue& out,
                           Destructors& destructors) {
  if (argc != 1)
    return base::ErrStatus("Unsupported number of arg passed to DEMANGLE");
  sqlite3_value* value = argv[0];
  if (sqlite3_value_type(value) == SQLITE_NULL)
    return base::OkStatus();

  if (sqlite3_value_type(value) != SQLITE_TEXT)
    return base::ErrStatus("Unsupported type of arg passed to DEMANGLE");

  const char* mangled =
      reinterpret_cast<const char*>(sqlite3_value_text(value));

  std::unique_ptr<char, base::FreeDeleter> demangled =
      demangle::Demangle(mangled);
  if (!demangled)
    return base::OkStatus();

  destructors.string_destructor = free;
  out = SqlValue::String(demangled.release());
  return base::OkStatus();
}

struct WriteFile : public SqlFunction {
  using Context = TraceStorage;
  static base::Status Run(TraceStorage* storage,
                          size_t,
                          sqlite3_value** argv,
                          SqlValue&,
                          Destructors&);
};

base::Status WriteFile::Run(TraceStorage*,
                            size_t argc,
                            sqlite3_value** argv,
                            SqlValue& out,
                            Destructors&) {
  if (argc != 2) {
    return base::ErrStatus("WRITE_FILE: expected %d args but got %zu", 2, argc);
  }

  base::Status status =
      sqlite::utils::TypeCheckSqliteValue(argv[0], SqlValue::kString);
  if (!status.ok()) {
    return base::ErrStatus("WRITE_FILE: argument 1, filename; %s",
                           status.c_message());
  }

  status = sqlite::utils::TypeCheckSqliteValue(argv[1], SqlValue::kBytes);
  if (!status.ok()) {
    return base::ErrStatus("WRITE_FILE: argument 2, content; %s",
                           status.c_message());
  }

  const std::string filename =
      reinterpret_cast<const char*>(sqlite3_value_text(argv[0]));

  base::ScopedFstream file = base::OpenFstream(filename.c_str(), "wb");
  if (!file) {
    return base::ErrStatus("WRITE_FILE: Couldn't open output file %s (%s)",
                           filename.c_str(), strerror(errno));
  }

  int int_len = sqlite3_value_bytes(argv[1]);
  PERFETTO_CHECK(int_len >= 0);
  size_t len = (static_cast<size_t>(int_len));
  // Make sure to call last as sqlite3_value_bytes can invalidate pointer
  // returned.
  const void* data = sqlite3_value_text(argv[1]);
  if (fwrite(data, 1, len, file.get()) != len || fflush(file.get()) != 0) {
    return base::ErrStatus("WRITE_FILE: Failed to write to file %s (%s)",
                           filename.c_str(), strerror(errno));
  }

  out = SqlValue::Long(int_len);

  return base::OkStatus();
}

struct ExtractArg : public SqlFunction {
  using Context = TraceStorage;
  static base::Status Run(TraceStorage* storage,
                          size_t argc,
                          sqlite3_value** argv,
                          SqlValue& out,
                          Destructors& destructors);
};

base::Status ExtractArg::Run(TraceStorage* storage,
                             size_t argc,
                             sqlite3_value** argv,
                             SqlValue& out,
                             Destructors& destructors) {
  if (argc != 2)
    return base::ErrStatus("EXTRACT_ARG: 2 args required");

  // If the arg set id is null, just return null as the result.
  if (sqlite3_value_type(argv[0]) == SQLITE_NULL)
    return base::OkStatus();

  if (sqlite3_value_type(argv[0]) != SQLITE_INTEGER)
    return base::ErrStatus("EXTRACT_ARG: 1st argument should be arg set id");

  if (sqlite3_value_type(argv[1]) != SQLITE_TEXT)
    return base::ErrStatus("EXTRACT_ARG: 2nd argument should be key");

  uint32_t arg_set_id = static_cast<uint32_t>(sqlite3_value_int(argv[0]));
  const char* key = reinterpret_cast<const char*>(sqlite3_value_text(argv[1]));

  std::optional<Variadic> opt_value;
  RETURN_IF_ERROR(storage->ExtractArg(arg_set_id, key, &opt_value));

  if (!opt_value)
    return base::OkStatus();

  // This function always returns static strings (i.e. scoped to lifetime
  // of the TraceStorage thread pool) so prevent SQLite from making copies.
  destructors.string_destructor = sqlite::utils::kSqliteStatic;

  switch (opt_value->type) {
    case Variadic::kNull:
      return base::OkStatus();
    case Variadic::kInt:
      out = SqlValue::Long(opt_value->int_value);
      return base::OkStatus();
    case Variadic::kUint:
      out = SqlValue::Long(static_cast<int64_t>(opt_value->uint_value));
      return base::OkStatus();
    case Variadic::kString:
      out =
          SqlValue::String(storage->GetString(opt_value->string_value).data());
      return base::OkStatus();
    case Variadic::kReal:
      out = SqlValue::Double(opt_value->real_value);
      return base::OkStatus();
    case Variadic::kBool:
      out = SqlValue::Long(opt_value->bool_value);
      return base::OkStatus();
    case Variadic::kPointer:
      out = SqlValue::Long(static_cast<int64_t>(opt_value->pointer_value));
      return base::OkStatus();
    case Variadic::kJson:
      out = SqlValue::String(storage->GetString(opt_value->json_value).data());
      return base::OkStatus();
  }
  PERFETTO_FATAL("For GCC");
}

struct SourceGeq : public SqlFunction {
  static base::Status Run(void*,
                          size_t,
                          sqlite3_value**,
                          SqlValue&,
                          Destructors&) {
    return base::ErrStatus(
        "SOURCE_GEQ should not be called from the global scope");
  }
};

struct TablePtrBind : public SqlFunction {
  static base::Status Run(void*,
                          size_t,
                          sqlite3_value**,
                          SqlValue&,
                          Destructors&) {
    return base::ErrStatus(
        "__intrinsic_table_ptr_bind should not be called from the global "
        "scope");
  }
};

struct Glob : public SqlFunction {
  static base::Status Run(void*,
                          size_t,
                          sqlite3_value** argv,
                          SqlValue& out,
                          Destructors&) {
    const char* pattern =
        reinterpret_cast<const char*>(sqlite3_value_text(argv[0]));
    const char* text =
        reinterpret_cast<const char*>(sqlite3_value_text(argv[1]));
    if (pattern && text) {
      out = SqlValue::Long(sqlite3_strglob(pattern, text) == 0);
    }
    return base::OkStatus();
  }
};

struct Regex : public SqlFunction {
  static base::Status Run(void*,
                          size_t,
                          sqlite3_value** argv,
                          SqlValue& out,
                          Destructors&) {
    if constexpr (regex::IsRegexSupported()) {
      const char* pattern_str =
          reinterpret_cast<const char*>(sqlite3_value_text(argv[0]));
      const char* text =
          reinterpret_cast<const char*>(sqlite3_value_text(argv[1]));
      if (pattern_str && text) {
        auto regex = regex::Regex::Create(pattern_str);
        if (!regex.status().ok()) {
          return regex.status();
        }
        out = SqlValue::Long(regex->Search(text));
      }
      return base::OkStatus();
    }
    PERFETTO_FATAL("Regex not supported");
  }
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_PERFETTO_SQL_INTRINSICS_FUNCTIONS_UTILS_H_
