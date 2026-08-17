// Copyright 2003-2009 The RE2 Authors.  All Rights Reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

#ifndef RE2_RE2_H_
#define RE2_RE2_H_

// C++ interface to the re2 regular-expression library.
// RE2 supports Perl-style regular expressions (with extensions like
// \d, \w, \s, ...).
//
// -----------------------------------------------------------------------
// REGEXP SYNTAX:
//
// This module uses the re2 library and hence supports
// its syntax for regular expressions, which is similar to Perl's with
// some of the more complicated things thrown away.  In particular,
// backreferences and generalized assertions are not available, nor is \Z.
//
// See https://github.com/google/re2/wiki/Syntax for the syntax
// supported by RE2, and a comparison with PCRE and PERL regexps.
//
// For those not familiar with Perl's regular expressions,
// here are some examples of the most commonly used extensions:
//
//   "hello (\\w+) world"  -- \w matches a "word" character
//   "version (\\d+)"      -- \d matches a digit
//   "hello\\s+world"      -- \s matches any whitespace character
//   "\\b(\\w+)\\b"        -- \b matches non-empty string at word boundary
//   "(?i)hello"           -- (?i) turns on case-insensitive matching
//   "/\\*(.*?)\\*/"       -- .*? matches . minimum no. of times possible
//
// The double backslashes are needed when writing C++ string literals.
// However, they should NOT be used when writing C++11 raw string literals:
//
//   R"(hello (\w+) world)"  -- \w matches a "word" character
//   R"(version (\d+))"      -- \d matches a digit
//   R"(hello\s+world)"      -- \s matches any whitespace character
//   R"(\b(\w+)\b)"          -- \b matches non-empty string at word boundary
//   R"((?i)hello)"          -- (?i) turns on case-insensitive matching
//   R"(/\*(.*?)\*/)"        -- .*? matches . minimum no. of times possible
//
// When using UTF-8 encoding, case-insensitive matching will perform
// simple case folding, not full case folding.
//
// -----------------------------------------------------------------------
// MATCHING INTERFACE:
//
// The "FullMatch" operation checks that supplied text matches a
// supplied pattern exactly.
//
// Example: successful match
//    ABSL_CHECK(RE2::FullMatch("hello", "h.*o"));
//
// Example: unsuccessful match (requires full match):
//    ABSL_CHECK(!RE2::FullMatch("hello", "e"));
//
// -----------------------------------------------------------------------
// UTF-8 AND THE MATCHING INTERFACE:
//
// By default, the pattern and input text are interpreted as UTF-8.
// The RE2::Latin1 option causes them to be interpreted as Latin-1.
//
// Example:
//    ABSL_CHECK(RE2::FullMatch(utf8_string, RE2(utf8_pattern)));
//    ABSL_CHECK(RE2::FullMatch(latin1_string, RE2(latin1_pattern,
//                                                 RE2::Latin1)));
//
// -----------------------------------------------------------------------
// SUBMATCH EXTRACTION:
//
// You can supply extra pointer arguments to extract submatches.
// On match failure, none of the pointees will have been modified.
// On match success, the submatches will be converted (as necessary) and
// their values will be assigned to their pointees until all conversions
// have succeeded or one conversion has failed.
// On conversion failure, the pointees will be in an indeterminate state
// because the caller has no way of knowing which conversion failed.
// However, conversion cannot fail for types like string and string_view
// that do not inspect the submatch contents. Hence, in the common case
// where all of the pointees are of such types, failure is always due to
// match failure and thus none of the pointees will have been modified.
//
// Example: extracts "ruby" into "s" and 1234 into "i"
//    int i;
//    std::string s;
//    ABSL_CHECK(RE2::FullMatch("ruby:1234", "(\\w+):(\\d+)", &s, &i));
//
// Example: extracts "ruby" into "s" and no value into "i"
//    absl::optional<int> i;
//    std::string s;
//    ABSL_CHECK(RE2::FullMatch("ruby", "(\\w+)(?::(\\d+))?", &s, &i));
//
// Example: fails because string cannot be stored in integer
//    ABSL_CHECK(!RE2::FullMatch("ruby", "(.*)", &i));
//
// Example: fails because there aren't enough sub-patterns
//    ABSL_CHECK(!RE2::FullMatch("ruby:1234", "\\w+:\\d+", &s));
//
// Example: does not try to extract any extra sub-patterns
//    ABSL_CHECK(RE2::FullMatch("ruby:1234", "(\\w+):(\\d+)", &s));
//
// Example: does not try to extract into NULL
//    ABSL_CHECK(RE2::FullMatch("ruby:1234", "(\\w+):(\\d+)", NULL, &i));
//
// Example: integer overflow causes failure
//    ABSL_CHECK(!RE2::FullMatch("ruby:1234567891234", "\\w+:(\\d+)", &i));
//
// NOTE(rsc): Asking for submatches slows successful matches quite a bit.
// This may get a little faster in the future, but right now is slower
// than PCRE.  On the other hand, failed matches run *very* fast (faster
// than PCRE), as do matches without submatch extraction.
//
// -----------------------------------------------------------------------
// PARTIAL MATCHES
//
// You can use the "PartialMatch" operation when you want the pattern
// to match any substring of the text.
//
// Example: simple search for a string:
//      ABSL_CHECK(RE2::PartialMatch("hello", "ell"));
//
// Example: find first number in a string
//      int number;
//      ABSL_CHECK(RE2::PartialMatch("x*100 + 20", "(\\d+)", &number));
//      ABSL_CHECK_EQ(number, 100);
//
// -----------------------------------------------------------------------
// PRE-COMPILED REGULAR EXPRESSIONS
//
// RE2 makes it easy to use any string as a regular expression, without
// requiring a separate compilation step.
//
// If speed is of the essence, you can create a pre-compiled "RE2"
// object from the pattern and use it multiple times.  If you do so,
// you can typically parse text faster than with sscanf.
//
// Example: precompile pattern for faster matching:
//    RE2 pattern("h.*o");
//    while (ReadLine(&str)) {
//      if (RE2::FullMatch(str, pattern)) ...;
//    }
//
// -----------------------------------------------------------------------
// SCANNING TEXT INCREMENTALLY
//
// The "Consume" operation may be useful if you want to repeatedly
// match regular expressions at the front of a string and skip over
// them as they match.  This requires use of the string_view type,
// which represents a sub-range of a real string.
//
// Example: read lines of the form "var = value" from a string.
//      std::string contents = ...;         // Fill string somehow
//      absl::string_view input(contents);  // Wrap a string_view around it
//
//      std::string var;
//      int value;
//      while (RE2::Consume(&input, "(\\w+) = (\\d+)\n", &var, &value)) {
//        ...;
//      }
//
// Each successful call to "Consume" will set "var/value", and also
// advance "input" so it points past the matched text.  Note that if the
// regular expression matches an empty string, input will advance
// by 0 bytes.  If the regular expression being used might match
// an empty string, the loop body must check for this case and either
// advance the string or break out of the loop.
//
// The "FindAndConsume" operation is similar to "Consume" but does not
// anchor your match at the beginning of the string.  For example, you
// could extract all words from a string by repeatedly calling
//     RE2::FindAndConsume(&input, "(\\w+)", &word)
//
// -----------------------------------------------------------------------
// USING VARIABLE NUMBER OF ARGUMENTS
//
// The above operations require you to know the number of arguments
// when you write the code.  This is not always possible or easy (for
// example, the regular expression may be calculated at run time).
// You can use the "N" version of the operations when the number of
// match arguments are determined at run time.
//
// Example:
//   const RE2::Arg* args[10];
//   int n;
//   // ... populate args with pointers to RE2::Arg values ...
//   // ... set n to the number of RE2::Arg objects ...
//   bool match = RE2::FullMatchN(input, pattern, args, n);
//
// The last statement is equivalent to
//
//   bool match = RE2::FullMatch(input, pattern,
//                               *args[0], *args[1], ..., *args[n - 1]);
//
// -----------------------------------------------------------------------
// PARSING HEX/OCTAL/C-RADIX NUMBERS
//
// By default, if you pass a pointer to a numeric value, the
// corresponding text is interpreted as a base-10 number.  You can
// instead wrap the pointer with a call to one of the operators Hex(),
// Octal(), or CRadix() to interpret the text in another base.  The
// CRadix operator interprets C-style "0" (base-8) and "0x" (base-16)
// prefixes, but defaults to base-10.
//
// Example:
//   int a, b, c, d;
//   ABSL_CHECK(RE2::FullMatch("100 40 0100 0x40", "(.*) (.*) (.*) (.*)",
//         RE2::Octal(&a), RE2::Hex(&b), RE2::CRadix(&c), RE2::CRadix(&d));
// will leave 64 in a, b, c, and d.

