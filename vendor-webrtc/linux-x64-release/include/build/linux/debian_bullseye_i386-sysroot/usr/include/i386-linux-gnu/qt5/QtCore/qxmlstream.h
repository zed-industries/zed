/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtCore module of the Qt Toolkit.
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

#ifndef QXMLSTREAM_H
#define QXMLSTREAM_H

#include <QtCore/qiodevice.h>

#ifndef QT_NO_XMLSTREAM

#include <QtCore/qstring.h>
#include <QtCore/qvector.h>
#include <QtCore/qscopedpointer.h>

QT_BEGIN_NAMESPACE


class Q_CORE_EXPORT QXmlStreamStringRef {
    QString m_string;
    int m_position, m_size;
public:
    inline QXmlStreamStringRef():m_position(0), m_size(0){}
    inline QXmlStreamStringRef(const QStringRef &aString)
        :m_string(aString.string()?*aString.string():QString()), m_position(aString.position()), m_size(aString.size()){}
    QXmlStreamStringRef(const QString &aString) : m_string(aString), m_position(0), m_size(m_string.size()) {}
    QXmlStreamStringRef(QString &&aString) noexcept : m_string(std::move(aString)), m_position(0), m_size(m_string.size()) {}

#if QT_VERSION < QT_VERSION_CHECK(6,0,0)
    QXmlStreamStringRef(const QXmlStreamStringRef &other) // = default
        : m_string(other.m_string), m_position(other.m_position), m_size(other.m_size) {}
    QXmlStreamStringRef(QXmlStreamStringRef &&other) noexcept // = default
        : m_string(std::move(other.m_string)), m_position(other.m_position), m_size(other.m_size) {}
    QXmlStreamStringRef &operator=(QXmlStreamStringRef &&other) noexcept // = default
    { swap(other); return *this; }
    QXmlStreamStringRef &operator=(const QXmlStreamStringRef &other) // = default
    { m_string = other.m_string; m_position = other.m_position; m_size = other.m_size; return *this; }
    inline ~QXmlStreamStringRef() {} // ### this prevents (or deprecates) all the move/copy special member functions,
                                     // ### that's why we need to provide them by hand above. We can't remove it in
                                     // ### Qt 5, since that would change the way its passed to functions. In Qt 6, remove all.
#endif // Qt < 6.0

    void swap(QXmlStreamStringRef &other) noexcept
    {
        qSwap(m_string, other.m_string);
        qSwap(m_position, other.m_position);
        qSwap(m_size, other.m_size);
    }

    inline void clear() { m_string.clear(); m_position = m_size = 0; }
    inline operator QStringRef() const { return QStringRef(&m_string, m_position, m_size); }
    inline const QString *string() const { return &m_string; }
    inline int position() const { return m_position; }
    inline int size() const { return m_size; }
};
Q_DECLARE_SHARED_NOT_MOVABLE_UNTIL_QT6(QXmlStreamStringRef)


class QXmlStreamReaderPrivate;
class QXmlStreamAttributes;
class Q_CORE_EXPORT QXmlStreamAttribute {
    QXmlStreamStringRef m_name, m_namespaceUri, m_qualifiedName, m_value;
#if QT_VERSION < QT_VERSION_CHECK(6, 0, 0)
    void *reserved;
#endif
    uint m_isDefault : 1;
    friend class QXmlStreamReaderPrivate;
    friend class QXmlStreamAttributes;
public:
    QXmlStreamAttribute();
    QXmlStreamAttribute(const QString &qualifiedName, const QString &value);
    QXmlStreamAttribute(const QString &namespaceUri, const QString &name, const QString &value);
#if QT_VERSION < QT_VERSION_CHECK(6, 0, 0)
    QXmlStreamAttribute(const QXmlStreamAttribute &);
    QXmlStreamAttribute(QXmlStreamAttribute &&other) noexcept // = default;
        : m_name(std::move(other.m_name)),
          m_namespaceUri(std::move(other.m_namespaceUri)),
          m_qualifiedName(std::move(other.m_qualifiedName)),
          m_value(std::move(other.m_value)),
          reserved(other.reserved),
          m_isDefault(other.m_isDefault)
    {
        other.reserved = nullptr;
    }
    QXmlStreamAttribute &operator=(QXmlStreamAttribute &&other) noexcept // = default;
    {
        m_name = std::move(other.m_name);
        m_namespaceUri = std::move(other.m_namespaceUri);
        m_qualifiedName = std::move(other.m_qualifiedName);
        m_value = std::move(other.m_value);
        qSwap(reserved, other.reserved);
        m_isDefault = other.m_isDefault;
        return *this;
    }
    QXmlStreamAttribute& operator=(const QXmlStreamAttribute &);
    ~QXmlStreamAttribute();
#endif // < Qt 6

