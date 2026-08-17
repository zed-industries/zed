/* GTK - The GIMP Toolkit
 * Copyright (C) 1998, 2001 Tim Janik
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.	 See the GNU
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

#ifndef __GTK_ACCEL_GROUP_H__
#define __GTK_ACCEL_GROUP_H__


#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gdk/gdk.h>
#include <gtk/gtkenums.h>

G_BEGIN_DECLS


/* --- type macros --- */
#define GTK_TYPE_ACCEL_GROUP              (gtk_accel_group_get_type ())
#define GTK_ACCEL_GROUP(object)           (G_TYPE_CHECK_INSTANCE_CAST ((object), GTK_TYPE_ACCEL_GROUP, GtkAccelGroup))
#define GTK_ACCEL_GROUP_CLASS(klass)      (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_ACCEL_GROUP, GtkAccelGroupClass))
#define GTK_IS_ACCEL_GROUP(object)        (G_TYPE_CHECK_INSTANCE_TYPE ((object), GTK_TYPE_ACCEL_GROUP))
#define GTK_IS_ACCEL_GROUP_CLASS(klass)   (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_ACCEL_GROUP))
#define GTK_ACCEL_GROUP_GET_CLASS(obj)    (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_ACCEL_GROUP, GtkAccelGroupClass))


/* --- accel flags --- */
/**
 * GtkAccelFlags:
 * @GTK_ACCEL_VISIBLE: Accelerator is visible
 * @GTK_ACCEL_LOCKED: Accelerator not removable
 * @GTK_ACCEL_MASK: Mask
 *
 * Accelerator flags used with gtk_accel_group_connect().
 */
typedef enum
{
  GTK_ACCEL_VISIBLE        = 1 << 0,
  GTK_ACCEL_LOCKED         = 1 << 1,
  GTK_ACCEL_MASK           = 0x07
} GtkAccelFlags;


/* --- typedefs & structures --- */
typedef struct _GtkAccelGroup	          GtkAccelGroup;
typedef struct _GtkAccelGroupClass        GtkAccelGroupClass;
typedef struct _GtkAccelGroupPrivate      GtkAccelGroupPrivate;
typedef struct _GtkAccelKey               GtkAccelKey;
typedef struct _GtkAccelGroupEntry        GtkAccelGroupEntry;
typedef gboolean (*GtkAccelGroupActivate) (GtkAccelGroup  *accel_group,
					   GObject        *acceleratable,
					   guint           keyval,
					   GdkModifierType modifier);

/**
 * GtkAccelGroupFindFunc:
 * @key: 
 * @closure: 
 * @data: (closure):
 * 
 * Since: 2.2
 */
typedef gboolean (*GtkAccelGroupFindFunc) (GtkAccelKey    *key,
					   GClosure       *closure,
					   gpointer        data);

/**
 * GtkAccelGroup:
 * 
 * An object representing and maintaining a group of accelerators.
 */
struct _GtkAccelGroup
{
  GObject               parent;
  GtkAccelGroupPrivate *priv;
};

/**
 * GtkAccelGroupClass:
 * @parent_class: The parent class.
 * @accel_changed: Signal emitted when an entry is added to or removed
 *    from the accel group.
 */
struct _GtkAccelGroupClass
{
  GObjectClass parent_class;

  /*< public >*/

  void	(*accel_changed)	(GtkAccelGroup	*accel_group,
				 guint           keyval,
				 GdkModifierType modifier,
				 GClosure       *accel_closure);

