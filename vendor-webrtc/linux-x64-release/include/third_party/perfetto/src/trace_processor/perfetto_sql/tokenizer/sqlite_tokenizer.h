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

#ifndef SRC_TRACE_PROCESSOR_PERFETTO_SQL_TOKENIZER_SQLITE_TOKENIZER_H_
#define SRC_TRACE_PROCESSOR_PERFETTO_SQL_TOKENIZER_SQLITE_TOKENIZER_H_

#include <cstdint>
#include <string_view>
#include <utility>

#include "src/trace_processor/perfetto_sql/grammar/perfettosql_grammar.h"
#include "src/trace_processor/sqlite/sql_source.h"

namespace perfetto::trace_processor {

// Tokenizes SQL statements according to SQLite SQL language specification:
// https://www2.sqlite.org/hlr40000.html
//
// Usage of this class:
// SqliteTokenizer tzr(std::move(my_sql_source));
// for (auto t = tzr.Next(); t.token_type != TK_SEMI; t = tzr.Next()) {
//   // Handle t here
// }
class SqliteTokenizer {
 public:
  // A single SQL token according to the SQLite standard.
  struct Token {
    // The string contents of the token.
    std::string_view str;

    // The type of the token.
    int token_type = TK_ILLEGAL;

    bool operator==(const Token& o) const {
      return str == o.str && token_type == o.token_type;
    }

    // Returns if the token is empty or semicolon.
    bool IsTerminal() const { return token_type == TK_SEMI || str.empty(); }
  };

  enum class EndToken {
    kExclusive,
    kInclusive,
  };

  // Creates a tokenizer which tokenizes |sql|.
  explicit SqliteTokenizer(SqlSource sql);

  SqliteTokenizer(const SqliteTokenizer&) = delete;
  SqliteTokenizer& operator=(const SqliteTokenizer&) = delete;

  SqliteTokenizer(SqliteTokenizer&&) = delete;
  SqliteTokenizer& operator=(SqliteTokenizer&&) = delete;

  // Returns the next SQL token.
  Token Next();

  // Returns the next SQL token which is not of type TK_SPACE.
  Token NextNonWhitespace();

  // Returns the next SQL token which is terminal.
  Token NextTerminal();

  // Returns an SqlSource containing all the tokens between |start| and |end|.
  //
  // Note: |start| and |end| must both have been previously returned by this
  // tokenizer. If |end_token| == kInclusive, the end token is also included
  // in the substring.
  SqlSource Substr(const Token& start,
                   const Token& end,
                   EndToken end_token = EndToken::kExclusive) const;

  // Returns an SqlSource containing only the SQL backing |token|.
  //
  // Note: |token| must have been previously returned by this tokenizer.
  SqlSource SubstrToken(const Token& token) const;

  // Returns a traceback error message for the SqlSource backing this tokenizer
  // pointing to |token|. See SqlSource::AsTraceback for more information about
  // this method.
  //
  // Note: |token| must have been previously returned by this tokenizer.
  std::string AsTraceback(const Token&) const;

  // Replaces the SQL in |rewriter| between |start| and |end| with the contents
  // of |rewrite|. If |end_token| == kInclusive, the end token is also included
  // in the rewrite.
  void Rewrite(SqlSource::Rewriter& rewriter,
               const Token& start,
               const Token& end,
               SqlSource rewrite,
               EndToken end_token = EndToken::kExclusive) const;

  // Replaces the SQL in |rewriter| backing |token| with the contents of
  // |rewrite|.
  void RewriteToken(SqlSource::Rewriter&,
                    const Token&,
                    SqlSource rewrite) const;

  // Resets this tokenizer to tokenize |source|. Any previous returned tokens
  // are invalidated.
  void Reset(SqlSource source) {
    source_ = std::move(source);
    offset_ = 0;
    last_non_space_token_ = 0;
  }

 private:
  SqlSource source_;
  uint32_t offset_ = 0;
  int last_non_space_token_ = 0;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_PERFETTO_SQL_TOKENIZER_SQLITE_TOKENIZER_H_
