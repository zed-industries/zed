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

#ifndef __GTK_STYLE_PROVIDER_H__
#define __GTK_STYLE_PROVIDER_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <glib-object.h>
#include <gtk/gtkenums.h>
#include <gtk/deprecated/gtkiconfactory.h>
#include <gtk/deprecated/gtkstyleproperties.h>
#include <gtk/gtktypes.h>

G_BEGIN_DECLS

#define GTK_TYPE_STYLE_PROVIDER          (gtk_style_provider_get_type ())
#define GTK_STYLE_PROVIDER(o)            (G_TYPE_CHECK_INSTANCE_CAST ((o), GTK_TYPE_STYLE_PROVIDER, GtkStyleProvider))
#define GTK_IS_STYLE_PROVIDER(o)         (G_TYPE_CHECK_INSTANCE_TYPE ((o), GTK_TYPE_STYLE_PROVIDER))
#define GTK_STYLE_PROVIDER_GET_IFACE(o)  (G_TYPE_INSTANCE_GET_INTERFACE ((o), GTK_TYPE_STYLE_PROVIDER, GtkStyleProviderIface))

/**
 * GTK_STYLE_PROVIDER_PRIORITY_FALLBACK:
 *
 * The priority used for default style information
 * that is used in the absence of themes.
 *
 * Note that this is not very useful for providing default
 * styling for custom style classes - themes are likely to
 * override styling provided at this priority with
 * catch-all `* {...}` rules.
 */
#define GTK_STYLE_PROVIDER_PRIORITY_FALLBACK      1

/**
 * GTK_STYLE_PROVIDER_PRIORITY_THEME:
 *
 * The priority used for style information provided
 * by themes.
 */
#define GTK_STYLE_PROVIDER_PRIORITY_THEME     200

/**
 * GTK_STYLE_PROVIDER_PRIORITY_SETTINGS:
 *
 * The priority used for style information provided
 * via #GtkSettings.
 *
 * This priority is higher than #GTK_STYLE_PROVIDER_PRIORITY_THEME
 * to let settings override themes.
 */
#define GTK_STYLE_PROVIDER_PRIORITY_SETTINGS    400

/**
 * GTK_STYLE_PROVIDER_PRIORITY_APPLICATION:
 *
 * A priority that can be used when adding a #GtkStyleProvider
 * for application-specific style information.
 */
#define GTK_STYLE_PROVIDER_PRIORITY_APPLICATION 600

/**
 * GTK_STYLE_PROVIDER_PRIORITY_USER:
 *
 * The priority used for the style information from
 * `XDG_CONFIG_HOME/gtk-3.0/gtk.css`.
 *
 * You should not use priorities higher than this, to
 * give the user the last word.
 */
#define GTK_STYLE_PROVIDER_PRIORITY_USER        800

typedef struct _GtkStyleProviderIface GtkStyleProviderIface;
typedef struct _GtkStyleProvider GtkStyleProvider; /* dummy typedef */

/**
 * GtkStyleProviderIface:
 * @get_style: Gets a set of style information that applies to a widget path.
 * @get_style_property: Gets the value of a widget style property that applies to a widget path.
 * @get_icon_factory: Gets the icon factory that applies to a widget path.
 */
struct _GtkStyleProviderIface
{
  /*< private >*/
  GTypeInterface g_iface;

  /*< public >*/

  GtkStyleProperties * (* get_style) (GtkStyleProvider *provider,
                                      GtkWidgetPath    *path);

  gboolean (* get_style_property) (GtkStyleProvider *provider,
                                   GtkWidgetPath    *path,
                                   GtkStateFlags     state,
                                   GParamSpec       *pspec,
                                   GValue           *value);

  GtkIconFactory * (* get_icon_factory) (GtkStyleProvider *provider,
					 GtkWidgetPath    *path);
};

GDK_AVAILABLE_IN_ALL
GType gtk_style_provider_get_type (void) G_GNUC_CONST;

GDK_DEPRECATED_IN_3_8
GtkStyleProperties *gtk_style_provider_get_style (GtkStyleProvider *provider,
                                                  GtkWidgetPath    *path);

GDK_AVAILABLE_IN_ALL
gboolean gtk_style_provider_get_style_property (GtkStyleProvider *provider,
                                                GtkWidgetPath    *path,
                                                GtkStateFlags     state,
                                                GParamSpec       *pspec,
                                                GValue           *value);

GDK_DEPRECATED_IN_3_8_FOR(NULL)
GtkIconFactory * gtk_style_provider_get_icon_factory (GtkStyleProvider *provider,
						      GtkWidgetPath    *path);

G_END_DECLS

#endif /* __GTK_STYLE_PROVIDER_H__ */