#include <stddef.h>
#include <stdint.h>

#include <algorithm>
#include <map>
#include <string>
#include <type_traits>
#include <vector>

#include "absl/base/call_once.h"
#include "absl/strings/string_view.h"
#include "absl/types/optional.h"
#include "re2/stringpiece.h"

#if defined(__APPLE__)
#include <TargetConditionals.h>
#endif

namespace re2 {
class Prog;
class Regexp;
}  // namespace re2

namespace re2 {

// Interface for regular expression matching.  Also corresponds to a
// pre-compiled regular expression.  An "RE2" object is safe for
// concurrent use by multiple threads.
class RE2 {
 public:
  // We convert user-passed pointers into special Arg objects
  class Arg;
  class Options;

  // Defined in set.h.
  class Set;

  enum ErrorCode {
    NoError = 0,

    // Unexpected error
    ErrorInternal,

    // Parse errors
    ErrorBadEscape,          // bad escape sequence
    ErrorBadCharClass,       // bad character class
    ErrorBadCharRange,       // bad character class range
    ErrorMissingBracket,     // missing closing ]
    ErrorMissingParen,       // missing closing )
    ErrorUnexpectedParen,    // unexpected closing )
    ErrorTrailingBackslash,  // trailing \ at end of regexp
    ErrorRepeatArgument,     // repeat argument missing, e.g. "*"
    ErrorRepeatSize,         // bad repetition argument
    ErrorRepeatOp,           // bad repetition operator
    ErrorBadPerlOp,          // bad perl operator
    ErrorBadUTF8,            // invalid UTF-8 in regexp
    ErrorBadNamedCapture,    // bad named capture group
    ErrorPatternTooLarge     // pattern too large (compile failed)
  };

