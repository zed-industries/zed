/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtGui module of the Qt Toolkit.
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

#ifndef QTEXTOPTION_H
#define QTEXTOPTION_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/qnamespace.h>
#include <QtCore/qchar.h>
#include <QtCore/qmetatype.h>


QT_BEGIN_NAMESPACE


template <typename T> class QList;
struct QTextOptionPrivate;

class Q_GUI_EXPORT QTextOption
{
public:
    enum TabType {
        LeftTab,
        RightTab,
        CenterTab,
        DelimiterTab
    };

    struct Q_GUI_EXPORT Tab {
        inline Tab() : position(80), type(QTextOption::LeftTab) { }
        inline Tab(qreal pos, TabType tabType, QChar delim = QChar())
            : position(pos), type(tabType), delimiter(delim) {}

        inline bool operator==(const Tab &other) const {
            return type == other.type
                   && qFuzzyCompare(position, other.position)
                   && delimiter == other.delimiter;
        }

        inline bool operator!=(const Tab &other) const {
            return !operator==(other);
        }

        qreal position;
        TabType type;
        QChar delimiter;
    };

    QTextOption();
    /*implicit*/ QTextOption(Qt::Alignment alignment);
    ~QTextOption();

    QTextOption(const QTextOption &o);
    QTextOption &operator=(const QTextOption &o);

    inline void setAlignment(Qt::Alignment alignment);
    inline Qt::Alignment alignment() const { return Qt::Alignment(align); }

    inline void setTextDirection(Qt::LayoutDirection aDirection) { this->direction = aDirection; }
    inline Qt::LayoutDirection textDirection() const { return Qt::LayoutDirection(direction); }

    enum WrapMode {
        NoWrap,
        WordWrap,
        ManualWrap,
        WrapAnywhere,
        WrapAtWordBoundaryOrAnywhere
    };
    inline void setWrapMode(WrapMode wrap) { wordWrap = wrap; }
    inline WrapMode wrapMode() const { return static_cast<WrapMode>(wordWrap); }

    enum Flag {
        ShowTabsAndSpaces = 0x1,
        ShowLineAndParagraphSeparators = 0x2,
        AddSpaceForLineAndParagraphSeparators = 0x4,
        SuppressColors = 0x8,
        ShowDocumentTerminator = 0x10,
        IncludeTrailingSpaces = 0x80000000
    };
    Q_DECLARE_FLAGS(Flags, Flag)
    inline void setFlags(Flags flags);
    inline Flags flags() const { return Flags(f); }

#if QT_DEPRECATED_SINCE(5, 10)
    QT_DEPRECATED inline void setTabStop(qreal tabStop);
    QT_DEPRECATED inline qreal tabStop() const { return tabStopDistance(); }
#endif

    inline void setTabStopDistance(qreal tabStopDistance);
    inline qreal tabStopDistance() const { return tab; }

    void setTabArray(const QList<qreal> &tabStops);
    QList<qreal> tabArray() const;

    void setTabs(const QList<Tab> &tabStops);
    QList<Tab> tabs() const;

    void setUseDesignMetrics(bool b) { design = b; }
    bool useDesignMetrics() const { return design; }

private:
    uint align : 8;
    uint wordWrap : 4;
    uint design : 1;
    uint direction : 2;
    uint unused : 17;
    uint unused2; // ### Qt 6: remove unnecessary, extra 32 bits
    uint f;
    qreal tab;
    QTextOptionPrivate *d;
};

Q_DECLARE_OPERATORS_FOR_FLAGS(QTextOption::Flags)

inline void QTextOption::setAlignment(Qt::Alignment aalignment)
{ align = aalignment; }

inline void QTextOption::setFlags(Flags aflags)
{ f = aflags; }

#if QT_DEPRECATED_SINCE(5, 10)
inline void QTextOption::setTabStop(qreal atabStop)
{ setTabStopDistance(atabStop); }
#endif

inline void QTextOption::setTabStopDistance(qreal atabStop)
{ tab = atabStop; }

QT_END_NAMESPACE

Q_DECLARE_METATYPE( QTextOption::Tab )

#endif // QTEXTOPTION_H
