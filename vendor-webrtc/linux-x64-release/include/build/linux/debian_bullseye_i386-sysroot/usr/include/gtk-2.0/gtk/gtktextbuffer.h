/* GTK - The GIMP Toolkit
 * gtktextbuffer.h Copyright (C) 2000 Red Hat, Inc.
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

/*
 * Modified by the GTK+ Team and others 1997-2000.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GTK_TEXT_BUFFER_H__
#define __GTK_TEXT_BUFFER_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwidget.h>
#include <gtk/gtkclipboard.h>
#include <gtk/gtktexttagtable.h>
#include <gtk/gtktextiter.h>
#include <gtk/gtktextmark.h>
#include <gtk/gtktextchild.h>

G_BEGIN_DECLS

/*
 * This is the PUBLIC representation of a text buffer.
 * GtkTextBTree is the PRIVATE internal representation of it.
 */

/* these values are used as "info" for the targets contained in the
 * lists returned by gtk_text_buffer_get_copy,paste_target_list()
 *
 * the enum counts down from G_MAXUINT to avoid clashes with application
 * added drag destinations which usually start at 0.
 */
typedef enum
{
  GTK_TEXT_BUFFER_TARGET_INFO_BUFFER_CONTENTS = - 1,
  GTK_TEXT_BUFFER_TARGET_INFO_RICH_TEXT       = - 2,
  GTK_TEXT_BUFFER_TARGET_INFO_TEXT            = - 3
} GtkTextBufferTargetInfo;

typedef struct _GtkTextBTree GtkTextBTree;

typedef struct _GtkTextLogAttrCache GtkTextLogAttrCache;

#define GTK_TYPE_TEXT_BUFFER            (gtk_text_buffer_get_type ())
#define GTK_TEXT_BUFFER(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_TEXT_BUFFER, GtkTextBuffer))
#define GTK_TEXT_BUFFER_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_TEXT_BUFFER, GtkTextBufferClass))
#define GTK_IS_TEXT_BUFFER(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_TEXT_BUFFER))
#define GTK_IS_TEXT_BUFFER_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_TEXT_BUFFER))
#define GTK_TEXT_BUFFER_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_TEXT_BUFFER, GtkTextBufferClass))

typedef struct _GtkTextBufferClass GtkTextBufferClass;

struct _GtkTextBuffer
{
  GObject parent_instance;

  GtkTextTagTable *GSEAL (tag_table);
  GtkTextBTree *GSEAL (btree);

  GSList *GSEAL (clipboard_contents_buffers);
  GSList *GSEAL (selection_clipboards);

  GtkTextLogAttrCache *GSEAL (log_attr_cache);

  guint GSEAL (user_action_count);

  /* Whether the buffer has been modified since last save */
  guint GSEAL (modified) : 1;

  guint GSEAL (has_selection) : 1;
};

struct _GtkTextBufferClass
{
  GObjectClass parent_class;

  void (* insert_text)     (GtkTextBuffer *buffer,
                            GtkTextIter *pos,
                            const gchar *text,
                            gint length);

  void (* insert_pixbuf)   (GtkTextBuffer *buffer,
                            GtkTextIter   *pos,
                            GdkPixbuf     *pixbuf);

  void (* insert_child_anchor)   (GtkTextBuffer      *buffer,
                                  GtkTextIter        *pos,
                                  GtkTextChildAnchor *anchor);

  void (* delete_range)     (GtkTextBuffer *buffer,
                             GtkTextIter   *start,
                             GtkTextIter   *end);

  /* Only for text/widgets/pixbuf changed, marks/tags don't cause this
   * to be emitted
   */
  void (* changed)         (GtkTextBuffer *buffer);


  /* New value for the modified flag */
  void (* modified_changed)   (GtkTextBuffer *buffer);

  /* Mark moved or created */
  void (* mark_set)           (GtkTextBuffer *buffer,
                               const GtkTextIter *location,
                               GtkTextMark *mark);

  void (* mark_deleted)       (GtkTextBuffer *buffer,
                               GtkTextMark *mark);

