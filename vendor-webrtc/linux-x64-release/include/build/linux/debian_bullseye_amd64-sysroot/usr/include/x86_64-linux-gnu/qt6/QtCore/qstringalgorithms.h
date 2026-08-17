// Copyright (C) 2017 Klarälvdalens Datakonsult AB, a KDAB Group company, info@kdab.com, author Marc Mutz <marc.mutz@kdab.com>
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QSTRINGALGORITHMS_H
#define QSTRINGALGORITHMS_H

#include <QtCore/qnamespace.h>
#include <QtCore/qstringfwd.h>
#include <QtCore/qcontainerfwd.h>
#if 0
#pragma qt_class(QStringAlgorithms)
#endif

QT_BEGIN_NAMESPACE

namespace QtPrivate {

[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION qsizetype qustrlen(const char16_t *str) noexcept;
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION const char16_t *qustrchr(QStringView str, char16_t ch) noexcept;

[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION int compareStrings(QStringView   lhs, QStringView   rhs, Qt::CaseSensitivity cs = Qt::CaseSensitive) noexcept;
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION int compareStrings(QStringView   lhs, QLatin1StringView rhs, Qt::CaseSensitivity cs = Qt::CaseSensitive) noexcept;
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION int compareStrings(QStringView   lhs, QBasicUtf8StringView<false> rhs, Qt::CaseSensitivity cs = Qt::CaseSensitive) noexcept;
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION int compareStrings(QLatin1StringView lhs, QStringView   rhs, Qt::CaseSensitivity cs = Qt::CaseSensitive) noexcept;
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION int compareStrings(QLatin1StringView lhs, QLatin1StringView rhs, Qt::CaseSensitivity cs = Qt::CaseSensitive) noexcept;
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION int compareStrings(QLatin1StringView lhs, QBasicUtf8StringView<false> rhs, Qt::CaseSensitivity cs = Qt::CaseSensitive) noexcept;
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION int compareStrings(QBasicUtf8StringView<false> lhs, QStringView   rhs, Qt::CaseSensitivity cs = Qt::CaseSensitive) noexcept;
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION int compareStrings(QBasicUtf8StringView<false> lhs, QLatin1StringView rhs, Qt::CaseSensitivity cs = Qt::CaseSensitive) noexcept;
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION int compareStrings(QBasicUtf8StringView<false> lhs, QBasicUtf8StringView<false> rhs, Qt::CaseSensitivity cs = Qt::CaseSensitive) noexcept;

[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION bool equalStrings(QStringView   lhs, QStringView   rhs) noexcept;
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION bool equalStrings(QStringView   lhs, QLatin1StringView rhs) noexcept;
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION bool equalStrings(QStringView   lhs, QBasicUtf8StringView<false> rhs) noexcept;
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION bool equalStrings(QLatin1StringView lhs, QStringView   rhs) noexcept;
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION bool equalStrings(QLatin1StringView lhs, QLatin1StringView rhs) noexcept;
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION bool equalStrings(QLatin1StringView lhs, QBasicUtf8StringView<false> rhs) noexcept;
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION bool equalStrings(QBasicUtf8StringView<false> lhs, QStringView   rhs) noexcept;
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION bool equalStrings(QBasicUtf8StringView<false> lhs, QLatin1StringView rhs) noexcept;
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION bool equalStrings(QBasicUtf8StringView<false> lhs, QBasicUtf8StringView<false> rhs) noexcept;

[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION bool startsWith(QStringView   haystack, QStringView   needle, Qt::CaseSensitivity cs = Qt::CaseSensitive) noexcept;
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION bool startsWith(QStringView   haystack, QLatin1StringView needle, Qt::CaseSensitivity cs = Qt::CaseSensitive) noexcept;
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION bool startsWith(QLatin1StringView haystack, QStringView   needle, Qt::CaseSensitivity cs = Qt::CaseSensitive) noexcept;
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION bool startsWith(QLatin1StringView haystack, QLatin1StringView needle, Qt::CaseSensitivity cs = Qt::CaseSensitive) noexcept;

[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION bool endsWith(QStringView   haystack, QStringView   needle, Qt::CaseSensitivity cs = Qt::CaseSensitive) noexcept;
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION bool endsWith(QStringView   haystack, QLatin1StringView needle, Qt::CaseSensitivity cs = Qt::CaseSensitive) noexcept;
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION bool endsWith(QLatin1StringView haystack, QStringView   needle, Qt::CaseSensitivity cs = Qt::CaseSensitive) noexcept;
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION bool endsWith(QLatin1StringView haystack, QLatin1StringView needle, Qt::CaseSensitivity cs = Qt::CaseSensitive) noexcept;

[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION qsizetype findString(QStringView haystack, qsizetype from, QStringView needle, Qt::CaseSensitivity cs = Qt::CaseSensitive) noexcept;
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION qsizetype findString(QStringView haystack, qsizetype from, QLatin1StringView needle, Qt::CaseSensitivity cs = Qt::CaseSensitive) noexcept;
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION qsizetype findString(QLatin1StringView haystack, qsizetype from, QStringView needle, Qt::CaseSensitivity cs = Qt::CaseSensitive) noexcept;
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION qsizetype findString(QLatin1StringView haystack, qsizetype from, QLatin1StringView needle, Qt::CaseSensitivity cs = Qt::CaseSensitive) noexcept;

[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION qsizetype lastIndexOf(QStringView haystack, qsizetype from, QStringView needle, Qt::CaseSensitivity cs = Qt::CaseSensitive) noexcept;
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION qsizetype lastIndexOf(QStringView haystack, qsizetype from, QLatin1StringView needle, Qt::CaseSensitivity cs = Qt::CaseSensitive) noexcept;
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION qsizetype lastIndexOf(QLatin1StringView haystack, qsizetype from, QStringView needle, Qt::CaseSensitivity cs = Qt::CaseSensitive) noexcept;
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION qsizetype lastIndexOf(QLatin1StringView haystack, qsizetype from, QLatin1StringView needle, Qt::CaseSensitivity cs = Qt::CaseSensitive) noexcept;

[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION QStringView   trimmed(QStringView   s) noexcept;
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION QLatin1StringView trimmed(QLatin1StringView s) noexcept;

[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION qsizetype count(QStringView haystack, QChar needle, Qt::CaseSensitivity cs = Qt::CaseSensitive) noexcept;
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION qsizetype count(QStringView haystack, QStringView needle, Qt::CaseSensitivity cs = Qt::CaseSensitive) noexcept;
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION qsizetype count(QStringView haystack, QLatin1StringView needle, Qt::CaseSensitivity cs = Qt::CaseSensitive);
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION qsizetype count(QLatin1StringView haystack, QLatin1StringView needle, Qt::CaseSensitivity cs = Qt::CaseSensitive);
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION qsizetype count(QLatin1StringView haystack, QStringView needle, Qt::CaseSensitivity cs = Qt::CaseSensitive);
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION qsizetype count(QLatin1StringView haystack, QChar needle, Qt::CaseSensitivity cs = Qt::CaseSensitive) noexcept;

#if QT_CONFIG(regularexpression)
// ### Qt 7: unify these overloads;
// remove the ones taking only a QStringView, export the others, adjust callers
[[nodiscard]] qsizetype indexOf(QStringView viewHaystack,
                                const QString *stringHaystack,
                                const QRegularExpression &re,
                                qsizetype from = 0,
                                QRegularExpressionMatch *rmatch = nullptr);
[[nodiscard]] Q_CORE_EXPORT qsizetype indexOf(QStringView haystack,
                                              const QRegularExpression &re,
                                              qsizetype from = 0,
                                              QRegularExpressionMatch *rmatch = nullptr);
[[nodiscard]] qsizetype lastIndexOf(QStringView viewHaystack,
                                    const QString *stringHaystack,
                                    const QRegularExpression &re,
                                    qsizetype from = -1,
                                    QRegularExpressionMatch *rmatch = nullptr);
[[nodiscard]] Q_CORE_EXPORT qsizetype lastIndexOf(QStringView haystack,
                                                  const QRegularExpression &re,
                                                  qsizetype from = -1,
                                                  QRegularExpressionMatch *rmatch = nullptr);
[[nodiscard]] bool contains(QStringView viewHaystack,
                            const QString *stringHaystack,
                            const QRegularExpression &re,
                            QRegularExpressionMatch *rmatch = nullptr);
[[nodiscard]] Q_CORE_EXPORT bool contains(QStringView haystack,
                                          const QRegularExpression &re,
                                          QRegularExpressionMatch *rmatch = nullptr);
[[nodiscard]] Q_CORE_EXPORT qsizetype count(QStringView haystack, const QRegularExpression &re);
#endif

[[nodiscard]] Q_CORE_EXPORT QString convertToQString(QAnyStringView s);

[[nodiscard]] Q_CORE_EXPORT QByteArray convertToLatin1(QStringView str);
[[nodiscard]] Q_CORE_EXPORT QByteArray convertToUtf8(QStringView str);
[[nodiscard]] Q_CORE_EXPORT QByteArray convertToLocal8Bit(QStringView str);
[[nodiscard]] Q_CORE_EXPORT QList<uint> convertToUcs4(QStringView str); // ### Qt 7 char32_t

[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION bool isRightToLeft(QStringView string) noexcept;

[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION bool isAscii(QLatin1StringView s) noexcept;
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION bool isAscii(QStringView   s) noexcept;
[[nodiscard]] constexpr inline                   bool isLatin1(QLatin1StringView s) noexcept;
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION bool isLatin1(QStringView   s) noexcept;
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION bool isValidUtf16(QStringView s) noexcept;

} // namespace QtPRivate

QT_END_NAMESPACE

#endif // QSTRINGALGORTIHMS_H
