/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Copyright (C) 2016 Intel Corporation.
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

#ifndef QDEBUG_H
#define QDEBUG_H

#include <QtCore/qalgorithms.h>
#include <QtCore/qhash.h>
#include <QtCore/qlist.h>
#include <QtCore/qmap.h>
#include <QtCore/qpair.h>
#include <QtCore/qtextstream.h>
#include <QtCore/qstring.h>
#include <QtCore/qvector.h>
#include <QtCore/qset.h>
#include <QtCore/qcontiguouscache.h>
#include <QtCore/qsharedpointer.h>

// all these have already been included by various headers above, but don't rely on indirect includes:
#include <vector>
#include <list>
#include <map>
#include <utility>

QT_BEGIN_NAMESPACE


class Q_CORE_EXPORT QDebug
{
    friend class QMessageLogger;
    friend class QDebugStateSaver;
    friend class QDebugStateSaverPrivate;
    struct Stream {
        enum { VerbosityShift = 29, VerbosityMask = 0x7 };

        Stream(QIODevice *device) : ts(device), ref(1), type(QtDebugMsg),
            space(true), message_output(false), flags(DefaultVerbosity << VerbosityShift) {}
        Stream(QString *string) : ts(string, QIODevice::WriteOnly), ref(1), type(QtDebugMsg),
            space(true), message_output(false), flags(DefaultVerbosity << VerbosityShift) {}
        Stream(QtMsgType t) : ts(&buffer, QIODevice::WriteOnly), ref(1), type(t),
            space(true), message_output(true), flags(DefaultVerbosity << VerbosityShift) {}
        QTextStream ts;
        QString buffer;
        int ref;
        QtMsgType type;
        bool space;
        bool message_output;
        QMessageLogContext context;

        enum FormatFlag { // Note: Bits 29..31 are reserved for the verbose level introduced in 5.6.
            NoQuotes = 0x1
        };

        // ### Qt 6: unify with space, introduce own version member
        bool testFlag(FormatFlag flag) const { return (context.version > 1) ? (flags & flag) : false; }
        void setFlag(FormatFlag flag) { if (context.version > 1) { flags |= flag; } }
        void unsetFlag(FormatFlag flag) { if (context.version > 1) { flags &= ~flag; } }
        int verbosity() const
        { return context.version > 1 ? (flags >> VerbosityShift) & VerbosityMask : int(DefaultVerbosity); }
        void setVerbosity(int v)
        {
            if (context.version > 1) {
                flags &= ~(uint(VerbosityMask) << VerbosityShift);
                flags |= (v & VerbosityMask) << VerbosityShift;
            }
        }
        // added in 5.4
        int flags;
    } *stream;

    enum Latin1Content { ContainsBinary = 0, ContainsLatin1 };

    void putUcs4(uint ucs4);
    void putString(const QChar *begin, size_t length);
    void putByteArray(const char *begin, size_t length, Latin1Content content);
public:
    inline QDebug(QIODevice *device) : stream(new Stream(device)) {}
    inline QDebug(QString *string) : stream(new Stream(string)) {}
    inline QDebug(QtMsgType t) : stream(new Stream(t)) {}
    inline QDebug(const QDebug &o):stream(o.stream) { ++stream->ref; }
    QDebug(QDebug &&other) noexcept : stream{qExchange(other.stream, nullptr)} {}
    inline QDebug &operator=(const QDebug &other);
    QDebug &operator=(QDebug &&other) noexcept
    { QDebug{std::move(other)}.swap(*this); return *this; }
    ~QDebug();
    inline void swap(QDebug &other) noexcept { qSwap(stream, other.stream); }

    QDebug &resetFormat();

    inline QDebug &space() { stream->space = true; stream->ts << ' '; return *this; }
    inline QDebug &nospace() { stream->space = false; return *this; }
    inline QDebug &maybeSpace() { if (stream->space) stream->ts << ' '; return *this; }
    inline QDebug &verbosity(int verbosityLevel) { setVerbosity(verbosityLevel); return *this; }
    int verbosity() const { return stream->verbosity(); }
    void setVerbosity(int verbosityLevel) { stream->setVerbosity(verbosityLevel); }
    enum VerbosityLevel { MinimumVerbosity = 0, DefaultVerbosity = 2, MaximumVerbosity = 7 };

    bool autoInsertSpaces() const { return stream->space; }
    void setAutoInsertSpaces(bool b) { stream->space = b; }

