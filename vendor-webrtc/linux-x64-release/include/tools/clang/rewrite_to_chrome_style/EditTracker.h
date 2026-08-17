// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_REWRITE_TO_CHROME_STYLE_EDIT_TRACKER_H_
#define TOOLS_CLANG_REWRITE_TO_CHROME_STYLE_EDIT_TRACKER_H_

#include <map>

#include "clang/Basic/SourceLocation.h"
#include "clang/Basic/SourceManager.h"
#include "llvm/ADT/StringMap.h"
#include "llvm/ADT/StringRef.h"
#include "llvm/ADT/StringSet.h"

namespace llvm {
class raw_ostream;
}  // namespace llvm

struct EditInfo {
  std::string new_text;
  llvm::StringSet<> filenames;
};

enum class RenameCategory {
  kEnumValue,
  kField,
  kFunction,
  kUnresolved,
  kVariable,
};

// Simple class that tracks the edits made by path. Used to dump the databaes
// used by the Blink rebase helper.
class EditTracker {
 public:
  explicit EditTracker(RenameCategory category);

  void Add(const clang::SourceManager& source_manager,
           clang::SourceLocation location,
           llvm::StringRef original_text,
           llvm::StringRef new_text);

  // Serializes the tracked edits to |output|. Emits:
  // <filename>:<tag>:<original text>:<new text>
  // for each distinct filename for each tracked edit.
  void SerializeTo(llvm::raw_ostream& output) const;

 private:
  EditTracker(const EditTracker&) = delete;
  EditTracker& operator=(const EditTracker&) = delete;

  // The string key is the original text.
  llvm::StringMap<EditInfo> tracked_edits_;

  RenameCategory category_;
};

#endif  // #define TOOLS_CLANG_REWRITE_TO_CHROME_STYLE_EDIT_TRACKER_H_
