// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_URL_PATTERN_URL_PATTERN_COMPONENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_URL_PATTERN_URL_PATTERN_COMPONENT_H_

#include <optional>

#include "base/types/pass_key.h"
#include "third_party/blink/renderer/platform/bindings/script_regexp.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/trace_traits.h"
#include "third_party/blink/renderer/platform/wtf/text/string_view.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/liburlpattern/parse.h"
#include "third_party/liburlpattern/pattern.h"

namespace blink {

class ExceptionState;
class URLPatternOptions;

namespace url_pattern {

// A struct representing all the information needed to match a particular
// component of a URL.
class Component final : public GarbageCollected<Component> {
 public:
  // Enumeration defining the different types of components.  Each component
  // type uses a slightly different kind of character encoding.  In addition,
  // different component types using different liburlpattern parse options.
  enum class Type {
    kProtocol,
    kUsername,
    kPassword,
    kHostname,
    kPort,
    kPathname,
    kSearch,
    kHash,
  };
  Type type() const { return type_; }

  // A utility function that takes a given `pattern` and compiles it into a
  // Component structure.  If the `pattern` is null, then it will be defaulted
  // to `*`.  The `type` specifies which URL component is the pattern is being
  // compiled for.  This will select the correct encoding callback,
  // liburlpattern options, and populate errors messages with the correct
  // component string.
  static Component* Compile(v8::Isolate* isolate,
                            StringView pattern,
                            Type type,
                            Component* protocol_component,
                            const URLPatternOptions& external_options,
                            ExceptionState& exception_state);

  // Compare the pattern strings in the two given components.  This provides a
  // mostly lexicographical ordering based on fixed text in the patterns.
  // Matching groups and modifiers are treated such that more restrictive
  // patterns are greater in value.  Group names are not considered in the
  // comparison.
  static int Compare(const Component& lh, const Component& rh);

  // Constructs a Component with a real `pattern` that compiled to the given
  // `regexp`.
  Component(Type type,
            liburlpattern::Pattern pattern,
            ScriptRegexp* regexp,
            Vector<String> name_list,
            base::PassKey<Component> key);

  // Match the given `input` against the component pattern.  Returns `true`
  // if there is a match.  If `group_list` is not nullptr, then it will be
  // populated with group name:value tuples captured by the pattern.
  bool Match(StringView input,
             Vector<std::pair<String, String>>* group_list) const;

  // Convert the compiled component pattern back into a pattern string.  This
  // will be functionally equivalent to the original, but may differ based on
  // canonicalization that occurred during parsing.
  String GeneratePatternString() const;

  // Method to determine if the URL associated with this component should be
  // treated as a "standard" URL like `https://foo` vs a "path" URL like
  // `data:foo`.  This should only be called for kProtocol components.
  bool ShouldTreatAsStandardURL() const;

  // Returns if this component has at least one part that uses an ECMAScript
  // regular expression.
  bool HasRegExpGroups() const { return pattern_.HasRegexGroups(); }

  const std::vector<liburlpattern::Part>& PartList() const;

  void Trace(Visitor* visitor) const;

 private:
  const Type type_;

  // The parsed pattern.
  const liburlpattern::Pattern pattern_;

  // The pattern compiled down to a js regular expression.  This is only
  // generated if `pattern_.CanDirectMatch()` returns false.
  const Member<ScriptRegexp> regexp_;

  // The names to be applied to the regular expression capture groups.  Note,
  // liburlpattern regular expressions do not use named capture groups directly.
  // `name_list_` will only be populated if `pattern_.CanDirectMatch()` returns
  // false.
  const Vector<String> name_list_;

  // The cached result of computing if a protocol component should cause the
  // pattern to be treated as a standard URL.  This should only be set and read
  // by protocol components executing ShouldTreatAsStandardURL().
  mutable std::optional<bool> should_treat_as_standard_url_;
};

}  // namespace url_pattern
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_URL_PATTERN_URL_PATTERN_COMPONENT_H_