    inline QStringRef namespaceUri() const { return m_namespaceUri; }
    inline QStringRef name() const { return m_name; }
    inline QStringRef qualifiedName() const { return m_qualifiedName; }
    inline QStringRef prefix() const {
        return QStringRef(m_qualifiedName.string(),
                          m_qualifiedName.position(),
                          qMax(0, m_qualifiedName.size() - m_name.size() - 1));
    }
    inline QStringRef value() const { return m_value; }
    inline bool isDefault() const { return m_isDefault; }
    inline bool operator==(const QXmlStreamAttribute &other) const {
        return (value() == other.value()
                && (namespaceUri().isNull() ? (qualifiedName() == other.qualifiedName())
                    : (namespaceUri() == other.namespaceUri() && name() == other.name())));
    }
    inline bool operator!=(const QXmlStreamAttribute &other) const
        { return !operator==(other); }
};

Q_DECLARE_TYPEINFO(QXmlStreamAttribute, Q_MOVABLE_TYPE);

class Q_CORE_EXPORT QXmlStreamAttributes : public QVector<QXmlStreamAttribute>
{
public:
    inline QXmlStreamAttributes() {}
    QStringRef value(const QString &namespaceUri, const QString &name) const;
    QStringRef value(const QString &namespaceUri, QLatin1String name) const;
    QStringRef value(QLatin1String namespaceUri, QLatin1String name) const;
    QStringRef value(const QString &qualifiedName) const;
    QStringRef value(QLatin1String qualifiedName) const;
    void append(const QString &namespaceUri, const QString &name, const QString &value);
    void append(const QString &qualifiedName, const QString &value);

    inline bool hasAttribute(const QString &qualifiedName) const
    {
        return !value(qualifiedName).isNull();
    }

    inline bool hasAttribute(QLatin1String qualifiedName) const
    {
        return !value(qualifiedName).isNull();
    }

    inline bool hasAttribute(const QString &namespaceUri, const QString &name) const
    {
        return !value(namespaceUri, name).isNull();
    }

    using QVector<QXmlStreamAttribute>::append;
};

class Q_CORE_EXPORT QXmlStreamNamespaceDeclaration {
    QXmlStreamStringRef m_prefix, m_namespaceUri;
#if QT_VERSION < QT_VERSION_CHECK(6, 0, 0)
    void *reserved;
#endif

    friend class QXmlStreamReaderPrivate;
public:
    QXmlStreamNamespaceDeclaration();
    QXmlStreamNamespaceDeclaration(const QString &prefix, const QString &namespaceUri);
#if QT_VERSION < QT_VERSION_CHECK(6, 0, 0)
    QXmlStreamNamespaceDeclaration(const QXmlStreamNamespaceDeclaration &);
    QXmlStreamNamespaceDeclaration(QXmlStreamNamespaceDeclaration &&other) noexcept // = default
        : m_prefix(std::move(other.m_prefix)),
          m_namespaceUri(std::move(other.m_namespaceUri)),
          reserved(other.reserved)
    {
        other.reserved = nullptr;
    }
    QXmlStreamNamespaceDeclaration &operator=(QXmlStreamNamespaceDeclaration &&other) noexcept // = default
    {
        m_prefix = std::move(other.m_prefix);
        m_namespaceUri = std::move(other.m_namespaceUri);
        qSwap(reserved, other.reserved);
        return *this;
    }
    ~QXmlStreamNamespaceDeclaration();
    QXmlStreamNamespaceDeclaration& operator=(const QXmlStreamNamespaceDeclaration &);
#endif // < Qt 6

