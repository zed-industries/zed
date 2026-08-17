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

#ifndef __GTK_RC_H__
#define __GTK_RC_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwidget.h>
#include <gtk/gtkbindings.h>

G_BEGIN_DECLS

/* Forward declarations */
typedef struct _GtkRcContext    GtkRcContext;
typedef struct _GtkRcStyleClass GtkRcStyleClass;

#define GTK_TYPE_RC_STYLE              (gtk_rc_style_get_type ())
#define GTK_RC_STYLE(object)           (G_TYPE_CHECK_INSTANCE_CAST ((object), GTK_TYPE_RC_STYLE, GtkRcStyle))
#define GTK_RC_STYLE_CLASS(klass)      (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_RC_STYLE, GtkRcStyleClass))
#define GTK_IS_RC_STYLE(object)        (G_TYPE_CHECK_INSTANCE_TYPE ((object), GTK_TYPE_RC_STYLE))
#define GTK_IS_RC_STYLE_CLASS(klass)   (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_RC_STYLE))
#define GTK_RC_STYLE_GET_CLASS(obj)    (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_RC_STYLE, GtkRcStyleClass))

/**
 * GtkRcFlags:
 * @GTK_RC_FG :Deprecated
 * @GTK_RC_BG: Deprecated
 * @GTK_RC_TEXT: Deprecated
 * @GTK_RC_BASE: Deprecated
 *
 * Deprecated
 */
typedef enum
{
  GTK_RC_FG             = 1 << 0,
  GTK_RC_BG             = 1 << 1,
  GTK_RC_TEXT           = 1 << 2,
  GTK_RC_BASE           = 1 << 3
} GtkRcFlags;

/**
 * GtkRcStyle:
 * @name: Name
 * @bg_pixmap_name: Pixmap name
 * @font_desc: A #PangoFontDescription
 * @color_flags: #GtkRcFlags
 * @fg: Foreground colors
 * @bg: Background colors
 * @text: Text colors
 * @base: Base colors
 * @xthickness: X thickness
 * @ythickness: Y thickness
 *
 * The #GtkRcStyle-struct is used to represent a set
 * of information about the appearance of a widget.
 * This can later be composited together with other
 * #GtkRcStyle-struct<!-- -->s to form a #GtkStyle.
 */
struct _GtkRcStyle
{
  GObject parent_instance;

  /*< public >*/

  gchar *name;
  gchar *bg_pixmap_name[5];
  PangoFontDescription *font_desc;

  GtkRcFlags color_flags[5];
  GdkColor   fg[5];
  GdkColor   bg[5];
  GdkColor   text[5];
  GdkColor   base[5];

  gint xthickness;
  gint ythickness;

  /*< private >*/
  GArray *rc_properties;

  /* list of RC style lists including this RC style */
  GSList *rc_style_lists;

  GSList *icon_factories;

  guint engine_specified : 1;   /* The RC file specified the engine */
};

/**
 * GtkRcStyleClass:
 * @parent_class: The parent class.
 * @create_rc_style: 
 * @parse: 
 * @merge: 
 * @create_style: 
 */
struct _GtkRcStyleClass
{
  GObjectClass parent_class;

  /*< public >*/

  /* Create an empty RC style of the same type as this RC style.
   * The default implementation, which does
   * g_object_new (G_OBJECT_TYPE (style), NULL);
   * should work in most cases.
   */
  GtkRcStyle * (*create_rc_style) (GtkRcStyle *rc_style);

  /* Fill in engine specific parts of GtkRcStyle by parsing contents
   * of brackets. Returns G_TOKEN_NONE if successful, otherwise returns
   * the token it expected but didn't get.
   */
  guint     (*parse)  (GtkRcStyle   *rc_style,
                       GtkSettings  *settings,
                       GScanner     *scanner);

  /* Combine RC style data from src into dest. If overridden, this
   * function should chain to the parent.
   */
  void      (*merge)  (GtkRcStyle *dest,
                       GtkRcStyle *src);

  /* Create an empty style suitable to this RC style
   */
  GtkStyle * (*create_style) (GtkRcStyle *rc_style);

  /*< private >*/

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

GSList*   _gtk_rc_parse_widget_class_path (const gchar *pattern);
void      _gtk_rc_free_widget_class_path (GSList       *list);
gboolean  _gtk_rc_match_widget_class     (GSList       *list,
                                          gint          length,
                                          gchar        *path,
                                          gchar        *path_reversed);

GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext)
void      gtk_rc_add_default_file       (const gchar *filename);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext)
void      gtk_rc_set_default_files      (gchar **filenames);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext)
gchar**   gtk_rc_get_default_files      (void);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext)
GtkStyle* gtk_rc_get_style              (GtkWidget   *widget);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext)
GtkStyle* gtk_rc_get_style_by_paths     (GtkSettings *settings,
                                         const char  *widget_path,
                                         const char  *class_path,
                                         GType        type);

GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext)
gboolean gtk_rc_reparse_all_for_settings (GtkSettings *settings,
                                          gboolean     force_load);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext)
void     gtk_rc_reset_styles             (GtkSettings *settings);

GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext)
gchar*   gtk_rc_find_pixmap_in_path (GtkSettings  *settings,
                                     GScanner     *scanner,
                                     const gchar  *pixmap_file);

GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext)
void     gtk_rc_parse                   (const gchar *filename);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext)
void      gtk_rc_parse_string           (const gchar *rc_string);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext)
gboolean  gtk_rc_reparse_all            (void);

GDK_DEPRECATED_IN_3_0
GType       gtk_rc_style_get_type   (void) G_GNUC_CONST;
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext)
GtkRcStyle* gtk_rc_style_new        (void);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext)
GtkRcStyle* gtk_rc_style_copy       (GtkRcStyle *orig);

GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext)
gchar*      gtk_rc_find_module_in_path (const gchar *module_file);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext)
gchar*      gtk_rc_get_theme_dir       (void);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext)
gchar*      gtk_rc_get_module_dir      (void);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext)
gchar*      gtk_rc_get_im_module_path  (void);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext)
gchar*      gtk_rc_get_im_module_file  (void);

/* private functions/definitions */

/**
 * GtkRcTokenType:
 * @GTK_RC_TOKEN_INVALID: Deprecated
 * @GTK_RC_TOKEN_INCLUDE: Deprecated
 * @GTK_RC_TOKEN_NORMAL: Deprecated
 * @GTK_RC_TOKEN_ACTIVE: Deprecated
 * @GTK_RC_TOKEN_PRELIGHT: Deprecated
 * @GTK_RC_TOKEN_SELECTED: Deprecated
 * @GTK_RC_TOKEN_INSENSITIVE: Deprecated
 * @GTK_RC_TOKEN_FG: Deprecated
 * @GTK_RC_TOKEN_BG: Deprecated
 * @GTK_RC_TOKEN_TEXT: Deprecated
 * @GTK_RC_TOKEN_BASE: Deprecated
 * @GTK_RC_TOKEN_XTHICKNESS: Deprecated
 * @GTK_RC_TOKEN_YTHICKNESS: Deprecated
 * @GTK_RC_TOKEN_FONT: Deprecated
 * @GTK_RC_TOKEN_FONTSET: Deprecated
 * @GTK_RC_TOKEN_FONT_NAME: Deprecated
 * @GTK_RC_TOKEN_BG_PIXMAP: Deprecated
 * @GTK_RC_TOKEN_PIXMAP_PATH: Deprecated
 * @GTK_RC_TOKEN_STYLE: Deprecated
 * @GTK_RC_TOKEN_BINDING: Deprecated
 * @GTK_RC_TOKEN_BIND: Deprecated
 * @GTK_RC_TOKEN_WIDGET: Deprecated
 * @GTK_RC_TOKEN_WIDGET_CLASS: Deprecated
 * @GTK_RC_TOKEN_CLASS: Deprecated
 * @GTK_RC_TOKEN_LOWEST: Deprecated
 * @GTK_RC_TOKEN_GTK: Deprecated
 * @GTK_RC_TOKEN_APPLICATION: Deprecated
 * @GTK_RC_TOKEN_THEME: Deprecated
 * @GTK_RC_TOKEN_RC: Deprecated
 * @GTK_RC_TOKEN_HIGHEST: Deprecated
 * @GTK_RC_TOKEN_ENGINE: Deprecated
 * @GTK_RC_TOKEN_MODULE_PATH: Deprecated
 * @GTK_RC_TOKEN_IM_MODULE_PATH: Deprecated
 * @GTK_RC_TOKEN_IM_MODULE_FILE: Deprecated
 * @GTK_RC_TOKEN_STOCK: Deprecated
 * @GTK_RC_TOKEN_LTR: Deprecated
 * @GTK_RC_TOKEN_RTL: Deprecated
 * @GTK_RC_TOKEN_COLOR: Deprecated
 * @GTK_RC_TOKEN_UNBIND: Deprecated
 * @GTK_RC_TOKEN_LAST: Deprecated
 *
 * The #GtkRcTokenType enumeration represents the tokens
 * in the RC file. It is exposed so that theme engines
 * can reuse these tokens when parsing the theme-engine
 * specific portions of a RC file.
 *
 * Deprecated: 3.0: Use #GtkCssProvider instead.
 */