  void (* apply_tag)          (GtkTextBuffer *buffer,
                               GtkTextTag *tag,
                               const GtkTextIter *start_char,
                               const GtkTextIter *end_char);

  void (* remove_tag)         (GtkTextBuffer *buffer,
                               GtkTextTag *tag,
                               const GtkTextIter *start_char,
                               const GtkTextIter *end_char);

  /* Called at the start and end of an atomic user action */
  void (* begin_user_action)  (GtkTextBuffer *buffer);
  void (* end_user_action)    (GtkTextBuffer *buffer);

  void (* paste_done)         (GtkTextBuffer *buffer,
                               GtkClipboard  *clipboard);

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
  void (*_gtk_reserved5) (void);
};

GType        gtk_text_buffer_get_type       (void) G_GNUC_CONST;



/* table is NULL to create a new one */
GtkTextBuffer *gtk_text_buffer_new            (GtkTextTagTable *table);
gint           gtk_text_buffer_get_line_count (GtkTextBuffer   *buffer);
gint           gtk_text_buffer_get_char_count (GtkTextBuffer   *buffer);


GtkTextTagTable* gtk_text_buffer_get_tag_table (GtkTextBuffer  *buffer);

/* Delete whole buffer, then insert */
void gtk_text_buffer_set_text          (GtkTextBuffer *buffer,
                                        const gchar   *text,
                                        gint           len);

/* Insert into the buffer */
void gtk_text_buffer_insert            (GtkTextBuffer *buffer,
                                        GtkTextIter   *iter,
                                        const gchar   *text,
                                        gint           len);
void gtk_text_buffer_insert_at_cursor  (GtkTextBuffer *buffer,
                                        const gchar   *text,
                                        gint           len);

gboolean gtk_text_buffer_insert_interactive           (GtkTextBuffer *buffer,
                                                       GtkTextIter   *iter,
                                                       const gchar   *text,
                                                       gint           len,
                                                       gboolean       default_editable);
gboolean gtk_text_buffer_insert_interactive_at_cursor (GtkTextBuffer *buffer,
                                                       const gchar   *text,
                                                       gint           len,
                                                       gboolean       default_editable);

void     gtk_text_buffer_insert_range             (GtkTextBuffer     *buffer,
                                                   GtkTextIter       *iter,
                                                   const GtkTextIter *start,
                                                   const GtkTextIter *end);
gboolean gtk_text_buffer_insert_range_interactive (GtkTextBuffer     *buffer,
                                                   GtkTextIter       *iter,
                                                   const GtkTextIter *start,
                                                   const GtkTextIter *end,
                                                   gboolean           default_editable);

void    gtk_text_buffer_insert_with_tags          (GtkTextBuffer     *buffer,
                                                   GtkTextIter       *iter,
                                                   const gchar       *text,
                                                   gint               len,
                                                   GtkTextTag        *first_tag,
                                                   ...) G_GNUC_NULL_TERMINATED;

void    gtk_text_buffer_insert_with_tags_by_name  (GtkTextBuffer     *buffer,
                                                   GtkTextIter       *iter,
                                                   const gchar       *text,
                                                   gint               len,
                                                   const gchar       *first_tag_name,
                                                   ...) G_GNUC_NULL_TERMINATED;

/* Delete from the buffer */
void     gtk_text_buffer_delete             (GtkTextBuffer *buffer,
					     GtkTextIter   *start,
					     GtkTextIter   *end);
gboolean gtk_text_buffer_delete_interactive (GtkTextBuffer *buffer,
					     GtkTextIter   *start_iter,
					     GtkTextIter   *end_iter,
					     gboolean       default_editable);
gboolean gtk_text_buffer_backspace          (GtkTextBuffer *buffer,
					     GtkTextIter   *iter,
					     gboolean       interactive,
					     gboolean       default_editable);

/* Obtain strings from the buffer */
gchar          *gtk_text_buffer_get_text            (GtkTextBuffer     *buffer,
                                                     const GtkTextIter *start,
                                                     const GtkTextIter *end,
                                                     gboolean           include_hidden_chars);