  // Predefined common options.
  // If you need more complicated things, instantiate
  // an Option class, possibly passing one of these to
  // the Option constructor, change the settings, and pass that
  // Option class to the RE2 constructor.
  enum CannedOptions {
    DefaultOptions = 0,
    Latin1, // treat input as Latin-1 (default UTF-8)
    POSIX, // POSIX syntax, leftmost-longest match
    Quiet // do not log about regexp parse errors
  };

  // Need to have the const char* and const std::string& forms for implicit
  // conversions when passing string literals to FullMatch and PartialMatch.
  // Otherwise the absl::string_view form would be sufficient.
  RE2(const char* pattern);
  RE2(const std::string& pattern);
  RE2(absl::string_view pattern);
  RE2(absl::string_view pattern, const Options& options);
  ~RE2();

  // Not copyable.
  // RE2 objects are expensive. You should probably use std::shared_ptr<RE2>
  // instead. If you really must copy, RE2(first.pattern(), first.options())
  // effectively does so: it produces a second object that mimics the first.
  RE2(const RE2&) = delete;
  RE2& operator=(const RE2&) = delete;
  // Not movable.
  // RE2 objects are thread-safe and logically immutable. You should probably
  // use std::unique_ptr<RE2> instead. Otherwise, consider std::deque<RE2> if
  // direct emplacement into a container is desired. If you really must move,
  // be prepared to submit a design document along with your feature request.
  RE2(RE2&&) = delete;
  RE2& operator=(RE2&&) = delete;

  // Returns whether RE2 was created properly.
  bool ok() const { return error_code() == NoError; }

  // The string specification for this RE2.  E.g.
  //   RE2 re("ab*c?d+");
  //   re.pattern();    // "ab*c?d+"
  const std::string& pattern() const { return *pattern_; }

  // If RE2 could not be created properly, returns an error string.
  // Else returns the empty string.
  const std::string& error() const { return *error_; }

  // If RE2 could not be created properly, returns an error code.
  // Else returns RE2::NoError (== 0).
  ErrorCode error_code() const { return error_code_; }

  // If RE2 could not be created properly, returns the offending
  // portion of the regexp.
  const std::string& error_arg() const { return *error_arg_; }

  // Returns the program size, a very approximate measure of a regexp's "cost".
  // Larger numbers are more expensive than smaller numbers.
  int ProgramSize() const;
  int ReverseProgramSize() const;

  // If histogram is not null, outputs the program fanout
  // as a histogram bucketed by powers of 2.
  // Returns the number of the largest non-empty bucket.
  int ProgramFanout(std::vector<int>* histogram) const;
  int ReverseProgramFanout(std::vector<int>* histogram) const;

  // Returns the underlying Regexp; not for general use.
  // Returns entire_regexp_ so that callers don't need
  // to know about prefix_ and prefix_foldcase_.
  re2::Regexp* Regexp() const { return entire_regexp_; }

  /***** The array-based matching interface ******/

  // The functions here have names ending in 'N' and are used to implement
  // the functions whose names are the prefix before the 'N'. It is sometimes
  // useful to invoke them directly, but the syntax is awkward, so the 'N'-less
  // versions should be preferred.
  static bool FullMatchN(absl::string_view text, const RE2& re,
                         const Arg* const args[], int n);
  static bool PartialMatchN(absl::string_view text, const RE2& re,
                            const Arg* const args[], int n);
  static bool ConsumeN(absl::string_view* input, const RE2& re,
                       const Arg* const args[], int n);
  static bool FindAndConsumeN(absl::string_view* input, const RE2& re,
                              const Arg* const args[], int n);

 private:
  template <typename F, typename SP>
  static inline bool Apply(F f, SP sp, const RE2& re) {
    return f(sp, re, NULL, 0);
  }

  template <typename F, typename SP, typename... A>
  static inline bool Apply(F f, SP sp, const RE2& re, const A&... a) {
    const Arg* const args[] = {&a...};
    const int n = sizeof...(a);
    return f(sp, re, args, n);
  }

 public:
  // In order to allow FullMatch() et al. to be called with a varying number
  // of arguments of varying types, we use two layers of variadic templates.
  // The first layer constructs the temporary Arg objects. The second layer
  // (above) constructs the array of pointers to the temporary Arg objects.

  /***** The useful part: the matching interface *****/

