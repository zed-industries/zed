/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtXml module of the Qt Toolkit.
**
** $QT_BEGIN_LICENSE:LGPL$
** Commercial License Usage
** Licensees holding valid commercial Qt licenses may use this file in
** accordance with the commercial license agreement provided with the
** Software or, alternatively, in accordance with the terms contained in
** a written agreement between you and The Qt Company. For licensing terms
** and conditions see https://www.qt.io/terms-conditions. For further
** information use the contact form at https://www.qt.io/contact-us.
**
** GNU Lesser General Public License Usage
** Alternatively, this file may be used under the terms of the GNU Lesser
** General Public License version 3 as published by the Free Software
** Foundation and appearing in the file LICENSE.LGPL3 included in the
** packaging of this file. Please review the following information to
** ensure the GNU Lesser General Public License version 3 requirements
** will be met: https://www.gnu.org/licenses/lgpl-3.0.html.
**
** GNU General Public License Usage
** Alternatively, this file may be used under the terms of the GNU
** General Public License version 2.0 or (at your option) the GNU General
** Public license version 3 or any later version approved by the KDE Free
** Qt Foundation. The licenses are as published by the Free Software
** Foundation and appearing in the file LICENSE.GPL2 and LICENSE.GPL3
** included in the packaging of this file. Please review the following
** information to ensure the GNU General Public License requirements will
** be met: https://www.gnu.org/licenses/gpl-2.0.html and
** https://www.gnu.org/licenses/gpl-3.0.html.
**
** $QT_END_LICENSE$
**
****************************************************************************/

#ifndef QDOM_H
#define QDOM_H

#include <QtXml/qtxmlglobal.h>
#include <QtCore/qstring.h>

QT_BEGIN_NAMESPACE


#ifndef QT_NO_DOM

class QIODevice;
class QTextStream;

class QXmlInputSource;
class QXmlReader;

class QDomDocumentPrivate;
class QDomDocumentTypePrivate;
class QDomDocumentFragmentPrivate;
class QDomNodePrivate;
class QDomNodeListPrivate;
class QDomImplementationPrivate;
class QDomElementPrivate;
class QDomNotationPrivate;
class QDomEntityPrivate;
class QDomEntityReferencePrivate;
class QDomProcessingInstructionPrivate;
class QDomAttrPrivate;
class QDomCharacterDataPrivate;
class QDomTextPrivate;
class QDomCommentPrivate;
class QDomCDATASectionPrivate;
class QDomNamedNodeMapPrivate;
class QDomImplementationPrivate;

class QDomNodeList;
class QDomElement;
class QDomText;
class QDomComment;
class QDomCDATASection;
class QDomProcessingInstruction;
class QDomAttr;
class QDomEntityReference;
class QDomDocument;
class QDomNamedNodeMap;
class QDomDocument;
class QDomDocumentFragment;
class QDomDocumentType;
class QDomImplementation;
class QDomNode;
class QDomEntity;
class QDomNotation;
class QDomCharacterData;
class QXmlStreamReader;

class Q_XML_EXPORT QDomImplementation
{
public:
    QDomImplementation();
    QDomImplementation(const QDomImplementation&);
    ~QDomImplementation();
    QDomImplementation& operator= (const QDomImplementation&);
    bool operator== (const QDomImplementation&) const;
    bool operator!= (const QDomImplementation&) const;

    // functions
    bool hasFeature(const QString& feature, const QString& version) const;
    QDomDocumentType createDocumentType(const QString& qName, const QString& publicId, const QString& systemId);
    QDomDocument createDocument(const QString& nsURI, const QString& qName, const QDomDocumentType& doctype);

    enum InvalidDataPolicy { AcceptInvalidChars = 0, DropInvalidChars, ReturnNullNode };
    static InvalidDataPolicy invalidDataPolicy();
    static void setInvalidDataPolicy(InvalidDataPolicy policy);

    // Qt extension
    bool isNull();

private:
    QDomImplementationPrivate* impl;
    QDomImplementation(QDomImplementationPrivate*);

    friend class QDomDocument;
};

class Q_XML_EXPORT QDomNode
{
public:
    enum NodeType {
        ElementNode               = 1,
        AttributeNode             = 2,
        TextNode                  = 3,
        CDATASectionNode          = 4,
        EntityReferenceNode       = 5,
        EntityNode                = 6,
        ProcessingInstructionNode = 7,
        CommentNode               = 8,
        DocumentNode              = 9,
        DocumentTypeNode          = 10,
        DocumentFragmentNode      = 11,
        NotationNode              = 12,
        BaseNode                  = 21,// this is not in the standard
        CharacterDataNode         = 22 // this is not in the standard
    };

