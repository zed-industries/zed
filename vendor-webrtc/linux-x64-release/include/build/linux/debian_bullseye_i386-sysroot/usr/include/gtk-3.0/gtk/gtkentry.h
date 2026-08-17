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
#include <gtk/gtkmenu.h>
#include <gtk/gtkentrybuffer.h>
#include <gtk/gtkentrycompletion.h>
#include <gtk/gtkimage.h>
#include <gtk/gtkselection.h>


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
 *
 * Since: 2.16
 */
typedef enum
{
  GTK_ENTRY_ICON_PRIMARY,
  GTK_ENTRY_ICON_SECONDARY
} GtkEntryIconPosition;

typedef struct _GtkEntry              GtkEntry;
typedef struct _GtkEntryPrivate       GtkEntryPrivate;
typedef struct _GtkEntryClass         GtkEntryClass;

struct _GtkEntry
{
  /*< private >*/
  GtkWidget  parent_instance;

  GtkEntryPrivate *priv;
};

/**
 * GtkEntryClass:
 * @parent_class: The parent class.
 * @populate_popup: Class handler for the #GtkEntry::populate-popup signal. If
 *   non-%NULL, this will be called to add additional entries to the context
 *   menu when it is displayed.
 * @activate: Class handler for the #GtkEntry::activate signal. The default
 *   implementation calls gtk_window_activate_default() on the entry’s top-level
 *   window.
 * @move_cursor: Class handler for the #GtkEntry::move-cursor signal. The
 *   default implementation specifies the standard #GtkEntry cursor movement
 *   behavior.
 * @insert_at_cursor: Class handler for the #GtkEntry::insert-at-cursor signal.
 *   The default implementation inserts text at the cursor.
 * @delete_from_cursor: Class handler for the #GtkEntry::delete-from-cursor
 *   signal. The default implementation deletes the selection or the specified
 *   number of characters or words.
 * @backspace: Class handler for the #GtkEntry::backspace signal. The default
 *   implementation deletes the selection or a single character or word.
 * @cut_clipboard: Class handler for the #GtkEntry::cut-clipboard signal. The
 *   default implementation cuts the selection, if one exists.
 * @copy_clipboard: Class handler for the #GtkEntry::copy-clipboard signal. The
 *   default implementation copies the selection, if one exists.
 * @paste_clipboard: Class handler for the #GtkEntry::paste-clipboard signal.
 *   The default implementation pastes at the current cursor position or over
 *   the current selection if one exists.
 * @toggle_overwrite: Class handler for the #GtkEntry::toggle-overwrite signal.
 *   The default implementation toggles overwrite mode and blinks the cursor.
 * @get_text_area_size: Calculate the size of the text area, which is its
 *   allocated width and requested height, minus space for margins and borders.
 *   This virtual function must be non-%NULL.
 * @get_frame_size: Calculate the size of the text area frame, which is its
 *   allocated width and requested height, minus space for margins and borders,
 *   and taking baseline and text height into account. This virtual function
 *   must be non-%NULL.
 *
 * Class structure for #GtkEntry. All virtual functions have a default
 * implementation. Derived classes may set the virtual function pointers for the
 * signal handlers to %NULL, but must keep @get_text_area_size and
 * @get_frame_size non-%NULL; either use the default implementation, or provide
 * a custom one.
 */
struct _GtkEntryClass
{
  GtkWidgetClass parent_class;

  /* Hook to customize right-click popup */
  void (* populate_popup)   (GtkEntry       *entry,
                             GtkWidget      *popup);

  /* Action signals
   */
  void (* activate)           (GtkEntry             *entry);
  void (* move_cursor)        (GtkEntry             *entry,
			       GtkMovementStep       step,
			       gint                  count,
			       gboolean              extend_selection);
  void (* insert_at_cursor)   (GtkEntry             *entry,
			       const gchar          *str);
  void (* delete_from_cursor) (GtkEntry             *entry,
			       GtkDeleteType         type,
			       gint                  count);
  void (* backspace)          (GtkEntry             *entry);
  void (* cut_clipboard)      (GtkEntry             *entry);
  void (* copy_clipboard)     (GtkEntry             *entry);
  void (* paste_clipboard)    (GtkEntry             *entry);
  void (* toggle_overwrite)   (GtkEntry             *entry);

  /* hooks to add other objects beside the entry (like in GtkSpinButton) */
  void (* get_text_area_size) (GtkEntry       *entry,
			       gint           *x,
			       gint           *y,
			       gint           *width,
			       gint           *height);
  void (* get_frame_size)     (GtkEntry       *entry,
                               gint           *x,
                               gint           *y,
			       gint           *width,
			       gint           *height);
  void (* insert_emoji)       (GtkEntry             *entry);

  /*< private >*/

