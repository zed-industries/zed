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
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

/*
 * Modified by the GTK+ Team and others 1997-2000.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GTK_TEXT_BUFFER_H__
#define __GTK_TEXT_BUFFER_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
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

/**
 * GtkTextBufferTargetInfo:
 * @GTK_TEXT_BUFFER_TARGET_INFO_BUFFER_CONTENTS: Buffer contents
 * @GTK_TEXT_BUFFER_TARGET_INFO_RICH_TEXT: Rich text
 * @GTK_TEXT_BUFFER_TARGET_INFO_TEXT: Text
 *
 * These values are used as “info” for the targets contained in the
 * lists returned by gtk_text_buffer_get_copy_target_list() and
 * gtk_text_buffer_get_paste_target_list().
 *
 * The values counts down from `-1` to avoid clashes
 * with application added drag destinations which usually start at 0.
 */
typedef enum
{
  GTK_TEXT_BUFFER_TARGET_INFO_BUFFER_CONTENTS = - 1,
  GTK_TEXT_BUFFER_TARGET_INFO_RICH_TEXT       = - 2,
  GTK_TEXT_BUFFER_TARGET_INFO_TEXT            = - 3
} GtkTextBufferTargetInfo;

typedef struct _GtkTextBTree GtkTextBTree;

#define GTK_TYPE_TEXT_BUFFER            (gtk_text_buffer_get_type ())
#define GTK_TEXT_BUFFER(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_TEXT_BUFFER, GtkTextBuffer))
#define GTK_TEXT_BUFFER_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_TEXT_BUFFER, GtkTextBufferClass))
#define GTK_IS_TEXT_BUFFER(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_TEXT_BUFFER))
#define GTK_IS_TEXT_BUFFER_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_TEXT_BUFFER))
#define GTK_TEXT_BUFFER_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_TEXT_BUFFER, GtkTextBufferClass))

typedef struct _GtkTextBufferPrivate GtkTextBufferPrivate;
typedef struct _GtkTextBufferClass GtkTextBufferClass;

struct _GtkTextBuffer
{
  GObject parent_instance;

  GtkTextBufferPrivate *priv;
};

/**
 * GtkTextBufferClass:
 * @parent_class: The object class structure needs to be the first.
 * @insert_text: The class handler for the #GtkTextBuffer::insert-text signal.
 * @insert_pixbuf: The class handler for the #GtkTextBuffer::insert-pixbuf
 *   signal.
 * @insert_child_anchor: The class handler for the
 *   #GtkTextBuffer::insert-child-anchor signal.
 * @delete_range: The class handler for the #GtkTextBuffer::delete-range signal.
 * @changed: The class handler for the #GtkTextBuffer::changed signal.
 * @modified_changed: The class handler for the #GtkTextBuffer::modified-changed
 *   signal.
 * @mark_set: The class handler for the #GtkTextBuffer::mark-set signal.
 * @mark_deleted: The class handler for the #GtkTextBuffer::mark-deleted signal.
 * @apply_tag: The class handler for the #GtkTextBuffer::apply-tag signal.
 * @remove_tag: The class handler for the #GtkTextBuffer::remove-tag signal.
 * @begin_user_action: The class handler for the
 *   #GtkTextBuffer::begin-user-action signal.
 * @end_user_action: The class handler for the #GtkTextBuffer::end-user-action
 *   signal.
 * @paste_done: The class handler for the #GtkTextBuffer::paste-done signal.
 */
struct _GtkTextBufferClass
{
  GObjectClass parent_class;

  void (* insert_text)            (GtkTextBuffer      *buffer,
                                   GtkTextIter        *pos,
                                   const gchar        *new_text,
                                   gint                new_text_length);

  void (* insert_pixbuf)          (GtkTextBuffer      *buffer,
                                   GtkTextIter        *iter,
                                   GdkPixbuf          *pixbuf);

  void (* insert_child_anchor)    (GtkTextBuffer      *buffer,
                                   GtkTextIter        *iter,
                                   GtkTextChildAnchor *anchor);

  void (* delete_range)           (GtkTextBuffer      *buffer,
                                   GtkTextIter        *start,
                                   GtkTextIter        *end);

  void (* changed)                (GtkTextBuffer      *buffer);

  void (* modified_changed)       (GtkTextBuffer      *buffer);

  void (* mark_set)               (GtkTextBuffer      *buffer,
                                   const GtkTextIter  *location,
                                   GtkTextMark        *mark);

  void (* mark_deleted)           (GtkTextBuffer      *buffer,
                                   GtkTextMark        *mark);

  void (* apply_tag)              (GtkTextBuffer      *buffer,
                                   GtkTextTag         *tag,
                                   const GtkTextIter  *start,
                                   const GtkTextIter  *end);

