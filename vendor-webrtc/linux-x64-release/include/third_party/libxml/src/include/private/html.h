#ifndef XML_HTML_H_PRIVATE__
#define XML_HTML_H_PRIVATE__

#include <libxml/xmlversion.h>

#ifdef LIBXML_HTML_ENABLED

XML_HIDDEN xmlNodePtr
htmlCtxtParseContentInternal(xmlParserCtxtPtr ctxt, xmlParserInputPtr input);

#endif /* LIBXML_HTML_ENABLED */

#endif /* XML_HTML_H_PRIVATE__ */

