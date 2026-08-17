/* ATK -  Accessibility Toolkit
 * Copyright 2001 Sun Microsystems Inc.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

#ifndef __ATK_TABLE_H__
#define __ATK_TABLE_H__

#if defined(ATK_DISABLE_SINGLE_INCLUDES) && !defined (__ATK_H_INSIDE__) && !defined (ATK_COMPILATION)
#error "Only <atk/atk.h> can be included directly."
#endif

#include <atk/atkobject.h>

G_BEGIN_DECLS

#define ATK_TYPE_TABLE                    (atk_table_get_type ())
#define ATK_IS_TABLE(obj)                 G_TYPE_CHECK_INSTANCE_TYPE ((obj), ATK_TYPE_TABLE)
#define ATK_TABLE(obj)                    G_TYPE_CHECK_INSTANCE_CAST ((obj), ATK_TYPE_TABLE, AtkTable)
#define ATK_TABLE_GET_IFACE(obj)          (G_TYPE_INSTANCE_GET_INTERFACE ((obj), ATK_TYPE_TABLE, AtkTableIface))

#ifndef _TYPEDEF_ATK_TABLE_
#define _TYPEDEF_ATK_TABLE_
typedef struct _AtkTable AtkTable;
#endif
typedef struct _AtkTableIface AtkTableIface;

struct _AtkTableIface
{
  GTypeInterface parent;

  AtkObject*        (* ref_at)                   (AtkTable      *table,
                                                  gint          row,
                                                  gint          column);
  gint              (* get_index_at)             (AtkTable      *table,
                                                  gint          row,
                                                  gint          column);
  gint              (* get_column_at_index)      (AtkTable      *table,
                                                  gint          index_);
  gint              (* get_row_at_index)         (AtkTable      *table,
                                                  gint          index_);
  gint              (* get_n_columns)           (AtkTable      *table);
  gint              (* get_n_rows)               (AtkTable      *table);
  gint              (* get_column_extent_at)     (AtkTable      *table,
                                                  gint          row,
                                                  gint          column);
  gint              (* get_row_extent_at)        (AtkTable      *table,
                                                  gint          row,
                                                  gint          column);
  AtkObject*
                    (* get_caption)              (AtkTable      *table);
  const gchar*      (* get_column_description)   (AtkTable      *table,
                                                  gint          column);
  AtkObject*        (* get_column_header)        (AtkTable      *table,
						  gint		column);
  const gchar*      (* get_row_description)      (AtkTable      *table,
                                                  gint          row);
  AtkObject*        (* get_row_header)           (AtkTable      *table,
						  gint		row);
  AtkObject*        (* get_summary)              (AtkTable      *table);
  void              (* set_caption)              (AtkTable      *table,
                                                  AtkObject     *caption);
  void              (* set_column_description)   (AtkTable      *table,
                                                  gint          column,
                                                  const gchar   *description);
  void              (* set_column_header)        (AtkTable      *table,
                                                  gint          column,
                                                  AtkObject     *header);
  void              (* set_row_description)      (AtkTable      *table,
                                                  gint          row,
                                                  const gchar   *description);
  void              (* set_row_header)           (AtkTable      *table,
                                                  gint          row,
                                                  AtkObject     *header);
  void              (* set_summary)              (AtkTable      *table,
                                                  AtkObject     *accessible);
  gint              (* get_selected_columns)     (AtkTable      *table,
                                                  gint          **selected);
  gint              (* get_selected_rows)        (AtkTable      *table,
                                                  gint          **selected);
  gboolean          (* is_column_selected)       (AtkTable      *table,
                                                  gint          column);
  gboolean          (* is_row_selected)          (AtkTable      *table,
                                                  gint          row);
  gboolean          (* is_selected)              (AtkTable      *table,
                                                  gint          row,
                                                  gint          column);
  gboolean          (* add_row_selection)        (AtkTable      *table,
                                                  gint          row);
  gboolean          (* remove_row_selection)     (AtkTable      *table,
                                                  gint          row);
  gboolean          (* add_column_selection)     (AtkTable      *table,
                                                  gint          column);
  gboolean          (* remove_column_selection)  (AtkTable      *table,
                                                  gint          column);

  /*
   * signal handlers
   */
  void              (* row_inserted)             (AtkTable      *table,
                                                  gint          row,
                                                  gint          num_inserted);
  void              (* column_inserted)          (AtkTable      *table,
                                                  gint          column,
                                                  gint          num_inserted);
  void              (* row_deleted)              (AtkTable      *table,
                                                  gint          row,
                                                  gint          num_deleted);
  void              (* column_deleted)           (AtkTable      *table,
                                                  gint          column,
                                                  gint          num_deleted);
  void              (* row_reordered)            (AtkTable      *table);
  void              (* column_reordered)         (AtkTable      *table);
  void              (* model_changed)            (AtkTable      *table);
};

