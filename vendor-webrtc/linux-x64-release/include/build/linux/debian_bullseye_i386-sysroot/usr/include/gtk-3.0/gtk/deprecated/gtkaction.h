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

#ifndef __GTK_ACTION_H__
#define __GTK_ACTION_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwidget.h>

G_BEGIN_DECLS

#define GTK_TYPE_ACTION            (gtk_action_get_type ())
#define GTK_ACTION(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_ACTION, GtkAction))
#define GTK_ACTION_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_ACTION, GtkActionClass))
#define GTK_IS_ACTION(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_ACTION))
#define GTK_IS_ACTION_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_ACTION))
#define GTK_ACTION_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS((obj), GTK_TYPE_ACTION, GtkActionClass))

typedef struct _GtkAction      GtkAction;
typedef struct _GtkActionClass GtkActionClass;
typedef struct _GtkActionPrivate GtkActionPrivate;

struct _GtkAction
{
  GObject object;

  /*< private >*/
  GtkActionPrivate *private_data;
};

/**
 * GtkActionClass:
 * @parent_class: The parent class.
 * @activate: Signal emitted when the action is activated.
 */
struct _GtkActionClass
{
  GObjectClass parent_class;

  /*< public >*/

  /* activation signal */
  void       (* activate)           (GtkAction    *action);

  /*< private >*/

  GType      menu_item_type;
  GType      toolbar_item_type;

  /* widget creation routines (not signals) */
  GtkWidget *(* create_menu_item)   (GtkAction *action);
  GtkWidget *(* create_tool_item)   (GtkAction *action);
  void       (* connect_proxy)      (GtkAction *action,
				     GtkWidget *proxy);
  void       (* disconnect_proxy)   (GtkAction *action,
				     GtkWidget *proxy);

