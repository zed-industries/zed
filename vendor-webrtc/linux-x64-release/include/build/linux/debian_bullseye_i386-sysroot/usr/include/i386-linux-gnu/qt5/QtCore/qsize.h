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

#ifndef QSIZE_H
#define QSIZE_H

#include <QtCore/qnamespace.h>
#include <QtCore/qmargins.h>

#if defined(Q_OS_DARWIN) || defined(Q_QDOC)
struct CGSize;
#endif

QT_BEGIN_NAMESPACE


class Q_CORE_EXPORT QSize
{
public:
    Q_DECL_CONSTEXPR QSize() noexcept;
    Q_DECL_CONSTEXPR QSize(int w, int h) noexcept;

    Q_DECL_CONSTEXPR inline bool isNull() const noexcept;
    Q_DECL_CONSTEXPR inline bool isEmpty() const noexcept;
    Q_DECL_CONSTEXPR inline bool isValid() const noexcept;

    Q_DECL_CONSTEXPR inline int width() const noexcept;
    Q_DECL_CONSTEXPR inline int height() const noexcept;
    Q_DECL_RELAXED_CONSTEXPR inline void setWidth(int w) noexcept;
    Q_DECL_RELAXED_CONSTEXPR inline void setHeight(int h) noexcept;
    void transpose() noexcept;
    Q_REQUIRED_RESULT Q_DECL_CONSTEXPR inline QSize transposed() const noexcept;

    inline void scale(int w, int h, Qt::AspectRatioMode mode) noexcept;
    inline void scale(const QSize &s, Qt::AspectRatioMode mode) noexcept;
    Q_REQUIRED_RESULT QSize scaled(int w, int h, Qt::AspectRatioMode mode) const noexcept;
    Q_REQUIRED_RESULT QSize scaled(const QSize &s, Qt::AspectRatioMode mode) const noexcept;

    Q_REQUIRED_RESULT Q_DECL_CONSTEXPR inline QSize expandedTo(const QSize &) const noexcept;
    Q_REQUIRED_RESULT Q_DECL_CONSTEXPR inline QSize boundedTo(const QSize &) const noexcept;

    Q_REQUIRED_RESULT Q_DECL_CONSTEXPR QSize grownBy(QMargins m) const noexcept
    { return {width() + m.left() + m.right(), height() + m.top() + m.bottom()}; }
    Q_REQUIRED_RESULT Q_DECL_CONSTEXPR QSize shrunkBy(QMargins m) const noexcept
    { return {width() - m.left() - m.right(), height() - m.top() - m.bottom()}; }

    Q_DECL_RELAXED_CONSTEXPR inline int &rwidth() noexcept;
    Q_DECL_RELAXED_CONSTEXPR inline int &rheight() noexcept;

    Q_DECL_RELAXED_CONSTEXPR inline QSize &operator+=(const QSize &) noexcept;
    Q_DECL_RELAXED_CONSTEXPR inline QSize &operator-=(const QSize &) noexcept;
    Q_DECL_RELAXED_CONSTEXPR inline QSize &operator*=(qreal c) noexcept;
    inline QSize &operator/=(qreal c);

    friend inline Q_DECL_CONSTEXPR bool operator==(const QSize &, const QSize &) noexcept;
    friend inline Q_DECL_CONSTEXPR bool operator!=(const QSize &, const QSize &) noexcept;
    friend inline Q_DECL_CONSTEXPR const QSize operator+(const QSize &, const QSize &) noexcept;
    friend inline Q_DECL_CONSTEXPR const QSize operator-(const QSize &, const QSize &) noexcept;
    friend inline Q_DECL_CONSTEXPR const QSize operator*(const QSize &, qreal) noexcept;
    friend inline Q_DECL_CONSTEXPR const QSize operator*(qreal, const QSize &) noexcept;
    friend inline const QSize operator/(const QSize &, qreal);

#if defined(Q_OS_DARWIN) || defined(Q_QDOC)
    Q_REQUIRED_RESULT CGSize toCGSize() const noexcept;
#endif

private:
    int wd;
    int ht;
};
Q_DECLARE_TYPEINFO(QSize, Q_MOVABLE_TYPE);

/*****************************************************************************
  QSize stream functions
 *****************************************************************************/

#ifndef QT_NO_DATASTREAM
Q_CORE_EXPORT QDataStream &operator<<(QDataStream &, const QSize &);
Q_CORE_EXPORT QDataStream &operator>>(QDataStream &, QSize &);
#endif


/*****************************************************************************
  QSize inline functions
 *****************************************************************************/

Q_DECL_CONSTEXPR inline QSize::QSize() noexcept : wd(-1), ht(-1) {}

Q_DECL_CONSTEXPR inline QSize::QSize(int w, int h) noexcept : wd(w), ht(h) {}