  void (* remove_tag)             (GtkTextBuffer      *buffer,
                                   GtkTextTag         *tag,
                                   const GtkTextIter  *start,
                                   const GtkTextIter  *end);

  void (* begin_user_action)      (GtkTextBuffer      *buffer);

  void (* end_user_action)        (GtkTextBuffer      *buffer);

  void (* paste_done)             (GtkTextBuffer      *buffer,
                                   GtkClipboard       *clipboard);

  /*< private >*/

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

GDK_AVAILABLE_IN_ALL
GType        gtk_text_buffer_get_type       (void) G_GNUC_CONST;



/* table is NULL to create a new one */
GDK_AVAILABLE_IN_ALL
GtkTextBuffer *gtk_text_buffer_new            (GtkTextTagTable *table);
GDK_AVAILABLE_IN_ALL
gint           gtk_text_buffer_get_line_count (GtkTextBuffer   *buffer);
GDK_AVAILABLE_IN_ALL
gint           gtk_text_buffer_get_char_count (GtkTextBuffer   *buffer);


GDK_AVAILABLE_IN_ALL
GtkTextTagTable* gtk_text_buffer_get_tag_table (GtkTextBuffer  *buffer);

/* Delete whole buffer, then insert */
GDK_AVAILABLE_IN_ALL
void gtk_text_buffer_set_text          (GtkTextBuffer *buffer,
                                        const gchar   *text,
                                        gint           len);

/* Insert into the buffer */
GDK_AVAILABLE_IN_ALL
void gtk_text_buffer_insert            (GtkTextBuffer *buffer,
                                        GtkTextIter   *iter,
                                        const gchar   *text,
                                        gint           len);
GDK_AVAILABLE_IN_ALL
void gtk_text_buffer_insert_at_cursor  (GtkTextBuffer *buffer,
                                        const gchar   *text,
                                        gint           len);

GDK_AVAILABLE_IN_ALL
gboolean gtk_text_buffer_insert_interactive           (GtkTextBuffer *buffer,
                                                       GtkTextIter   *iter,
                                                       const gchar   *text,
                                                       gint           len,
                                                       gboolean       default_editable);
GDK_AVAILABLE_IN_ALL
gboolean gtk_text_buffer_insert_interactive_at_cursor (GtkTextBuffer *buffer,
                                                       const gchar   *text,
                                                       gint           len,
                                                       gboolean       default_editable);

GDK_AVAILABLE_IN_ALL
void     gtk_text_buffer_insert_range             (GtkTextBuffer     *buffer,
                                                   GtkTextIter       *iter,
                                                   const GtkTextIter *start,
                                                   const GtkTextIter *end);
GDK_AVAILABLE_IN_ALL
gboolean gtk_text_buffer_insert_range_interactive (GtkTextBuffer     *buffer,
                                                   GtkTextIter       *iter,
                                                   const GtkTextIter *start,
                                                   const GtkTextIter *end,
                                                   gboolean           default_editable);

GDK_AVAILABLE_IN_ALL
void    gtk_text_buffer_insert_with_tags          (GtkTextBuffer     *buffer,
                                                   GtkTextIter       *iter,
                                                   const gchar       *text,
                                                   gint               len,
                                                   GtkTextTag        *first_tag,
                                                   ...) G_GNUC_NULL_TERMINATED;

GDK_AVAILABLE_IN_ALL
void    gtk_text_buffer_insert_with_tags_by_name  (GtkTextBuffer     *buffer,
                                                   GtkTextIter       *iter,
                                                   const gchar       *text,
                                                   gint               len,
                                                   const gchar       *first_tag_name,
                                                   ...) G_GNUC_NULL_TERMINATED;

GDK_AVAILABLE_IN_3_16
void     gtk_text_buffer_insert_markup            (GtkTextBuffer     *buffer,
                                                   GtkTextIter       *iter,
                                                   const gchar       *markup,
                                                   gint               len);

/* Delete from the buffer */
GDK_AVAILABLE_IN_ALL
void     gtk_text_buffer_delete             (GtkTextBuffer *buffer,
					     GtkTextIter   *start,
					     GtkTextIter   *end);
GDK_AVAILABLE_IN_ALL
gboolean gtk_text_buffer_delete_interactive (GtkTextBuffer *buffer,
					     GtkTextIter   *start_iter,
					     GtkTextIter   *end_iter,
					     gboolean       default_editable);
GDK_AVAILABLE_IN_ALL
gboolean gtk_text_buffer_backspace          (GtkTextBuffer *buffer,
					     GtkTextIter   *iter,
					     gboolean       interactive,
					     gboolean       default_editable);

/* Obtain strings from the buffer */
GDK_AVAILABLE_IN_ALL
gchar          *gtk_text_buffer_get_text            (GtkTextBuffer     *buffer,
                                                     const GtkTextIter *start,
                                                     const GtkTextIter *end,
                                                     gboolean           include_hidden_chars);

GDK_AVAILABLE_IN_ALL
gchar          *gtk_text_buffer_get_slice           (GtkTextBuffer     *buffer,
                                                     const GtkTextIter *start,
                                                     const GtkTextIter *end,
                                                     gboolean           include_hidden_chars);

/* Insert a pixbuf */
GDK_AVAILABLE_IN_ALL
void gtk_text_buffer_insert_pixbuf         (GtkTextBuffer *buffer,
                                            GtkTextIter   *iter,
                                            GdkPixbuf     *pixbuf);

/* Insert a child anchor */
GDK_AVAILABLE_IN_ALL
void               gtk_text_buffer_insert_child_anchor (GtkTextBuffer      *buffer,
                                                        GtkTextIter        *iter,
                                                        GtkTextChildAnchor *anchor);

/* Convenience, create and insert a child anchor */
GDK_AVAILABLE_IN_ALL
GtkTextChildAnchor *gtk_text_buffer_create_child_anchor (GtkTextBuffer *buffer,
                                                         GtkTextIter   *iter);

/* Mark manipulation */
GDK_AVAILABLE_IN_ALL
void           gtk_text_buffer_add_mark    (GtkTextBuffer     *buffer,
                                            GtkTextMark       *mark,
                                            const GtkTextIter *where);
GDK_AVAILABLE_IN_ALL
GtkTextMark   *gtk_text_buffer_create_mark (GtkTextBuffer     *buffer,
                                            const gchar       *mark_name,
                                            const GtkTextIter *where,
                                            gboolean           left_gravity);
GDK_AVAILABLE_IN_ALL
void           gtk_text_buffer_move_mark   (GtkTextBuffer     *buffer,
                                            GtkTextMark       *mark,
                                            const GtkTextIter *where);
GDK_AVAILABLE_IN_ALL
void           gtk_text_buffer_delete_mark (GtkTextBuffer     *buffer,
                                            GtkTextMark       *mark);
GDK_AVAILABLE_IN_ALL
GtkTextMark*   gtk_text_buffer_get_mark    (GtkTextBuffer     *buffer,
                                            const gchar       *name);

GDK_AVAILABLE_IN_ALL
void gtk_text_buffer_move_mark_by_name   (GtkTextBuffer     *buffer,
                                          const gchar       *name,
                                          const GtkTextIter *where);
GDK_AVAILABLE_IN_ALL
void gtk_text_buffer_delete_mark_by_name (GtkTextBuffer     *buffer,
                                          const gchar       *name);

GDK_AVAILABLE_IN_ALL
GtkTextMark* gtk_text_buffer_get_insert          (GtkTextBuffer *buffer);
GDK_AVAILABLE_IN_ALL
GtkTextMark* gtk_text_buffer_get_selection_bound (GtkTextBuffer *buffer);

/* efficiently move insert and selection_bound at the same time */
GDK_AVAILABLE_IN_ALL
void gtk_text_buffer_place_cursor (GtkTextBuffer     *buffer,
                                   const GtkTextIter *where);
GDK_AVAILABLE_IN_ALL
void gtk_text_buffer_select_range (GtkTextBuffer     *buffer,
                                   const GtkTextIter *ins,
				   const GtkTextIter *bound);



/* Tag manipulation */
GDK_AVAILABLE_IN_ALL
void gtk_text_buffer_apply_tag             (GtkTextBuffer     *buffer,
                                            GtkTextTag        *tag,
                                            const GtkTextIter *start,
                                            const GtkTextIter *end);
GDK_AVAILABLE_IN_ALL
void gtk_text_buffer_remove_tag            (GtkTextBuffer     *buffer,
                                            GtkTextTag        *tag,
                                            const GtkTextIter *start,
                                            const GtkTextIter *end);
GDK_AVAILABLE_IN_ALL
void gtk_text_buffer_apply_tag_by_name     (GtkTextBuffer     *buffer,
                                            const gchar       *name,
                                            const GtkTextIter *start,
                                            const GtkTextIter *end);
GDK_AVAILABLE_IN_ALL
void gtk_text_buffer_remove_tag_by_name    (GtkTextBuffer     *buffer,
                                            const gchar       *name,
                                            const GtkTextIter *start,
                                            const GtkTextIter *end);
GDK_AVAILABLE_IN_ALL
void gtk_text_buffer_remove_all_tags       (GtkTextBuffer     *buffer,
                                            const GtkTextIter *start,
                                            const GtkTextIter *end);


/* You can either ignore the return value, or use it to
 * set the attributes of the tag. tag_name can be NULL
 */
GDK_AVAILABLE_IN_ALL
GtkTextTag    *gtk_text_buffer_create_tag (GtkTextBuffer *buffer,
                                           const gchar   *tag_name,
                                           const gchar   *first_property_name,
                                           ...);

/* Obtain iterators pointed at various places, then you can move the
 * iterator around using the GtkTextIter operators
 */
GDK_AVAILABLE_IN_ALL
void gtk_text_buffer_get_iter_at_line_offset (GtkTextBuffer *buffer,
                                              GtkTextIter   *iter,
                                              gint           line_number,
                                              gint           char_offset);
GDK_AVAILABLE_IN_ALL
void gtk_text_buffer_get_iter_at_line_index  (GtkTextBuffer *buffer,
                                              GtkTextIter   *iter,
                                              gint           line_number,
                                              gint           byte_index);
GDK_AVAILABLE_IN_ALL
void gtk_text_buffer_get_iter_at_offset      (GtkTextBuffer *buffer,
                                              GtkTextIter   *iter,
                                              gint           char_offset);
GDK_AVAILABLE_IN_ALL
void gtk_text_buffer_get_iter_at_line        (GtkTextBuffer *buffer,
                                              GtkTextIter   *iter,
                                              gint           line_number);
GDK_AVAILABLE_IN_ALL
void gtk_text_buffer_get_start_iter          (GtkTextBuffer *buffer,
                                              GtkTextIter   *iter);
GDK_AVAILABLE_IN_ALL
void gtk_text_buffer_get_end_iter            (GtkTextBuffer *buffer,
                                              GtkTextIter   *iter);
GDK_AVAILABLE_IN_ALL
void gtk_text_buffer_get_bounds              (GtkTextBuffer *buffer,
                                              GtkTextIter   *start,
                                              GtkTextIter   *end);
GDK_AVAILABLE_IN_ALL
void gtk_text_buffer_get_iter_at_mark        (GtkTextBuffer *buffer,
                                              GtkTextIter   *iter,
                                              GtkTextMark   *mark);

GDK_AVAILABLE_IN_ALL
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

GDK_AVAILABLE_IN_ALL
gboolean        gtk_text_buffer_get_modified            (GtkTextBuffer *buffer);
GDK_AVAILABLE_IN_ALL
void            gtk_text_buffer_set_modified            (GtkTextBuffer *buffer,
                                                         gboolean       setting);

GDK_AVAILABLE_IN_ALL
gboolean        gtk_text_buffer_get_has_selection       (GtkTextBuffer *buffer);

GDK_AVAILABLE_IN_ALL
void gtk_text_buffer_add_selection_clipboard    (GtkTextBuffer     *buffer,
						 GtkClipboard      *clipboard);
GDK_AVAILABLE_IN_ALL
void gtk_text_buffer_remove_selection_clipboard (GtkTextBuffer     *buffer,
						 GtkClipboard      *clipboard);

GDK_AVAILABLE_IN_ALL
void            gtk_text_buffer_cut_clipboard           (GtkTextBuffer *buffer,
							 GtkClipboard  *clipboard,
                                                         gboolean       default_editable);
GDK_AVAILABLE_IN_ALL
void            gtk_text_buffer_copy_clipboard          (GtkTextBuffer *buffer,
							 GtkClipboard  *clipboard);
GDK_AVAILABLE_IN_ALL
void            gtk_text_buffer_paste_clipboard         (GtkTextBuffer *buffer,
							 GtkClipboard  *clipboard,
							 GtkTextIter   *override_location,
                                                         gboolean       default_editable);

GDK_AVAILABLE_IN_ALL
gboolean        gtk_text_buffer_get_selection_bounds    (GtkTextBuffer *buffer,
                                                         GtkTextIter   *start,
                                                         GtkTextIter   *end);
GDK_AVAILABLE_IN_ALL
gboolean        gtk_text_buffer_delete_selection        (GtkTextBuffer *buffer,
                                                         gboolean       interactive,
                                                         gboolean       default_editable);

/* Called to specify atomic user actions, used to implement undo */
GDK_AVAILABLE_IN_ALL
void            gtk_text_buffer_begin_user_action       (GtkTextBuffer *buffer);
GDK_AVAILABLE_IN_ALL
void            gtk_text_buffer_end_user_action         (GtkTextBuffer *buffer);

GDK_AVAILABLE_IN_ALL
GtkTargetList * gtk_text_buffer_get_copy_target_list    (GtkTextBuffer *buffer);
GDK_AVAILABLE_IN_ALL
GtkTargetList * gtk_text_buffer_get_paste_target_list   (GtkTextBuffer *buffer);


G_END_DECLS

#endif
