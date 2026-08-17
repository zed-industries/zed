/* GTK - The GIMP Toolkit
 * gtkrecentchoosermenu.h - Recently used items menu widget
 * Copyright (C) 2006, Emmanuele Bassi
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

#ifndef __GTK_RECENT_CHOOSER_MENU_H__
#define __GTK_RECENT_CHOOSER_MENU_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkmenu.h>
#include <gtk/gtkrecentchooser.h>

G_BEGIN_DECLS

#define GTK_TYPE_RECENT_CHOOSER_MENU		(gtk_recent_chooser_menu_get_type ())
#define GTK_RECENT_CHOOSER_MENU(obj)		(G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_RECENT_CHOOSER_MENU, GtkRecentChooserMenu))
#define GTK_IS_RECENT_CHOOSER_MENU(obj)		(G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_RECENT_CHOOSER_MENU))
#define GTK_RECENT_CHOOSER_MENU_CLASS(klass)	(G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_RECENT_CHOOSER_MENU, GtkRecentChooserMenuClass))
#define GTK_IS_RECENT_CHOOSER_MENU_CLASS(klass)	(G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_RECENT_CHOOSER_MENU))
#define GTK_RECENT_CHOOSER_MENU_GET_CLASS(obj)	(G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_RECENT_CHOOSER_MENU, GtkRecentChooserMenuClass))

typedef struct _GtkRecentChooserMenu		GtkRecentChooserMenu;
typedef struct _GtkRecentChooserMenuClass	GtkRecentChooserMenuClass;
typedef struct _GtkRecentChooserMenuPrivate	GtkRecentChooserMenuPrivate;

struct _GtkRecentChooserMenu
{
  GtkMenu parent_instance;

  /*< private >*/
  GtkRecentChooserMenuPrivate *priv;
};

struct _GtkRecentChooserMenuClass
{
  GtkMenuClass parent_class;

  /* padding for future expansion */
  void (* gtk_recent1) (void);
  void (* gtk_recent2) (void);
  void (* gtk_recent3) (void);
  void (* gtk_recent4) (void);
};

GDK_AVAILABLE_IN_ALL
GType      gtk_recent_chooser_menu_get_type         (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GtkWidget *gtk_recent_chooser_menu_new              (void);
GDK_AVAILABLE_IN_ALL
GtkWidget *gtk_recent_chooser_menu_new_for_manager  (GtkRecentManager     *manager);

GDK_AVAILABLE_IN_ALL
gboolean   gtk_recent_chooser_menu_get_show_numbers (GtkRecentChooserMenu *menu);
GDK_AVAILABLE_IN_ALL
void       gtk_recent_chooser_menu_set_show_numbers (GtkRecentChooserMenu *menu,
						     gboolean              show_numbers);

G_END_DECLS

#endif /* ! __GTK_RECENT_CHOOSER_MENU_H__ */
