/* gtkcombo - combo widget for gtk+
 * Copyright 1997 Paolo Molaro
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

#ifndef GTK_DISABLE_DEPRECATED

#ifndef __GTK_SMART_COMBO_H__
#define __GTK_SMART_COMBO_H__

#include <gtk/gtk.h>


G_BEGIN_DECLS

#define GTK_TYPE_COMBO              (gtk_combo_get_type ())
#define GTK_COMBO(obj)              (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_COMBO, GtkCombo))
#define GTK_COMBO_CLASS(klass)      (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_COMBO, GtkComboClass))
#define GTK_IS_COMBO(obj)           (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_COMBO))
#define GTK_IS_COMBO_CLASS(klass)   (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_COMBO))
#define GTK_COMBO_GET_CLASS(obj)    (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_COMBO, GtkComboClass))


typedef struct _GtkCombo	GtkCombo;
typedef struct _GtkComboClass	GtkComboClass;

/* you should access only the entry and list fields directly */
struct _GtkCombo {
	GtkHBox hbox;
  
        /*< public >*/
	GtkWidget *entry;
	
        /*< private >*/
	GtkWidget *button;
	GtkWidget *popup;
	GtkWidget *popwin;
	
        /*< public >*/
	GtkWidget *list;

        /*< private >*/
	guint entry_change_id;
	guint list_change_id;	/* unused */

	guint value_in_list:1;
	guint ok_if_empty:1;
	guint case_sensitive:1;
	guint use_arrows:1;
	guint use_arrows_always:1;

        guint16 current_button;
	guint activate_id;
};

struct _GtkComboClass {
	GtkHBoxClass parent_class;

        /* Padding for future expansion */
        void (*_gtk_reserved1) (void);
        void (*_gtk_reserved2) (void);
        void (*_gtk_reserved3) (void);
        void (*_gtk_reserved4) (void);
};

GType      gtk_combo_get_type              (void) G_GNUC_CONST;

GtkWidget* gtk_combo_new                   (void);
/* the text in the entry must be or not be in the list */
void       gtk_combo_set_value_in_list     (GtkCombo*    combo, 
                                            gboolean     val,
                                            gboolean     ok_if_empty);
/* set/unset arrows working for changing the value (can be annoying) */
void       gtk_combo_set_use_arrows        (GtkCombo*    combo, 
                                            gboolean     val);
/* up/down arrows change value if current value not in list */
void       gtk_combo_set_use_arrows_always (GtkCombo*    combo, 
                                            gboolean     val);
/* perform case-sensitive compares */
void       gtk_combo_set_case_sensitive    (GtkCombo*    combo, 
                                            gboolean     val);
/* call this function on an item if it isn't a label or you
   want it to have a different value to be displayed in the entry */
void       gtk_combo_set_item_string       (GtkCombo*    combo,
                                            GtkItem*     item,
                                            const gchar* item_value);
/* simple interface */
void       gtk_combo_set_popdown_strings   (GtkCombo*    combo, 
                                            GList        *strings);

void       gtk_combo_disable_activate      (GtkCombo*    combo);

G_END_DECLS

#endif /* __GTK_SMART_COMBO_H__ */

#endif /* GTK_DISABLE_DEPRECATED */
