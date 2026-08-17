/*
 * Copyright (C) 2009, 2012 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_CONTEXT_MENU_DATA_CONTEXT_MENU_DATA_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_CONTEXT_MENU_DATA_CONTEXT_MENU_DATA_H_

#include <optional>
#include <vector>

#include "services/network/public/mojom/referrer_policy.mojom-shared.h"
#include "third_party/blink/public/common/context_menu_data/menu_item_info.h"
#include "third_party/blink/public/common/input/web_menu_source_type.h"
#include "third_party/blink/public/common/navigation/impression.h"
#include "third_party/blink/public/mojom/context_menu/context_menu.mojom-shared.h"
#include "third_party/blink/public/mojom/forms/form_control_type.mojom-shared.h"
#include "ui/gfx/geometry/point.h"
#include "ui/gfx/geometry/rect.h"
#include "url/gurl.h"

namespace blink {

// This struct is passed to WebViewClient::ShowContextMenu.
struct ContextMenuData {
  // The type of media the context menu is being invoked on.
  // using MediaType = blink::mojom::ContextMenuDataMediaType;
  blink::mojom::ContextMenuDataMediaType media_type;

  // The x and y position of the mouse pointer (relative to the webview).
  gfx::Point mouse_position;

  // The absolute URL of the link that is in context.
  GURL link_url;

  // The absolute URL of the image/video/audio that is in context.
  GURL src_url;

  // Whether the image in context is a null.
  bool has_image_contents;

  // This is true if the context menu was invoked on an image, media or plugin
  // document. In these cases the resource for the hit-tested element might be
  // the main resource, not a subresource.
  bool is_image_media_plugin_document = false;

  // The encoding for the frame in context.
  std::string frame_encoding;

  enum MediaFlags {
    kMediaNone = 0,
    kMediaInError = 1,
    kMediaPaused = 1 << 1,
    kMediaMuted = 1 << 2,
    kMediaLoop = 1 << 3,
    kMediaCanSave = 1 << 4,
    kMediaHasAudio = 1 << 5,
    kMediaCanToggleControls = 1 << 6,
    kMediaControls = 1 << 7,
    kMediaCanPrint = 1 << 8,
    kMediaCanRotate = 1 << 9,
    kMediaCanPictureInPicture = 1 << 10,
    kMediaPictureInPicture = 1 << 11,
    kMediaCanLoop = 1 << 12,
    kMediaHasVideo = 1 << 13,
    kMediaHasReadableVideoFrame = 1 << 14,
    kMediaEncrypted = 1 << 15,
  };

  // Extra attributes describing media elements.
  int media_flags;

  // The text of the link that is in the context.
  std::string link_text;

  // If the node is a link, the impression declared by the link's conversion
  // measurement attributes.
  std::optional<Impression> impression;

  // The raw text of the selection in context.
  std::string selected_text;

  // Title attribute of the selection in context.
  std::string title_text;

  // Alt attribute of the selection in context.
  std::string alt_text;

  // Whether spell checking is enabled.
  bool is_spell_checking_enabled;

  // Suggested filename for saving file.
  std::string suggested_filename;

  // The editable (possibily) misspelled word.
  std::u16string misspelled_word;

  // If misspelledWord is not empty, holds suggestions from the dictionary.
  std::vector<std::u16string> dictionary_suggestions;

  // Whether context is editable.
  bool is_editable;

  enum CheckableMenuItemFlags {
    kCheckableMenuItemDisabled = 0x0,
    kCheckableMenuItemEnabled = 0x1,
    kCheckableMenuItemChecked = 0x2,
  };

  // Writing direction menu items - values are unions of
  // CheckableMenuItemFlags.
  int writing_direction_default;
  int writing_direction_left_to_right;
  int writing_direction_right_to_left;

  // Which edit operations are available in the context.
  int edit_flags;

  // The referrer policy applicable to this context.
  network::mojom::ReferrerPolicy referrer_policy;

  // Custom context menu items provided by the WebCore internals.
  std::vector<MenuItemInfo> custom_items;

  // Selection in viewport coordinates.
  gfx::Rect selection_rect;

  // TODO(https://crbug.com/781914): Remove this field after we done with Blink
  // side tracking.
  // Global index of start position for the current selection.
  // If the current element is editable, then it starts from the first
  // character within the element, otherwise, it starts from the beginning of
  // the current webpage.
  int selection_start_offset;

  WebMenuSourceType source_type;

  // True when the context contains text selected by a text fragment. See
  // TextFragmentAnchor.
  bool opened_from_highlight = false;

  // True when the context menu was opened from an element with the
  // `interesttarget` attribute.
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

  ContextMenuData()
      : media_type(blink::mojom::ContextMenuDataMediaType::kNone),
        has_image_contents(false),
        media_flags(kMediaNone),
        is_spell_checking_enabled(false),
        is_editable(false),
        writing_direction_default(kCheckableMenuItemDisabled),
        writing_direction_left_to_right(kCheckableMenuItemEnabled),
        writing_direction_right_to_left(kCheckableMenuItemEnabled),
        edit_flags(0) {}
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_CONTEXT_MENU_DATA_CONTEXT_MENU_DATA_H_
