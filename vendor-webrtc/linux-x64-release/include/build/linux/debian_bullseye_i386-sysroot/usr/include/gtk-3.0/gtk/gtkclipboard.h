/* GTK - The GIMP Toolkit
 * Copyright (C) 2000 Red Hat, Inc.
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
 *
 * Global clipboard abstraction.
 */

#ifndef __GTK_CLIPBOARD_H__
#define __GTK_CLIPBOARD_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkselection.h>

G_BEGIN_DECLS

#define GTK_TYPE_CLIPBOARD            (gtk_clipboard_get_type ())
#define GTK_CLIPBOARD(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_CLIPBOARD, GtkClipboard))
#define GTK_IS_CLIPBOARD(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_CLIPBOARD))

/**
 * GtkClipboardReceivedFunc:
 * @clipboard: the #GtkClipboard
 * @selection_data: a #GtkSelectionData containing the data was received.
 *   If retrieving the data failed, then then length field
 *   of @selection_data will be negative.
 * @data: (closure): the @user_data supplied to
 *   gtk_clipboard_request_contents().
 *
 * A function to be called when the results of gtk_clipboard_request_contents()
 * are received, or when the request fails.
 */
typedef void (* GtkClipboardReceivedFunc)         (GtkClipboard     *clipboard,
					           GtkSelectionData *selection_data,
					           gpointer          data);

/**
 * GtkClipboardTextReceivedFunc:
 * @clipboard: the #GtkClipboard
 * @text: (nullable): the text received, as a UTF-8 encoded string, or
 *   %NULL if retrieving the data failed.
 * @data: (closure): the @user_data supplied to
 *   gtk_clipboard_request_text().
 *
 * A function to be called when the results of gtk_clipboard_request_text()
 * are received, or when the request fails.
 */
typedef void (* GtkClipboardTextReceivedFunc)     (GtkClipboard     *clipboard,
					           const gchar      *text,
					           gpointer          data);

/**
 * GtkClipboardRichTextReceivedFunc:
 * @clipboard: the #GtkClipboard
 * @format: The format of the rich text
 * @text: (nullable) (type utf8): the rich text received, as
 *   a UTF-8 encoded string, or %NULL if retrieving the data failed.
 * @length: Length of the text.
 * @data: (closure): the @user_data supplied to
 *   gtk_clipboard_request_rich_text().
 *
 * A function to be called when the results of
 * gtk_clipboard_request_rich_text() are received, or when the request
 * fails.
 *
 * Since: 2.10
 */
typedef void (* GtkClipboardRichTextReceivedFunc) (GtkClipboard     *clipboard,
                                                   GdkAtom           format,
					           const guint8     *text,
                                                   gsize             length,
					           gpointer          data);

/**
 * GtkClipboardImageReceivedFunc:
 * @clipboard: the #GtkClipboard
 * @pixbuf: the received image
 * @data: (closure): the @user_data supplied to
 *   gtk_clipboard_request_image().
 *
 * A function to be called when the results of gtk_clipboard_request_image()
 * are received, or when the request fails.
 *
 * Since: 2.6
 */
typedef void (* GtkClipboardImageReceivedFunc)    (GtkClipboard     *clipboard,
						   GdkPixbuf        *pixbuf,
						   gpointer          data);

/**
 * GtkClipboardURIReceivedFunc:
 * @clipboard: the #GtkClipboard
 * @uris: (array zero-terminated=1): the received URIs
 * @data: (closure): the @user_data supplied to
 *   gtk_clipboard_request_uris().
 *
 * A function to be called when the results of
 * gtk_clipboard_request_uris() are received, or when the request
 * fails.
 *
 * Since: 2.14
 */
typedef void (* GtkClipboardURIReceivedFunc)      (GtkClipboard     *clipboard,
						   gchar           **uris,
						   gpointer          data);

/**
 * GtkClipboardTargetsReceivedFunc:
 * @clipboard: the #GtkClipboard
 * @atoms: (nullable) (array length=n_atoms): the supported targets,
 *   as array of #GdkAtom, or %NULL if retrieving the data failed.
 * @n_atoms: the length of the @atoms array.
 * @data: (closure): the @user_data supplied to
 *   gtk_clipboard_request_targets().
 *
 * A function to be called when the results of gtk_clipboard_request_targets()
 * are received, or when the request fails.
 *
 * Since: 2.4
 */
