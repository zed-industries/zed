/*
 * Copyright (c) 2013 - 2014 Red Hat, Inc.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or (at your
 * option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
 * or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public
 * License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
 *
 */

#ifndef __GTK_ACTION_BAR_H__
#define __GTK_ACTION_BAR_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkbin.h>

G_BEGIN_DECLS

#define GTK_TYPE_ACTION_BAR            (gtk_action_bar_get_type ())
#define GTK_ACTION_BAR(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_ACTION_BAR, GtkActionBar))
#define GTK_ACTION_BAR_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_ACTION_BAR, GtkActionBarClass))
#define GTK_IS_ACTION_BAR(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_ACTION_BAR))
#define GTK_IS_ACTION_BAR_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_ACTION_BAR))
#define GTK_ACTION_BAR_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_ACTION_BAR, GtkActionBarClass))

typedef struct _GtkActionBar              GtkActionBar;
typedef struct _GtkActionBarPrivate       GtkActionBarPrivate;
typedef struct _GtkActionBarClass         GtkActionBarClass;

struct _GtkActionBar
{
  /*< private >*/
  GtkBin bin;
};

struct _GtkActionBarClass
{
  /*< private >*/
  GtkBinClass parent_class;

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

GDK_AVAILABLE_IN_3_12
GType        gtk_action_bar_get_type          (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_3_12
GtkWidget   *gtk_action_bar_new               (void);
GDK_AVAILABLE_IN_3_12
GtkWidget   *gtk_action_bar_get_center_widget (GtkActionBar *action_bar);
GDK_AVAILABLE_IN_3_12
void         gtk_action_bar_set_center_widget (GtkActionBar *action_bar,
                                               GtkWidget    *center_widget);

GDK_AVAILABLE_IN_3_12
void         gtk_action_bar_pack_start        (GtkActionBar *action_bar,
                                               GtkWidget    *child);
GDK_AVAILABLE_IN_3_12
void         gtk_action_bar_pack_end          (GtkActionBar *action_bar,
                                               GtkWidget    *child);
G_END_DECLS

#endif /* __GTK_ACTION_BAR_H__ */
