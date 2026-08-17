// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QTEXTLIST_H
#define QTEXTLIST_H

#include <QtGui/qtguiglobal.h>
#include <QtGui/qtextobject.h>
#include <QtCore/qobject.h>

QT_BEGIN_NAMESPACE


class QTextListPrivate;
class QTextCursor;

class Q_GUI_EXPORT QTextList : public QTextBlockGroup
{
    Q_OBJECT
public:
    explicit QTextList(QTextDocument *doc);
    ~QTextList();

    int count() const;

    QTextBlock item(int i) const;

    int itemNumber(const QTextBlock &) const;
    QString itemText(const QTextBlock &) const;

    void removeItem(int i);
    void remove(const QTextBlock &);

    void add(const QTextBlock &block);

    inline void setFormat(const QTextListFormat &format);
    QTextListFormat format() const { return QTextObject::format().toListFormat(); }

private:
    Q_DISABLE_COPY(QTextList)
    Q_DECLARE_PRIVATE(QTextList)
};

inline void QTextList::setFormat(const QTextListFormat &aformat)
{ QTextObject::setFormat(aformat); }

QT_END_NAMESPACE

#endif // QTEXTLIST_H
