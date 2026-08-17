/*
 * GTK - The GIMP Toolkit
 * Copyright (C) 1998, 1999 Red Hat, Inc.
 * All rights reserved.
 *
 * This Library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public License as
 * published by the Free Software Foundation; either version 2 of the
 * License, or (at your option) any later version.
 *
 * This Library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

/*
 * Author: James Henstridge <james@daa.com.au>
 *
 * Modified by the GTK+ Team and others 2003.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GTK_ACTION_GROUP_H__
#define __GTK_ACTION_GROUP_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/deprecated/gtkaction.h>
#include <gtk/deprecated/gtkstock.h>

G_BEGIN_DECLS

#define GTK_TYPE_ACTION_GROUP              (gtk_action_group_get_type ())
#define GTK_ACTION_GROUP(obj)              (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_ACTION_GROUP, GtkActionGroup))
#define GTK_ACTION_GROUP_CLASS(vtable)     (G_TYPE_CHECK_CLASS_CAST ((vtable), GTK_TYPE_ACTION_GROUP, GtkActionGroupClass))
#define GTK_IS_ACTION_GROUP(obj)           (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_ACTION_GROUP))
#define GTK_IS_ACTION_GROUP_CLASS(vtable)  (G_TYPE_CHECK_CLASS_TYPE ((vtable), GTK_TYPE_ACTION_GROUP))
#define GTK_ACTION_GROUP_GET_CLASS(inst)   (G_TYPE_INSTANCE_GET_CLASS ((inst), GTK_TYPE_ACTION_GROUP, GtkActionGroupClass))

typedef struct _GtkActionGroup        GtkActionGroup;
typedef struct _GtkActionGroupPrivate GtkActionGroupPrivate;
typedef struct _GtkActionGroupClass   GtkActionGroupClass;
typedef struct _GtkActionEntry        GtkActionEntry;
typedef struct _GtkToggleActionEntry  GtkToggleActionEntry;
typedef struct _GtkRadioActionEntry   GtkRadioActionEntry;

struct _GtkActionGroup
{
  GObject parent;

  /*< private >*/
  GtkActionGroupPrivate *priv;
};

/**
 * GtkActionGroupClass:
 * @parent_class: The parent class.
 * @get_action: Looks up an action in the action group by name.
 */
struct _GtkActionGroupClass
{
  GObjectClass parent_class;

  GtkAction *(* get_action) (GtkActionGroup *action_group,
                             const gchar    *action_name);

