// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_CONTEXT_MENU_DATA_CONTEXT_MENU_MOJOM_TRAITS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_CONTEXT_MENU_DATA_CONTEXT_MENU_MOJOM_TRAITS_H_

#include <optional>

#include "build/build_config.h"
#include "mojo/public/cpp/bindings/struct_traits.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/context_menu_data/untrustworthy_context_menu_params.h"
#include "third_party/blink/public/mojom/context_menu/context_menu.mojom.h"
#include "third_party/blink/public/mojom/forms/form_control_type.mojom-shared.h"
#include "ui/base/mojom/menu_source_type.mojom-forward.h"
#include "url/mojom/url_gurl_mojom_traits.h"

namespace mojo {

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::FormRendererIdDataView, uint64_t> {
  static uint64_t id(uint64_t r) { return r; }

  static bool Read(blink::mojom::FormRendererIdDataView data, uint64_t* out);
};

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::FieldRendererIdDataView, uint64_t> {
  static uint64_t id(uint64_t r) { return r; }

  static bool Read(blink::mojom::FieldRendererIdDataView data, uint64_t* out);
};

template <>
struct BLINK_COMMON_EXPORT
    StructTraits<blink::mojom::UntrustworthyContextMenuParamsDataView,
                 blink::UntrustworthyContextMenuParams> {
  static blink::mojom::ContextMenuDataMediaType media_type(
      const blink::UntrustworthyContextMenuParams& r) {
    return r.media_type;
  }

  static int x(const blink::UntrustworthyContextMenuParams& r) { return r.x; }

  static int y(const blink::UntrustworthyContextMenuParams& r) { return r.y; }

  static const GURL& link_url(const blink::UntrustworthyContextMenuParams& r) {
    return r.link_url;
  }

  static const std::u16string& link_text(
      const blink::UntrustworthyContextMenuParams& r) {
    return r.link_text;
  }

  static const std::optional<blink::Impression>& impression(
      const blink::UntrustworthyContextMenuParams& r) {
    return r.impression;
  }

  static const GURL& unfiltered_link_url(
      const blink::UntrustworthyContextMenuParams& r) {
    return r.unfiltered_link_url;
  }

  static const GURL& src_url(const blink::UntrustworthyContextMenuParams& r) {
    return r.src_url;
  }

  static bool has_image_contents(
      const blink::UntrustworthyContextMenuParams& r) {
    return r.has_image_contents;
  }

  static bool is_image_media_plugin_document(
      const blink::UntrustworthyContextMenuParams& r) {
    return r.is_image_media_plugin_document;
  }

  static int media_flags(const blink::UntrustworthyContextMenuParams& r) {
    return r.media_flags;
  }

  static const std::u16string& selection_text(
      const blink::UntrustworthyContextMenuParams& r) {
    return r.selection_text;
  }

  static const std::u16string& title_text(
      const blink::UntrustworthyContextMenuParams& r) {
    return r.title_text;
  }

  static const std::u16string& alt_text(
      const blink::UntrustworthyContextMenuParams& r) {
    return r.alt_text;
  }

  static const std::u16string& suggested_filename(
      const blink::UntrustworthyContextMenuParams& r) {
    return r.suggested_filename;
  }

  static const std::u16string& misspelled_word(
      const blink::UntrustworthyContextMenuParams& r) {
    return r.misspelled_word;
  }

  static const std::vector<std::u16string>& dictionary_suggestions(
      const blink::UntrustworthyContextMenuParams& r) {
    return r.dictionary_suggestions;
  }

  static bool spellcheck_enabled(
      const blink::UntrustworthyContextMenuParams& r) {
    return r.spellcheck_enabled;
  }

  static bool is_editable(const blink::UntrustworthyContextMenuParams& r) {
    return r.is_editable;
  }

  static int writing_direction_default(
      const blink::UntrustworthyContextMenuParams& r) {
    return r.writing_direction_default;
  }

  static int writing_direction_left_to_right(
      const blink::UntrustworthyContextMenuParams& r) {
    return r.writing_direction_left_to_right;
  }

  static int writing_direction_right_to_left(
      const blink::UntrustworthyContextMenuParams& r) {
    return r.writing_direction_right_to_left;
  }

  static int edit_flags(const blink::UntrustworthyContextMenuParams& r) {
    return r.edit_flags;
  }

  static const std::string& frame_charset(
      const blink::UntrustworthyContextMenuParams& r) {
    return r.frame_charset;
  }

  static network::mojom::ReferrerPolicy referrer_policy(
      const blink::UntrustworthyContextMenuParams& r) {
    return r.referrer_policy;
  }

  static const GURL& link_followed(
      const blink::UntrustworthyContextMenuParams& r) {
    return r.link_followed;
  }

  static const std::vector<blink::mojom::CustomContextMenuItemPtr>&
  custom_items(const blink::UntrustworthyContextMenuParams& r) {
    return r.custom_items;
  }

  static ui::mojom::MenuSourceType source_type(
      const blink::UntrustworthyContextMenuParams& r) {
    return r.source_type;
  }

  static const gfx::Rect& selection_rect(
      const blink::UntrustworthyContextMenuParams& r) {
    return r.selection_rect;
  }

  static int selection_start_offset(
      const blink::UntrustworthyContextMenuParams& r) {
    return r.selection_start_offset;
  }

  static bool opened_from_highlight(
      const blink::UntrustworthyContextMenuParams& r) {
    return r.opened_from_highlight;
  }

  static bool opened_from_interest_target(
      const blink::UntrustworthyContextMenuParams& r) {
    return r.opened_from_interest_target;
  }

  static std::optional<blink::mojom::FormControlType> form_control_type(
      const blink::UntrustworthyContextMenuParams& r) {
    return r.form_control_type;
  }

  static bool is_content_editable_for_autofill(
      const blink::UntrustworthyContextMenuParams& r) {
    return r.is_content_editable_for_autofill;
  }

  static uint64_t field_renderer_id(
      const blink::UntrustworthyContextMenuParams& r) {
    return r.field_renderer_id;
  }

  static uint64_t form_renderer_id(
      const blink::UntrustworthyContextMenuParams& r) {
    return r.form_renderer_id;
  }

  static bool Read(blink::mojom::UntrustworthyContextMenuParamsDataView r,
                   blink::UntrustworthyContextMenuParams* out);
};

}  // namespace mojo

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_CONTEXT_MENU_DATA_CONTEXT_MENU_MOJOM_TRAITS_H_
