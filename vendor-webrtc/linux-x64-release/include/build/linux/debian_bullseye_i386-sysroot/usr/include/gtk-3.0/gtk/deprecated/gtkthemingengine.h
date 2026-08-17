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

#ifndef __GTK_THEMING_ENGINE_H__
#define __GTK_THEMING_ENGINE_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <glib-object.h>
#include <cairo.h>

#include <gtk/gtkborder.h>
#include <gtk/gtkenums.h>
#include <gtk/deprecated/gtkstyleproperties.h>
#include <gtk/gtktypes.h>

G_BEGIN_DECLS

#define GTK_TYPE_THEMING_ENGINE         (gtk_theming_engine_get_type ())
#define GTK_THEMING_ENGINE(o)           (G_TYPE_CHECK_INSTANCE_CAST ((o), GTK_TYPE_THEMING_ENGINE, GtkThemingEngine))
#define GTK_THEMING_ENGINE_CLASS(c)     (G_TYPE_CHECK_CLASS_CAST    ((c), GTK_TYPE_THEMING_ENGINE, GtkThemingEngineClass))
#define GTK_IS_THEMING_ENGINE(o)        (G_TYPE_CHECK_INSTANCE_TYPE ((o), GTK_TYPE_THEMING_ENGINE))
#define GTK_IS_THEMING_ENGINE_CLASS(c)  (G_TYPE_CHECK_CLASS_TYPE    ((c), GTK_TYPE_THEMING_ENGINE))
#define GTK_THEMING_ENGINE_GET_CLASS(o) (G_TYPE_INSTANCE_GET_CLASS  ((o), GTK_TYPE_THEMING_ENGINE, GtkThemingEngineClass))

typedef struct _GtkThemingEngine GtkThemingEngine;
typedef struct GtkThemingEnginePrivate GtkThemingEnginePrivate;
typedef struct _GtkThemingEngineClass GtkThemingEngineClass;

struct _GtkThemingEngine
{
  GObject parent_object;
  GtkThemingEnginePrivate *priv;
};

/**
 * GtkThemingEngineClass:
 * @parent_class: The parent class.
 * @render_line: Renders a line between two points.
 * @render_background: Renders the background area of a widget region.
 * @render_frame: Renders the frame around a widget area.
 * @render_frame_gap: Renders the frame around a widget area with a gap in it.
 * @render_extension: Renders a extension to a box, usually a notebook tab.
 * @render_check: Renders a checkmark, as in #GtkCheckButton.
 * @render_option: Renders an option, as in #GtkRadioButton.
 * @render_arrow: Renders an arrow pointing to a certain direction.
 * @render_expander: Renders an element what will expose/expand part of
 *                   the UI, as in #GtkExpander.
 * @render_focus: Renders the focus indicator.
 * @render_layout: Renders a #PangoLayout
 * @render_slider: Renders a slider control, as in #GtkScale.
 * @render_handle: Renders a handle to drag UI elements, as in #GtkPaned.
 * @render_activity: Renders an area displaying activity, such as in #GtkSpinner,
 *                   or #GtkProgressBar.
 * @render_icon_pixbuf: Renders an icon as a #GdkPixbuf.
 * @render_icon: Renders an icon given as a #GdkPixbuf.
 * @render_icon_surface: Renders an icon given as a #cairo_surface_t.
 *
 * Base class for theming engines.
 */
struct _GtkThemingEngineClass
{
  GObjectClass parent_class;

  /*< public >*/

