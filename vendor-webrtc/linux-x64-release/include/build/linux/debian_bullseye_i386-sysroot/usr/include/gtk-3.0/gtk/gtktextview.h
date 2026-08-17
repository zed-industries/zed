/* GTK - The GIMP Toolkit
 * gtktextview.h Copyright (C) 2000 Red Hat, Inc.
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

#ifndef __GTK_TEXT_VIEW_H__
#define __GTK_TEXT_VIEW_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkcontainer.h>
#include <gtk/gtkimcontext.h>
#include <gtk/gtktextbuffer.h>
#include <gtk/gtkmenu.h>

G_BEGIN_DECLS

#define GTK_TYPE_TEXT_VIEW             (gtk_text_view_get_type ())
#define GTK_TEXT_VIEW(obj)             (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_TEXT_VIEW, GtkTextView))
#define GTK_TEXT_VIEW_CLASS(klass)     (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_TEXT_VIEW, GtkTextViewClass))
#define GTK_IS_TEXT_VIEW(obj)          (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_TEXT_VIEW))
#define GTK_IS_TEXT_VIEW_CLASS(klass)  (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_TEXT_VIEW))
#define GTK_TEXT_VIEW_GET_CLASS(obj)   (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_TEXT_VIEW, GtkTextViewClass))

/**
 * GtkTextWindowType:
 * @GTK_TEXT_WINDOW_PRIVATE: Invalid value, used as a marker
 * @GTK_TEXT_WINDOW_WIDGET: Window that floats over scrolling areas.
 * @GTK_TEXT_WINDOW_TEXT: Scrollable text window.
 * @GTK_TEXT_WINDOW_LEFT: Left side border window.
 * @GTK_TEXT_WINDOW_RIGHT: Right side border window.
 * @GTK_TEXT_WINDOW_TOP: Top border window.
 * @GTK_TEXT_WINDOW_BOTTOM: Bottom border window.
 *
 * Used to reference the parts of #GtkTextView.
 */
typedef enum
{
  GTK_TEXT_WINDOW_PRIVATE,
  GTK_TEXT_WINDOW_WIDGET,
  GTK_TEXT_WINDOW_TEXT,
  GTK_TEXT_WINDOW_LEFT,
  GTK_TEXT_WINDOW_RIGHT,
  GTK_TEXT_WINDOW_TOP,
  GTK_TEXT_WINDOW_BOTTOM
} GtkTextWindowType;

/**
 * GtkTextViewLayer:
 * @GTK_TEXT_VIEW_LAYER_BELOW: Old deprecated layer, use %GTK_TEXT_VIEW_LAYER_BELOW_TEXT instead
 * @GTK_TEXT_VIEW_LAYER_ABOVE: Old deprecated layer, use %GTK_TEXT_VIEW_LAYER_ABOVE_TEXT instead
 * @GTK_TEXT_VIEW_LAYER_BELOW_TEXT: The layer rendered below the text (but above the background).  Since: 3.20
 * @GTK_TEXT_VIEW_LAYER_ABOVE_TEXT: The layer rendered above the text.  Since: 3.20
 *
 * Used to reference the layers of #GtkTextView for the purpose of customized
 * drawing with the ::draw_layer vfunc.
 */
typedef enum
{
  GTK_TEXT_VIEW_LAYER_BELOW,
  GTK_TEXT_VIEW_LAYER_ABOVE,
  GTK_TEXT_VIEW_LAYER_BELOW_TEXT,
  GTK_TEXT_VIEW_LAYER_ABOVE_TEXT
} GtkTextViewLayer;

/**
 * GtkTextExtendSelection:
 * @GTK_TEXT_EXTEND_SELECTION_WORD: Selects the current word. It is triggered by
 *   a double-click for example.
 * @GTK_TEXT_EXTEND_SELECTION_LINE: Selects the current line. It is triggered by
 *   a triple-click for example.
 *
 * Granularity types that extend the text selection. Use the
 * #GtkTextView::extend-selection signal to customize the selection.
 *
 * Since: 3.16
 */
typedef enum
{
  GTK_TEXT_EXTEND_SELECTION_WORD,
  GTK_TEXT_EXTEND_SELECTION_LINE
} GtkTextExtendSelection;

/**
 * GTK_TEXT_VIEW_PRIORITY_VALIDATE: (value 125)
 *
 * The priority at which the text view validates onscreen lines
 * in an idle job in the background.
 */