  // Matches "text" against "re".  If pointer arguments are
  // supplied, copies matched sub-patterns into them.
  //
  // You can pass in a "const char*" or a "std::string" for "text".
  // You can pass in a "const char*" or a "std::string" or a "RE2" for "re".
  //
  // The provided pointer arguments can be pointers to any scalar numeric
  // type, or one of:
  //    std::string        (matched piece is copied to string)
  //    absl::string_view  (string_view is mutated to point to matched piece)
  //    absl::optional<T>  (T is a supported numeric or string type as above)
  //    T                  ("bool T::ParseFrom(const char*, size_t)" must exist)
  //    (void*)NULL        (the corresponding matched sub-pattern is not copied)
  //
  // Returns true iff all of the following conditions are satisfied:
  //   a. "text" matches "re" fully - from the beginning to the end of "text".
  //   b. The number of matched sub-patterns is >= number of supplied pointers.
  //   c. The "i"th argument has a suitable type for holding the
  //      string captured as the "i"th sub-pattern.  If you pass in
  //      NULL for the "i"th argument, or pass fewer arguments than
  //      number of sub-patterns, the "i"th captured sub-pattern is
  //      ignored.
  //
  // CAVEAT: An optional sub-pattern that does not exist in the
  // matched string is assigned the null string.  Therefore, the
  // following returns false because the null string - absence of
  // a string (not even the empty string) - is not a valid number:
  //
  //    int number;
  //    RE2::FullMatch("abc", "[a-z]+(\\d+)?", &number);
  //
  // Use absl::optional<int> instead to handle this case correctly.
  template <typename... A>
  static bool FullMatch(absl::string_view text, const RE2& re, A&&... a) {
    return Apply(FullMatchN, text, re, Arg(std::forward<A>(a))...);
  }

  // Like FullMatch(), except that "re" is allowed to match a substring
  // of "text".
  //
  // Returns true iff all of the following conditions are satisfied:
  //   a. "text" matches "re" partially - for some substring of "text".
  //   b. The number of matched sub-patterns is >= number of supplied pointers.
  //   c. The "i"th argument has a suitable type for holding the
  //      string captured as the "i"th sub-pattern.  If you pass in
  //      NULL for the "i"th argument, or pass fewer arguments than
  //      number of sub-patterns, the "i"th captured sub-pattern is
  //      ignored.
  template <typename... A>
  static bool PartialMatch(absl::string_view text, const RE2& re, A&&... a) {
    return Apply(PartialMatchN, text, re, Arg(std::forward<A>(a))...);
  }

  // Like FullMatch() and PartialMatch(), except that "re" has to match
  // a prefix of the text, and "input" is advanced past the matched
  // text.  Note: "input" is modified iff this routine returns true
  // and "re" matched a non-empty substring of "input".
  //
  // Returns true iff all of the following conditions are satisfied:
  //   a. "input" matches "re" partially - for some prefix of "input".
  //   b. The number of matched sub-patterns is >= number of supplied pointers.
  //   c. The "i"th argument has a suitable type for holding the
  //      string captured as the "i"th sub-pattern.  If you pass in
  //      NULL for the "i"th argument, or pass fewer arguments than
  //      number of sub-patterns, the "i"th captured sub-pattern is
  //      ignored.
  template <typename... A>
  static bool Consume(absl::string_view* input, const RE2& re, A&&... a) {
    return Apply(ConsumeN, input, re, Arg(std::forward<A>(a))...);
  }

  // Like Consume(), but does not anchor the match at the beginning of
  // the text.  That is, "re" need not start its match at the beginning
  // of "input".  For example, "FindAndConsume(s, "(\\w+)", &word)" finds
  // the next word in "s" and stores it in "word".
  //
  // Returns true iff all of the following conditions are satisfied:
  //   a. "input" matches "re" partially - for some substring of "input".
  //   b. The number of matched sub-patterns is >= number of supplied pointers.
  //   c. The "i"th argument has a suitable type for holding the
  //      string captured as the "i"th sub-pattern.  If you pass in
  //      NULL for the "i"th argument, or pass fewer arguments than
  //      number of sub-patterns, the "i"th captured sub-pattern is
  //      ignored.
  template <typename... A>
  static bool FindAndConsume(absl::string_view* input, const RE2& re, A&&... a) {
    return Apply(FindAndConsumeN, input, re, Arg(std::forward<A>(a))...);
  }

  // Replace the first match of "re" in "str" with "rewrite".
  // Within "rewrite", backslash-escaped digits (\1 to \9) can be
  // used to insert text matching corresponding parenthesized group
  // from the pattern.  \0 in "rewrite" refers to the entire matching
  // text.  E.g.,
  //
  //   std::string s = "yabba dabba doo";
  //   ABSL_CHECK(RE2::Replace(&s, "b+", "d"));
  //
  // will leave "s" containing "yada dabba doo"
  //
  // Returns true if the pattern matches and a replacement occurs,
  // false otherwise.
  static bool Replace(std::string* str,
                      const RE2& re,
                      absl::string_view rewrite);

  // Like Replace(), except replaces successive non-overlapping occurrences
  // of the pattern in the string with the rewrite. E.g.
  //
  //   std::string s = "yabba dabba doo";
  //   ABSL_CHECK(RE2::GlobalReplace(&s, "b+", "d"));
  //
  // will leave "s" containing "yada dada doo"
  // Replacements are not subject to re-matching.
  //
  // Because GlobalReplace only replaces non-overlapping matches,
  // replacing "ana" within "banana" makes only one replacement, not two.
  //
  // Returns the number of replacements made.
  static int GlobalReplace(std::string* str,
                           const RE2& re,
                           absl::string_view rewrite);

