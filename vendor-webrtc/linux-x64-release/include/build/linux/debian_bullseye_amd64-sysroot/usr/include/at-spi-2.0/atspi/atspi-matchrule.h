/*
 * AT-SPI - Assistive Technology Service Provider Interface
 * (Gnome Accessibility Project; http://developer.gnome.org/projects/gap)
 *
 * Copyright 2002 Ximian, Inc.
 *           2002 Sun Microsystems Inc.
 * Copyright 2010, 2011 Novell, Inc.
 *           
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 */

#ifndef _ATSPI_MATCH_RULE_H_
#define _ATSPI_MATCH_RULE_H_

#include "glib-object.h"

#include "atspi-stateset.h"
#include "atspi-constants.h"
#include "atspi-types.h"

G_BEGIN_DECLS

#define ATSPI_TYPE_MATCH_RULE                        (atspi_match_rule_get_type ())
#define ATSPI_MATCH_RULE(obj)                        (G_TYPE_CHECK_INSTANCE_CAST ((obj), ATSPI_TYPE_MATCH_RULE, AtspiMatchRule))
#define ATSPI_MATCH_RULE_CLASS(klass)                (G_TYPE_CHECK_CLASS_CAST ((klass), ATSPI_TYPE_MATCH_RULE, AtspiMatchRuleClass))
#define ATSPI_IS_MATCH_RULE(obj)                     (G_TYPE_CHECK_INSTANCE_TYPE ((obj), ATSPI_TYPE_MATCH_RULE))
#define ATSPI_IS_MATCH_RULE_CLASS(klass)             (G_TYPE_CHECK_CLASS_TYPE ((klass), ATSPI_TYPE_MATCH_RULE))
#define ATSPI_MATCH_RULE_GET_CLASS(obj)              (G_TYPE_INSTANCE_GET_CLASS ((obj), ATSPI_TYPE_MATCH_RULE, AtspiMatchRuleClass))

typedef struct _AtspiMatchRule AtspiMatchRule;
struct _AtspiMatchRule
{
  GObject parent;
  AtspiStateSet *states;
  AtspiCollectionMatchType statematchtype;
  GHashTable *attributes;
  AtspiCollectionMatchType attributematchtype;
  GArray *interfaces;
  AtspiCollectionMatchType interfacematchtype;
  gint roles [4];
  AtspiCollectionMatchType rolematchtype;
  gboolean invert;
};

typedef struct _AtspiMatchRuleClass AtspiMatchRuleClass;
struct _AtspiMatchRuleClass
{
  GObjectClass parent_class;
};

GType atspi_match_rule_get_type ();

AtspiMatchRule *
atspi_match_rule_new (AtspiStateSet *states,
                      AtspiCollectionMatchType statematchtype,
                      GHashTable *attributes,
                      AtspiCollectionMatchType attributematchtype,
                      GArray *roles,
                      AtspiCollectionMatchType rolematchtype,
                      GArray *interfaces,
                      AtspiCollectionMatchType interfacematchtype,
                      gboolean invert);

G_END_DECLS

#endif	/* _ATSPI_MATCH_RULE_H_ */