    inline QStringRef prefix() const { return m_prefix; }
    inline QStringRef namespaceUri() const { return m_namespaceUri; }
    inline bool operator==(const QXmlStreamNamespaceDeclaration &other) const {
        return (prefix() == other.prefix() && namespaceUri() == other.namespaceUri());
    }
    inline bool operator!=(const QXmlStreamNamespaceDeclaration &other) const
        { return !operator==(other); }
};

Q_DECLARE_TYPEINFO(QXmlStreamNamespaceDeclaration, Q_MOVABLE_TYPE);
typedef QVector<QXmlStreamNamespaceDeclaration> QXmlStreamNamespaceDeclarations;

class Q_CORE_EXPORT QXmlStreamNotationDeclaration {
    QXmlStreamStringRef m_name, m_systemId, m_publicId;
#if QT_VERSION < QT_VERSION_CHECK(6, 0, 0)
    void *reserved;
#endif

    friend class QXmlStreamReaderPrivate;
public:
    QXmlStreamNotationDeclaration();
#if QT_VERSION < QT_VERSION_CHECK(6, 0, 0)
    ~QXmlStreamNotationDeclaration();
    QXmlStreamNotationDeclaration(const QXmlStreamNotationDeclaration &);
    QXmlStreamNotationDeclaration(QXmlStreamNotationDeclaration &&other) noexcept // = default
        : m_name(std::move(other.m_name)),
          m_systemId(std::move(other.m_systemId)),
          m_publicId(std::move(other.m_publicId)),
          reserved(other.reserved)
    {
        other.reserved = nullptr;
    }
    QXmlStreamNotationDeclaration& operator=(const QXmlStreamNotationDeclaration &);
    QXmlStreamNotationDeclaration &operator=(QXmlStreamNotationDeclaration &&other) noexcept // = default
    {
        m_name = std::move(other.m_name);
        m_systemId = std::move(other.m_systemId);
        m_publicId = std::move(other.m_publicId);
        qSwap(reserved, other.reserved);
        return *this;
    }
#endif // < Qt 6

    inline QStringRef name() const { return m_name; }
    inline QStringRef systemId() const { return m_systemId; }
    inline QStringRef publicId() const { return m_publicId; }
    inline bool operator==(const QXmlStreamNotationDeclaration &other) const {
        return (name() == other.name() && systemId() == other.systemId()
                && publicId() == other.publicId());
    }
    inline bool operator!=(const QXmlStreamNotationDeclaration &other) const
        { return !operator==(other); }
};

Q_DECLARE_TYPEINFO(QXmlStreamNotationDeclaration, Q_MOVABLE_TYPE);
typedef QVector<QXmlStreamNotationDeclaration> QXmlStreamNotationDeclarations;

class Q_CORE_EXPORT QXmlStreamEntityDeclaration {
    QXmlStreamStringRef m_name, m_notationName, m_systemId, m_publicId, m_value;
#if QT_VERSION < QT_VERSION_CHECK(6, 0, 0)
    void *reserved;
#endif

    friend class QXmlStreamReaderPrivate;
public:
    QXmlStreamEntityDeclaration();
#if QT_VERSION < QT_VERSION_CHECK(6, 0, 0)
    ~QXmlStreamEntityDeclaration();
    QXmlStreamEntityDeclaration(const QXmlStreamEntityDeclaration &);
    QXmlStreamEntityDeclaration(QXmlStreamEntityDeclaration &&other) noexcept // = default
        : m_name(std::move(other.m_name)),
          m_notationName(std::move(other.m_notationName)),
          m_systemId(std::move(other.m_systemId)),
          m_publicId(std::move(other.m_publicId)),
          m_value(std::move(other.m_value)),
          reserved(other.reserved)
    {
        other.reserved = nullptr;
    }
    QXmlStreamEntityDeclaration& operator=(const QXmlStreamEntityDeclaration &);
    QXmlStreamEntityDeclaration &operator=(QXmlStreamEntityDeclaration &&other) noexcept // = default
    {
        m_name = std::move(other.m_name);
        m_notationName = std::move(other.m_notationName);
        m_systemId = std::move(other.m_systemId);
        m_publicId = std::move(other.m_publicId);
        m_value = std::move(other.m_value);
        qSwap(reserved, other.reserved);
        return *this;
    }
#endif // < Qt 6

