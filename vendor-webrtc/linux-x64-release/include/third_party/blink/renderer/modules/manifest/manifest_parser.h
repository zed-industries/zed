// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MANIFEST_MANIFEST_PARSER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MANIFEST_MANIFEST_PARSER_H_

#include <stdint.h>

#include <optional>
#include <string>

#include "base/types/strong_alias.h"
#include "services/network/public/cpp/permissions_policy/permissions_policy_declaration.h"
#include "third_party/blink/public/common/manifest/manifest.h"
#include "third_party/blink/public/common/safe_url_pattern.h"
#include "third_party/blink/public/mojom/manifest/manifest.mojom-blink-forward.h"
#include "third_party/blink/public/mojom/manifest/manifest.mojom-blink.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/graphics/color.h"
#include "third_party/blink/renderer/platform/json/json_values.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace gfx {
class Size;
}  // namespace gfx

namespace blink {

class KURL;
class ExecutionContext;

// ManifestParser handles the logic of parsing the Web Manifest from a string.
// It implements:
// https://w3c.github.io/manifest/#processing
// Takes a |ExecutionContext| to check origin trial statuses with.
class MODULES_EXPORT ManifestParser {
  STACK_ALLOCATED();

 public:
  ManifestParser(const String& data,
                 const KURL& manifest_url,
                 const KURL& document_url,
                 ExecutionContext* execution_context);

  ManifestParser(const ManifestParser&) = delete;
  ManifestParser& operator=(const ManifestParser&) = delete;

  ~ManifestParser();

  static void SetFileHandlerExtensionLimitForTesting(int limit);

  // Parse the Manifest from a string using following:
  // https://w3c.github.io/manifest/#processing
  bool Parse();

  bool failed() const;

  // Takes ownership of the Manifest produced by Parse(). Once called, the
  // parser is invalid and should no longer be used.
  mojom::blink::ManifestPtr TakeManifest();

  // Take any errors generated.
  void TakeErrors(Vector<mojom::blink::ManifestErrorPtr>* errors);

 private:
  // Used to indicate whether to strip whitespace when parsing a string.
  using Trim = base::StrongAlias<class TrimTag, bool>;

  // Partially represents `URLPatternInit` in the URL Pattern spec.
  // https://urlpattern.spec.whatwg.org/#dictdef-urlpatterninit
  struct PatternInit {
    PatternInit(std::optional<String> protocol,
                std::optional<String> username,
                std::optional<String> password,
                std::optional<String> hostname,
                std::optional<String> port,
                std::optional<String> pathname,
                std::optional<String> search,
                std::optional<String> hash,
                KURL base_url);
    ~PatternInit();
    PatternInit(const PatternInit&) = delete;
    PatternInit& operator=(const PatternInit&) = delete;
    PatternInit(PatternInit&&);
    PatternInit& operator=(PatternInit&&);

    // Returns true if any of protocol, hostname, or port are filled.
    bool IsAbsolute() const;

    std::optional<String> protocol;
    std::optional<String> username;
    std::optional<String> password;
    std::optional<String> hostname;
    std::optional<String> port;
    std::optional<String> pathname;
    std::optional<String> search;
    std::optional<String> hash;
    KURL base_url;
  };

  // Indicate restrictions to be placed on the parsed URL with respect to the
  // document URL or manifest scope.
  enum class ParseURLRestrictions {
    kNoRestrictions = 0,
    kSameOriginOnly,  // Parsed URLs must be same origin as the document URL.
    kWithinScope,     // Parsed URLs must be within scope of the manifest scope
                      // (implies same origin as document URL).
  };

  // These values are persisted to logs. Entries should not be renumbered and
  // numeric values should never be reused.
  enum class ParseIdResultType {
    kSucceed = 0,
    kDefaultToStartUrl = 1,
    kInvalidStartUrl = 2,
    kFeatureDisabled = 3,  // No longer emitted, feature flag is removed.
    kMaxValue = kFeatureDisabled,
  };

