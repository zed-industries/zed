/* gtkstatusicon.h:
 *
 * Copyright (C) 2003 Sun Microsystems, Inc.
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
 *
 * Authors:
 *      Mark McLoughlin <mark@skynet.ie>
 */

#ifndef __GTK_STATUS_ICON_H__
#define __GTK_STATUS_ICON_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkimage.h>
#include <gtk/gtkmenu.h>

G_BEGIN_DECLS

#define GTK_TYPE_STATUS_ICON         (gtk_status_icon_get_type ())
#define GTK_STATUS_ICON(o)           (G_TYPE_CHECK_INSTANCE_CAST ((o), GTK_TYPE_STATUS_ICON, GtkStatusIcon))
#define GTK_STATUS_ICON_CLASS(k)     (G_TYPE_CHECK_CLASS_CAST ((k), GTK_TYPE_STATUS_ICON, GtkStatusIconClass))
#define GTK_IS_STATUS_ICON(o)        (G_TYPE_CHECK_INSTANCE_TYPE ((o), GTK_TYPE_STATUS_ICON))
#define GTK_IS_STATUS_ICON_CLASS(k)  (G_TYPE_CHECK_CLASS_TYPE ((k), GTK_TYPE_STATUS_ICON))
#define GTK_STATUS_ICON_GET_CLASS(o) (G_TYPE_INSTANCE_GET_CLASS ((o), GTK_TYPE_STATUS_ICON, GtkStatusIconClass))

typedef struct _GtkStatusIcon	     GtkStatusIcon;
typedef struct _GtkStatusIconClass   GtkStatusIconClass;
typedef struct _GtkStatusIconPrivate GtkStatusIconPrivate;

struct _GtkStatusIcon
{
  GObject               parent_instance;

  GtkStatusIconPrivate *priv;
};

struct _GtkStatusIconClass
{
  GObjectClass parent_class;

  void     (* activate)             (GtkStatusIcon  *status_icon);
  void     (* popup_menu)           (GtkStatusIcon  *status_icon,
                                     guint           button,
                                     guint32         activate_time);
  gboolean (* size_changed)         (GtkStatusIcon  *status_icon,
                                     gint            size);
  gboolean (* button_press_event)   (GtkStatusIcon  *status_icon,
                                     GdkEventButton *event);
  gboolean (* button_release_event) (GtkStatusIcon  *status_icon,
                                     GdkEventButton *event);
  gboolean (* scroll_event)         (GtkStatusIcon  *status_icon,
                                     GdkEventScroll *event);
  gboolean (* query_tooltip)        (GtkStatusIcon  *status_icon,
                                     gint            x,
                                     gint            y,
                                     gboolean        keyboard_mode,
                                     GtkTooltip     *tooltip);

  void (*__gtk_reserved1) (void);
  void (*__gtk_reserved2) (void);
  void (*__gtk_reserved3) (void);
  void (*__gtk_reserved4) (void);
};

GDK_AVAILABLE_IN_ALL
GType                 gtk_status_icon_get_type           (void) G_GNUC_CONST;

GDK_DEPRECATED_IN_3_14
GtkStatusIcon        *gtk_status_icon_new                (void);
GDK_DEPRECATED_IN_3_14
GtkStatusIcon        *gtk_status_icon_new_from_pixbuf    (GdkPixbuf          *pixbuf);
GDK_DEPRECATED_IN_3_14
GtkStatusIcon        *gtk_status_icon_new_from_file      (const gchar        *filename);
GDK_DEPRECATED_IN_3_10_FOR(gtk_status_icon_new_from_icon_name)
GtkStatusIcon        *gtk_status_icon_new_from_stock     (const gchar        *stock_id);
GDK_DEPRECATED_IN_3_14
GtkStatusIcon        *gtk_status_icon_new_from_icon_name (const gchar        *icon_name);
GDK_DEPRECATED_IN_3_14
GtkStatusIcon        *gtk_status_icon_new_from_gicon     (GIcon              *icon);

GDK_DEPRECATED_IN_3_14
void                  gtk_status_icon_set_from_pixbuf    (GtkStatusIcon      *status_icon,
							  GdkPixbuf          *pixbuf);
GDK_DEPRECATED_IN_3_14
void                  gtk_status_icon_set_from_file      (GtkStatusIcon      *status_icon,
							  const gchar        *filename);
