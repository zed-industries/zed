// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QXMLSTREAM_H
#define QXMLSTREAM_H

#include <QtCore/qiodevice.h>

#ifndef QT_NO_XMLSTREAM

#include <QtCore/qlist.h>
#include <QtCore/qscopedpointer.h>
#include <QtCore/qstring.h>

QT_BEGIN_NAMESPACE

namespace QtPrivate {

class QXmlString {
    QStringPrivate m_string;
public:
    QXmlString(QStringPrivate &&d) : m_string(std::move(d)) {}
    QXmlString(const QString &s) : m_string(s.data_ptr()) {}
    QXmlString & operator=(const QString &s) { m_string = s.data_ptr(); return *this; }
    QXmlString & operator=(QString &&s) { m_string.swap(s.data_ptr()); return *this; }
    inline constexpr QXmlString() {}

    void swap(QXmlString &other) noexcept
    {
        m_string.swap(other.m_string);
    }

    inline operator QStringView() const { return QStringView(m_string.data(), m_string.size); }
    inline qsizetype size() const { return m_string.size; }
};

}
Q_DECLARE_SHARED(QtPrivate::QXmlString)


class QXmlStreamReaderPrivate;
class QXmlStreamAttributes;
class Q_CORE_EXPORT QXmlStreamAttribute {
    QtPrivate::QXmlString m_name, m_namespaceUri, m_qualifiedName, m_value;
    uint m_isDefault : 1;
    friend class QXmlStreamReaderPrivate;
    friend class QXmlStreamAttributes;
public:
    QXmlStreamAttribute();
    QXmlStreamAttribute(const QString &qualifiedName, const QString &value);
    QXmlStreamAttribute(const QString &namespaceUri, const QString &name, const QString &value);

    inline QStringView namespaceUri() const { return m_namespaceUri; }
    inline QStringView name() const { return m_name; }
    inline QStringView qualifiedName() const { return m_qualifiedName; }
    inline QStringView prefix() const {
        return QStringView(m_qualifiedName).left(qMax(0, m_qualifiedName.size() - m_name.size() - 1));
    }
    inline QStringView value() const { return m_value; }
    inline bool isDefault() const { return m_isDefault; }
    inline bool operator==(const QXmlStreamAttribute &other) const {
        return (value() == other.value()
                && (namespaceUri().isNull() ? (qualifiedName() == other.qualifiedName())
                    : (namespaceUri() == other.namespaceUri() && name() == other.name())));
    }
    inline bool operator!=(const QXmlStreamAttribute &other) const
        { return !operator==(other); }
};

Q_DECLARE_TYPEINFO(QXmlStreamAttribute, Q_RELOCATABLE_TYPE);

// We export each out-of-line method individually to prevent MSVC from
// exporting the whole QList class.
class QXmlStreamAttributes : public QList<QXmlStreamAttribute>
{
public:
    inline QXmlStreamAttributes() {}
    Q_CORE_EXPORT QStringView value(const QString &namespaceUri, const QString &name) const;
    Q_CORE_EXPORT QStringView value(const QString &namespaceUri, QLatin1StringView name) const;
    Q_CORE_EXPORT QStringView value(QLatin1StringView namespaceUri, QLatin1StringView name) const;
    Q_CORE_EXPORT QStringView value(const QString &qualifiedName) const;
    Q_CORE_EXPORT QStringView value(QLatin1StringView qualifiedName) const;
    Q_CORE_EXPORT void append(const QString &namespaceUri, const QString &name, const QString &value);
    Q_CORE_EXPORT void append(const QString &qualifiedName, const QString &value);

    inline bool hasAttribute(const QString &qualifiedName) const
    {
        return !value(qualifiedName).isNull();
    }

    inline bool hasAttribute(QLatin1StringView qualifiedName) const
    {
        return !value(qualifiedName).isNull();
    }

    inline bool hasAttribute(const QString &namespaceUri, const QString &name) const
    {
        return !value(namespaceUri, name).isNull();
    }

    using QList<QXmlStreamAttribute>::append;
};

class Q_CORE_EXPORT QXmlStreamNamespaceDeclaration {
    QtPrivate::QXmlString m_prefix, m_namespaceUri;

    friend class QXmlStreamReaderPrivate;
public:
    QXmlStreamNamespaceDeclaration();
    QXmlStreamNamespaceDeclaration(const QString &prefix, const QString &namespaceUri);

    inline QStringView prefix() const { return m_prefix; }
    inline QStringView namespaceUri() const { return m_namespaceUri; }
    inline bool operator==(const QXmlStreamNamespaceDeclaration &other) const {
        return (prefix() == other.prefix() && namespaceUri() == other.namespaceUri());
    }
    inline bool operator!=(const QXmlStreamNamespaceDeclaration &other) const
        { return !operator==(other); }
};

Q_DECLARE_TYPEINFO(QXmlStreamNamespaceDeclaration, Q_RELOCATABLE_TYPE);
typedef QList<QXmlStreamNamespaceDeclaration> QXmlStreamNamespaceDeclarations;

class Q_CORE_EXPORT QXmlStreamNotationDeclaration {
    QtPrivate::QXmlString m_name, m_systemId, m_publicId;

    friend class QXmlStreamReaderPrivate;
public:
    QXmlStreamNotationDeclaration();

    inline QStringView name() const { return m_name; }
    inline QStringView systemId() const { return m_systemId; }
    inline QStringView publicId() const { return m_publicId; }
    inline bool operator==(const QXmlStreamNotationDeclaration &other) const {
        return (name() == other.name() && systemId() == other.systemId()
                && publicId() == other.publicId());
    }
    inline bool operator!=(const QXmlStreamNotationDeclaration &other) const
        { return !operator==(other); }
};

Q_DECLARE_TYPEINFO(QXmlStreamNotationDeclaration, Q_RELOCATABLE_TYPE);
typedef QList<QXmlStreamNotationDeclaration> QXmlStreamNotationDeclarations;

class Q_CORE_EXPORT QXmlStreamEntityDeclaration {
    QtPrivate::QXmlString m_name, m_notationName, m_systemId, m_publicId, m_value;

    friend class QXmlStreamReaderPrivate;
public:
    QXmlStreamEntityDeclaration();

    inline QStringView name() const { return m_name; }
    inline QStringView notationName() const { return m_notationName; }
    inline QStringView systemId() const { return m_systemId; }
    inline QStringView publicId() const { return m_publicId; }
    inline QStringView value() const { return m_value; }
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

Q_DECLARE_TYPEINFO(QXmlStreamEntityDeclaration, Q_RELOCATABLE_TYPE);
typedef QList<QXmlStreamEntityDeclaration> QXmlStreamEntityDeclarations;

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
    QStringView documentVersion() const;
    QStringView documentEncoding() const;

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

    QStringView name() const;
    QStringView namespaceUri() const;
    QStringView qualifiedName() const;
    QStringView prefix() const;

    QStringView processingInstructionTarget() const;
    QStringView processingInstructionData() const;

    QStringView text() const;

    QXmlStreamNamespaceDeclarations namespaceDeclarations() const;
    void addExtraNamespaceDeclaration(const QXmlStreamNamespaceDeclaration &extraNamespaceDeclaraction);
    void addExtraNamespaceDeclarations(const QXmlStreamNamespaceDeclarations &extraNamespaceDeclaractions);
    QXmlStreamNotationDeclarations notationDeclarations() const;
    QXmlStreamEntityDeclarations entityDeclarations() const;
    QStringView dtdName() const;
    QStringView dtdPublicId() const;
    QStringView dtdSystemId() const;

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