  /* Padding for future expansion */
  void (*_gtk_reserved1)      (void);
  void (*_gtk_reserved2)      (void);
  void (*_gtk_reserved3)      (void);
  void (*_gtk_reserved4)      (void);
  void (*_gtk_reserved5)      (void);
  void (*_gtk_reserved6)      (void);
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
void       gtk_entry_get_text_area              (GtkEntry       *entry,
                                                 GdkRectangle   *text_area);

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

GDK_DEPRECATED_IN_3_4
void             gtk_entry_set_inner_border     (GtkEntry        *entry,
                                                 const GtkBorder *border);
GDK_DEPRECATED_IN_3_4
const GtkBorder* gtk_entry_get_inner_border     (GtkEntry        *entry);

GDK_AVAILABLE_IN_ALL
void       gtk_entry_set_overwrite_mode         (GtkEntry      *entry,
                                                 gboolean       overwrite);
GDK_AVAILABLE_IN_ALL
gboolean   gtk_entry_get_overwrite_mode         (GtkEntry      *entry);

/* text is truncated if needed */
GDK_AVAILABLE_IN_ALL
void       gtk_entry_set_max_length 		(GtkEntry      *entry,
						 gint           max);
GDK_AVAILABLE_IN_ALL
gint       gtk_entry_get_max_length             (GtkEntry      *entry);
GDK_AVAILABLE_IN_ALL
guint16    gtk_entry_get_text_length            (GtkEntry      *entry);

GDK_AVAILABLE_IN_ALL
void       gtk_entry_set_activates_default      (GtkEntry      *entry,
                                                 gboolean       setting);
GDK_AVAILABLE_IN_ALL
gboolean   gtk_entry_get_activates_default      (GtkEntry      *entry);

GDK_AVAILABLE_IN_ALL
void       gtk_entry_set_width_chars            (GtkEntry      *entry,
                                                 gint           n_chars);
GDK_AVAILABLE_IN_ALL
gint       gtk_entry_get_width_chars            (GtkEntry      *entry);

GDK_AVAILABLE_IN_3_12
void       gtk_entry_set_max_width_chars        (GtkEntry      *entry,
                                                 gint           n_chars);
GDK_AVAILABLE_IN_3_12
gint       gtk_entry_get_max_width_chars        (GtkEntry      *entry);

/* Somewhat more convenient than the GtkEditable generic functions
 */
GDK_AVAILABLE_IN_ALL
void       gtk_entry_set_text                   (GtkEntry      *entry,
                                                 const gchar   *text);
/* returns a reference to the text */
GDK_AVAILABLE_IN_ALL
const gchar* gtk_entry_get_text        (GtkEntry      *entry);

GDK_AVAILABLE_IN_ALL
PangoLayout* gtk_entry_get_layout               (GtkEntry      *entry);
GDK_AVAILABLE_IN_ALL
void         gtk_entry_get_layout_offsets       (GtkEntry      *entry,
                                                 gint          *x,
                                                 gint          *y);
GDK_AVAILABLE_IN_ALL
void       gtk_entry_set_alignment              (GtkEntry      *entry,
                                                 gfloat         xalign);
GDK_AVAILABLE_IN_ALL
gfloat     gtk_entry_get_alignment              (GtkEntry      *entry);

GDK_AVAILABLE_IN_ALL
void                gtk_entry_set_completion (GtkEntry           *entry,
                                              GtkEntryCompletion *completion);
GDK_AVAILABLE_IN_ALL
GtkEntryCompletion *gtk_entry_get_completion (GtkEntry           *entry);

GDK_AVAILABLE_IN_ALL
gint       gtk_entry_layout_index_to_text_index (GtkEntry      *entry,
                                                 gint           layout_index);
GDK_AVAILABLE_IN_ALL
gint       gtk_entry_text_index_to_layout_index (GtkEntry      *entry,
                                                 gint           text_index);

/* For scrolling cursor appropriately
 */
GDK_AVAILABLE_IN_ALL
void           gtk_entry_set_cursor_hadjustment (GtkEntry      *entry,
                                                 GtkAdjustment *adjustment);
GDK_AVAILABLE_IN_ALL
GtkAdjustment* gtk_entry_get_cursor_hadjustment (GtkEntry      *entry);

/* Progress API
 */
GDK_AVAILABLE_IN_ALL
void           gtk_entry_set_progress_fraction   (GtkEntry     *entry,
                                                  gdouble       fraction);
GDK_AVAILABLE_IN_ALL
gdouble        gtk_entry_get_progress_fraction   (GtkEntry     *entry);

GDK_AVAILABLE_IN_ALL
void           gtk_entry_set_progress_pulse_step (GtkEntry     *entry,
                                                  gdouble       fraction);
GDK_AVAILABLE_IN_ALL
gdouble        gtk_entry_get_progress_pulse_step (GtkEntry     *entry);

GDK_AVAILABLE_IN_ALL
void           gtk_entry_progress_pulse          (GtkEntry     *entry);
GDK_AVAILABLE_IN_3_2
const gchar*   gtk_entry_get_placeholder_text    (GtkEntry             *entry);
GDK_AVAILABLE_IN_3_2
void           gtk_entry_set_placeholder_text    (GtkEntry             *entry,
                                                  const gchar          *text);
/* Setting and managing icons
 */
GDK_AVAILABLE_IN_ALL
void           gtk_entry_set_icon_from_pixbuf            (GtkEntry             *entry,
							  GtkEntryIconPosition  icon_pos,
							  GdkPixbuf            *pixbuf);
GDK_DEPRECATED_IN_3_10_FOR(gtk_entry_set_icon_from_icon_name)
void           gtk_entry_set_icon_from_stock             (GtkEntry             *entry,
							  GtkEntryIconPosition  icon_pos,
							  const gchar          *stock_id);
GDK_AVAILABLE_IN_ALL
void           gtk_entry_set_icon_from_icon_name         (GtkEntry             *entry,
							  GtkEntryIconPosition  icon_pos,
							  const gchar          *icon_name);
GDK_AVAILABLE_IN_ALL
void           gtk_entry_set_icon_from_gicon             (GtkEntry             *entry,
							  GtkEntryIconPosition  icon_pos,
							  GIcon                *icon);
GDK_AVAILABLE_IN_ALL
GtkImageType gtk_entry_get_icon_storage_type             (GtkEntry             *entry,
							  GtkEntryIconPosition  icon_pos);
GDK_AVAILABLE_IN_ALL
GdkPixbuf*   gtk_entry_get_icon_pixbuf                   (GtkEntry             *entry,
							  GtkEntryIconPosition  icon_pos);
GDK_DEPRECATED_IN_3_10_FOR(gtk_entry_get_icon_name)
const gchar* gtk_entry_get_icon_stock                    (GtkEntry             *entry,
							  GtkEntryIconPosition  icon_pos);
GDK_AVAILABLE_IN_ALL
const gchar* gtk_entry_get_icon_name                     (GtkEntry             *entry,
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
gint         gtk_entry_get_icon_at_pos                   (GtkEntry             *entry,
							  gint                  x,
							  gint                  y);
GDK_AVAILABLE_IN_ALL
void         gtk_entry_set_icon_tooltip_text             (GtkEntry             *entry,
							  GtkEntryIconPosition  icon_pos,
							  const gchar          *tooltip);
GDK_AVAILABLE_IN_ALL
gchar *      gtk_entry_get_icon_tooltip_text             (GtkEntry             *entry,
                                                          GtkEntryIconPosition  icon_pos);
GDK_AVAILABLE_IN_ALL
void         gtk_entry_set_icon_tooltip_markup           (GtkEntry             *entry,
							  GtkEntryIconPosition  icon_pos,
							  const gchar          *tooltip);
GDK_AVAILABLE_IN_ALL
gchar *      gtk_entry_get_icon_tooltip_markup           (GtkEntry             *entry,
                                                          GtkEntryIconPosition  icon_pos);
GDK_AVAILABLE_IN_ALL
void         gtk_entry_set_icon_drag_source              (GtkEntry             *entry,
							  GtkEntryIconPosition  icon_pos,
							  GtkTargetList        *target_list,
							  GdkDragAction         actions);
GDK_AVAILABLE_IN_ALL
gint         gtk_entry_get_current_icon_drag_source      (GtkEntry             *entry);
GDK_AVAILABLE_IN_ALL
void         gtk_entry_get_icon_area                     (GtkEntry             *entry,
                                                          GtkEntryIconPosition  icon_pos,
                                                          GdkRectangle         *icon_area);

GDK_AVAILABLE_IN_ALL
gboolean    gtk_entry_im_context_filter_keypress         (GtkEntry             *entry,
                                                          GdkEventKey          *event);
GDK_AVAILABLE_IN_ALL
void        gtk_entry_reset_im_context                   (GtkEntry             *entry);

GDK_AVAILABLE_IN_3_6
void            gtk_entry_set_input_purpose                  (GtkEntry             *entry,
                                                              GtkInputPurpose       purpose);
GDK_AVAILABLE_IN_3_6
GtkInputPurpose gtk_entry_get_input_purpose                  (GtkEntry             *entry);

GDK_AVAILABLE_IN_3_6
void            gtk_entry_set_input_hints                    (GtkEntry             *entry,
                                                              GtkInputHints         hints);
GDK_AVAILABLE_IN_3_6
GtkInputHints   gtk_entry_get_input_hints                    (GtkEntry             *entry);

GDK_AVAILABLE_IN_3_6
void            gtk_entry_set_attributes                     (GtkEntry             *entry,
                                                              PangoAttrList        *attrs);
GDK_AVAILABLE_IN_3_6
PangoAttrList  *gtk_entry_get_attributes                     (GtkEntry             *entry);

GDK_AVAILABLE_IN_3_10
void            gtk_entry_set_tabs                           (GtkEntry             *entry,
                                                              PangoTabArray        *tabs);

GDK_AVAILABLE_IN_3_10
PangoTabArray  *gtk_entry_get_tabs                           (GtkEntry             *entry);

GDK_AVAILABLE_IN_3_16
void           gtk_entry_grab_focus_without_selecting        (GtkEntry             *entry);

G_END_DECLS

#endif /* __GTK_ENTRY_H__ */
