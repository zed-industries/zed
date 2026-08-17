// Copyright 2020 The Chromium Authors
// Copyright 2014 Blake Embrey (hello@blakeembrey.com)
// Use of this source code is governed by an MIT-style license that can be
// found in the LICENSE file or at https://opensource.org/licenses/MIT.

#ifndef THIRD_PARTY_LIBURLPATTERN_OPTIONS_H_
#define THIRD_PARTY_LIBURLPATTERN_OPTIONS_H_

#include <string>
#include "base/component_export.h"

namespace liburlpattern {

// A structure that may be provided to the Parse() function to control matching
// behavior.  This corresponds to a union of the two path-to-regexp structures
// at:
//
//  https://github.com/pillarjs/path-to-regexp/blob/125c43e6481f68cc771a5af22b914acdb8c5ba1f/src/index.ts#L126-L135
//
// and:
//
//  https://github.com/pillarjs/path-to-regexp/blob/125c43e6481f68cc771a5af22b914acdb8c5ba1f/src/index.ts#L498-L527
struct COMPONENT_EXPORT(LIBURLPATTERN) Options {
  // |delimiter_list| contains a list of characters that are considered segment
  // separators when performing a kSegmentWildcard.  This is the behavior you
  // get when you specify a name `:foo` without a custom regular expression.
  // These characters are also optionally permitted at the end of an input
  // string when |strict| is false.
  std::string delimiter_list = "/#?";

  // |prefix_list| contains a list of characters to automatically treat as a
  // prefix when they appear before a kName or kRegex Token; e.g. "/:foo",
  // includes the leading "/" as the prefix for the "foo" named group by
  // default.
  std::string prefix_list = "./";

  // True if matching should be case sensitive.
  bool sensitive = false;

  // When true matching will not permit an optional trailing delimiter.  For
  // example, when false a pattern "/foo" will match the string "/foo/".  The
  // allowed trailing delimiter characters are contained in |delimiter_list|.
  bool strict = false;

  // When true requires matching to the end of the input string.
  bool end = true;

  // When true requires matching to begin at the start of the input string.
  bool start = true;

  // A list of characters that can also signal the "end" of an input string.
  std::string ends_with;

  // Note, we do not include an |encode| option here like path-to-regexp.  This
  // library requires that calling code percent encode UTF8 as ascii before
  // calling Parse().
};

}  // namespace liburlpattern

#endif  // THIRD_PARTY_LIBURLPATTERN_OPTIONS_H_
