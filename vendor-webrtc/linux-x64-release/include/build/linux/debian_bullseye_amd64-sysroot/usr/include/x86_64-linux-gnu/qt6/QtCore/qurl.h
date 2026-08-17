// Copyright (C) 2020 The Qt Company Ltd.
// Copyright (C) 2016 Intel Corporation.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QURL_H
#define QURL_H

#include <QtCore/qbytearray.h>
#include <QtCore/qobjectdefs.h>
#include <QtCore/qstring.h>
#include <QtCore/qlist.h>
#include <QtCore/qpair.h>
#include <QtCore/qglobal.h>

#if defined(Q_OS_DARWIN) || defined(Q_QDOC)
Q_FORWARD_DECLARE_CF_TYPE(CFURL);
Q_FORWARD_DECLARE_OBJC_CLASS(NSURL);
#endif

QT_BEGIN_NAMESPACE


class QUrlQuery;
class QUrlPrivate;
class QDataStream;

template <typename E1, typename E2>
class QUrlTwoFlags
{
    int i;
public:
    constexpr inline QUrlTwoFlags() : i(0) {}
    constexpr inline QUrlTwoFlags(E1 f) : i(f) {}
    constexpr inline QUrlTwoFlags(E2 f) : i(f) {}
    constexpr inline QUrlTwoFlags(QFlag f) : i(f) {}
    constexpr inline QUrlTwoFlags(QFlags<E1> f) : i(f.operator typename QFlags<E1>::Int()) {}
    constexpr inline QUrlTwoFlags(QFlags<E2> f) : i(f.operator typename QFlags<E2>::Int()) {}

    inline QUrlTwoFlags &operator&=(int mask) { i &= mask; return *this; }
    inline QUrlTwoFlags &operator&=(uint mask) { i &= mask; return *this; }
    inline QUrlTwoFlags &operator&=(QFlags<E1> mask) { i &= mask.toInt(); return *this; }
    inline QUrlTwoFlags &operator&=(QFlags<E2> mask) { i &= mask.toInt(); return *this; }
    inline QUrlTwoFlags &operator|=(QUrlTwoFlags f) { i |= f.i; return *this; }
    inline QUrlTwoFlags &operator|=(E1 f) { i |= f; return *this; }
    inline QUrlTwoFlags &operator|=(E2 f) { i |= f; return *this; }
    inline QUrlTwoFlags &operator|=(QFlags<E1> mask) { i |= mask.toInt(); return *this; }
    inline QUrlTwoFlags &operator|=(QFlags<E2> mask) { i |= mask.toInt(); return *this; }
    inline QUrlTwoFlags &operator^=(QUrlTwoFlags f) { i ^= f.i; return *this; }
    inline QUrlTwoFlags &operator^=(E1 f) { i ^= f; return *this; }
    inline QUrlTwoFlags &operator^=(E2 f) { i ^= f; return *this; }
    inline QUrlTwoFlags &operator^=(QFlags<E1> mask) { i ^= mask.toInt(); return *this; }
    inline QUrlTwoFlags &operator^=(QFlags<E2> mask) { i ^= mask.toInt(); return *this; }

    constexpr inline operator QFlags<E1>() const { return QFlag(i); }
    constexpr inline operator QFlags<E2>() const { return QFlag(i); }
    constexpr inline operator int() const { return i; }
    constexpr inline bool operator!() const { return !i; }

    constexpr inline QUrlTwoFlags operator|(QUrlTwoFlags f) const
    { return QUrlTwoFlags(QFlag(i | f.i)); }
    constexpr inline QUrlTwoFlags operator|(E1 f) const
    { return QUrlTwoFlags(QFlag(i | f)); }
    constexpr inline QUrlTwoFlags operator|(E2 f) const
    { return QUrlTwoFlags(QFlag(i | f)); }
    constexpr inline QUrlTwoFlags operator^(QUrlTwoFlags f) const
    { return QUrlTwoFlags(QFlag(i ^ f.i)); }
    constexpr inline QUrlTwoFlags operator^(E1 f) const
    { return QUrlTwoFlags(QFlag(i ^ f)); }
    constexpr inline QUrlTwoFlags operator^(E2 f) const
    { return QUrlTwoFlags(QFlag(i ^ f)); }
    constexpr inline QUrlTwoFlags operator&(int mask) const
    { return QUrlTwoFlags(QFlag(i & mask)); }
    constexpr inline QUrlTwoFlags operator&(uint mask) const
    { return QUrlTwoFlags(QFlag(i & mask)); }
    constexpr inline QUrlTwoFlags operator&(E1 f) const
    { return QUrlTwoFlags(QFlag(i & f)); }
    constexpr inline QUrlTwoFlags operator&(E2 f) const
    { return QUrlTwoFlags(QFlag(i & f)); }
    constexpr inline QUrlTwoFlags operator~() const
    { return QUrlTwoFlags(QFlag(~i)); }

