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

#ifndef __GTK_SHORTCUT_ACTION_H__
#define __GTK_SHORTCUT_ACTION_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtktypes.h>

G_BEGIN_DECLS

#define GTK_TYPE_SHORTCUT_ACTION (gtk_shortcut_action_get_type ())

/**
 * GtkShortcutFunc:
 * @widget: The widget passed to the activation
 * @args: (nullable): The arguments passed to the activation
 * @user_data: (nullable): The user data provided when activating the action
 *
 * Prototype for shortcuts based on user callbacks.
 *
 * Returns: %TRUE if the action was successful.
 */
typedef gboolean (* GtkShortcutFunc) (GtkWidget *widget,
                                      GVariant  *args,
                                      gpointer   user_data);

/**
 * GtkShortcutActionFlags:
 * @GTK_SHORTCUT_ACTION_EXCLUSIVE: The action is the only
 *   action that can be activated. If this flag is not set,
 *   a future activation may select a different action.
 *
 * List of flags that can be passed to action activation.
 *
 * More flags may be added in the future.
 **/
typedef enum {
  GTK_SHORTCUT_ACTION_EXCLUSIVE = 1 << 0
} GtkShortcutActionFlags;

GDK_AVAILABLE_IN_ALL
GDK_DECLARE_INTERNAL_TYPE (GtkShortcutAction, gtk_shortcut_action, GTK, SHORTCUT_ACTION, GObject)

GDK_AVAILABLE_IN_ALL
char *                  gtk_shortcut_action_to_string           (GtkShortcutAction      *self);
GDK_AVAILABLE_IN_ALL
GtkShortcutAction *     gtk_shortcut_action_parse_string        (const char *            string);

GDK_AVAILABLE_IN_ALL
void                    gtk_shortcut_action_print               (GtkShortcutAction      *self,
                                                                 GString                *string);
GDK_AVAILABLE_IN_ALL
gboolean                gtk_shortcut_action_activate            (GtkShortcutAction      *self,
                                                                 GtkShortcutActionFlags  flags,
                                                                 GtkWidget              *widget,
                                                                 GVariant               *args);

#define GTK_TYPE_NOTHING_ACTION (gtk_nothing_action_get_type())

/**
 * GtkNothingAction:
 *
 * A `GtkShortcutAction` that does nothing.
 */
GDK_AVAILABLE_IN_ALL
GDK_DECLARE_INTERNAL_TYPE (GtkNothingAction, gtk_nothing_action, GTK, NOTHING_ACTION, GtkShortcutAction)

GDK_AVAILABLE_IN_ALL
GtkShortcutAction *     gtk_nothing_action_get                  (void);

#define GTK_TYPE_CALLBACK_ACTION (gtk_callback_action_get_type())

/**
 * GtkCallbackAction:
 *
 * A `GtkShortcutAction` that invokes a callback.
 */
GDK_AVAILABLE_IN_ALL
GDK_DECLARE_INTERNAL_TYPE (GtkCallbackAction, gtk_callback_action, GTK, CALLBACK_ACTION, GtkShortcutAction)

GDK_AVAILABLE_IN_ALL
GtkShortcutAction *     gtk_callback_action_new                 (GtkShortcutFunc         callback,
                                                                 gpointer                data,
                                                                 GDestroyNotify          destroy);

#define GTK_TYPE_MNEMONIC_ACTION (gtk_mnemonic_action_get_type())

/**
 * GtkMnemonicAction:
 *
 * A `GtkShortcutAction` that calls gtk_widget_mnemonic_activate().
 */
GDK_AVAILABLE_IN_ALL
GDK_DECLARE_INTERNAL_TYPE (GtkMnemonicAction, gtk_mnemonic_action, GTK, MNEMONIC_ACTION, GtkShortcutAction)

GDK_AVAILABLE_IN_ALL
GtkShortcutAction *     gtk_mnemonic_action_get                 (void);

#define GTK_TYPE_ACTIVATE_ACTION (gtk_activate_action_get_type())

/**
 * GtkActivateAction:
 *
 * A `GtkShortcutAction` that calls gtk_widget_activate().
 */
GDK_AVAILABLE_IN_ALL
GDK_DECLARE_INTERNAL_TYPE (GtkActivateAction, gtk_activate_action, GTK, ACTIVATE_ACTION, GtkShortcutAction)

GDK_AVAILABLE_IN_ALL
GtkShortcutAction *     gtk_activate_action_get                 (void);

#define GTK_TYPE_SIGNAL_ACTION (gtk_signal_action_get_type())

/**
 * GtkSignalAction:
 *
 * A `GtkShortcut`Action that emits a signal.
 *
 * Signals that are used in this way are referred to as keybinding signals,
 * and they are expected to be defined with the %G_SIGNAL_ACTION flag.
 */
GDK_AVAILABLE_IN_ALL
GDK_DECLARE_INTERNAL_TYPE (GtkSignalAction, gtk_signal_action, GTK, SIGNAL_ACTION, GtkShortcutAction)

GDK_AVAILABLE_IN_ALL
GtkShortcutAction *     gtk_signal_action_new                   (const char      *signal_name);
GDK_AVAILABLE_IN_ALL
const char *            gtk_signal_action_get_signal_name       (GtkSignalAction *self);

#define GTK_TYPE_NAMED_ACTION (gtk_named_action_get_type())

/**
 * GtkNamedAction:
 *
 * A `GtkShortcutAction` that activates an action by name.
 */
GDK_AVAILABLE_IN_ALL
GDK_DECLARE_INTERNAL_TYPE (GtkNamedAction, gtk_named_action, GTK, NAMED_ACTION, GtkShortcutAction)

GDK_AVAILABLE_IN_ALL
GtkShortcutAction *     gtk_named_action_new                    (const char     *name);
GDK_AVAILABLE_IN_ALL
const char *            gtk_named_action_get_action_name        (GtkNamedAction *self);

G_END_DECLS

#endif /* __GTK_SHORTCUT_ACTION_H__ */
