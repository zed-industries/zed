// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_LIBXML_CHROMIUM_XML_WRITER_H_
#define THIRD_PARTY_LIBXML_CHROMIUM_XML_WRITER_H_

#include <string>

extern "C" {
struct _xmlBuffer;
struct _xmlTextWriter;
}

// XmlWriter is a wrapper class around libxml's xmlWriter,
// providing a simplified C++ API.
// StartWriting must be called before other methods, and StopWriting
// must be called before GetWrittenString() will return results.
class XmlWriter {
 public:
  XmlWriter();
  ~XmlWriter();

  // Allocates the xmlTextWriter and an xmlBuffer and starts an XML document.
  // This must be called before any other functions. By default, indenting is
  // set to true.
  void StartWriting();

  // Ends the XML document and frees the xmlTextWriter.
  // This must be called before GetWrittenString() is called.
  void StopWriting();

  // Wrappers around libxml functions -----------------------------------------

  // All following elements will be indented to match their depth.
  void StartIndenting();

  // All follow elements will not be indented.
  void StopIndenting();

  // Start an element with the given name. All future elements added will be
  // children of this element, until it is ended. Returns false on error.
  bool StartElement(const std::string& element_name);

  // Ends the current open element. Returns false on error.
  bool EndElement();

  // Appends to the content of the current open element.
  bool AppendElementContent(const std::string& content);

  // Adds an attribute to the current open element. Returns false on error.
  bool AddAttribute(const std::string& attribute_name,
                    const std::string& attribute_value);

  // Adds a new element with name |element_name| and content |content|
  // to the buffer. Example: <|element_name|>|content|</|element_name|>
  // Returns false on errors.
  bool WriteElement(const std::string& element_name,
                    const std::string& content);

  // Helper functions not provided by xmlTextWriter ---------------------------

  // Returns the string that has been written to the buffer.
  std::string GetWrittenString();

 private:
  // The underlying libxml xmlTextWriter.
  _xmlTextWriter* writer_;

  // Stores the output.
  _xmlBuffer* buffer_;
};

#endif  // THIRD_PARTY_LIBXML_CHROMIUM_XML_WRITER_H_
