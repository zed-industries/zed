/* Pango
 * pango-bidi-type.h: Bidirectional Character Types
 *
 * Copyright (C) 2008 Jürg Billeter <j@bitron.ch>
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.	 See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

#ifndef __PANGO_BIDI_TYPE_H__
#define __PANGO_BIDI_TYPE_H__

#include <glib.h>

#include <pango/pango-version-macros.h>
#include <pango/pango-direction.h>

G_BEGIN_DECLS

#ifndef PANGO_DISABLE_DEPRECATED
/**
 * PangoBidiType:
 * @PANGO_BIDI_TYPE_L: Left-to-Right
 * @PANGO_BIDI_TYPE_LRE: Left-to-Right Embedding
 * @PANGO_BIDI_TYPE_LRO: Left-to-Right Override
 * @PANGO_BIDI_TYPE_R: Right-to-Left
 * @PANGO_BIDI_TYPE_AL: Right-to-Left Arabic
 * @PANGO_BIDI_TYPE_RLE: Right-to-Left Embedding
 * @PANGO_BIDI_TYPE_RLO: Right-to-Left Override
 * @PANGO_BIDI_TYPE_PDF: Pop Directional Format
 * @PANGO_BIDI_TYPE_EN: European Number
 * @PANGO_BIDI_TYPE_ES: European Number Separator
 * @PANGO_BIDI_TYPE_ET: European Number Terminator
 * @PANGO_BIDI_TYPE_AN: Arabic Number
 * @PANGO_BIDI_TYPE_CS: Common Number Separator
 * @PANGO_BIDI_TYPE_NSM: Nonspacing Mark
 * @PANGO_BIDI_TYPE_BN: Boundary Neutral
 * @PANGO_BIDI_TYPE_B: Paragraph Separator
 * @PANGO_BIDI_TYPE_S: Segment Separator
 * @PANGO_BIDI_TYPE_WS: Whitespace
 * @PANGO_BIDI_TYPE_ON: Other Neutrals
 *
 * The #PangoBidiType type represents the bidirectional character
 * type of a Unicode character as specified by the
 * <ulink url="http://www.unicode.org/reports/tr9/">Unicode bidirectional algorithm</ulink>.
 *
 * Since: 1.22
 * Deprecated: 1.44: Use fribidi for this information
 **/
typedef enum {
  /* Strong types */
  PANGO_BIDI_TYPE_L,
  PANGO_BIDI_TYPE_LRE,
  PANGO_BIDI_TYPE_LRO,
  PANGO_BIDI_TYPE_R,
  PANGO_BIDI_TYPE_AL,
  PANGO_BIDI_TYPE_RLE,
  PANGO_BIDI_TYPE_RLO,

  /* Weak types */
  PANGO_BIDI_TYPE_PDF,
  PANGO_BIDI_TYPE_EN,
  PANGO_BIDI_TYPE_ES,
  PANGO_BIDI_TYPE_ET,
  PANGO_BIDI_TYPE_AN,
  PANGO_BIDI_TYPE_CS,
  PANGO_BIDI_TYPE_NSM,
  PANGO_BIDI_TYPE_BN,

  /* Neutral types */
  PANGO_BIDI_TYPE_B,
  PANGO_BIDI_TYPE_S,
  PANGO_BIDI_TYPE_WS,
  PANGO_BIDI_TYPE_ON
} PangoBidiType;

PANGO_DEPRECATED_IN_1_44
PangoBidiType pango_bidi_type_for_unichar (gunichar ch) G_GNUC_CONST;

PANGO_DEPRECATED_IN_1_44
PangoDirection pango_unichar_direction      (gunichar     ch) G_GNUC_CONST;
PANGO_DEPRECATED_IN_1_44
PangoDirection pango_find_base_dir          (const gchar *text,
					     gint         length);

PANGO_DEPRECATED_FOR(g_unichar_get_mirror_char)
gboolean       pango_get_mirror_char        (gunichar     ch,
					     gunichar    *mirrored_ch);
#endif

G_END_DECLS

#endif /* __PANGO_BIDI_TYPE_H__ */
