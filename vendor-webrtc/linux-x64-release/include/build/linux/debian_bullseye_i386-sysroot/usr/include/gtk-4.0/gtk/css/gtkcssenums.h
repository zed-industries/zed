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

#ifndef __GTK_CSS_ENUMS_H__
#define __GTK_CSS_ENUMS_H__

#if !defined (__GTK_CSS_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/css/gtkcss.h> can be included directly."
#endif

#include <glib.h>
#include <gdk/gdkversionmacros.h>

/**
 * GtkCssParserError:
 * @GTK_CSS_PARSER_ERROR_FAILED: Unknown failure.
 * @GTK_CSS_PARSER_ERROR_SYNTAX: The given text does not form valid syntax
 * @GTK_CSS_PARSER_ERROR_IMPORT: Failed to import a resource
 * @GTK_CSS_PARSER_ERROR_NAME: The given name has not been defined
 * @GTK_CSS_PARSER_ERROR_UNKNOWN_VALUE: The given value is not correct
 *
 * Errors that can occur while parsing CSS.
 *
 * These errors are unexpected and will cause parts of the given CSS
 * to be ignored.
 */
typedef enum
{
  GTK_CSS_PARSER_ERROR_FAILED,
  GTK_CSS_PARSER_ERROR_SYNTAX,
  GTK_CSS_PARSER_ERROR_IMPORT,
  GTK_CSS_PARSER_ERROR_NAME,
  GTK_CSS_PARSER_ERROR_UNKNOWN_VALUE
} GtkCssParserError;

/**
 * GtkCssParserWarning:
 * @GTK_CSS_PARSER_WARNING_DEPRECATED: The given construct is
 *   deprecated and will be removed in a future version
 * @GTK_CSS_PARSER_WARNING_SYNTAX: A syntax construct was used
 *   that should be avoided
 * @GTK_CSS_PARSER_WARNING_UNIMPLEMENTED: A feature is not implemented
 *
 * Warnings that can occur while parsing CSS.
 *
 * Unlike `GtkCssParserError`s, warnings do not cause the parser to
 * skip any input, but they indicate issues that should be fixed.
 */
typedef enum
{
  GTK_CSS_PARSER_WARNING_DEPRECATED,
  GTK_CSS_PARSER_WARNING_SYNTAX,
  GTK_CSS_PARSER_WARNING_UNIMPLEMENTED
} GtkCssParserWarning;

#endif /* __GTK_CSS_ENUMS_H__ */