typedef void (* GtkClipboardTargetsReceivedFunc)  (GtkClipboard     *clipboard,
					           GdkAtom          *atoms,
						   gint              n_atoms,
					           gpointer          data);

/* Should these functions have GtkClipboard *clipboard as the first argument?
 * right now for ClearFunc, you may have trouble determining _which_ clipboard
 * was cleared, if you reuse your ClearFunc for multiple clipboards.
 */
/**
 * GtkClipboardGetFunc:
 * @clipboard: the #GtkClipboard
 * @selection_data: a #GtkSelectionData argument in which the requested
 *   data should be stored.
 * @info: the info field corresponding to the requested target from the
 *   #GtkTargetEntry array passed to gtk_clipboard_set_with_data() or
 *   gtk_clipboard_set_with_owner().
 * @user_data_or_owner: the @user_data argument passed to
 *   gtk_clipboard_set_with_data(), or the @owner argument passed to
 *   gtk_clipboard_set_with_owner()
 *
 * A function that will be called to provide the contents of the selection.
 * If multiple types of data were advertised, the requested type can
 * be determined from the @info parameter or by checking the target field
 * of @selection_data. If the data could successfully be converted into
 * then it should be stored into the @selection_data object by
 * calling gtk_selection_data_set() (or related functions such
 * as gtk_selection_data_set_text()). If no data is set, the requestor
 * will be informed that the attempt to get the data failed.
 */
typedef void (* GtkClipboardGetFunc)          (GtkClipboard     *clipboard,
					       GtkSelectionData *selection_data,
					       guint             info,
					       gpointer          user_data_or_owner);

/**
 * GtkClipboardClearFunc:
 * @clipboard: the #GtkClipboard
 * @user_data_or_owner: the @user_data argument passed to gtk_clipboard_set_with_data(),
 *   or the @owner argument passed to gtk_clipboard_set_with_owner()
 *
 * A function that will be called when the contents of the clipboard are changed
 * or cleared. Once this has called, the @user_data_or_owner argument
 * will not be used again.
 */
typedef void (* GtkClipboardClearFunc)        (GtkClipboard     *clipboard,
					       gpointer          user_data_or_owner);

