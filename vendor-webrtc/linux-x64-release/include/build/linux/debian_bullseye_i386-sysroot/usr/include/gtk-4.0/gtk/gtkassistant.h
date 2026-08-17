/* 
 * GTK - The GIMP Toolkit
 * Copyright (C) 1999  Red Hat, Inc.
 * Copyright (C) 2002  Anders Carlsson <andersca@gnu.org>
 * Copyright (C) 2003  Matthias Clasen <mclasen@redhat.com>
 * Copyright (C) 2005  Carlos Garnacho Parro <carlosg@gnome.org>
 *
 * All rights reserved.
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

#ifndef __GTK_ASSISTANT_H__
#define __GTK_ASSISTANT_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwindow.h>

G_BEGIN_DECLS

#define GTK_TYPE_ASSISTANT         (gtk_assistant_get_type ())
#define GTK_ASSISTANT(o)           (G_TYPE_CHECK_INSTANCE_CAST ((o), GTK_TYPE_ASSISTANT, GtkAssistant))
#define GTK_IS_ASSISTANT(o)        (G_TYPE_CHECK_INSTANCE_TYPE ((o), GTK_TYPE_ASSISTANT))

/**
 * GtkAssistantPageType:
 * @GTK_ASSISTANT_PAGE_CONTENT: The page has regular contents. Both the
 *  Back and forward buttons will be shown.
 * @GTK_ASSISTANT_PAGE_INTRO: The page contains an introduction to the
 *  assistant task. Only the Forward button will be shown if there is a
 *   next page.
 * @GTK_ASSISTANT_PAGE_CONFIRM: The page lets the user confirm or deny the
 *  changes. The Back and Apply buttons will be shown.
 * @GTK_ASSISTANT_PAGE_SUMMARY: The page informs the user of the changes
 *  done. Only the Close button will be shown.
 * @GTK_ASSISTANT_PAGE_PROGRESS: Used for tasks that take a long time to
 *  complete, blocks the assistant until the page is marked as complete.
 *   Only the back button will be shown.
 * @GTK_ASSISTANT_PAGE_CUSTOM: Used for when other page types are not
 *  appropriate. No buttons will be shown, and the application must
 *  add its own buttons through gtk_assistant_add_action_widget().
 *
 * Determines the page role inside a `GtkAssistant`.
 *
 * The role is used to handle buttons sensitivity and visibility.
 *
 * Note that an assistant needs to end its page flow with a page of type
 * %GTK_ASSISTANT_PAGE_CONFIRM, %GTK_ASSISTANT_PAGE_SUMMARY or
 * %GTK_ASSISTANT_PAGE_PROGRESS to be correct.
 *
 * The Cancel button will only be shown if the page isn’t “committed”.
 * See gtk_assistant_commit() for details.
 */
typedef enum
{
  GTK_ASSISTANT_PAGE_CONTENT,
  GTK_ASSISTANT_PAGE_INTRO,
  GTK_ASSISTANT_PAGE_CONFIRM,
  GTK_ASSISTANT_PAGE_SUMMARY,
  GTK_ASSISTANT_PAGE_PROGRESS,
  GTK_ASSISTANT_PAGE_CUSTOM
} GtkAssistantPageType;

typedef struct _GtkAssistant        GtkAssistant;

#define GTK_TYPE_ASSISTANT_PAGE (gtk_assistant_page_get_type ())
#define GTK_ASSISTANT_PAGE(obj) (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_ASSISTANT_PAGE, GtkAssistantPage))
#define GTK_IS_ASSISTANT_PAGE(obj) (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_ASSISTANT_PAGE))

typedef struct _GtkAssistantPage GtkAssistantPage;

/**
 * GtkAssistantPageFunc:
 * @current_page: The page number used to calculate the next page.
 * @data: (closure): user data.
 *
 * Type of callback used to calculate the next page in a `GtkAssistant`.
 *
 * It’s called both for computing the next page when the user presses the
 * “forward” button and for handling the behavior of the “last” button.
 *
 * See [method@Gtk.Assistant.set_forward_page_func].
 *
 * Returns: The next page number
 */
typedef int (*GtkAssistantPageFunc) (int current_page, gpointer data);