    inline QStringRef name() const { return m_name; }
    inline QStringRef notationName() const { return m_notationName; }
    inline QStringRef systemId() const { return m_systemId; }
    inline QStringRef publicId() const { return m_publicId; }
    inline QStringRef value() const { return m_value; }
    inline bool operator==(const QXmlStreamEntityDeclaration &other) const {
        return (name() == other.name()
                && notationName() == other.notationName()
                && systemId() == other.systemId()
                && publicId() == other.publicId()
                && value() == other.value());
    }
    inline bool operator!=(const QXmlStreamEntityDeclaration &other) const
        { return !operator==(other); }
};

Q_DECLARE_TYPEINFO(QXmlStreamEntityDeclaration, Q_MOVABLE_TYPE);
typedef QVector<QXmlStreamEntityDeclaration> QXmlStreamEntityDeclarations;


class Q_CORE_EXPORT QXmlStreamEntityResolver
{
public:
    virtual ~QXmlStreamEntityResolver();
    virtual QString resolveEntity(const QString& publicId, const QString& systemId);
    virtual QString resolveUndeclaredEntity(const QString &name);
};

#ifndef QT_NO_XMLSTREAMREADER
class Q_CORE_EXPORT QXmlStreamReader {
    QDOC_PROPERTY(bool namespaceProcessing READ namespaceProcessing WRITE setNamespaceProcessing)
public:
    enum TokenType {
        NoToken = 0,
        Invalid,
        StartDocument,
        EndDocument,
        StartElement,
        EndElement,
        Characters,
        Comment,
        DTD,
        EntityReference,
        ProcessingInstruction
    };


    QXmlStreamReader();
    explicit QXmlStreamReader(QIODevice *device);
    explicit QXmlStreamReader(const QByteArray &data);
    explicit QXmlStreamReader(const QString &data);
    explicit QXmlStreamReader(const char * data);
    ~QXmlStreamReader();

    void setDevice(QIODevice *device);
    QIODevice *device() const;
    void addData(const QByteArray &data);
    void addData(const QString &data);
    void addData(const char *data);
    void clear();


    bool atEnd() const;
    TokenType readNext();

    bool readNextStartElement();
    void skipCurrentElement();

    TokenType tokenType() const;
    QString tokenString() const;

    void setNamespaceProcessing(bool);
    bool namespaceProcessing() const;

    inline bool isStartDocument() const { return tokenType() == StartDocument; }
    inline bool isEndDocument() const { return tokenType() == EndDocument; }
    inline bool isStartElement() const { return tokenType() == StartElement; }
    inline bool isEndElement() const { return tokenType() == EndElement; }
    inline bool isCharacters() const { return tokenType() == Characters; }
    bool isWhitespace() const;
    bool isCDATA() const;
    inline bool isComment() const { return tokenType() == Comment; }
    inline bool isDTD() const { return tokenType() == DTD; }
    inline bool isEntityReference() const { return tokenType() == EntityReference; }
    inline bool isProcessingInstruction() const { return tokenType() == ProcessingInstruction; }

    bool isStandaloneDocument() const;
    QStringRef documentVersion() const;
    QStringRef documentEncoding() const;

    qint64 lineNumber() const;
    qint64 columnNumber() const;
    qint64 characterOffset() const;

    QXmlStreamAttributes attributes() const;

    enum ReadElementTextBehaviour {
        ErrorOnUnexpectedElement,
        IncludeChildElements,
        SkipChildElements
    };
    QString readElementText(ReadElementTextBehaviour behaviour = ErrorOnUnexpectedElement);

    QStringRef name() const;
    QStringRef namespaceUri() const;
    QStringRef qualifiedName() const;
    QStringRef prefix() const;

    QStringRef processingInstructionTarget() const;
    QStringRef processingInstructionData() const;

    QStringRef text() const;

