// Copyright (C) 2020 Giuseppe D'Angelo <dangelog@gmail.com>.
// Copyright (C) 2020 Klarälvdalens Datakonsult AB, a KDAB Group company, info@kdab.com, author Giuseppe D'Angelo <giuseppe.dangelo@kdab.com>
// Copyright (C) 2021 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QREGULAREXPRESSION_H
#define QREGULAREXPRESSION_H

#include <QtCore/qglobal.h>
#include <QtCore/qstring.h>
#include <QtCore/qstringview.h>
#include <QtCore/qshareddata.h>
#include <QtCore/qvariant.h>

#include <iterator>

QT_REQUIRE_CONFIG(regularexpression);

QT_BEGIN_NAMESPACE

class QRegularExpressionMatch;
class QRegularExpressionMatchIterator;
struct QRegularExpressionPrivate;
class QRegularExpression;

QT_DECLARE_QESDP_SPECIALIZATION_DTOR_WITH_EXPORT(QRegularExpressionPrivate, Q_CORE_EXPORT)

Q_CORE_EXPORT size_t qHash(const QRegularExpression &key, size_t seed = 0) noexcept;

class Q_CORE_EXPORT QRegularExpression
{
public:
    enum PatternOption {
        NoPatternOption                = 0x0000,
        CaseInsensitiveOption          = 0x0001,
        DotMatchesEverythingOption     = 0x0002,
        MultilineOption                = 0x0004,
        ExtendedPatternSyntaxOption    = 0x0008,
        InvertedGreedinessOption       = 0x0010,
        DontCaptureOption              = 0x0020,
        UseUnicodePropertiesOption     = 0x0040,
        // Formerly (no-ops deprecated in 5.12, removed 6.0):
        // OptimizeOnFirstUsageOption = 0x0080,
        // DontAutomaticallyOptimizeOption = 0x0100,
    };
    Q_DECLARE_FLAGS(PatternOptions, PatternOption)

    PatternOptions patternOptions() const;
    void setPatternOptions(PatternOptions options);

    QRegularExpression();
    explicit QRegularExpression(const QString &pattern, PatternOptions options = NoPatternOption);
    QRegularExpression(const QRegularExpression &re);
    QRegularExpression(QRegularExpression &&re) = default;
    ~QRegularExpression();
    QRegularExpression &operator=(const QRegularExpression &re);
    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_PURE_SWAP(QRegularExpression)

    void swap(QRegularExpression &other) noexcept { d.swap(other.d); }

    QString pattern() const;
    void setPattern(const QString &pattern);

    [[nodiscard]]
    bool isValid() const;
    qsizetype patternErrorOffset() const;
    QString errorString() const;

    int captureCount() const;
    QStringList namedCaptureGroups() const;

    enum MatchType {
        NormalMatch = 0,
        PartialPreferCompleteMatch,
        PartialPreferFirstMatch,
        NoMatch
    };

    enum MatchOption {
        NoMatchOption              = 0x0000,
        AnchorAtOffsetMatchOption  = 0x0001,
        AnchoredMatchOption Q_DECL_ENUMERATOR_DEPRECATED_X(
            "Use AnchorAtOffsetMatchOption instead") = AnchorAtOffsetMatchOption, // Rename@Qt6.0
        DontCheckSubjectStringMatchOption = 0x0002
    };
    Q_DECLARE_FLAGS(MatchOptions, MatchOption)

    [[nodiscard]]
    QRegularExpressionMatch match(const QString &subject,
                                  qsizetype offset          = 0,
                                  MatchType matchType       = NormalMatch,
                                  MatchOptions matchOptions = NoMatchOption) const;

    [[nodiscard]]
    QRegularExpressionMatch match(QStringView subjectView,
                                  qsizetype offset          = 0,
                                  MatchType matchType       = NormalMatch,
                                  MatchOptions matchOptions = NoMatchOption) const;

    [[nodiscard]]
    QRegularExpressionMatchIterator globalMatch(const QString &subject,
                                                qsizetype offset          = 0,
                                                MatchType matchType       = NormalMatch,
                                                MatchOptions matchOptions = NoMatchOption) const;

    [[nodiscard]]
    QRegularExpressionMatchIterator globalMatch(QStringView subjectView,
                                                qsizetype offset          = 0,
                                                MatchType matchType       = NormalMatch,
                                                MatchOptions matchOptions = NoMatchOption) const;

    void optimize() const;

    enum WildcardConversionOption {
        DefaultWildcardConversion = 0x0,
        UnanchoredWildcardConversion = 0x1
    };
    Q_DECLARE_FLAGS(WildcardConversionOptions, WildcardConversionOption)

    static QString escape(const QString &str)
    {
        return escape(qToStringViewIgnoringNull(str));
    }

    static QString wildcardToRegularExpression(const QString &str, WildcardConversionOptions options = DefaultWildcardConversion)
    {
        return wildcardToRegularExpression(qToStringViewIgnoringNull(str), options);
    }