    inline QDebug &quote() { stream->unsetFlag(Stream::NoQuotes); return *this; }
    inline QDebug &noquote() { stream->setFlag(Stream::NoQuotes); return *this; }
    inline QDebug &maybeQuote(char c = '"') { if (!(stream->testFlag(Stream::NoQuotes))) stream->ts << c; return *this; }

    inline QDebug &operator<<(QChar t) { putUcs4(t.unicode()); return maybeSpace(); }
    inline QDebug &operator<<(bool t) { stream->ts << (t ? "true" : "false"); return maybeSpace(); }
    inline QDebug &operator<<(char t) { stream->ts << t; return maybeSpace(); }
    inline QDebug &operator<<(signed short t) { stream->ts << t; return maybeSpace(); }
    inline QDebug &operator<<(unsigned short t) { stream->ts << t; return maybeSpace(); }
#ifdef Q_COMPILER_UNICODE_STRINGS
    inline QDebug &operator<<(char16_t t) { return *this << QChar(ushort(t)); }
    inline QDebug &operator<<(char32_t t) { putUcs4(t); return maybeSpace(); }
#endif
    inline QDebug &operator<<(signed int t) { stream->ts << t; return maybeSpace(); }
    inline QDebug &operator<<(unsigned int t) { stream->ts << t; return maybeSpace(); }
    inline QDebug &operator<<(signed long t) { stream->ts << t; return maybeSpace(); }
    inline QDebug &operator<<(unsigned long t) { stream->ts << t; return maybeSpace(); }
    inline QDebug &operator<<(qint64 t) { stream->ts << t; return maybeSpace(); }
    inline QDebug &operator<<(quint64 t) { stream->ts << t; return maybeSpace(); }
    inline QDebug &operator<<(float t) { stream->ts << t; return maybeSpace(); }
    inline QDebug &operator<<(double t) { stream->ts << t; return maybeSpace(); }
    inline QDebug &operator<<(const char* t) { stream->ts << QString::fromUtf8(t); return maybeSpace(); }
#if QT_STRINGVIEW_LEVEL < 2
    inline QDebug &operator<<(const QString & t) { putString(t.constData(), uint(t.length())); return maybeSpace(); }
    inline QDebug &operator<<(const QStringRef & t) { putString(t.constData(), uint(t.length())); return maybeSpace(); }
#endif
    inline QDebug &operator<<(QStringView s) { putString(s.data(), size_t(s.size())); return maybeSpace(); }
    inline QDebug &operator<<(QLatin1String t) { putByteArray(t.latin1(), t.size(), ContainsLatin1); return maybeSpace(); }
    inline QDebug &operator<<(const QByteArray & t) { putByteArray(t.constData(), t.size(), ContainsBinary); return maybeSpace(); }
    inline QDebug &operator<<(const void * t) { stream->ts << t; return maybeSpace(); }
    inline QDebug &operator<<(std::nullptr_t) { stream->ts << "(nullptr)"; return maybeSpace(); }
    inline QDebug &operator<<(QTextStreamFunction f) {
        stream->ts << f;
        return *this;
    }

    inline QDebug &operator<<(QTextStreamManipulator m)
    { stream->ts << m; return *this; }
};

Q_DECLARE_SHARED(QDebug)

class QDebugStateSaverPrivate;
class Q_CORE_EXPORT QDebugStateSaver
{
public:
    QDebugStateSaver(QDebug &dbg);
    ~QDebugStateSaver();
private:
    Q_DISABLE_COPY(QDebugStateSaver)
    QScopedPointer<QDebugStateSaverPrivate> d;
};

class QNoDebug
{
public:
    inline QNoDebug &operator<<(QTextStreamFunction) { return *this; }
    inline QNoDebug &operator<<(QTextStreamManipulator) { return *this; }
    inline QNoDebug &space() { return *this; }
    inline QNoDebug &nospace() { return *this; }
    inline QNoDebug &maybeSpace() { return *this; }
    inline QNoDebug &quote() { return *this; }
    inline QNoDebug &noquote() { return *this; }
    inline QNoDebug &maybeQuote(const char = '"') { return *this; }
    inline QNoDebug &verbosity(int) { return *this; }

    template<typename T>
    inline QNoDebug &operator<<(const T &) { return *this; }
};

inline QDebug &QDebug::operator=(const QDebug &other)
{
    QDebug{other}.swap(*this);
    return *this;
}