  /*< private >*/

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

/**
 * GtkActionEntry:
 * @name: The name of the action.
 * @stock_id: The stock id for the action, or the name of an icon from the
 *  icon theme.
 * @label: The label for the action. This field should typically be marked
 *  for translation, see gtk_action_group_set_translation_domain(). If
 *  @label is %NULL, the label of the stock item with id @stock_id is used.
 * @accelerator: The accelerator for the action, in the format understood by
 *  gtk_accelerator_parse().
 * @tooltip: The tooltip for the action. This field should typically be
 *  marked for translation, see gtk_action_group_set_translation_domain().
 * @callback: The function to call when the action is activated.
 *
 * #GtkActionEntry structs are used with gtk_action_group_add_actions() to
 * construct actions.
 *
 * Deprecated: 3.10
 */
struct _GtkActionEntry 
{
  const gchar     *name;
  const gchar     *stock_id;
  const gchar     *label;
  const gchar     *accelerator;
  const gchar     *tooltip;
  GCallback  callback;
};

/**
 * GtkToggleActionEntry:
 * @name: The name of the action.
 * @stock_id: The stock id for the action, or the name of an icon from the
 *  icon theme.
 * @label: The label for the action. This field should typically be marked
 *  for translation, see gtk_action_group_set_translation_domain().
 * @accelerator: The accelerator for the action, in the format understood by
 *  gtk_accelerator_parse().
 * @tooltip: The tooltip for the action. This field should typically be
 *  marked for translation, see gtk_action_group_set_translation_domain().
 * @callback: The function to call when the action is activated.
 * @is_active: The initial state of the toggle action.
 *
 * #GtkToggleActionEntry structs are used with
 * gtk_action_group_add_toggle_actions() to construct toggle actions.
 *
 * Deprecated: 3.10
 */
struct _GtkToggleActionEntry 
{
  const gchar     *name;
  const gchar     *stock_id;
  const gchar     *label;
  const gchar     *accelerator;
  const gchar     *tooltip;
  GCallback  callback;
  gboolean   is_active;
};

/**
 * GtkRadioActionEntry:
 * @name: The name of the action.
 * @stock_id: The stock id for the action, or the name of an icon from the
 *  icon theme.
 * @label: The label for the action. This field should typically be marked
 *  for translation, see gtk_action_group_set_translation_domain().
 * @accelerator: The accelerator for the action, in the format understood by
 *  gtk_accelerator_parse().
 * @tooltip: The tooltip for the action. This field should typically be
 *  marked for translation, see gtk_action_group_set_translation_domain().
 * @value: The value to set on the radio action. See
 *  gtk_radio_action_get_current_value().
 *
 * #GtkRadioActionEntry structs are used with
 * gtk_action_group_add_radio_actions() to construct groups of radio actions.
 *
 * Deprecated: 3.10
 */
struct _GtkRadioActionEntry 
{
  const gchar *name;
  const gchar *stock_id;
  const gchar *label;
  const gchar *accelerator;
  const gchar *tooltip;
  gint   value; 
};

GDK_DEPRECATED_IN_3_10
GType           gtk_action_group_get_type                (void) G_GNUC_CONST;
GDK_DEPRECATED_IN_3_10
GtkActionGroup *gtk_action_group_new                     (const gchar                *name);
GDK_DEPRECATED_IN_3_10
const gchar    *gtk_action_group_get_name                (GtkActionGroup             *action_group);
GDK_DEPRECATED_IN_3_10
gboolean        gtk_action_group_get_sensitive           (GtkActionGroup             *action_group);
GDK_DEPRECATED_IN_3_10
void            gtk_action_group_set_sensitive           (GtkActionGroup             *action_group,
							  gboolean                    sensitive);
GDK_DEPRECATED_IN_3_10
gboolean        gtk_action_group_get_visible             (GtkActionGroup             *action_group);
GDK_DEPRECATED_IN_3_10
void            gtk_action_group_set_visible             (GtkActionGroup             *action_group,
							  gboolean                    visible);
GDK_DEPRECATED_IN_3_10
GtkAccelGroup  *gtk_action_group_get_accel_group         (GtkActionGroup             *action_group);
GDK_DEPRECATED_IN_3_10
void            gtk_action_group_set_accel_group         (GtkActionGroup             *action_group,
                                                          GtkAccelGroup              *accel_group);

GDK_DEPRECATED_IN_3_10
GtkAction      *gtk_action_group_get_action              (GtkActionGroup             *action_group,
							  const gchar                *action_name);
GDK_DEPRECATED_IN_3_10
GList          *gtk_action_group_list_actions            (GtkActionGroup             *action_group);
GDK_DEPRECATED_IN_3_10
void            gtk_action_group_add_action              (GtkActionGroup             *action_group,
							  GtkAction                  *action);
GDK_DEPRECATED_IN_3_10
void            gtk_action_group_add_action_with_accel   (GtkActionGroup             *action_group,
							  GtkAction                  *action,
							  const gchar                *accelerator);
GDK_DEPRECATED_IN_3_10
void            gtk_action_group_remove_action           (GtkActionGroup             *action_group,
							  GtkAction                  *action);
GDK_DEPRECATED_IN_3_10
void            gtk_action_group_add_actions             (GtkActionGroup             *action_group,
							  const GtkActionEntry       *entries,
							  guint                       n_entries,
							  gpointer                    user_data);
GDK_DEPRECATED_IN_3_10
void            gtk_action_group_add_toggle_actions      (GtkActionGroup             *action_group,
							  const GtkToggleActionEntry *entries,
							  guint                       n_entries,
							  gpointer                    user_data);
GDK_DEPRECATED_IN_3_10
void            gtk_action_group_add_radio_actions       (GtkActionGroup             *action_group,
							  const GtkRadioActionEntry  *entries,
							  guint                       n_entries,
							  gint                        value,
							  GCallback                   on_change,
							  gpointer                    user_data);
GDK_DEPRECATED_IN_3_10
void            gtk_action_group_add_actions_full        (GtkActionGroup             *action_group,
							  const GtkActionEntry       *entries,
							  guint                       n_entries,
							  gpointer                    user_data,
							  GDestroyNotify              destroy);
GDK_DEPRECATED_IN_3_10
void            gtk_action_group_add_toggle_actions_full (GtkActionGroup             *action_group,
							  const GtkToggleActionEntry *entries,
							  guint                       n_entries,
							  gpointer                    user_data,
							  GDestroyNotify              destroy);
GDK_DEPRECATED_IN_3_10
void            gtk_action_group_add_radio_actions_full  (GtkActionGroup             *action_group,
							  const GtkRadioActionEntry  *entries,
							  guint                       n_entries,
							  gint                        value,
							  GCallback                   on_change,
							  gpointer                    user_data,
							  GDestroyNotify              destroy);
GDK_DEPRECATED_IN_3_10
void            gtk_action_group_set_translate_func      (GtkActionGroup             *action_group,
							  GtkTranslateFunc            func,
							  gpointer                    data,
							  GDestroyNotify              notify);
GDK_DEPRECATED_IN_3_10
void            gtk_action_group_set_translation_domain  (GtkActionGroup             *action_group,
							  const gchar                *domain);
GDK_DEPRECATED_IN_3_10
const gchar *   gtk_action_group_translate_string        (GtkActionGroup             *action_group,
  	                                                  const gchar                *string);

/* Protected for use by GtkAction */
void _gtk_action_group_emit_connect_proxy    (GtkActionGroup *action_group,
                                              GtkAction      *action,
                                              GtkWidget      *proxy);
void _gtk_action_group_emit_disconnect_proxy (GtkActionGroup *action_group,
                                              GtkAction      *action,
                                              GtkWidget      *proxy);
void _gtk_action_group_emit_pre_activate     (GtkActionGroup *action_group,
                                              GtkAction      *action);
void _gtk_action_group_emit_post_activate    (GtkActionGroup *action_group,
                                              GtkAction      *action);

G_END_DECLS

#endif  /* __GTK_ACTION_GROUP_H__ */