#define GTK_TEXT_VIEW_PRIORITY_VALIDATE (GDK_PRIORITY_REDRAW + 5)

typedef struct _GtkTextView        GtkTextView;
typedef struct _GtkTextViewPrivate GtkTextViewPrivate;
typedef struct _GtkTextViewClass   GtkTextViewClass;

struct _GtkTextView
{
  GtkContainer parent_instance;

  /*< private >*/

  GtkTextViewPrivate *priv;
};

/**
 * GtkTextViewClass:
 * @parent_class: The object class structure needs to be the first
 * @populate_popup: The class handler for the #GtkTextView::populate-popup
 *   signal.
 * @move_cursor: The class handler for the #GtkTextView::move-cursor
 *   keybinding signal.
 * @set_anchor: The class handler for the #GtkTextView::set-anchor
 *   keybinding signal.
 * @insert_at_cursor: The class handler for the #GtkTextView::insert-at-cursor
 *   keybinding signal.
 * @delete_from_cursor: The class handler for the #GtkTextView::delete-from-cursor
 *   keybinding signal.
 * @backspace: The class handler for the #GtkTextView::backspace
 *   keybinding signal.
 * @cut_clipboard: The class handler for the #GtkTextView::cut-clipboard
 *   keybinding signal
 * @copy_clipboard: The class handler for the #GtkTextview::copy-clipboard
 *   keybinding signal.
 * @paste_clipboard: The class handler for the #GtkTextView::paste-clipboard
 *   keybinding signal.
 * @toggle_overwrite: The class handler for the #GtkTextView::toggle-overwrite
 *   keybinding signal.
 * @create_buffer: The create_buffer vfunc is called to create a #GtkTextBuffer
 *   for the text view. The default implementation is to just call
 *   gtk_text_buffer_new(). Since: 3.10
 * @draw_layer: The draw_layer vfunc is called before and after the text
 *   view is drawing its own text. Applications can override this vfunc
 *   in a subclass to draw customized content underneath or above the
 *   text. In the %GTK_TEXT_VIEW_LAYER_BELOW_TEXT and %GTK_TEXT_VIEW_LAYER_ABOVE_TEXT
 *   the drawing is done in the buffer coordinate space, but the older (deprecated)
 *   layers %GTK_TEXT_VIEW_LAYER_BELOW and %GTK_TEXT_VIEW_LAYER_ABOVE work in viewport
 *   coordinates, which makes them unnecessarily hard to use. Since: 3.14
 * @extend_selection: The class handler for the #GtkTextView::extend-selection
 *   signal. Since 3.16
 */
struct _GtkTextViewClass
{
  GtkContainerClass parent_class;

  /*< public >*/

  void (* populate_popup)        (GtkTextView      *text_view,
                                  GtkWidget        *popup);
  void (* move_cursor)           (GtkTextView      *text_view,
                                  GtkMovementStep   step,
                                  gint              count,
                                  gboolean          extend_selection);
  void (* set_anchor)            (GtkTextView      *text_view);
  void (* insert_at_cursor)      (GtkTextView      *text_view,
                                  const gchar      *str);
  void (* delete_from_cursor)    (GtkTextView      *text_view,
                                  GtkDeleteType     type,
                                  gint              count);
  void (* backspace)             (GtkTextView      *text_view);
  void (* cut_clipboard)         (GtkTextView      *text_view);
  void (* copy_clipboard)        (GtkTextView      *text_view);
  void (* paste_clipboard)       (GtkTextView      *text_view);
  void (* toggle_overwrite)      (GtkTextView      *text_view);
  GtkTextBuffer * (* create_buffer) (GtkTextView   *text_view);
  void (* draw_layer)            (GtkTextView      *text_view,
			          GtkTextViewLayer  layer,
			          cairo_t          *cr);
  gboolean (* extend_selection)  (GtkTextView            *text_view,
                                  GtkTextExtendSelection  granularity,
                                  const GtkTextIter      *location,
                                  GtkTextIter            *start,
                                  GtkTextIter            *end);
  void (* insert_emoji)          (GtkTextView      *text_view);

