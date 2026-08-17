// Copyright 2021 The Chromium Authors
// Use of this source code is governed by an MIT-style license that can be
// found in the LICENSE file or at https://opensource.org/licenses/MIT.

#ifndef THIRD_PARTY_LIBURLPATTERN_CONSTRUCTOR_STRING_PARSER_H_
#define THIRD_PARTY_LIBURLPATTERN_CONSTRUCTOR_STRING_PARSER_H_

#include <functional>
#include <optional>
#include <string_view>

#include "base/component_export.h"
#include "third_party/abseil-cpp/absl/status/status.h"
#include "third_party/liburlpattern/tokenize.h"

namespace liburlpattern {

// A helper class to parse the first string passed to the URLPattern
// constructor.  In general the parser works by using the liburlpattern
// tokenizer to first split up the input into pattern tokens.  It can
// then look through the tokens to find non-special characters that match
// the different URL component separators.  Each component is then split
// off and stored in a `Result` object that can be accessed via `GetResult()`.
// The intent is that this object should then be processed as if it was passed
// into the constructor itself.
class COMPONENT_EXPORT(LIBURLPATTERN) ConstructorStringParser {
 public:
  struct Result {
    std::optional<std::string_view> protocol;
    std::optional<std::string_view> username;
    std::optional<std::string_view> password;
    std::optional<std::string_view> hostname;
    std::optional<std::string_view> port;
    std::optional<std::string_view> pathname;
    std::optional<std::string_view> search;
    std::optional<std::string_view> hash;
  };
  using ProtocolCheckCallback =
      std::function<absl::StatusOr<bool>(std::string_view)>;

  explicit ConstructorStringParser(std::string_view constructor_string);

  // Attempt to parse the input string used to construct the Parser object.
  // This method may only be called once.  Retrieve the parse result by calling
  // `GetResult()`.
  // `protocol_matches_special_scheme` is called with a protocol string. It must
  // return whether the protocol component which is compiled from the protocol
  // string matches a special scheme. It is not called for relative pattern
  // string. The protocol component created inside the callback can be reused
  // when creating a URLPattern object.
  absl::Status Parse(ProtocolCheckCallback protocol_matches_special_scheme);

  // Return the parse result.  Should only be called after `Parse()` succeeds.
  const Result& GetResult() const { return result_; }

 private:
  enum class StringParseState {
    kInit,
    kProtocol,
    kAuthority,
    kUsername,
    kPassword,
    kHostname,
    kPort,
    kPathname,
    kSearch,
    kHash,
    kDone,
  };

  using Skip = int;

  // A utility function to move from the current `state_` to `new_state`.  This
  // method will populate the component string in `result_` corresponding to the
  // current `state_` automatically.  It will also set `component_start_` and
  // `token_index_` to point to the first token of the next section based on how
  // many tokens the `skip` argument indicates should be ignored.
  void ChangeState(StringParseState new_state, Skip skip);

  // A utility function to move to `new_state`.  This is like `ChangeState()`,
  // but does not automatically set the component string for the current state.
  void ChangeStateWithoutSettingComponent(StringParseState new_state,
                                          Skip skip);

  // Rewind the `token_index_` back to the current `component_start_`.
  void Rewind();

  // Like `Rewind()`, but also sets the state.  This is used for cases where
  // the parser needs to "look ahead" to determine what parse state to enter.
  void RewindAndSetState(StringParseState new_state);

  // Attempt to access the Token at the given `index`.  If the `index` is out
  // of bounds for the `token_list_`, then the last Token in the list is
  // returned.  This will always be a `TokenType::kEnd` token.
  const Token& SafeToken(size_t index) const;

  // Returns true if the token at the given `index` is not a special pattern
  // character and if it matches the given `value`.  This simply checks that the
  // token type is kChar, kEscapedChar, or kInvalidChar.
  bool IsNonSpecialPatternChar(size_t index, const char* value) const;

  // Returns true if the token at the given `index` is the protocol component
  // suffix; e.g. ':'.
  bool IsProtocolSuffix() const;

  // Returns true if the next two tokens are slashes; e.g. `//`.
  bool NextIsAuthoritySlashes() const;

  // Returns true if the tokan at the given `index` is the `@` character used
  // to separate username and password from the hostname.
  bool IsIdentityTerminator() const;

  // Returns true if the current token is the password prefix; e.g. `:`.
  bool IsPasswordPrefix() const;

  // Returns true if the current token is the port prefix; e.g. `:`.
  bool IsPortPrefix() const;

  // Returns true if the current token is the start of the pathname; e.g. `/`.
  bool IsPathnameStart() const;

  // Returns true if the current token is the search component prefix; e.g. `?`.
  // This also takes into account if this could be a valid pattern modifier by
  // looking at the preceding tokens.
  bool IsSearchPrefix() const;

  // Returns true if the current token is the hsah component prefix; e.g. `#`.
  bool IsHashPrefix() const;

  // These methods indicate if the current token is opening or closing a pattern
  // grouping; e.g. `{` or `}`.
  bool IsGroupOpen() const;
  bool IsGroupClose() const;

  // These methods indicate if the current token is an opening or closing
  // bracket for an ipv6 hostname; e.g. '[' or ']'.
  bool IsIPv6Open() const;
  bool IsIPv6Close() const;

  // This method returns a std::string_view consisting of the tokens between
  // `component_start_` and the current `token_index_`.
  std::string_view MakeComponentString() const;

  // The input UTF-8 string to the parser.
  const std::string_view input_;

  // The list of Tokens produced by calling `Tokenize()` on `input_`.
  std::vector<Token> token_list_;

  // As we parse the input string we populate a `URLPatternInit` dictionary
  // with each component pattern.  This is then the final result of the parse.
  Result result_;

  // The index of the first Token to include in the component string.
  size_t component_start_ = 0;

  // The index of the current Token being considered.
  size_t token_index_ = 0;

  // The value to add to `token_index_` on each turn the through the parse
  // loop.  While typically this is `1`, it is also set to `0` at times for
  // things like state transitions, etc.  It is automatically reset back to
  // `1` at the top of the parse loop.
  size_t token_increment_ = 1;

  // The current nesting depth of `{ }` pattern groupings.
  int group_depth_ = 0;

  // The current netsting depth of `[ ]` in hostname patterns.
  int hostname_ipv6_bracket_depth_ = 0;

  // The current parse state.  This should only be changed via `ChangeState()`
  // or `RewindAndSetState()`.
  StringParseState state_ = StringParseState::kInit;

  // True if we should apply parse rules as if this is a "standard" URL.  If
  // false then this is treated as a "not a base URL" or "path" URL.
  bool should_treat_as_standard_url_ = false;
};

}  // namespace liburlpattern

#endif  // THIRD_PARTY_LIBURLPATTERN_CONSTRUCTOR_STRING_PARSER_H_
