/* GTK - The GIMP Toolkit
 * Copyright (C) 1995-1997 Peter Mattis, Spencer Kimball and Josh MacDonald
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

#ifndef __GTK_OBJECT_H__
#define __GTK_OBJECT_H__


#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gdkconfig.h>
#include <gtk/gtkenums.h>
#include <gtk/gtktypeutils.h>
#include <gtk/gtkdebug.h>


G_BEGIN_DECLS

/* macros for casting a pointer to a GtkObject or GtkObjectClass pointer,
 * and to test whether `object' and `klass' are of type GTK_TYPE_OBJECT.
 * these are the standard macros for all GtkObject-derived classes.
 */
#define GTK_TYPE_OBJECT              (gtk_object_get_type ())
#define GTK_OBJECT(object)           (G_TYPE_CHECK_INSTANCE_CAST ((object), GTK_TYPE_OBJECT, GtkObject))
#define GTK_OBJECT_CLASS(klass)      (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_OBJECT, GtkObjectClass))
#define GTK_IS_OBJECT(object)        (G_TYPE_CHECK_INSTANCE_TYPE ((object), GTK_TYPE_OBJECT))
#define GTK_IS_OBJECT_CLASS(klass)   (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_OBJECT))
#define GTK_OBJECT_GET_CLASS(object) (G_TYPE_INSTANCE_GET_CLASS ((object), GTK_TYPE_OBJECT, GtkObjectClass))

/* Macros for extracting various fields from GtkObject and GtkObjectClass.
 */
#ifndef GTK_DISABLE_DEPRECATED
/**
 * GTK_OBJECT_TYPE:
 * @object: a #GtkObject.
 *
 * Gets the type of an object.
 *
 * Deprecated: 2.20: Use G_OBJECT_TYPE() instead.
 */
#define GTK_OBJECT_TYPE                   G_OBJECT_TYPE
/**
 * GTK_OBJECT_TYPE_NAME:
 * @object: a #GtkObject.
 *
 * Gets the name of an object's type.
 *
 * Deprecated: 2.20: Use G_OBJECT_TYPE_NAME() instead.
 */
#define GTK_OBJECT_TYPE_NAME              G_OBJECT_TYPE_NAME
#endif

#if !defined (GTK_DISABLE_DEPRECATED) || defined (GTK_COMPILATION)
/* GtkObject only uses the first 4 bits of the flags field.
 * Derived objects may use the remaining bits. Though this
 * is a kinda nasty break up, it does make the size of
 * derived objects smaller.
 */
typedef enum
{
  GTK_IN_DESTRUCTION	= 1 << 0, /* Used internally during dispose */
  GTK_FLOATING		= 1 << 1,
  GTK_RESERVED_1	= 1 << 2,
  GTK_RESERVED_2	= 1 << 3
} GtkObjectFlags;

/* Macros for extracting the object_flags from GtkObject.
 */
#define GTK_OBJECT_FLAGS(obj)		  (GTK_OBJECT (obj)->flags)
#ifndef GTK_DISABLE_DEPRECATED
#define GTK_OBJECT_FLOATING(obj)	  (g_object_is_floating (obj))
#endif

/* Macros for setting and clearing bits in the object_flags field of GtkObject.
 */
#define GTK_OBJECT_SET_FLAGS(obj,flag)	  G_STMT_START{ (GTK_OBJECT_FLAGS (obj) |= (flag)); }G_STMT_END
#define GTK_OBJECT_UNSET_FLAGS(obj,flag)  G_STMT_START{ (GTK_OBJECT_FLAGS (obj) &= ~(flag)); }G_STMT_END
#endif

typedef struct _GtkObjectClass	GtkObjectClass;


struct _GtkObject
{
  GInitiallyUnowned parent_instance;

  /* 32 bits of flags. GtkObject only uses 4 of these bits and
   *  GtkWidget uses the rest. This is done because structs are
   *  aligned on 4 or 8 byte boundaries. If a new bitfield were
   *  used in GtkWidget much space would be wasted.
   */
  guint32 GSEAL (flags);
};

struct _GtkObjectClass
{
  GInitiallyUnownedClass parent_class;

  /* Non overridable class methods to set and get per class arguments */
  void (*set_arg) (GtkObject *object,
		   GtkArg    *arg,
		   guint      arg_id);
  void (*get_arg) (GtkObject *object,
		   GtkArg    *arg,
		   guint      arg_id);

  /* Default signal handler for the ::destroy signal, which is
   *  invoked to request that references to the widget be dropped.
   *  If an object class overrides destroy() in order to perform class
   *  specific destruction then it must still invoke its superclass'
   *  implementation of the method after it is finished with its
   *  own cleanup. (See gtk_widget_real_destroy() for an example of
   *  how to do this).
   */
  void (*destroy)  (GtkObject *object);
};



