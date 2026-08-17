// Copyright (C) 2020 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QSTRINGBUILDER_H
#define QSTRINGBUILDER_H

#if 0
// syncqt can not handle the templates in this file, and it doesn't need to
// process them anyway because they are internal.
#pragma qt_class(QStringBuilder)
#pragma qt_sync_stop_processing
#endif

#include <QtCore/qstring.h>
#include <QtCore/qbytearray.h>

#include <string.h>

QT_BEGIN_NAMESPACE


struct Q_CORE_EXPORT QAbstractConcatenable
{
protected:
    static void convertFromUtf8(QByteArrayView in, QChar *&out) noexcept;
    static inline void convertFromAscii(char a, QChar *&out) noexcept
    {
        *out++ = QLatin1Char(a);
    }
    static void appendLatin1To(QLatin1StringView in, QChar *out) noexcept;
};

template <typename T> struct QConcatenable {};

namespace QtStringBuilder {
    template <typename A, typename B> struct ConvertToTypeHelper
    { typedef A ConvertTo; };
    template <typename T> struct ConvertToTypeHelper<T, QString>
    { typedef QString ConvertTo; };
}

template<typename Builder, typename T>
struct QStringBuilderCommon
{
    T toUpper() const { return resolved().toUpper(); }
    T toLower() const { return resolved().toLower(); }

protected:
    T resolved() const { return *static_cast<const Builder*>(this); }
};

template<typename Builder, typename T>
struct QStringBuilderBase : public QStringBuilderCommon<Builder, T>
{
};

template<typename Builder>
struct QStringBuilderBase<Builder, QString> : public QStringBuilderCommon<Builder, QString>
{
    QByteArray toLatin1() const { return this->resolved().toLatin1(); }
    QByteArray toUtf8() const { return this->resolved().toUtf8(); }
    QByteArray toLocal8Bit() const { return this->resolved().toLocal8Bit(); }
};

template <typename A, typename B>
class QStringBuilder : public QStringBuilderBase<QStringBuilder<A, B>, typename QtStringBuilder::ConvertToTypeHelper<typename QConcatenable<A>::ConvertTo, typename QConcatenable<B>::ConvertTo>::ConvertTo>
{
public:
    QStringBuilder(const A &a_, const B &b_) : a(a_), b(b_) {}
private:
    friend class QByteArray;
    friend class QString;
    template <typename T> T convertTo() const
    {
        const qsizetype len = QConcatenable< QStringBuilder<A, B> >::size(*this);
        T s(len, Qt::Uninitialized);

        // we abuse const_cast / constData here because we know we've just
        // allocated the data and we're the only reference count
        typename T::iterator d = const_cast<typename T::iterator>(s.constData());
        typename T::const_iterator const start = d;
        QConcatenable< QStringBuilder<A, B> >::appendTo(*this, d);

        if (!QConcatenable< QStringBuilder<A, B> >::ExactSize && len != d - start) {
            // this resize is necessary since we allocate a bit too much
            // when dealing with variable sized 8-bit encodings
            s.resize(d - start);
        }
        return s;
    }

    typedef QConcatenable<QStringBuilder<A, B> > Concatenable;
public:
    typedef typename Concatenable::ConvertTo ConvertTo;
    operator ConvertTo() const { return convertTo<ConvertTo>(); }

    qsizetype size() const { return Concatenable::size(*this); }

    const A &a;
    const B &b;
};

template <>
class QStringBuilder <QString, QString> : public QStringBuilderBase<QStringBuilder<QString, QString>, QString>
{
    public:
        QStringBuilder(const QString &a_, const QString &b_) : a(a_), b(b_) {}
        QStringBuilder(const QStringBuilder &other) : a(other.a), b(other.b) {}

        operator QString() const
        { QString r(a); r += b; return r; }

        const QString &a;
        const QString &b;

    private:
        QStringBuilder &operator=(const QStringBuilder &) = delete;
};

template <>
class QStringBuilder <QByteArray, QByteArray> : public QStringBuilderBase<QStringBuilder<QByteArray, QByteArray>, QByteArray>
{
    public:
        QStringBuilder(const QByteArray &a_, const QByteArray &b_) : a(a_), b(b_) {}
        QStringBuilder(const QStringBuilder &other) : a(other.a), b(other.b) {}