  // Helper function to parse booleans present on a given |dictionary| in a
  // given field identified by its |key|.
  // Returns the parsed boolean if any, or |default_value| if parsing failed.
  bool ParseBoolean(const JSONObject* object,
                    const String& key,
                    bool default_value);

  // Helper function to parse strings present on a given |dictionary| in a given
  // field identified by its |key|.
  // Returns the parsed string if any, a null optional if the parsing failed.
  std::optional<String> ParseString(const JSONObject* object,
                                    const String& key,
                                    Trim trim);

  // Helper function to parse strings present in a member that itself is
  // a dictionary like 'shortcut' as defined in:
  // https://w3c.github.io/manifest/#shortcutitem-and-its-members Each strings
  // will identifiable by its |key|. This helper includes the member_name in any
  // ManifestError added while parsing. This helps disambiguate member property
  // names from top level member names. Returns the parsed string if any, a null
  // optional if the parsing failed.
  std::optional<String> ParseStringForMember(const JSONObject* object,
                                             const String& member_name,
                                             const String& key,
                                             bool required,
                                             Trim trim);

  // Helper function to parse colors present on a given |dictionary| in a given
  // field identified by its |key|. Returns a null optional if the value is not
  // present or is not a valid color.
  std::optional<RGBA32> ParseColor(const JSONObject* object, const String& key);

  // Helper function to parse URLs present on a given |dictionary| in a given
  // field identified by its |key|. The URL is first parsed as a string then
  // resolved using |base_url|. |enforce_document_origin| specified whether to
  // enforce matching of the document's and parsed URL's origins.
  // Returns a KURL. If the parsing failed or origin matching was enforced but
  // not present, the returned KURL will be empty.
  // |ignore_empty_string| treats empty string as the field missing.
  // TODO(crbug.com/1223173): remove ignore_empty_string when all url fields are
  // parsed the same.
  KURL ParseURL(const JSONObject* object,
                const String& key,
                const KURL& base_url,
                ParseURLRestrictions origin_restriction,
                bool ignore_empty_string = false);

  // Helper function to parse "enum" fields that accept a single value or a
  // list of values to allow sites to be backwards compatible with browsers that
  // don't support the latest spec.
  // Example:
  //  - Spec specifies valid field values are A, B and C.
  //  - Browser supports only A and B.
  //  - Site specifies "field": ["C", "B"].
  //  - Browser will fail to parse C and fallback to B.
  template <typename Enum>
  Enum ParseFirstValidEnum(const JSONObject* object,
                           const String& key,
                           Enum (*parse_enum)(const std::string&),
                           Enum invalid_value);

  // Parses the 'dir' field of the manifest, as defined in:
  // https://w3c.github.io/manifest/#dfn-process-the-dir-member
  // Returns the parsed TextDirection.
  mojom::blink::Manifest::TextDirection ParseDir(const JSONObject* object);

  // Parses the 'name' field of the manifest, as defined in:
  // https://w3c.github.io/manifest/#dfn-steps-for-processing-the-name-member
  // Returns the parsed string if any, a null string if the parsing failed.
  String ParseName(const JSONObject* object);

  // Parses the 'short_name' field of the manifest, as defined in:
  // https://w3c.github.io/manifest/#dfn-steps-for-processing-the-short-name-member
  // Returns the parsed string if any, a null string if the parsing failed.
  String ParseShortName(const JSONObject* object);

  // Parses the 'description' field of the manifest, as defined in:
  // https://w3c.github.io/manifest/#description-member-0
  // Returns the parsed string if any, a null string if the parsing failed.
  String ParseDescription(const JSONObject* object);

  // Parses the 'id' field of the manifest.
  std::pair<KURL, ParseIdResultType> ParseId(const JSONObject* object,
                                             const KURL& start_url);

