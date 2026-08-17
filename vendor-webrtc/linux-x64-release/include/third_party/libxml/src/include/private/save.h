#ifndef XML_SAVE_H_PRIVATE__
#define XML_SAVE_H_PRIVATE__

#include <libxml/tree.h>
#include <libxml/xmlsave.h>
#include <libxml/xmlversion.h>

#ifdef LIBXML_OUTPUT_ENABLED

XML_HIDDEN int
xmlSaveNotationDecl(xmlSaveCtxtPtr ctxt, xmlNotationPtr cur);
XML_HIDDEN int
xmlSaveNotationTable(xmlSaveCtxtPtr ctxt, xmlNotationTablePtr cur);

XML_HIDDEN void
xmlBufAttrSerializeTxtContent(xmlOutputBufferPtr buf, xmlDocPtr doc,
                              const xmlChar *string);
XML_HIDDEN void
xmlNsListDumpOutput(xmlOutputBufferPtr buf, xmlNsPtr cur);

#endif /* LIBXML_OUTPUT_ENABLED */

#endif /* XML_SAVE_H_PRIVATE__ */

