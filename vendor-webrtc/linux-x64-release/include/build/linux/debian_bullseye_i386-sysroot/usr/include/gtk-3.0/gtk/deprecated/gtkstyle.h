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

#ifndef __GTK_STYLE_H__
#define __GTK_STYLE_H__


#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gdk/gdk.h>
#include <gtk/gtkenums.h>
#include <gtk/gtktypes.h>


G_BEGIN_DECLS

#define GTK_TYPE_STYLE              (gtk_style_get_type ())
#define GTK_STYLE(object)           (G_TYPE_CHECK_INSTANCE_CAST ((object), GTK_TYPE_STYLE, GtkStyle))
#define GTK_STYLE_CLASS(klass)      (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_STYLE, GtkStyleClass))
#define GTK_IS_STYLE(object)        (G_TYPE_CHECK_INSTANCE_TYPE ((object), GTK_TYPE_STYLE))
#define GTK_IS_STYLE_CLASS(klass)   (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_STYLE))
#define GTK_STYLE_GET_CLASS(obj)    (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_STYLE, GtkStyleClass))

/* Some forward declarations needed to rationalize the header
 * files.
 */
typedef struct _GtkStyleClass  GtkStyleClass;
typedef struct _GtkThemeEngine GtkThemeEngine;
typedef struct _GtkRcProperty  GtkRcProperty;

/**
 * GtkExpanderStyle:
 * @GTK_EXPANDER_COLLAPSED: The style used for a collapsed subtree.
 * @GTK_EXPANDER_SEMI_COLLAPSED: Intermediate style used during animation.
 * @GTK_EXPANDER_SEMI_EXPANDED: Intermediate style used during animation.
 * @GTK_EXPANDER_EXPANDED: The style used for an expanded subtree.
 *
 * Used to specify the style of the expanders drawn by a #GtkTreeView.
 */
typedef enum
{
  GTK_EXPANDER_COLLAPSED,
  GTK_EXPANDER_SEMI_COLLAPSED,
  GTK_EXPANDER_SEMI_EXPANDED,
  GTK_EXPANDER_EXPANDED
} GtkExpanderStyle;

/**
 * GTK_STYLE_ATTACHED:
 * @style: a #GtkStyle.
 *
 * Returns: whether the style is attached to a window.
 */
#define GTK_STYLE_ATTACHED(style)       (GTK_STYLE (style)->attach_count > 0)

/**
 * GtkStyle:
 * @fg: Set of foreground #GdkColor
 * @bg: Set of background #GdkColor
 * @light: Set of light #GdkColor
 * @dark: Set of dark #GdkColor
 * @mid: Set of mid #GdkColor
 * @text: Set of text #GdkColor
 * @base: Set of base #GdkColor
 * @text_aa: Color halfway between text/base
 * @black: #GdkColor to use for black
 * @white: #GdkColor to use for white
 * @font_desc: #PangoFontDescription
 * @xthickness: Thickness in X direction
 * @ythickness: Thickness in Y direction
 * @background: Set of background #cairo_pattern_t
 */
struct _GtkStyle
{
  /*< private >*/
  GObject parent_instance;

  /*< public >*/

  GdkColor fg[5];
  GdkColor bg[5];
  GdkColor light[5];
  GdkColor dark[5];
  GdkColor mid[5];
  GdkColor text[5];
  GdkColor base[5];
  GdkColor text_aa[5];          /* Halfway between text/base */

  GdkColor black;
  GdkColor white;
  PangoFontDescription *font_desc;

  gint xthickness;
  gint ythickness;

  cairo_pattern_t *background[5];

  /*< private >*/

  gint attach_count;

  GdkVisual *visual;
  PangoFontDescription *private_font_desc; /* Font description for style->private_font or %NULL */

  /* the RcStyle from which this style was created */
  GtkRcStyle     *rc_style;

  GSList         *styles;         /* of type GtkStyle* */
  GArray         *property_cache;
  GSList         *icon_factories; /* of type GtkIconFactory* */
};