  // Parses the 'scope' field of the manifest, as defined in:
  // https://w3c.github.io/manifest/#scope-member. Returns the parsed KURL if
  // any, or start URL (falling back to document URL) without filename, path,
  // and query if there is no defined scope or if the parsing failed.
  KURL ParseScope(const JSONObject* object, const KURL& start_url);

  enum class ParseStartUrlResult {
    // The start_url was parsed from the json entry without errors.
    kParsedFromJson,
    // There was no start_url entry or parsing failed.
    kDefaultDocumentUrl
  };
  // Parses the 'start_url' field of the manifest, as defined in:
  // https://w3c.github.io/manifest/#dfn-steps-for-processing-the-start_url-member
  std::pair<KURL, ParseStartUrlResult> ParseStartURL(const JSONObject* object,
                                                     const KURL& document_url);

  // Parses the 'display' field of the manifest, as defined in:
  // https://w3c.github.io/manifest/#dfn-steps-for-processing-the-display-member
  // Returns the parsed DisplayMode if any, DisplayMode::kUndefined if the
  // parsing failed.
  blink::mojom::DisplayMode ParseDisplay(const JSONObject* object);

  // Parses the 'display_override' field of the manifest.
  // https://github.com/WICG/display-override/blob/master/explainer.md
  // Returns a vector of the parsed DisplayMode if any, an empty vector if
  // the field was not present or empty.
  Vector<mojom::blink::DisplayMode> ParseDisplayOverride(
      const JSONObject* object);

  // Parses the 'orientation' field of the manifest, as defined in:
  // https://w3c.github.io/manifest/#dfn-steps-for-processing-the-orientation-member
  // Returns the parsed device::mojom::blink::ScreenOrientationLockType if any,
  // device::mojom::blink::ScreenOrientationLockType::DEFAULT if the parsing
  // failed.
  device::mojom::blink::ScreenOrientationLockType ParseOrientation(
      const JSONObject* object);

  // Parses the 'src' field of an icon, as defined in:
  // https://w3c.github.io/manifest/#dfn-steps-for-processing-the-src-member-of-an-image
  // Returns the parsed KURL if any, an empty KURL if the parsing failed.
  KURL ParseIconSrc(const JSONObject* icon);

  // Parses the 'type' field of an icon, as defined in:
  // https://w3c.github.io/manifest/#dfn-steps-for-processing-the-type-member-of-an-image
  // Returns the parsed string if any, an empty string if the parsing failed.
  String ParseIconType(const JSONObject* icon);

  // Parses the 'sizes' field of an icon, as defined in:
  // https://w3c.github.io/manifest/#dfn-steps-for-processing-a-sizes-member-of-an-image
  // Returns a vector of gfx::Size with the successfully parsed sizes, if any.
  // An empty vector if the field was not present or empty. "Any" is represented
  // by gfx::Size(0, 0).
  Vector<gfx::Size> ParseIconSizes(const JSONObject* icon);

  // Parses the 'purpose' field of an icon, as defined in:
  // https://w3c.github.io/manifest/#dfn-steps-for-processing-a-purpose-member-of-an-image
  // Returns a vector of ManifestImageResource::Purpose with the successfully
  // parsed icon purposes, and nullopt if the parsing failed.
  std::optional<Vector<mojom::blink::ManifestImageResource::Purpose>>
  ParseIconPurpose(const JSONObject* icon);

  // Parses the 'icons' field of a Manifest, as defined in:
  // https://w3c.github.io/manifest/#dfn-steps-for-processing-an-array-of-images
  // Returns a vector of ManifestImageResourcePtr with the successfully parsed
  // icons, if any. An empty vector if the field was not present or empty.
  Vector<mojom::blink::ManifestImageResourcePtr> ParseIcons(
      const JSONObject* object);

  // Parses the 'screenshots' field of a Manifest, as defined in:
  // https://www.w3.org/TR/manifest-app-info/#screenshots-member
  // Returns a vector of ManifestImageResourcePtr with the successfully parsed
  // screenshots, if any. An empty vector if the field was not present or empty.
  Vector<mojom::blink::ManifestScreenshotPtr> ParseScreenshots(
      const JSONObject* object);

