/* GTK - The GIMP Toolkit
 * gtktextlayout.h
 *
 * Copyright (c) 1992-1994 The Regents of the University of California.
 * Copyright (c) 1994-1997 Sun Microsystems, Inc.
 * Copyright (c) 2000 Red Hat, Inc.
 * Tk->Gtk port by Havoc Pennington
 * Pango support by Owen Taylor
 *
 * This file can be used under your choice of two licenses, the LGPL
 * and the original Tk license.
 *
 * LGPL:
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
 * License along with this library; if not, write to the Free
 * Software Foundation, Inc., 675 Mass Ave, Cambridge, MA 02139, USA.
 *
 * Original Tk license:
 *
 * This software is copyrighted by the Regents of the University of
 * California, Sun Microsystems, Inc., and other parties.  The
 * following terms apply to all files associated with the software
 * unless explicitly disclaimed in individual files.
 *
 * The authors hereby grant permission to use, copy, modify,
 * distribute, and license this software and its documentation for any
 * purpose, provided that existing copyright notices are retained in
 * all copies and that this notice is included verbatim in any
 * distributions. No written agreement, license, or royalty fee is
 * required for any of the authorized uses.  Modifications to this
 * software may be copyrighted by their authors and need not follow
 * the licensing terms described here, provided that the new terms are
 * clearly indicated on the first page of each file where they apply.
 *
 * IN NO EVENT SHALL THE AUTHORS OR DISTRIBUTORS BE LIABLE TO ANY
 * PARTY FOR DIRECT, INDIRECT, SPECIAL, INCIDENTAL, OR CONSEQUENTIAL
 * DAMAGES ARISING OUT OF THE USE OF THIS SOFTWARE, ITS DOCUMENTATION,
 * OR ANY DERIVATIVES THEREOF, EVEN IF THE AUTHORS HAVE BEEN ADVISED
 * OF THE POSSIBILITY OF SUCH DAMAGE.
 *
 * THE AUTHORS AND DISTRIBUTORS SPECIFICALLY DISCLAIM ANY WARRANTIES,
 * INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE, AND
 * NON-INFRINGEMENT.  THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS,
 * AND THE AUTHORS AND DISTRIBUTORS HAVE NO OBLIGATION TO PROVIDE
 * MAINTENANCE, SUPPORT, UPDATES, ENHANCEMENTS, OR MODIFICATIONS.
 *
 * GOVERNMENT USE: If you are acquiring this software on behalf of the
 * U.S. government, the Government shall have only "Restricted Rights"
 * in the software and related documentation as defined in the Federal
 * Acquisition Regulations (FARs) in Clause 52.227.19 (c) (2).  If you
 * are acquiring the software on behalf of the Department of Defense,
 * the software shall be classified as "Commercial Computer Software"
 * and the Government shall have only "Restricted Rights" as defined
 * in Clause 252.227-7013 (c) (1) of DFARs.  Notwithstanding the
 * foregoing, the authors grant the U.S. Government and others acting
 * in its behalf permission to use and distribute the software in
 * accordance with the terms specified in this license.
 *
 */
/*
 * Modified by the GTK+ Team and others 1997-2000.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GTK_TEXT_LAYOUT_H__
#define __GTK_TEXT_LAYOUT_H__

/* This is a "semi-private" header; it is intended for
 * use by the text widget, and the text canvas item,
 * but that's all. We may have to install it so the
 * canvas item can use it, but users are not supposed
 * to use it.
 */
#ifndef GTK_TEXT_USE_INTERNAL_UNSUPPORTED_API
#error "You are not supposed to be including this file; the equivalent public API is in gtktextview.h"
#endif

#include <gtk/gtk.h>

G_BEGIN_DECLS

/* forward declarations that have to be here to avoid including
 * gtktextbtree.h
 */
typedef struct _GtkTextLine     GtkTextLine;
typedef struct _GtkTextLineData GtkTextLineData;

#define GTK_TYPE_TEXT_LAYOUT             (gtk_text_layout_get_type ())
#define GTK_TEXT_LAYOUT(obj)             (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_TEXT_LAYOUT, GtkTextLayout))
#define GTK_TEXT_LAYOUT_CLASS(klass)     (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_TEXT_LAYOUT, GtkTextLayoutClass))
#define GTK_IS_TEXT_LAYOUT(obj)          (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_TEXT_LAYOUT))
#define GTK_IS_TEXT_LAYOUT_CLASS(klass)  (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_TEXT_LAYOUT))
#define GTK_TEXT_LAYOUT_GET_CLASS(obj)   (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_TEXT_LAYOUT, GtkTextLayoutClass))