GDK_AVAILABLE_IN_ALL
GType         gtk_clipboard_get_type (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
GtkClipboard *gtk_clipboard_get_for_display (GdkDisplay   *display,
					     GdkAtom       selection);
GDK_AVAILABLE_IN_ALL
GtkClipboard *gtk_clipboard_get             (GdkAtom       selection);

GDK_AVAILABLE_IN_3_16
GtkClipboard *gtk_clipboard_get_default     (GdkDisplay    *display);

GDK_AVAILABLE_IN_ALL
GdkDisplay   *gtk_clipboard_get_display     (GtkClipboard *clipboard);


GDK_AVAILABLE_IN_ALL
gboolean gtk_clipboard_set_with_data  (GtkClipboard          *clipboard,
				       const GtkTargetEntry  *targets,
				       guint                  n_targets,
				       GtkClipboardGetFunc    get_func,
				       GtkClipboardClearFunc  clear_func,
				       gpointer               user_data);
GDK_AVAILABLE_IN_ALL
gboolean gtk_clipboard_set_with_owner (GtkClipboard          *clipboard,
				       const GtkTargetEntry  *targets,
				       guint                  n_targets,
				       GtkClipboardGetFunc    get_func,
				       GtkClipboardClearFunc  clear_func,
				       GObject               *owner);
GDK_AVAILABLE_IN_ALL
GObject *gtk_clipboard_get_owner      (GtkClipboard          *clipboard);
GDK_AVAILABLE_IN_ALL
void     gtk_clipboard_clear          (GtkClipboard          *clipboard);
GDK_AVAILABLE_IN_ALL
void     gtk_clipboard_set_text       (GtkClipboard          *clipboard,
				       const gchar           *text,
				       gint                   len);
GDK_AVAILABLE_IN_ALL
void     gtk_clipboard_set_image      (GtkClipboard          *clipboard,
				       GdkPixbuf             *pixbuf);

GDK_AVAILABLE_IN_ALL
void gtk_clipboard_request_contents  (GtkClipboard                     *clipboard,
                                      GdkAtom                           target,
                                      GtkClipboardReceivedFunc          callback,
                                      gpointer                          user_data);
GDK_AVAILABLE_IN_ALL
void gtk_clipboard_request_text      (GtkClipboard                     *clipboard,
                                      GtkClipboardTextReceivedFunc      callback,
                                      gpointer                          user_data);
GDK_AVAILABLE_IN_ALL
void gtk_clipboard_request_rich_text (GtkClipboard                     *clipboard,
                                      GtkTextBuffer                    *buffer,
                                      GtkClipboardRichTextReceivedFunc  callback,
                                      gpointer                          user_data);
GDK_AVAILABLE_IN_ALL
void gtk_clipboard_request_image     (GtkClipboard                     *clipboard,
                                      GtkClipboardImageReceivedFunc     callback,
                                      gpointer                          user_data);
GDK_AVAILABLE_IN_ALL
void gtk_clipboard_request_uris      (GtkClipboard                     *clipboard,
                                      GtkClipboardURIReceivedFunc       callback,
                                      gpointer                          user_data);
GDK_AVAILABLE_IN_ALL
void gtk_clipboard_request_targets   (GtkClipboard                     *clipboard,
                                      GtkClipboardTargetsReceivedFunc   callback,
                                      gpointer                          user_data);

GDK_AVAILABLE_IN_ALL
GtkSelectionData *gtk_clipboard_wait_for_contents  (GtkClipboard  *clipboard,
                                                    GdkAtom        target);
GDK_AVAILABLE_IN_ALL
gchar *           gtk_clipboard_wait_for_text      (GtkClipboard  *clipboard);
GDK_AVAILABLE_IN_ALL
guint8 *          gtk_clipboard_wait_for_rich_text (GtkClipboard  *clipboard,
                                                    GtkTextBuffer *buffer,
                                                    GdkAtom       *format,
                                                    gsize         *length);
GDK_AVAILABLE_IN_ALL
GdkPixbuf *       gtk_clipboard_wait_for_image     (GtkClipboard  *clipboard);
GDK_AVAILABLE_IN_ALL
gchar **          gtk_clipboard_wait_for_uris      (GtkClipboard  *clipboard);
GDK_AVAILABLE_IN_ALL
gboolean          gtk_clipboard_wait_for_targets   (GtkClipboard  *clipboard,
                                                    GdkAtom      **targets,
                                                    gint          *n_targets);

GDK_AVAILABLE_IN_ALL
gboolean gtk_clipboard_wait_is_text_available      (GtkClipboard  *clipboard);
GDK_AVAILABLE_IN_ALL
gboolean gtk_clipboard_wait_is_rich_text_available (GtkClipboard  *clipboard,
                                                    GtkTextBuffer *buffer);
GDK_AVAILABLE_IN_ALL
gboolean gtk_clipboard_wait_is_image_available     (GtkClipboard  *clipboard);
GDK_AVAILABLE_IN_ALL
gboolean gtk_clipboard_wait_is_uris_available      (GtkClipboard  *clipboard);
GDK_AVAILABLE_IN_ALL
gboolean gtk_clipboard_wait_is_target_available    (GtkClipboard  *clipboard,
                                                    GdkAtom        target);


GDK_AVAILABLE_IN_ALL
void gtk_clipboard_set_can_store (GtkClipboard         *clipboard,
				  const GtkTargetEntry *targets,
				  gint                  n_targets);

GDK_AVAILABLE_IN_ALL
void gtk_clipboard_store         (GtkClipboard   *clipboard);

GDK_AVAILABLE_IN_3_22
GdkAtom gtk_clipboard_get_selection (GtkClipboard *clipboard);

G_END_DECLS

#endif /* __GTK_CLIPBOARD_H__ */