  // Parse the 'form_factor' field of 'screenshots' as defined in:
  // https://www.w3.org/TR/manifest-app-info/#form_factor-member
  mojom::blink::ManifestScreenshot::FormFactor ParseScreenshotFormFactor(
      const JSONObject* screenshot);

  // Parse the 'label' field of 'screenshots' as defined in:
  // https://www.w3.org/TR/manifest-app-info/#label-member
  String ParseScreenshotLabel(const JSONObject* object);

  // A helper function for parsing ImageResources under |key| in the manifest.
  Vector<mojom::blink::ManifestImageResourcePtr> ParseImageResourceArray(
      const String& key,
      const JSONObject* object);

  std::optional<mojom::blink::ManifestImageResourcePtr> ParseImageResource(
      const JSONValue* object);

  // Parses the 'name' field of a shortcut, as defined in:
  // https://w3c.github.io/manifest/#shortcuts-member
  // Returns the parsed string if any, a null string if the parsing failed.
  String ParseShortcutName(const JSONObject* shortcut);

  // Parses the 'short_name' field of a shortcut, as defined in:
  // https://w3c.github.io/manifest/#shortcuts-member
  // Returns the parsed string if any, a null string if the parsing failed.
  String ParseShortcutShortName(const JSONObject* shortcut);

  // Parses the 'description' field of a shortcut, as defined in:
  // https://w3c.github.io/manifest/#shortcuts-member
  // Returns the parsed string if any, a null string if the parsing failed.
  String ParseShortcutDescription(const JSONObject* shortcut);

  // Parses the 'url' field of an icon, as defined in:
  // https://w3c.github.io/manifest/#shortcuts-member
  // Returns the parsed KURL if any, an empty KURL if the parsing failed.
  KURL ParseShortcutUrl(const JSONObject* shortcut);

  // Parses the 'shortcuts' field of a Manifest, as defined in:
  // https://w3c.github.io/manifest/#shortcuts-member
  // Returns a vector of ManifestShortcutPtr with the successfully parsed
  // shortcuts, if any. An empty vector if the field was not present or empty.
  Vector<mojom::blink::ManifestShortcutItemPtr> ParseShortcuts(
      const JSONObject* object);

  // Parses the name field of a share target file, as defined in:
  // https://wicg.github.io/web-share-target/level-2/#sharetargetfiles-and-its-members
  // Returns the parsed string if any, an empty string if the parsing failed.
  String ParseFileFilterName(const JSONObject* file);

  // Parses the accept field of a file filter, as defined in:
  // https://wicg.github.io/web-share-target/level-2/#sharetargetfiles-and-its-members
  // Returns the vector of parsed strings if any exist, an empty vector if the
  // parsing failed or no accept instances were provided.
  Vector<String> ParseFileFilterAccept(const JSONObject* file);

  // Parses the |key| field of |from| as a list of FileFilters.
  // This is used to parse |file_handlers| and |share_target.params.files|
  // Returns a parsed vector of share target files.
  Vector<mojom::blink::ManifestFileFilterPtr> ParseTargetFiles(
      const String& key,
      const JSONObject* from);

  // Parses a single FileFilter (see above comment) and appends it to
  // the given |files| vector.
  void ParseFileFilter(const JSONObject* file_dictionary,
                       Vector<mojom::blink::ManifestFileFilterPtr>* files);

  // Parses the method field of a Share Target, as defined in:
  // https://wicg.github.io/web-share-target/#sharetarget-and-its-members
  // Returns an optional share target method enum object.
  std::optional<mojom::blink::ManifestShareTarget::Method>
  ParseShareTargetMethod(const JSONObject* share_target_dict);

  // Parses the enctype field of a Share Target, as defined in:
  // https://wicg.github.io/web-share-target/#sharetarget-and-its-members
  // Returns an optional share target enctype enum object.
  std::optional<mojom::blink::ManifestShareTarget::Enctype>
  ParseShareTargetEnctype(const JSONObject* share_target_dict);

