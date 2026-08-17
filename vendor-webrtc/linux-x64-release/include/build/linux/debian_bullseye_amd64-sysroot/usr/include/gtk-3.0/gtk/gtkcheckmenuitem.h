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
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.	 See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

/*
 * Modified by the GTK+ Team and others 1997-2001.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GTK_CHECK_MENU_ITEM_H__
#define __GTK_CHECK_MENU_ITEM_H__


#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkmenuitem.h>


G_BEGIN_DECLS

#define GTK_TYPE_CHECK_MENU_ITEM            (gtk_check_menu_item_get_type ())
#define GTK_CHECK_MENU_ITEM(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_CHECK_MENU_ITEM, GtkCheckMenuItem))
#define GTK_CHECK_MENU_ITEM_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_CHECK_MENU_ITEM, GtkCheckMenuItemClass))
#define GTK_IS_CHECK_MENU_ITEM(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_CHECK_MENU_ITEM))
#define GTK_IS_CHECK_MENU_ITEM_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_CHECK_MENU_ITEM))
#define GTK_CHECK_MENU_ITEM_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_CHECK_MENU_ITEM, GtkCheckMenuItemClass))


typedef struct _GtkCheckMenuItem              GtkCheckMenuItem;
typedef struct _GtkCheckMenuItemPrivate       GtkCheckMenuItemPrivate;
typedef struct _GtkCheckMenuItemClass         GtkCheckMenuItemClass;

struct _GtkCheckMenuItem
{
  GtkMenuItem menu_item;

  /*< private >*/
  GtkCheckMenuItemPrivate *priv;
};

/**
 * GtkCheckMenuItemClass:
 * @parent_class: The parent class.
 * @toggled: Signal emitted when the state of the check box is changed.
 * @draw_indicator: Called to draw the check indicator.
 */
struct _GtkCheckMenuItemClass
{
  GtkMenuItemClass parent_class;

  /*< public >*/

  void (* toggled)	  (GtkCheckMenuItem *check_menu_item);
  void (* draw_indicator) (GtkCheckMenuItem *check_menu_item,
			   cairo_t          *cr);

  /*< private >*/

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};


GDK_AVAILABLE_IN_ALL
GType	   gtk_check_menu_item_get_type	         (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GtkWidget* gtk_check_menu_item_new               (void);
GDK_AVAILABLE_IN_ALL
GtkWidget* gtk_check_menu_item_new_with_label    (const gchar      *label);
GDK_AVAILABLE_IN_ALL
GtkWidget* gtk_check_menu_item_new_with_mnemonic (const gchar      *label);
GDK_AVAILABLE_IN_ALL
void       gtk_check_menu_item_set_active        (GtkCheckMenuItem *check_menu_item,
						  gboolean          is_active);
GDK_AVAILABLE_IN_ALL
gboolean   gtk_check_menu_item_get_active        (GtkCheckMenuItem *check_menu_item);
GDK_AVAILABLE_IN_ALL
void       gtk_check_menu_item_toggled           (GtkCheckMenuItem *check_menu_item);
GDK_AVAILABLE_IN_ALL
void       gtk_check_menu_item_set_inconsistent  (GtkCheckMenuItem *check_menu_item,
						  gboolean          setting);
GDK_AVAILABLE_IN_ALL
gboolean   gtk_check_menu_item_get_inconsistent  (GtkCheckMenuItem *check_menu_item);
GDK_AVAILABLE_IN_ALL
void       gtk_check_menu_item_set_draw_as_radio (GtkCheckMenuItem *check_menu_item,
						  gboolean          draw_as_radio);
GDK_AVAILABLE_IN_ALL
gboolean   gtk_check_menu_item_get_draw_as_radio (GtkCheckMenuItem *check_menu_item);

G_END_DECLS

#endif /* __GTK_CHECK_MENU_ITEM_H__ */
