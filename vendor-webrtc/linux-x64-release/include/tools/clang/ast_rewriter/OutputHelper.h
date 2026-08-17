// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_AST_REWRITER_OUTPUTHELPER_H_
#define TOOLS_CLANG_AST_REWRITER_OUTPUTHELPER_H_

#include <string>

#include "clang/Tooling/Core/Replacement.h"
#include "clang/Tooling/Tooling.h"
#include "llvm/ADT/StringSet.h"

// This is a general helper class for emitting the substitution directives
// consumed by apply_edits.py.
// See
// https://chromium.googlesource.com/chromium/src/+/HEAD/docs/clang_tool_refactoring.md
// for general documentation on the format.
//
// From a consumer's perspective, the most important functions are `Delete`,
// `Replace`, and `Wrap`, which each emit a substitution directive to stdout.
// The class also maintains a list of headers to be added to every file where a
// replacement occurred; the directives are emitted at the end of the file.
//
// For the most part, you should be able to re-use this class without any
// changes. It's possible certain use cases may require more complex logic, if
// e.g. you're doing multiple kinds of replacement at once, and different ones
// need to add different sets of headers.
//
// The substitution directives all take a CharSourceRange as their primary
// argument. Despite the name, these represent a range of _either_ characters or
// tokens, as reported by their isTokenRange() method. Both versions store a
// start and an end SourceLocation; in the 'char' case, these point to the first
// and last character of the range, respectively. In the 'token' case, they
// point to the first character in the first/last token of the range. This means
// that typically they will point _before_ the last character of the range, e.g.
// in the code "Foo + Bar", the end of a character range will point at 'r',
// while the end of a token range will point at 'B'. In both cases, the start
// of the range will be 'F'.
//
// From a usage perspective, the primary difference is which construction
// function you should call. If you have character-granular information, then
// call CharSourceRange::getCharRange; if you have token-level information, then
// call CharSourceRange::getTokenRange.
class OutputHelper : public clang::tooling::SourceFileCallbacks {
 public:
  OutputHelper() = default;
  ~OutputHelper() = default;

  OutputHelper(const OutputHelper&) = delete;
  OutputHelper& operator=(const OutputHelper&) = delete;

  OutputHelper(llvm::StringSet<> headers_to_add)
      : headers_to_add_(std::move(headers_to_add)) {};

  // Replaces `replacement_range` with `replacement_text`.
  void Replace(const clang::CharSourceRange& replacement_range,
               std::string replacement_text,
               const clang::SourceManager& source_manager,
               const clang::LangOptions& lang_opts);

  // Deletes `replacement_range`.
  void Delete(const clang::CharSourceRange& replacement_range,
              const clang::SourceManager& source_manager,
              const clang::LangOptions& lang_opts);

  // Inserts `lhs` and `rhs` to the left and right of `replacement_range`.
  void Wrap(const clang::CharSourceRange& replacement_range,
            std::string_view lhs,
            std::string_view rhs,
            const clang::SourceManager& source_manager,
            const clang::LangOptions& lang_opts);

 private:
  // By inheriting from clang::tooling::SourceFileCallbacks, OutputHelper
  // automatically executes setup and teardown code at the beginning/end of each
  // file.

  // This is run automatically when the tool is first invoked.
  bool handleBeginSource(clang::CompilerInstance& compiler) override;

  // This is run automatically at the end of the file.
  void handleEndSource() override;

  // Called by PrintReplacement to determine if we should actually replace in
  // this file.
  bool ShouldOutput() { return current_language_ == clang::Language::CXX; }

  // Emit the requested replacement in the proper format.
  void PrintReplacement(llvm::StringRef file_path,
                        unsigned offset,
                        unsigned length,
                        std::string_view replacement_text) {
    if (ShouldOutput()) {
      files_replaced_in_.insert(file_path);
      std::string final_text = std::string(replacement_text);
      // The rewriting format expects newlines to be replaced with \0
      std::replace(final_text.begin(), final_text.end(), '\n', '\0');
      llvm::outs() << "r:::" << file_path << ":::" << offset << ":::" << length
                   << ":::" << final_text << "\n";
    }
  }

  // The language of the file we're currently looking at.
  clang::Language current_language_ = clang::Language::Unknown;
  // At the end, we'll add additional headers to each file we emitted a
  // replacement directive for.
  llvm::StringSet<> files_replaced_in_;
  llvm::StringSet<> headers_to_add_;
};

#endif  // TOOLS_CLANG_AST_REWRITER_OUTPUTHELPER_H_