/**
 * GtkStyleClass:
 * @parent_class: The parent class.
 * @realize: 
 * @unrealize: 
 * @copy: 
 * @clone: 
 * @init_from_rc: 
 * @set_background: 
 * @render_icon: 
 * @draw_hline: 
 * @draw_vline: 
 * @draw_shadow: 
 * @draw_arrow: 
 * @draw_diamond: 
 * @draw_box: 
 * @draw_flat_box: 
 * @draw_check: 
 * @draw_option: 
 * @draw_tab: 
 * @draw_shadow_gap: 
 * @draw_box_gap: 
 * @draw_extension: 
 * @draw_focus: 
 * @draw_slider: 
 * @draw_handle: 
 * @draw_expander: 
 * @draw_layout: 
 * @draw_resize_grip: 
 * @draw_spinner: 
 */
struct _GtkStyleClass
{
  GObjectClass parent_class;

  /*< public >*/

  /* Initialize for a particular visual. style->visual
   * will have been set at this point. Will typically chain
   * to parent.
   */
  void (*realize)               (GtkStyle               *style);

  /* Clean up for a particular visual. Will typically chain
   * to parent.
   */
  void (*unrealize)             (GtkStyle               *style);

  /* Make style an exact duplicate of src.
   */
  void (*copy)                  (GtkStyle               *style,
                                 GtkStyle               *src);

  /* Create an empty style of the same type as this style.
   * The default implementation, which does
   * g_object_new (G_OBJECT_TYPE (style), NULL);
   * should work in most cases.
   */
  GtkStyle *(*clone)             (GtkStyle               *style);

  /* Initialize the GtkStyle with the values in the GtkRcStyle.
   * should chain to the parent implementation.
   */
  void     (*init_from_rc)      (GtkStyle               *style,
                                 GtkRcStyle             *rc_style);

  void (*set_background)        (GtkStyle               *style,
                                 GdkWindow              *window,
                                 GtkStateType            state_type);


  GdkPixbuf * (* render_icon)   (GtkStyle               *style,
                                 const GtkIconSource    *source,
                                 GtkTextDirection        direction,
                                 GtkStateType            state,
                                 GtkIconSize             size,
                                 GtkWidget              *widget,
                                 const gchar            *detail);

  /* Drawing functions
   */