ATK_AVAILABLE_IN_ALL
GType atk_table_get_type (void);

ATK_AVAILABLE_IN_ALL
AtkObject*        atk_table_ref_at               (AtkTable         *table,
                                                  gint             row,
                                                  gint             column);
ATK_DEPRECATED_IN_2_12_FOR(atk_table_ref_at)
gint              atk_table_get_index_at         (AtkTable         *table,
                                                  gint             row,
                                                  gint             column);
ATK_DEPRECATED_IN_2_12
gint              atk_table_get_column_at_index  (AtkTable         *table,
                                                  gint             index_);
ATK_DEPRECATED_IN_2_12
gint              atk_table_get_row_at_index     (AtkTable         *table,
                                                  gint             index_);
ATK_AVAILABLE_IN_ALL
gint              atk_table_get_n_columns        (AtkTable         *table);
ATK_AVAILABLE_IN_ALL
gint              atk_table_get_n_rows           (AtkTable         *table);
ATK_AVAILABLE_IN_ALL
gint              atk_table_get_column_extent_at (AtkTable         *table,
                                                  gint             row,
                                                  gint             column);
ATK_AVAILABLE_IN_ALL
gint              atk_table_get_row_extent_at    (AtkTable         *table,
                                                  gint             row,
                                                  gint             column);
ATK_AVAILABLE_IN_ALL
AtkObject*
                  atk_table_get_caption          (AtkTable         *table);
ATK_AVAILABLE_IN_ALL
const gchar*      atk_table_get_column_description
                                                 (AtkTable         *table,
                                                  gint             column);
ATK_AVAILABLE_IN_ALL
AtkObject*        atk_table_get_column_header    (AtkTable         *table,
						  gint		   column);
ATK_AVAILABLE_IN_ALL
const gchar*      atk_table_get_row_description  (AtkTable         *table,
                                                  gint             row);
ATK_AVAILABLE_IN_ALL
AtkObject*        atk_table_get_row_header       (AtkTable         *table,
						  gint		   row);
ATK_AVAILABLE_IN_ALL
AtkObject*        atk_table_get_summary          (AtkTable         *table);
ATK_AVAILABLE_IN_ALL
void              atk_table_set_caption          (AtkTable         *table,
                                                  AtkObject        *caption);
ATK_AVAILABLE_IN_ALL
void              atk_table_set_column_description 
                                                 (AtkTable         *table,
                                                  gint             column,
                                                  const gchar      *description);
ATK_AVAILABLE_IN_ALL
void              atk_table_set_column_header    (AtkTable         *table,
                                                  gint             column,
                                                  AtkObject        *header);
ATK_AVAILABLE_IN_ALL
void              atk_table_set_row_description  (AtkTable         *table,
                                                  gint             row,
                                                  const gchar      *description);
ATK_AVAILABLE_IN_ALL
void              atk_table_set_row_header       (AtkTable         *table,
                                                  gint             row,
                                                  AtkObject        *header);
ATK_AVAILABLE_IN_ALL
void              atk_table_set_summary          (AtkTable         *table,
                                                  AtkObject        *accessible);
ATK_AVAILABLE_IN_ALL
gint              atk_table_get_selected_columns (AtkTable         *table,
                                                  gint             **selected);
ATK_AVAILABLE_IN_ALL
gint              atk_table_get_selected_rows    (AtkTable         *table,
                                                  gint             **selected);
ATK_AVAILABLE_IN_ALL
gboolean          atk_table_is_column_selected   (AtkTable         *table,
                                                  gint             column);
ATK_AVAILABLE_IN_ALL
gboolean          atk_table_is_row_selected      (AtkTable         *table,
                                                  gint             row);
ATK_AVAILABLE_IN_ALL
gboolean          atk_table_is_selected          (AtkTable         *table,
                                                  gint             row,
                                                  gint             column);
ATK_AVAILABLE_IN_ALL
gboolean          atk_table_add_row_selection    (AtkTable         *table,
                                                  gint             row);
ATK_AVAILABLE_IN_ALL
gboolean          atk_table_remove_row_selection (AtkTable         *table,
                                                  gint             row);
ATK_AVAILABLE_IN_ALL
gboolean          atk_table_add_column_selection (AtkTable         *table,
                                                  gint             column);
ATK_AVAILABLE_IN_ALL
gboolean          atk_table_remove_column_selection  
                                                 (AtkTable         *table,
                                                  gint             column);

G_END_DECLS

#endif /* __ATK_TABLE_H__ */
