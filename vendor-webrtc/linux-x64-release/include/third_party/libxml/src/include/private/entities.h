#ifndef XML_ENTITIES_H_PRIVATE__
#define XML_ENTITIES_H_PRIVATE__

#include <libxml/tree.h>
#include <libxml/xmlstring.h>

/*
 * Entity flags
 *
 * XML_ENT_PARSED: The entity was parsed and `children` points to the
 * content.
 *
 * XML_ENT_CHECKED: The entity was checked for loops and amplification.
 * expandedSize was set.
 *
 * XML_ENT_VALIDATED: The entity contains a valid attribute value.
 * Only used when entities aren't substituted.
 */
#define XML_ENT_PARSED      (1u << 0)
#define XML_ENT_CHECKED     (1u << 1)
#define XML_ENT_VALIDATED   (1u << 2)
#define XML_ENT_EXPANDING   (1u << 3)

#define XML_ESCAPE_ATTR             (1u << 0)
#define XML_ESCAPE_NON_ASCII        (1u << 1)
#define XML_ESCAPE_HTML             (1u << 2)
#define XML_ESCAPE_QUOT             (1u << 3)
#define XML_ESCAPE_ALLOW_INVALID    (1u << 4)

XML_HIDDEN int
xmlSerializeHexCharRef(char *buf, int val);
XML_HIDDEN int
xmlSerializeDecCharRef(char *buf, int val);

XML_HIDDEN xmlChar *
xmlEscapeText(const xmlChar *text, int flags);

XML_HIDDEN xmlChar *
xmlEncodeEntitiesInternal(xmlDocPtr doc, const xmlChar *input,
                          unsigned flags);

#endif /* XML_ENTITIES_H_PRIVATE__ */