gchar          *gtk_text_buffer_get_slice           (GtkTextBuffer     *buffer,
                                                     const GtkTextIter *start,
                                                     const GtkTextIter *end,
                                                     gboolean           include_hidden_chars);

/* Insert a pixbuf */
void gtk_text_buffer_insert_pixbuf         (GtkTextBuffer *buffer,
                                            GtkTextIter   *iter,
                                            GdkPixbuf     *pixbuf);

/* Insert a child anchor */
void               gtk_text_buffer_insert_child_anchor (GtkTextBuffer      *buffer,
                                                        GtkTextIter        *iter,
                                                        GtkTextChildAnchor *anchor);

/* Convenience, create and insert a child anchor */
GtkTextChildAnchor *gtk_text_buffer_create_child_anchor (GtkTextBuffer *buffer,
                                                         GtkTextIter   *iter);

/* Mark manipulation */
void           gtk_text_buffer_add_mark    (GtkTextBuffer     *buffer,
                                            GtkTextMark       *mark,
                                            const GtkTextIter *where);
GtkTextMark   *gtk_text_buffer_create_mark (GtkTextBuffer     *buffer,
                                            const gchar       *mark_name,
                                            const GtkTextIter *where,
                                            gboolean           left_gravity);
void           gtk_text_buffer_move_mark   (GtkTextBuffer     *buffer,
                                            GtkTextMark       *mark,
                                            const GtkTextIter *where);
void           gtk_text_buffer_delete_mark (GtkTextBuffer     *buffer,
                                            GtkTextMark       *mark);
GtkTextMark*   gtk_text_buffer_get_mark    (GtkTextBuffer     *buffer,
                                            const gchar       *name);

void gtk_text_buffer_move_mark_by_name   (GtkTextBuffer     *buffer,
                                          const gchar       *name,
                                          const GtkTextIter *where);
void gtk_text_buffer_delete_mark_by_name (GtkTextBuffer     *buffer,
                                          const gchar       *name);

GtkTextMark* gtk_text_buffer_get_insert          (GtkTextBuffer *buffer);
GtkTextMark* gtk_text_buffer_get_selection_bound (GtkTextBuffer *buffer);

/* efficiently move insert and selection_bound at the same time */
void gtk_text_buffer_place_cursor (GtkTextBuffer     *buffer,
                                   const GtkTextIter *where);
void gtk_text_buffer_select_range (GtkTextBuffer     *buffer,
                                   const GtkTextIter *ins,
				   const GtkTextIter *bound);



/* Tag manipulation */
void gtk_text_buffer_apply_tag             (GtkTextBuffer     *buffer,
                                            GtkTextTag        *tag,
                                            const GtkTextIter *start,
                                            const GtkTextIter *end);
void gtk_text_buffer_remove_tag            (GtkTextBuffer     *buffer,
                                            GtkTextTag        *tag,
                                            const GtkTextIter *start,
                                            const GtkTextIter *end);
void gtk_text_buffer_apply_tag_by_name     (GtkTextBuffer     *buffer,
                                            const gchar       *name,
                                            const GtkTextIter *start,
                                            const GtkTextIter *end);
void gtk_text_buffer_remove_tag_by_name    (GtkTextBuffer     *buffer,
                                            const gchar       *name,
                                            const GtkTextIter *start,
                                            const GtkTextIter *end);
void gtk_text_buffer_remove_all_tags       (GtkTextBuffer     *buffer,
                                            const GtkTextIter *start,
                                            const GtkTextIter *end);


/* You can either ignore the return value, or use it to
 * set the attributes of the tag. tag_name can be NULL
 */
GtkTextTag    *gtk_text_buffer_create_tag (GtkTextBuffer *buffer,
                                           const gchar   *tag_name,
                                           const gchar   *first_property_name,
                                           ...);

/* Obtain iterators pointed at various places, then you can move the
 * iterator around using the GtkTextIter operators
 */
void gtk_text_buffer_get_iter_at_line_offset (GtkTextBuffer *buffer,
                                              GtkTextIter   *iter,
                                              gint           line_number,
                                              gint           char_offset);