        operator QByteArray() const
        { QByteArray r(a); r += b; return r; }

        const QByteArray &a;
        const QByteArray &b;

    private:
        QStringBuilder &operator=(const QStringBuilder &) = delete;
};


template <> struct QConcatenable<char> : private QAbstractConcatenable
{
    typedef char type;
    typedef QByteArray ConvertTo;
    enum { ExactSize = true };
    static qsizetype size(const char) { return 1; }
#ifndef QT_NO_CAST_FROM_ASCII
    QT_ASCII_CAST_WARN static inline void appendTo(const char c, QChar *&out)
    {
        QAbstractConcatenable::convertFromAscii(c, out);
    }
#endif
    static inline void appendTo(const char c, char *&out)
    { *out++ = c; }
};

template <> struct QConcatenable<QByteArrayView> : private QAbstractConcatenable
{
    typedef QByteArrayView type;
    typedef QByteArray ConvertTo;
    enum { ExactSize = true };
    static qsizetype size(QByteArrayView bav) { return bav.size(); }
#ifndef QT_NO_CAST_FROM_ASCII
    QT_ASCII_CAST_WARN static inline void appendTo(QByteArrayView bav, QChar *&out)
    {
        QAbstractConcatenable::convertFromUtf8(bav, out);
    }
#endif
    static inline void appendTo(QByteArrayView bav, char *&out)
    {
        qsizetype n = bav.size();
        if (n)
            memcpy(out, bav.data(), n);
        out += n;
    }
};

template <> struct QConcatenable<char16_t> : private QAbstractConcatenable
{
    typedef char16_t type;
    typedef QString ConvertTo;
    enum { ExactSize = true };
    static constexpr qsizetype size(char16_t) { return 1; }
    static inline void appendTo(char16_t c, QChar *&out)
    { *out++ = c; }
};

template <> struct QConcatenable<QLatin1Char>
{
    typedef QLatin1Char type;
    typedef QString ConvertTo;
    enum { ExactSize = true };
    static qsizetype size(const QLatin1Char) { return 1; }
    static inline void appendTo(const QLatin1Char c, QChar *&out)
    { *out++ = c; }
    static inline void appendTo(const QLatin1Char c, char *&out)
    { *out++ = c.toLatin1(); }
};

template <> struct QConcatenable<QChar> : private QAbstractConcatenable
{
    typedef QChar type;
    typedef QString ConvertTo;
    enum { ExactSize = true };
    static qsizetype size(const QChar) { return 1; }
    static inline void appendTo(const QChar c, QChar *&out)
    { *out++ = c; }
};

template <> struct QConcatenable<QChar::SpecialCharacter> : private QAbstractConcatenable
{
    typedef QChar::SpecialCharacter type;
    typedef QString ConvertTo;
    enum { ExactSize = true };
    static qsizetype size(const QChar::SpecialCharacter) { return 1; }
    static inline void appendTo(const QChar::SpecialCharacter c, QChar *&out)
    { *out++ = c; }
};

template <> struct QConcatenable<QLatin1StringView> : private QAbstractConcatenable
{
    typedef QLatin1StringView type;
    typedef QString ConvertTo;
    enum { ExactSize = true };
    static qsizetype size(const QLatin1StringView a) { return a.size(); }
    static inline void appendTo(const QLatin1StringView a, QChar *&out)
    {
        appendLatin1To(a, out);
        out += a.size();
    }
    static inline void appendTo(const QLatin1StringView a, char *&out)
    {
        if (const char *data = a.data()) {
            memcpy(out, data, a.size());
            out += a.size();
        }
    }
};

template <> struct QConcatenable<QString> : private QAbstractConcatenable
{
    typedef QString type;
    typedef QString ConvertTo;
    enum { ExactSize = true };
    static qsizetype size(const QString &a) { return a.size(); }
    static inline void appendTo(const QString &a, QChar *&out)
    {
        const qsizetype n = a.size();
        if (n)
            memcpy(out, reinterpret_cast<const char*>(a.constData()), sizeof(QChar) * n);
        out += n;
    }
};

