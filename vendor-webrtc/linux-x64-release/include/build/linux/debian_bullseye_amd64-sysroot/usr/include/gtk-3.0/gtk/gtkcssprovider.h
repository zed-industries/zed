/* GTK - The GIMP Toolkit
 * Copyright (C) 2010 Carlos Garnacho <carlosg@gnome.org>
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

#ifndef __GTK_CSS_PROVIDER_H__
#define __GTK_CSS_PROVIDER_H__

#include <gio/gio.h>
#include <gtk/gtkcsssection.h>

G_BEGIN_DECLS

#define GTK_TYPE_CSS_PROVIDER         (gtk_css_provider_get_type ())
#define GTK_CSS_PROVIDER(o)           (G_TYPE_CHECK_INSTANCE_CAST ((o), GTK_TYPE_CSS_PROVIDER, GtkCssProvider))
#define GTK_CSS_PROVIDER_CLASS(c)     (G_TYPE_CHECK_CLASS_CAST    ((c), GTK_TYPE_CSS_PROVIDER, GtkCssProviderClass))
#define GTK_IS_CSS_PROVIDER(o)        (G_TYPE_CHECK_INSTANCE_TYPE ((o), GTK_TYPE_CSS_PROVIDER))
#define GTK_IS_CSS_PROVIDER_CLASS(c)  (G_TYPE_CHECK_CLASS_TYPE    ((c), GTK_TYPE_CSS_PROVIDER))
#define GTK_CSS_PROVIDER_GET_CLASS(o) (G_TYPE_INSTANCE_GET_CLASS  ((o), GTK_TYPE_CSS_PROVIDER, GtkCssProviderClass))

/**
 * GTK_CSS_PROVIDER_ERROR:
 *
 * Domain for #GtkCssProvider errors.
 */
#define GTK_CSS_PROVIDER_ERROR (gtk_css_provider_error_quark ())

/**
 * GtkCssProviderError:
 * @GTK_CSS_PROVIDER_ERROR_FAILED: Failed.
 * @GTK_CSS_PROVIDER_ERROR_SYNTAX: Syntax error.
 * @GTK_CSS_PROVIDER_ERROR_IMPORT: Import error.
 * @GTK_CSS_PROVIDER_ERROR_NAME: Name error.
 * @GTK_CSS_PROVIDER_ERROR_DEPRECATED: Deprecation error.
 * @GTK_CSS_PROVIDER_ERROR_UNKNOWN_VALUE: Unknown value.
 *
 * Error codes for %GTK_CSS_PROVIDER_ERROR.
 */
typedef enum
{
  GTK_CSS_PROVIDER_ERROR_FAILED,
  GTK_CSS_PROVIDER_ERROR_SYNTAX,
  GTK_CSS_PROVIDER_ERROR_IMPORT,
  GTK_CSS_PROVIDER_ERROR_NAME,
  GTK_CSS_PROVIDER_ERROR_DEPRECATED,
  GTK_CSS_PROVIDER_ERROR_UNKNOWN_VALUE
} GtkCssProviderError;

GDK_AVAILABLE_IN_ALL
GQuark gtk_css_provider_error_quark (void);

typedef struct _GtkCssProvider GtkCssProvider;
typedef struct _GtkCssProviderClass GtkCssProviderClass;
typedef struct _GtkCssProviderPrivate GtkCssProviderPrivate;

struct _GtkCssProvider
{
  GObject parent_instance;
  GtkCssProviderPrivate *priv;
};

struct _GtkCssProviderClass
{
  GObjectClass parent_class;

  void (* parsing_error)                        (GtkCssProvider  *provider,
                                                 GtkCssSection   *section,
                                                 const GError *   error);

  /* Padding for future expansion */
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

GDK_AVAILABLE_IN_ALL
GType gtk_css_provider_get_type (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GtkCssProvider * gtk_css_provider_new (void);

GDK_AVAILABLE_IN_3_2
char *           gtk_css_provider_to_string      (GtkCssProvider  *provider);

GDK_AVAILABLE_IN_ALL
gboolean         gtk_css_provider_load_from_data (GtkCssProvider  *css_provider,
                                                  const gchar     *data,
                                                  gssize           length,
                                                  GError         **error);
GDK_AVAILABLE_IN_ALL
gboolean         gtk_css_provider_load_from_file (GtkCssProvider  *css_provider,
                                                  GFile           *file,
                                                  GError         **error);
GDK_AVAILABLE_IN_ALL
gboolean         gtk_css_provider_load_from_path (GtkCssProvider  *css_provider,
                                                  const gchar     *path,
                                                  GError         **error);

GDK_AVAILABLE_IN_3_16
void             gtk_css_provider_load_from_resource (GtkCssProvider *css_provider,
                                                      const gchar    *resource_path);

GDK_DEPRECATED_FOR(gtk_css_provider_new)
GtkCssProvider * gtk_css_provider_get_default (void);

GDK_AVAILABLE_IN_ALL
GtkCssProvider * gtk_css_provider_get_named (const gchar *name,
                                             const gchar *variant);

G_END_DECLS

#endif /* __GTK_CSS_PROVIDER_H__ */