    static inline QString anchoredPattern(const QString &expression)
    {
        return anchoredPattern(qToStringViewIgnoringNull(expression));
    }

    static QString escape(QStringView str);
    static QString wildcardToRegularExpression(QStringView str, WildcardConversionOptions options = DefaultWildcardConversion);
    static QString anchoredPattern(QStringView expression);

    static QRegularExpression fromWildcard(QStringView pattern, Qt::CaseSensitivity cs = Qt::CaseInsensitive,
                                           WildcardConversionOptions options = DefaultWildcardConversion);

    bool operator==(const QRegularExpression &re) const;
    inline bool operator!=(const QRegularExpression &re) const { return !operator==(re); }

private:
    friend struct QRegularExpressionPrivate;
    friend class QRegularExpressionMatch;
    friend struct QRegularExpressionMatchPrivate;
    friend class QRegularExpressionMatchIterator;
    friend Q_CORE_EXPORT size_t qHash(const QRegularExpression &key, size_t seed) noexcept;

    QRegularExpression(QRegularExpressionPrivate &dd);
    QExplicitlySharedDataPointer<QRegularExpressionPrivate> d;
};

Q_DECLARE_SHARED(QRegularExpression)
Q_DECLARE_OPERATORS_FOR_FLAGS(QRegularExpression::PatternOptions)
Q_DECLARE_OPERATORS_FOR_FLAGS(QRegularExpression::MatchOptions)

#ifndef QT_NO_DATASTREAM
Q_CORE_EXPORT QDataStream &operator<<(QDataStream &out, const QRegularExpression &re);
Q_CORE_EXPORT QDataStream &operator>>(QDataStream &in, QRegularExpression &re);
#endif

#ifndef QT_NO_DEBUG_STREAM
Q_CORE_EXPORT QDebug operator<<(QDebug debug, const QRegularExpression &re);
Q_CORE_EXPORT QDebug operator<<(QDebug debug, QRegularExpression::PatternOptions patternOptions);
#endif

struct QRegularExpressionMatchPrivate;
QT_DECLARE_QESDP_SPECIALIZATION_DTOR_WITH_EXPORT(QRegularExpressionMatchPrivate, Q_CORE_EXPORT)

class Q_CORE_EXPORT QRegularExpressionMatch
{
public:
    QRegularExpressionMatch();
    ~QRegularExpressionMatch();
    QRegularExpressionMatch(const QRegularExpressionMatch &match);
    QRegularExpressionMatch(QRegularExpressionMatch &&match) = default;
    QRegularExpressionMatch &operator=(const QRegularExpressionMatch &match);
    QRegularExpressionMatch &operator=(QRegularExpressionMatch &&match) noexcept
    { d.swap(match.d); return *this; }
    void swap(QRegularExpressionMatch &other) noexcept { d.swap(other.d); }

    QRegularExpression regularExpression() const;
    QRegularExpression::MatchType matchType() const;
    QRegularExpression::MatchOptions matchOptions() const;

    bool hasMatch() const;
    bool hasPartialMatch() const;

    bool isValid() const;

    int lastCapturedIndex() const;

    bool hasCaptured(const QString &name) const
    { return hasCaptured(QStringView(name)); }
    bool hasCaptured(QStringView name) const;
    bool hasCaptured(int nth) const;

    QString captured(int nth = 0) const;
    QStringView capturedView(int nth = 0) const;

    QString captured(const QString &name) const
    { return captured(QStringView(name)); }
    QString captured(QStringView name) const;
    QStringView capturedView(QStringView name) const;

    QStringList capturedTexts() const;

    qsizetype capturedStart(int nth = 0) const;
    qsizetype capturedLength(int nth = 0) const;
    qsizetype capturedEnd(int nth = 0) const;

    qsizetype capturedStart(const QString &name) const
    { return capturedStart(QStringView(name)); }
    qsizetype capturedLength(const QString &name) const
    { return capturedLength(QStringView(name)); }
    qsizetype capturedEnd(const QString &name) const
    { return capturedEnd(QStringView(name)); }

    qsizetype capturedStart(QStringView name) const;
    qsizetype capturedLength(QStringView name) const;
    qsizetype capturedEnd(QStringView name) const;

private:
    friend class QRegularExpression;
    friend struct QRegularExpressionMatchPrivate;
    friend class QRegularExpressionMatchIterator;

    QRegularExpressionMatch(QRegularExpressionMatchPrivate &dd);
    QExplicitlySharedDataPointer<QRegularExpressionMatchPrivate> d;
};

Q_DECLARE_SHARED(QRegularExpressionMatch)

#ifndef QT_NO_DEBUG_STREAM
Q_CORE_EXPORT QDebug operator<<(QDebug debug, const QRegularExpressionMatch &match);
#endif

namespace QtPrivate {
class QRegularExpressionMatchIteratorRangeBasedForIterator;
class QRegularExpressionMatchIteratorRangeBasedForIteratorSentinel {};
}

