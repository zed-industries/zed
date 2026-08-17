// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_MANIFEST_MANIFEST_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_MANIFEST_MANIFEST_H_

#include <stddef.h>
#include <stdint.h>

#include <optional>
#include <string>
#include <variant>
#include <vector>

#include "mojo/public/cpp/bindings/struct_traits.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/safe_url_pattern.h"
#include "third_party/blink/public/mojom/manifest/manifest.mojom-forward.h"
#include "third_party/blink/public/mojom/manifest/manifest.mojom-shared.h"
#include "third_party/blink/public/mojom/manifest/manifest_launch_handler.mojom-forward.h"
#include "ui/gfx/geometry/size.h"
#include "url/gurl.h"
#include "url/origin.h"

namespace blink {

class BLINK_COMMON_EXPORT Manifest {
 public:
  // Structure representing an icon as per the Manifest specification, see:
  // https://w3c.github.io/manifest/#dom-imageresource
  struct BLINK_COMMON_EXPORT ImageResource {
    ImageResource();
    ImageResource(const ImageResource& other);
    ~ImageResource();

    bool operator==(const ImageResource& other) const;

    // MUST be a valid url. If an icon doesn't have a valid URL, it will not be
    // successfully parsed, thus will not be represented in the Manifest.
    GURL src;

    // Empty if the parsing failed or the field was not present. The type can be
    // any string and doesn't have to be a valid image MIME type at this point.
    // It is up to the consumer of the object to check if the type matches a
    // supported type.
    std::u16string type;

    // Empty if the parsing failed, the field was not present or empty.
    // The special value "any" is represented by gfx::Size(0, 0).
    std::vector<gfx::Size> sizes;

    // Never empty. Defaults to a vector with a single value, IconPurpose::ANY,
    // if not explicitly specified in the manifest.
    std::vector<mojom::ManifestImageResource_Purpose> purpose;
  };

  // Structure representing a shortcut as per the Manifest specification, see:
  // https://w3c.github.io/manifest/#shortcuts-member
  struct BLINK_COMMON_EXPORT ShortcutItem {
    ShortcutItem();
    ~ShortcutItem();

    bool operator==(const ShortcutItem& other) const;

    std::u16string name;
    std::optional<std::u16string> short_name;
    std::optional<std::u16string> description;
    GURL url;
    std::vector<ImageResource> icons;
  };

  struct BLINK_COMMON_EXPORT FileFilter {
    bool operator==(const FileFilter& other) const;

    std::u16string name;
    std::vector<std::u16string> accept;
  };

  // Structure representing a Web Share target's query parameter keys.
  struct BLINK_COMMON_EXPORT ShareTargetParams {
    ShareTargetParams();
    ~ShareTargetParams();

    bool operator==(const ShareTargetParams& other) const;

    std::optional<std::u16string> title;
    std::optional<std::u16string> text;
    std::optional<std::u16string> url;
    std::vector<FileFilter> files;
  };

  // Structure representing how a Web Share target handles an incoming share.
  struct BLINK_COMMON_EXPORT ShareTarget {
    ShareTarget();
    ~ShareTarget();

    bool operator==(const ShareTarget& other) const;

    // The URL used for sharing. Query parameters are added to this comprised of
    // keys from |params| and values from the shared data.
    GURL action;

    // The HTTP request method for the web share target.
    blink::mojom::ManifestShareTarget_Method method;

    // The way that share data is encoded in "POST" request.
    blink::mojom::ManifestShareTarget_Enctype enctype;

    ShareTargetParams params;
  };

  // Structure representing a related application.
  struct BLINK_COMMON_EXPORT RelatedApplication {
    RelatedApplication();
    ~RelatedApplication();

    bool operator==(const RelatedApplication& other) const;

    // The platform on which the application can be found. This can be any
    // string, and is interpreted by the consumer of the object. Empty if the
    // parsing failed.
    std::optional<std::u16string> platform;

    // URL at which the application can be found. One of |url| and |id| must be
    // present. Empty if the parsing failed or the field was not present.
    GURL url;

    // An id which is used to represent the application on the platform. One of
    // |url| and |id| must be present. Empty if the parsing failed or the field
    // was not present.
    std::optional<std::u16string> id;
  };

  // This class wraps mojom::blink::ManifestLaunchHandler but with the following
  // changes:
  // 1. Copy constructor support (See crbug.com/1236358).
  // 2. Additional client mode parsing so that callsites don't have to worry
  // about invalid values of the client_mode in the manifest.
  // 3. Ability to determine if the client mode was directly provided in the
  // manifest.
  // See ManifestLaunchHandler for class comments.
  class BLINK_COMMON_EXPORT LaunchHandler {
   public:
    using ClientMode = mojom::ManifestLaunchHandler_ClientMode;

    LaunchHandler();
    explicit LaunchHandler(std::optional<ClientMode> client_mode);

    ClientMode parsed_client_mode() const;
    bool client_mode_valid_and_specified() const;

    bool operator==(const LaunchHandler& other) const;
    bool operator!=(const LaunchHandler& other) const;

    bool TargetsExistingClients() const;
    bool NeverNavigateExistingClients() const;

   private:
    friend struct mojo::StructTraits<
        blink::mojom::ManifestLaunchHandlerDataView,
        ::blink::Manifest::LaunchHandler>;
    std::optional<ClientMode> client_mode_;
  };

  // Structure containing translations for the translatable manifest fields.
  struct BLINK_COMMON_EXPORT TranslationItem {
    TranslationItem();
    ~TranslationItem();

    bool operator==(const TranslationItem& other) const;

    std::optional<std::string> name;
    std::optional<std::string> short_name;
    std::optional<std::string> description;
  };

  // Parameters for the home tab customisation to the tab strip.
  struct BLINK_COMMON_EXPORT HomeTabParams {
    HomeTabParams();
    ~HomeTabParams();

    bool operator==(const HomeTabParams& other) const;

    std::vector<ImageResource> icons;
    std::vector<SafeUrlPattern> scope_patterns;
  };

  // Parameters for the new tab button customisation to the tab strip.
  struct BLINK_COMMON_EXPORT NewTabButtonParams {
    NewTabButtonParams();
    ~NewTabButtonParams();

    bool operator==(const NewTabButtonParams& other) const;

    std::optional<GURL> url;
  };

  // Structure containing customisations for the tab strip.
  struct BLINK_COMMON_EXPORT TabStrip {
    TabStrip();
    ~TabStrip();

    bool operator==(const TabStrip& other) const;

    using Visibility = blink::mojom::TabStripMemberVisibility;
    using HomeTab = std::variant<Visibility, blink::Manifest::HomeTabParams>;
    using NewTabButton = blink::Manifest::NewTabButtonParams;

    HomeTab home_tab;
    NewTabButton new_tab_button;
  };
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_MANIFEST_MANIFEST_H_