Q_DECL_CONSTEXPR inline bool QSize::isNull() const noexcept
{ return wd==0 && ht==0; }

Q_DECL_CONSTEXPR inline bool QSize::isEmpty() const noexcept
{ return wd<1 || ht<1; }

Q_DECL_CONSTEXPR inline bool QSize::isValid() const noexcept
{ return wd>=0 && ht>=0; }

Q_DECL_CONSTEXPR inline int QSize::width() const noexcept
{ return wd; }

Q_DECL_CONSTEXPR inline int QSize::height() const noexcept
{ return ht; }

Q_DECL_RELAXED_CONSTEXPR inline void QSize::setWidth(int w) noexcept
{ wd = w; }

Q_DECL_RELAXED_CONSTEXPR inline void QSize::setHeight(int h) noexcept
{ ht = h; }

Q_DECL_CONSTEXPR inline QSize QSize::transposed() const noexcept
{ return QSize(ht, wd); }

inline void QSize::scale(int w, int h, Qt::AspectRatioMode mode) noexcept
{ scale(QSize(w, h), mode); }

inline void QSize::scale(const QSize &s, Qt::AspectRatioMode mode) noexcept
{ *this = scaled(s, mode); }

inline QSize QSize::scaled(int w, int h, Qt::AspectRatioMode mode) const noexcept
{ return scaled(QSize(w, h), mode); }

Q_DECL_RELAXED_CONSTEXPR inline int &QSize::rwidth() noexcept
{ return wd; }

Q_DECL_RELAXED_CONSTEXPR inline int &QSize::rheight() noexcept
{ return ht; }

Q_DECL_RELAXED_CONSTEXPR inline QSize &QSize::operator+=(const QSize &s) noexcept
{ wd+=s.wd; ht+=s.ht; return *this; }

Q_DECL_RELAXED_CONSTEXPR inline QSize &QSize::operator-=(const QSize &s) noexcept
{ wd-=s.wd; ht-=s.ht; return *this; }

Q_DECL_RELAXED_CONSTEXPR inline QSize &QSize::operator*=(qreal c) noexcept
{ wd = qRound(wd*c); ht = qRound(ht*c); return *this; }

Q_DECL_CONSTEXPR inline bool operator==(const QSize &s1, const QSize &s2) noexcept
{ return s1.wd == s2.wd && s1.ht == s2.ht; }

Q_DECL_CONSTEXPR inline bool operator!=(const QSize &s1, const QSize &s2) noexcept
{ return s1.wd != s2.wd || s1.ht != s2.ht; }

Q_DECL_CONSTEXPR inline const QSize operator+(const QSize & s1, const QSize & s2) noexcept
{ return QSize(s1.wd+s2.wd, s1.ht+s2.ht); }

Q_DECL_CONSTEXPR inline const QSize operator-(const QSize &s1, const QSize &s2) noexcept
{ return QSize(s1.wd-s2.wd, s1.ht-s2.ht); }

Q_DECL_CONSTEXPR inline const QSize operator*(const QSize &s, qreal c) noexcept
{ return QSize(qRound(s.wd*c), qRound(s.ht*c)); }

Q_DECL_CONSTEXPR inline const QSize operator*(qreal c, const QSize &s) noexcept
{ return QSize(qRound(s.wd*c), qRound(s.ht*c)); }

inline QSize &QSize::operator/=(qreal c)
{
    Q_ASSERT(!qFuzzyIsNull(c));
    wd = qRound(wd/c); ht = qRound(ht/c);
    return *this;
}

inline const QSize operator/(const QSize &s, qreal c)
{
    Q_ASSERT(!qFuzzyIsNull(c));
    return QSize(qRound(s.wd/c), qRound(s.ht/c));
}

Q_DECL_CONSTEXPR inline QSize QSize::expandedTo(const QSize & otherSize) const noexcept
{
    return QSize(qMax(wd,otherSize.wd), qMax(ht,otherSize.ht));
}

Q_DECL_CONSTEXPR inline QSize QSize::boundedTo(const QSize & otherSize) const noexcept
{
    return QSize(qMin(wd,otherSize.wd), qMin(ht,otherSize.ht));
}

#ifndef QT_NO_DEBUG_STREAM
Q_CORE_EXPORT QDebug operator<<(QDebug, const QSize &);
#endif


class Q_CORE_EXPORT QSizeF
{
public:
    Q_DECL_CONSTEXPR QSizeF() noexcept;
    Q_DECL_CONSTEXPR QSizeF(const QSize &sz) noexcept;
    Q_DECL_CONSTEXPR QSizeF(qreal w, qreal h) noexcept;