  void (* render_line) (GtkThemingEngine *engine,
                        cairo_t          *cr,
                        gdouble           x0,
                        gdouble           y0,
                        gdouble           x1,
                        gdouble           y1);
  void (* render_background) (GtkThemingEngine *engine,
                              cairo_t          *cr,
                              gdouble           x,
                              gdouble           y,
                              gdouble           width,
                              gdouble           height);
  void (* render_frame) (GtkThemingEngine *engine,
                         cairo_t          *cr,
                         gdouble           x,
                         gdouble           y,
                         gdouble           width,
                         gdouble           height);
  void (* render_frame_gap) (GtkThemingEngine *engine,
                             cairo_t          *cr,
                             gdouble           x,
                             gdouble           y,
                             gdouble           width,
                             gdouble           height,
                             GtkPositionType   gap_side,
                             gdouble           xy0_gap,
                             gdouble           xy1_gap);
  void (* render_extension) (GtkThemingEngine *engine,
                             cairo_t          *cr,
                             gdouble           x,
                             gdouble           y,
                             gdouble           width,
                             gdouble           height,
                             GtkPositionType   gap_side);
  void (* render_check) (GtkThemingEngine *engine,
                         cairo_t          *cr,
                         gdouble           x,
                         gdouble           y,
                         gdouble           width,
                         gdouble           height);
  void (* render_option) (GtkThemingEngine *engine,
                          cairo_t          *cr,
                          gdouble           x,
                          gdouble           y,
                          gdouble           width,
                          gdouble           height);
  void (* render_arrow) (GtkThemingEngine *engine,
                         cairo_t          *cr,
                         gdouble           angle,
                         gdouble           x,
                         gdouble           y,
                         gdouble           size);
  void (* render_expander) (GtkThemingEngine *engine,
                            cairo_t          *cr,
                            gdouble           x,
                            gdouble           y,
                            gdouble           width,
                            gdouble           height);
  void (* render_focus) (GtkThemingEngine *engine,
                         cairo_t          *cr,
                         gdouble           x,
                         gdouble           y,
                         gdouble           width,
                         gdouble           height);
  void (* render_layout) (GtkThemingEngine *engine,
                          cairo_t          *cr,
                          gdouble           x,
                          gdouble           y,
                          PangoLayout      *layout);
  void (* render_slider) (GtkThemingEngine *engine,
                          cairo_t          *cr,
                          gdouble           x,
                          gdouble           y,
                          gdouble           width,
                          gdouble           height,
                          GtkOrientation    orientation);
  void (* render_handle)    (GtkThemingEngine *engine,
                             cairo_t          *cr,
                             gdouble           x,
                             gdouble           y,
                             gdouble           width,
                             gdouble           height);
  void (* render_activity) (GtkThemingEngine *engine,
                            cairo_t          *cr,
                            gdouble           x,
                            gdouble           y,
                            gdouble           width,
                            gdouble           height);

  GdkPixbuf * (* render_icon_pixbuf) (GtkThemingEngine    *engine,
                                      const GtkIconSource *source,
                                      GtkIconSize          size);
  void (* render_icon) (GtkThemingEngine *engine,
                        cairo_t          *cr,
			GdkPixbuf        *pixbuf,
                        gdouble           x,
                        gdouble           y);
  void (* render_icon_surface) (GtkThemingEngine *engine,
				cairo_t          *cr,
				cairo_surface_t  *surface,
				gdouble           x,
				gdouble           y);

  /*< private >*/
  gpointer padding[14];
};

GDK_DEPRECATED_IN_3_14
GType gtk_theming_engine_get_type (void) G_GNUC_CONST;

/* function implemented in gtkcsscustomproperty.c */
GDK_DEPRECATED_IN_3_8
void gtk_theming_engine_register_property (const gchar            *name_space,
                                           GtkStylePropertyParser  parse_func,
                                           GParamSpec             *pspec);

GDK_DEPRECATED_IN_3_14
void gtk_theming_engine_get_property (GtkThemingEngine *engine,
                                      const gchar      *property,
                                      GtkStateFlags     state,
                                      GValue           *value);
GDK_DEPRECATED_IN_3_14
void gtk_theming_engine_get_valist   (GtkThemingEngine *engine,
                                      GtkStateFlags     state,
                                      va_list           args);
