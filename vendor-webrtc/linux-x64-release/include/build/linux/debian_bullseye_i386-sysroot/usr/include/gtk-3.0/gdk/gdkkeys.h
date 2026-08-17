/* GDK - The GIMP Drawing Kit
 * Copyright (C) 2000 Red Hat, Inc.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.	 See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

/*
 * Modified by the GTK+ Team and others 1997-2000.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GDK_KEYS_H__
#define __GDK_KEYS_H__

#if !defined (__GDK_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdkversionmacros.h>
#include <gdk/gdktypes.h>

G_BEGIN_DECLS


typedef struct _GdkKeymapKey GdkKeymapKey;

/**
 * GdkKeymapKey:
 * @keycode: the hardware keycode. This is an identifying number for a
 *   physical key.
 * @group: indicates movement in a horizontal direction. Usually groups are used
 *   for two different languages. In group 0, a key might have two English
 *   characters, and in group 1 it might have two Hebrew characters. The Hebrew
 *   characters will be printed on the key next to the English characters.
 * @level: indicates which symbol on the key will be used, in a vertical direction.
 *   So on a standard US keyboard, the key with the number “1” on it also has the
 *   exclamation point ("!") character on it. The level indicates whether to use
 *   the “1” or the “!” symbol. The letter keys are considered to have a lowercase
 *   letter at level 0, and an uppercase letter at level 1, though only the
 *   uppercase letter is printed.
 *
 * A #GdkKeymapKey is a hardware key that can be mapped to a keyval.
 */
struct _GdkKeymapKey
{
  guint keycode;
  gint  group;
  gint  level;
};


#define GDK_TYPE_KEYMAP              (gdk_keymap_get_type ())
#define GDK_KEYMAP(object)           (G_TYPE_CHECK_INSTANCE_CAST ((object), GDK_TYPE_KEYMAP, GdkKeymap))
#define GDK_IS_KEYMAP(object)        (G_TYPE_CHECK_INSTANCE_TYPE ((object), GDK_TYPE_KEYMAP))

/**
 * GdkKeymap:
 *
 * A #GdkKeymap defines the translation from keyboard state
 * (including a hardware key, a modifier mask, and active keyboard group)
 * to a keyval. This translation has two phases. The first phase is
 * to determine the effective keyboard group and level for the keyboard
 * state; the second phase is to look up the keycode/group/level triplet
 * in the keymap and see what keyval it corresponds to.
 */

GDK_AVAILABLE_IN_ALL
GType gdk_keymap_get_type (void) G_GNUC_CONST;

GDK_DEPRECATED_IN_3_22_FOR(gdk_keymap_get_for_display)
GdkKeymap* gdk_keymap_get_default     (void);
GDK_AVAILABLE_IN_ALL
GdkKeymap* gdk_keymap_get_for_display (GdkDisplay *display);

GDK_AVAILABLE_IN_ALL
guint          gdk_keymap_lookup_key               (GdkKeymap           *keymap,
						    const GdkKeymapKey  *key);
GDK_AVAILABLE_IN_ALL
gboolean       gdk_keymap_translate_keyboard_state (GdkKeymap           *keymap,
						    guint                hardware_keycode,
						    GdkModifierType      state,
						    gint                 group,
						    guint               *keyval,
						    gint                *effective_group,
						    gint                *level,
						    GdkModifierType     *consumed_modifiers);
GDK_AVAILABLE_IN_ALL
gboolean       gdk_keymap_get_entries_for_keyval   (GdkKeymap           *keymap,
						    guint                keyval,
						    GdkKeymapKey       **keys,
						    gint                *n_keys);
GDK_AVAILABLE_IN_ALL
gboolean       gdk_keymap_get_entries_for_keycode  (GdkKeymap           *keymap,
						    guint                hardware_keycode,
						    GdkKeymapKey       **keys,
						    guint              **keyvals,
						    gint                *n_entries);

GDK_AVAILABLE_IN_ALL
PangoDirection gdk_keymap_get_direction            (GdkKeymap           *keymap);
GDK_AVAILABLE_IN_ALL
gboolean       gdk_keymap_have_bidi_layouts        (GdkKeymap           *keymap);
GDK_AVAILABLE_IN_ALL
gboolean       gdk_keymap_get_caps_lock_state      (GdkKeymap           *keymap);
GDK_AVAILABLE_IN_ALL
gboolean       gdk_keymap_get_num_lock_state       (GdkKeymap           *keymap);
GDK_AVAILABLE_IN_3_18
gboolean       gdk_keymap_get_scroll_lock_state    (GdkKeymap           *keymap);
GDK_AVAILABLE_IN_3_4
guint          gdk_keymap_get_modifier_state       (GdkKeymap           *keymap);
GDK_AVAILABLE_IN_ALL
void           gdk_keymap_add_virtual_modifiers    (GdkKeymap           *keymap,
                                                    GdkModifierType     *state);
GDK_AVAILABLE_IN_ALL
gboolean       gdk_keymap_map_virtual_modifiers    (GdkKeymap           *keymap,
                                                    GdkModifierType     *state);
GDK_AVAILABLE_IN_3_4
GdkModifierType gdk_keymap_get_modifier_mask       (GdkKeymap           *keymap,
                                                    GdkModifierIntent    intent);


/* Key values
 */
GDK_AVAILABLE_IN_ALL
gchar*   gdk_keyval_name         (guint        keyval) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
guint    gdk_keyval_from_name    (const gchar *keyval_name);
GDK_AVAILABLE_IN_ALL
void     gdk_keyval_convert_case (guint        symbol,
				  guint       *lower,
				  guint       *upper);
GDK_AVAILABLE_IN_ALL
guint    gdk_keyval_to_upper     (guint        keyval) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
guint    gdk_keyval_to_lower     (guint        keyval) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
gboolean gdk_keyval_is_upper     (guint        keyval) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
gboolean gdk_keyval_is_lower     (guint        keyval) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
guint32  gdk_keyval_to_unicode   (guint        keyval) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
guint    gdk_unicode_to_keyval   (guint32      wc) G_GNUC_CONST;


G_END_DECLS

#endif /* __GDK_KEYS_H__ */
