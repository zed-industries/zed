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

#ifndef __GTK_MENU_ITEM_H__
#define __GTK_MENU_ITEM_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkbin.h>


G_BEGIN_DECLS

#define GTK_TYPE_MENU_ITEM            (gtk_menu_item_get_type ())
#define GTK_MENU_ITEM(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_MENU_ITEM, GtkMenuItem))
#define GTK_MENU_ITEM_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_MENU_ITEM, GtkMenuItemClass))
#define GTK_IS_MENU_ITEM(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_MENU_ITEM))
#define GTK_IS_MENU_ITEM_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_MENU_ITEM))
#define GTK_MENU_ITEM_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_MENU_ITEM, GtkMenuItemClass))


typedef struct _GtkMenuItem        GtkMenuItem;
typedef struct _GtkMenuItemClass   GtkMenuItemClass;
typedef struct _GtkMenuItemPrivate GtkMenuItemPrivate;

struct _GtkMenuItem
{
  GtkBin bin;

  /*< private >*/
  GtkMenuItemPrivate *priv;
};

/**
 * GtkMenuItemClass:
 * @parent_class: The parent class.
 * @hide_on_activate: If %TRUE, then we should always
 *    hide the menu when the %GtkMenuItem is activated. Otherwise,
 *    it is up to the caller.
 * @activate: Signal emitted when the item is activated.
 * @activate_item: Signal emitted when the item is activated, but also
 *    if the menu item has a submenu.
 * @toggle_size_request: 
 * @toggle_size_allocate: 
 * @set_label: Sets @text on the #GtkMenuItem label
 * @get_label: Gets @text from the #GtkMenuItem label
 * @select: Signal emitted when the item is selected.
 * @deselect: Signal emitted when the item is deselected.
 */
struct _GtkMenuItemClass
{
  GtkBinClass parent_class;

  /*< public >*/

  /* If the following flag is true, then we should always
   * hide the menu when the MenuItem is activated. Otherwise,
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
  const gchar * (* get_label)   (GtkMenuItem *menu_item);

  void (* select)               (GtkMenuItem *menu_item);
  void (* deselect)             (GtkMenuItem *menu_item);

  /*< private >*/

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};


GDK_AVAILABLE_IN_ALL
GType      gtk_menu_item_get_type             (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GtkWidget* gtk_menu_item_new                  (void);
GDK_AVAILABLE_IN_ALL
GtkWidget* gtk_menu_item_new_with_label       (const gchar         *label);
GDK_AVAILABLE_IN_ALL
GtkWidget* gtk_menu_item_new_with_mnemonic    (const gchar         *label);
GDK_AVAILABLE_IN_ALL
void       gtk_menu_item_set_submenu          (GtkMenuItem         *menu_item,
                                               GtkWidget           *submenu);
GDK_AVAILABLE_IN_ALL
GtkWidget* gtk_menu_item_get_submenu          (GtkMenuItem         *menu_item);
GDK_AVAILABLE_IN_ALL
void       gtk_menu_item_select               (GtkMenuItem         *menu_item);
GDK_AVAILABLE_IN_ALL
void       gtk_menu_item_deselect             (GtkMenuItem         *menu_item);
GDK_AVAILABLE_IN_ALL
void       gtk_menu_item_activate             (GtkMenuItem         *menu_item);
GDK_AVAILABLE_IN_ALL
void       gtk_menu_item_toggle_size_request  (GtkMenuItem         *menu_item,
                                               gint                *requisition);
GDK_AVAILABLE_IN_ALL
void       gtk_menu_item_toggle_size_allocate (GtkMenuItem         *menu_item,
                                               gint                 allocation);
GDK_DEPRECATED_IN_3_2
void       gtk_menu_item_set_right_justified  (GtkMenuItem         *menu_item,
                                               gboolean             right_justified);
GDK_DEPRECATED_IN_3_2
gboolean   gtk_menu_item_get_right_justified  (GtkMenuItem         *menu_item);
GDK_AVAILABLE_IN_ALL
void          gtk_menu_item_set_accel_path    (GtkMenuItem         *menu_item,
                                               const gchar         *accel_path);
GDK_AVAILABLE_IN_ALL
const gchar * gtk_menu_item_get_accel_path    (GtkMenuItem    *menu_item);

GDK_AVAILABLE_IN_ALL
void          gtk_menu_item_set_label         (GtkMenuItem         *menu_item,
                                               const gchar         *label);
GDK_AVAILABLE_IN_ALL
const gchar * gtk_menu_item_get_label         (GtkMenuItem         *menu_item);

GDK_AVAILABLE_IN_ALL
void       gtk_menu_item_set_use_underline    (GtkMenuItem         *menu_item,
                                               gboolean             setting);
GDK_AVAILABLE_IN_ALL
gboolean   gtk_menu_item_get_use_underline    (GtkMenuItem         *menu_item);

GDK_AVAILABLE_IN_ALL
void       gtk_menu_item_set_reserve_indicator (GtkMenuItem        *menu_item,
                                                gboolean            reserve);
GDK_AVAILABLE_IN_ALL
gboolean   gtk_menu_item_get_reserve_indicator (GtkMenuItem        *menu_item);

G_END_DECLS

#endif /* __GTK_MENU_ITEM_H__ */
