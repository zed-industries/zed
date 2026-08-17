/* gtkemojichooser.h: An Emoji chooser widget
 * Copyright 2017, Red Hat, Inc.
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

#pragma once

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwidget.h>

G_BEGIN_DECLS

#define GTK_TYPE_EMOJI_CHOOSER                 (gtk_emoji_chooser_get_type ())
#define GTK_EMOJI_CHOOSER(obj)                 (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_EMOJI_CHOOSER, GtkEmojiChooser))
#define GTK_EMOJI_CHOOSER_CLASS(klass)         (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_EMOJI_CHOOSER, GtkEmojiChooserClass))
#define GTK_IS_EMOJI_CHOOSER(obj)              (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_EMOJI_CHOOSER))
#define GTK_IS_EMOJI_CHOOSER_CLASS(klass)      (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_EMOJI_CHOOSER))
#define GTK_EMOJI_CHOOSER_GET_CLASS(obj)       (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_EMOJI_CHOOSER, GtkEmojiChooserClass))

typedef struct _GtkEmojiChooser      GtkEmojiChooser;
typedef struct _GtkEmojiChooserClass GtkEmojiChooserClass;

GDK_AVAILABLE_IN_ALL
GType      gtk_emoji_chooser_get_type (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkWidget *gtk_emoji_chooser_new      (void);

G_END_DECLS