namespace QtPrivate {

template <typename SequentialContainer>
inline QDebug printSequentialContainer(QDebug debug, const char *which, const SequentialContainer &c)
{
    const bool oldSetting = debug.autoInsertSpaces();
    debug.nospace() << which << '(';
    typename SequentialContainer::const_iterator it = c.begin(), end = c.end();
    if (it != end) {
        debug << *it;
        ++it;
    }
    while (it != end) {
        debug << ", " << *it;
        ++it;
    }
    debug << ')';
    debug.setAutoInsertSpaces(oldSetting);
    return debug.maybeSpace();
}

} // namespace QtPrivate

template <class T>
inline QDebug operator<<(QDebug debug, const QList<T> &list)
{
    return QtPrivate::printSequentialContainer(debug, "" /*for historical reasons*/, list);
}

template <typename T>
inline QDebug operator<<(QDebug debug, const QVector<T> &vec)
{
    return QtPrivate::printSequentialContainer(debug, "QVector", vec);
}

template <typename T, typename Alloc>
inline QDebug operator<<(QDebug debug, const std::vector<T, Alloc> &vec)
{
    return QtPrivate::printSequentialContainer(debug, "std::vector", vec);
}

template <typename T, typename Alloc>
inline QDebug operator<<(QDebug debug, const std::list<T, Alloc> &vec)
{
    return QtPrivate::printSequentialContainer(debug, "std::list", vec);
}

template <typename Key, typename T, typename Compare, typename Alloc>
inline QDebug operator<<(QDebug debug, const std::map<Key, T, Compare, Alloc> &map)
{
    return QtPrivate::printSequentialContainer(debug, "std::map", map); // yes, sequential: *it is std::pair
}

template <typename Key, typename T, typename Compare, typename Alloc>
inline QDebug operator<<(QDebug debug, const std::multimap<Key, T, Compare, Alloc> &map)
{
    return QtPrivate::printSequentialContainer(debug, "std::multimap", map); // yes, sequential: *it is std::pair
}

template <class Key, class T>
inline QDebug operator<<(QDebug debug, const QMap<Key, T> &map)
{
    const bool oldSetting = debug.autoInsertSpaces();
    debug.nospace() << "QMap(";
    for (typename QMap<Key, T>::const_iterator it = map.constBegin();
         it != map.constEnd(); ++it) {
        debug << '(' << it.key() << ", " << it.value() << ')';
    }
    debug << ')';
    debug.setAutoInsertSpaces(oldSetting);
    return debug.maybeSpace();
}

template <class Key, class T>
inline QDebug operator<<(QDebug debug, const QHash<Key, T> &hash)
{
    const bool oldSetting = debug.autoInsertSpaces();
    debug.nospace() << "QHash(";
    for (typename QHash<Key, T>::const_iterator it = hash.constBegin();
            it != hash.constEnd(); ++it)
        debug << '(' << it.key() << ", " << it.value() << ')';
    debug << ')';
    debug.setAutoInsertSpaces(oldSetting);
    return debug.maybeSpace();
}

template <class T1, class T2>
inline QDebug operator<<(QDebug debug, const QPair<T1, T2> &pair)
{
    const bool oldSetting = debug.autoInsertSpaces();
    debug.nospace() << "QPair(" << pair.first << ',' << pair.second << ')';
    debug.setAutoInsertSpaces(oldSetting);
    return debug.maybeSpace();
}

template <class T1, class T2>
inline QDebug operator<<(QDebug debug, const std::pair<T1, T2> &pair)
{
    const bool oldSetting = debug.autoInsertSpaces();
    debug.nospace() << "std::pair(" << pair.first << ',' << pair.second << ')';
    debug.setAutoInsertSpaces(oldSetting);
    return debug.maybeSpace();
}

template <typename T>
inline QDebug operator<<(QDebug debug, const QSet<T> &set)
{
    return QtPrivate::printSequentialContainer(debug, "QSet", set);
}

template <class T>
inline QDebug operator<<(QDebug debug, const QContiguousCache<T> &cache)
{
    const bool oldSetting = debug.autoInsertSpaces();
    debug.nospace() << "QContiguousCache(";
    for (int i = cache.firstIndex(); i <= cache.lastIndex(); ++i) {
        debug << cache[i];
        if (i != cache.lastIndex())
            debug << ", ";
    }
    debug << ')';
    debug.setAutoInsertSpaces(oldSetting);
    return debug.maybeSpace();
}

