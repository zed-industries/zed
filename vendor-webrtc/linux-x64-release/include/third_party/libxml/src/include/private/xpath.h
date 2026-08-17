#ifndef XML_XPATH_H_PRIVATE__
#define XML_XPATH_H_PRIVATE__

#include <libxml/xpath.h>

XML_HIDDEN void
xmlInitXPathInternal(void);

#ifdef LIBXML_XPATH_ENABLED
XML_HIDDEN void
xmlXPathErrMemory(xmlXPathContextPtr ctxt);
XML_HIDDEN void
xmlXPathPErrMemory(xmlXPathParserContextPtr ctxt);
#endif

#endif /* XML_XPATH_H_PRIVATE__ */
