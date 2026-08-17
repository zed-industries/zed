/* GTK - The GIMP Toolkit
 * Copyright (C) 1995-1997 Peter Mattis, Spencer Kimball and Josh MacDonald
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
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

#ifndef __GTK_MENU_BAR_H__
#define __GTK_MENU_BAR_H__


#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkmenushell.h>


G_BEGIN_DECLS


#define	GTK_TYPE_MENU_BAR               (gtk_menu_bar_get_type ())
#define GTK_MENU_BAR(obj)               (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_MENU_BAR, GtkMenuBar))
#define GTK_MENU_BAR_CLASS(klass)       (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_MENU_BAR, GtkMenuBarClass))
#define GTK_IS_MENU_BAR(obj)            (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_MENU_BAR))
#define GTK_IS_MENU_BAR_CLASS(klass)    (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_MENU_BAR))
#define GTK_MENU_BAR_GET_CLASS(obj)     (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_MENU_BAR, GtkMenuBarClass))

typedef struct _GtkMenuBar         GtkMenuBar;
typedef struct _GtkMenuBarPrivate  GtkMenuBarPrivate;
typedef struct _GtkMenuBarClass    GtkMenuBarClass;

struct _GtkMenuBar
{
  GtkMenuShell menu_shell;

  /*< private >*/
  GtkMenuBarPrivate *priv;
};

struct _GtkMenuBarClass
{
  GtkMenuShellClass parent_class;

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};


GDK_AVAILABLE_IN_ALL
GType      gtk_menu_bar_get_type        (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkWidget* gtk_menu_bar_new             (void);
GDK_AVAILABLE_IN_3_4
GtkWidget* gtk_menu_bar_new_from_model  (GMenuModel *model);

GDK_AVAILABLE_IN_ALL
GtkPackDirection gtk_menu_bar_get_pack_direction (GtkMenuBar       *menubar);
GDK_AVAILABLE_IN_ALL
void             gtk_menu_bar_set_pack_direction (GtkMenuBar       *menubar,
						  GtkPackDirection  pack_dir);
GDK_AVAILABLE_IN_ALL
GtkPackDirection gtk_menu_bar_get_child_pack_direction (GtkMenuBar       *menubar);
GDK_AVAILABLE_IN_ALL
void             gtk_menu_bar_set_child_pack_direction (GtkMenuBar       *menubar,
							GtkPackDirection  child_pack_dir);

/* Private functions */
void _gtk_menu_bar_cycle_focus (GtkMenuBar       *menubar,
				GtkDirectionType  dir);
GList* _gtk_menu_bar_get_viewable_menu_bars (GtkWindow *window);



G_END_DECLS


#endif /* __GTK_MENU_BAR_H__ */