template <class T>
inline QDebug operator<<(QDebug debug, const QSharedPointer<T> &ptr)
{
    QDebugStateSaver saver(debug);
    debug.nospace() << "QSharedPointer(" << ptr.data() << ")";
    return debug;
}

Q_CORE_EXPORT void qt_QMetaEnum_flagDebugOperator(QDebug &debug, size_t sizeofT, int value);

template <typename Int>
void qt_QMetaEnum_flagDebugOperator(QDebug &debug, size_t sizeofT, Int value)
{
    const QDebugStateSaver saver(debug);
    debug.resetFormat();
    debug.nospace() << "QFlags(" << Qt::hex << Qt::showbase;
    bool needSeparator = false;
    for (uint i = 0; i < sizeofT * 8; ++i) {
        if (value & (Int(1) << i)) {
            if (needSeparator)
                debug << '|';
            else
                needSeparator = true;
            debug << (Int(1) << i);
        }
    }
    debug << ')';
}

#if !defined(QT_NO_QOBJECT) && !defined(Q_QDOC)
Q_CORE_EXPORT QDebug qt_QMetaEnum_debugOperator(QDebug&, int value, const QMetaObject *meta, const char *name);
Q_CORE_EXPORT QDebug qt_QMetaEnum_flagDebugOperator(QDebug &dbg, quint64 value, const QMetaObject *meta, const char *name);

template<typename T>
typename std::enable_if<QtPrivate::IsQEnumHelper<T>::Value, QDebug>::type
operator<<(QDebug dbg, T value)
{
    const QMetaObject *obj = qt_getEnumMetaObject(value);
    const char *name = qt_getEnumName(value);
    return qt_QMetaEnum_debugOperator(dbg, typename QFlags<T>::Int(value), obj, name);
}

template<typename T,
         typename A = typename std::enable_if<std::is_enum<T>::value, void>::type,
         typename B = typename std::enable_if<sizeof(T) <= sizeof(int), void>::type,
         typename C = typename std::enable_if<!QtPrivate::IsQEnumHelper<T>::Value, void>::type,
         typename D = typename std::enable_if<QtPrivate::IsQEnumHelper<QFlags<T>>::Value, void>::type>
inline QDebug operator<<(QDebug dbg, T value)
{
    typedef QFlags<T> FlagsT;
    const QMetaObject *obj = qt_getEnumMetaObject(FlagsT());
    const char *name = qt_getEnumName(FlagsT());
    return qt_QMetaEnum_debugOperator(dbg, typename FlagsT::Int(value), obj, name);
}

template <class T>
inline typename std::enable_if<
    QtPrivate::IsQEnumHelper<T>::Value || QtPrivate::IsQEnumHelper<QFlags<T> >::Value,
    QDebug>::type
qt_QMetaEnum_flagDebugOperator_helper(QDebug debug, const QFlags<T> &flags)
{
    const QMetaObject *obj = qt_getEnumMetaObject(T());
    const char *name = qt_getEnumName(T());
    return qt_QMetaEnum_flagDebugOperator(debug, quint64(flags), obj, name);
}

template <class T>
inline typename std::enable_if<
    !QtPrivate::IsQEnumHelper<T>::Value && !QtPrivate::IsQEnumHelper<QFlags<T> >::Value,
    QDebug>::type
qt_QMetaEnum_flagDebugOperator_helper(QDebug debug, const QFlags<T> &flags)
#else // !QT_NO_QOBJECT && !Q_QDOC
template <class T>
inline QDebug qt_QMetaEnum_flagDebugOperator_helper(QDebug debug, const QFlags<T> &flags)
#endif
{
    qt_QMetaEnum_flagDebugOperator(debug, sizeof(T), typename QFlags<T>::Int(flags));
    return debug;
}

template<typename T>
inline QDebug operator<<(QDebug debug, const QFlags<T> &flags)
{
    // We have to use an indirection otherwise specialisation of some other overload of the
    // operator<< the compiler would try to instantiate QFlags<T> for the std::enable_if
    return qt_QMetaEnum_flagDebugOperator_helper(debug, flags);
}

#ifdef Q_OS_MAC

// We provide QDebug stream operators for commonly used Core Foundation
// and Core Graphics types, as well as NSObject. Additional CF/CG types
// may be added by the user, using Q_DECLARE_QDEBUG_OPERATOR_FOR_CF_TYPE.