  /*< private >*/

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

/**
 * GtkAccelKey:
 * @accel_key: The accelerator keyval
 * @accel_mods:The accelerator modifiers
 * @accel_flags: The accelerator flags
 */
struct _GtkAccelKey
{
  guint           accel_key;
  GdkModifierType accel_mods;
  guint           accel_flags : 16;
};


/* -- Accelerator Groups --- */
GDK_AVAILABLE_IN_ALL
GType          gtk_accel_group_get_type           (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkAccelGroup* gtk_accel_group_new	      	  (void);
GDK_AVAILABLE_IN_ALL
gboolean       gtk_accel_group_get_is_locked      (GtkAccelGroup  *accel_group);
GDK_AVAILABLE_IN_ALL
GdkModifierType 
               gtk_accel_group_get_modifier_mask  (GtkAccelGroup  *accel_group);
GDK_AVAILABLE_IN_ALL
void	       gtk_accel_group_lock		  (GtkAccelGroup  *accel_group);
GDK_AVAILABLE_IN_ALL
void	       gtk_accel_group_unlock		  (GtkAccelGroup  *accel_group);
GDK_AVAILABLE_IN_ALL
void	       gtk_accel_group_connect		  (GtkAccelGroup  *accel_group,
						   guint	   accel_key,
						   GdkModifierType accel_mods,
						   GtkAccelFlags   accel_flags,
						   GClosure	  *closure);
GDK_AVAILABLE_IN_ALL
void           gtk_accel_group_connect_by_path    (GtkAccelGroup  *accel_group,
						   const gchar	  *accel_path,
						   GClosure	  *closure);
GDK_AVAILABLE_IN_ALL
gboolean       gtk_accel_group_disconnect	  (GtkAccelGroup  *accel_group,
						   GClosure	  *closure);
GDK_AVAILABLE_IN_ALL
gboolean       gtk_accel_group_disconnect_key	  (GtkAccelGroup  *accel_group,
						   guint	   accel_key,
						   GdkModifierType accel_mods);
GDK_AVAILABLE_IN_ALL
gboolean       gtk_accel_group_activate           (GtkAccelGroup   *accel_group,
                                                   GQuark	   accel_quark,
                                                   GObject	  *acceleratable,
                                                   guint	   accel_key,
                                                   GdkModifierType accel_mods);


/* --- GtkActivatable glue --- */
void		_gtk_accel_group_attach		(GtkAccelGroup	*accel_group,
						 GObject	*object);
void		_gtk_accel_group_detach		(GtkAccelGroup	*accel_group,
						 GObject	*object);
GDK_AVAILABLE_IN_ALL
gboolean        gtk_accel_groups_activate      	(GObject	*object,
						 guint		 accel_key,
						 GdkModifierType accel_mods);
GDK_AVAILABLE_IN_ALL
GSList*	        gtk_accel_groups_from_object    (GObject	*object);
GDK_AVAILABLE_IN_ALL
GtkAccelKey*	gtk_accel_group_find		(GtkAccelGroup	      *accel_group,
						 GtkAccelGroupFindFunc find_func,
						 gpointer              data);
GDK_AVAILABLE_IN_ALL
GtkAccelGroup*	gtk_accel_group_from_accel_closure (GClosure    *closure);


/* --- Accelerators--- */
GDK_AVAILABLE_IN_ALL
gboolean gtk_accelerator_valid		      (guint	        keyval,
					       GdkModifierType  modifiers) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
void	 gtk_accelerator_parse		      (const gchar     *accelerator,
					       guint	       *accelerator_key,
					       GdkModifierType *accelerator_mods);
GDK_AVAILABLE_IN_3_4
void gtk_accelerator_parse_with_keycode       (const gchar     *accelerator,
                                               guint           *accelerator_key,
                                               guint          **accelerator_codes,
                                               GdkModifierType *accelerator_mods);
GDK_AVAILABLE_IN_ALL
gchar*	 gtk_accelerator_name		      (guint	        accelerator_key,
					       GdkModifierType  accelerator_mods);
GDK_AVAILABLE_IN_3_4
gchar*	 gtk_accelerator_name_with_keycode    (GdkDisplay      *display,
                                               guint            accelerator_key,
                                               guint            keycode,
                                               GdkModifierType  accelerator_mods);
GDK_AVAILABLE_IN_ALL
gchar*   gtk_accelerator_get_label            (guint           accelerator_key,
                                               GdkModifierType accelerator_mods);
GDK_AVAILABLE_IN_3_4
gchar*   gtk_accelerator_get_label_with_keycode (GdkDisplay      *display,
                                                 guint            accelerator_key,
                                                 guint            keycode,
                                                 GdkModifierType  accelerator_mods);
GDK_AVAILABLE_IN_ALL
void	 gtk_accelerator_set_default_mod_mask (GdkModifierType  default_mod_mask);
GDK_AVAILABLE_IN_ALL
GdkModifierType
	 gtk_accelerator_get_default_mod_mask (void);

GDK_AVAILABLE_IN_ALL
GtkAccelGroupEntry*	gtk_accel_group_query	(GtkAccelGroup	*accel_group,
						 guint		 accel_key,
						 GdkModifierType accel_mods,
						 guint          *n_entries);

struct _GtkAccelGroupEntry
{
  GtkAccelKey  key;
  GClosure    *closure;
  GQuark       accel_path_quark;
};

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkAccelGroup, g_object_unref)

G_END_DECLS

#endif /* __GTK_ACCEL_GROUP_H__ */