  void (*draw_hline)            (GtkStyle               *style,
                                 cairo_t                *cr,
                                 GtkStateType            state_type,
                                 GtkWidget              *widget,
                                 const gchar            *detail,
                                 gint                    x1,
                                 gint                    x2,
                                 gint                    y);
  void (*draw_vline)            (GtkStyle               *style,
                                 cairo_t                *cr,
                                 GtkStateType            state_type,
                                 GtkWidget              *widget,
                                 const gchar            *detail,
                                 gint                    y1_,
                                 gint                    y2_,
                                 gint                    x);
  void (*draw_shadow)           (GtkStyle               *style,
                                 cairo_t                *cr,
                                 GtkStateType            state_type,
                                 GtkShadowType           shadow_type,
                                 GtkWidget              *widget,
                                 const gchar            *detail,
                                 gint                    x,
                                 gint                    y,
                                 gint                    width,
                                 gint                    height);
  void (*draw_arrow)            (GtkStyle               *style,
                                 cairo_t                *cr,
                                 GtkStateType            state_type,
                                 GtkShadowType           shadow_type,
                                 GtkWidget              *widget,
                                 const gchar            *detail,
                                 GtkArrowType            arrow_type,
                                 gboolean                fill,
                                 gint                    x,
                                 gint                    y,
                                 gint                    width,
                                 gint                    height);
  void (*draw_diamond)          (GtkStyle               *style,
                                 cairo_t                *cr,
                                 GtkStateType            state_type,
                                 GtkShadowType           shadow_type,
                                 GtkWidget              *widget,
                                 const gchar            *detail,
                                 gint                    x,
                                 gint                    y,
                                 gint                    width,
                                 gint                    height);
  void (*draw_box)              (GtkStyle               *style,
                                 cairo_t                *cr,
                                 GtkStateType            state_type,
                                 GtkShadowType           shadow_type,
                                 GtkWidget              *widget,
                                 const gchar            *detail,
                                 gint                    x,
                                 gint                    y,
                                 gint                    width,
                                 gint                    height);
  void (*draw_flat_box)         (GtkStyle               *style,
                                 cairo_t                *cr,
                                 GtkStateType            state_type,
                                 GtkShadowType           shadow_type,
                                 GtkWidget              *widget,
                                 const gchar            *detail,
                                 gint                    x,
                                 gint                    y,
                                 gint                    width,
                                 gint                    height);
  void (*draw_check)            (GtkStyle               *style,
                                 cairo_t                *cr,
                                 GtkStateType            state_type,
                                 GtkShadowType           shadow_type,
                                 GtkWidget              *widget,
                                 const gchar            *detail,
                                 gint                    x,
                                 gint                    y,
                                 gint                    width,
                                 gint                    height);
  void (*draw_option)           (GtkStyle               *style,
                                 cairo_t                *cr,
                                 GtkStateType            state_type,
                                 GtkShadowType           shadow_type,
                                 GtkWidget              *widget,
                                 const gchar            *detail,
                                 gint                    x,
                                 gint                    y,
                                 gint                    width,
                                 gint                    height);
  void (*draw_tab)              (GtkStyle               *style,
                                 cairo_t                *cr,
                                 GtkStateType            state_type,
                                 GtkShadowType           shadow_type,
                                 GtkWidget              *widget,
                                 const gchar            *detail,
                                 gint                    x,
                                 gint                    y,
                                 gint                    width,
                                 gint                    height);
  void (*draw_shadow_gap)       (GtkStyle               *style,
                                 cairo_t                *cr,
                                 GtkStateType            state_type,
                                 GtkShadowType           shadow_type,
                                 GtkWidget              *widget,
                                 const gchar            *detail,
                                 gint                    x,
                                 gint                    y,
                                 gint                    width,
                                 gint                    height,
                                 GtkPositionType         gap_side,
                                 gint                    gap_x,
                                 gint                    gap_width);
  void (*draw_box_gap)          (GtkStyle               *style,
                                 cairo_t                *cr,
                                 GtkStateType            state_type,
                                 GtkShadowType           shadow_type,
                                 GtkWidget              *widget,
                                 const gchar            *detail,
                                 gint                    x,
                                 gint                    y,
                                 gint                    width,
                                 gint                    height,
                                 GtkPositionType         gap_side,
                                 gint                    gap_x,
                                 gint                    gap_width);
  void (*draw_extension)        (GtkStyle               *style,
                                 cairo_t                *cr,
                                 GtkStateType            state_type,
                                 GtkShadowType           shadow_type,
                                 GtkWidget              *widget,
                                 const gchar            *detail,
                                 gint                    x,
                                 gint                    y,
                                 gint                    width,
                                 gint                    height,
                                 GtkPositionType         gap_side);
  void (*draw_focus)            (GtkStyle               *style,
                                 cairo_t                *cr,
                                 GtkStateType            state_type,
                                 GtkWidget              *widget,
                                 const gchar            *detail,
                                 gint                    x,
                                 gint                    y,
                                 gint                    width,
                                 gint                    height);
  void (*draw_slider)           (GtkStyle               *style,
                                 cairo_t                *cr,
                                 GtkStateType            state_type,
                                 GtkShadowType           shadow_type,
                                 GtkWidget              *widget,
                                 const gchar            *detail,
                                 gint                    x,
                                 gint                    y,
                                 gint                    width,
                                 gint                    height,
                                 GtkOrientation          orientation);
  void (*draw_handle)           (GtkStyle               *style,
                                 cairo_t                *cr,
                                 GtkStateType            state_type,
                                 GtkShadowType           shadow_type,
                                 GtkWidget              *widget,
                                 const gchar            *detail,
                                 gint                    x,
                                 gint                    y,
                                 gint                    width,
                                 gint                    height,
                                 GtkOrientation          orientation);