typedef enum {
  GTK_RC_TOKEN_INVALID = G_TOKEN_LAST,
  GTK_RC_TOKEN_INCLUDE,
  GTK_RC_TOKEN_NORMAL,
  GTK_RC_TOKEN_ACTIVE,
  GTK_RC_TOKEN_PRELIGHT,
  GTK_RC_TOKEN_SELECTED,
  GTK_RC_TOKEN_INSENSITIVE,
  GTK_RC_TOKEN_FG,
  GTK_RC_TOKEN_BG,
  GTK_RC_TOKEN_TEXT,
  GTK_RC_TOKEN_BASE,
  GTK_RC_TOKEN_XTHICKNESS,
  GTK_RC_TOKEN_YTHICKNESS,
  GTK_RC_TOKEN_FONT,
  GTK_RC_TOKEN_FONTSET,
  GTK_RC_TOKEN_FONT_NAME,
  GTK_RC_TOKEN_BG_PIXMAP,
  GTK_RC_TOKEN_PIXMAP_PATH,
  GTK_RC_TOKEN_STYLE,
  GTK_RC_TOKEN_BINDING,
  GTK_RC_TOKEN_BIND,
  GTK_RC_TOKEN_WIDGET,
  GTK_RC_TOKEN_WIDGET_CLASS,
  GTK_RC_TOKEN_CLASS,
  GTK_RC_TOKEN_LOWEST,
  GTK_RC_TOKEN_GTK,
  GTK_RC_TOKEN_APPLICATION,
  GTK_RC_TOKEN_THEME,
  GTK_RC_TOKEN_RC,
  GTK_RC_TOKEN_HIGHEST,
  GTK_RC_TOKEN_ENGINE,
  GTK_RC_TOKEN_MODULE_PATH,
  GTK_RC_TOKEN_IM_MODULE_PATH,
  GTK_RC_TOKEN_IM_MODULE_FILE,
  GTK_RC_TOKEN_STOCK,
  GTK_RC_TOKEN_LTR,
  GTK_RC_TOKEN_RTL,
  GTK_RC_TOKEN_COLOR,
  GTK_RC_TOKEN_UNBIND,
  GTK_RC_TOKEN_LAST
} GtkRcTokenType;


/**
 * GtkPathPriorityType:
 * @GTK_PATH_PRIO_LOWEST: Deprecated
 * @GTK_PATH_PRIO_GTK: Deprecated
 * @GTK_PATH_PRIO_APPLICATION: Deprecated
 * @GTK_PATH_PRIO_THEME: Deprecated
 * @GTK_PATH_PRIO_RC: Deprecated
 * @GTK_PATH_PRIO_HIGHEST: Deprecated
 *
 * Priorities for path lookups.
 * See also gtk_binding_set_add_path().
 *
 * Deprecated: 3.0
 */
typedef enum
{
  GTK_PATH_PRIO_LOWEST      = 0,
  GTK_PATH_PRIO_GTK         = 4,
  GTK_PATH_PRIO_APPLICATION = 8,
  GTK_PATH_PRIO_THEME       = 10,
  GTK_PATH_PRIO_RC          = 12,
  GTK_PATH_PRIO_HIGHEST     = 15
} GtkPathPriorityType;
#define GTK_PATH_PRIO_MASK 0x0f

/**
 * GtkPathType:
 * @GTK_PATH_WIDGET: Deprecated
 * @GTK_PATH_WIDGET_CLASS: Deprecated
 * @GTK_PATH_CLASS: Deprecated
 *
 * Widget path types.
 * See also gtk_binding_set_add_path().
 *
 * Deprecated: 3.0
 */
typedef enum
{
  GTK_PATH_WIDGET,
  GTK_PATH_WIDGET_CLASS,
  GTK_PATH_CLASS
} GtkPathType;

GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext)
GScanner* gtk_rc_scanner_new    (void);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext)
guint     gtk_rc_parse_color    (GScanner            *scanner,
                                 GdkColor            *color);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext)
guint     gtk_rc_parse_color_full (GScanner          *scanner,
                                   GtkRcStyle        *style,
                                   GdkColor          *color);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext)
guint     gtk_rc_parse_state    (GScanner            *scanner,
                                 GtkStateType        *state);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext)
guint     gtk_rc_parse_priority (GScanner            *scanner,
                                 GtkPathPriorityType *priority);

/* rc properties
 * (structure forward declared in gtkstyle.h)
 */
/**
 * GtkRcProperty:
 * @type_name: quark-ified type identifier
 * @property_name: quark-ified property identifier like
 *   “GtkScrollbar::spacing”
 * @origin: field similar to one found in #GtkSettingsValue
 * @value:field similar to one found in #GtkSettingsValue
 *
 * Deprecated
 */
struct _GtkRcProperty
{
  /* quark-ified property identifier like “GtkScrollbar::spacing” */
  GQuark type_name;
  GQuark property_name;

  /* fields similar to GtkSettingsValue */
  gchar *origin;
  GValue value;
};

GDK_DEPRECATED_IN_3_0
void      gtk_binding_set_add_path (GtkBindingSet       *binding_set,
                                    GtkPathType          path_type,
                                    const gchar         *path_pattern,
                                    GtkPathPriorityType  priority);

G_END_DECLS

#endif /* __GTK_RC_H__ */