typedef struct _GtkTextLayout         GtkTextLayout;
typedef struct _GtkTextLayoutClass    GtkTextLayoutClass;
typedef struct _GtkTextLineDisplay    GtkTextLineDisplay;
typedef struct _GtkTextCursorDisplay  GtkTextCursorDisplay;
typedef struct _GtkTextAttrAppearance GtkTextAttrAppearance;

struct _GtkTextLayout
{
  GObject parent_instance;

  /* width of the display area on-screen,
   * i.e. pixels we should wrap to fit inside. */
  gint screen_width;

  /* width/height of the total logical area being layed out */
  gint width;
  gint height;

  /* Pixel offsets from the left and from the top to be used when we
   * draw; these allow us to create left/top margins. We don't need
   * anything special for bottom/right margins, because those don't
   * affect drawing.
   */
  /* gint left_edge; */
  /* gint top_edge; */

  GtkTextBuffer *buffer;

  /* Default style used if no tags override it */
  GtkTextAttributes *default_style;

  /* Pango contexts used for creating layouts */
  PangoContext *ltr_context;
  PangoContext *rtl_context;

  /* A cache of one style; this is used to ensure
   * we don't constantly regenerate the style
   * over long runs with the same style. */
  GtkTextAttributes *one_style_cache;

  /* A cache of one line display. Getting the same line
   * many times in a row is the most common case.
   */
  GtkTextLineDisplay *one_display_cache;

  /* Whether we are allowed to wrap right now */
  gint wrap_loop_count;
  
  /* Whether to show the insertion cursor */
  guint cursor_visible : 1;

  /* For what GtkTextDirection to draw cursor GTK_TEXT_DIR_NONE -
   * means draw both cursors.
   */
  guint cursor_direction : 2;

  /* The keyboard direction is used to default the alignment when
     there are no strong characters.
  */
  guint keyboard_direction : 2;

  /* The preedit string and attributes, if any */

  gchar *preedit_string;
  PangoAttrList *preedit_attrs;
  gint preedit_len;
  gint preedit_cursor;

  guint overwrite_mode : 1;
};

struct _GtkTextLayoutClass
{
  GObjectClass parent_class;

  /* Some portion of the layout was invalidated
   */
  void  (*invalidated)  (GtkTextLayout *layout);

  /* A range of the layout changed appearance and possibly height
   */
  void  (*changed)              (GtkTextLayout     *layout,
                                 gint               y,
                                 gint               old_height,
                                 gint               new_height);
  GtkTextLineData* (*wrap)      (GtkTextLayout     *layout,
                                 GtkTextLine       *line,
                                 GtkTextLineData   *line_data); /* may be NULL */
  void  (*get_log_attrs)        (GtkTextLayout     *layout,
                                 GtkTextLine       *line,
                                 PangoLogAttr     **attrs,
                                 gint              *n_attrs);
  void  (*invalidate)           (GtkTextLayout     *layout,
                                 const GtkTextIter *start,
                                 const GtkTextIter *end);
  void  (*free_line_data)       (GtkTextLayout     *layout,
                                 GtkTextLine       *line,
                                 GtkTextLineData   *line_data);

  void (*allocate_child)        (GtkTextLayout     *layout,
                                 GtkWidget         *child,
                                 gint               x,
                                 gint               y);

  void (*invalidate_cursors)    (GtkTextLayout     *layout,
                                 const GtkTextIter *start,
                                 const GtkTextIter *end);

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
};

struct _GtkTextAttrAppearance
{
  PangoAttribute attr;
  GtkTextAppearance appearance;
};
struct _GtkTextCursorDisplay
{
  gint x;
  gint y;
  gint height;
  guint is_strong : 1;
  guint is_weak : 1;
};
struct _GtkTextLineDisplay
{
  PangoLayout *layout;
  GSList *cursors;
  GSList *shaped_objects;	/* Only for backwards compatibility */
  
  GtkTextDirection direction;