    enum EncodingPolicy
    {
        EncodingFromDocument      = 1,
        EncodingFromTextStream    = 2
    };

    QDomNode();
    QDomNode(const QDomNode&);
    QDomNode& operator= (const QDomNode&);
    bool operator== (const QDomNode&) const;
    bool operator!= (const QDomNode&) const;
    ~QDomNode();

    // DOM functions
    QDomNode insertBefore(const QDomNode& newChild, const QDomNode& refChild);
    QDomNode insertAfter(const QDomNode& newChild, const QDomNode& refChild);
    QDomNode replaceChild(const QDomNode& newChild, const QDomNode& oldChild);
    QDomNode removeChild(const QDomNode& oldChild);
    QDomNode appendChild(const QDomNode& newChild);
    bool hasChildNodes() const;
    QDomNode cloneNode(bool deep = true) const;
    void normalize();
    bool isSupported(const QString& feature, const QString& version) const;

    // DOM read-only attributes
    QString nodeName() const;
    NodeType nodeType() const;
    QDomNode parentNode() const;
    QDomNodeList childNodes() const;
    QDomNode firstChild() const;
    QDomNode lastChild() const;
    QDomNode previousSibling() const;
    QDomNode nextSibling() const;
    QDomNamedNodeMap attributes() const;
    QDomDocument ownerDocument() const;
    QString namespaceURI() const;
    QString localName() const;
    bool hasAttributes() const;

    // DOM attributes
    QString nodeValue() const;
    void setNodeValue(const QString&);
    QString prefix() const;
    void setPrefix(const QString& pre);

    // Qt extensions
    bool isAttr() const;
    bool isCDATASection() const;
    bool isDocumentFragment() const;
    bool isDocument() const;
    bool isDocumentType() const;
    bool isElement() const;
    bool isEntityReference() const;
    bool isText() const;
    bool isEntity() const;
    bool isNotation() const;
    bool isProcessingInstruction() const;
    bool isCharacterData() const;
    bool isComment() const;

    /**
     * Shortcut to avoid dealing with QDomNodeList
     * all the time.
     */
    QDomNode namedItem(const QString& name) const;

    bool isNull() const;
    void clear();

    QDomAttr toAttr() const;
    QDomCDATASection toCDATASection() const;
    QDomDocumentFragment toDocumentFragment() const;
    QDomDocument toDocument() const;
    QDomDocumentType toDocumentType() const;
    QDomElement toElement() const;
    QDomEntityReference toEntityReference() const;
    QDomText toText() const;
    QDomEntity toEntity() const;
    QDomNotation toNotation() const;
    QDomProcessingInstruction toProcessingInstruction() const;
    QDomCharacterData toCharacterData() const;
    QDomComment toComment() const;

    void save(QTextStream&, int, EncodingPolicy=QDomNode::EncodingFromDocument) const;

    QDomElement firstChildElement(const QString &tagName = QString()) const;
    QDomElement lastChildElement(const QString &tagName = QString()) const;
    QDomElement previousSiblingElement(const QString &tagName = QString()) const;
    QDomElement nextSiblingElement(const QString &taName = QString()) const;

    int lineNumber() const;
    int columnNumber() const;

protected:
    QDomNodePrivate* impl;
    QDomNode(QDomNodePrivate*);

private:
    friend class QDomDocument;
    friend class QDomDocumentType;
    friend class QDomNodeList;
    friend class QDomNamedNodeMap;
};

class Q_XML_EXPORT QDomNodeList
{
public:
    QDomNodeList();
    QDomNodeList(const QDomNodeList&);
    QDomNodeList& operator= (const QDomNodeList&);
    bool operator== (const QDomNodeList&) const;
    bool operator!= (const QDomNodeList&) const;
    ~QDomNodeList();

    // DOM functions
    QDomNode item(int index) const;
    inline QDomNode at(int index) const { return item(index); } // Qt API consistency

    // DOM read only attributes
    int length() const;
    inline int count() const { return length(); } // Qt API consitancy
    inline int size() const { return length(); } // Qt API consistency
    inline bool isEmpty() const { return length() == 0; } // Qt API consistency

private:
    QDomNodeListPrivate* impl;
    QDomNodeList(QDomNodeListPrivate*);

    friend class QDomNode;
    friend class QDomElement;
    friend class QDomDocument;
};

class Q_XML_EXPORT QDomDocumentType : public QDomNode
{
public:
    QDomDocumentType();
    QDomDocumentType(const QDomDocumentType& x);
    QDomDocumentType& operator= (const QDomDocumentType&);

