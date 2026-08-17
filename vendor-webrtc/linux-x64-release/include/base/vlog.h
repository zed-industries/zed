// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_VLOG_H_
#define BASE_VLOG_H_

#include <string>
#include <string_view>
#include <vector>

#include "base/base_export.h"
#include "base/memory/raw_ref.h"

namespace logging {

// A helper class containing all the settings for vlogging.
class BASE_EXPORT VlogInfo {
 public:
  static const int kDefaultVlogLevel;

  // `v_switch` gives the default maximal active V-logging level; 0 is
  // the default.  Normally positive values are used for V-logging
  // levels.
  //
  // `vmodule_switch` gives the per-module maximal V-logging levels to
  // override the value given by `v_switch`.
  // E.g. "my_module=2,foo*=3" would change the logging level for all
  // code in source files "my_module.*" and "foo*.*" ("-inl" suffixes
  // are also disregarded for this matching).
  //
  // `min_log_level` references an int that stores the log level. If a valid
  // `v_switch` is provided, it will set the log level, and the default
  // vlog severity will be read from there.
  //
  // Any pattern containing a forward or backward slash will be tested
  // against the whole pathname and not just the module.  E.g.,
  // "*/foo/bar/*=2" would change the logging level for all code in
  // source files under a "foo/bar" directory.
  VlogInfo(const std::string& v_switch,
           const std::string& vmodule_switch,
           int& min_log_level);
  VlogInfo(const VlogInfo&) = delete;
  VlogInfo& operator=(const VlogInfo&) = delete;
  ~VlogInfo();

  // Returns the vlog level for a given file (usually taken from
  // __FILE__).
  int GetVlogLevel(std::string_view file) const;

  // Returns a new VlogInfo based on `this` but with extra modules/levels added
  // according to `vmodule_switch`.
  VlogInfo* WithSwitches(const std::string& vmodule_switch) const;

 private:
  void SetMaxVlogLevel(int level);
  int GetMaxVlogLevel() const;

  // VmodulePattern holds all the information for each pattern parsed
  // from `vmodule_switch`.
  struct VmodulePattern {
    enum MatchTarget { MATCH_MODULE, MATCH_FILE };

    explicit VmodulePattern(const std::string& pattern);

    VmodulePattern();

    std::string pattern;
    int vlog_level;
    MatchTarget match_target;
  };

  VlogInfo(std::vector<VmodulePattern> vmodule_levels, int& min_log_level);

  // Parses `VmodulePatterns` from a string, typically provided on the
  // commandline.
  static std::vector<VmodulePattern> ParseVmoduleLevels(
      const std::string& vmodule_switch);

  const std::vector<VmodulePattern> vmodule_levels_;
  raw_ref<int> const min_log_level_;
};

// Returns true if the string passed in matches the vlog pattern.  The
// vlog pattern string can contain wildcards like * and ?.  ? matches
// exactly one character while * matches 0 or more characters.  Also,
// as a special case, a / or \ character matches either / or \.
//
// Examples:
//   "kh?n" matches "khan" but not "khn" or "khaan"
//   "kh*n" matches "khn", "khan", or even "khaaaaan"
//   "/foo\bar" matches "/foo/bar", "\foo\bar", or "/foo\bar"
//     (disregarding C escaping rules)
BASE_EXPORT bool MatchVlogPattern(std::string_view string,
                                  std::string_view vlog_pattern);

}  // namespace logging

#endif  // BASE_VLOG_H_