  // Parses the 'params' field of a Share Target, as defined in:
  // https://wicg.github.io/web-share-target/#sharetarget-and-its-members
  // Returns a parsed mojom::blink:ManifestShareTargetParamsPtr, not all fields
  // need to be populated.
  mojom::blink::ManifestShareTargetParamsPtr ParseShareTargetParams(
      const JSONObject* share_target_params);

  // Parses the 'share_target' field of a Manifest, as defined in:
  // https://wicg.github.io/web-share-target/#share_target-member
  // Returns the parsed Web Share target. The returned Share Target is null if
  // the field didn't exist, parsing failed, or it was empty.
  std::optional<mojom::blink::ManifestShareTargetPtr> ParseShareTarget(
      const JSONObject* object);

  // Keep in sync with ScopeExtensionTypeMap. E.g. kOrigin -> "origin"
  enum class ScopeExtensionType {
    kOrigin = 0,
  };
  static constexpr const char* ScopeExtensionTypeMap[1] = {"origin"};

  // Parses the 'scope_extensions' field of a Manifest, as defined in:
  // https://github.com/WICG/manifest-incubations/blob/gh-pages/scope_extensions-explainer.md
  // Returns the parsed list of ScopeExtensions. The returned ScopeExtensions
  // are empty if the field didn't exist, parsing failed, the input list was
  // empty, or if the blink feature flag is disabled.
  // This feature is experimental and is only enabled by the blink feature flag:
  // blink::features::kWebAppEnableScopeExtensions.
  Vector<mojom::blink::ManifestScopeExtensionPtr> ParseScopeExtensions(
      const JSONObject* object);

  // Parses a single scope extension entry in 'scope_extensions', as defined in:
  // https://github.com/WICG/manifest-incubations/blob/gh-pages/scope_extensions-explainer.md
  // Returns |std::nullopt| if the ScopeExtension was invalid, or a
  // ScopeExtension if parsing succeeded.
  // This feature is experimental and is only enabled by the blink feature flag:
  // blink::features::kWebAppEnableScopeExtensions.
  std::optional<mojom::blink::ManifestScopeExtensionPtr> ParseScopeExtension(
      const JSONObject* object);

  // Parses a single scope extension origin in 'scope_extensions', as defined
  // in:
  // https://github.com/WICG/manifest-incubations/blob/gh-pages/scope_extensions-explainer.md
  // Returns |std::nullopt| if the ScopeExtension origin was invalid, or a
  // ScopeExtension if parsing succeeded.
  // This feature is experimental and is only enabled by the blink feature flag:
  // blink::features::kWebAppEnableScopeExtensions.
  std::optional<mojom::blink::ManifestScopeExtensionPtr>
  ParseScopeExtensionOrigin(const String& origin_string);

  // Parses the 'file_handlers' field of a Manifest, as defined in:
  // https://github.com/WICG/file-handling/blob/main/explainer.md
  // Returns the parsed list of FileHandlers. The returned FileHandlers are
  // empty if the field didn't exist, parsing failed, or the input list was
  // empty.
  Vector<mojom::blink::ManifestFileHandlerPtr> ParseFileHandlers(
      const JSONObject* object);

  // Parses a FileHandler from an entry in the 'file_handlers' list, as
  // defined in: https://github.com/WICG/file-handling/blob/main/explainer.md.
  // Returns |std::nullopt| if the FileHandler was invalid, or a
  // FileHandler, if parsing succeeded.
  std::optional<mojom::blink::ManifestFileHandlerPtr> ParseFileHandler(
      const JSONObject* file_handler_entry);

  // Parses the 'accept' field of a FileHandler, as defined in:
  // https://github.com/WICG/file-handling/blob/main/explainer.md.
  // Returns the parsed accept map. Invalid accept entries are ignored.
  HashMap<String, Vector<String>> ParseFileHandlerAccept(
      const JSONObject* accept);