  void (*draw_expander)         (GtkStyle               *style,
                                 cairo_t                *cr,
                                 GtkStateType            state_type,
                                 GtkWidget              *widget,
                                 const gchar            *detail,
                                 gint                    x,
                                 gint                    y,
                                 GtkExpanderStyle        expander_style);
  void (*draw_layout)           (GtkStyle               *style,
                                 cairo_t                *cr,
                                 GtkStateType            state_type,
                                 gboolean                use_text,
                                 GtkWidget              *widget,
                                 const gchar            *detail,
                                 gint                    x,
                                 gint                    y,
                                 PangoLayout            *layout);
  void (*draw_resize_grip)      (GtkStyle               *style,
                                 cairo_t                *cr,
                                 GtkStateType            state_type,
                                 GtkWidget              *widget,
                                 const gchar            *detail,
                                 GdkWindowEdge           edge,
                                 gint                    x,
                                 gint                    y,
                                 gint                    width,
                                 gint                    height);
  void (*draw_spinner)          (GtkStyle               *style,
                                 cairo_t                *cr,
                                 GtkStateType            state_type,
                                 GtkWidget              *widget,
                                 const gchar            *detail,
                                 guint                   step,
                                 gint                    x,
                                 gint                    y,
                                 gint                    width,
                                 gint                    height);

  /*< private >*/

  /* Padding for future expansion */
  void (*_gtk_reserved1)  (void);
  void (*_gtk_reserved2)  (void);
  void (*_gtk_reserved3)  (void);
  void (*_gtk_reserved4)  (void);
  void (*_gtk_reserved5)  (void);
  void (*_gtk_reserved6)  (void);
  void (*_gtk_reserved7)  (void);
  void (*_gtk_reserved8)  (void);
  void (*_gtk_reserved9)  (void);
  void (*_gtk_reserved10) (void);
  void (*_gtk_reserved11) (void);
};

GDK_DEPRECATED_IN_3_0
GType     gtk_style_get_type                 (void) G_GNUC_CONST;
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext)
GtkStyle* gtk_style_new                      (void);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext)
GtkStyle* gtk_style_copy                     (GtkStyle     *style);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext)
GtkStyle* gtk_style_attach                   (GtkStyle     *style,
                                              GdkWindow    *window);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext)
void      gtk_style_detach                   (GtkStyle     *style);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext)
void      gtk_style_set_background           (GtkStyle     *style,
                                              GdkWindow    *window,
                                              GtkStateType  state_type);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext and gtk_render_background)
void      gtk_style_apply_default_background (GtkStyle     *style,
                                              cairo_t      *cr,
                                              GdkWindow    *window,
                                              GtkStateType  state_type,
                                              gint          x,
                                              gint          y,
                                              gint          width,
                                              gint          height);

GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext and a style class)
GtkIconSet* gtk_style_lookup_icon_set        (GtkStyle     *style,
                                              const gchar  *stock_id);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext and a style class)
gboolean    gtk_style_lookup_color           (GtkStyle     *style,
                                              const gchar  *color_name,
                                              GdkColor     *color);

GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext and gtk_render_icon)
GdkPixbuf*  gtk_style_render_icon     (GtkStyle            *style,
                                       const GtkIconSource *source,
                                       GtkTextDirection     direction,
                                       GtkStateType         state,
                                       GtkIconSize          size,
                                       GtkWidget           *widget,
                                       const gchar         *detail);

GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext and gtk_render_line)
void gtk_paint_hline             (GtkStyle           *style,
                                  cairo_t            *cr,
                                  GtkStateType        state_type,
                                  GtkWidget          *widget,
                                  const gchar        *detail,
                                  gint                x1,
                                  gint                x2,
                                  gint                y);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext and gtk_render_line)
void gtk_paint_vline             (GtkStyle           *style,
                                  cairo_t            *cr,
                                  GtkStateType        state_type,
                                  GtkWidget          *widget,
                                  const gchar        *detail,
                                  gint                y1_,
                                  gint                y2_,
                                  gint                x);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext and gtk_render_background)
void gtk_paint_shadow            (GtkStyle           *style,
                                  cairo_t            *cr,
                                  GtkStateType        state_type,
                                  GtkShadowType       shadow_type,
                                  GtkWidget          *widget,
                                  const gchar        *detail,
                                  gint                x,
                                  gint                y,
                                  gint                width,
                                  gint                height);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext and gtk_render_arrow)
void gtk_paint_arrow             (GtkStyle           *style,
                                  cairo_t            *cr,
                                  GtkStateType        state_type,
                                  GtkShadowType       shadow_type,
                                  GtkWidget          *widget,
                                  const gchar        *detail,
                                  GtkArrowType        arrow_type,
                                  gboolean            fill,
                                  gint                x,
                                  gint                y,
                                  gint                width,
                                  gint                height);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext and gtk_render_icon)
void gtk_paint_diamond           (GtkStyle           *style,
                                  cairo_t            *cr,
                                  GtkStateType        state_type,
                                  GtkShadowType       shadow_type,
                                  GtkWidget          *widget,
                                  const gchar        *detail,
                                  gint                x,
                                  gint                y,
                                  gint                width,
                                  gint                height);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext and gtk_render_frame)
void gtk_paint_box               (GtkStyle           *style,
                                  cairo_t            *cr,
                                  GtkStateType        state_type,
                                  GtkShadowType       shadow_type,
                                  GtkWidget          *widget,
                                  const gchar        *detail,
                                  gint                x,
                                  gint                y,
                                  gint                width,
                                  gint                height);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext and gtk_render_background)
void gtk_paint_flat_box          (GtkStyle           *style,
                                  cairo_t            *cr,
                                  GtkStateType        state_type,
                                  GtkShadowType       shadow_type,
                                  GtkWidget          *widget,
                                  const gchar        *detail,
                                  gint                x,
                                  gint                y,
                                  gint                width,
                                  gint                height);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext and gtk_render_check)
void gtk_paint_check             (GtkStyle           *style,
                                  cairo_t            *cr,
                                  GtkStateType        state_type,
                                  GtkShadowType       shadow_type,
                                  GtkWidget          *widget,
                                  const gchar        *detail,
                                  gint                x,
                                  gint                y,
                                  gint                width,
                                  gint                height);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext and gtk_render_option)
void gtk_paint_option            (GtkStyle           *style,
                                  cairo_t            *cr,
                                  GtkStateType        state_type,
                                  GtkShadowType       shadow_type,
                                  GtkWidget          *widget,
                                  const gchar        *detail,
                                  gint                x,
                                  gint                y,
                                  gint                width,
                                  gint                height);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext and gtk_render_background)
void gtk_paint_tab               (GtkStyle           *style,
                                  cairo_t            *cr,
                                  GtkStateType        state_type,
                                  GtkShadowType       shadow_type,
                                  GtkWidget          *widget,
                                  const gchar        *detail,
                                  gint                x,
                                  gint                y,
                                  gint                width,
                                  gint                height);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext)
void gtk_paint_shadow_gap        (GtkStyle           *style,
                                  cairo_t            *cr,
                                  GtkStateType        state_type,
                                  GtkShadowType       shadow_type,
                                  GtkWidget          *widget,
                                  const gchar        *detail,
                                  gint                x,
                                  gint                y,
                                  gint                width,
                                  gint                height,
                                  GtkPositionType     gap_side,
                                  gint                gap_x,
                                  gint                gap_width);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext)