GDK_DEPRECATED_IN_3_14
void gtk_theming_engine_get          (GtkThemingEngine *engine,
                                      GtkStateFlags     state,
                                      ...) G_GNUC_NULL_TERMINATED;

GDK_DEPRECATED_IN_3_14
void gtk_theming_engine_get_style_property (GtkThemingEngine *engine,
                                            const gchar      *property_name,
                                            GValue           *value);
GDK_DEPRECATED_IN_3_14
void gtk_theming_engine_get_style_valist   (GtkThemingEngine *engine,
                                            va_list           args);
GDK_DEPRECATED_IN_3_14
void gtk_theming_engine_get_style          (GtkThemingEngine *engine,
                                            ...);

GDK_DEPRECATED_IN_3_14
gboolean gtk_theming_engine_lookup_color (GtkThemingEngine *engine,
                                          const gchar      *color_name,
                                          GdkRGBA          *color);

GDK_DEPRECATED_IN_3_14
const GtkWidgetPath * gtk_theming_engine_get_path (GtkThemingEngine *engine);

GDK_DEPRECATED_IN_3_14
gboolean gtk_theming_engine_has_class  (GtkThemingEngine *engine,
                                        const gchar      *style_class);
GDK_DEPRECATED_IN_3_14
gboolean gtk_theming_engine_has_region (GtkThemingEngine *engine,
                                        const gchar      *style_region,
                                        GtkRegionFlags   *flags);

GDK_DEPRECATED_IN_3_14
GtkStateFlags gtk_theming_engine_get_state        (GtkThemingEngine *engine);
GDK_DEPRECATED_IN_3_6
gboolean      gtk_theming_engine_state_is_running (GtkThemingEngine *engine,
                                                   GtkStateType      state,
                                                   gdouble          *progress);

GDK_DEPRECATED_IN_3_8_FOR(gtk_theming_engine_get_state)
GtkTextDirection gtk_theming_engine_get_direction (GtkThemingEngine *engine);

GDK_DEPRECATED_IN_3_14
GtkJunctionSides gtk_theming_engine_get_junction_sides (GtkThemingEngine *engine);

/* Helper functions */
GDK_DEPRECATED_IN_3_14
void gtk_theming_engine_get_color            (GtkThemingEngine *engine,
                                              GtkStateFlags     state,
                                              GdkRGBA          *color);
GDK_DEPRECATED_IN_3_14
void gtk_theming_engine_get_background_color (GtkThemingEngine *engine,
                                              GtkStateFlags     state,
                                              GdkRGBA          *color);
GDK_DEPRECATED_IN_3_14
void gtk_theming_engine_get_border_color     (GtkThemingEngine *engine,
                                              GtkStateFlags     state,
                                              GdkRGBA          *color);

GDK_DEPRECATED_IN_3_14
void gtk_theming_engine_get_border  (GtkThemingEngine *engine,
                                     GtkStateFlags     state,
                                     GtkBorder        *border);
GDK_DEPRECATED_IN_3_14
void gtk_theming_engine_get_padding (GtkThemingEngine *engine,
                                     GtkStateFlags     state,
                                     GtkBorder        *padding);
GDK_DEPRECATED_IN_3_14
void gtk_theming_engine_get_margin  (GtkThemingEngine *engine,
                                     GtkStateFlags     state,
                                     GtkBorder        *margin);

GDK_DEPRECATED_IN_3_8_FOR(gtk_theming_engine_get)
const PangoFontDescription * gtk_theming_engine_get_font (GtkThemingEngine *engine,
                                                          GtkStateFlags     state);

GDK_DEPRECATED_IN_3_14
GtkThemingEngine * gtk_theming_engine_load (const gchar *name);

GDK_DEPRECATED_IN_3_14
GdkScreen * gtk_theming_engine_get_screen (GtkThemingEngine *engine);

G_END_DECLS

#endif /* __GTK_THEMING_ENGINE_H__ */
