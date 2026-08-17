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

#ifndef __GTK_MENU_ITEM_H__
#define __GTK_MENU_ITEM_H__


#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkitem.h>


G_BEGIN_DECLS

#define	GTK_TYPE_MENU_ITEM		(gtk_menu_item_get_type ())
#define GTK_MENU_ITEM(obj)		(G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_MENU_ITEM, GtkMenuItem))
#define GTK_MENU_ITEM_CLASS(klass)	(G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_MENU_ITEM, GtkMenuItemClass))
#define GTK_IS_MENU_ITEM(obj)		(G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_MENU_ITEM))
#define GTK_IS_MENU_ITEM_CLASS(klass)	(G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_MENU_ITEM))
#define GTK_MENU_ITEM_GET_CLASS(obj)    (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_MENU_ITEM, GtkMenuItemClass))


typedef struct _GtkMenuItem	  GtkMenuItem;
typedef struct _GtkMenuItemClass  GtkMenuItemClass;

struct _GtkMenuItem
{
  GtkItem item;

  GtkWidget *GSEAL (submenu);
  GdkWindow *GSEAL (event_window);

  guint16 GSEAL (toggle_size);
  guint16 GSEAL (accelerator_width);
  gchar  *GSEAL (accel_path);

  guint GSEAL (show_submenu_indicator) : 1;
  guint GSEAL (submenu_placement) : 1;
  guint GSEAL (submenu_direction) : 1;
  guint GSEAL (right_justify): 1;
  guint GSEAL (timer_from_keypress) : 1;
  guint GSEAL (from_menubar) : 1;
  guint GSEAL (timer);
};

struct _GtkMenuItemClass
{
  GtkItemClass parent_class;
  
  /* If the following flag is true, then we should always hide
   * the menu when the MenuItem is activated. Otherwise, the 
   * it is up to the caller. For instance, when navigating
   * a menu with the keyboard, <Space> doesn't hide, but
   * <Return> does.
   */
  guint hide_on_activate : 1;
  
  void (* activate)             (GtkMenuItem *menu_item);
  void (* activate_item)        (GtkMenuItem *menu_item);
  void (* toggle_size_request)  (GtkMenuItem *menu_item,
				 gint        *requisition);
  void (* toggle_size_allocate) (GtkMenuItem *menu_item,
				 gint         allocation);
  void (* set_label)            (GtkMenuItem *menu_item,
				 const gchar *label);
  const gchar *(* get_label) (GtkMenuItem *menu_item);

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
};


GType	   gtk_menu_item_get_type	      (void) G_GNUC_CONST;
GtkWidget* gtk_menu_item_new                  (void);
GtkWidget* gtk_menu_item_new_with_label       (const gchar         *label);
GtkWidget* gtk_menu_item_new_with_mnemonic    (const gchar         *label);
void       gtk_menu_item_set_submenu          (GtkMenuItem         *menu_item,
					       GtkWidget           *submenu);
GtkWidget* gtk_menu_item_get_submenu          (GtkMenuItem         *menu_item);
void       gtk_menu_item_select               (GtkMenuItem         *menu_item);
void       gtk_menu_item_deselect             (GtkMenuItem         *menu_item);
void       gtk_menu_item_activate             (GtkMenuItem         *menu_item);
void       gtk_menu_item_toggle_size_request  (GtkMenuItem         *menu_item,
					       gint                *requisition);
void       gtk_menu_item_toggle_size_allocate (GtkMenuItem         *menu_item,
					       gint                 allocation);
void       gtk_menu_item_set_right_justified  (GtkMenuItem         *menu_item,
					       gboolean             right_justified);
gboolean   gtk_menu_item_get_right_justified  (GtkMenuItem         *menu_item);
void	   gtk_menu_item_set_accel_path	      (GtkMenuItem	   *menu_item,
					       const gchar	   *accel_path);
const gchar* gtk_menu_item_get_accel_path     (GtkMenuItem    *menu_item);

void       gtk_menu_item_set_label            (GtkMenuItem         *menu_item,
 					       const gchar         *label);
const gchar *gtk_menu_item_get_label          (GtkMenuItem         *menu_item);

void       gtk_menu_item_set_use_underline    (GtkMenuItem         *menu_item,
 					       gboolean             setting);
gboolean   gtk_menu_item_get_use_underline    (GtkMenuItem         *menu_item);

/* private */
void	  _gtk_menu_item_refresh_accel_path   (GtkMenuItem	   *menu_item,
					       const gchar	   *prefix,
					       GtkAccelGroup	   *accel_group,
					       gboolean		    group_changed);
gboolean  _gtk_menu_item_is_selectable        (GtkWidget           *menu_item);
void      _gtk_menu_item_popup_submenu        (GtkWidget           *menu_item,
                                               gboolean             with_delay);
void      _gtk_menu_item_popdown_submenu      (GtkWidget           *menu_item);

#ifndef GTK_DISABLE_DEPRECATED
void       gtk_menu_item_remove_submenu       (GtkMenuItem         *menu_item);
#define gtk_menu_item_right_justify(menu_item) gtk_menu_item_set_right_justified ((menu_item), TRUE)
#endif /* GTK_DISABLE_DEPRECATED */

G_END_DECLS

#endif /* __GTK_MENU_ITEM_H__ */
