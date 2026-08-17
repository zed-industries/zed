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

#ifndef __GTK_UI_MANAGER_H__
#define __GTK_UI_MANAGER_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkaccelgroup.h>
#include <gtk/gtkwidget.h>
#include <gtk/deprecated/gtkaction.h>
#include <gtk/deprecated/gtkactiongroup.h>

G_BEGIN_DECLS

#define GTK_TYPE_UI_MANAGER            (gtk_ui_manager_get_type ())
#define GTK_UI_MANAGER(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_UI_MANAGER, GtkUIManager))
#define GTK_UI_MANAGER_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_UI_MANAGER, GtkUIManagerClass))
#define GTK_IS_UI_MANAGER(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_UI_MANAGER))
#define GTK_IS_UI_MANAGER_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_UI_MANAGER))
#define GTK_UI_MANAGER_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS((obj), GTK_TYPE_UI_MANAGER, GtkUIManagerClass))

typedef struct _GtkUIManager      GtkUIManager;
typedef struct _GtkUIManagerClass GtkUIManagerClass;
typedef struct _GtkUIManagerPrivate GtkUIManagerPrivate;


struct _GtkUIManager {
  GObject parent;

  /*< private >*/
  GtkUIManagerPrivate *private_data;
};

struct _GtkUIManagerClass {
  GObjectClass parent_class;

  /* Signals */
  void (* add_widget)       (GtkUIManager *manager,
                             GtkWidget    *widget);
  void (* actions_changed)  (GtkUIManager *manager);
  void (* connect_proxy)    (GtkUIManager *manager,
			     GtkAction    *action,
			     GtkWidget    *proxy);
  void (* disconnect_proxy) (GtkUIManager *manager,
			     GtkAction    *action,
			     GtkWidget    *proxy);
  void (* pre_activate)     (GtkUIManager *manager,
			     GtkAction    *action);
  void (* post_activate)    (GtkUIManager *manager,
			     GtkAction    *action);

  /* Virtual functions */
  GtkWidget * (* get_widget) (GtkUIManager *manager,
                              const gchar  *path);
  GtkAction * (* get_action) (GtkUIManager *manager,
                              const gchar  *path);

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

/**
 * GtkUIManagerItemType:
 * @GTK_UI_MANAGER_AUTO: Pick the type of the UI element according to context.
 * @GTK_UI_MANAGER_MENUBAR: Create a menubar.
 * @GTK_UI_MANAGER_MENU: Create a menu.
 * @GTK_UI_MANAGER_TOOLBAR: Create a toolbar.
 * @GTK_UI_MANAGER_PLACEHOLDER: Insert a placeholder.
 * @GTK_UI_MANAGER_POPUP: Create a popup menu.
 * @GTK_UI_MANAGER_MENUITEM: Create a menuitem.
 * @GTK_UI_MANAGER_TOOLITEM: Create a toolitem.
 * @GTK_UI_MANAGER_SEPARATOR: Create a separator.
 * @GTK_UI_MANAGER_ACCELERATOR: Install an accelerator.
 * @GTK_UI_MANAGER_POPUP_WITH_ACCELS: Same as %GTK_UI_MANAGER_POPUP, but the
 *   actions’ accelerators are shown.
 *
 * These enumeration values are used by gtk_ui_manager_add_ui() to determine
 * what UI element to create.
 *
 * Deprecated: 3.10
 */
typedef enum {
  GTK_UI_MANAGER_AUTO              = 0,
  GTK_UI_MANAGER_MENUBAR           = 1 << 0,
  GTK_UI_MANAGER_MENU              = 1 << 1,
  GTK_UI_MANAGER_TOOLBAR           = 1 << 2,
  GTK_UI_MANAGER_PLACEHOLDER       = 1 << 3,
  GTK_UI_MANAGER_POPUP             = 1 << 4,
  GTK_UI_MANAGER_MENUITEM          = 1 << 5,
  GTK_UI_MANAGER_TOOLITEM          = 1 << 6,
  GTK_UI_MANAGER_SEPARATOR         = 1 << 7,
  GTK_UI_MANAGER_ACCELERATOR       = 1 << 8,
  GTK_UI_MANAGER_POPUP_WITH_ACCELS = 1 << 9
} GtkUIManagerItemType;

GDK_DEPRECATED_IN_3_10
GType          gtk_ui_manager_get_type            (void) G_GNUC_CONST;
GDK_DEPRECATED_IN_3_10
GtkUIManager  *gtk_ui_manager_new                 (void);
GDK_DEPRECATED_IN_3_4
void           gtk_ui_manager_set_add_tearoffs    (GtkUIManager          *manager,
                                                   gboolean               add_tearoffs);
GDK_DEPRECATED_IN_3_4
gboolean       gtk_ui_manager_get_add_tearoffs    (GtkUIManager          *manager);

GDK_DEPRECATED_IN_3_10
void           gtk_ui_manager_insert_action_group (GtkUIManager          *manager,
						   GtkActionGroup        *action_group,
						   gint                   pos);
GDK_DEPRECATED_IN_3_10
void           gtk_ui_manager_remove_action_group (GtkUIManager          *manager,
						   GtkActionGroup        *action_group);
GDK_DEPRECATED_IN_3_10
GList         *gtk_ui_manager_get_action_groups   (GtkUIManager          *manager);
GDK_DEPRECATED_IN_3_10
GtkAccelGroup *gtk_ui_manager_get_accel_group     (GtkUIManager          *manager);
GDK_DEPRECATED_IN_3_10
GtkWidget     *gtk_ui_manager_get_widget          (GtkUIManager          *manager,
						   const gchar           *path);
GDK_DEPRECATED_IN_3_10
GSList        *gtk_ui_manager_get_toplevels       (GtkUIManager          *manager,
						   GtkUIManagerItemType   types);
GDK_DEPRECATED_IN_3_10
GtkAction     *gtk_ui_manager_get_action          (GtkUIManager          *manager,
						   const gchar           *path);
GDK_DEPRECATED_IN_3_10
guint          gtk_ui_manager_add_ui_from_string  (GtkUIManager          *manager,
						   const gchar           *buffer,
						   gssize                 length,
						   GError               **error);
GDK_DEPRECATED_IN_3_10
guint          gtk_ui_manager_add_ui_from_file    (GtkUIManager          *manager,
						   const gchar           *filename,
						   GError               **error);
GDK_DEPRECATED_IN_3_10
guint          gtk_ui_manager_add_ui_from_resource(GtkUIManager          *manager,
						   const gchar           *resource_path,
						   GError               **error);
GDK_DEPRECATED_IN_3_10
void           gtk_ui_manager_add_ui              (GtkUIManager          *manager,
						   guint                  merge_id,
						   const gchar           *path,
						   const gchar           *name,
						   const gchar           *action,
						   GtkUIManagerItemType   type,
						   gboolean               top);
GDK_DEPRECATED_IN_3_10
void           gtk_ui_manager_remove_ui           (GtkUIManager          *manager,
						   guint                  merge_id);
GDK_DEPRECATED_IN_3_10
gchar         *gtk_ui_manager_get_ui              (GtkUIManager          *manager);
GDK_DEPRECATED_IN_3_10
void           gtk_ui_manager_ensure_update       (GtkUIManager          *manager);
GDK_DEPRECATED_IN_3_10
guint          gtk_ui_manager_new_merge_id        (GtkUIManager          *manager);

G_END_DECLS

#endif /* __GTK_UI_MANAGER_H__ */
