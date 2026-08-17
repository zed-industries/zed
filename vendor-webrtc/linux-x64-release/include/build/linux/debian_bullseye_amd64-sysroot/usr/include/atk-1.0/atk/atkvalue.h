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

#ifndef __ATK_VALUE_H__
#define __ATK_VALUE_H__

#if defined(ATK_DISABLE_SINGLE_INCLUDES) && !defined (__ATK_H_INSIDE__) && !defined (ATK_COMPILATION)
#error "Only <atk/atk.h> can be included directly."
#endif

#include <atk/atkobject.h>
#include <atk/atkrange.h>

G_BEGIN_DECLS

#define ATK_TYPE_VALUE                    (atk_value_get_type ())
#define ATK_IS_VALUE(obj)                 G_TYPE_CHECK_INSTANCE_TYPE ((obj), ATK_TYPE_VALUE)
#define ATK_VALUE(obj)                    G_TYPE_CHECK_INSTANCE_CAST ((obj), ATK_TYPE_VALUE, AtkValue)
#define ATK_VALUE_GET_IFACE(obj)          (G_TYPE_INSTANCE_GET_INTERFACE ((obj), ATK_TYPE_VALUE, AtkValueIface))

#ifndef _TYPEDEF_ATK_VALUE_
#define _TYPEDEF_ATK_VALUE__
typedef struct _AtkValue AtkValue;
#endif
typedef struct _AtkValueIface AtkValueIface;

/**
 * AtkValueType:
 *
 * Default types for a given value. Those are defined in order to
 * easily get localized strings to describe a given value or a given
 * subrange, using atk_value_type_get_localized_name().
 *
 */
typedef enum
{
  ATK_VALUE_VERY_WEAK,
  ATK_VALUE_WEAK,
  ATK_VALUE_ACCEPTABLE,
  ATK_VALUE_STRONG,
  ATK_VALUE_VERY_STRONG,
  ATK_VALUE_VERY_LOW,
  ATK_VALUE_LOW,
  ATK_VALUE_MEDIUM,
  ATK_VALUE_HIGH,
  ATK_VALUE_VERY_HIGH,
  ATK_VALUE_VERY_BAD,
  ATK_VALUE_BAD,
  ATK_VALUE_GOOD,
  ATK_VALUE_VERY_GOOD,
  ATK_VALUE_BEST,
  ATK_VALUE_LAST_DEFINED
}AtkValueType;

/**
 * AtkValueIface:
 * @get_current_value: This virtual function is deprecated since 2.12
 *  and it should not be overriden.
 * @get_maximum_value: This virtual function is deprecated since 2.12
 *  and it should not be overriden.
 * @get_minimum_value: This virtual function is deprecated since 2.12
 *  and it should not be overriden.
 * @set_current_value: This virtual function is deprecated since 2.12
 *  and it should not be overriden.
 * @get_minimum_increment: This virtual function is deprecated since
 *  2.12 and it should not be overriden.
 * @get_value_and_text: gets the current value and the human readable
 * text alternative (if available) of this object. Since 2.12.
 * @get_range: gets the range that defines the minimum and maximum
 *  value of this object. Returns NULL if there is no range
 *  defined. Since 2.12.
 * @get_increment: gets the minimum increment by which the value of
 *  this object may be changed. If zero it is undefined. Since 2.12.
 * @get_sub_ranges: returns a list of different subranges, and their
 *  description (if available) of this object. Returns NULL if there
 *  is not subranges defined. Since 2.12.
 * @set_value: sets the value of this object. Since 2.12.
 */
struct _AtkValueIface
{
  GTypeInterface parent;

  /*<deprecated>*/
  void     (* get_current_value) (AtkValue     *obj,
                                  GValue       *value);
  void     (* get_maximum_value) (AtkValue     *obj,
                                  GValue       *value);
  void     (* get_minimum_value) (AtkValue     *obj,
                                  GValue       *value);
  gboolean (* set_current_value) (AtkValue     *obj,
                                  const GValue *value);
  void     (* get_minimum_increment) (AtkValue   *obj,
				      GValue     *value);
  /*</deprecated>*/
  void     (* get_value_and_text) (AtkValue *obj,
                                   gdouble *value,
                                   gchar  **text);
  AtkRange*(* get_range)          (AtkValue *obj);
  gdouble  (* get_increment)      (AtkValue *obj);
  GSList*  (* get_sub_ranges)     (AtkValue *obj);
  void     (* set_value)          (AtkValue     *obj,
                                   const gdouble new_value);

};

ATK_AVAILABLE_IN_ALL
GType            atk_value_get_type (void);

ATK_DEPRECATED_IN_2_12_FOR(atk_value_get_value_and_text)
void      atk_value_get_current_value (AtkValue     *obj,
                                       GValue       *value);

ATK_DEPRECATED_IN_2_12_FOR(atk_value_get_range)
void     atk_value_get_maximum_value  (AtkValue     *obj,
                                       GValue       *value);
ATK_DEPRECATED_IN_2_12_FOR(atk_value_get_range)
void     atk_value_get_minimum_value  (AtkValue     *obj,
                                       GValue       *value);
ATK_DEPRECATED_IN_2_12_FOR(atk_value_set_value)
gboolean atk_value_set_current_value  (AtkValue     *obj,
                                       const GValue *value);
ATK_DEPRECATED_IN_2_12_FOR(atk_value_get_increment)
void     atk_value_get_minimum_increment  (AtkValue     *obj,
					   GValue       *value);

ATK_AVAILABLE_IN_2_12
void      atk_value_get_value_and_text (AtkValue *obj,
                                        gdouble *value,
                                        gchar  **text);
ATK_AVAILABLE_IN_2_12
AtkRange* atk_value_get_range          (AtkValue *obj);
ATK_AVAILABLE_IN_2_12
gdouble   atk_value_get_increment      (AtkValue *obj);
ATK_AVAILABLE_IN_2_12
GSList*   atk_value_get_sub_ranges     (AtkValue *obj);
ATK_AVAILABLE_IN_2_12
void      atk_value_set_value          (AtkValue     *obj,
                                        const gdouble new_value);
/* AtkValueType methods */
ATK_AVAILABLE_IN_ALL
const gchar* atk_value_type_get_name           (AtkValueType value_type);
ATK_AVAILABLE_IN_ALL
const gchar* atk_value_type_get_localized_name (AtkValueType value_type);

G_END_DECLS

#endif /* __ATK_VALUE_H__ */