    inline bool isNull() const noexcept;
    Q_DECL_CONSTEXPR inline bool isEmpty() const noexcept;
    Q_DECL_CONSTEXPR inline bool isValid() const noexcept;

    Q_DECL_CONSTEXPR inline qreal width() const noexcept;
    Q_DECL_CONSTEXPR inline qreal height() const noexcept;
    Q_DECL_RELAXED_CONSTEXPR inline void setWidth(qreal w) noexcept;
    Q_DECL_RELAXED_CONSTEXPR inline void setHeight(qreal h) noexcept;
    void transpose() noexcept;
    Q_REQUIRED_RESULT Q_DECL_CONSTEXPR inline QSizeF transposed() const noexcept;

    inline void scale(qreal w, qreal h, Qt::AspectRatioMode mode) noexcept;
    inline void scale(const QSizeF &s, Qt::AspectRatioMode mode) noexcept;
    Q_REQUIRED_RESULT QSizeF scaled(qreal w, qreal h, Qt::AspectRatioMode mode) const noexcept;
    Q_REQUIRED_RESULT QSizeF scaled(const QSizeF &s, Qt::AspectRatioMode mode) const noexcept;

    Q_REQUIRED_RESULT Q_DECL_CONSTEXPR inline QSizeF expandedTo(const QSizeF &) const noexcept;
    Q_REQUIRED_RESULT Q_DECL_CONSTEXPR inline QSizeF boundedTo(const QSizeF &) const noexcept;

    Q_REQUIRED_RESULT Q_DECL_CONSTEXPR QSizeF grownBy(QMarginsF m) const noexcept
    { return {width() + m.left() + m.right(), height() + m.top() + m.bottom()}; }
    Q_REQUIRED_RESULT Q_DECL_CONSTEXPR QSizeF shrunkBy(QMarginsF m) const noexcept
    { return {width() - m.left() - m.right(), height() - m.top() - m.bottom()}; }

    Q_DECL_RELAXED_CONSTEXPR inline qreal &rwidth() noexcept;
    Q_DECL_RELAXED_CONSTEXPR inline qreal &rheight() noexcept;

    Q_DECL_RELAXED_CONSTEXPR inline QSizeF &operator+=(const QSizeF &) noexcept;
    Q_DECL_RELAXED_CONSTEXPR inline QSizeF &operator-=(const QSizeF &) noexcept;
    Q_DECL_RELAXED_CONSTEXPR inline QSizeF &operator*=(qreal c) noexcept;
    inline QSizeF &operator/=(qreal c);

    friend Q_DECL_CONSTEXPR inline bool operator==(const QSizeF &, const QSizeF &) noexcept;
    friend Q_DECL_CONSTEXPR inline bool operator!=(const QSizeF &, const QSizeF &) noexcept;
    friend Q_DECL_CONSTEXPR inline const QSizeF operator+(const QSizeF &, const QSizeF &) noexcept;
    friend Q_DECL_CONSTEXPR inline const QSizeF operator-(const QSizeF &, const QSizeF &) noexcept;
    friend Q_DECL_CONSTEXPR inline const QSizeF operator*(const QSizeF &, qreal) noexcept;
    friend Q_DECL_CONSTEXPR inline const QSizeF operator*(qreal, const QSizeF &) noexcept;
    friend inline const QSizeF operator/(const QSizeF &, qreal);

    Q_DECL_CONSTEXPR inline QSize toSize() const noexcept;

#if defined(Q_OS_DARWIN) || defined(Q_QDOC)
    Q_REQUIRED_RESULT static QSizeF fromCGSize(CGSize size) noexcept;
    Q_REQUIRED_RESULT CGSize toCGSize() const noexcept;
#endif

private:
    qreal wd;
    qreal ht;
};
Q_DECLARE_TYPEINFO(QSizeF, Q_MOVABLE_TYPE);


/*****************************************************************************
  QSizeF stream functions
 *****************************************************************************/

#ifndef QT_NO_DATASTREAM
Q_CORE_EXPORT QDataStream &operator<<(QDataStream &, const QSizeF &);
Q_CORE_EXPORT QDataStream &operator>>(QDataStream &, QSizeF &);
#endif


/*****************************************************************************
  QSizeF inline functions
 *****************************************************************************/

Q_DECL_CONSTEXPR inline QSizeF::QSizeF() noexcept : wd(-1.), ht(-1.) {}

Q_DECL_CONSTEXPR inline QSizeF::QSizeF(const QSize &sz) noexcept : wd(sz.width()), ht(sz.height()) {}

Q_DECL_CONSTEXPR inline QSizeF::QSizeF(qreal w, qreal h) noexcept : wd(w), ht(h) {}

inline bool QSizeF::isNull() const noexcept
{ return qIsNull(wd) && qIsNull(ht); }

