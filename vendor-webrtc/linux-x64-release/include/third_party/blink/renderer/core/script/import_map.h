// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_IMPORT_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_IMPORT_MAP_H_

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/script/import_map_error.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/loader/fetch/integrity_metadata.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/weborigin/kurl_hash.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"

namespace blink {

class ConsoleLogger;
class ExecutionContext;
class ImportMapError;
class JSONObject;
class ParsedSpecifier;

// Import maps.
// https://html.spec.whatwg.org/C#import-maps
class CORE_EXPORT ImportMap final : public GarbageCollected<ImportMap> {
 public:
  static ImportMap* Parse(const String& text,
                          const KURL& base_url,
                          ExecutionContext& context,
                          std::optional<ImportMapError>* error_to_rethrow);

  // <spec href="https://html.spec.whatwg.org/C#module-specifier-map">A
  // specifier map is an ordered map from strings to resolution results.</spec>
  //
  // An invalid KURL corresponds to a null resolution result in the spec.
  using SpecifierMap = HashMap<AtomicString, KURL>;

  // <spec href="https://html.spec.whatwg.org/C#concept-import-map-scopes">an
  // ordered map of URLs to specifier maps.</spec>
  //
  // Since we don't have an ordered map, we're using a combination of a map and
  // a sorted vector.
  using ScopesMap = HashMap<AtomicString, SpecifierMap>;
  using ScopesVector = Vector<AtomicString>;

  using IntegrityMap = HashMap<KURL, String>;

  // Empty import map.
  ImportMap();

  ImportMap(const ImportMap&);

  ImportMap(SpecifierMap&& imports,
            ScopesMap&& scopes_map,
            IntegrityMap&& integrity);

  // Return values of Resolve(), ResolveImportsMatch() and
  // ResolveImportsMatchInternal():
  // - std::nullopt: corresponds to returning a null in the spec,
  //   i.e. allowing fallback to a less specific scope etc.
  // - An invalid KURL: corresponds to throwing an error in the spec.
  // - A valid KURL: corresponds to returning a valid URL in the spec.
  std::optional<KURL> Resolve(const ParsedSpecifier&,
                              const KURL& base_url,
                              String* debug_message) const;
  String ResolveIntegrity(const KURL& module_url) const;

  // https://html.spec.whatwg.org/C/#merge-existing-and-new-import-maps
  // `new_import_map` is modified in place here, and should not be used after
  // this call.
  void MergeExistingAndNewImportMaps(
      ImportMap* new_import_map,
      const HashMap<AtomicString, HashSet<AtomicString>>&
          scoped_resolved_module_map,
      const HashSet<AtomicString>& toplevel_resolved_module_set,
      ConsoleLogger&);

  String ToStringForTesting() const;

  void Trace(Visitor*) const {}

 private:
  using MatchResult = SpecifierMap::const_iterator;

  // https://html.spec.whatwg.org/C#resolving-an-imports-match
  std::optional<KURL> ResolveImportsMatch(const ParsedSpecifier&,
                                          const SpecifierMap&,
                                          String* debug_message) const;
  std::optional<MatchResult> MatchPrefix(const ParsedSpecifier&,
                                         const SpecifierMap&) const;
  static SpecifierMap SortAndNormalizeSpecifierMap(const JSONObject* imports,
                                                   const KURL& base_url,
                                                   ConsoleLogger&);

  KURL ResolveImportsMatchInternal(const String& normalizedSpecifier,
                                   const MatchResult&,
                                   String* debug_message) const;

  void InitializeScopesVector();

  // https://html.spec.whatwg.org/C#concept-import-map-imports
  SpecifierMap imports_;

  // https://html.spec.whatwg.org/C#concept-import-map-scopes
  ScopesMap scopes_map_;
  // This contains the sorted keys of scopes_map_, used to iterate over it in
  // order.
  ScopesVector scopes_vector_;

  // https://html.spec.whatwg.org/C#concept-import-map-integrity
  IntegrityMap integrity_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SCRIPT_IMPORT_MAP_H_
