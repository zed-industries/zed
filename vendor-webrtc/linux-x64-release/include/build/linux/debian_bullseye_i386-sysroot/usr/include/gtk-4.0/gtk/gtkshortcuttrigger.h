/*
 * Copyright © 2018 Benjamin Otte
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 *
 * Authors: Benjamin Otte <otte@gnome.org>
 */

#ifndef __GTK_SHORTCUT_TRIGGER_H__
#define __GTK_SHORTCUT_TRIGGER_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtktypes.h>

G_BEGIN_DECLS

#define GTK_TYPE_SHORTCUT_TRIGGER (gtk_shortcut_trigger_get_type ())

GDK_AVAILABLE_IN_ALL
GDK_DECLARE_INTERNAL_TYPE (GtkShortcutTrigger, gtk_shortcut_trigger, GTK, SHORTCUT_TRIGGER, GObject)

GDK_AVAILABLE_IN_ALL
GtkShortcutTrigger *    gtk_shortcut_trigger_parse_string       (const char         *string);

GDK_AVAILABLE_IN_ALL
char *                  gtk_shortcut_trigger_to_string          (GtkShortcutTrigger *self);
GDK_AVAILABLE_IN_ALL
void                    gtk_shortcut_trigger_print              (GtkShortcutTrigger *self,
                                                                 GString            *string);
GDK_AVAILABLE_IN_ALL
char *                  gtk_shortcut_trigger_to_label           (GtkShortcutTrigger *self,
                                                                 GdkDisplay         *display);
GDK_AVAILABLE_IN_ALL
gboolean                gtk_shortcut_trigger_print_label        (GtkShortcutTrigger *self,
                                                                 GdkDisplay         *display,
                                                                 GString            *string);

GDK_AVAILABLE_IN_ALL
guint                   gtk_shortcut_trigger_hash               (gconstpointer       trigger);
GDK_AVAILABLE_IN_ALL
gboolean                gtk_shortcut_trigger_equal              (gconstpointer       trigger1,
                                                                 gconstpointer       trigger2);
GDK_AVAILABLE_IN_ALL
int                     gtk_shortcut_trigger_compare            (gconstpointer       trigger1,
                                                                 gconstpointer       trigger2);

GDK_AVAILABLE_IN_ALL
GdkKeyMatch             gtk_shortcut_trigger_trigger            (GtkShortcutTrigger *self,
                                                                 GdkEvent           *event,
                                                                 gboolean            enable_mnemonics);


#define GTK_TYPE_NEVER_TRIGGER (gtk_never_trigger_get_type())

/**
 * GtkNeverTrigger:
 *
 * A `GtkShortcutTrigger` that never triggers.
 */
GDK_AVAILABLE_IN_ALL
GDK_DECLARE_INTERNAL_TYPE (GtkNeverTrigger, gtk_never_trigger, GTK, NEVER_TRIGGER, GtkShortcutTrigger)

GDK_AVAILABLE_IN_ALL
GtkShortcutTrigger *    gtk_never_trigger_get                   (void);

#define GTK_TYPE_KEYVAL_TRIGGER (gtk_keyval_trigger_get_type())

/**
 * GtkKeyvalTrigger:
 *
 * A `GtkShortcutTrigger` that triggers when a specific keyval and modifiers are pressed.
 */

GDK_AVAILABLE_IN_ALL
GDK_DECLARE_INTERNAL_TYPE (GtkKeyvalTrigger, gtk_keyval_trigger, GTK, KEYVAL_TRIGGER, GtkShortcutTrigger)

GDK_AVAILABLE_IN_ALL
GtkShortcutTrigger *    gtk_keyval_trigger_new                  (guint             keyval,
                                                                 GdkModifierType   modifiers);
GDK_AVAILABLE_IN_ALL
GdkModifierType         gtk_keyval_trigger_get_modifiers        (GtkKeyvalTrigger *self);
GDK_AVAILABLE_IN_ALL
guint                   gtk_keyval_trigger_get_keyval           (GtkKeyvalTrigger *self);

#define GTK_TYPE_MNEMONIC_TRIGGER (gtk_mnemonic_trigger_get_type())

/**
 * GtkMnemonicTrigger:
 *
 * A `GtkShortcutTrigger` that triggers when a specific mnemonic is pressed.
 *
 * Mnemonics require a *mnemonic modifier* (typically <kbd>Alt</kbd>) to be
 * pressed together with the mnemonic key.
 */
GDK_AVAILABLE_IN_ALL
GDK_DECLARE_INTERNAL_TYPE (GtkMnemonicTrigger, gtk_mnemonic_trigger, GTK, MNEMONIC_TRIGGER, GtkShortcutTrigger)

GDK_AVAILABLE_IN_ALL
GtkShortcutTrigger *    gtk_mnemonic_trigger_new                (guint               keyval);
GDK_AVAILABLE_IN_ALL
guint                   gtk_mnemonic_trigger_get_keyval         (GtkMnemonicTrigger *self);

#define GTK_TYPE_ALTERNATIVE_TRIGGER (gtk_alternative_trigger_get_type())

/**
 * GtkAlternativeTrigger:
 *
 * A `GtkShortcutTrigger` that combines two triggers.
 *
 * The `GtkAlternativeTrigger` triggers when either of two trigger.
 *
 * This can be cascaded to combine more than two triggers.
 */

GDK_AVAILABLE_IN_ALL
GDK_DECLARE_INTERNAL_TYPE (GtkAlternativeTrigger, gtk_alternative_trigger, GTK, ALTERNATIVE_TRIGGER, GtkShortcutTrigger)

GDK_AVAILABLE_IN_ALL
GtkShortcutTrigger *    gtk_alternative_trigger_new             (GtkShortcutTrigger    *first,
                                                                 GtkShortcutTrigger    *second);
GDK_AVAILABLE_IN_ALL
GtkShortcutTrigger *    gtk_alternative_trigger_get_first       (GtkAlternativeTrigger *self);
GDK_AVAILABLE_IN_ALL
GtkShortcutTrigger *    gtk_alternative_trigger_get_second      (GtkAlternativeTrigger *self);

G_END_DECLS

#endif /* __GTK_SHORTCUT_TRIGGER_H__ */
