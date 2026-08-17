/*
 * Copyright © 2019 Benjamin Otte
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 *
 * Authors: Benjamin Otte <otte@gnome.org>
 */


#ifndef __GTK_CSS_ERROR_H__
#define __GTK_CSS_ERROR_H__

#include <glib.h>
#include <gdk/gdkversionmacros.h>

G_BEGIN_DECLS

/**
 * GTK_CSS_PARSER_ERROR:
 *
 * Domain for `GtkCssParser` errors.
 */
#define GTK_CSS_PARSER_ERROR (gtk_css_parser_error_quark ())

GDK_AVAILABLE_IN_ALL
GQuark gtk_css_parser_error_quark (void);

/**
 * GTK_CSS_PARSER_WARNING:
 *
 * Domain for `GtkCssParser` warnings.
 */
#define GTK_CSS_PARSER_WARNING (gtk_css_parser_warning_quark ())

GDK_AVAILABLE_IN_ALL
GQuark gtk_css_parser_warning_quark (void);

G_END_DECLS

#endif /* __GTK_CSS_ERROR_H__ */
