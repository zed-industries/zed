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
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

/*
 * Modified by the GTK+ Team and others 1997-2000.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GTK_RADIO_MENU_ITEM_H__
#define __GTK_RADIO_MENU_ITEM_H__


#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkcheckmenuitem.h>


G_BEGIN_DECLS

#define GTK_TYPE_RADIO_MENU_ITEM	      (gtk_radio_menu_item_get_type ())
#define GTK_RADIO_MENU_ITEM(obj)	      (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_RADIO_MENU_ITEM, GtkRadioMenuItem))
#define GTK_RADIO_MENU_ITEM_CLASS(klass)      (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_RADIO_MENU_ITEM, GtkRadioMenuItemClass))
#define GTK_IS_RADIO_MENU_ITEM(obj)	      (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_RADIO_MENU_ITEM))
#define GTK_IS_RADIO_MENU_ITEM_CLASS(klass)   (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_RADIO_MENU_ITEM))
#define GTK_RADIO_MENU_ITEM_GET_CLASS(obj)    (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_RADIO_MENU_ITEM, GtkRadioMenuItemClass))


typedef struct _GtkRadioMenuItem       GtkRadioMenuItem;
typedef struct _GtkRadioMenuItemClass  GtkRadioMenuItemClass;

struct _GtkRadioMenuItem
{
  GtkCheckMenuItem check_menu_item;

  GSList *GSEAL (group);
};

struct _GtkRadioMenuItemClass
{
  GtkCheckMenuItemClass parent_class;

  /* Signals */
  void (*group_changed) (GtkRadioMenuItem *radio_menu_item);

  /* Padding for future expansion */
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};


GType      gtk_radio_menu_item_get_type	         (void) G_GNUC_CONST;

GtkWidget* gtk_radio_menu_item_new                           (GSList           *group);
GtkWidget* gtk_radio_menu_item_new_with_label                (GSList           *group,
							      const gchar      *label);
GtkWidget* gtk_radio_menu_item_new_with_mnemonic             (GSList           *group,
							      const gchar      *label);
GtkWidget* gtk_radio_menu_item_new_from_widget               (GtkRadioMenuItem *group);
GtkWidget *gtk_radio_menu_item_new_with_mnemonic_from_widget (GtkRadioMenuItem *group,
							      const gchar      *label);
GtkWidget *gtk_radio_menu_item_new_with_label_from_widget    (GtkRadioMenuItem *group,
							      const gchar      *label);
GSList*    gtk_radio_menu_item_get_group                     (GtkRadioMenuItem *radio_menu_item);
void       gtk_radio_menu_item_set_group                     (GtkRadioMenuItem *radio_menu_item,
							      GSList           *group);

#ifndef GTK_DISABLE_DEPRECATED
#define gtk_radio_menu_item_group gtk_radio_menu_item_get_group
#endif

G_END_DECLS

#endif /* __GTK_RADIO_MENU_ITEM_H__ */