Q_DECL_CONSTEXPR inline bool QSizeF::isEmpty() const noexcept
{ return wd <= 0. || ht <= 0.; }

Q_DECL_CONSTEXPR inline bool QSizeF::isValid() const noexcept
{ return wd >= 0. && ht >= 0.; }

Q_DECL_CONSTEXPR inline qreal QSizeF::width() const noexcept
{ return wd; }

Q_DECL_CONSTEXPR inline qreal QSizeF::height() const noexcept
{ return ht; }

Q_DECL_RELAXED_CONSTEXPR inline void QSizeF::setWidth(qreal w) noexcept
{ wd = w; }

Q_DECL_RELAXED_CONSTEXPR inline void QSizeF::setHeight(qreal h) noexcept
{ ht = h; }

Q_DECL_CONSTEXPR inline QSizeF QSizeF::transposed() const noexcept
{ return QSizeF(ht, wd); }

inline void QSizeF::scale(qreal w, qreal h, Qt::AspectRatioMode mode) noexcept
{ scale(QSizeF(w, h), mode); }

inline void QSizeF::scale(const QSizeF &s, Qt::AspectRatioMode mode) noexcept
{ *this = scaled(s, mode); }

inline QSizeF QSizeF::scaled(qreal w, qreal h, Qt::AspectRatioMode mode) const noexcept
{ return scaled(QSizeF(w, h), mode); }

Q_DECL_RELAXED_CONSTEXPR inline qreal &QSizeF::rwidth() noexcept
{ return wd; }

Q_DECL_RELAXED_CONSTEXPR inline qreal &QSizeF::rheight() noexcept
{ return ht; }

Q_DECL_RELAXED_CONSTEXPR inline QSizeF &QSizeF::operator+=(const QSizeF &s) noexcept
{ wd += s.wd; ht += s.ht; return *this; }

Q_DECL_RELAXED_CONSTEXPR inline QSizeF &QSizeF::operator-=(const QSizeF &s) noexcept
{ wd -= s.wd; ht -= s.ht; return *this; }

Q_DECL_RELAXED_CONSTEXPR inline QSizeF &QSizeF::operator*=(qreal c) noexcept
{ wd *= c; ht *= c; return *this; }

Q_DECL_CONSTEXPR inline bool operator==(const QSizeF &s1, const QSizeF &s2) noexcept
{ return qFuzzyCompare(s1.wd, s2.wd) && qFuzzyCompare(s1.ht, s2.ht); }

Q_DECL_CONSTEXPR inline bool operator!=(const QSizeF &s1, const QSizeF &s2) noexcept
{ return !qFuzzyCompare(s1.wd, s2.wd) || !qFuzzyCompare(s1.ht, s2.ht); }

Q_DECL_CONSTEXPR inline const QSizeF operator+(const QSizeF & s1, const QSizeF & s2) noexcept
{ return QSizeF(s1.wd+s2.wd, s1.ht+s2.ht); }

Q_DECL_CONSTEXPR inline const QSizeF operator-(const QSizeF &s1, const QSizeF &s2) noexcept
{ return QSizeF(s1.wd-s2.wd, s1.ht-s2.ht); }

Q_DECL_CONSTEXPR inline const QSizeF operator*(const QSizeF &s, qreal c) noexcept
{ return QSizeF(s.wd*c, s.ht*c); }

Q_DECL_CONSTEXPR inline const QSizeF operator*(qreal c, const QSizeF &s) noexcept
{ return QSizeF(s.wd*c, s.ht*c); }

inline QSizeF &QSizeF::operator/=(qreal c)
{
    Q_ASSERT(!qFuzzyIsNull(c));
    wd = wd/c; ht = ht/c;
    return *this;
}

inline const QSizeF operator/(const QSizeF &s, qreal c)
{
    Q_ASSERT(!qFuzzyIsNull(c));
    return QSizeF(s.wd/c, s.ht/c);
}

Q_DECL_CONSTEXPR inline QSizeF QSizeF::expandedTo(const QSizeF & otherSize) const noexcept
{
    return QSizeF(qMax(wd,otherSize.wd), qMax(ht,otherSize.ht));
}

Q_DECL_CONSTEXPR inline QSizeF QSizeF::boundedTo(const QSizeF & otherSize) const noexcept
{
    return QSizeF(qMin(wd,otherSize.wd), qMin(ht,otherSize.ht));
}

Q_DECL_CONSTEXPR inline QSize QSizeF::toSize() const noexcept
{
    return QSize(qRound(wd), qRound(ht));
}

#ifndef QT_NO_DEBUG_STREAM
Q_CORE_EXPORT QDebug operator<<(QDebug, const QSizeF &);
#endif

QT_END_NAMESPACE

#endif // QSIZE_H
