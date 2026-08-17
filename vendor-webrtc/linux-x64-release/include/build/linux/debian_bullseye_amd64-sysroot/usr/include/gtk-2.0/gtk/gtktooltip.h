/* gtktooltip.h
 *
 * Copyright (C) 2006-2007 Imendio AB
 * Contact: Kristian Rietveld <kris@imendio.com>
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

#ifndef __GTK_TOOLTIP_H__
#define __GTK_TOOLTIP_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwindow.h>

G_BEGIN_DECLS

#define GTK_TYPE_TOOLTIP                 (gtk_tooltip_get_type ())
#define GTK_TOOLTIP(obj)                 (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_TOOLTIP, GtkTooltip))
#define GTK_IS_TOOLTIP(obj)              (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_TOOLTIP))

GType gtk_tooltip_get_type (void);

void gtk_tooltip_set_markup              (GtkTooltip         *tooltip,
                                          const gchar        *markup);
void gtk_tooltip_set_text                (GtkTooltip         *tooltip,
                                          const gchar        *text);
void gtk_tooltip_set_icon                (GtkTooltip         *tooltip,
                                          GdkPixbuf          *pixbuf);
void gtk_tooltip_set_icon_from_stock     (GtkTooltip         *tooltip,
                                          const gchar        *stock_id,
                                          GtkIconSize         size);
void gtk_tooltip_set_icon_from_icon_name (GtkTooltip         *tooltip,
				          const gchar        *icon_name,
				          GtkIconSize         size);
void gtk_tooltip_set_icon_from_gicon     (GtkTooltip         *tooltip,
					  GIcon              *gicon,
					  GtkIconSize         size);
void gtk_tooltip_set_custom	         (GtkTooltip         *tooltip,
                                          GtkWidget          *custom_widget);

void gtk_tooltip_set_tip_area            (GtkTooltip         *tooltip,
                                          const GdkRectangle *rect);

void gtk_tooltip_trigger_tooltip_query   (GdkDisplay         *display);


void _gtk_tooltip_focus_in               (GtkWidget          *widget);
void _gtk_tooltip_focus_out              (GtkWidget          *widget);
void _gtk_tooltip_toggle_keyboard_mode   (GtkWidget          *widget);
void _gtk_tooltip_handle_event           (GdkEvent           *event);
void _gtk_tooltip_hide                   (GtkWidget          *widget);

GtkWidget * _gtk_widget_find_at_coords   (GdkWindow          *window,
                                          gint                window_x,
                                          gint                window_y,
                                          gint               *widget_x,
                                          gint               *widget_y);

G_END_DECLS

#endif /* __GTK_TOOLTIP_H__ */
