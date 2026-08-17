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
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

#ifndef __GTK_ICON_THEME_H__
#define __GTK_ICON_THEME_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gdk-pixbuf/gdk-pixbuf.h>
#include <gdk/gdk.h>

G_BEGIN_DECLS

#define GTK_TYPE_ICON_INFO              (gtk_icon_info_get_type ())

#define GTK_TYPE_ICON_THEME             (gtk_icon_theme_get_type ())
#define GTK_ICON_THEME(obj)             (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_ICON_THEME, GtkIconTheme))
#define GTK_ICON_THEME_CLASS(klass)     (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_ICON_THEME, GtkIconThemeClass))
#define GTK_IS_ICON_THEME(obj)          (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_ICON_THEME))
#define GTK_IS_ICON_THEME_CLASS(klass)  (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_ICON_THEME))
#define GTK_ICON_THEME_GET_CLASS(obj)   (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_ICON_THEME, GtkIconThemeClass))

typedef struct _GtkIconInfo         GtkIconInfo;
typedef struct _GtkIconTheme        GtkIconTheme;
typedef struct _GtkIconThemeClass   GtkIconThemeClass;
typedef struct _GtkIconThemePrivate GtkIconThemePrivate;

struct _GtkIconTheme
{
  /*< private >*/
  GObject parent_instance;

  GtkIconThemePrivate *GSEAL (priv);
};

struct _GtkIconThemeClass
{
  GObjectClass parent_class;

  void (* changed)  (GtkIconTheme *icon_theme);
};

/**
 * GtkIconLookupFlags:
 * @GTK_ICON_LOOKUP_NO_SVG: Never return SVG icons, even if gdk-pixbuf
 *   supports them. Cannot be used together with %GTK_ICON_LOOKUP_FORCE_SVG.
 * @GTK_ICON_LOOKUP_FORCE_SVG: Return SVG icons, even if gdk-pixbuf
 *   doesn't support them.
 *   Cannot be used together with %GTK_ICON_LOOKUP_NO_SVG.
 * @GTK_ICON_LOOKUP_USE_BUILTIN: When passed to
 *   gtk_icon_theme_lookup_icon() includes builtin icons
 *   as well as files. For a builtin icon, gtk_icon_info_get_filename()
 *   returns %NULL and you need to call gtk_icon_info_get_builtin_pixbuf().
 * @GTK_ICON_LOOKUP_GENERIC_FALLBACK: Try to shorten icon name at '-'
 *   characters before looking at inherited themes. For more general
 *   fallback, see gtk_icon_theme_choose_icon(). Since 2.12.
 * @GTK_ICON_LOOKUP_FORCE_SIZE: Always return the icon scaled to the
 *   requested size. Since 2.14.
 * 
 * Used to specify options for gtk_icon_theme_lookup_icon()
 **/
typedef enum
{
  GTK_ICON_LOOKUP_NO_SVG           = 1 << 0,
  GTK_ICON_LOOKUP_FORCE_SVG        = 1 << 1,
  GTK_ICON_LOOKUP_USE_BUILTIN      = 1 << 2,
  GTK_ICON_LOOKUP_GENERIC_FALLBACK = 1 << 3,
  GTK_ICON_LOOKUP_FORCE_SIZE       = 1 << 4
} GtkIconLookupFlags;

#define GTK_ICON_THEME_ERROR gtk_icon_theme_error_quark ()

/**
 * GtkIconThemeError:
 * @GTK_ICON_THEME_NOT_FOUND: The icon specified does not exist in the theme
 * @GTK_ICON_THEME_FAILED: An unspecified error occurred.
 * 
 * Error codes for GtkIconTheme operations.
 **/
typedef enum {
  GTK_ICON_THEME_NOT_FOUND,
  GTK_ICON_THEME_FAILED
} GtkIconThemeError;

GQuark gtk_icon_theme_error_quark (void);

#ifdef G_OS_WIN32
/* Reserve old name for DLL ABI backward compatibility */
#define gtk_icon_theme_set_search_path gtk_icon_theme_set_search_path_utf8
#define gtk_icon_theme_get_search_path gtk_icon_theme_get_search_path_utf8
#define gtk_icon_theme_append_search_path gtk_icon_theme_append_search_path_utf8
#define gtk_icon_theme_prepend_search_path gtk_icon_theme_prepend_search_path_utf8
#define gtk_icon_info_get_filename gtk_icon_info_get_filename_utf8
#endif