void gtk_text_buffer_get_iter_at_line_index  (GtkTextBuffer *buffer,
                                              GtkTextIter   *iter,
                                              gint           line_number,
                                              gint           byte_index);
void gtk_text_buffer_get_iter_at_offset      (GtkTextBuffer *buffer,
                                              GtkTextIter   *iter,
                                              gint           char_offset);
void gtk_text_buffer_get_iter_at_line        (GtkTextBuffer *buffer,
                                              GtkTextIter   *iter,
                                              gint           line_number);
void gtk_text_buffer_get_start_iter          (GtkTextBuffer *buffer,
                                              GtkTextIter   *iter);
void gtk_text_buffer_get_end_iter            (GtkTextBuffer *buffer,
                                              GtkTextIter   *iter);
void gtk_text_buffer_get_bounds              (GtkTextBuffer *buffer,
                                              GtkTextIter   *start,
                                              GtkTextIter   *end);
void gtk_text_buffer_get_iter_at_mark        (GtkTextBuffer *buffer,
                                              GtkTextIter   *iter,
                                              GtkTextMark   *mark);

void gtk_text_buffer_get_iter_at_child_anchor (GtkTextBuffer      *buffer,
                                               GtkTextIter        *iter,
                                               GtkTextChildAnchor *anchor);

/* There's no get_first_iter because you just get the iter for
   line or char 0 */

/* Used to keep track of whether the buffer needs saving; anytime the
   buffer contents change, the modified flag is turned on. Whenever
   you save, turn it off. Tags and marks do not affect the modified
   flag, but if you would like them to you can connect a handler to
   the tag/mark signals and call set_modified in your handler */

gboolean        gtk_text_buffer_get_modified            (GtkTextBuffer *buffer);
void            gtk_text_buffer_set_modified            (GtkTextBuffer *buffer,
                                                         gboolean       setting);

gboolean        gtk_text_buffer_get_has_selection       (GtkTextBuffer *buffer);

void gtk_text_buffer_add_selection_clipboard    (GtkTextBuffer     *buffer,
						 GtkClipboard      *clipboard);
void gtk_text_buffer_remove_selection_clipboard (GtkTextBuffer     *buffer,
						 GtkClipboard      *clipboard);

void            gtk_text_buffer_cut_clipboard           (GtkTextBuffer *buffer,
							 GtkClipboard  *clipboard,
                                                         gboolean       default_editable);
void            gtk_text_buffer_copy_clipboard          (GtkTextBuffer *buffer,
							 GtkClipboard  *clipboard);
void            gtk_text_buffer_paste_clipboard         (GtkTextBuffer *buffer,
							 GtkClipboard  *clipboard,
							 GtkTextIter   *override_location,
                                                         gboolean       default_editable);

gboolean        gtk_text_buffer_get_selection_bounds    (GtkTextBuffer *buffer,
                                                         GtkTextIter   *start,
                                                         GtkTextIter   *end);
gboolean        gtk_text_buffer_delete_selection        (GtkTextBuffer *buffer,
                                                         gboolean       interactive,
                                                         gboolean       default_editable);

/* Called to specify atomic user actions, used to implement undo */
void            gtk_text_buffer_begin_user_action       (GtkTextBuffer *buffer);
void            gtk_text_buffer_end_user_action         (GtkTextBuffer *buffer);

GtkTargetList * gtk_text_buffer_get_copy_target_list    (GtkTextBuffer *buffer);
GtkTargetList * gtk_text_buffer_get_paste_target_list   (GtkTextBuffer *buffer);

/* INTERNAL private stuff */
void            _gtk_text_buffer_spew                  (GtkTextBuffer      *buffer);

GtkTextBTree*   _gtk_text_buffer_get_btree             (GtkTextBuffer      *buffer);

const PangoLogAttr* _gtk_text_buffer_get_line_log_attrs (GtkTextBuffer     *buffer,
                                                         const GtkTextIter *anywhere_in_line,
                                                         gint              *char_len);

void _gtk_text_buffer_notify_will_remove_tag (GtkTextBuffer *buffer,
                                              GtkTextTag    *tag);

G_END_DECLS

#endif