    // DOM read only attributes
    QString name() const;
    QDomNamedNodeMap entities() const;
    QDomNamedNodeMap notations() const;
    QString publicId() const;
    QString systemId() const;
    QString internalSubset() const;

    // Overridden from QDomNode
    inline QDomNode::NodeType nodeType() const { return DocumentTypeNode; }

private:
    QDomDocumentType(QDomDocumentTypePrivate*);

    friend class QDomImplementation;
    friend class QDomDocument;
    friend class QDomNode;
};

class Q_XML_EXPORT QDomDocument : public QDomNode
{
public:
    QDomDocument();
    explicit QDomDocument(const QString& name);
    explicit QDomDocument(const QDomDocumentType& doctype);
    QDomDocument(const QDomDocument& x);
    QDomDocument& operator= (const QDomDocument&);
    ~QDomDocument();

    // DOM functions
    QDomElement createElement(const QString& tagName);
    QDomDocumentFragment createDocumentFragment();
    QDomText createTextNode(const QString& data);
    QDomComment createComment(const QString& data);
    QDomCDATASection createCDATASection(const QString& data);
    QDomProcessingInstruction createProcessingInstruction(const QString& target, const QString& data);
    QDomAttr createAttribute(const QString& name);
    QDomEntityReference createEntityReference(const QString& name);
    QDomNodeList elementsByTagName(const QString& tagname) const;
    QDomNode importNode(const QDomNode& importedNode, bool deep);
    QDomElement createElementNS(const QString& nsURI, const QString& qName);
    QDomAttr createAttributeNS(const QString& nsURI, const QString& qName);
    QDomNodeList elementsByTagNameNS(const QString& nsURI, const QString& localName);
    QDomElement elementById(const QString& elementId);

    // DOM read only attributes
    QDomDocumentType doctype() const;
    QDomImplementation implementation() const;
    QDomElement documentElement() const;

    // Overridden from QDomNode
    inline QDomNode::NodeType nodeType() const { return DocumentNode; }

    // Qt extensions
    bool setContent(const QByteArray& text, bool namespaceProcessing, QString *errorMsg=nullptr, int *errorLine=nullptr, int *errorColumn=nullptr );
    bool setContent(const QString& text, bool namespaceProcessing, QString *errorMsg=nullptr, int *errorLine=nullptr, int *errorColumn=nullptr );
    bool setContent(QIODevice* dev, bool namespaceProcessing, QString *errorMsg=nullptr, int *errorLine=nullptr, int *errorColumn=nullptr );
#if QT_DEPRECATED_SINCE(5, 15)
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
    QT_DEPRECATED_X("Use other overloads instead")
    bool setContent(QXmlInputSource *source, bool namespaceProcessing, QString *errorMsg=nullptr, int *errorLine=nullptr, int *errorColumn=nullptr );
QT_WARNING_POP
#endif
    bool setContent(const QByteArray& text, QString *errorMsg=nullptr, int *errorLine=nullptr, int *errorColumn=nullptr );
    bool setContent(const QString& text, QString *errorMsg=nullptr, int *errorLine=nullptr, int *errorColumn=nullptr );
    bool setContent(QIODevice* dev, QString *errorMsg=nullptr, int *errorLine=nullptr, int *errorColumn=nullptr );
#if QT_DEPRECATED_SINCE(5, 15)
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
    QT_DEPRECATED_X("Use other overloads instead")
    bool setContent(QXmlInputSource *source, QXmlReader *reader, QString *errorMsg=nullptr, int *errorLine=nullptr, int *errorColumn=nullptr );
QT_WARNING_POP
#endif
    bool setContent(QXmlStreamReader *reader, bool namespaceProcessing, QString *errorMsg = nullptr,
                    int *errorLine = nullptr, int *errorColumn = nullptr);

    // Qt extensions
    QString toString(int = 1) const;
    QByteArray toByteArray(int = 1) const;

private:
    QDomDocument(QDomDocumentPrivate*);

    friend class QDomNode;
};

class Q_XML_EXPORT QDomNamedNodeMap
{
public:
    QDomNamedNodeMap();
    QDomNamedNodeMap(const QDomNamedNodeMap&);
    QDomNamedNodeMap& operator= (const QDomNamedNodeMap&);
    bool operator== (const QDomNamedNodeMap&) const;
    bool operator!= (const QDomNamedNodeMap&) const;
    ~QDomNamedNodeMap();

    // DOM functions
    QDomNode namedItem(const QString& name) const;
    QDomNode setNamedItem(const QDomNode& newNode);
    QDomNode removeNamedItem(const QString& name);
    QDomNode item(int index) const;
    QDomNode namedItemNS(const QString& nsURI, const QString& localName) const;
    QDomNode setNamedItemNS(const QDomNode& newNode);
    QDomNode removeNamedItemNS(const QString& nsURI, const QString& localName);