void gtk_paint_box_gap           (GtkStyle           *style,
                                  cairo_t            *cr,
                                  GtkStateType        state_type,
                                  GtkShadowType       shadow_type,
                                  GtkWidget          *widget,
                                  const gchar        *detail,
                                  gint                x,
                                  gint                y,
                                  gint                width,
                                  gint                height,
                                  GtkPositionType     gap_side,
                                  gint                gap_x,
                                  gint                gap_width);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext and gtk_render_extension)
void gtk_paint_extension         (GtkStyle           *style,
                                  cairo_t            *cr,
                                  GtkStateType        state_type,
                                  GtkShadowType       shadow_type,
                                  GtkWidget          *widget,
                                  const gchar        *detail,
                                  gint                x,
                                  gint                y,
                                  gint                width,
                                  gint                height,
                                  GtkPositionType     gap_side);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext and gtk_render_focus)
void gtk_paint_focus             (GtkStyle           *style,
                                  cairo_t            *cr,
                                  GtkStateType        state_type,
                                  GtkWidget          *widget,
                                  const gchar        *detail,
                                  gint                x,
                                  gint                y,
                                  gint                width,
                                  gint                height);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext and gtk_render_focus)
void gtk_paint_slider            (GtkStyle           *style,
                                  cairo_t            *cr,
                                  GtkStateType        state_type,
                                  GtkShadowType       shadow_type,
                                  GtkWidget          *widget,
                                  const gchar        *detail,
                                  gint                x,
                                  gint                y,
                                  gint                width,
                                  gint                height,
                                  GtkOrientation      orientation);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext and gtk_render_handle)
void gtk_paint_handle            (GtkStyle           *style,
                                  cairo_t            *cr,
                                  GtkStateType        state_type,
                                  GtkShadowType       shadow_type,
                                  GtkWidget          *widget,
                                  const gchar        *detail,
                                  gint                x,
                                  gint                y,
                                  gint                width,
                                  gint                height,
                                  GtkOrientation      orientation);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext and gtk_render_expander)
void gtk_paint_expander          (GtkStyle           *style,
                                  cairo_t            *cr,
                                  GtkStateType        state_type,
                                  GtkWidget          *widget,
                                  const gchar        *detail,
                                  gint                x,
                                  gint                y,
                                  GtkExpanderStyle    expander_style);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext and gtk_render_layout)
void gtk_paint_layout            (GtkStyle           *style,
                                  cairo_t            *cr,
                                  GtkStateType        state_type,
                                  gboolean            use_text,
                                  GtkWidget          *widget,
                                  const gchar        *detail,
                                  gint                x,
                                  gint                y,
                                  PangoLayout        *layout);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext and gtk_render_handle)
void gtk_paint_resize_grip       (GtkStyle           *style,
                                  cairo_t            *cr,
                                  GtkStateType        state_type,
                                  GtkWidget          *widget,
                                  const gchar        *detail,
                                  GdkWindowEdge       edge,
                                  gint                x,
                                  gint                y,
                                  gint                width,
                                  gint                height);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext and gtk_render_icon)
void gtk_paint_spinner           (GtkStyle           *style,
                                  cairo_t            *cr,
                                  GtkStateType        state_type,
                                  GtkWidget          *widget,
                                  const gchar        *detail,
                                  guint               step,
                                  gint                x,
                                  gint                y,
                                  gint                width,
                                  gint                height);

GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext and gtk_style_context_get_property)
void gtk_style_get_style_property (GtkStyle    *style,
                                   GType        widget_type,
                                   const gchar *property_name,
                                   GValue      *value);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext and gtk_style_context_get_property)
void gtk_style_get_valist         (GtkStyle    *style,
                                   GType        widget_type,
                                   const gchar *first_property_name,
                                   va_list      var_args);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext and gtk_style_context_get_property)