  // Like Replace, except that if the pattern matches, "rewrite"
  // is copied into "out" with substitutions.  The non-matching
  // portions of "text" are ignored.
  //
  // Returns true iff a match occurred and the extraction happened
  // successfully;  if no match occurs, the string is left unaffected.
  //
  // REQUIRES: "text" must not alias any part of "*out".
  static bool Extract(absl::string_view text,
                      const RE2& re,
                      absl::string_view rewrite,
                      std::string* out);

  // Escapes all potentially meaningful regexp characters in
  // 'unquoted'.  The returned string, used as a regular expression,
  // will match exactly the original string.  For example,
  //           1.5-2.0?
  // may become:
  //           1\.5\-2\.0\?
  static std::string QuoteMeta(absl::string_view unquoted);

  // Computes range for any strings matching regexp. The min and max can in
  // some cases be arbitrarily precise, so the caller gets to specify the
  // maximum desired length of string returned.
  //
  // Assuming PossibleMatchRange(&min, &max, N) returns successfully, any
  // string s that is an anchored match for this regexp satisfies
  //   min <= s && s <= max.
  //
  // Note that PossibleMatchRange() will only consider the first copy of an
  // infinitely repeated element (i.e., any regexp element followed by a '*' or
  // '+' operator). Regexps with "{N}" constructions are not affected, as those
  // do not compile down to infinite repetitions.
  //
  // Returns true on success, false on error.
  bool PossibleMatchRange(std::string* min, std::string* max,
                          int maxlen) const;

  // Generic matching interface

  // Type of match.
  enum Anchor {
    UNANCHORED,         // No anchoring
    ANCHOR_START,       // Anchor at start only
    ANCHOR_BOTH         // Anchor at start and end
  };

  // Return the number of capturing sub-patterns, or -1 if the
  // regexp wasn't valid on construction.  The overall match ($0)
  // does not count: if the regexp is "(a)(b)", returns 2.
  int NumberOfCapturingGroups() const { return num_captures_; }

  // Return a map from names to capturing indices.
  // The map records the index of the leftmost group
  // with the given name.
  // Only valid until the re is deleted.
  const std::map<std::string, int>& NamedCapturingGroups() const;

  // Return a map from capturing indices to names.
  // The map has no entries for unnamed groups.
  // Only valid until the re is deleted.
  const std::map<int, std::string>& CapturingGroupNames() const;

  // General matching routine.
  // Match against text starting at offset startpos
  // and stopping the search at offset endpos.
  // Returns true if match found, false if not.
  // On a successful match, fills in submatch[] (up to nsubmatch entries)
  // with information about submatches.
  // I.e. matching RE2("(foo)|(bar)baz") on "barbazbla" will return true, with
  // submatch[0] = "barbaz", submatch[1].data() = NULL, submatch[2] = "bar",
  // submatch[3].data() = NULL, ..., up to submatch[nsubmatch-1].data() = NULL.
  // Caveat: submatch[] may be clobbered even on match failure.
  //
  // Don't ask for more match information than you will use:
  // runs much faster with nsubmatch == 1 than nsubmatch > 1, and
  // runs even faster if nsubmatch == 0.
  // Doesn't make sense to use nsubmatch > 1 + NumberOfCapturingGroups(),
  // but will be handled correctly.
  //
  // Passing text == absl::string_view() will be handled like any other
  // empty string, but note that on return, it will not be possible to tell
  // whether submatch i matched the empty string or did not match:
  // either way, submatch[i].data() == NULL.
  bool Match(absl::string_view text,
             size_t startpos,
             size_t endpos,
             Anchor re_anchor,
             absl::string_view* submatch,
             int nsubmatch) const;

  // Check that the given rewrite string is suitable for use with this
  // regular expression.  It checks that:
  //   * The regular expression has enough parenthesized subexpressions
  //     to satisfy all of the \N tokens in rewrite
  //   * The rewrite string doesn't have any syntax errors.  E.g.,
  //     '\' followed by anything other than a digit or '\'.
  // A true return value guarantees that Replace() and Extract() won't
  // fail because of a bad rewrite string.
  bool CheckRewriteString(absl::string_view rewrite,
                          std::string* error) const;

  // Returns the maximum submatch needed for the rewrite to be done by
  // Replace(). E.g. if rewrite == "foo \\2,\\1", returns 2.
  static int MaxSubmatch(absl::string_view rewrite);

  // Append the "rewrite" string, with backslash substitutions from "vec",
  // to string "out".
  // Returns true on success.  This method can fail because of a malformed
  // rewrite string.  CheckRewriteString guarantees that the rewrite will
  // be sucessful.
  bool Rewrite(std::string* out,
               absl::string_view rewrite,
               const absl::string_view* vec,
               int veclen) const;