    // DOM read only attributes
    int length() const;
    int count() const { return length(); } // Qt API consitancy
    inline int size() const { return length(); } // Qt API consistency
    inline bool isEmpty() const { return length() == 0; } // Qt API consistency

    // Qt extension
    bool contains(const QString& name) const;

private:
    QDomNamedNodeMapPrivate* impl;
    QDomNamedNodeMap(QDomNamedNodeMapPrivate*);

    friend class QDomNode;
    friend class QDomDocumentType;
    friend class QDomElement;
};

class Q_XML_EXPORT QDomDocumentFragment : public QDomNode
{
public:
    QDomDocumentFragment();
    QDomDocumentFragment(const QDomDocumentFragment& x);
    QDomDocumentFragment& operator= (const QDomDocumentFragment&);

    // Overridden from QDomNode
    inline QDomNode::NodeType nodeType() const { return DocumentFragmentNode; }

private:
    QDomDocumentFragment(QDomDocumentFragmentPrivate*);

    friend class QDomDocument;
    friend class QDomNode;
};

class Q_XML_EXPORT QDomCharacterData : public QDomNode
{
public:
    QDomCharacterData();
    QDomCharacterData(const QDomCharacterData& x);
    QDomCharacterData& operator= (const QDomCharacterData&);

    // DOM functions
    QString substringData(unsigned long offset, unsigned long count);
    void appendData(const QString& arg);
    void insertData(unsigned long offset, const QString& arg);
    void deleteData(unsigned long offset, unsigned long count);
    void replaceData(unsigned long offset, unsigned long count, const QString& arg);

    // DOM read only attributes
    int length() const;

    // DOM attributes
    QString data() const;
    void setData(const QString&);

    // Overridden from QDomNode
    QDomNode::NodeType nodeType() const;

private:
    QDomCharacterData(QDomCharacterDataPrivate*);

    friend class QDomDocument;
    friend class QDomText;
    friend class QDomComment;
    friend class QDomNode;
};

class Q_XML_EXPORT QDomAttr : public QDomNode
{
public:
    QDomAttr();
    QDomAttr(const QDomAttr& x);
    QDomAttr& operator= (const QDomAttr&);

    // DOM read only attributes
    QString name() const;
    bool specified() const;
    QDomElement ownerElement() const;

    // DOM attributes
    QString value() const;
    void setValue(const QString&);

    // Overridden from QDomNode
    inline QDomNode::NodeType nodeType() const { return AttributeNode; }

private:
    QDomAttr(QDomAttrPrivate*);

    friend class QDomDocument;
    friend class QDomElement;
    friend class QDomNode;
};

class Q_XML_EXPORT QDomElement : public QDomNode
{
public:
    QDomElement();
    QDomElement(const QDomElement& x);
    QDomElement& operator= (const QDomElement&);

    // DOM functions
    QString attribute(const QString& name, const QString& defValue = QString() ) const;
    void setAttribute(const QString& name, const QString& value);
    void setAttribute(const QString& name, qlonglong value);
    void setAttribute(const QString& name, qulonglong value);
    inline void setAttribute(const QString& name, int value)
        { setAttribute(name, qlonglong(value)); }
    inline void setAttribute(const QString& name, uint value)
        { setAttribute(name, qulonglong(value)); }
    void setAttribute(const QString& name, float value);
    void setAttribute(const QString& name, double value);
    void removeAttribute(const QString& name);
    QDomAttr attributeNode(const QString& name);
    QDomAttr setAttributeNode(const QDomAttr& newAttr);
    QDomAttr removeAttributeNode(const QDomAttr& oldAttr);
    QDomNodeList elementsByTagName(const QString& tagname) const;
    bool hasAttribute(const QString& name) const;

    QString attributeNS(const QString nsURI, const QString& localName, const QString& defValue = QString()) const;
    void setAttributeNS(const QString nsURI, const QString& qName, const QString& value);
    inline void setAttributeNS(const QString nsURI, const QString& qName, int value)
        { setAttributeNS(nsURI, qName, qlonglong(value)); }
    inline void setAttributeNS(const QString nsURI, const QString& qName, uint value)
        { setAttributeNS(nsURI, qName, qulonglong(value)); }
    void setAttributeNS(const QString nsURI, const QString& qName, qlonglong value);
    void setAttributeNS(const QString nsURI, const QString& qName, qulonglong value);
    void setAttributeNS(const QString nsURI, const QString& qName, double value);
    void removeAttributeNS(const QString& nsURI, const QString& localName);
    QDomAttr attributeNodeNS(const QString& nsURI, const QString& localName);
    QDomAttr setAttributeNodeNS(const QDomAttr& newAttr);
    QDomNodeList elementsByTagNameNS(const QString& nsURI, const QString& localName) const;
    bool hasAttributeNS(const QString& nsURI, const QString& localName) const;

