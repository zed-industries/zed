// Copyright (C) 2020 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QPAGERANGES_H
#define QPAGERANGES_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/qstring.h>
#include <QtCore/qlist.h>
#include <QtCore/qshareddata.h>
#include <QtCore/qmetatype.h>

QT_BEGIN_NAMESPACE

class QDebug;
class QDataStream;
class QPageRangesPrivate;
QT_DECLARE_QESDP_SPECIALIZATION_DTOR_WITH_EXPORT(QPageRangesPrivate, Q_GUI_EXPORT)

class Q_GUI_EXPORT QPageRanges
{
public:
    QPageRanges();
    ~QPageRanges();

    QPageRanges(const QPageRanges &other) noexcept;
    QPageRanges &operator=(const QPageRanges &other) noexcept;

    QPageRanges(QPageRanges &&other) noexcept = default;
    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_PURE_SWAP(QPageRanges)
    void swap(QPageRanges &other) noexcept
    { d.swap(other.d); }

    friend bool operator==(const QPageRanges &lhs, const QPageRanges &rhs) noexcept
    { return lhs.isEqual(rhs); }
    friend bool operator!=(const QPageRanges &lhs, const QPageRanges &rhs) noexcept
    { return !lhs.isEqual(rhs); }

    struct Range {
        int from = -1;
        int to = -1;
        bool contains(int pageNumber) const noexcept
        { return from <= pageNumber && to >= pageNumber; }
        friend bool operator==(Range lhs, Range rhs) noexcept
        { return lhs.from == rhs.from && lhs.to == rhs.to; }
        friend bool operator!=(Range lhs, Range rhs) noexcept
        { return !(lhs == rhs); }
        friend bool operator<(Range lhs, Range rhs) noexcept
        { return lhs.from < rhs.from || (!(rhs.from < lhs.from) && lhs.to < rhs.to); }
    };

    void addPage(int pageNumber);
    void addRange(int from, int to);
    QList<Range> toRangeList() const;
    void clear();

    QString toString() const;
    static QPageRanges fromString(const QString &ranges);

    bool contains(int pageNumber) const;
    bool isEmpty() const;
    int firstPage() const;
    int lastPage() const;

    void detach();

private:
    bool isEqual(const QPageRanges &other) const noexcept;

    QExplicitlySharedDataPointer<QPageRangesPrivate> d;
};

#ifndef QT_NO_DATASTREAM
Q_GUI_EXPORT QDataStream &operator<<(QDataStream &, const QPageRanges &);
Q_GUI_EXPORT QDataStream &operator>>(QDataStream &, QPageRanges &);
#endif

#ifndef QT_NO_DEBUG_STREAM
Q_GUI_EXPORT QDebug operator<<(QDebug dbg, const QPageRanges &pageRanges);
#endif

Q_DECLARE_SHARED(QPageRanges)
Q_DECLARE_TYPEINFO(QPageRanges::Range, Q_RELOCATABLE_TYPE);

QT_END_NAMESPACE

QT_DECL_METATYPE_EXTERN(QPageRanges, Q_GUI_EXPORT)

#endif // QPAGERANGES_H
