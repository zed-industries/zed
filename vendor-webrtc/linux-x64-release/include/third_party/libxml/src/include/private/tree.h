#ifndef XML_TREE_H_PRIVATE__
#define XML_TREE_H_PRIVATE__

XML_HIDDEN extern int
xmlRegisterCallbacks;

XML_HIDDEN int
xmlSearchNsSafe(xmlNodePtr node, const xmlChar *href, xmlNsPtr *out);
XML_HIDDEN int
xmlSearchNsByHrefSafe(xmlNodePtr node, const xmlChar *href, xmlNsPtr *out);

XML_HIDDEN int
xmlNodeParseContent(xmlNodePtr node, const xmlChar *content, int len);
XML_HIDDEN xmlNodePtr
xmlStaticCopyNode(xmlNodePtr node, xmlDocPtr doc, xmlNodePtr parent,
                  int extended);
XML_HIDDEN xmlNodePtr
xmlStaticCopyNodeList(xmlNodePtr node, xmlDocPtr doc, xmlNodePtr parent);
XML_HIDDEN const xmlChar *
xmlSplitQName4(const xmlChar *name, xmlChar **prefixPtr);

#endif /* XML_TREE_H_PRIVATE__ */
