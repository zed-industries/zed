// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QTEXTSTREAM_H
#define QTEXTSTREAM_H

#include <QtCore/qiodevicebase.h>
#include <QtCore/qchar.h>
#include <QtCore/qscopedpointer.h>
#include <QtCore/qstringconverter_base.h>

#include <stdio.h>

#ifdef Status
#error qtextstream.h must be included before any header file that defines Status
#endif

QT_BEGIN_NAMESPACE

class QIODevice;
class QLocale;
class QString;

class QTextStreamPrivate;
class Q_CORE_EXPORT QTextStream : public QIODeviceBase
{
    Q_DECLARE_PRIVATE(QTextStream)

public:
    enum RealNumberNotation {
        SmartNotation,
        FixedNotation,
        ScientificNotation
    };
    enum FieldAlignment {
        AlignLeft,
        AlignRight,
        AlignCenter,
        AlignAccountingStyle
    };
    enum Status {
        Ok,
        ReadPastEnd,
        ReadCorruptData,
        WriteFailed
    };
    enum NumberFlag {
        ShowBase = 0x1,
        ForcePoint = 0x2,
        ForceSign = 0x4,
        UppercaseBase = 0x8,
        UppercaseDigits = 0x10
    };
    Q_DECLARE_FLAGS(NumberFlags, NumberFlag)

    QTextStream();
    explicit QTextStream(QIODevice *device);
    explicit QTextStream(FILE *fileHandle, OpenMode openMode = ReadWrite);
    explicit QTextStream(QString *string, OpenMode openMode = ReadWrite);
    explicit QTextStream(QByteArray *array, OpenMode openMode = ReadWrite);
    explicit QTextStream(const QByteArray &array, OpenMode openMode = ReadOnly);
    virtual ~QTextStream();

    void setEncoding(QStringConverter::Encoding encoding);
    QStringConverter::Encoding encoding() const;
    void setAutoDetectUnicode(bool enabled);
    bool autoDetectUnicode() const;
    void setGenerateByteOrderMark(bool generate);
    bool generateByteOrderMark() const;

    void setLocale(const QLocale &locale);
    QLocale locale() const;

    void setDevice(QIODevice *device);
    QIODevice *device() const;

    void setString(QString *string, OpenMode openMode = ReadWrite);
    QString *string() const;

    Status status() const;
    void setStatus(Status status);
    void resetStatus();

    bool atEnd() const;
    void reset();
    void flush();
    bool seek(qint64 pos);
    qint64 pos() const;

    void skipWhiteSpace();

    QString readLine(qint64 maxlen = 0);
    bool readLineInto(QString *line, qint64 maxlen = 0);
    QString readAll();
    QString read(qint64 maxlen);

    void setFieldAlignment(FieldAlignment alignment);
    FieldAlignment fieldAlignment() const;

    void setPadChar(QChar ch);
    QChar padChar() const;

    void setFieldWidth(int width);
    int fieldWidth() const;

    void setNumberFlags(NumberFlags flags);
    NumberFlags numberFlags() const;

    void setIntegerBase(int base);
    int integerBase() const;

    void setRealNumberNotation(RealNumberNotation notation);
    RealNumberNotation realNumberNotation() const;

    void setRealNumberPrecision(int precision);
    int realNumberPrecision() const;

    QTextStream &operator>>(QChar &ch);
    QTextStream &operator>>(char &ch);
    QTextStream &operator>>(char16_t &ch)
    { QChar c; *this >> c; ch = c.unicode(); return *this; }
    QTextStream &operator>>(signed short &i);
    QTextStream &operator>>(unsigned short &i);
    QTextStream &operator>>(signed int &i);
    QTextStream &operator>>(unsigned int &i);
    QTextStream &operator>>(signed long &i);
    QTextStream &operator>>(unsigned long &i);
    QTextStream &operator>>(qlonglong &i);
    QTextStream &operator>>(qulonglong &i);
    QTextStream &operator>>(float &f);
    QTextStream &operator>>(double &f);
    QTextStream &operator>>(QString &s);
    QTextStream &operator>>(QByteArray &array);
    QTextStream &operator>>(char *c);