  // Constructor options
  class Options {
   public:
    // The options are (defaults in parentheses):
    //
    //   utf8             (true)  text and pattern are UTF-8; otherwise Latin-1
    //   posix_syntax     (false) restrict regexps to POSIX egrep syntax
    //   longest_match    (false) search for longest match, not first match
    //   log_errors       (true)  log syntax and execution errors to ERROR
    //   max_mem          (see below)  approx. max memory footprint of RE2
    //   literal          (false) interpret string as literal, not regexp
    //   never_nl         (false) never match \n, even if it is in regexp
    //   dot_nl           (false) dot matches everything including new line
    //   never_capture    (false) parse all parens as non-capturing
    //   case_sensitive   (true)  match is case-sensitive (regexp can override
    //                              with (?i) unless in posix_syntax mode)
    //
    // The following options are only consulted when posix_syntax == true.
    // When posix_syntax == false, these features are always enabled and
    // cannot be turned off; to perform multi-line matching in that case,
    // begin the regexp with (?m).
    //   perl_classes     (false) allow Perl's \d \s \w \D \S \W
    //   word_boundary    (false) allow Perl's \b \B (word boundary and not)
    //   one_line         (false) ^ and $ only match beginning and end of text
    //
    // The max_mem option controls how much memory can be used
    // to hold the compiled form of the regexp (the Prog) and
    // its cached DFA graphs.  Code Search placed limits on the number
    // of Prog instructions and DFA states: 10,000 for both.
    // In RE2, those limits would translate to about 240 KB per Prog
    // and perhaps 2.5 MB per DFA (DFA state sizes vary by regexp; RE2 does a
    // better job of keeping them small than Code Search did).
    // Each RE2 has two Progs (one forward, one reverse), and each Prog
    // can have two DFAs (one first match, one longest match).
    // That makes 4 DFAs:
    //
    //   forward, first-match    - used for UNANCHORED or ANCHOR_START searches
    //                               if opt.longest_match() == false
    //   forward, longest-match  - used for all ANCHOR_BOTH searches,
    //                               and the other two kinds if
    //                               opt.longest_match() == true
    //   reverse, first-match    - never used
    //   reverse, longest-match  - used as second phase for unanchored searches
    //
    // The RE2 memory budget is statically divided between the two
    // Progs and then the DFAs: two thirds to the forward Prog
    // and one third to the reverse Prog.  The forward Prog gives half
    // of what it has left over to each of its DFAs.  The reverse Prog
    // gives it all to its longest-match DFA.
    //
    // Once a DFA fills its budget, it flushes its cache and starts over.
    // If this happens too often, RE2 falls back on the NFA implementation.

    // For now, make the default budget something close to Code Search.
    static const int kDefaultMaxMem = 8<<20;

    enum Encoding {
      EncodingUTF8 = 1,
      EncodingLatin1
    };

    Options() :
      max_mem_(kDefaultMaxMem),
      encoding_(EncodingUTF8),
      posix_syntax_(false),
      longest_match_(false),
      log_errors_(true),
      literal_(false),
      never_nl_(false),
      dot_nl_(false),
      never_capture_(false),
      case_sensitive_(true),
      perl_classes_(false),
      word_boundary_(false),
      one_line_(false) {
    }

    /*implicit*/ Options(CannedOptions);

    int64_t max_mem() const { return max_mem_; }
    void set_max_mem(int64_t m) { max_mem_ = m; }

    Encoding encoding() const { return encoding_; }
    void set_encoding(Encoding encoding) { encoding_ = encoding; }

    bool posix_syntax() const { return posix_syntax_; }
    void set_posix_syntax(bool b) { posix_syntax_ = b; }

    bool longest_match() const { return longest_match_; }
    void set_longest_match(bool b) { longest_match_ = b; }

    bool log_errors() const { return log_errors_; }
    void set_log_errors(bool b) { log_errors_ = b; }

    bool literal() const { return literal_; }
    void set_literal(bool b) { literal_ = b; }

    bool never_nl() const { return never_nl_; }
    void set_never_nl(bool b) { never_nl_ = b; }

    bool dot_nl() const { return dot_nl_; }
    void set_dot_nl(bool b) { dot_nl_ = b; }

    bool never_capture() const { return never_capture_; }
    void set_never_capture(bool b) { never_capture_ = b; }

    bool case_sensitive() const { return case_sensitive_; }
    void set_case_sensitive(bool b) { case_sensitive_ = b; }

    bool perl_classes() const { return perl_classes_; }
    void set_perl_classes(bool b) { perl_classes_ = b; }

    bool word_boundary() const { return word_boundary_; }
    void set_word_boundary(bool b) { word_boundary_ = b; }

    bool one_line() const { return one_line_; }
    void set_one_line(bool b) { one_line_ = b; }

    void Copy(const Options& src) {
      *this = src;
    }

    int ParseFlags() const;

   private:
    int64_t max_mem_;
    Encoding encoding_;
    bool posix_syntax_;
    bool longest_match_;
    bool log_errors_;
    bool literal_;
    bool never_nl_;
    bool dot_nl_;
    bool never_capture_;
    bool case_sensitive_;
    bool perl_classes_;
    bool word_boundary_;
    bool one_line_;
  };

  // Returns the options set in the constructor.
  const Options& options() const { return options_; }

  // Argument converters; see below.
  template <typename T>
  static Arg CRadix(T* ptr);
  template <typename T>
  static Arg Hex(T* ptr);
  template <typename T>
  static Arg Octal(T* ptr);

  // Controls the maximum count permitted by GlobalReplace(); -1 is unlimited.
  // FOR FUZZING ONLY.
  static void FUZZING_ONLY_set_maximum_global_replace_count(int i);

 private:
  void Init(absl::string_view pattern, const Options& options);

  bool DoMatch(absl::string_view text,
               Anchor re_anchor,
               size_t* consumed,
               const Arg* const args[],
               int n) const;