    constexpr inline bool testFlag(E1 f) const { return (i & f) == f && (f != 0 || i == int(f)); }
    constexpr inline bool testFlag(E2 f) const { return (i & f) == f && (f != 0 || i == int(f)); }
};

template<typename E1, typename E2>
class QTypeInfo<QUrlTwoFlags<E1, E2> > : public QTypeInfoMerger<QUrlTwoFlags<E1, E2>, E1, E2> {};

class QUrl;
// qHash is a friend, but we can't use default arguments for friends (§8.3.6.4)
Q_CORE_EXPORT size_t qHash(const QUrl &url, size_t seed = 0) noexcept;

class Q_CORE_EXPORT QUrl
{
public:
    enum ParsingMode {
        TolerantMode,
        StrictMode,
        DecodedMode
    };

    // encoding / toString values
    enum UrlFormattingOption : unsigned int {
        None = 0x0,
        RemoveScheme = 0x1,
        RemovePassword = 0x2,
        RemoveUserInfo = RemovePassword | 0x4,
        RemovePort = 0x8,
        RemoveAuthority = RemoveUserInfo | RemovePort | 0x10,
        RemovePath = 0x20,
        RemoveQuery = 0x40,
        RemoveFragment = 0x80,
        // 0x100 was a private code in Qt 4, keep unused for a while
        PreferLocalFile = 0x200,
        StripTrailingSlash = 0x400,
        RemoveFilename = 0x800,
        NormalizePathSegments = 0x1000
    };

    enum ComponentFormattingOption : unsigned int {
        PrettyDecoded = 0x000000,
        EncodeSpaces = 0x100000,
        EncodeUnicode = 0x200000,
        EncodeDelimiters = 0x400000 | 0x800000,
        EncodeReserved = 0x1000000,
        DecodeReserved = 0x2000000,
        // 0x4000000 used to indicate full-decode mode

        FullyEncoded = EncodeSpaces | EncodeUnicode | EncodeDelimiters | EncodeReserved,
        FullyDecoded = FullyEncoded | DecodeReserved | 0x4000000
    };
    Q_DECLARE_FLAGS(ComponentFormattingOptions, ComponentFormattingOption)
#ifdef Q_QDOC
private:
    // We need to let qdoc think that FormattingOptions is a normal QFlags, but
    // it needs to be a QUrlTwoFlags for compiling default arguments of some functions.
    template<typename T> struct QFlags : QUrlTwoFlags<T, ComponentFormattingOption>
    { using QUrlTwoFlags<T, ComponentFormattingOption>::QUrlTwoFlags; };
public:
    Q_DECLARE_FLAGS(FormattingOptions, UrlFormattingOption)
#else
    typedef QUrlTwoFlags<UrlFormattingOption, ComponentFormattingOption> FormattingOptions;
#endif

    QUrl();
    QUrl(const QUrl &copy);
    QUrl &operator =(const QUrl &copy);
#ifdef QT_NO_URL_CAST_FROM_STRING
    explicit QUrl(const QString &url, ParsingMode mode = TolerantMode);
#else
    QUrl(const QString &url, ParsingMode mode = TolerantMode);
    QUrl &operator=(const QString &url);
#endif
    QUrl(QUrl &&other) noexcept : d(other.d)
    { other.d = nullptr; }
    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_PURE_SWAP(QUrl)
    ~QUrl();

    void swap(QUrl &other) noexcept { qt_ptr_swap(d, other.d); }

