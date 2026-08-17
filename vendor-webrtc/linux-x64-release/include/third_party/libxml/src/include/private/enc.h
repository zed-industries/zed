#ifndef XML_ENC_H_PRIVATE__
#define XML_ENC_H_PRIVATE__

#include <libxml/encoding.h>
#include <libxml/tree.h>

XML_HIDDEN void
xmlInitEncodingInternal(void);

XML_HIDDEN xmlCharEncError
xmlEncInputChunk(xmlCharEncodingHandler *handler, unsigned char *out,
                 int *outlen, const unsigned char *in, int *inlen,
                 int flush);
XML_HIDDEN xmlCharEncError
xmlCharEncInput(xmlParserInputBufferPtr input, size_t *sizeOut, int flush);
XML_HIDDEN int
xmlCharEncOutput(xmlOutputBufferPtr output, int init);

#endif /* XML_ENC_H_PRIVATE__ */
