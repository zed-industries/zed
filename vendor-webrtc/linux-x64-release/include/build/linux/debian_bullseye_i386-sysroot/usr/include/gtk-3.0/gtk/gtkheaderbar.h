/*
 * Copyright (c) 2013 Red Hat, Inc.
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

#ifndef __GTK_HEADER_BAR_H__
#define __GTK_HEADER_BAR_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkcontainer.h>

G_BEGIN_DECLS

#define GTK_TYPE_HEADER_BAR            (gtk_header_bar_get_type ())
#define GTK_HEADER_BAR(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_HEADER_BAR, GtkHeaderBar))
#define GTK_HEADER_BAR_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_HEADER_BAR, GtkHeaderBarClass))
#define GTK_IS_HEADER_BAR(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_HEADER_BAR))
#define GTK_IS_HEADER_BAR_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_HEADER_BAR))
#define GTK_HEADER_BAR_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_HEADER_BAR, GtkHeaderBarClass))

typedef struct _GtkHeaderBar              GtkHeaderBar;
typedef struct _GtkHeaderBarPrivate       GtkHeaderBarPrivate;
typedef struct _GtkHeaderBarClass         GtkHeaderBarClass;

struct _GtkHeaderBar
{
  GtkContainer container;
};

struct _GtkHeaderBarClass
{
  GtkContainerClass parent_class;

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

GDK_AVAILABLE_IN_3_10
GType        gtk_header_bar_get_type          (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_3_10
GtkWidget   *gtk_header_bar_new               (void);
GDK_AVAILABLE_IN_3_10
void         gtk_header_bar_set_title         (GtkHeaderBar *bar,
                                               const gchar  *title);
GDK_AVAILABLE_IN_3_10
const gchar *gtk_header_bar_get_title         (GtkHeaderBar *bar);
GDK_AVAILABLE_IN_3_10
void         gtk_header_bar_set_subtitle      (GtkHeaderBar *bar,
                                               const gchar  *subtitle);
GDK_AVAILABLE_IN_3_10
const gchar *gtk_header_bar_get_subtitle      (GtkHeaderBar *bar);


GDK_AVAILABLE_IN_3_10
void         gtk_header_bar_set_custom_title  (GtkHeaderBar *bar,
                                               GtkWidget    *title_widget);
GDK_AVAILABLE_IN_3_10
GtkWidget   *gtk_header_bar_get_custom_title  (GtkHeaderBar *bar);
GDK_AVAILABLE_IN_3_10
void         gtk_header_bar_pack_start        (GtkHeaderBar *bar,
                                               GtkWidget    *child);
GDK_AVAILABLE_IN_3_10
void         gtk_header_bar_pack_end          (GtkHeaderBar *bar,
                                               GtkWidget    *child);

GDK_AVAILABLE_IN_3_10
gboolean     gtk_header_bar_get_show_close_button (GtkHeaderBar *bar);

GDK_AVAILABLE_IN_3_10
void         gtk_header_bar_set_show_close_button (GtkHeaderBar *bar,
                                                   gboolean      setting);

GDK_AVAILABLE_IN_3_12
void         gtk_header_bar_set_has_subtitle (GtkHeaderBar *bar,
                                              gboolean      setting);
GDK_AVAILABLE_IN_3_12
gboolean     gtk_header_bar_get_has_subtitle (GtkHeaderBar *bar);

GDK_AVAILABLE_IN_3_12
void         gtk_header_bar_set_decoration_layout (GtkHeaderBar *bar,
                                                   const gchar  *layout);
GDK_AVAILABLE_IN_3_12
const gchar *gtk_header_bar_get_decoration_layout (GtkHeaderBar *bar);

G_END_DECLS

#endif /* __GTK_HEADER_BAR_H__ */
