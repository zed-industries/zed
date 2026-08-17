/*
 * Copyright (C) 2018 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_SQLITE_SQL_SOURCE_H_
#define SRC_TRACE_PROCESSOR_SQLITE_SQL_SOURCE_H_

#include <cstdint>
#include <optional>
#include <string>
#include <string_view>
#include <tuple>
#include <vector>

#include "perfetto/base/logging.h"

namespace perfetto {
namespace trace_processor {

// An SQL string which retains knowledge of the source of the SQL (i.e. stdlib
// module, ExecuteQuery etc). It also supports "rewriting" parts or all of the
// SQL string with a different string which is useful in cases where SQL is
// substituted such as macros or function inlining.
class SqlSource {
 public:
  class Rewriter;

  // Creates a SqlSource instance wrapping SQL passed to
  // |TraceProcessor::ExecuteQuery|.
  static SqlSource FromExecuteQuery(std::string sql);

  // Creates a SqlSource instance wrapping SQL executed when running a metric.
  static SqlSource FromMetric(std::string sql, const std::string& metric_file);

  // Creates a SqlSource instance wrapping SQL executed when running a metric
  // file (i.e. with RUN_METRIC).
  static SqlSource FromMetricFile(std::string sql,
                                  const std::string& metric_file);

  // Creates a SqlSource instance wrapping SQL executed when including a module.
  static SqlSource FromModuleInclude(std::string sql,
                                     const std::string& module);

  // Creates a SqlSource instance wrapping SQL which is an internal
  // implementation detail of trace processor.
  static SqlSource FromTraceProcessorImplementation(std::string sql);

  // Returns this SqlSource instance as a string which can be appended as a
  // "traceback" frame to an error message. Callers should pass an |offset|
  // parameter which indicates the exact location of the error in the SQL
  // string. 0 and |sql().size()| are both valid offset positions and correspond
  // to the start and end of the source respectively.
  //
  // Specifically, this string will include:
  //  a) context about the source of the SQL
  //  b) line and column number of the error
  //  c) a snippet of the SQL and a caret (^) character pointing to the location
  //     of the error.
  std::string AsTraceback(uint32_t offset) const;

  // Same as |AsTraceback| but for offsets which come from SQLite instead of
  // from trace processor tokenization or parsing.
  std::string AsTracebackForSqliteOffset(std::optional<uint32_t> offset) const;

  // Creates a SqlSource instance with the SQL taken as a substring starting
  // at |offset| with |len| characters.
  SqlSource Substr(uint32_t offset, uint32_t len) const;

  // Rewrites the SQL backing |this| to SQL from |source| ignoring any existing
  // rewrites in |this|.
  //
  // This is useful when PerfettoSQL statements are transpiled into SQLite
  // statements but we want to preserve the context of the original statement.
  SqlSource RewriteAllIgnoreExisting(SqlSource source) const;

  // Returns the SQL string backing this SqlSource instance;
  const std::string& sql() const { return root_.rewritten_sql; }

  // Returns the original SQL string backing this SqlSource instance;
  const std::string& original_sql() const { return root_.original_sql; }

  // Returns whether this SqlSource has been rewritten.
  bool IsRewritten() const { return root_.IsRewritten(); }

 private:
  struct Rewrite;

  // Represents a tree of SQL rewrites, preserving the source for each rewrite.
  //
  // Suppose that we have the following situation:
  // User: `SELECT foo!(a) FROM bar!(slice) a`
  // foo : `$1.x, $1.y`
  // bar : `(SELECT baz!($1) FROM $1)`
  // baz : `$1.x, $1.y, $1.z`
  //
  // We want to expand this to
  // ```SELECT a.x, a.y FROM (SELECT slice.x, slice.y, slice.z FROM slice) a```
  // while retaining information about the source of the rewrite.
  //
  // For example, the string `a.x, a.y` came from foo, `slice.x, slice.y,
  // slice.z` came from bar, which itself recursively came from baz etc.
  //
  // The purpose of this class is to keep track of the information required for
  // this "tree" of rewrites (i.e. expansions). In the example above, the tree
  // would look as follows:
  //                      User
  //                     /    |
  //                   foo    bar
  //                   /
  //                 baz
  //
  // The properties in each of these nodes is as follows:
  // User {
  //   original_sql: "SELECT foo!(a) FROM bar!(slice) a"
  //   rewritten_sql: "SELECT a.x, a.y FROM (SELECT slice.x, slice.y, slice.z
  //                   FROM slice) a"
  //   rewrites: [
  //     {original_sql_start: 7, original_sql_end: 14, node: foo},
  //     {original_sql_start: 20, original_sql_end: 31, node: bar}]
  //   ]
  // }
  // foo {
  //   original_sql: "$1.x, $1.y"
  //   rewritten_sql: "a.x, a.y"
  //   rewrites: []
  // }
  // bar {
  //   original_sql: "(SELECT baz!($1) FROM $1 LIMIT 1)"
  //   rewritten_sql: "(SELECT slice.x, slice.y, slice.z FROM slice)"
  //   rewrites: [{original_sql_start: 8, original_sql_end: 16, node: baz}]
  // }
  // baz {
  //   original_sql = "$1.x, $1.y, $1.z"
  //   rewritten_sql = "slice.x, slice.y, slice.z"
  //   rewrites: []
  // }
  struct Node {
    std::string name;
    bool include_traceback_header = false;
    uint32_t line = 1;
    uint32_t col = 1;

    // The original SQL string used to create this node.
    std::string original_sql;

    // The list of rewrites which are applied to |original_sql| ordered by the
    // offsets.
    std::vector<Rewrite> rewrites;

    // The SQL string which is the result of applying |rewrites| to
    // |original_sql|. See |SqlSource::ApplyRewrites| for details on how this is
    // computed.
    std::string rewritten_sql;

    // Returns the "traceback" for this node and all recursive nodes. See
    // |SqlSource::AsTraceback| for details.
    std::string AsTraceback(uint32_t rewritten_offset) const;

    // Returns the "traceback" for this node only. See |SqlSource::AsTraceback|
    // for details.
    std::string SelfTraceback(uint32_t rewritten_offset,
                              uint32_t original_offset) const;

    Node Substr(uint32_t rewritten_offset, uint32_t rewritten_len) const;

    bool IsRewritten() const {
      PERFETTO_CHECK(rewrites.empty() == (original_sql == rewritten_sql));
      return !rewrites.empty();
    }

    // Given a |rewritten_offset| for this node, returns the offset into the
    // |original_sql| which matches that |rewritten_offset|.
    //
    // IMPORTANT: if |rewritten_offset| is *inside* a rewrite, the original
    // offset will point to the *start of the rewrite*. For example, if
    // we have:
    //   original_sql: "SELECT foo!(a) FROM slice a"
    //   rewritten_sql: "SELECT a.x, a.y FROM slice a"
    //   rewrites: [
    //     {
    //       original_sql_start: 7,
    //       original_sql_end: 14,
    //       rewritten_sql_start: 7,
    //       rewritten_sql_end: 15,
    //       node: foo
    //     }
    //   ]
    // then:
    //   RewrittenOffsetToOriginalOffset(7) == 7     // 7 = start of foo
    //   RewrittenOffsetToOriginalOffset(14) == 7    // 7 = start of foo
    //   RewrittenOffsetToOriginalOffset(15) == 14   // 14 = end of foo
    //   RewrittenOffsetToOriginalOffset(16) == 15
    uint32_t RewrittenOffsetToOriginalOffset(uint32_t rewritten_offset) const;

    // Given an |original_offset| for this node, returns the index of a
    // rewrite whose original range contains |original_offset|.
    // Returns std::nullopt if there is no such rewrite.
    std::optional<uint32_t> RewriteForOriginalOffset(
        uint32_t original_offset) const;
  };

  // Defines a rewrite. See the documentation for |SqlSource::Node| for details
  // on this.
  struct Rewrite {
    // The start and end offsets in |original_sql|.
    uint32_t original_sql_start;
    uint32_t original_sql_end;

    // The start and end offsets in |rewritten_sql|.
    uint32_t rewritten_sql_start;
    uint32_t rewritten_sql_end;

    // Node containing the SQL which replaces the segment of SQL in
    // |original_sql|.
    Node rewrite_node;
  };

  SqlSource();
  explicit SqlSource(Node);
  SqlSource(std::string sql, std::string name, bool include_traceback_header);

  static std::string ApplyRewrites(const std::string&,
                                   const std::vector<Rewrite>&);

  Node root_;
};

// Used to rewrite a SqlSource using SQL from other SqlSources.
class SqlSource::Rewriter {
 public:
  // Creates a Rewriter object which can be used to rewrite the SQL backing
  // |source|.
  //
  // Note that rewrites of portions of the SQL which have already been rewritten
  // is supported but *only in limited cases*. Specifically, the new rewrite
  // must not cross the boundary of any existing rewrite.
  //
  // For example, if we have:
  //   SqlSource {
  //     original_sql: "SELECT foo!(a) FROM bar!(slice) a"
  //     rewritten_sql: "SELECT a.x, a.y FROM (SELECT slice.x FROM slice) a"
  //   }
  // then the following are valid:
  //   # Replaces "SELECT " with "INSERT ". Valid because it does not touch
  //   # any rewrite.
  //   Rewrite(0, 7, "INSERT ")
  //
  //   # Replaces "a.x, a." with "a.z, ". Valid because it only touches the
  //   # contents of the existing "foo" rewrite.
  //   Rewrite(7, 14, "a.z, ")
  // while the following are invalid:
  //   # Fails to replace "SELECT a" with "I". Invalid because it affects both
  //   # non-rewritten source and the "foo" rewrite.
  //   Rewrite(0, 8, "I")
  //
  //   # Fails to replace "a.x, a.y FROM (" with "(". Invalid because it affects
  //   # the "foo" rewrite, non-rewritten source and the "bar" rewrite.
  //   Rewrite(7, 23, "(")
  explicit Rewriter(SqlSource source);

  // Replaces the SQL in |source.rewritten_sql| between |rewritten_start| and
  // |rewritten_end| with the contents of |rewrite|.
  //
  // Note that calls to Rewrite must be monontonic and non-overlapping. i.e.
  // if Rewrite(0, 10) is called, the next |rewritten_end| must be greater than
  // or equal to 10.
  //
  // Note also that all offsets passed to this function correspond to offsets
  // into |source.rewritten_sql|: past calls to rewrite do not affect future
  // offsets.
  void Rewrite(uint32_t rewritten_start,
               uint32_t rewritten_end,
               SqlSource rewrite);

  // Returns the rewritten SqlSource instance.
  SqlSource Build() &&;

 private:
  explicit Rewriter(Node);

  Node orig_;
  std::vector<SqlSource::Rewriter> nested_;
  std::vector<SqlSource::Rewrite> non_nested_;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_SQLITE_SQL_SOURCE_H_