    QTextStream &operator<<(QChar ch);
    QTextStream &operator<<(char ch);
    QTextStream &operator<<(char16_t ch) { return *this << QChar(ch); }
    QTextStream &operator<<(signed short i);
    QTextStream &operator<<(unsigned short i);
    QTextStream &operator<<(signed int i);
    QTextStream &operator<<(unsigned int i);
    QTextStream &operator<<(signed long i);
    QTextStream &operator<<(unsigned long i);
    QTextStream &operator<<(qlonglong i);
    QTextStream &operator<<(qulonglong i);
    QTextStream &operator<<(float f);
    QTextStream &operator<<(double f);
    QTextStream &operator<<(const QString &s);
    QTextStream &operator<<(QStringView s);
    QTextStream &operator<<(QLatin1StringView s);
    QTextStream &operator<<(const QByteArray &array);
    QTextStream &operator<<(const char *c);
    QTextStream &operator<<(const void *ptr);

private:
    Q_DISABLE_COPY(QTextStream)
    friend class QDebugStateSaverPrivate;
    friend class QDebug;

    QScopedPointer<QTextStreamPrivate> d_ptr;
};

Q_DECLARE_OPERATORS_FOR_FLAGS(QTextStream::NumberFlags)

/*****************************************************************************
  QTextStream manipulators
 *****************************************************************************/

typedef QTextStream & (*QTextStreamFunction)(QTextStream &);// manipulator function
typedef void (QTextStream::*QTSMFI)(int); // manipulator w/int argument
typedef void (QTextStream::*QTSMFC)(QChar); // manipulator w/QChar argument


class Q_CORE_EXPORT QTextStreamManipulator
{
public:
    constexpr QTextStreamManipulator(QTSMFI m, int a) noexcept : mf(m), mc(nullptr), arg(a), ch() {}
    constexpr QTextStreamManipulator(QTSMFC m, QChar c) noexcept : mf(nullptr), mc(m), arg(-1), ch(c) {}
    void exec(QTextStream &s) { if (mf) { (s.*mf)(arg); } else { (s.*mc)(ch); } }

private:
    QTSMFI mf;                                        // QTextStream member function
    QTSMFC mc;                                        // QTextStream member function
    int arg;                                          // member function argument
    QChar ch;
};

inline QTextStream &operator>>(QTextStream &s, QTextStreamFunction f)
{ return (*f)(s); }

inline QTextStream &operator<<(QTextStream &s, QTextStreamFunction f)
{ return (*f)(s); }

inline QTextStream &operator<<(QTextStream &s, QTextStreamManipulator m)
{ m.exec(s); return s; }

namespace Qt {
Q_CORE_EXPORT QTextStream &bin(QTextStream &s);
Q_CORE_EXPORT QTextStream &oct(QTextStream &s);
Q_CORE_EXPORT QTextStream &dec(QTextStream &s);
Q_CORE_EXPORT QTextStream &hex(QTextStream &s);

Q_CORE_EXPORT QTextStream &showbase(QTextStream &s);
Q_CORE_EXPORT QTextStream &forcesign(QTextStream &s);
Q_CORE_EXPORT QTextStream &forcepoint(QTextStream &s);
Q_CORE_EXPORT QTextStream &noshowbase(QTextStream &s);
Q_CORE_EXPORT QTextStream &noforcesign(QTextStream &s);
Q_CORE_EXPORT QTextStream &noforcepoint(QTextStream &s);

Q_CORE_EXPORT QTextStream &uppercasebase(QTextStream &s);
Q_CORE_EXPORT QTextStream &uppercasedigits(QTextStream &s);
Q_CORE_EXPORT QTextStream &lowercasebase(QTextStream &s);
Q_CORE_EXPORT QTextStream &lowercasedigits(QTextStream &s);

Q_CORE_EXPORT QTextStream &fixed(QTextStream &s);
Q_CORE_EXPORT QTextStream &scientific(QTextStream &s);

Q_CORE_EXPORT QTextStream &left(QTextStream &s);
Q_CORE_EXPORT QTextStream &right(QTextStream &s);
Q_CORE_EXPORT QTextStream &center(QTextStream &s);

Q_CORE_EXPORT QTextStream &endl(QTextStream &s);
Q_CORE_EXPORT QTextStream &flush(QTextStream &s);
Q_CORE_EXPORT QTextStream &reset(QTextStream &s);

Q_CORE_EXPORT QTextStream &bom(QTextStream &s);

Q_CORE_EXPORT QTextStream &ws(QTextStream &s);

} // namespace Qt

inline QTextStreamManipulator qSetFieldWidth(int width)
{
    QTSMFI func = &QTextStream::setFieldWidth;
    return QTextStreamManipulator(func,width);
}

inline QTextStreamManipulator qSetPadChar(QChar ch)
{
    QTSMFC func = &QTextStream::setPadChar;
    return QTextStreamManipulator(func, ch);
}

inline QTextStreamManipulator qSetRealNumberPrecision(int precision)
{
    QTSMFI func = &QTextStream::setRealNumberPrecision;
    return QTextStreamManipulator(func, precision);
}

QT_END_NAMESPACE

#endif // QTEXTSTREAM_H
