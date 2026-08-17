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

#include <gio/gio.h>
#include <gdk/gdk.h>

G_BEGIN_DECLS

#define GTK_TYPE_CSS_SECTION         (gtk_css_section_get_type ())

/**
 * GtkCssSectionType:
 * @GTK_CSS_SECTION_DOCUMENT: The section describes a complete document.
 *   This section time is the only one where gtk_css_section_get_parent()
 *   might return %NULL.
 * @GTK_CSS_SECTION_IMPORT: The section defines an import rule.
 * @GTK_CSS_SECTION_COLOR_DEFINITION: The section defines a color. This
 *   is a GTK extension to CSS.
 * @GTK_CSS_SECTION_BINDING_SET: The section defines a binding set. This
 *   is a GTK extension to CSS.
 * @GTK_CSS_SECTION_RULESET: The section defines a CSS ruleset.
 * @GTK_CSS_SECTION_SELECTOR: The section defines a CSS selector.
 * @GTK_CSS_SECTION_DECLARATION: The section defines the declaration of
 *   a CSS variable.
 * @GTK_CSS_SECTION_VALUE: The section defines the value of a CSS declaration.
 * @GTK_CSS_SECTION_KEYFRAMES: The section defines keyframes. See [CSS
 *   Animations](http://dev.w3.org/csswg/css3-animations/#keyframes) for details. Since 3.6
 *
 * The different types of sections indicate parts of a CSS document as
 * parsed by GTK’s CSS parser. They are oriented towards the
 * [CSS Grammar](http://www.w3.org/TR/CSS21/grammar.html),
 * but may contain extensions.
 *
 * More types might be added in the future as the parser incorporates
 * more features.
 *
 * Since: 3.2
 */
typedef enum
{
  GTK_CSS_SECTION_DOCUMENT,
  GTK_CSS_SECTION_IMPORT,
  GTK_CSS_SECTION_COLOR_DEFINITION,
  GTK_CSS_SECTION_BINDING_SET,
  GTK_CSS_SECTION_RULESET,
  GTK_CSS_SECTION_SELECTOR,
  GTK_CSS_SECTION_DECLARATION,
  GTK_CSS_SECTION_VALUE,
  GTK_CSS_SECTION_KEYFRAMES
} GtkCssSectionType;

/**
 * GtkCssSection:
 *
 * Defines a part of a CSS document. Because sections are nested into
 * one another, you can use gtk_css_section_get_parent() to get the
 * containing region.
 *
 * Since: 3.2
 */
typedef struct _GtkCssSection GtkCssSection;

GDK_AVAILABLE_IN_3_2
GType              gtk_css_section_get_type            (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_3_2
GtkCssSection *    gtk_css_section_ref                 (GtkCssSection        *section);
GDK_AVAILABLE_IN_3_2
void               gtk_css_section_unref               (GtkCssSection        *section);

GDK_AVAILABLE_IN_3_2
GtkCssSectionType  gtk_css_section_get_section_type    (const GtkCssSection  *section);
GDK_AVAILABLE_IN_3_2
GtkCssSection *    gtk_css_section_get_parent          (const GtkCssSection  *section);
GDK_AVAILABLE_IN_3_2
GFile *            gtk_css_section_get_file            (const GtkCssSection  *section);
GDK_AVAILABLE_IN_3_2
guint              gtk_css_section_get_start_line      (const GtkCssSection  *section);
GDK_AVAILABLE_IN_3_2
guint              gtk_css_section_get_start_position  (const GtkCssSection  *section);
GDK_AVAILABLE_IN_3_2
guint              gtk_css_section_get_end_line        (const GtkCssSection  *section);
GDK_AVAILABLE_IN_3_2
guint              gtk_css_section_get_end_position    (const GtkCssSection  *section);

G_END_DECLS

#endif /* __GTK_CSS_SECTION_H__ */