/* Application-level methods */

GType gtk_object_get_type (void) G_GNUC_CONST;

#ifndef GTK_DISABLE_DEPRECATED
void gtk_object_sink	  (GtkObject *object);
#endif
void gtk_object_destroy	  (GtkObject *object);

/****************************************************************/

#ifndef GTK_DISABLE_DEPRECATED

GtkObject*	gtk_object_new		  (GType	       type,
					   const gchar	      *first_property_name,
					   ...);
GtkObject*	gtk_object_ref		  (GtkObject	      *object);
void		gtk_object_unref	  (GtkObject	      *object);
void gtk_object_weakref	  (GtkObject	    *object,
			   GDestroyNotify    notify,
			   gpointer	     data);
void gtk_object_weakunref (GtkObject	    *object,
			   GDestroyNotify    notify,
			   gpointer	     data);

/* Set 'data' to the "object_data" field of the object. The
 *  data is indexed by the "key". If there is already data
 *  associated with "key" then the new data will replace it.
 *  If 'data' is NULL then this call is equivalent to
 *  'gtk_object_remove_data'.
 *  The gtk_object_set_data_full variant acts just the same,
 *  but takes an additional argument which is a function to
 *  be called when the data is removed.
 *  `gtk_object_remove_data' is equivalent to the above,
 *  where 'data' is NULL
 *  `gtk_object_get_data' gets the data associated with "key".
 */
void	 gtk_object_set_data	     (GtkObject	     *object,
				      const gchar    *key,
				      gpointer	      data);
void	 gtk_object_set_data_full    (GtkObject	     *object,
				      const gchar    *key,
				      gpointer	      data,
				      GDestroyNotify  destroy);
void	 gtk_object_remove_data	     (GtkObject	     *object,
				      const gchar    *key);
gpointer gtk_object_get_data	     (GtkObject	     *object,
				      const gchar    *key);
void	 gtk_object_remove_no_notify (GtkObject	     *object,
				      const gchar    *key);

/* Set/get the "user_data" object data field of "object". It should
 *  be noted that these functions are no different than calling
 *  `gtk_object_set_data'/`gtk_object_get_data' with a key of "user_data".
 *  They are merely provided as a convenience.
 */
void	 gtk_object_set_user_data (GtkObject	*object,
				   gpointer	 data);
gpointer gtk_object_get_user_data (GtkObject	*object);


/* Object-level methods */

/* Object data method variants that operate on key ids. */
void gtk_object_set_data_by_id		(GtkObject	 *object,
					 GQuark		  data_id,
					 gpointer	  data);
void gtk_object_set_data_by_id_full	(GtkObject	 *object,
					 GQuark		  data_id,
					 gpointer	  data,
					 GDestroyNotify   destroy);
gpointer gtk_object_get_data_by_id	(GtkObject	 *object,
					 GQuark		  data_id);
void  gtk_object_remove_data_by_id	(GtkObject	 *object,
					 GQuark		  data_id);
void  gtk_object_remove_no_notify_by_id	(GtkObject	 *object,
					 GQuark		  key_id);
#define	gtk_object_data_try_key	    g_quark_try_string
#define	gtk_object_data_force_id    g_quark_from_string

/* GtkArg flag bits for gtk_object_add_arg_type
 */
typedef enum
{
  GTK_ARG_READABLE	 = G_PARAM_READABLE,
  GTK_ARG_WRITABLE	 = G_PARAM_WRITABLE,
  GTK_ARG_CONSTRUCT	 = G_PARAM_CONSTRUCT,
  GTK_ARG_CONSTRUCT_ONLY = G_PARAM_CONSTRUCT_ONLY,
  GTK_ARG_CHILD_ARG	 = 1 << 4
} GtkArgFlags;
#define	GTK_ARG_READWRITE	(GTK_ARG_READABLE | GTK_ARG_WRITABLE)
void	gtk_object_get		(GtkObject	*object,
				 const gchar	*first_property_name,
				 ...) G_GNUC_NULL_TERMINATED;
void	gtk_object_set		(GtkObject	*object,
				 const gchar	*first_property_name,
				 ...) G_GNUC_NULL_TERMINATED;
void	gtk_object_add_arg_type		(const gchar    *arg_name,
					 GType           arg_type,
					 guint           arg_flags,
					 guint           arg_id);

#endif /* GTK_DISABLE_DEPRECATED */

G_END_DECLS

#endif /* __GTK_OBJECT_H__ */
