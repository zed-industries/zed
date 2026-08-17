/* GTK - The GIMP Toolkit
 * gtkpapersize.h: Paper Size
 * Copyright (C) 2006, Red Hat, Inc.
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

#ifndef __GTK_PAPER_SIZE_H__
#define __GTK_PAPER_SIZE_H__


#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gdk/gdk.h>
#include <gtk/gtkenums.h>


G_BEGIN_DECLS

typedef struct _GtkPaperSize GtkPaperSize;

#define GTK_TYPE_PAPER_SIZE    (gtk_paper_size_get_type ())

/* Common names, from PWG 5101.1-2002 PWG: Standard for Media Standardized Names */
/**
 * GTK_PAPER_NAME_A3:
 *
 * Name for the A3 paper size.
 */
#define GTK_PAPER_NAME_A3 "iso_a3"

/**
 * GTK_PAPER_NAME_A4:
 *
 * Name for the A4 paper size.
 */
#define GTK_PAPER_NAME_A4 "iso_a4"

/**
 * GTK_PAPER_NAME_A5:
 *
 * Name for the A5 paper size.
 */
#define GTK_PAPER_NAME_A5 "iso_a5"

/**
 * GTK_PAPER_NAME_B5:
 *
 * Name for the B5 paper size.
 */
#define GTK_PAPER_NAME_B5 "iso_b5"

/**
 * GTK_PAPER_NAME_LETTER:
 *
 * Name for the Letter paper size.
 */
#define GTK_PAPER_NAME_LETTER "na_letter"

/**
 * GTK_PAPER_NAME_EXECUTIVE:
 *
 * Name for the Executive paper size.
 */
#define GTK_PAPER_NAME_EXECUTIVE "na_executive"

/**
 * GTK_PAPER_NAME_LEGAL:
 *
 * Name for the Legal paper size.
 */
#define GTK_PAPER_NAME_LEGAL "na_legal"

GDK_AVAILABLE_IN_ALL
GType gtk_paper_size_get_type (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GtkPaperSize *gtk_paper_size_new          (const gchar  *name);
GDK_AVAILABLE_IN_ALL
GtkPaperSize *gtk_paper_size_new_from_ppd (const gchar  *ppd_name,
					   const gchar  *ppd_display_name,
					   gdouble       width,
					   gdouble       height);
GDK_AVAILABLE_IN_ALL
GtkPaperSize *gtk_paper_size_new_from_ipp (const gchar  *ipp_name,
					   gdouble       width,
					   gdouble       height);
GDK_AVAILABLE_IN_ALL
GtkPaperSize *gtk_paper_size_new_custom   (const gchar  *name,
					   const gchar  *display_name,
					   gdouble       width,
					   gdouble       height,
					   GtkUnit       unit);
GDK_AVAILABLE_IN_ALL
GtkPaperSize *gtk_paper_size_copy         (GtkPaperSize *other);
GDK_AVAILABLE_IN_ALL
void          gtk_paper_size_free         (GtkPaperSize *size);
GDK_AVAILABLE_IN_ALL
gboolean      gtk_paper_size_is_equal     (GtkPaperSize *size1,
					   GtkPaperSize *size2);

GDK_AVAILABLE_IN_ALL
GList        *gtk_paper_size_get_paper_sizes (gboolean include_custom);

/* The width is always the shortest side, measure in mm */
GDK_AVAILABLE_IN_ALL
const gchar *gtk_paper_size_get_name         (GtkPaperSize *size);
GDK_AVAILABLE_IN_ALL
const gchar *gtk_paper_size_get_display_name (GtkPaperSize *size);
GDK_AVAILABLE_IN_ALL
const gchar *gtk_paper_size_get_ppd_name     (GtkPaperSize *size);

GDK_AVAILABLE_IN_ALL
gdouble  gtk_paper_size_get_width        (GtkPaperSize *size, GtkUnit unit);
GDK_AVAILABLE_IN_ALL
gdouble  gtk_paper_size_get_height       (GtkPaperSize *size, GtkUnit unit);
GDK_AVAILABLE_IN_ALL
gboolean gtk_paper_size_is_custom        (GtkPaperSize *size);
GDK_AVAILABLE_IN_ALL
gboolean gtk_paper_size_is_ipp           (GtkPaperSize *size);

/* Only for custom sizes: */
GDK_AVAILABLE_IN_ALL
void    gtk_paper_size_set_size                  (GtkPaperSize *size, 
                                                  gdouble       width, 
                                                  gdouble       height, 
                                                  GtkUnit       unit);

GDK_AVAILABLE_IN_ALL
gdouble gtk_paper_size_get_default_top_margin    (GtkPaperSize *size,
						  GtkUnit       unit);
GDK_AVAILABLE_IN_ALL
gdouble gtk_paper_size_get_default_bottom_margin (GtkPaperSize *size,
						  GtkUnit       unit);
GDK_AVAILABLE_IN_ALL
gdouble gtk_paper_size_get_default_left_margin   (GtkPaperSize *size,
						  GtkUnit       unit);
GDK_AVAILABLE_IN_ALL
gdouble gtk_paper_size_get_default_right_margin  (GtkPaperSize *size,
						  GtkUnit       unit);

GDK_AVAILABLE_IN_ALL
const gchar *gtk_paper_size_get_default (void);

GDK_AVAILABLE_IN_ALL
GtkPaperSize *gtk_paper_size_new_from_key_file (GKeyFile    *key_file,
					        const gchar *group_name,
					        GError     **error);
GDK_AVAILABLE_IN_ALL
void     gtk_paper_size_to_key_file            (GtkPaperSize *size,
					        GKeyFile     *key_file,
					        const gchar  *group_name);

GDK_AVAILABLE_IN_3_22
GtkPaperSize *gtk_paper_size_new_from_gvariant (GVariant     *variant);
GDK_AVAILABLE_IN_3_22
GVariant     *gtk_paper_size_to_gvariant       (GtkPaperSize *paper_size);

G_END_DECLS

#endif /* __GTK_PAPER_SIZE_H__ */