struct QRegularExpressionMatchIteratorPrivate;
QT_DECLARE_QESDP_SPECIALIZATION_DTOR_WITH_EXPORT(QRegularExpressionMatchIteratorPrivate, Q_CORE_EXPORT)

class Q_CORE_EXPORT QRegularExpressionMatchIterator
{
public:
    QRegularExpressionMatchIterator();
    ~QRegularExpressionMatchIterator();
    QRegularExpressionMatchIterator(const QRegularExpressionMatchIterator &iterator);
    QRegularExpressionMatchIterator(QRegularExpressionMatchIterator &&iterator) = default;
    QRegularExpressionMatchIterator &operator=(const QRegularExpressionMatchIterator &iterator);
    QRegularExpressionMatchIterator &operator=(QRegularExpressionMatchIterator &&iterator) noexcept
    { d.swap(iterator.d); return *this; }
    void swap(QRegularExpressionMatchIterator &other) noexcept { d.swap(other.d); }

    bool isValid() const;

    bool hasNext() const;
    QRegularExpressionMatch next();
    QRegularExpressionMatch peekNext() const;

    QRegularExpression regularExpression() const;
    QRegularExpression::MatchType matchType() const;
    QRegularExpression::MatchOptions matchOptions() const;

private:
    friend class QRegularExpression;
    friend Q_CORE_EXPORT QtPrivate::QRegularExpressionMatchIteratorRangeBasedForIterator begin(const QRegularExpressionMatchIterator &iterator);
    friend QtPrivate::QRegularExpressionMatchIteratorRangeBasedForIteratorSentinel end(const QRegularExpressionMatchIterator &) { return {}; }

    QRegularExpressionMatchIterator(QRegularExpressionMatchIteratorPrivate &dd);
    QExplicitlySharedDataPointer<QRegularExpressionMatchIteratorPrivate> d;
};

namespace QtPrivate {

// support for range-based for loop
class QRegularExpressionMatchIteratorRangeBasedForIterator
{
public:
    using value_type = QRegularExpressionMatch;
    using difference_type = int;
    using reference_type = const QRegularExpressionMatch &;
    using pointer_type = const QRegularExpressionMatch *;
    using iterator_category = std::forward_iterator_tag;

    QRegularExpressionMatchIteratorRangeBasedForIterator()
        : m_atEnd(true)
    {
    }

    explicit QRegularExpressionMatchIteratorRangeBasedForIterator(const QRegularExpressionMatchIterator &iterator)
        : m_matchIterator(iterator),
        m_currentMatch(),
        m_atEnd(false)
    {
        ++*this;
    }

    const QRegularExpressionMatch &operator*() const
    {
        Q_ASSERT_X(!m_atEnd, Q_FUNC_INFO, "operator* called on an iterator already at the end");
        return m_currentMatch;
    }

    QRegularExpressionMatchIteratorRangeBasedForIterator &operator++()
    {
        Q_ASSERT_X(!m_atEnd, Q_FUNC_INFO, "operator++ called on an iterator already at the end");
        if (m_matchIterator.hasNext()) {
            m_currentMatch = m_matchIterator.next();
        } else {
            m_currentMatch = QRegularExpressionMatch();
            m_atEnd = true;
        }

        return *this;
    }

    QRegularExpressionMatchIteratorRangeBasedForIterator operator++(int)
    {
        QRegularExpressionMatchIteratorRangeBasedForIterator i = *this;
        ++*this;
        return i;
    }

private:
    // [input.iterators] imposes operator== on us. Unfortunately, it's not
    // trivial to implement, so just do the bare minimum to satifisfy
    // Cpp17EqualityComparable.
    friend bool operator==(const QRegularExpressionMatchIteratorRangeBasedForIterator &lhs,
                           const QRegularExpressionMatchIteratorRangeBasedForIterator &rhs) noexcept
    {
        return (&lhs == &rhs);
    }

    friend bool operator!=(const QRegularExpressionMatchIteratorRangeBasedForIterator &lhs,
                           const QRegularExpressionMatchIteratorRangeBasedForIterator &rhs) noexcept
    {
        return !(lhs == rhs);
    }

    // This is what we really use in a range-based for.
    friend bool operator==(const QRegularExpressionMatchIteratorRangeBasedForIterator &lhs,
                           QRegularExpressionMatchIteratorRangeBasedForIteratorSentinel) noexcept
    {
        return lhs.m_atEnd;
    }

    friend bool operator!=(const QRegularExpressionMatchIteratorRangeBasedForIterator &lhs,
                           QRegularExpressionMatchIteratorRangeBasedForIteratorSentinel) noexcept
    {
        return !lhs.m_atEnd;
    }

    QRegularExpressionMatchIterator m_matchIterator;
    QRegularExpressionMatch m_currentMatch;
    bool m_atEnd;
};

} // namespace QtPrivate

Q_DECLARE_SHARED(QRegularExpressionMatchIterator)

QT_END_NAMESPACE

#endif // QREGULAREXPRESSION_H
