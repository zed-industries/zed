/* GTK - The GIMP Toolkit
 * Copyright (C) Red Hat, Inc.
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

#ifndef __GTK_IMAGE_MENU_ITEM_H__
#define __GTK_IMAGE_MENU_ITEM_H__


#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkmenuitem.h>


G_BEGIN_DECLS

#define GTK_TYPE_IMAGE_MENU_ITEM            (gtk_image_menu_item_get_type ())
#define GTK_IMAGE_MENU_ITEM(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_IMAGE_MENU_ITEM, GtkImageMenuItem))
#define GTK_IMAGE_MENU_ITEM_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_IMAGE_MENU_ITEM, GtkImageMenuItemClass))
#define GTK_IS_IMAGE_MENU_ITEM(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_IMAGE_MENU_ITEM))
#define GTK_IS_IMAGE_MENU_ITEM_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_IMAGE_MENU_ITEM))
#define GTK_IMAGE_MENU_ITEM_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_IMAGE_MENU_ITEM, GtkImageMenuItemClass))


typedef struct _GtkImageMenuItem              GtkImageMenuItem;
typedef struct _GtkImageMenuItemPrivate       GtkImageMenuItemPrivate;
typedef struct _GtkImageMenuItemClass         GtkImageMenuItemClass;

struct _GtkImageMenuItem
{
  GtkMenuItem menu_item;

  /*< private >*/
  GtkImageMenuItemPrivate *priv;
};

/**
 * GtkImageMenuItemClass:
 * @parent_class: The parent class.
 */
struct _GtkImageMenuItemClass
{
  GtkMenuItemClass parent_class;

  /*< private >*/

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

GDK_DEPRECATED_IN_3_10_FOR(gtk_menu_item_get_type)
GType	   gtk_image_menu_item_get_type          (void) G_GNUC_CONST;
GDK_DEPRECATED_IN_3_10_FOR(gtk_menu_item_new)
GtkWidget* gtk_image_menu_item_new               (void);
GDK_DEPRECATED_IN_3_10_FOR(gtk_menu_item_new_with_label)
GtkWidget* gtk_image_menu_item_new_with_label    (const gchar      *label);
GDK_DEPRECATED_IN_3_10_FOR(gtk_menu_item_new_with_mnemonic)
GtkWidget* gtk_image_menu_item_new_with_mnemonic (const gchar      *label);
GDK_DEPRECATED_IN_3_10_FOR(gtk_menu_item_new)
GtkWidget* gtk_image_menu_item_new_from_stock    (const gchar      *stock_id,
                                                  GtkAccelGroup    *accel_group);
GDK_DEPRECATED_IN_3_10
void       gtk_image_menu_item_set_always_show_image (GtkImageMenuItem *image_menu_item,
                                                      gboolean          always_show);
GDK_DEPRECATED_IN_3_10
gboolean   gtk_image_menu_item_get_always_show_image (GtkImageMenuItem *image_menu_item);
GDK_DEPRECATED_IN_3_10
void       gtk_image_menu_item_set_image         (GtkImageMenuItem *image_menu_item,
                                                  GtkWidget        *image);
GDK_DEPRECATED_IN_3_10
GtkWidget* gtk_image_menu_item_get_image         (GtkImageMenuItem *image_menu_item);
GDK_DEPRECATED_IN_3_10
void       gtk_image_menu_item_set_use_stock     (GtkImageMenuItem *image_menu_item,
						  gboolean          use_stock);
GDK_DEPRECATED_IN_3_10
gboolean   gtk_image_menu_item_get_use_stock     (GtkImageMenuItem *image_menu_item);
GDK_DEPRECATED_IN_3_10
void       gtk_image_menu_item_set_accel_group   (GtkImageMenuItem *image_menu_item, 
						  GtkAccelGroup    *accel_group);

G_END_DECLS

#endif /* __GTK_IMAGE_MENU_ITEM_H__ */
