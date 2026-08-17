// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_LIBXML_CHROMIUM_XML_READER_H_
#define THIRD_PARTY_LIBXML_CHROMIUM_XML_READER_H_

#include <map>
#include <string>
#include <string_view>

extern "C" {
struct _xmlTextReader;
}

// XmlReader is a wrapper class around libxml's xmlReader,
// providing a simplified C++ API.
class XmlReader {
 public:
  XmlReader();
  ~XmlReader();

  // Load a document into the reader from memory.  |input| must be UTF-8 and
  // exist for the lifetime of this object.  Returns false on error.
  // TODO(evanm): handle encodings other than UTF-8?
  bool Load(std::string_view input);

  // Load a document into the reader from a file.  Returns false on error.
  bool LoadFile(const std::string& file_path);

  // Wrappers around libxml functions -----------------------------------------

  // Read() advances to the next node.  Returns false on EOF or error.
  bool Read();

  // Next(), when pointing at an opening tag, advances to the node after
  // the matching closing tag.  Returns false on EOF or error.
  bool Next();

  // Return the depth in the tree of the current node.
  int Depth();

  // Returns the "local" name of the current node.
  // For a tag like <foo:bar>, this is the string "bar".
  std::string NodeName();

  // Returns the name of the current node.
  // For a tag like <foo:bar>, this is the string "foo:bar".
  std::string NodeFullName();

  // When pointing at a tag, retrieves the value of an attribute.
  // Returns false on failure.
  // E.g. for <foo bar:baz="a">, NodeAttribute("bar:baz", &value)
  // returns true and |value| is set to "a".
  bool NodeAttribute(const char* name, std::string* value);

  // Populates |attributes| with all the attributes of the current tag and
  // returns true. Note that namespace declarations are not reported.
  // Returns false if there are no attributes in the current tag.
  bool GetAllNodeAttributes(std::map<std::string, std::string>* attributes);

  // Populates |namespaces| with all the namespaces (prefix/URI pairs) declared
  // in the current tag and returns true. Note that the default namespace, if
  // declared in the tag, is populated with an empty prefix.
  // Returns false if there are no namespaces declared in the current tag.
  bool GetAllDeclaredNamespaces(std::map<std::string, std::string>* namespaces);

  // Sets |content| to the content of the current node if it is a
  // text, cdata, or significant-whitespace node, respectively.
  // Returns true if the current node is a node of the corresponding, false
  // otherwise.
  bool GetTextIfTextElement(std::string* content);
  bool GetTextIfCDataElement(std::string* content);
  bool GetTextIfSignificantWhitespaceElement(std::string* content);

  // Returns true if the node is an element (e.g. <foo>). Note this returns
  // false for self-closing elements (e.g. <foo/>). Use IsEmptyElement() to
  // check for those.
  bool IsElement();

  // Returns true if the node is a closing element (e.g. </foo>).
  bool IsClosingElement();

  // Returns true if the current node is an empty (self-closing) element (e.g.
  // <foo/>).
  bool IsEmptyElement();

  // Helper functions not provided by libxml ----------------------------------

  // Return the string content within an element.
  // "<foo>bar</foo>" is a sequence of three nodes:
  // (1) open tag, (2) text, (3) close tag.
  // With the reader currently at (1), this returns the text of (2),
  // and advances past (3).
  // Returns false on error.
  bool ReadElementContent(std::string* content);

  // Skip to the next opening tag, returning false if we reach a closing
  // tag or EOF first.
  // If currently on an opening tag, doesn't advance at all.
  bool SkipToElement();

 private:
  // Returns the libxml node type of the current node.
  int NodeType();

  // A helper function for GetTextIf*Element() functions above.
  // Checks if the node is the specified `node_type`, and, if so, populates
  // `content` and returns true.
  bool GetTextFromNodeIfType(int node_type, std::string* content);

  // The underlying libxml xmlTextReader.
  _xmlTextReader* reader_;
};

#endif  // THIRD_PARTY_LIBXML_CHROMIUM_XML_READER_H_
