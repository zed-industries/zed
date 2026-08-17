/* Gtk+ testing utilities
 * Copyright (C) 2007 Imendio AB
 * Authors: Tim Janik
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

#ifndef __GTK_TEST_UTILS_H__
#define __GTK_TEST_UTILS_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwidget.h>
#include <gtk/gtkspinbutton.h>

G_BEGIN_DECLS

/* --- Gtk+ Test Utility API --- */
GDK_AVAILABLE_IN_ALL
void            gtk_test_init                   (int            *argcp,
                                                 char         ***argvp,
                                                 ...);
GDK_AVAILABLE_IN_ALL
void            gtk_test_register_all_types     (void);
GDK_AVAILABLE_IN_ALL
const GType*    gtk_test_list_all_types         (guint          *n_types);
GDK_AVAILABLE_IN_ALL
GtkWidget*      gtk_test_find_widget            (GtkWidget      *widget,
                                                 const gchar    *label_pattern,
                                                 GType           widget_type);
GDK_DEPRECATED_IN_3_20
GtkWidget*      gtk_test_create_widget          (GType           widget_type,
                                                 const gchar    *first_property_name,
                                                 ...);
GDK_DEPRECATED_IN_3_20
GtkWidget*      gtk_test_create_simple_window   (const gchar    *window_title,
                                                 const gchar    *dialog_text);
GDK_DEPRECATED_IN_3_20
GtkWidget*      gtk_test_display_button_window  (const gchar    *window_title,
                                                 const gchar    *dialog_text,
                                                 ...); /* NULL terminated list of (label, &int) pairs */
GDK_DEPRECATED_IN_3_20
void            gtk_test_slider_set_perc        (GtkWidget      *widget, /* GtkRange-alike */
                                                 double          percentage);
GDK_DEPRECATED_IN_3_20
double          gtk_test_slider_get_value       (GtkWidget      *widget);
GDK_DEPRECATED_IN_3_20
gboolean        gtk_test_spin_button_click      (GtkSpinButton  *spinner,
                                                 guint           button,
                                                 gboolean        upwards);
GDK_AVAILABLE_IN_3_10
void            gtk_test_widget_wait_for_draw   (GtkWidget      *widget);
GDK_DEPRECATED_IN_3_20
gboolean        gtk_test_widget_click           (GtkWidget      *widget,
                                                 guint           button,
                                                 GdkModifierType modifiers);
GDK_AVAILABLE_IN_ALL
gboolean        gtk_test_widget_send_key        (GtkWidget      *widget,
                                                 guint           keyval,
                                                 GdkModifierType modifiers);
/* operate on GtkEntry, GtkText, GtkTextView or GtkLabel */
GDK_DEPRECATED_IN_3_20
void            gtk_test_text_set               (GtkWidget      *widget,
                                                 const gchar    *string);
GDK_DEPRECATED_IN_3_20
gchar*          gtk_test_text_get               (GtkWidget      *widget);

/* --- Gtk+ Test low-level API --- */
GDK_AVAILABLE_IN_ALL
GtkWidget*      gtk_test_find_sibling           (GtkWidget      *base_widget,
                                                 GType           widget_type);
GDK_AVAILABLE_IN_ALL
GtkWidget*      gtk_test_find_label             (GtkWidget      *widget,
                                                 const gchar    *label_pattern);
G_END_DECLS

#endif /* __GTK_TEST_UTILS_H__ */
