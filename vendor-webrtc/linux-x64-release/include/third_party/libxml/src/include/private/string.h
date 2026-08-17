#ifndef XML_STRING_H_PRIVATE__
#define XML_STRING_H_PRIVATE__

#include <libxml/xmlstring.h>

XML_HIDDEN int
xmlStrVASPrintf(xmlChar **out, int maxSize, const char *msg, va_list ap);
XML_HIDDEN int
xmlStrASPrintf(xmlChar **out, int maxSize, const char *msg, ...);
XML_HIDDEN xmlChar *
xmlEscapeFormatString(xmlChar **msg);

#endif /* XML_STRING_H_PRIVATE__ */