void gtk_style_get                (GtkStyle    *style,
                                   GType        widget_type,
                                   const gchar *first_property_name,
                                   ...) G_GNUC_NULL_TERMINATED;


/* --- private API --- */
GtkStyle*     _gtk_style_new_for_path     (GdkScreen          *screen,
                                           GtkWidgetPath      *path);
void          _gtk_style_shade            (const GdkColor     *a,
                                           GdkColor           *b,
                                           gdouble             k);

GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext)
gboolean    gtk_style_has_context         (GtkStyle *style);

GDK_DEPRECATED_IN_3_0
void        gtk_widget_style_attach       (GtkWidget     *widget);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext)
gboolean    gtk_widget_has_rc_style       (GtkWidget            *widget);
GDK_DEPRECATED_IN_3_0
void        gtk_widget_set_style          (GtkWidget            *widget,
                                           GtkStyle             *style);
GDK_DEPRECATED_IN_3_0
void        gtk_widget_ensure_style       (GtkWidget            *widget);
GDK_DEPRECATED_IN_3_0_FOR(gtk_widget_get_style_context)
GtkStyle *  gtk_widget_get_style          (GtkWidget            *widget);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext)
void        gtk_widget_modify_style       (GtkWidget            *widget,
                                           GtkRcStyle           *style);
GDK_DEPRECATED_IN_3_0_FOR(GtkStyleContext)
GtkRcStyle *gtk_widget_get_modifier_style (GtkWidget            *widget);
GDK_DEPRECATED_IN_3_0_FOR(gtk_widget_override_color)
void        gtk_widget_modify_fg          (GtkWidget            *widget,
                                           GtkStateType          state,
                                           const GdkColor       *color);
GDK_DEPRECATED_IN_3_0_FOR(gtk_widget_override_background_color)
void        gtk_widget_modify_bg          (GtkWidget            *widget,
                                           GtkStateType          state,
                                           const GdkColor       *color);
GDK_DEPRECATED_IN_3_0_FOR(CSS style classes)
void        gtk_widget_modify_text        (GtkWidget            *widget,
                                           GtkStateType          state,
                                           const GdkColor       *color);
GDK_DEPRECATED_IN_3_0_FOR(CSS style classes)
void        gtk_widget_modify_base        (GtkWidget            *widget,
                                           GtkStateType          state,
                                           const GdkColor       *color);
GDK_DEPRECATED_IN_3_0_FOR(CSS style classes)
void        gtk_widget_modify_cursor      (GtkWidget            *widget,
                                           const GdkColor       *primary,
                                           const GdkColor       *secondary);
GDK_DEPRECATED_IN_3_0_FOR(gtk_widget_override_font)
void        gtk_widget_modify_font        (GtkWidget            *widget,
                                           PangoFontDescription *font_desc);
GDK_DEPRECATED_IN_3_0_FOR(gtk_widget_reset_style)
void       gtk_widget_reset_rc_styles     (GtkWidget      *widget);
GDK_DEPRECATED_IN_3_0_FOR(gtk_style_context_new)
GtkStyle*  gtk_widget_get_default_style   (void);
GDK_DEPRECATED_IN_3_0_FOR(gtk_widget_get_path)
void       gtk_widget_path                (GtkWidget *widget,
                                           guint     *path_length,
                                           gchar    **path,
                                           gchar    **path_reversed);
GDK_DEPRECATED_IN_3_0
void       gtk_widget_class_path          (GtkWidget *widget,
                                           guint     *path_length,
                                           gchar    **path,
                                           gchar    **path_reversed);
GDK_DEPRECATED_IN_3_0_FOR(gtk_widget_render_icon_pixbuf)
GdkPixbuf *gtk_widget_render_icon         (GtkWidget   *widget,
                                           const gchar *stock_id,
                                           GtkIconSize  size,
                                           const gchar *detail);

G_END_DECLS

#endif /* __GTK_STYLE_H__ */
