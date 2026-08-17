/* GTK - The GIMP Toolkit
 * Copyright (C) 1995-1997 Peter Mattis, Spencer Kimball and Josh MacDonald
 *
 * Copyright (C) 2004-2006 Christian Hammond
 * Copyright (C) 2008 Cody Russell
 * Copyright (C) 2008 Red Hat, Inc.
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

#ifndef __GTK_ENTRY_H__
#define __GTK_ENTRY_H__


#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkeditable.h>
#include <gtk/gtkimcontext.h>
#include <gtk/gtkentrybuffer.h>
#include <gtk/gtkentrycompletion.h>
#include <gtk/gtkimage.h>


G_BEGIN_DECLS

#define GTK_TYPE_ENTRY                  (gtk_entry_get_type ())
#define GTK_ENTRY(obj)                  (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_ENTRY, GtkEntry))
#define GTK_ENTRY_CLASS(klass)          (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_ENTRY, GtkEntryClass))
#define GTK_IS_ENTRY(obj)               (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_ENTRY))
#define GTK_IS_ENTRY_CLASS(klass)       (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_ENTRY))
#define GTK_ENTRY_GET_CLASS(obj)        (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_ENTRY, GtkEntryClass))

/**
 * GtkEntryIconPosition:
 * @GTK_ENTRY_ICON_PRIMARY: At the beginning of the entry (depending on the text direction).
 * @GTK_ENTRY_ICON_SECONDARY: At the end of the entry (depending on the text direction).
 *
 * Specifies the side of the entry at which an icon is placed.
 */
typedef enum
{
  GTK_ENTRY_ICON_PRIMARY,
  GTK_ENTRY_ICON_SECONDARY
} GtkEntryIconPosition;

typedef struct _GtkEntry              GtkEntry;
typedef struct _GtkEntryClass         GtkEntryClass;

struct _GtkEntry
{
  /*< private >*/
  GtkWidget  parent_instance;
};

/**
 * GtkEntryClass:
 * @parent_class: The parent class.
 * @activate: Class handler for the `GtkEntry::activate` signal. The default
 *   implementation activates the gtk.activate-default action.
 *
 * Class structure for `GtkEntry`. All virtual functions have a default
 * implementation. Derived classes may set the virtual function pointers for the
 * signal handlers to %NULL, but must keep @get_text_area_size and
 * @get_frame_size non-%NULL; either use the default implementation, or provide
 * a custom one.
 */
struct _GtkEntryClass
{
  GtkWidgetClass parent_class;

  /* Action signals
   */
  void (* activate)           (GtkEntry             *entry);

  /*< private >*/

  gpointer padding[8];
};