template <> struct QConcatenable<QStringView> : private QAbstractConcatenable
{
    typedef QStringView type;
    typedef QString ConvertTo;
    enum { ExactSize = true };
    static qsizetype size(QStringView a) { return a.size(); }
    static inline void appendTo(QStringView a, QChar *&out)
    {
        const auto n = a.size();
        if (n)
            memcpy(out, a.data(), sizeof(QChar) * n);
        out += n;
    }
};

template <qsizetype N> struct QConcatenable<const char[N]> : private QAbstractConcatenable
{
    typedef const char type[N];
    typedef QByteArray ConvertTo;
    enum { ExactSize = false };
    static qsizetype size(const char[N]) { return N - 1; }
#ifndef QT_NO_CAST_FROM_ASCII
    QT_ASCII_CAST_WARN static inline void appendTo(const char a[N], QChar *&out)
    {
        QAbstractConcatenable::convertFromUtf8(QByteArrayView(a, N - 1), out);
    }
#endif
    static inline void appendTo(const char a[N], char *&out)
    {
        while (*a)
            *out++ = *a++;
    }
};

template <qsizetype N> struct QConcatenable<char[N]> : QConcatenable<const char[N]>
{
    typedef char type[N];
};

template <> struct QConcatenable<const char *> : private QAbstractConcatenable
{
    typedef const char *type;
    typedef QByteArray ConvertTo;
    enum { ExactSize = false };
    static qsizetype size(const char *a) { return qstrlen(a); }
#ifndef QT_NO_CAST_FROM_ASCII
    QT_ASCII_CAST_WARN static inline void appendTo(const char *a, QChar *&out)
    { QAbstractConcatenable::convertFromUtf8(QByteArrayView(a), out); }
#endif
    static inline void appendTo(const char *a, char *&out)
    {
        if (!a)
            return;
        while (*a)
            *out++ = *a++;
    }
};

template <> struct QConcatenable<char *> : QConcatenable<const char*>
{
    typedef char *type;
};

template <qsizetype N> struct QConcatenable<const char16_t[N]> : private QAbstractConcatenable
{
    using type = const char16_t[N];
    using ConvertTo = QString;
    enum { ExactSize = true };
    static qsizetype size(const char16_t[N]) { return N - 1; }
    static void appendTo(const char16_t a[N], QChar *&out)
    {
        memcpy(static_cast<void *>(out), a, (N - 1) * sizeof(char16_t));
        out += N - 1;
    }
};

template <qsizetype N> struct QConcatenable<char16_t[N]> : QConcatenable<const char16_t[N]>
{
    using type = char16_t[N];
};

template <> struct QConcatenable<const char16_t *> : private QAbstractConcatenable
{
    using type = const char16_t *;
    using ConvertTo = QString;
    enum { ExactSize = true };
    static qsizetype size(const char16_t *a) { return QStringView(a).size(); }
    QT_ASCII_CAST_WARN static inline void appendTo(const char16_t *a, QChar *&out)
    {
        if (!a)
            return;
        while (*a)
            *out++ = *a++;
    }
};

template <> struct QConcatenable<char16_t *> : QConcatenable<const char16_t*>
{
    typedef char16_t *type;
};

template <> struct QConcatenable<QByteArray> : private QAbstractConcatenable
{
    typedef QByteArray type;
    typedef QByteArray ConvertTo;
    enum { ExactSize = false };
    static qsizetype size(const QByteArray &ba) { return ba.size(); }
#ifndef QT_NO_CAST_FROM_ASCII
    QT_ASCII_CAST_WARN static inline void appendTo(const QByteArray &ba, QChar *&out)
    {
        QAbstractConcatenable::convertFromUtf8(ba, out);
    }
#endif
    static inline void appendTo(const QByteArray &ba, char *&out)
    {
        const char *a = ba.constData();
        const char * const end = ba.end();
        while (a != end)
            *out++ = *a++;
    }
};