  GtkWidget *(* create_menu)        (GtkAction *action);

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

GDK_DEPRECATED_IN_3_10
GType        gtk_action_get_type               (void) G_GNUC_CONST;
GDK_DEPRECATED_IN_3_10
GtkAction   *gtk_action_new                    (const gchar *name,
						const gchar *label,
						const gchar *tooltip,
						const gchar *stock_id);
GDK_DEPRECATED_IN_3_10
const gchar* gtk_action_get_name               (GtkAction     *action);
GDK_DEPRECATED_IN_3_10
gboolean     gtk_action_is_sensitive           (GtkAction     *action);
GDK_DEPRECATED_IN_3_10
gboolean     gtk_action_get_sensitive          (GtkAction     *action);
GDK_DEPRECATED_IN_3_10
void         gtk_action_set_sensitive          (GtkAction     *action,
						gboolean       sensitive);
GDK_DEPRECATED_IN_3_10
gboolean     gtk_action_is_visible             (GtkAction     *action);
GDK_DEPRECATED_IN_3_10
gboolean     gtk_action_get_visible            (GtkAction     *action);
GDK_DEPRECATED_IN_3_10
void         gtk_action_set_visible            (GtkAction     *action,
						gboolean       visible);
GDK_DEPRECATED_IN_3_10
void         gtk_action_activate               (GtkAction     *action);
GDK_DEPRECATED_IN_3_10
GtkWidget *  gtk_action_create_icon            (GtkAction     *action,
						GtkIconSize    icon_size);
GDK_DEPRECATED_IN_3_10
GtkWidget *  gtk_action_create_menu_item       (GtkAction     *action);
GDK_DEPRECATED_IN_3_10
GtkWidget *  gtk_action_create_tool_item       (GtkAction     *action);
GDK_DEPRECATED_IN_3_10
GtkWidget *  gtk_action_create_menu            (GtkAction     *action);
GDK_DEPRECATED_IN_3_10
GSList *     gtk_action_get_proxies            (GtkAction     *action);
GDK_DEPRECATED_IN_3_10
void         gtk_action_connect_accelerator    (GtkAction     *action);
GDK_DEPRECATED_IN_3_10
void         gtk_action_disconnect_accelerator (GtkAction     *action);
GDK_DEPRECATED_IN_3_10
const gchar *gtk_action_get_accel_path         (GtkAction     *action);
GDK_DEPRECATED_IN_3_10
GClosure    *gtk_action_get_accel_closure      (GtkAction     *action);
GDK_DEPRECATED_IN_3_10
void         gtk_action_block_activate         (GtkAction     *action);
GDK_DEPRECATED_IN_3_10
void         gtk_action_unblock_activate       (GtkAction     *action);

void         _gtk_action_add_to_proxy_list     (GtkAction     *action,
						GtkWidget     *proxy);
void         _gtk_action_remove_from_proxy_list(GtkAction     *action,
						GtkWidget     *proxy);

/* protected ... for use by child actions */
void         _gtk_action_emit_activate         (GtkAction     *action);

/* protected ... for use by action groups */
GDK_DEPRECATED_IN_3_10
void         gtk_action_set_accel_path         (GtkAction     *action,
						const gchar   *accel_path);
GDK_DEPRECATED_IN_3_10
void         gtk_action_set_accel_group        (GtkAction     *action,
						GtkAccelGroup *accel_group);
void         _gtk_action_sync_menu_visible     (GtkAction     *action,
						GtkWidget     *proxy,
						gboolean       empty);

GDK_DEPRECATED_IN_3_10
void                  gtk_action_set_label              (GtkAction   *action,
                                                         const gchar *label);
GDK_DEPRECATED_IN_3_10
const gchar *         gtk_action_get_label              (GtkAction   *action);
GDK_DEPRECATED_IN_3_10
void                  gtk_action_set_short_label        (GtkAction   *action,
                                                         const gchar *short_label);
GDK_DEPRECATED_IN_3_10
const gchar *         gtk_action_get_short_label        (GtkAction   *action);
GDK_DEPRECATED_IN_3_10
void                  gtk_action_set_tooltip            (GtkAction   *action,
                                                         const gchar *tooltip);
GDK_DEPRECATED_IN_3_10
const gchar *         gtk_action_get_tooltip            (GtkAction   *action);
GDK_DEPRECATED_IN_3_10
void                  gtk_action_set_stock_id           (GtkAction   *action,
                                                         const gchar *stock_id);
GDK_DEPRECATED_IN_3_10
const gchar *         gtk_action_get_stock_id           (GtkAction   *action);
GDK_DEPRECATED_IN_3_10
void                  gtk_action_set_gicon              (GtkAction   *action,
                                                         GIcon       *icon);
GDK_DEPRECATED_IN_3_10
GIcon                *gtk_action_get_gicon              (GtkAction   *action);
GDK_DEPRECATED_IN_3_10
void                  gtk_action_set_icon_name          (GtkAction   *action,
                                                         const gchar *icon_name);
GDK_DEPRECATED_IN_3_10
const gchar *         gtk_action_get_icon_name          (GtkAction   *action);
GDK_DEPRECATED_IN_3_10
void                  gtk_action_set_visible_horizontal (GtkAction   *action,
                                                         gboolean     visible_horizontal);
GDK_DEPRECATED_IN_3_10
gboolean              gtk_action_get_visible_horizontal (GtkAction   *action);
GDK_DEPRECATED_IN_3_10
void                  gtk_action_set_visible_vertical   (GtkAction   *action,
                                                         gboolean     visible_vertical);
GDK_DEPRECATED_IN_3_10
gboolean              gtk_action_get_visible_vertical   (GtkAction   *action);
GDK_DEPRECATED_IN_3_10
void                  gtk_action_set_is_important       (GtkAction   *action,
                                                         gboolean     is_important);
GDK_DEPRECATED_IN_3_10
gboolean              gtk_action_get_is_important       (GtkAction   *action);
GDK_DEPRECATED_IN_3_10
void                  gtk_action_set_always_show_image  (GtkAction   *action,
                                                         gboolean     always_show);
GDK_DEPRECATED_IN_3_10
gboolean              gtk_action_get_always_show_image  (GtkAction   *action);


G_END_DECLS

#endif  /* __GTK_ACTION_H__ */