  /*< private >*/

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

GDK_AVAILABLE_IN_ALL
GType          gtk_text_view_get_type              (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkWidget *    gtk_text_view_new                   (void);
GDK_AVAILABLE_IN_ALL
GtkWidget *    gtk_text_view_new_with_buffer       (GtkTextBuffer *buffer);
GDK_AVAILABLE_IN_ALL
void           gtk_text_view_set_buffer            (GtkTextView   *text_view,
                                                    GtkTextBuffer *buffer);
GDK_AVAILABLE_IN_ALL
GtkTextBuffer *gtk_text_view_get_buffer            (GtkTextView   *text_view);
GDK_AVAILABLE_IN_ALL
gboolean       gtk_text_view_scroll_to_iter        (GtkTextView   *text_view,
                                                    GtkTextIter   *iter,
                                                    gdouble        within_margin,
                                                    gboolean       use_align,
                                                    gdouble        xalign,
                                                    gdouble        yalign);
GDK_AVAILABLE_IN_ALL
void           gtk_text_view_scroll_to_mark        (GtkTextView   *text_view,
                                                    GtkTextMark   *mark,
                                                    gdouble        within_margin,
                                                    gboolean       use_align,
                                                    gdouble        xalign,
                                                    gdouble        yalign);
GDK_AVAILABLE_IN_ALL
void           gtk_text_view_scroll_mark_onscreen  (GtkTextView   *text_view,
                                                    GtkTextMark   *mark);
GDK_AVAILABLE_IN_ALL
gboolean       gtk_text_view_move_mark_onscreen    (GtkTextView   *text_view,
                                                    GtkTextMark   *mark);
GDK_AVAILABLE_IN_ALL
gboolean       gtk_text_view_place_cursor_onscreen (GtkTextView   *text_view);

GDK_AVAILABLE_IN_ALL
void           gtk_text_view_get_visible_rect      (GtkTextView   *text_view,
                                                    GdkRectangle  *visible_rect);
GDK_AVAILABLE_IN_ALL
void           gtk_text_view_set_cursor_visible    (GtkTextView   *text_view,
                                                    gboolean       setting);
GDK_AVAILABLE_IN_ALL
gboolean       gtk_text_view_get_cursor_visible    (GtkTextView   *text_view);

GDK_AVAILABLE_IN_3_20
void           gtk_text_view_reset_cursor_blink    (GtkTextView   *text_view);

GDK_AVAILABLE_IN_ALL
void           gtk_text_view_get_cursor_locations  (GtkTextView       *text_view,
                                                    const GtkTextIter *iter,
                                                    GdkRectangle      *strong,
                                                    GdkRectangle      *weak);
GDK_AVAILABLE_IN_ALL
void           gtk_text_view_get_iter_location     (GtkTextView   *text_view,
                                                    const GtkTextIter *iter,
                                                    GdkRectangle  *location);
GDK_AVAILABLE_IN_ALL
gboolean       gtk_text_view_get_iter_at_location  (GtkTextView   *text_view,
                                                    GtkTextIter   *iter,
                                                    gint           x,
                                                    gint           y);
GDK_AVAILABLE_IN_ALL
gboolean       gtk_text_view_get_iter_at_position  (GtkTextView   *text_view,
                                                    GtkTextIter   *iter,
						    gint          *trailing,
                                                    gint           x,
                                                    gint           y);
GDK_AVAILABLE_IN_ALL
void           gtk_text_view_get_line_yrange       (GtkTextView       *text_view,
                                                    const GtkTextIter *iter,
                                                    gint              *y,
                                                    gint              *height);

GDK_AVAILABLE_IN_ALL
void           gtk_text_view_get_line_at_y         (GtkTextView       *text_view,
                                                    GtkTextIter       *target_iter,
                                                    gint               y,
                                                    gint              *line_top);

GDK_AVAILABLE_IN_ALL
void gtk_text_view_buffer_to_window_coords (GtkTextView       *text_view,
                                            GtkTextWindowType  win,
                                            gint               buffer_x,
                                            gint               buffer_y,
                                            gint              *window_x,
                                            gint              *window_y);
GDK_AVAILABLE_IN_ALL
void gtk_text_view_window_to_buffer_coords (GtkTextView       *text_view,
                                            GtkTextWindowType  win,
                                            gint               window_x,
                                            gint               window_y,
                                            gint              *buffer_x,
                                            gint              *buffer_y);

GDK_DEPRECATED_IN_3_0_FOR(gtk_scrollable_get_hadjustment)
GtkAdjustment*   gtk_text_view_get_hadjustment (GtkTextView   *text_view);
GDK_DEPRECATED_IN_3_0_FOR(gtk_scrollable_get_vadjustment)
GtkAdjustment*   gtk_text_view_get_vadjustment (GtkTextView   *text_view);

GDK_AVAILABLE_IN_ALL
GdkWindow*        gtk_text_view_get_window      (GtkTextView       *text_view,
                                                 GtkTextWindowType  win);
GDK_AVAILABLE_IN_ALL
GtkTextWindowType gtk_text_view_get_window_type (GtkTextView       *text_view,
                                                 GdkWindow         *window);

GDK_AVAILABLE_IN_ALL
void gtk_text_view_set_border_window_size (GtkTextView       *text_view,
                                           GtkTextWindowType  type,
                                           gint               size);
GDK_AVAILABLE_IN_ALL
gint gtk_text_view_get_border_window_size (GtkTextView       *text_view,
					   GtkTextWindowType  type);

GDK_AVAILABLE_IN_ALL
gboolean gtk_text_view_forward_display_line           (GtkTextView       *text_view,
                                                       GtkTextIter       *iter);
GDK_AVAILABLE_IN_ALL
gboolean gtk_text_view_backward_display_line          (GtkTextView       *text_view,
                                                       GtkTextIter       *iter);
GDK_AVAILABLE_IN_ALL
gboolean gtk_text_view_forward_display_line_end       (GtkTextView       *text_view,
                                                       GtkTextIter       *iter);
GDK_AVAILABLE_IN_ALL
gboolean gtk_text_view_backward_display_line_start    (GtkTextView       *text_view,
                                                       GtkTextIter       *iter);
GDK_AVAILABLE_IN_ALL
gboolean gtk_text_view_starts_display_line            (GtkTextView       *text_view,
                                                       const GtkTextIter *iter);
GDK_AVAILABLE_IN_ALL
gboolean gtk_text_view_move_visually                  (GtkTextView       *text_view,
                                                       GtkTextIter       *iter,
                                                       gint               count);

GDK_AVAILABLE_IN_ALL
gboolean        gtk_text_view_im_context_filter_keypress        (GtkTextView       *text_view,
                                                                 GdkEventKey       *event);
GDK_AVAILABLE_IN_ALL
void            gtk_text_view_reset_im_context                  (GtkTextView       *text_view);

/* Adding child widgets */
GDK_AVAILABLE_IN_ALL
void gtk_text_view_add_child_at_anchor (GtkTextView          *text_view,
                                        GtkWidget            *child,
                                        GtkTextChildAnchor   *anchor);

GDK_AVAILABLE_IN_ALL
void gtk_text_view_add_child_in_window (GtkTextView          *text_view,
                                        GtkWidget            *child,
                                        GtkTextWindowType     which_window,
                                        /* window coordinates */
                                        gint                  xpos,
                                        gint                  ypos);

GDK_AVAILABLE_IN_ALL
void gtk_text_view_move_child          (GtkTextView          *text_view,
                                        GtkWidget            *child,
                                        /* window coordinates */
                                        gint                  xpos,
                                        gint                  ypos);

/* Default style settings (fallbacks if no tag affects the property) */

GDK_AVAILABLE_IN_ALL
void             gtk_text_view_set_wrap_mode          (GtkTextView      *text_view,
                                                       GtkWrapMode       wrap_mode);
GDK_AVAILABLE_IN_ALL
GtkWrapMode      gtk_text_view_get_wrap_mode          (GtkTextView      *text_view);
GDK_AVAILABLE_IN_ALL
void             gtk_text_view_set_editable           (GtkTextView      *text_view,
                                                       gboolean          setting);
GDK_AVAILABLE_IN_ALL
gboolean         gtk_text_view_get_editable           (GtkTextView      *text_view);
GDK_AVAILABLE_IN_ALL
void             gtk_text_view_set_overwrite          (GtkTextView      *text_view,
						       gboolean          overwrite);
GDK_AVAILABLE_IN_ALL
gboolean         gtk_text_view_get_overwrite          (GtkTextView      *text_view);
GDK_AVAILABLE_IN_ALL
void		 gtk_text_view_set_accepts_tab        (GtkTextView	*text_view,
						       gboolean		 accepts_tab);
GDK_AVAILABLE_IN_ALL
gboolean	 gtk_text_view_get_accepts_tab        (GtkTextView	*text_view);
GDK_AVAILABLE_IN_ALL
void             gtk_text_view_set_pixels_above_lines (GtkTextView      *text_view,
                                                       gint              pixels_above_lines);
GDK_AVAILABLE_IN_ALL
gint             gtk_text_view_get_pixels_above_lines (GtkTextView      *text_view);
GDK_AVAILABLE_IN_ALL
void             gtk_text_view_set_pixels_below_lines (GtkTextView      *text_view,
                                                       gint              pixels_below_lines);
GDK_AVAILABLE_IN_ALL
gint             gtk_text_view_get_pixels_below_lines (GtkTextView      *text_view);
GDK_AVAILABLE_IN_ALL
void             gtk_text_view_set_pixels_inside_wrap (GtkTextView      *text_view,
                                                       gint              pixels_inside_wrap);
GDK_AVAILABLE_IN_ALL
gint             gtk_text_view_get_pixels_inside_wrap (GtkTextView      *text_view);
GDK_AVAILABLE_IN_ALL
void             gtk_text_view_set_justification      (GtkTextView      *text_view,
                                                       GtkJustification  justification);
GDK_AVAILABLE_IN_ALL
GtkJustification gtk_text_view_get_justification      (GtkTextView      *text_view);
GDK_AVAILABLE_IN_ALL
void             gtk_text_view_set_left_margin        (GtkTextView      *text_view,
                                                       gint              left_margin);
GDK_AVAILABLE_IN_ALL
gint             gtk_text_view_get_left_margin        (GtkTextView      *text_view);
GDK_AVAILABLE_IN_ALL
void             gtk_text_view_set_right_margin       (GtkTextView      *text_view,
                                                       gint              right_margin);
GDK_AVAILABLE_IN_ALL
gint             gtk_text_view_get_right_margin       (GtkTextView      *text_view);
GDK_AVAILABLE_IN_3_18
void             gtk_text_view_set_top_margin         (GtkTextView      *text_view,
                                                       gint              top_margin);
GDK_AVAILABLE_IN_3_18
gint             gtk_text_view_get_top_margin         (GtkTextView      *text_view);
GDK_AVAILABLE_IN_3_18
void             gtk_text_view_set_bottom_margin      (GtkTextView      *text_view,
                                                       gint              bottom_margin);
GDK_AVAILABLE_IN_3_18
gint             gtk_text_view_get_bottom_margin       (GtkTextView      *text_view);
GDK_AVAILABLE_IN_ALL
void             gtk_text_view_set_indent             (GtkTextView      *text_view,
                                                       gint              indent);
GDK_AVAILABLE_IN_ALL
gint             gtk_text_view_get_indent             (GtkTextView      *text_view);
GDK_AVAILABLE_IN_ALL
void             gtk_text_view_set_tabs               (GtkTextView      *text_view,
                                                       PangoTabArray    *tabs);
GDK_AVAILABLE_IN_ALL
PangoTabArray*   gtk_text_view_get_tabs               (GtkTextView      *text_view);

/* note that the return value of this changes with the theme */
GDK_AVAILABLE_IN_ALL
GtkTextAttributes* gtk_text_view_get_default_attributes (GtkTextView    *text_view);

GDK_AVAILABLE_IN_3_6
void             gtk_text_view_set_input_purpose      (GtkTextView      *text_view,
                                                       GtkInputPurpose   purpose);
GDK_AVAILABLE_IN_3_6
GtkInputPurpose  gtk_text_view_get_input_purpose      (GtkTextView      *text_view);

GDK_AVAILABLE_IN_3_6
void             gtk_text_view_set_input_hints        (GtkTextView      *text_view,
                                                       GtkInputHints     hints);
GDK_AVAILABLE_IN_3_6
GtkInputHints    gtk_text_view_get_input_hints        (GtkTextView      *text_view);

GDK_AVAILABLE_IN_3_16
void             gtk_text_view_set_monospace          (GtkTextView      *text_view,
                                                       gboolean          monospace);
GDK_AVAILABLE_IN_3_16
gboolean         gtk_text_view_get_monospace          (GtkTextView      *text_view);

G_END_DECLS

#endif /* __GTK_TEXT_VIEW_H__ */