  // Parses an extension in the 'accept' field of a FileHandler, as defined in:
  // https://github.com/WICG/file-handling/blob/main/explainer.md. Returns
  // whether the parsing was successful and, if so, populates |output| with the
  // parsed extension.
  bool ParseFileHandlerAcceptExtension(const JSONValue* extension,
                                       String* ouput);

  // Parses the 'protocol_handlers' field of a Manifest, as defined in:
  // https://github.com/MicrosoftEdge/MSEdgeExplainers/blob/master/URLProtocolHandler/explainer.md
  // Returns the parsed list of ProtocolHandlers. The returned ProtocolHandlers
  // are empty if the field didn't exist, parsing failed, or the input list was
  // empty.
  Vector<mojom::blink::ManifestProtocolHandlerPtr> ParseProtocolHandlers(
      const JSONObject* object);

  // Parses a single ProtocolHandle field of a Manifest, as defined in:
  // https://github.com/MicrosoftEdge/MSEdgeExplainers/blob/master/URLProtocolHandler/explainer.md
  // Returns |std::nullopt| if the ProtocolHandler was invalid, or a
  // ProtocolHandler if parsing succeeded.
  std::optional<mojom::blink::ManifestProtocolHandlerPtr> ParseProtocolHandler(
      const JSONObject* protocol_dictionary);

  // Parses the 'start_url' field of the 'lock_screen' field of a Manifest,
  // as defined in:
  // https://github.com/WICG/lock-screen/
  // Returns the parsed KURL if any, or an empty KURL if parsing failed.
  KURL ParseLockScreenStartUrl(const JSONObject* lock_screen);

  // Parses the 'lock_screen' field of a Manifest, as defined in:
  // https://github.com/WICG/lock-screen/
  // Returns a parsed ManifestLockScreenPtr, or nullptr if not present or
  // parsing failed.
  mojom::blink::ManifestLockScreenPtr ParseLockScreen(
      const JSONObject* manifest);

  // Parses the 'new_note_url' field of the 'note_taking' field of a Manifest,
  // as defined in:
  // https://wicg.github.io/manifest-incubations/#dfn-new_note_url
  // Returns the parsed KURL if any, or an empty KURL if parsing failed.
  KURL ParseNoteTakingNewNoteUrl(const JSONObject* note_taking);

  // Parses the 'note_taking' field of a Manifest, as defined in:
  // https://wicg.github.io/manifest-incubations/index.html#dfn-note_taking
  // Returns a parsed ManifestNoteTakingPtr, or nullptr if not present or
  // parsing failed.
  mojom::blink::ManifestNoteTakingPtr ParseNoteTaking(
      const JSONObject* manifest);

  // Parses the 'platform' field of a related application, as defined in:
  // https://w3c.github.io/manifest/#dfn-steps-for-processing-the-platform-member-of-an-application
  // Returns the parsed string if any, a null string if the parsing failed.
  String ParseRelatedApplicationPlatform(const JSONObject* application);

  // Parses the 'url' field of a related application, as defined in:
  // https://w3c.github.io/manifest/#dfn-steps-for-processing-the-url-member-of-an-application
  // Returns the parsed KURL if any, a null optional if the parsing failed.
  std::optional<KURL> ParseRelatedApplicationURL(const JSONObject* application);

  // Parses the 'id' field of a related application, as defined in:
  // https://w3c.github.io/manifest/#dfn-steps-for-processing-the-id-member-of-an-application
  // Returns the parsed string if any, a null string if the parsing failed.
  String ParseRelatedApplicationId(const JSONObject* application);

  // Parses the 'related_applications' field of the manifest, as defined in:
  // https://w3c.github.io/manifest/#dfn-steps-for-processing-the-related_applications-member
  // Returns a vector of ManifestRelatedApplicationPtr with the successfully
  // parsed applications, if any. An empty vector if the field was not present
  // or empty.
  Vector<mojom::blink::ManifestRelatedApplicationPtr> ParseRelatedApplications(
      const JSONObject* object);

