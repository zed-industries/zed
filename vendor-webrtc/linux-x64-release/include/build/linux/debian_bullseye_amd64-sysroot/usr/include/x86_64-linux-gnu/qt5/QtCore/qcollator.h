/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Copyright (C) 2013 Aleix Pol Gonzalez <aleixpol@kde.org>
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

#ifndef QCOLLATOR_H
#define QCOLLATOR_H

#include <QtCore/qstring.h>
#include <QtCore/qstringlist.h>
#include <QtCore/qlocale.h>

QT_BEGIN_NAMESPACE

class QCollatorPrivate;
class QCollatorSortKeyPrivate;

class Q_CORE_EXPORT QCollatorSortKey
{
    friend class QCollator;
public:
    QCollatorSortKey(const QCollatorSortKey &other);
    ~QCollatorSortKey();
    QCollatorSortKey &operator=(const QCollatorSortKey &other);
    inline QCollatorSortKey &operator=(QCollatorSortKey &&other) noexcept
    { swap(other); return *this; }
    void swap(QCollatorSortKey &other) noexcept
    { d.swap(other.d); }

    int compare(const QCollatorSortKey &key) const;

protected:
    QCollatorSortKey(QCollatorSortKeyPrivate*);

    QExplicitlySharedDataPointer<QCollatorSortKeyPrivate> d;

private:
    QCollatorSortKey();
};

inline bool operator<(const QCollatorSortKey &lhs, const QCollatorSortKey &rhs)
{
    return lhs.compare(rhs) < 0;
}

class Q_CORE_EXPORT QCollator
{
public:
    QCollator();
    explicit QCollator(const QLocale &locale);
    QCollator(const QCollator &);
    ~QCollator();
    QCollator &operator=(const QCollator &);
    QCollator(QCollator &&other) noexcept
        : d(other.d) { other.d = nullptr; }
    QCollator &operator=(QCollator &&other) noexcept
    { swap(other); return *this; }

    void swap(QCollator &other) noexcept
    { qSwap(d, other.d); }

    void setLocale(const QLocale &locale);
    QLocale locale() const;

    Qt::CaseSensitivity caseSensitivity() const;
    void setCaseSensitivity(Qt::CaseSensitivity cs);

    void setNumericMode(bool on);
    bool numericMode() const;

    void setIgnorePunctuation(bool on);
    bool ignorePunctuation() const;

#if QT_STRINGVIEW_LEVEL < 2
    int compare(const QString &s1, const QString &s2) const;
    int compare(const QStringRef &s1, const QStringRef &s2) const;
    int compare(const QChar *s1, int len1, const QChar *s2, int len2) const;

    bool operator()(const QString &s1, const QString &s2) const
    { return compare(s1, s2) < 0; }
#endif
    int compare(QStringView s1, QStringView s2) const;

    bool operator()(QStringView s1, QStringView s2) const
    { return compare(s1, s2) < 0; }

    QCollatorSortKey sortKey(const QString &string) const;

private:
    QCollatorPrivate *d;

    void detach();
};

Q_DECLARE_SHARED(QCollatorSortKey)
Q_DECLARE_SHARED(QCollator)

QT_END_NAMESPACE

#endif // QCOLLATOR_P_H
