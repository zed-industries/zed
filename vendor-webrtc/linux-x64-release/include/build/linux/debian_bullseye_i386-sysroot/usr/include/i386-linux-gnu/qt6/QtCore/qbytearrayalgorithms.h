// Copyright (C) 2021 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QBYTEARRAYALGORITHMS_H
#define QBYTEARRAYALGORITHMS_H

#include <QtCore/qnamespace.h>

#include <string.h>
#include <stdarg.h>

#if 0
#pragma qt_class(QByteArrayAlgorithms)
#endif

QT_BEGIN_NAMESPACE

class QByteArrayView;

namespace QtPrivate {

[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION
bool startsWith(QByteArrayView haystack, QByteArrayView needle) noexcept;

[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION
bool endsWith(QByteArrayView haystack, QByteArrayView needle) noexcept;

[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION
qsizetype findByteArray(QByteArrayView haystack, qsizetype from, QByteArrayView needle) noexcept;

[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION
qsizetype lastIndexOf(QByteArrayView haystack, qsizetype from, QByteArrayView needle) noexcept;

[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION
qsizetype count(QByteArrayView haystack, QByteArrayView needle) noexcept;

[[nodiscard]] Q_CORE_EXPORT int compareMemory(QByteArrayView lhs, QByteArrayView rhs);

[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION QByteArrayView trimmed(QByteArrayView s) noexcept;

[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION bool isValidUtf8(QByteArrayView s) noexcept;

template <typename T>
class ParsedNumber
{
    T m_value;
    quint32 m_error : 1;
    quint32 m_reserved : 31;
    void *m_reserved2 = nullptr;
public:
    constexpr ParsedNumber() noexcept : m_value(), m_error(true), m_reserved(0) {}
    constexpr explicit ParsedNumber(T v) : m_value(v), m_error(false), m_reserved(0) {}

    // minimal optional-like API:
    explicit operator bool() const noexcept { return !m_error; }
    T &operator*() { Q_ASSERT(*this); return m_value; }
    const T &operator*() const { Q_ASSERT(*this); return m_value; }
    T *operator->() noexcept { return *this ? &m_value : nullptr; }
    const T *operator->() const noexcept { return *this ? &m_value : nullptr; }
    template <typename U> // not = T, as that'd allow calls that are incompatible with std::optional
    T value_or(U &&u) const { return *this ? m_value : T(std::forward<U>(u)); }
};

[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION ParsedNumber<double> toDouble(QByteArrayView a) noexcept;
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION ParsedNumber<float> toFloat(QByteArrayView a) noexcept;
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION ParsedNumber<qlonglong> toSignedInteger(QByteArrayView data, int base);
[[nodiscard]] Q_CORE_EXPORT Q_DECL_PURE_FUNCTION ParsedNumber<qulonglong> toUnsignedInteger(QByteArrayView data, int base);

// QByteArrayView has incomplete type here, and we can't include qbytearrayview.h,
// since it includes qbytearrayalgorithms.h. Use the ByteArrayView template type as
// a workaround.
template <typename T, typename ByteArrayView,
          typename = std::enable_if_t<std::is_same_v<ByteArrayView, QByteArrayView>>>
static inline T toIntegral(ByteArrayView data, bool *ok, int base)
{
    const auto val = [&] {
        if constexpr (std::is_unsigned_v<T>)
            return toUnsignedInteger(data, base);
        else
            return toSignedInteger(data, base);
    }();
    const bool failed = !val || T(*val) != *val;
    if (ok)
        *ok = !failed;
    if (failed)
        return 0;
    return T(*val);
}

} // namespace QtPrivate

/*****************************************************************************
  Safe and portable C string functions; extensions to standard string.h
 *****************************************************************************/

Q_CORE_EXPORT char *qstrdup(const char *);

inline size_t qstrlen(const char *str)
{
    QT_WARNING_PUSH
#if defined(Q_CC_GNU_ONLY) && Q_CC_GNU >= 900 && Q_CC_GNU < 1000
    // spurious compiler warning (https://gcc.gnu.org/bugzilla/show_bug.cgi?id=91490#c6)
    // when Q_DECLARE_METATYPE_TEMPLATE_1ARG is used
    QT_WARNING_DISABLE_GCC("-Wstringop-overflow")
#endif
    return str ? strlen(str) : 0;
    QT_WARNING_POP
}

inline size_t qstrnlen(const char *str, size_t maxlen)
{
    if (!str)
        return 0;
    auto end = static_cast<const char *>(memchr(str, '\0', maxlen));
    return end ? end - str : maxlen;
}

// implemented in qbytearray.cpp
Q_CORE_EXPORT char *qstrcpy(char *dst, const char *src);
Q_CORE_EXPORT char *qstrncpy(char *dst, const char *src, size_t len);

Q_CORE_EXPORT int qstrcmp(const char *str1, const char *str2);

inline int qstrncmp(const char *str1, const char *str2, size_t len)
{
    return (str1 && str2) ? strncmp(str1, str2, len)
        : (str1 ? 1 : (str2 ? -1 : 0));
}
Q_CORE_EXPORT int qstricmp(const char *, const char *);
Q_CORE_EXPORT int qstrnicmp(const char *, const char *, size_t len);
Q_CORE_EXPORT int qstrnicmp(const char *, qsizetype, const char *, qsizetype = -1);

// implemented in qvsnprintf.cpp
Q_CORE_EXPORT int qvsnprintf(char *str, size_t n, const char *fmt, va_list ap);
Q_CORE_EXPORT int qsnprintf(char *str, size_t n, const char *fmt, ...);

// qChecksum: Internet checksum
Q_CORE_EXPORT quint16 qChecksum(QByteArrayView data, Qt::ChecksumType standard = Qt::ChecksumIso3309);

QT_END_NAMESPACE

#endif // QBYTEARRAYALGORITHMS_H