GDK_AVAILABLE_IN_ALL
GType                 gtk_assistant_page_get_type         (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GType                 gtk_assistant_get_type              (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkWidget            *gtk_assistant_new                   (void);
GDK_AVAILABLE_IN_ALL
void                  gtk_assistant_next_page             (GtkAssistant         *assistant);
GDK_AVAILABLE_IN_ALL
void                  gtk_assistant_previous_page         (GtkAssistant         *assistant);
GDK_AVAILABLE_IN_ALL
int                   gtk_assistant_get_current_page      (GtkAssistant         *assistant);
GDK_AVAILABLE_IN_ALL
void                  gtk_assistant_set_current_page      (GtkAssistant         *assistant,
                                                           int                   page_num);
GDK_AVAILABLE_IN_ALL
int                   gtk_assistant_get_n_pages           (GtkAssistant         *assistant);
GDK_AVAILABLE_IN_ALL
GtkWidget            *gtk_assistant_get_nth_page          (GtkAssistant         *assistant,
                                                           int                   page_num);
GDK_AVAILABLE_IN_ALL
int                   gtk_assistant_prepend_page          (GtkAssistant         *assistant,
                                                           GtkWidget            *page);
GDK_AVAILABLE_IN_ALL
int                   gtk_assistant_append_page           (GtkAssistant         *assistant,
                                                           GtkWidget            *page);
GDK_AVAILABLE_IN_ALL
int                   gtk_assistant_insert_page           (GtkAssistant         *assistant,
                                                           GtkWidget            *page,
                                                           int                   position);
GDK_AVAILABLE_IN_ALL
void                  gtk_assistant_remove_page           (GtkAssistant         *assistant,
                                                           int                   page_num);
GDK_AVAILABLE_IN_ALL
void                  gtk_assistant_set_forward_page_func (GtkAssistant         *assistant,
                                                           GtkAssistantPageFunc  page_func,
                                                           gpointer              data,
                                                           GDestroyNotify        destroy);
GDK_AVAILABLE_IN_ALL
void                  gtk_assistant_set_page_type         (GtkAssistant         *assistant,
                                                           GtkWidget            *page,
                                                           GtkAssistantPageType  type);
GDK_AVAILABLE_IN_ALL
GtkAssistantPageType  gtk_assistant_get_page_type         (GtkAssistant         *assistant,
                                                           GtkWidget            *page);
GDK_AVAILABLE_IN_ALL
void                  gtk_assistant_set_page_title        (GtkAssistant         *assistant,
                                                           GtkWidget            *page,
                                                           const char           *title);
GDK_AVAILABLE_IN_ALL
const char *         gtk_assistant_get_page_title        (GtkAssistant         *assistant,
                                                           GtkWidget            *page);

GDK_AVAILABLE_IN_ALL
void                  gtk_assistant_set_page_complete     (GtkAssistant         *assistant,
                                                           GtkWidget            *page,
                                                           gboolean              complete);
GDK_AVAILABLE_IN_ALL
gboolean              gtk_assistant_get_page_complete     (GtkAssistant         *assistant,
                                                           GtkWidget            *page);
GDK_AVAILABLE_IN_ALL
void                  gtk_assistant_add_action_widget     (GtkAssistant         *assistant,
                                                           GtkWidget            *child);
GDK_AVAILABLE_IN_ALL
void                  gtk_assistant_remove_action_widget  (GtkAssistant         *assistant,
                                                           GtkWidget            *child);

GDK_AVAILABLE_IN_ALL
void                  gtk_assistant_update_buttons_state  (GtkAssistant *assistant);
GDK_AVAILABLE_IN_ALL
void                  gtk_assistant_commit                (GtkAssistant *assistant);

GDK_AVAILABLE_IN_ALL
GtkAssistantPage *    gtk_assistant_get_page       (GtkAssistant     *assistant,
                                                    GtkWidget        *child);
GDK_AVAILABLE_IN_ALL
GtkWidget *           gtk_assistant_page_get_child (GtkAssistantPage *page);

GDK_AVAILABLE_IN_ALL
GListModel *          gtk_assistant_get_pages (GtkAssistant *assistant);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkAssistant, g_object_unref)

G_END_DECLS

#endif /* __GTK_ASSISTANT_H__ */