  gint width;                   /* Width of layout */
  gint total_width;             /* width - margins, if no width set on layout, if width set on layout, -1 */
  gint height;
  /* Amount layout is shifted from left edge - this is the left margin
   * plus any other factors, such as alignment or indentation.
   */
  gint x_offset;
  gint left_margin;
  gint right_margin;
  gint top_margin;
  gint bottom_margin;
  gint insert_index;		/* Byte index of insert cursor within para or -1 */

  gboolean size_only;
  GtkTextLine *line;
  
  GdkColor *pg_bg_color;

  GdkRectangle block_cursor;
  guint cursors_invalid : 1;
  guint has_block_cursor : 1;
  guint cursor_at_line_end : 1;
};

extern PangoAttrType gtk_text_attr_appearance_type;

GType         gtk_text_layout_get_type    (void) G_GNUC_CONST;

GtkTextLayout*     gtk_text_layout_new                   (void);
void               gtk_text_layout_set_buffer            (GtkTextLayout     *layout,
							  GtkTextBuffer     *buffer);
GtkTextBuffer     *gtk_text_layout_get_buffer            (GtkTextLayout     *layout);
void               gtk_text_layout_set_default_style     (GtkTextLayout     *layout,
							  GtkTextAttributes *values);
void               gtk_text_layout_set_contexts          (GtkTextLayout     *layout,
							  PangoContext      *ltr_context,
							  PangoContext      *rtl_context);
void               gtk_text_layout_set_cursor_direction  (GtkTextLayout     *layout,
                                                          GtkTextDirection   direction);
void		   gtk_text_layout_set_overwrite_mode	 (GtkTextLayout     *layout,
							  gboolean           overwrite);
void               gtk_text_layout_set_keyboard_direction (GtkTextLayout     *layout,
							   GtkTextDirection keyboard_dir);
void               gtk_text_layout_default_style_changed (GtkTextLayout     *layout);

void gtk_text_layout_set_screen_width       (GtkTextLayout     *layout,
                                             gint               width);
void gtk_text_layout_set_preedit_string     (GtkTextLayout     *layout,
 					     const gchar       *preedit_string,
 					     PangoAttrList     *preedit_attrs,
 					     gint               cursor_pos);

void     gtk_text_layout_set_cursor_visible (GtkTextLayout     *layout,
                                             gboolean           cursor_visible);
gboolean gtk_text_layout_get_cursor_visible (GtkTextLayout     *layout);

/* Getting the size or the lines potentially results in a call to
 * recompute, which is pretty massively expensive. Thus it should
 * basically only be done in an idle handler.
 *
 * Long-term, we would really like to be able to do these without
 * a full recompute so they may get cheaper over time.
 */
void    gtk_text_layout_get_size  (GtkTextLayout  *layout,
                                   gint           *width,
                                   gint           *height);
GSList* gtk_text_layout_get_lines (GtkTextLayout  *layout,
                                   /* [top_y, bottom_y) */
                                   gint            top_y,
                                   gint            bottom_y,
                                   gint           *first_line_y);

void gtk_text_layout_wrap_loop_start (GtkTextLayout *layout);
void gtk_text_layout_wrap_loop_end   (GtkTextLayout *layout);

GtkTextLineDisplay* gtk_text_layout_get_line_display  (GtkTextLayout      *layout,
                                                       GtkTextLine        *line,
                                                       gboolean            size_only);
void                gtk_text_layout_free_line_display (GtkTextLayout      *layout,
                                                       GtkTextLineDisplay *display);

void gtk_text_layout_get_line_at_y     (GtkTextLayout     *layout,
                                        GtkTextIter       *target_iter,
                                        gint               y,
                                        gint              *line_top);
void gtk_text_layout_get_iter_at_pixel (GtkTextLayout     *layout,
                                        GtkTextIter       *iter,
                                        gint               x,
                                        gint               y);
void gtk_text_layout_get_iter_at_position (GtkTextLayout     *layout,
					   GtkTextIter       *iter,
					   gint              *trailing,
					   gint               x,
					   gint               y);
void gtk_text_layout_invalidate        (GtkTextLayout     *layout,
                                        const GtkTextIter *start,
                                        const GtkTextIter *end);
void gtk_text_layout_invalidate_cursors(GtkTextLayout     *layout,
                                        const GtkTextIter *start,
                                        const GtkTextIter *end);