    void setUrl(const QString &url, ParsingMode mode = TolerantMode);
    QString url(FormattingOptions options = FormattingOptions(PrettyDecoded)) const;
    QString toString(FormattingOptions options = FormattingOptions(PrettyDecoded)) const;
    QString toDisplayString(FormattingOptions options = FormattingOptions(PrettyDecoded)) const;
    [[nodiscard]] QUrl adjusted(FormattingOptions options) const;

    QByteArray toEncoded(FormattingOptions options = FullyEncoded) const;
    static QUrl fromEncoded(const QByteArray &url, ParsingMode mode = TolerantMode);

    enum UserInputResolutionOption {
        DefaultResolution,
        AssumeLocalFile
    };
    Q_DECLARE_FLAGS(UserInputResolutionOptions, UserInputResolutionOption)

    static QUrl fromUserInput(const QString &userInput, const QString &workingDirectory = QString(),
                              UserInputResolutionOptions options = DefaultResolution);

    bool isValid() const;
    QString errorString() const;

    bool isEmpty() const;
    void clear();

    void setScheme(const QString &scheme);
    QString scheme() const;

    void setAuthority(const QString &authority, ParsingMode mode = TolerantMode);
    QString authority(ComponentFormattingOptions options = PrettyDecoded) const;

    void setUserInfo(const QString &userInfo, ParsingMode mode = TolerantMode);
    QString userInfo(ComponentFormattingOptions options = PrettyDecoded) const;

    void setUserName(const QString &userName, ParsingMode mode = DecodedMode);
    QString userName(ComponentFormattingOptions options = FullyDecoded) const;

    void setPassword(const QString &password, ParsingMode mode = DecodedMode);
    QString password(ComponentFormattingOptions = FullyDecoded) const;

    void setHost(const QString &host, ParsingMode mode = DecodedMode);
    QString host(ComponentFormattingOptions = FullyDecoded) const;

    void setPort(int port);
    int port(int defaultPort = -1) const;

    void setPath(const QString &path, ParsingMode mode = DecodedMode);
    QString path(ComponentFormattingOptions options = FullyDecoded) const;
    QString fileName(ComponentFormattingOptions options = FullyDecoded) const;

    bool hasQuery() const;
    void setQuery(const QString &query, ParsingMode mode = TolerantMode);
    void setQuery(const QUrlQuery &query);
    QString query(ComponentFormattingOptions = PrettyDecoded) const;

    bool hasFragment() const;
    QString fragment(ComponentFormattingOptions options = PrettyDecoded) const;
    void setFragment(const QString &fragment, ParsingMode mode = TolerantMode);

    [[nodiscard]] QUrl resolved(const QUrl &relative) const;

    bool isRelative() const;
    bool isParentOf(const QUrl &url) const;

    bool isLocalFile() const;
    static QUrl fromLocalFile(const QString &localfile);
    QString toLocalFile() const;

    void detach();
    bool isDetached() const;

    bool operator <(const QUrl &url) const;
    bool operator ==(const QUrl &url) const;
    bool operator !=(const QUrl &url) const;

    bool matches(const QUrl &url, FormattingOptions options) const;

    static QString fromPercentEncoding(const QByteArray &);
    static QByteArray toPercentEncoding(const QString &,
                                        const QByteArray &exclude = QByteArray(),
                                        const QByteArray &include = QByteArray());
#if defined(Q_OS_DARWIN) || defined(Q_QDOC)
    static QUrl fromCFURL(CFURLRef url);
    CFURLRef toCFURL() const Q_DECL_CF_RETURNS_RETAINED;
    static QUrl fromNSURL(const NSURL *url);
    NSURL *toNSURL() const Q_DECL_NS_RETURNS_AUTORELEASED;
#endif

    enum AceProcessingOption : unsigned int {
        IgnoreIDNWhitelist = 0x1,
        AceTransitionalProcessing = 0x2,
    };
    Q_DECLARE_FLAGS(AceProcessingOptions, AceProcessingOption)

#if QT_CORE_REMOVED_SINCE(6, 3)
    static QString fromAce(const QByteArray &);
    static QByteArray toAce(const QString &);
#endif
    static QString fromAce(const QByteArray &domain, AceProcessingOptions options = {});
    static QByteArray toAce(const QString &domain, AceProcessingOptions options = {});

