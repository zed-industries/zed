#ifndef XML_REGEXP_H_PRIVATE__
#define XML_REGEXP_H_PRIVATE__

#include <libxml/xmlautomata.h>

#ifdef LIBXML_REGEXP_ENABLED

/*
 * -2 and -3 are used by xmlValidateElementType for other things.
 */
#define XML_REGEXP_OK               0
#define XML_REGEXP_NOT_FOUND        (-1)
#define XML_REGEXP_INTERNAL_ERROR   (-4)
#define XML_REGEXP_OUT_OF_MEMORY    (-5)
#define XML_REGEXP_INTERNAL_LIMIT   (-6)
#define XML_REGEXP_INVALID_UTF8     (-7)

XML_HIDDEN void
xmlAutomataSetFlags(xmlAutomataPtr am, int flags);

#endif /* LIBXML_REGEXP_ENABLED */

#endif /* XML_REGEXP_H_PRIVATE__ */