GDK_DEPRECATED_IN_3_10_FOR(gtk_status_icon_set_from_icon_name)
void                  gtk_status_icon_set_from_stock     (GtkStatusIcon      *status_icon,
							  const gchar        *stock_id);
GDK_DEPRECATED_IN_3_14
void                  gtk_status_icon_set_from_icon_name (GtkStatusIcon      *status_icon,
							  const gchar        *icon_name);
GDK_DEPRECATED_IN_3_14
void                  gtk_status_icon_set_from_gicon     (GtkStatusIcon      *status_icon,
                                                          GIcon              *icon);

GDK_DEPRECATED_IN_3_14
GtkImageType          gtk_status_icon_get_storage_type   (GtkStatusIcon      *status_icon);

GDK_DEPRECATED_IN_3_14
GdkPixbuf            *gtk_status_icon_get_pixbuf         (GtkStatusIcon      *status_icon);
GDK_DEPRECATED_IN_3_10_FOR(gtk_status_icon_get_icon_name)
const gchar *         gtk_status_icon_get_stock          (GtkStatusIcon      *status_icon);
GDK_DEPRECATED_IN_3_14
const gchar *         gtk_status_icon_get_icon_name      (GtkStatusIcon      *status_icon);
GDK_DEPRECATED_IN_3_14
GIcon                *gtk_status_icon_get_gicon          (GtkStatusIcon      *status_icon);

GDK_DEPRECATED_IN_3_14
gint                  gtk_status_icon_get_size           (GtkStatusIcon      *status_icon);

GDK_DEPRECATED_IN_3_14
void                  gtk_status_icon_set_screen         (GtkStatusIcon      *status_icon,
                                                          GdkScreen          *screen);
GDK_DEPRECATED_IN_3_14
GdkScreen            *gtk_status_icon_get_screen         (GtkStatusIcon      *status_icon);

GDK_DEPRECATED_IN_3_14
void                  gtk_status_icon_set_has_tooltip    (GtkStatusIcon      *status_icon,
                                                          gboolean            has_tooltip);
GDK_DEPRECATED_IN_3_14
void                  gtk_status_icon_set_tooltip_text   (GtkStatusIcon      *status_icon,
                                                          const gchar        *text);
GDK_DEPRECATED_IN_3_14
void                  gtk_status_icon_set_tooltip_markup (GtkStatusIcon      *status_icon,
                                                          const gchar        *markup);
GDK_DEPRECATED_IN_3_14
void                  gtk_status_icon_set_title          (GtkStatusIcon      *status_icon,
                                                          const gchar        *title);
GDK_DEPRECATED_IN_3_14
const gchar *         gtk_status_icon_get_title          (GtkStatusIcon      *status_icon);
GDK_DEPRECATED_IN_3_14
void                  gtk_status_icon_set_name           (GtkStatusIcon      *status_icon,
                                                          const gchar        *name);
GDK_DEPRECATED_IN_3_14
void                  gtk_status_icon_set_visible        (GtkStatusIcon      *status_icon,
							  gboolean            visible);
GDK_DEPRECATED_IN_3_14
gboolean              gtk_status_icon_get_visible        (GtkStatusIcon      *status_icon);

GDK_DEPRECATED_IN_3_14
gboolean              gtk_status_icon_is_embedded        (GtkStatusIcon      *status_icon);

GDK_DEPRECATED_IN_3_14
void                  gtk_status_icon_position_menu      (GtkMenu            *menu,
							  gint               *x,
							  gint               *y,
							  gboolean           *push_in,
							  gpointer            user_data);
GDK_DEPRECATED_IN_3_14
gboolean              gtk_status_icon_get_geometry       (GtkStatusIcon      *status_icon,
							  GdkScreen         **screen,
							  GdkRectangle       *area,
							  GtkOrientation     *orientation);
GDK_DEPRECATED_IN_3_14
gboolean              gtk_status_icon_get_has_tooltip    (GtkStatusIcon      *status_icon);
GDK_DEPRECATED_IN_3_14
gchar                *gtk_status_icon_get_tooltip_text   (GtkStatusIcon      *status_icon);
GDK_DEPRECATED_IN_3_14
gchar                *gtk_status_icon_get_tooltip_markup (GtkStatusIcon      *status_icon);

GDK_DEPRECATED_IN_3_14
guint32               gtk_status_icon_get_x11_window_id  (GtkStatusIcon      *status_icon);

G_END_DECLS

#endif /* __GTK_STATUS_ICON_H__ */