  re2::Prog* ReverseProg() const;

  // First cache line is relatively cold fields.
  const std::string* pattern_;    // string regular expression
  Options options_;               // option flags
  re2::Regexp* entire_regexp_;    // parsed regular expression
  re2::Regexp* suffix_regexp_;    // parsed regular expression, prefix_ removed
  const std::string* error_;      // error indicator (or points to empty string)
  const std::string* error_arg_;  // fragment of regexp showing error (or ditto)

  // Second cache line is relatively hot fields.
  // These are ordered oddly to pack everything.
  int num_captures_;              // number of capturing groups
  ErrorCode error_code_ : 29;     // error code (29 bits is more than enough)
  bool longest_match_ : 1;        // cached copy of options_.longest_match()
  bool is_one_pass_ : 1;          // can use prog_->SearchOnePass?
  bool prefix_foldcase_ : 1;      // prefix_ is ASCII case-insensitive
  std::string prefix_;            // required prefix (before suffix_regexp_)
  re2::Prog* prog_;               // compiled program for regexp

  // Reverse Prog for DFA execution only
  mutable re2::Prog* rprog_;
  // Map from capture names to indices
  mutable const std::map<std::string, int>* named_groups_;
  // Map from capture indices to names
  mutable const std::map<int, std::string>* group_names_;

  mutable absl::once_flag rprog_once_;
  mutable absl::once_flag named_groups_once_;
  mutable absl::once_flag group_names_once_;
};

/***** Implementation details *****/

namespace re2_internal {

// Types for which the 3-ary Parse() function template has specializations.
template <typename T> struct Parse3ary : public std::false_type {};
template <> struct Parse3ary<void> : public std::true_type {};
template <> struct Parse3ary<std::string> : public std::true_type {};
template <> struct Parse3ary<absl::string_view> : public std::true_type {};
template <> struct Parse3ary<char> : public std::true_type {};
template <> struct Parse3ary<signed char> : public std::true_type {};
template <> struct Parse3ary<unsigned char> : public std::true_type {};
template <> struct Parse3ary<float> : public std::true_type {};
template <> struct Parse3ary<double> : public std::true_type {};

template <typename T>
bool Parse(const char* str, size_t n, T* dest);

// Types for which the 4-ary Parse() function template has specializations.
template <typename T> struct Parse4ary : public std::false_type {};
template <> struct Parse4ary<long> : public std::true_type {};
template <> struct Parse4ary<unsigned long> : public std::true_type {};
template <> struct Parse4ary<short> : public std::true_type {};
template <> struct Parse4ary<unsigned short> : public std::true_type {};
template <> struct Parse4ary<int> : public std::true_type {};
template <> struct Parse4ary<unsigned int> : public std::true_type {};
template <> struct Parse4ary<long long> : public std::true_type {};
template <> struct Parse4ary<unsigned long long> : public std::true_type {};

template <typename T>
bool Parse(const char* str, size_t n, T* dest, int radix);

// Support absl::optional<T> for all T with a stock parser.
template <typename T> struct Parse3ary<absl::optional<T>> : public Parse3ary<T> {};
template <typename T> struct Parse4ary<absl::optional<T>> : public Parse4ary<T> {};

template <typename T>
bool Parse(const char* str, size_t n, absl::optional<T>* dest) {
  if (str == NULL) {
    if (dest != NULL)
      dest->reset();
    return true;
  }
  T tmp;
  if (Parse(str, n, &tmp)) {
    if (dest != NULL)
      dest->emplace(std::move(tmp));
    return true;
  }
  return false;
}

template <typename T>
bool Parse(const char* str, size_t n, absl::optional<T>* dest, int radix) {
  if (str == NULL) {
    if (dest != NULL)
      dest->reset();
    return true;
  }
  T tmp;
  if (Parse(str, n, &tmp, radix)) {
    if (dest != NULL)
      dest->emplace(std::move(tmp));
    return true;
  }
  return false;
}

}  // namespace re2_internal

class RE2::Arg {
 private:
  template <typename T>
  using CanParse3ary = typename std::enable_if<
      re2_internal::Parse3ary<T>::value,
      int>::type;

  template <typename T>
  using CanParse4ary = typename std::enable_if<
      re2_internal::Parse4ary<T>::value,
      int>::type;

  template <typename T>
  using CanParseFrom = typename std::enable_if<
      std::is_member_function_pointer<
          decltype(static_cast<bool (T::*)(const char*, size_t)>(
              &T::ParseFrom))>::value,
      int>::type;

 public:
  Arg() : Arg(nullptr) {}
  Arg(std::nullptr_t ptr) : arg_(ptr), parser_(DoNothing) {}

  template <typename T, CanParse3ary<T> = 0>
  Arg(T* ptr) : arg_(ptr), parser_(DoParse3ary<T>) {}

  template <typename T, CanParse4ary<T> = 0>
  Arg(T* ptr) : arg_(ptr), parser_(DoParse4ary<T>) {}

  template <typename T, CanParseFrom<T> = 0>
  Arg(T* ptr) : arg_(ptr), parser_(DoParseFrom<T>) {}