void gtk_text_layout_free_line_data    (GtkTextLayout     *layout,
                                        GtkTextLine       *line,
                                        GtkTextLineData   *line_data);

gboolean gtk_text_layout_is_valid        (GtkTextLayout *layout);
void     gtk_text_layout_validate_yrange (GtkTextLayout *layout,
                                          GtkTextIter   *anchor_line,
                                          gint           y0_,
                                          gint           y1_);
void     gtk_text_layout_validate        (GtkTextLayout *layout,
                                          gint           max_pixels);

/* This function should return the passed-in line data,
 * OR remove the existing line data from the line, and
 * return a NEW line data after adding it to the line.
 * That is, invariant after calling the callback is that
 * there should be exactly one line data for this view
 * stored on the btree line.
 */
GtkTextLineData* gtk_text_layout_wrap  (GtkTextLayout   *layout,
                                        GtkTextLine     *line,
                                        GtkTextLineData *line_data); /* may be NULL */
void     gtk_text_layout_changed              (GtkTextLayout     *layout,
                                               gint               y,
                                               gint               old_height,
                                               gint               new_height);
void     gtk_text_layout_cursors_changed      (GtkTextLayout     *layout,
                                               gint               y,
                                               gint               old_height,
                                               gint               new_height);
void     gtk_text_layout_get_iter_location    (GtkTextLayout     *layout,
                                               const GtkTextIter *iter,
                                               GdkRectangle      *rect);
void     gtk_text_layout_get_line_yrange      (GtkTextLayout     *layout,
                                               const GtkTextIter *iter,
                                               gint              *y,
                                               gint              *height);
void     _gtk_text_layout_get_line_xrange     (GtkTextLayout     *layout,
                                               const GtkTextIter *iter,
                                               gint              *x,
                                               gint              *width);
void     gtk_text_layout_get_cursor_locations (GtkTextLayout     *layout,
                                               GtkTextIter       *iter,
                                               GdkRectangle      *strong_pos,
                                               GdkRectangle      *weak_pos);
gboolean _gtk_text_layout_get_block_cursor    (GtkTextLayout     *layout,
					       GdkRectangle      *pos);
gboolean gtk_text_layout_clamp_iter_to_vrange (GtkTextLayout     *layout,
                                               GtkTextIter       *iter,
                                               gint               top,
                                               gint               bottom);

gboolean gtk_text_layout_move_iter_to_line_end      (GtkTextLayout *layout,
                                                     GtkTextIter   *iter,
                                                     gint           direction);
gboolean gtk_text_layout_move_iter_to_previous_line (GtkTextLayout *layout,
                                                     GtkTextIter   *iter);
gboolean gtk_text_layout_move_iter_to_next_line     (GtkTextLayout *layout,
                                                     GtkTextIter   *iter);
void     gtk_text_layout_move_iter_to_x             (GtkTextLayout *layout,
                                                     GtkTextIter   *iter,
                                                     gint           x);
gboolean gtk_text_layout_move_iter_visually         (GtkTextLayout *layout,
                                                     GtkTextIter   *iter,
                                                     gint           count);

gboolean gtk_text_layout_iter_starts_line           (GtkTextLayout       *layout,
                                                     const GtkTextIter   *iter);

void     gtk_text_layout_get_iter_at_line           (GtkTextLayout *layout,
                                                     GtkTextIter    *iter,
                                                     GtkTextLine    *line,
                                                     gint            byte_offset);

/* Don't use these. Use gtk_text_view_add_child_at_anchor().
 * These functions are defined in gtktextchild.c, but here
 * since they are semi-public and require GtkTextLayout to
 * be declared.
 */
void gtk_text_child_anchor_register_child   (GtkTextChildAnchor *anchor,
                                             GtkWidget          *child,
                                             GtkTextLayout      *layout);
void gtk_text_child_anchor_unregister_child (GtkTextChildAnchor *anchor,
                                             GtkWidget          *child);

void gtk_text_child_anchor_queue_resize     (GtkTextChildAnchor *anchor,
                                             GtkTextLayout      *layout);

void gtk_text_anchored_child_set_layout     (GtkWidget          *child,
                                             GtkTextLayout      *layout);

void gtk_text_layout_spew (GtkTextLayout *layout);

G_END_DECLS

#endif  /* __GTK_TEXT_LAYOUT_H__ */
