// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_FENCED_FRAME_REDACTED_FENCED_FRAME_CONFIG_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_FENCED_FRAME_REDACTED_FENCED_FRAME_CONFIG_MOJOM_TRAITS_H_

#include "mojo/public/cpp/bindings/enum_traits.h"
#include "mojo/public/cpp/bindings/union_traits.h"
#include "services/network/public/cpp/permissions_policy/permissions_policy_declaration.h"
#include "services/network/public/cpp/permissions_policy/permissions_policy_mojom_traits.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/fenced_frame/redacted_fenced_frame_config.h"
#include "third_party/blink/public/mojom/fenced_frame/fenced_frame_config.mojom.h"

namespace mojo {

template <typename T>
using Prop = blink::FencedFrame::RedactedFencedFrameProperty<T>;

template <>
struct BLINK_COMMON_EXPORT
    EnumTraits<blink::mojom::Opaque, blink::FencedFrame::Opaque> {
  static blink::mojom::Opaque ToMojom(blink::FencedFrame::Opaque input);
  static bool FromMojom(blink::mojom::Opaque input,
                        blink::FencedFrame::Opaque* out);
};

template <>
struct BLINK_COMMON_EXPORT
    EnumTraits<blink::mojom::ReportingDestination,
               blink::FencedFrame::ReportingDestination> {
  static blink::mojom::ReportingDestination ToMojom(
      blink::FencedFrame::ReportingDestination input);
  static bool FromMojom(blink::mojom::ReportingDestination input,
                        blink::FencedFrame::ReportingDestination* out);
};

template <>
struct BLINK_COMMON_EXPORT
    EnumTraits<blink::mojom::DeprecatedFencedFrameMode,
               blink::FencedFrame::DeprecatedFencedFrameMode> {
  static blink::mojom::DeprecatedFencedFrameMode ToMojom(
      blink::FencedFrame::DeprecatedFencedFrameMode input);
  static bool FromMojom(blink::mojom::DeprecatedFencedFrameMode input,
                        blink::FencedFrame::DeprecatedFencedFrameMode* out);
};

template <>
struct BLINK_COMMON_EXPORT StructTraits<blink::mojom::AdAuctionDataDataView,
                                        blink::FencedFrame::AdAuctionData> {
  static const url::Origin& interest_group_owner(
      const blink::FencedFrame::AdAuctionData& input);
  static const std::string& interest_group_name(
      const blink::FencedFrame::AdAuctionData& input);

  static bool Read(blink::mojom::AdAuctionDataDataView data,
                   blink::FencedFrame::AdAuctionData* out_data);
};

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::SharedStorageBudgetMetadataDataView,
                 blink::FencedFrame::SharedStorageBudgetMetadata> {
  static const net::SchemefulSite& site(
      const blink::FencedFrame::SharedStorageBudgetMetadata& input);
  static double budget_to_charge(
      const blink::FencedFrame::SharedStorageBudgetMetadata& input);
  static bool top_navigated(
      const blink::FencedFrame::SharedStorageBudgetMetadata& input);

  static bool Read(blink::mojom::SharedStorageBudgetMetadataDataView data,
                   blink::FencedFrame::SharedStorageBudgetMetadata* out_data);
};

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::ParentPermissionsInfoDataView,
                 blink::FencedFrame::ParentPermissionsInfo> {
  static const std::vector<network::ParsedPermissionsPolicyDeclaration>&
  parsed_permissions_policy(
      const blink::FencedFrame::ParentPermissionsInfo& input);
  static const url::Origin& origin(
      const blink::FencedFrame::ParentPermissionsInfo& input);

  static bool Read(blink::mojom::ParentPermissionsInfoDataView data,
                   blink::FencedFrame::ParentPermissionsInfo* out_data);
};

template <>
class BLINK_COMMON_EXPORT
    UnionTraits<blink::mojom::PotentiallyOpaqueURLDataView, Prop<GURL>> {
 public:
  static const GURL& transparent(const Prop<GURL>& mapped_url) {
    return *mapped_url.potentially_opaque_value;
  }
  static blink::FencedFrame::Opaque opaque(const Prop<GURL>&) {
    return blink::FencedFrame::Opaque::kOpaque;
  }

  static bool Read(blink::mojom::PotentiallyOpaqueURLDataView data,
                   Prop<GURL>* out);

  static blink::mojom::PotentiallyOpaqueURLDataView::Tag GetTag(
      const Prop<GURL>& mapped_url);
};

template <>
class BLINK_COMMON_EXPORT
    UnionTraits<blink::mojom::PotentiallyOpaqueSizeDataView, Prop<gfx::Size>> {
 public:
  static const gfx::Size& transparent(const Prop<gfx::Size>& size) {
    return *size.potentially_opaque_value;
  }
  static blink::FencedFrame::Opaque opaque(const Prop<gfx::Size>&) {
    return blink::FencedFrame::Opaque::kOpaque;
  }

  static bool Read(blink::mojom::PotentiallyOpaqueSizeDataView data,
                   Prop<gfx::Size>* out);

  static blink::mojom::PotentiallyOpaqueSizeDataView::Tag GetTag(
      const Prop<gfx::Size>& size);
};

template <>
class BLINK_COMMON_EXPORT
    UnionTraits<blink::mojom::PotentiallyOpaqueBoolDataView, Prop<bool>> {
 public:
  static const bool& transparent(const Prop<bool>& flag) {
    return *flag.potentially_opaque_value;
  }
  static blink::FencedFrame::Opaque opaque(const Prop<bool>&) {
    return blink::FencedFrame::Opaque::kOpaque;
  }

  static bool Read(blink::mojom::PotentiallyOpaqueBoolDataView data,
                   Prop<bool>* out);

  static blink::mojom::PotentiallyOpaqueBoolDataView::Tag GetTag(
      const Prop<bool>& flag);
};

template <>
class BLINK_COMMON_EXPORT
    UnionTraits<blink::mojom::PotentiallyOpaqueAdAuctionDataDataView,
                Prop<blink::FencedFrame::AdAuctionData>> {
 public:
  static const blink::FencedFrame::AdAuctionData& transparent(
      const Prop<blink::FencedFrame::AdAuctionData>& ad_auction_data) {
    return *ad_auction_data.potentially_opaque_value;
  }
  static blink::FencedFrame::Opaque opaque(
      const Prop<blink::FencedFrame::AdAuctionData>&) {
    return blink::FencedFrame::Opaque::kOpaque;
  }

  static bool Read(blink::mojom::PotentiallyOpaqueAdAuctionDataDataView data,
                   Prop<blink::FencedFrame::AdAuctionData>* out);

  static blink::mojom::PotentiallyOpaqueAdAuctionDataDataView::Tag GetTag(
      const Prop<blink::FencedFrame::AdAuctionData>& ad_auction_data);
};

template <>
class BLINK_COMMON_EXPORT UnionTraits<
    blink::mojom::PotentiallyOpaqueConfigVectorDataView,
    Prop<std::vector<blink::FencedFrame::RedactedFencedFrameConfig>>> {
 public:
  static const std::vector<blink::FencedFrame::RedactedFencedFrameConfig>&
  transparent(
      const Prop<std::vector<blink::FencedFrame::RedactedFencedFrameConfig>>&
          config_vector) {
    return *config_vector.potentially_opaque_value;
  }
  static blink::FencedFrame::Opaque opaque(
      const Prop<std::vector<blink::FencedFrame::RedactedFencedFrameConfig>>&) {
    return blink::FencedFrame::Opaque::kOpaque;
  }

  static bool Read(
      blink::mojom::PotentiallyOpaqueConfigVectorDataView data,
      Prop<std::vector<blink::FencedFrame::RedactedFencedFrameConfig>>* out);

  static blink::mojom::PotentiallyOpaqueConfigVectorDataView::Tag GetTag(
      const Prop<std::vector<blink::FencedFrame::RedactedFencedFrameConfig>>&
          config_vector);
};

template <>
class BLINK_COMMON_EXPORT UnionTraits<
    blink::mojom::PotentiallyOpaqueSharedStorageBudgetMetadataDataView,
    Prop<blink::FencedFrame::SharedStorageBudgetMetadata>> {
 public:
  static const blink::FencedFrame::SharedStorageBudgetMetadata& transparent(
      const Prop<blink::FencedFrame::SharedStorageBudgetMetadata>&
          shared_storage_budget_metadata) {
    return *shared_storage_budget_metadata.potentially_opaque_value;
  }
  static blink::FencedFrame::Opaque opaque(
      const Prop<blink::FencedFrame::SharedStorageBudgetMetadata>&) {
    return blink::FencedFrame::Opaque::kOpaque;
  }

  static bool Read(
      blink::mojom::PotentiallyOpaqueSharedStorageBudgetMetadataDataView data,
      Prop<blink::FencedFrame::SharedStorageBudgetMetadata>* out);

  static blink::mojom::PotentiallyOpaqueSharedStorageBudgetMetadataDataView::Tag
  GetTag(const Prop<blink::FencedFrame::SharedStorageBudgetMetadata>&
             shared_storage_budget_metadata);
};

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::FencedFrameConfigDataView,
                 blink::FencedFrame::RedactedFencedFrameConfig> {
  static const GURL& urn_uuid(
      const blink::FencedFrame::RedactedFencedFrameConfig& config) {
    // Whenever a redacted config is sent over an IPC, its `urn_` member is
    // expected to be non-nullopt.
    return config.urn_uuid_.value();
  }
  static const std::optional<Prop<GURL>>& mapped_url(
      const blink::FencedFrame::RedactedFencedFrameConfig& config) {
    return config.mapped_url_;
  }
  static const std::optional<Prop<gfx::Size>>& container_size(
      const blink::FencedFrame::RedactedFencedFrameConfig& config) {
    return config.container_size_;
  }
  static const std::optional<Prop<gfx::Size>>& content_size(
      const blink::FencedFrame::RedactedFencedFrameConfig& config) {
    return config.content_size_;
  }
  static const std::optional<Prop<bool>>& deprecated_should_freeze_initial_size(
      const blink::FencedFrame::RedactedFencedFrameConfig& config) {
    return config.deprecated_should_freeze_initial_size_;
  }
  static const std::optional<Prop<blink::FencedFrame::AdAuctionData>>&
  ad_auction_data(const blink::FencedFrame::RedactedFencedFrameConfig& config) {
    return config.ad_auction_data_;
  }
  static const std::optional<
      Prop<std::vector<blink::FencedFrame::RedactedFencedFrameConfig>>>&
  nested_configs(const blink::FencedFrame::RedactedFencedFrameConfig& config) {
    return config.nested_configs_;
  }
  static const std::optional<
      Prop<blink::FencedFrame::SharedStorageBudgetMetadata>>&
  shared_storage_budget_metadata(
      const blink::FencedFrame::RedactedFencedFrameConfig& config) {
    return config.shared_storage_budget_metadata_;
  }

  static const blink::FencedFrame::DeprecatedFencedFrameMode& mode(
      const blink::FencedFrame::RedactedFencedFrameConfig& config) {
    return config.mode_;
  }

  static const std::vector<network::mojom::PermissionsPolicyFeature>&
  effective_enabled_permissions(
      const blink::FencedFrame::RedactedFencedFrameConfig& config) {
    return config.effective_enabled_permissions_;
  }

  static const std::optional<blink::FencedFrame::ParentPermissionsInfo>&
  parent_permissions_info(
      const blink::FencedFrame::RedactedFencedFrameConfig& config) {
    return config.parent_permissions_info_;
  }

  static bool Read(blink::mojom::FencedFrameConfigDataView data,
                   blink::FencedFrame::RedactedFencedFrameConfig* out_config);
};

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::FencedFramePropertiesDataView,
                 blink::FencedFrame::RedactedFencedFrameProperties> {
  static const std::optional<Prop<GURL>>& mapped_url(
      const blink::FencedFrame::RedactedFencedFrameProperties& properties) {
    return properties.mapped_url_;
  }
  static const std::optional<Prop<gfx::Size>>& container_size(
      const blink::FencedFrame::RedactedFencedFrameProperties& properties) {
    return properties.container_size_;
  }
  static const std::optional<Prop<gfx::Size>>& content_size(
      const blink::FencedFrame::RedactedFencedFrameProperties& properties) {
    return properties.content_size_;
  }
  static const std::optional<Prop<bool>>& deprecated_should_freeze_initial_size(
      const blink::FencedFrame::RedactedFencedFrameProperties& properties) {
    return properties.deprecated_should_freeze_initial_size_;
  }
  static const std::optional<Prop<blink::FencedFrame::AdAuctionData>>&
  ad_auction_data(
      const blink::FencedFrame::RedactedFencedFrameProperties& properties) {
    return properties.ad_auction_data_;
  }
  static blink::mojom::PotentiallyOpaqueURNConfigVectorPtr
  nested_urn_config_pairs(
      const blink::FencedFrame::RedactedFencedFrameProperties& properties);
  static const std::optional<
      Prop<blink::FencedFrame::SharedStorageBudgetMetadata>>&
  shared_storage_budget_metadata(
      const blink::FencedFrame::RedactedFencedFrameProperties& properties) {
    return properties.shared_storage_budget_metadata_;
  }
  static const blink::FencedFrame::DeprecatedFencedFrameMode& mode(
      const blink::FencedFrame::RedactedFencedFrameProperties& properties) {
    return properties.mode_;
  }

  static const std::vector<network::mojom::PermissionsPolicyFeature>&
  effective_enabled_permissions(
      const blink::FencedFrame::RedactedFencedFrameProperties& properties) {
    return properties.effective_enabled_permissions_;
  }
  static bool can_disable_untrusted_network(
      const blink::FencedFrame::RedactedFencedFrameProperties& properties) {
    return properties.can_disable_untrusted_network_;
  }
  static bool is_cross_origin_content(
      const blink::FencedFrame::RedactedFencedFrameProperties& properties) {
    return properties.is_cross_origin_content_;
  }
  static bool allow_cross_origin_event_reporting(
      const blink::FencedFrame::RedactedFencedFrameProperties& properties) {
    return properties.allow_cross_origin_event_reporting_;
  }

  static const std::optional<blink::FencedFrame::ParentPermissionsInfo>&
  parent_permissions_info(
      const blink::FencedFrame::RedactedFencedFrameProperties& properties) {
    return properties.parent_permissions_info_;
  }

  static bool Read(
      blink::mojom::FencedFramePropertiesDataView data,
      blink::FencedFrame::RedactedFencedFrameProperties* out_properties);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_FENCED_FRAME_REDACTED_FENCED_FRAME_CONFIG_MOJOM_TRAITS_H_