    // DOM read only attributes
    QString tagName() const;
    void setTagName(const QString& name); // Qt extension

    // Overridden from QDomNode
    QDomNamedNodeMap attributes() const;
    inline QDomNode::NodeType nodeType() const { return ElementNode; }

    QString text() const;

private:
    QDomElement(QDomElementPrivate*);

    friend class QDomDocument;
    friend class QDomNode;
    friend class QDomAttr;
};

class Q_XML_EXPORT QDomText : public QDomCharacterData
{
public:
    QDomText();
    QDomText(const QDomText& x);
    QDomText& operator= (const QDomText&);

    // DOM functions
    QDomText splitText(int offset);

    // Overridden from QDomCharacterData
    inline QDomNode::NodeType nodeType() const { return TextNode; }

private:
    QDomText(QDomTextPrivate*);

    friend class QDomCDATASection;
    friend class QDomDocument;
    friend class QDomNode;
};

class Q_XML_EXPORT QDomComment : public QDomCharacterData
{
public:
    QDomComment();
    QDomComment(const QDomComment& x);
    QDomComment& operator= (const QDomComment&);

    // Overridden from QDomCharacterData
    inline QDomNode::NodeType nodeType() const { return CommentNode; }

private:
    QDomComment(QDomCommentPrivate*);

    friend class QDomDocument;
    friend class QDomNode;
};

class Q_XML_EXPORT QDomCDATASection : public QDomText
{
public:
    QDomCDATASection();
    QDomCDATASection(const QDomCDATASection& x);
    QDomCDATASection& operator= (const QDomCDATASection&);

    // Overridden from QDomText
    inline QDomNode::NodeType nodeType() const { return CDATASectionNode; }

private:
    QDomCDATASection(QDomCDATASectionPrivate*);

    friend class QDomDocument;
    friend class QDomNode;
};

class Q_XML_EXPORT QDomNotation : public QDomNode
{
public:
    QDomNotation();
    QDomNotation(const QDomNotation& x);
    QDomNotation& operator= (const QDomNotation&);

    // DOM read only attributes
    QString publicId() const;
    QString systemId() const;

    // Overridden from QDomNode
    inline QDomNode::NodeType nodeType() const { return NotationNode; }

private:
    QDomNotation(QDomNotationPrivate*);

    friend class QDomDocument;
    friend class QDomNode;
};

class Q_XML_EXPORT QDomEntity : public QDomNode
{
public:
    QDomEntity();
    QDomEntity(const QDomEntity& x);
    QDomEntity& operator= (const QDomEntity&);

    // DOM read only attributes
    QString publicId() const;
    QString systemId() const;
    QString notationName() const;

    // Overridden from QDomNode
    inline QDomNode::NodeType nodeType() const { return EntityNode; }

private:
    QDomEntity(QDomEntityPrivate*);

    friend class QDomNode;
};

class Q_XML_EXPORT QDomEntityReference : public QDomNode
{
public:
    QDomEntityReference();
    QDomEntityReference(const QDomEntityReference& x);
    QDomEntityReference& operator= (const QDomEntityReference&);

    // Overridden from QDomNode
    inline QDomNode::NodeType nodeType() const { return EntityReferenceNode; }

private:
    QDomEntityReference(QDomEntityReferencePrivate*);

    friend class QDomDocument;
    friend class QDomNode;
};

class Q_XML_EXPORT QDomProcessingInstruction : public QDomNode
{
public:
    QDomProcessingInstruction();
    QDomProcessingInstruction(const QDomProcessingInstruction& x);
    QDomProcessingInstruction& operator= (const QDomProcessingInstruction&);

    // DOM read only attributes
    QString target() const;

    // DOM attributes
    QString data() const;
    void setData(const QString& d);

    // Overridden from QDomNode
    inline QDomNode::NodeType nodeType() const { return ProcessingInstructionNode; }

private:
    QDomProcessingInstruction(QDomProcessingInstructionPrivate*);

    friend class QDomDocument;
    friend class QDomNode;
};


Q_XML_EXPORT QTextStream& operator<<(QTextStream&, const QDomNode&);

#endif // QT_NO_DOM

QT_END_NAMESPACE

#endif // QDOM_H
