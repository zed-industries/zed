/* GTK - The GIMP Toolkit
 * Copyright (C) 2011 Red Hat, Inc.
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

#ifndef __GTK_CSS_SECTION_H__
#define __GTK_CSS_SECTION_H__

#if !defined (__GTK_CSS_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/css/gtkcss.h> can be included directly."
#endif

#include <gio/gio.h>
#include <gdk/gdkversionmacros.h>
#include <gtk/css/gtkcsslocation.h>

G_BEGIN_DECLS

#define GTK_TYPE_CSS_SECTION         (gtk_css_section_get_type ())

/**
 * GtkCssSection:
 *
 * Defines a part of a CSS document.
 *
 * Because sections are nested into one another, you can use
 * [method@CssSection.get_parent] to get the containing region.
 */
typedef struct _GtkCssSection GtkCssSection;

GDK_AVAILABLE_IN_ALL
GType              gtk_css_section_get_type            (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GtkCssSection *    gtk_css_section_new                 (GFile                *file,
                                                        const GtkCssLocation *start,
                                                        const GtkCssLocation *end);
GDK_AVAILABLE_IN_ALL
GtkCssSection *    gtk_css_section_ref                 (GtkCssSection        *section);
GDK_AVAILABLE_IN_ALL
void               gtk_css_section_unref               (GtkCssSection        *section);

GDK_AVAILABLE_IN_ALL
void               gtk_css_section_print               (const GtkCssSection  *section,
                                                        GString              *string);
GDK_AVAILABLE_IN_ALL
char *             gtk_css_section_to_string           (const GtkCssSection  *section);

GDK_AVAILABLE_IN_ALL
GtkCssSection *    gtk_css_section_get_parent          (const GtkCssSection  *section);
GDK_AVAILABLE_IN_ALL
GFile *            gtk_css_section_get_file            (const GtkCssSection  *section);
GDK_AVAILABLE_IN_ALL
const GtkCssLocation *
                   gtk_css_section_get_start_location  (const GtkCssSection  *section);
GDK_AVAILABLE_IN_ALL
const GtkCssLocation *
                   gtk_css_section_get_end_location    (const GtkCssSection  *section);

G_END_DECLS

#endif /* __GTK_CSS_SECTION_H__ */