    QXmlStreamNamespaceDeclarations namespaceDeclarations() const;
    void addExtraNamespaceDeclaration(const QXmlStreamNamespaceDeclaration &extraNamespaceDeclaraction);
    void addExtraNamespaceDeclarations(const QXmlStreamNamespaceDeclarations &extraNamespaceDeclaractions);
    QXmlStreamNotationDeclarations notationDeclarations() const;
    QXmlStreamEntityDeclarations entityDeclarations() const;
    QStringRef dtdName() const;
    QStringRef dtdPublicId() const;
    QStringRef dtdSystemId() const;

    int entityExpansionLimit() const;
    void setEntityExpansionLimit(int limit);

    enum Error {
        NoError,
        UnexpectedElementError,
        CustomError,
        NotWellFormedError,
        PrematureEndOfDocumentError
    };
    void raiseError(const QString& message = QString());
    QString errorString() const;
    Error error() const;

    inline bool hasError() const
    {
        return error() != NoError;
    }

    void setEntityResolver(QXmlStreamEntityResolver *resolver);
    QXmlStreamEntityResolver *entityResolver() const;

private:
    Q_DISABLE_COPY(QXmlStreamReader)
    Q_DECLARE_PRIVATE(QXmlStreamReader)
    QScopedPointer<QXmlStreamReaderPrivate> d_ptr;

};
#endif // QT_NO_XMLSTREAMREADER

#ifndef QT_NO_XMLSTREAMWRITER

class QXmlStreamWriterPrivate;

class Q_CORE_EXPORT QXmlStreamWriter
{
    QDOC_PROPERTY(bool autoFormatting READ autoFormatting WRITE setAutoFormatting)
    QDOC_PROPERTY(int autoFormattingIndent READ autoFormattingIndent WRITE setAutoFormattingIndent)
public:
    QXmlStreamWriter();
    explicit QXmlStreamWriter(QIODevice *device);
    explicit QXmlStreamWriter(QByteArray *array);
    explicit QXmlStreamWriter(QString *string);
    ~QXmlStreamWriter();

    void setDevice(QIODevice *device);
    QIODevice *device() const;

#if QT_CONFIG(textcodec)
    void setCodec(QTextCodec *codec);
    void setCodec(const char *codecName);
    QTextCodec *codec() const;
#endif

    void setAutoFormatting(bool);
    bool autoFormatting() const;

    void setAutoFormattingIndent(int spacesOrTabs);
    int autoFormattingIndent() const;

    void writeAttribute(const QString &qualifiedName, const QString &value);
    void writeAttribute(const QString &namespaceUri, const QString &name, const QString &value);
    void writeAttribute(const QXmlStreamAttribute& attribute);
    void writeAttributes(const QXmlStreamAttributes& attributes);

    void writeCDATA(const QString &text);
    void writeCharacters(const QString &text);
    void writeComment(const QString &text);

    void writeDTD(const QString &dtd);

    void writeEmptyElement(const QString &qualifiedName);
    void writeEmptyElement(const QString &namespaceUri, const QString &name);

    void writeTextElement(const QString &qualifiedName, const QString &text);
    void writeTextElement(const QString &namespaceUri, const QString &name, const QString &text);

    void writeEndDocument();
    void writeEndElement();

    void writeEntityReference(const QString &name);
    void writeNamespace(const QString &namespaceUri, const QString &prefix = QString());
    void writeDefaultNamespace(const QString &namespaceUri);
    void writeProcessingInstruction(const QString &target, const QString &data = QString());

    void writeStartDocument();
    void writeStartDocument(const QString &version);
    void writeStartDocument(const QString &version, bool standalone);
    void writeStartElement(const QString &qualifiedName);
    void writeStartElement(const QString &namespaceUri, const QString &name);

#ifndef QT_NO_XMLSTREAMREADER
    void writeCurrentToken(const QXmlStreamReader &reader);
#endif

    bool hasError() const;

private:
    Q_DISABLE_COPY(QXmlStreamWriter)
    Q_DECLARE_PRIVATE(QXmlStreamWriter)
    QScopedPointer<QXmlStreamWriterPrivate> d_ptr;
};
#endif // QT_NO_XMLSTREAMWRITER

QT_END_NAMESPACE

#endif // QT_NO_XMLSTREAM
#endif // QXMLSTREAM_H