template <typename A, typename B>
struct QConcatenable< QStringBuilder<A, B> >
{
    typedef QStringBuilder<A, B> type;
    typedef typename QtStringBuilder::ConvertToTypeHelper<typename QConcatenable<A>::ConvertTo, typename QConcatenable<B>::ConvertTo>::ConvertTo ConvertTo;
    enum { ExactSize = QConcatenable<A>::ExactSize && QConcatenable<B>::ExactSize };
    static qsizetype size(const type &p)
    {
        return QConcatenable<A>::size(p.a) + QConcatenable<B>::size(p.b);
    }
    template<typename T> static inline void appendTo(const type &p, T *&out)
    {
        QConcatenable<A>::appendTo(p.a, out);
        QConcatenable<B>::appendTo(p.b, out);
    }
};

template <typename A, typename B>
QStringBuilder<typename QConcatenable<A>::type, typename QConcatenable<B>::type>
operator%(const A &a, const B &b)
{
   return QStringBuilder<typename QConcatenable<A>::type, typename QConcatenable<B>::type>(a, b);
}

// QT_USE_FAST_OPERATOR_PLUS was introduced in 4.7, QT_USE_QSTRINGBUILDER is to be used from 4.8 onwards
// QT_USE_FAST_OPERATOR_PLUS does not remove the normal operator+ for QByteArray
#if defined(QT_USE_FAST_OPERATOR_PLUS) || defined(QT_USE_QSTRINGBUILDER)
template <typename A, typename B>
QStringBuilder<typename QConcatenable<A>::type, typename QConcatenable<B>::type>
operator+(const A &a, const B &b)
{
   return QStringBuilder<typename QConcatenable<A>::type, typename QConcatenable<B>::type>(a, b);
}
#endif

namespace QtStringBuilder {
template <typename A, typename B>
QByteArray &appendToByteArray(QByteArray &a, const QStringBuilder<A, B> &b, char)
{
    // append 8-bit data to a byte array
    qsizetype len = a.size() + QConcatenable< QStringBuilder<A, B> >::size(b);
    a.detach(); // a detach() in a.data() could reset a.capacity() to a.size()
    if (len > a.data_ptr().freeSpaceAtEnd()) // capacity() was broken when prepend()-optimization landed
        a.reserve(qMax(len, 2 * a.capacity()));
    char *it = a.data() + a.size();
    QConcatenable< QStringBuilder<A, B> >::appendTo(b, it);
    a.resize(len); //we need to resize after the appendTo for the case str+=foo+str
    return a;
}

#ifndef QT_NO_CAST_TO_ASCII
template <typename A, typename B>
QByteArray &appendToByteArray(QByteArray &a, const QStringBuilder<A, B> &b, QChar)
{
    return a += QString(b).toUtf8();
}
#endif
}

template <typename A, typename B>
QByteArray &operator+=(QByteArray &a, const QStringBuilder<A, B> &b)
{
    return QtStringBuilder::appendToByteArray(a, b,
                                              typename QConcatenable< QStringBuilder<A, B> >::ConvertTo::value_type());
}

template <typename A, typename B>
QString &operator+=(QString &a, const QStringBuilder<A, B> &b)
{
    qsizetype len = a.size() + QConcatenable< QStringBuilder<A, B> >::size(b);
    a.detach(); // a detach() in a.data() could reset a.capacity() to a.size()
    if (len > a.data_ptr().freeSpaceAtEnd()) // capacity() was broken when prepend()-optimization landed
        a.reserve(qMax(len, 2 * a.capacity()));
    QChar *it = a.data() + a.size();
    QConcatenable< QStringBuilder<A, B> >::appendTo(b, it);
    // we need to resize after the appendTo for the case str+=foo+str
    a.resize(it - a.constData()); //may be smaller than len if there was conversion from utf8
    return a;
}

//
// inline QAnyStringView members requiring QStringBuilder:
//

template <typename A, typename B>
QAnyStringView::QAnyStringView(const QStringBuilder<A, B> &expr,
                               typename QConcatenable<QStringBuilder<A, B>>::ConvertTo &&capacity)
    : QAnyStringView(capacity = expr) {}

QT_END_NAMESPACE

#endif // QSTRINGBUILDER_H
