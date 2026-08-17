/* GtkIconTheme - a loader for icon themes
 * gtk-icon-loader.h Copyright (C) 2002, 2003 Red Hat, Inc.
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

#ifndef __GTK_ICON_THEME_H__
#define __GTK_ICON_THEME_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gdk/gdk.h>
#include <gtk/gtkenums.h>

G_BEGIN_DECLS

#define GTK_TYPE_ICON_PAINTABLE    (gtk_icon_paintable_get_type ())
#define GTK_ICON_PAINTABLE(obj)    (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_ICON_PAINTABLE, GtkIconPaintable))
#define GTK_IS_ICON_PAINTABLE(obj) (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_ICON_PAINTABLE))

#define GTK_TYPE_ICON_THEME        (gtk_icon_theme_get_type ())
#define GTK_ICON_THEME(obj)        (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_ICON_THEME, GtkIconTheme))
#define GTK_IS_ICON_THEME(obj)     (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_ICON_THEME))

typedef struct _GtkIconPaintable  GtkIconPaintable;
typedef struct _GtkIconTheme      GtkIconTheme;

/**
 * GtkIconLookupFlags:
 * @GTK_ICON_LOOKUP_FORCE_REGULAR: Try to always load regular icons, even
 *   when symbolic icon names are given
 * @GTK_ICON_LOOKUP_FORCE_SYMBOLIC: Try to always load symbolic icons, even
 *   when regular icon names are given
 * @GTK_ICON_LOOKUP_PRELOAD: Starts loading the texture in the background
 *   so it is ready when later needed.
 *
 * Used to specify options for gtk_icon_theme_lookup_icon().
 */
typedef enum
{
  GTK_ICON_LOOKUP_FORCE_REGULAR  = 1 << 0,
  GTK_ICON_LOOKUP_FORCE_SYMBOLIC = 1 << 1,
  GTK_ICON_LOOKUP_PRELOAD        = 1 << 2,
} GtkIconLookupFlags;

/**
 * GTK_ICON_THEME_ERROR:
 *
 * The `GQuark` used for `GtkIconThemeError` errors.
 */
#define GTK_ICON_THEME_ERROR gtk_icon_theme_error_quark ()

/**
 * GtkIconThemeError:
 * @GTK_ICON_THEME_NOT_FOUND: The icon specified does not exist in the theme
 * @GTK_ICON_THEME_FAILED: An unspecified error occurred.
 *
 * Error codes for `GtkIconTheme` operations.
 **/
typedef enum {
  GTK_ICON_THEME_NOT_FOUND,
  GTK_ICON_THEME_FAILED
} GtkIconThemeError;

GDK_AVAILABLE_IN_ALL
GQuark gtk_icon_theme_error_quark (void);

GDK_AVAILABLE_IN_ALL
GType            gtk_icon_theme_get_type             (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GtkIconTheme    *gtk_icon_theme_new                  (void);
GDK_AVAILABLE_IN_ALL
GtkIconTheme    *gtk_icon_theme_get_for_display      (GdkDisplay                  *display);

GDK_AVAILABLE_IN_ALL
GdkDisplay *     gtk_icon_theme_get_display          (GtkIconTheme                *self);

GDK_AVAILABLE_IN_ALL
void             gtk_icon_theme_set_search_path      (GtkIconTheme                *self,
                                                      const char * const          *path);
GDK_AVAILABLE_IN_ALL
char **          gtk_icon_theme_get_search_path      (GtkIconTheme                *self);
GDK_AVAILABLE_IN_ALL
void             gtk_icon_theme_add_search_path      (GtkIconTheme                *self,
                                                      const char                  *path);

GDK_AVAILABLE_IN_ALL
void             gtk_icon_theme_set_resource_path    (GtkIconTheme                *self,
                                                      const char * const          *path);
GDK_AVAILABLE_IN_ALL
char **          gtk_icon_theme_get_resource_path    (GtkIconTheme                *self);
GDK_AVAILABLE_IN_ALL
void             gtk_icon_theme_add_resource_path    (GtkIconTheme                *self,
                                                      const char                  *path);

GDK_AVAILABLE_IN_ALL
void             gtk_icon_theme_set_theme_name       (GtkIconTheme                *self,
                                                      const char                  *theme_name);
GDK_AVAILABLE_IN_ALL
char *           gtk_icon_theme_get_theme_name       (GtkIconTheme                *self);

GDK_AVAILABLE_IN_ALL
gboolean         gtk_icon_theme_has_icon             (GtkIconTheme                *self,
                                                      const char                  *icon_name);
GDK_AVAILABLE_IN_4_2
gboolean         gtk_icon_theme_has_gicon            (GtkIconTheme                *self,
                                                      GIcon                       *gicon);
GDK_AVAILABLE_IN_ALL
int              *gtk_icon_theme_get_icon_sizes      (GtkIconTheme                *self,
                                                      const char                  *icon_name);
GDK_AVAILABLE_IN_ALL
GtkIconPaintable *gtk_icon_theme_lookup_icon         (GtkIconTheme                *self,
                                                      const char                  *icon_name,
                                                      const char                  *fallbacks[],
                                                      int                          size,
                                                      int                          scale,
                                                      GtkTextDirection             direction,
                                                      GtkIconLookupFlags           flags);
GDK_AVAILABLE_IN_ALL
GtkIconPaintable *gtk_icon_theme_lookup_by_gicon     (GtkIconTheme                *self,
                                                      GIcon                       *icon,
                                                      int                          size,
                                                      int                          scale,
                                                      GtkTextDirection             direction,
                                                      GtkIconLookupFlags           flags);
GDK_AVAILABLE_IN_ALL
GtkIconPaintable *gtk_icon_paintable_new_for_file    (GFile                       *file,
                                                      int                          size,
                                                      int                          scale);
GDK_AVAILABLE_IN_ALL
char **               gtk_icon_theme_get_icon_names  (GtkIconTheme                *self);

GDK_AVAILABLE_IN_ALL
GType                 gtk_icon_paintable_get_type         (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GFile *               gtk_icon_paintable_get_file          (GtkIconPaintable  *self);
GDK_AVAILABLE_IN_ALL
const char *         gtk_icon_paintable_get_icon_name     (GtkIconPaintable  *self);
GDK_AVAILABLE_IN_ALL
gboolean              gtk_icon_paintable_is_symbolic       (GtkIconPaintable  *self);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkIconPaintable, g_object_unref)
G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkIconTheme, g_object_unref)

G_END_DECLS

#endif /* __GTK_ICON_THEME_H__ */
