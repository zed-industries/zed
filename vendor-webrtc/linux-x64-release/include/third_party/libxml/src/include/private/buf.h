#ifndef XML_BUF_H_PRIVATE__
#define XML_BUF_H_PRIVATE__

#include <libxml/parser.h>
#include <libxml/tree.h>

XML_HIDDEN xmlBufPtr
xmlBufCreate(size_t size);
XML_HIDDEN xmlBufPtr
xmlBufCreateMem(const xmlChar *mem, size_t size, int isStatic);
XML_HIDDEN void
xmlBufFree(xmlBufPtr buf);

XML_HIDDEN void
xmlBufEmpty(xmlBufPtr buf);

XML_HIDDEN int
xmlBufGrow(xmlBufPtr buf, size_t len);

XML_HIDDEN int
xmlBufAdd(xmlBufPtr buf, const xmlChar *str, size_t len);
XML_HIDDEN int
xmlBufCat(xmlBufPtr buf, const xmlChar *str);

XML_HIDDEN size_t
xmlBufAvail(const xmlBufPtr buf);
XML_HIDDEN int
xmlBufIsEmpty(const xmlBufPtr buf);
XML_HIDDEN int
xmlBufAddLen(xmlBufPtr buf, size_t len);

XML_HIDDEN xmlChar *
xmlBufDetach(xmlBufPtr buf);

XML_HIDDEN xmlBufPtr
xmlBufFromBuffer(xmlBufferPtr buffer);
XML_HIDDEN int
xmlBufBackToBuffer(xmlBufPtr buf, xmlBufferPtr ret);

XML_HIDDEN int
xmlBufResetInput(xmlBufPtr buf, xmlParserInputPtr input);
XML_HIDDEN int
xmlBufUpdateInput(xmlBufPtr buf, xmlParserInputPtr input, size_t pos);

#endif /* XML_BUF_H_PRIVATE__ */