#define QT_FOR_EACH_CORE_FOUNDATION_TYPE(F) \
    F(CFArray) \
    F(CFURL) \
    F(CFData) \
    F(CFNumber) \
    F(CFDictionary) \
    F(CFLocale) \
    F(CFDate) \
    F(CFBoolean) \
    F(CFTimeZone) \

#define QT_FOR_EACH_MUTABLE_CORE_FOUNDATION_TYPE(F) \
    F(CFError) \
    F(CFBundle) \

#define QT_FOR_EACH_CORE_GRAPHICS_TYPE(F) \
    F(CGPath) \

#define QT_FOR_EACH_MUTABLE_CORE_GRAPHICS_TYPE(F) \
    F(CGColorSpace) \
    F(CGImage) \
    F(CGFont) \
    F(CGColor) \

#define QT_FORWARD_DECLARE_CF_TYPE(type) Q_FORWARD_DECLARE_CF_TYPE(type);
#define QT_FORWARD_DECLARE_MUTABLE_CF_TYPE(type) Q_FORWARD_DECLARE_MUTABLE_CF_TYPE(type);
#define QT_FORWARD_DECLARE_CG_TYPE(type) Q_FORWARD_DECLARE_CG_TYPE(type);
#define QT_FORWARD_DECLARE_MUTABLE_CG_TYPE(type) Q_FORWARD_DECLARE_MUTABLE_CG_TYPE(type);

QT_END_NAMESPACE
Q_FORWARD_DECLARE_CF_TYPE(CFString);
Q_FORWARD_DECLARE_OBJC_CLASS(NSObject);
QT_FOR_EACH_CORE_FOUNDATION_TYPE(QT_FORWARD_DECLARE_CF_TYPE)
QT_FOR_EACH_MUTABLE_CORE_FOUNDATION_TYPE(QT_FORWARD_DECLARE_MUTABLE_CF_TYPE)
QT_FOR_EACH_CORE_GRAPHICS_TYPE(QT_FORWARD_DECLARE_CG_TYPE)
QT_FOR_EACH_MUTABLE_CORE_GRAPHICS_TYPE(QT_FORWARD_DECLARE_MUTABLE_CG_TYPE)
QT_BEGIN_NAMESPACE

#define QT_FORWARD_DECLARE_QDEBUG_OPERATOR_FOR_CF_TYPE(CFType) \
    Q_CORE_EXPORT QDebug operator<<(QDebug, CFType##Ref);

#define Q_DECLARE_QDEBUG_OPERATOR_FOR_CF_TYPE(CFType) \
    QDebug operator<<(QDebug debug, CFType##Ref ref) \
    { \
        if (!ref) \
            return debug << QT_STRINGIFY(CFType) "Ref(0x0)"; \
        if (CFStringRef description = CFCopyDescription(ref)) { \
            QDebugStateSaver saver(debug); \
            debug.noquote() << description; \
            CFRelease(description); \
        } \
        return debug; \
    }

// Defined in qcore_mac_objc.mm
Q_CORE_EXPORT QDebug operator<<(QDebug, const NSObject *);
Q_CORE_EXPORT QDebug operator<<(QDebug, CFStringRef);

QT_FOR_EACH_CORE_FOUNDATION_TYPE(QT_FORWARD_DECLARE_QDEBUG_OPERATOR_FOR_CF_TYPE)
QT_FOR_EACH_MUTABLE_CORE_FOUNDATION_TYPE(QT_FORWARD_DECLARE_QDEBUG_OPERATOR_FOR_CF_TYPE)
QT_FOR_EACH_CORE_GRAPHICS_TYPE(QT_FORWARD_DECLARE_QDEBUG_OPERATOR_FOR_CF_TYPE)
QT_FOR_EACH_MUTABLE_CORE_GRAPHICS_TYPE(QT_FORWARD_DECLARE_QDEBUG_OPERATOR_FOR_CF_TYPE)

#undef QT_FORWARD_DECLARE_CF_TYPE
#undef QT_FORWARD_DECLARE_MUTABLE_CF_TYPE
#undef QT_FORWARD_DECLARE_CG_TYPE
#undef QT_FORWARD_DECLARE_MUTABLE_CG_TYPE

#endif // Q_OS_MAC

QT_END_NAMESPACE

#endif // QDEBUG_H