  // Parses the 'prefer_related_applications' field on the manifest, as defined
  // in:
  // https://w3c.github.io/manifest/#dfn-steps-for-processing-the-prefer_related_applications-member
  // returns true iff the field could be parsed as the boolean true.
  bool ParsePreferRelatedApplications(const JSONObject* object);

  // Parses the 'theme_color' field of the manifest, as defined in:
  // https://w3c.github.io/manifest/#dfn-steps-for-processing-the-theme_color-member
  // Returns the parsed theme color if any, or a null optional otherwise.
  std::optional<RGBA32> ParseThemeColor(const JSONObject* object);

  // Parses the 'background_color' field of the manifest, as defined in:
  // https://w3c.github.io/manifest/#dfn-steps-for-processing-the-background_color-member
  // Returns the parsed background color if any, or a null optional otherwise.
  std::optional<RGBA32> ParseBackgroundColor(const JSONObject* object);

  // Parses the 'gcm_sender_id' field of the manifest.
  // This is a proprietary extension of the Web Manifest specification.
  // Returns the parsed string if any, a null string if the parsing failed.
  String ParseGCMSenderID(const JSONObject* object);

  // Parses the 'permissions_policy' field of the manifest.
  // This outsources semantic parsing of the policy to the
  // PermissionsPolicyParser.
  Vector<network::ParsedPermissionsPolicyDeclaration>
  ParseIsolatedAppPermissions(const JSONObject* object);
  Vector<String> ParseOriginAllowlist(const JSONArray* allowlist,
                                      const String& feature);

  // Parses the 'launch_handler' field of the manifest as defined in:
  // https://github.com/WICG/web-app-launch/blob/main/launch_handler.md
  // Returns default values if parsing fails.
  mojom::blink::ManifestLaunchHandlerPtr ParseLaunchHandler(
      const JSONObject* object);

  // Parses the 'translations' field of the manifest as defined in:
  // https://github.com/WICG/manifest-incubations/blob/gh-pages/translations-explainer.md
  // Returns empty map if parsing fails.
  HashMap<String, mojom::blink::ManifestTranslationItemPtr> ParseTranslations(
      const JSONObject* object);

  // Parses the 'tab_strip' field of the manifest as defined in:
  // https://github.com/WICG/manifest-incubations/blob/gh-pages/tabbed-mode-explainer.md
  mojom::blink::ManifestTabStripPtr ParseTabStrip(const JSONObject* object);

  mojom::blink::TabStripMemberVisibility ParseTabStripMemberVisibility(
      const JSONValue* json_value);

  // Parses the 'scope_patterns' field of the 'tab_strip.home_tab' field
  // of the manifest.
  Vector<SafeUrlPattern> ParseScopePatterns(const JSONObject* object);

  // Helper method to parse individual scope patterns.
  std::optional<SafeUrlPattern> ParseScopePattern(const PatternInit& init,
                                                  const KURL& base_url);

  std::optional<PatternInit> MaybeCreatePatternInit(
      const JSONObject* pattern_object);

  String ParseVersion(const JSONObject* object);

  // Parses the `update_token` field in the manifest iff the `id` is defined in
  // the manifest. Returns `String()` which is null otherwise.
  String ParseUpdateToken(const JSONObject* object, bool has_custom_id);

  void AddErrorInfo(const String& error_msg,
                    bool critical = false,
                    int error_line = 0,
                    int error_column = 0);

  const String data_;
  KURL manifest_url_;
  KURL document_url_;
  ExecutionContext* execution_context_;

  // The total number of file extensions seen so far while parsing
  // `file_handlers` `accept` entries.
  int total_file_handler_extension_count_ = 0;

  bool failed_;
  mojom::blink::ManifestPtr manifest_;
  Vector<mojom::blink::ManifestErrorPtr> errors_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MANIFEST_MANIFEST_PARSER_H_