  typedef bool (*Parser)(const char* str, size_t n, void* dest);

  template <typename T>
  Arg(T* ptr, Parser parser) : arg_(ptr), parser_(parser) {}

  bool Parse(const char* str, size_t n) const {
    return (*parser_)(str, n, arg_);
  }

 private:
  static bool DoNothing(const char* /*str*/, size_t /*n*/, void* /*dest*/) {
    return true;
  }

  template <typename T>
  static bool DoParse3ary(const char* str, size_t n, void* dest) {
    return re2_internal::Parse(str, n, reinterpret_cast<T*>(dest));
  }

  template <typename T>
  static bool DoParse4ary(const char* str, size_t n, void* dest) {
    return re2_internal::Parse(str, n, reinterpret_cast<T*>(dest), 10);
  }

  template <typename T>
  static bool DoParseFrom(const char* str, size_t n, void* dest) {
    if (dest == NULL) return true;
    return reinterpret_cast<T*>(dest)->ParseFrom(str, n);
  }

  void*         arg_;
  Parser        parser_;
};

template <typename T>
inline RE2::Arg RE2::CRadix(T* ptr) {
  return RE2::Arg(ptr, [](const char* str, size_t n, void* dest) -> bool {
    return re2_internal::Parse(str, n, reinterpret_cast<T*>(dest), 0);
  });
}

template <typename T>
inline RE2::Arg RE2::Hex(T* ptr) {
  return RE2::Arg(ptr, [](const char* str, size_t n, void* dest) -> bool {
    return re2_internal::Parse(str, n, reinterpret_cast<T*>(dest), 16);
  });
}

template <typename T>
inline RE2::Arg RE2::Octal(T* ptr) {
  return RE2::Arg(ptr, [](const char* str, size_t n, void* dest) -> bool {
    return re2_internal::Parse(str, n, reinterpret_cast<T*>(dest), 8);
  });
}

// Silence warnings about missing initializers for members of LazyRE2.
#if defined(__GNUC__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

// Helper for writing global or static RE2s safely.
// Write
//     static LazyRE2 re = {".*"};
// and then use *re instead of writing
//     static RE2 re(".*");
// The former is more careful about multithreaded
// situations than the latter.
//
// N.B. This class never deletes the RE2 object that
// it constructs: that's a feature, so that it can be used
// for global and function static variables.
class LazyRE2 {
 private:
  struct NoArg {};

 public:
  typedef RE2 element_type;  // support std::pointer_traits

  // Constructor omitted to preserve braced initialization in C++98.

  // Pretend to be a pointer to Type (never NULL due to on-demand creation):
  RE2& operator*() const { return *get(); }
  RE2* operator->() const { return get(); }

  // Named accessor/initializer:
  RE2* get() const {
    absl::call_once(once_, &LazyRE2::Init, this);
    return ptr_;
  }

  // All data fields must be public to support {"foo"} initialization.
  const char* pattern_;
  RE2::CannedOptions options_;
  NoArg barrier_against_excess_initializers_;

  mutable RE2* ptr_;
  mutable absl::once_flag once_;

 private:
  static void Init(const LazyRE2* lazy_re2) {
    lazy_re2->ptr_ = new RE2(lazy_re2->pattern_, lazy_re2->options_);
  }

  void operator=(const LazyRE2&);  // disallowed
};

namespace hooks {

// Most platforms support thread_local. Older versions of iOS don't support
// thread_local, but for the sake of brevity, we lump together all versions
// of Apple platforms that aren't macOS. If an iOS application really needs
// the context pointee someday, we can get more specific then...
//
// As per https://github.com/google/re2/issues/325, thread_local support in
// MinGW seems to be buggy. (FWIW, Abseil folks also avoid it.)
#define RE2_HAVE_THREAD_LOCAL
#if (defined(__APPLE__) && !(defined(TARGET_OS_OSX) && TARGET_OS_OSX)) || defined(__MINGW32__)
#undef RE2_HAVE_THREAD_LOCAL
#endif

// A hook must not make any assumptions regarding the lifetime of the context
// pointee beyond the current invocation of the hook. Pointers and references
// obtained via the context pointee should be considered invalidated when the
// hook returns. Hence, any data about the context pointee (e.g. its pattern)
// would have to be copied in order for it to be kept for an indefinite time.
//
// A hook must not use RE2 for matching. Control flow reentering RE2::Match()
// could result in infinite mutual recursion. To discourage that possibility,
// RE2 will not maintain the context pointer correctly when used in that way.
#ifdef RE2_HAVE_THREAD_LOCAL
extern thread_local const RE2* context;
#endif

struct DFAStateCacheReset {
  int64_t state_budget;
  size_t state_cache_size;
};

struct DFASearchFailure {
  // Nothing yet...
};

#define DECLARE_HOOK(type)                  \
  using type##Callback = void(const type&); \
  void Set##type##Hook(type##Callback* cb); \
  type##Callback* Get##type##Hook();

DECLARE_HOOK(DFAStateCacheReset)
DECLARE_HOOK(DFASearchFailure)

#undef DECLARE_HOOK

}  // namespace hooks

}  // namespace re2

using re2::RE2;
using re2::LazyRE2;

#endif  // RE2_RE2_H_