GType         gtk_icon_theme_get_type              (void) G_GNUC_CONST;

GtkIconTheme *gtk_icon_theme_new                   (void);
GtkIconTheme *gtk_icon_theme_get_default           (void);
GtkIconTheme *gtk_icon_theme_get_for_screen        (GdkScreen                   *screen);
void          gtk_icon_theme_set_screen            (GtkIconTheme                *icon_theme,
						    GdkScreen                   *screen);

void          gtk_icon_theme_set_search_path       (GtkIconTheme                *icon_theme,
						    const gchar                 *path[],
						    gint                         n_elements);
void          gtk_icon_theme_get_search_path       (GtkIconTheme                *icon_theme,
						    gchar                      **path[],
						    gint                        *n_elements);
void          gtk_icon_theme_append_search_path    (GtkIconTheme                *icon_theme,
						    const gchar                 *path);
void          gtk_icon_theme_prepend_search_path   (GtkIconTheme                *icon_theme,
						    const gchar                 *path);

void          gtk_icon_theme_set_custom_theme      (GtkIconTheme                *icon_theme,
						    const gchar                 *theme_name);

gboolean      gtk_icon_theme_has_icon              (GtkIconTheme                *icon_theme,
						    const gchar                 *icon_name);
gint         *gtk_icon_theme_get_icon_sizes        (GtkIconTheme                *icon_theme,
						    const gchar                 *icon_name);
GtkIconInfo * gtk_icon_theme_lookup_icon           (GtkIconTheme                *icon_theme,
						    const gchar                 *icon_name,
						    gint                         size,
						    GtkIconLookupFlags           flags);
GtkIconInfo * gtk_icon_theme_choose_icon           (GtkIconTheme                *icon_theme,
						    const gchar                 *icon_names[],
						    gint                         size,
						    GtkIconLookupFlags           flags);
GdkPixbuf *   gtk_icon_theme_load_icon             (GtkIconTheme                *icon_theme,
						    const gchar                 *icon_name,
						    gint                         size,
						    GtkIconLookupFlags           flags,
						    GError                     **error);

GtkIconInfo * gtk_icon_theme_lookup_by_gicon       (GtkIconTheme                *icon_theme,
                                                    GIcon                       *icon,
                                                    gint                         size,
                                                    GtkIconLookupFlags           flags);

GList *       gtk_icon_theme_list_icons            (GtkIconTheme                *icon_theme,
						    const gchar                 *context);
GList *       gtk_icon_theme_list_contexts         (GtkIconTheme                *icon_theme);
char *        gtk_icon_theme_get_example_icon_name (GtkIconTheme                *icon_theme);

gboolean      gtk_icon_theme_rescan_if_needed      (GtkIconTheme                *icon_theme);

void          gtk_icon_theme_add_builtin_icon      (const gchar *icon_name,
					            gint         size,
					            GdkPixbuf   *pixbuf);

GType                 gtk_icon_info_get_type           (void) G_GNUC_CONST;
GtkIconInfo *         gtk_icon_info_copy               (GtkIconInfo  *icon_info);
void                  gtk_icon_info_free               (GtkIconInfo  *icon_info);

GtkIconInfo *         gtk_icon_info_new_for_pixbuf     (GtkIconTheme  *icon_theme,
                                                        GdkPixbuf     *pixbuf);

gint                  gtk_icon_info_get_base_size      (GtkIconInfo   *icon_info);
const gchar *         gtk_icon_info_get_filename       (GtkIconInfo   *icon_info);
GdkPixbuf *           gtk_icon_info_get_builtin_pixbuf (GtkIconInfo   *icon_info);
GdkPixbuf *           gtk_icon_info_load_icon          (GtkIconInfo   *icon_info,
							GError       **error);
void                  gtk_icon_info_set_raw_coordinates (GtkIconInfo  *icon_info,
							 gboolean      raw_coordinates);

gboolean              gtk_icon_info_get_embedded_rect (GtkIconInfo    *icon_info,
						       GdkRectangle   *rectangle);
gboolean              gtk_icon_info_get_attach_points (GtkIconInfo    *icon_info,
						       GdkPoint      **points,
						       gint           *n_points);
const gchar *         gtk_icon_info_get_display_name  (GtkIconInfo    *icon_info);

/* Non-public methods */
void _gtk_icon_theme_check_reload                     (GdkDisplay *display);
void _gtk_icon_theme_ensure_builtin_cache             (void);

G_END_DECLS

#endif /* __GTK_ICON_THEME_H__ */
