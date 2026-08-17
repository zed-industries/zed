/* GTK - The GIMP Toolkit
 * Copyright (C) 1995-1997 Peter Mattis, Spencer Kimball and Josh MacDonald
 *
 * GtkItemFactory: Flexible item factory with automatic rc handling
 * Copyright (C) 1998 Tim Janik
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

#ifndef GTK_DISABLE_DEPRECATED

#ifndef __GTK_ITEM_FACTORY_H__
#define	__GTK_ITEM_FACTORY_H__

#include <gtk/gtk.h>


G_BEGIN_DECLS

typedef void	(*GtkPrintFunc)		   (gpointer		 func_data,
					    const gchar		*str);
/* We use () here to mean unspecified arguments. This is deprecated
 * as of C99, but we can't change it without breaking compatibility.
 * (Note that if we are included from a C++ program () will mean
 * (void) so an explicit cast will be needed.)
 */
typedef	void	(*GtkItemFactoryCallback)  ();
typedef	void	(*GtkItemFactoryCallback1) (gpointer		 callback_data,
					    guint		 callback_action,
					    GtkWidget		*widget);

#define GTK_TYPE_ITEM_FACTORY            (gtk_item_factory_get_type ())
#define GTK_ITEM_FACTORY(object)         (G_TYPE_CHECK_INSTANCE_CAST ((object), GTK_TYPE_ITEM_FACTORY, GtkItemFactory))
#define GTK_ITEM_FACTORY_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_ITEM_FACTORY, GtkItemFactoryClass))
#define GTK_IS_ITEM_FACTORY(object)      (G_TYPE_CHECK_INSTANCE_TYPE ((object), GTK_TYPE_ITEM_FACTORY))
#define GTK_IS_ITEM_FACTORY_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_ITEM_FACTORY))
#define GTK_ITEM_FACTORY_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_ITEM_FACTORY, GtkItemFactoryClass))


typedef	struct	_GtkItemFactory			GtkItemFactory;
typedef	struct	_GtkItemFactoryClass		GtkItemFactoryClass;
typedef	struct	_GtkItemFactoryEntry		GtkItemFactoryEntry;
typedef	struct	_GtkItemFactoryItem		GtkItemFactoryItem;

struct _GtkItemFactory
{
  GtkObject		 object;

  gchar			*path;
  GtkAccelGroup		*accel_group;
  GtkWidget		*widget;
  GSList		*items;

  GtkTranslateFunc       translate_func;
  gpointer               translate_data;
  GDestroyNotify         translate_notify;
};

struct _GtkItemFactoryClass
{
  GtkObjectClass	 object_class;

  GHashTable		*item_ht;

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

struct _GtkItemFactoryEntry
{
  gchar *path;
  gchar *accelerator;

  GtkItemFactoryCallback callback;
  guint			 callback_action;

  /* possible values:
   * NULL		-> "<Item>"
   * ""			-> "<Item>"
   * "<Title>"		-> create a title item
   * "<Item>"		-> create a simple item
   * "<ImageItem>"	-> create an item holding an image
   * "<StockItem>"	-> create an item holding a stock image
   * "<CheckItem>"	-> create a check item
   * "<ToggleItem>"	-> create a toggle item
   * "<RadioItem>"	-> create a radio item
   * <path>		-> path of a radio item to link against
   * "<Separator>"	-> create a separator
   * "<Tearoff>"	-> create a tearoff separator
   * "<Branch>"		-> create an item to hold sub items
   * "<LastBranch>"	-> create a right justified item to hold sub items
   */
  gchar		 *item_type;