GDK_AVAILABLE_IN_ALL
GType      gtk_entry_get_type       		(void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkWidget* gtk_entry_new            		(void);
GDK_AVAILABLE_IN_ALL
GtkWidget* gtk_entry_new_with_buffer            (GtkEntryBuffer *buffer);

GDK_AVAILABLE_IN_ALL
GtkEntryBuffer* gtk_entry_get_buffer            (GtkEntry       *entry);
GDK_AVAILABLE_IN_ALL
void       gtk_entry_set_buffer                 (GtkEntry       *entry,
                                                 GtkEntryBuffer *buffer);

GDK_AVAILABLE_IN_ALL
void       gtk_entry_set_visibility 		(GtkEntry      *entry,
						 gboolean       visible);
GDK_AVAILABLE_IN_ALL
gboolean   gtk_entry_get_visibility             (GtkEntry      *entry);

GDK_AVAILABLE_IN_ALL
void       gtk_entry_set_invisible_char         (GtkEntry      *entry,
                                                 gunichar       ch);
GDK_AVAILABLE_IN_ALL
gunichar   gtk_entry_get_invisible_char         (GtkEntry      *entry);
GDK_AVAILABLE_IN_ALL
void       gtk_entry_unset_invisible_char       (GtkEntry      *entry);

GDK_AVAILABLE_IN_ALL
void       gtk_entry_set_has_frame              (GtkEntry      *entry,
                                                 gboolean       setting);
GDK_AVAILABLE_IN_ALL
gboolean   gtk_entry_get_has_frame              (GtkEntry      *entry);

GDK_AVAILABLE_IN_ALL
void       gtk_entry_set_overwrite_mode         (GtkEntry      *entry,
                                                 gboolean       overwrite);
GDK_AVAILABLE_IN_ALL
gboolean   gtk_entry_get_overwrite_mode         (GtkEntry      *entry);

/* text is truncated if needed */
GDK_AVAILABLE_IN_ALL
void       gtk_entry_set_max_length 		(GtkEntry      *entry,
						 int            max);
GDK_AVAILABLE_IN_ALL
int        gtk_entry_get_max_length             (GtkEntry      *entry);
GDK_AVAILABLE_IN_ALL
guint16    gtk_entry_get_text_length            (GtkEntry      *entry);

GDK_AVAILABLE_IN_ALL
void       gtk_entry_set_activates_default      (GtkEntry      *entry,
                                                 gboolean       setting);
GDK_AVAILABLE_IN_ALL
gboolean   gtk_entry_get_activates_default      (GtkEntry      *entry);

GDK_AVAILABLE_IN_ALL
void       gtk_entry_set_alignment              (GtkEntry      *entry,
                                                 float          xalign);
GDK_AVAILABLE_IN_ALL
float      gtk_entry_get_alignment              (GtkEntry      *entry);

GDK_AVAILABLE_IN_ALL
void                gtk_entry_set_completion (GtkEntry           *entry,
                                              GtkEntryCompletion *completion);
GDK_AVAILABLE_IN_ALL
GtkEntryCompletion *gtk_entry_get_completion (GtkEntry           *entry);

/* Progress API
 */
GDK_AVAILABLE_IN_ALL
void           gtk_entry_set_progress_fraction   (GtkEntry     *entry,
                                                  double        fraction);
GDK_AVAILABLE_IN_ALL
double         gtk_entry_get_progress_fraction   (GtkEntry     *entry);

GDK_AVAILABLE_IN_ALL
void           gtk_entry_set_progress_pulse_step (GtkEntry     *entry,
                                                  double        fraction);
GDK_AVAILABLE_IN_ALL
double         gtk_entry_get_progress_pulse_step (GtkEntry     *entry);

GDK_AVAILABLE_IN_ALL
void           gtk_entry_progress_pulse          (GtkEntry     *entry);
GDK_AVAILABLE_IN_ALL
const char *   gtk_entry_get_placeholder_text    (GtkEntry             *entry);
GDK_AVAILABLE_IN_ALL
void           gtk_entry_set_placeholder_text    (GtkEntry             *entry,
                                                  const char           *text);
/* Setting and managing icons
 */
GDK_AVAILABLE_IN_ALL
void           gtk_entry_set_icon_from_paintable         (GtkEntry             *entry,
							  GtkEntryIconPosition  icon_pos,
							  GdkPaintable         *paintable);
GDK_AVAILABLE_IN_ALL
void           gtk_entry_set_icon_from_icon_name         (GtkEntry             *entry,
							  GtkEntryIconPosition  icon_pos,
							  const char           *icon_name);
GDK_AVAILABLE_IN_ALL
void           gtk_entry_set_icon_from_gicon             (GtkEntry             *entry,
							  GtkEntryIconPosition  icon_pos,
							  GIcon                *icon);
GDK_AVAILABLE_IN_ALL
GtkImageType   gtk_entry_get_icon_storage_type           (GtkEntry             *entry,
							  GtkEntryIconPosition  icon_pos);
GDK_AVAILABLE_IN_ALL
GdkPaintable * gtk_entry_get_icon_paintable              (GtkEntry             *entry,
							  GtkEntryIconPosition  icon_pos);
GDK_AVAILABLE_IN_ALL
const char * gtk_entry_get_icon_name                     (GtkEntry             *entry,
							  GtkEntryIconPosition  icon_pos);
GDK_AVAILABLE_IN_ALL
GIcon*       gtk_entry_get_icon_gicon                    (GtkEntry             *entry,
							  GtkEntryIconPosition  icon_pos);
GDK_AVAILABLE_IN_ALL
void         gtk_entry_set_icon_activatable              (GtkEntry             *entry,
							  GtkEntryIconPosition  icon_pos,
							  gboolean              activatable);
GDK_AVAILABLE_IN_ALL
gboolean     gtk_entry_get_icon_activatable              (GtkEntry             *entry,
							  GtkEntryIconPosition  icon_pos);
GDK_AVAILABLE_IN_ALL
void         gtk_entry_set_icon_sensitive                (GtkEntry             *entry,
							  GtkEntryIconPosition  icon_pos,
							  gboolean              sensitive);
GDK_AVAILABLE_IN_ALL
gboolean     gtk_entry_get_icon_sensitive                (GtkEntry             *entry,
							  GtkEntryIconPosition  icon_pos);
GDK_AVAILABLE_IN_ALL
int          gtk_entry_get_icon_at_pos                   (GtkEntry             *entry,
							  int                   x,
							  int                   y);
GDK_AVAILABLE_IN_ALL
void         gtk_entry_set_icon_tooltip_text             (GtkEntry             *entry,
							  GtkEntryIconPosition  icon_pos,
							  const char           *tooltip);
GDK_AVAILABLE_IN_ALL
char *      gtk_entry_get_icon_tooltip_text             (GtkEntry             *entry,
                                                          GtkEntryIconPosition  icon_pos);
GDK_AVAILABLE_IN_ALL
void         gtk_entry_set_icon_tooltip_markup           (GtkEntry             *entry,
							  GtkEntryIconPosition  icon_pos,
							  const char           *tooltip);
GDK_AVAILABLE_IN_ALL
char *      gtk_entry_get_icon_tooltip_markup           (GtkEntry             *entry,
                                                          GtkEntryIconPosition  icon_pos);
GDK_AVAILABLE_IN_ALL
void         gtk_entry_set_icon_drag_source              (GtkEntry             *entry,
							  GtkEntryIconPosition  icon_pos,
							  GdkContentProvider   *provider,
							  GdkDragAction         actions);
GDK_AVAILABLE_IN_ALL
int          gtk_entry_get_current_icon_drag_source      (GtkEntry             *entry);
GDK_AVAILABLE_IN_ALL
void         gtk_entry_get_icon_area                     (GtkEntry             *entry,
                                                          GtkEntryIconPosition  icon_pos,
                                                          GdkRectangle         *icon_area);

GDK_AVAILABLE_IN_ALL
void        gtk_entry_reset_im_context                   (GtkEntry             *entry);

GDK_AVAILABLE_IN_ALL
void            gtk_entry_set_input_purpose                  (GtkEntry             *entry,
                                                              GtkInputPurpose       purpose);
GDK_AVAILABLE_IN_ALL
GtkInputPurpose gtk_entry_get_input_purpose                  (GtkEntry             *entry);

GDK_AVAILABLE_IN_ALL
void            gtk_entry_set_input_hints                    (GtkEntry             *entry,
                                                              GtkInputHints         hints);
GDK_AVAILABLE_IN_ALL
GtkInputHints   gtk_entry_get_input_hints                    (GtkEntry             *entry);

GDK_AVAILABLE_IN_ALL
void            gtk_entry_set_attributes                     (GtkEntry             *entry,
                                                              PangoAttrList        *attrs);
GDK_AVAILABLE_IN_ALL
PangoAttrList  *gtk_entry_get_attributes                     (GtkEntry             *entry);

GDK_AVAILABLE_IN_ALL
void            gtk_entry_set_tabs                           (GtkEntry             *entry,
                                                              PangoTabArray        *tabs);

GDK_AVAILABLE_IN_ALL
PangoTabArray  *gtk_entry_get_tabs                           (GtkEntry             *entry);

GDK_AVAILABLE_IN_ALL
gboolean       gtk_entry_grab_focus_without_selecting        (GtkEntry             *entry);

GDK_AVAILABLE_IN_ALL
void           gtk_entry_set_extra_menu                      (GtkEntry             *entry,
                                                              GMenuModel           *model);
GDK_AVAILABLE_IN_ALL
GMenuModel *   gtk_entry_get_extra_menu                      (GtkEntry             *entry);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkEntry, g_object_unref)

G_END_DECLS

#endif /* __GTK_ENTRY_H__ */