    static QStringList idnWhitelist();
    static QStringList toStringList(const QList<QUrl> &uris, FormattingOptions options = FormattingOptions(PrettyDecoded));
    static QList<QUrl> fromStringList(const QStringList &uris, ParsingMode mode = TolerantMode);

    static void setIdnWhitelist(const QStringList &);
    friend Q_CORE_EXPORT size_t qHash(const QUrl &url, size_t seed) noexcept;

private:
    QUrlPrivate *d;
    friend class QUrlQuery;

public:
    typedef QUrlPrivate * DataPtr;
    inline DataPtr &data_ptr() { return d; }
};

Q_DECLARE_SHARED(QUrl)
Q_DECLARE_OPERATORS_FOR_FLAGS(QUrl::ComponentFormattingOptions)
//Q_DECLARE_OPERATORS_FOR_FLAGS(QUrl::FormattingOptions)
Q_DECLARE_OPERATORS_FOR_FLAGS(QUrl::AceProcessingOptions)

#ifndef Q_QDOC
constexpr inline QUrl::FormattingOptions operator|(QUrl::UrlFormattingOption f1, QUrl::UrlFormattingOption f2)
{ return QUrl::FormattingOptions(f1) | f2; }
constexpr inline QUrl::FormattingOptions operator|(QUrl::UrlFormattingOption f1, QUrl::FormattingOptions f2)
{ return f2 | f1; }
constexpr inline QIncompatibleFlag operator|(QUrl::UrlFormattingOption f1, int f2)
{ return QIncompatibleFlag(uint(f1) | f2); }

// add operators for OR'ing the two types of flags
inline QUrl::FormattingOptions &operator|=(QUrl::FormattingOptions &i, QUrl::ComponentFormattingOptions f)
{ i |= QUrl::UrlFormattingOption(f.toInt()); return i; }
constexpr inline QUrl::FormattingOptions operator|(QUrl::UrlFormattingOption i, QUrl::ComponentFormattingOption f)
{ return i | QUrl::UrlFormattingOption(qToUnderlying(f)); }
constexpr inline QUrl::FormattingOptions operator|(QUrl::UrlFormattingOption i, QUrl::ComponentFormattingOptions f)
{ return i | QUrl::UrlFormattingOption(f.toInt()); }
constexpr inline QUrl::FormattingOptions operator|(QUrl::ComponentFormattingOption f, QUrl::UrlFormattingOption i)
{ return i | QUrl::UrlFormattingOption(qToUnderlying(f)); }
constexpr inline QUrl::FormattingOptions operator|(QUrl::ComponentFormattingOptions f, QUrl::UrlFormattingOption i)
{ return i | QUrl::UrlFormattingOption(f.toInt()); }
constexpr inline QUrl::FormattingOptions operator|(QUrl::FormattingOptions i, QUrl::ComponentFormattingOptions f)
{ return i | QUrl::UrlFormattingOption(f.toInt()); }
constexpr inline QUrl::FormattingOptions operator|(QUrl::ComponentFormattingOption f, QUrl::FormattingOptions i)
{ return i | QUrl::UrlFormattingOption(qToUnderlying(f)); }
constexpr inline QUrl::FormattingOptions operator|(QUrl::ComponentFormattingOptions f, QUrl::FormattingOptions i)
{ return i | QUrl::UrlFormattingOption(f.toInt()); }

//inline QUrl::UrlFormattingOption &operator=(const QUrl::UrlFormattingOption &i, QUrl::ComponentFormattingOptions f)
//{ i = int(f); f; }
#endif // Q_QDOC

#ifndef QT_NO_DATASTREAM
Q_CORE_EXPORT QDataStream &operator<<(QDataStream &, const QUrl &);
Q_CORE_EXPORT QDataStream &operator>>(QDataStream &, QUrl &);
#endif

#ifndef QT_NO_DEBUG_STREAM
Q_CORE_EXPORT QDebug operator<<(QDebug, const QUrl &);
#endif

QT_END_NAMESPACE

#endif // QURL_H