  /* Extra data for some item types:
   *  ImageItem  -> pointer to inlined pixbuf stream
   *  StockItem  -> name of stock item
   */
  gconstpointer extra_data;
};

struct _GtkItemFactoryItem
{
  gchar *path;
  GSList *widgets;
};


GType		gtk_item_factory_get_type	    (void) G_GNUC_CONST;

/* `container_type' must be of GTK_TYPE_MENU_BAR, GTK_TYPE_MENU,
 * or GTK_TYPE_OPTION_MENU.
 */
GtkItemFactory*	gtk_item_factory_new	   (GType		 container_type,
					    const gchar		*path,
					    GtkAccelGroup       *accel_group);
void		gtk_item_factory_construct (GtkItemFactory	*ifactory,
					    GType		 container_type,
					    const gchar		*path,
					    GtkAccelGroup       *accel_group);
     
/* These functions operate on GtkItemFactoryClass basis.
 */
void		gtk_item_factory_add_foreign        (GtkWidget	    *accel_widget,
						     const gchar    *full_path,
						     GtkAccelGroup  *accel_group,
						     guint	     keyval,
						     GdkModifierType modifiers);
     
GtkItemFactory*       gtk_item_factory_from_widget      (GtkWidget *widget);
const gchar *         gtk_item_factory_path_from_widget (GtkWidget *widget);

GtkWidget*	gtk_item_factory_get_item	      (GtkItemFactory *ifactory,
						       const gchar    *path);
GtkWidget*	gtk_item_factory_get_widget	      (GtkItemFactory *ifactory,
						       const gchar    *path);
GtkWidget*	gtk_item_factory_get_widget_by_action (GtkItemFactory *ifactory,
						       guint	       action);
GtkWidget*	gtk_item_factory_get_item_by_action   (GtkItemFactory *ifactory,
						       guint	       action);

void	gtk_item_factory_create_item	(GtkItemFactory		*ifactory,
					 GtkItemFactoryEntry	*entry,
					 gpointer		 callback_data,
					 guint			 callback_type);
void	gtk_item_factory_create_items	(GtkItemFactory		*ifactory,
					 guint			 n_entries,
					 GtkItemFactoryEntry	*entries,
					 gpointer		 callback_data);
void	gtk_item_factory_delete_item	(GtkItemFactory		*ifactory,
					 const gchar		*path);
void	gtk_item_factory_delete_entry	(GtkItemFactory		*ifactory,
					 GtkItemFactoryEntry	*entry);
void	gtk_item_factory_delete_entries	(GtkItemFactory		*ifactory,
					 guint			 n_entries,
					 GtkItemFactoryEntry	*entries);
void	gtk_item_factory_popup		(GtkItemFactory		*ifactory,
					 guint			 x,
					 guint			 y,
					 guint			 mouse_button,
					 guint32		 time_);
void	gtk_item_factory_popup_with_data(GtkItemFactory		*ifactory,
					 gpointer		 popup_data,
					 GDestroyNotify          destroy,
					 guint			 x,
					 guint			 y,
					 guint			 mouse_button,
					 guint32		 time_);
gpointer gtk_item_factory_popup_data	(GtkItemFactory		*ifactory);
gpointer gtk_item_factory_popup_data_from_widget (GtkWidget	*widget);
void   gtk_item_factory_set_translate_func (GtkItemFactory      *ifactory,
					    GtkTranslateFunc     func,
					    gpointer             data,
					    GDestroyNotify       notify);

/* Compatibility functions for deprecated GtkMenuFactory code
 */

/* Used by gtk_item_factory_create_menu_entries () */
typedef void (*GtkMenuCallback) (GtkWidget *widget,
				 gpointer   user_data);
typedef struct {
  gchar *path;
  gchar *accelerator;
  GtkMenuCallback callback;
  gpointer callback_data;
  GtkWidget *widget;
} GtkMenuEntry;

/* Used by gtk_item_factory_callback_marshal () */
typedef	void	(*GtkItemFactoryCallback2) (GtkWidget		*widget,
					    gpointer		 callback_data,
					    guint		 callback_action);

/* Used by gtk_item_factory_create_items () */
void	gtk_item_factory_create_items_ac (GtkItemFactory	*ifactory,
					  guint			 n_entries,
					  GtkItemFactoryEntry	*entries,
					  gpointer		 callback_data,
					  guint			 callback_type);

GtkItemFactory*	gtk_item_factory_from_path   (const gchar       *path);
void	gtk_item_factory_create_menu_entries (guint		 n_entries,
					      GtkMenuEntry      *entries);
void	gtk_item_factories_path_delete	   (const gchar		*ifactory_path,
					    const gchar		*path);

G_END_DECLS

#endif /* !GTK_DISABLE_DEPRECATED */

#endif	/* __GTK_ITEM_FACTORY_H__ */

