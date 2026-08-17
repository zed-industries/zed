// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_CONTEXT_MENU_DATA_UNTRUSTWORTHY_CONTEXT_MENU_PARAMS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_CONTEXT_MENU_DATA_UNTRUSTWORTHY_CONTEXT_MENU_PARAMS_H_

#include <stdint.h>

#include <map>
#include <optional>
#include <string>
#include <vector>

#include "build/build_config.h"
#include "services/network/public/mojom/referrer_policy.mojom.h"
#include "third_party/blink/public/common/navigation/impression.h"
#include "third_party/blink/public/mojom/context_menu/context_menu.mojom-forward.h"
#include "third_party/blink/public/mojom/forms/form_control_type.mojom-shared.h"
#include "ui/base/mojom/menu_source_type.mojom-forward.h"
#include "ui/gfx/geometry/rect.h"
#include "url/gurl.h"

namespace blink {

// SECURITY NOTE: Data in this struct is untrustworthy, because it is sent in a
// Mojo from a renderer process.  The browser process should use
// ContextMenuParams, after validating UntrustworthyContextMenuParams in a Mojo
// handling routine.
struct BLINK_COMMON_EXPORT UntrustworthyContextMenuParams {
  UntrustworthyContextMenuParams();
  UntrustworthyContextMenuParams(const UntrustworthyContextMenuParams& other);
  UntrustworthyContextMenuParams& operator=(
      const UntrustworthyContextMenuParams& other);
  ~UntrustworthyContextMenuParams();

  // This is the type of Context Node that the context menu was invoked on.
  blink::mojom::ContextMenuDataMediaType media_type;

  // These values represent the coordinates of the mouse when the context menu
  // was invoked.  Coords are relative to the associated RenderView's origin.
  int x;
  int y;

  // This is the URL of the link that encloses the node the context menu was
  // invoked on.
  GURL link_url;

  // The text associated with the link. May be an empty string if the contents
  // of the link are an image.
  // Will be empty if |link_url| is empty.
  std::u16string link_text;

  // The impression declared by the link. May be std::nullopt even if
  // |link_url| is non-empty.
  std::optional<blink::Impression> impression;

  // The link URL to be used ONLY for "copy link address". We don't validate
  // this field in the frontend process.
  GURL unfiltered_link_url;

  // This is the source URL for the element that the context menu was
  // invoked on. Example of elements with source URLs are img, audio, and
  // video.
  GURL src_url;

  // This is true if the context menu was invoked on an image which has
  // non-empty contents.
  bool has_image_contents;

  // This is true if the context menu was invoked on an image, media or plugin
  // document. In these cases the resource for the hit-tested element might be
  // the main resource, not a subresource.
  bool is_image_media_plugin_document;

  // These are the parameters for the media element that the context menu
  // was invoked on.
  int media_flags;

  // This is the text of the selection that the context menu was invoked on.
  std::u16string selection_text;

  // This is the title text of the selection that the context menu was
  // invoked on.
  std::u16string title_text;

  // This is the alt text of the selection that the context menu was
  // invoked on.
  std::u16string alt_text;

  // This is the suggested filename to be used when saving file through "Save
  // Link As" option of context menu.
  std::u16string suggested_filename;

  // The misspelled word under the cursor, if any. Used to generate the
  // |dictionary_suggestions| list.
  std::u16string misspelled_word;

  // Suggested replacements for a misspelled word under the cursor.
  // This vector gets populated in the render process host
  // by intercepting ViewHostMsg_ContextMenu in ResourceMessageFilter
  // and populating dictionary_suggestions if the type is EDITABLE
  // and the misspelled_word is not empty.
  std::vector<std::u16string> dictionary_suggestions;

  // If editable, flag for whether spell check is enabled or not.
  bool spellcheck_enabled;

  // Whether context is editable.
  bool is_editable;

  // Writing direction menu items.
  int writing_direction_default;
  int writing_direction_left_to_right;
  int writing_direction_right_to_left;

  // These flags indicate to the browser whether the renderer believes it is
  // able to perform the corresponding action.
  int edit_flags;

  // The character encoding of the frame on which the menu is invoked.
  std::string frame_charset;

  // The referrer policy of the frame on which the menu is invoked.
  network::mojom::ReferrerPolicy referrer_policy;

  GURL link_followed;
  std::vector<blink::mojom::CustomContextMenuItemPtr> custom_items;

  ui::mojom::MenuSourceType source_type;

  // For the outermost main frame's widget, this will be the selection rect in
  // viewport space. For a local root, this is in the coordinates of the local
  // frame root.
  gfx::Rect selection_rect;

  // Start position of the selection text.
  int selection_start_offset;

  // The context menu was opened by right clicking on an existing
  // highlight/fragment.
  bool opened_from_highlight = false;

  // The context menu was opened from an element with the `interesttarget`
  // attribute.
  bool opened_from_interest_target = false;

  // The type of the form control element on which the context menu is invoked,
  // if any.
  std::optional<mojom::FormControlType> form_control_type;

  // Indicates whether the context menu is invoked on a non-form,
  // non-form-control element that is contenteditable. Thus, it is mutually
  // exclusive with `form_control_type`.
  bool is_content_editable_for_autofill = false;

  // Identifies the element the context menu was invoked on if either
  // `form_control_type` is engaged or `is_content_editable_for_autofill` is
  // true.
  // See `autofill::FieldRendererId` for the semantics of renderer IDs.
  uint64_t field_renderer_id = 0;

  // Identifies form to which the field identified by `field_renderer_id` is
  // associated.
  // See `autofill::FormRendererId` for the semantics of renderer IDs.
  uint64_t form_renderer_id = 0;

 private:
  void Assign(const UntrustworthyContextMenuParams& other);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_CONTEXT_MENU_DATA_UNTRUSTWORTHY_CONTEXT_MENU_PARAMS_H_
