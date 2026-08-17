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

#ifndef QABSTRACTTEXTDOCUMENTLAYOUT_H
#define QABSTRACTTEXTDOCUMENTLAYOUT_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/qobject.h>
#include <QtGui/qtextlayout.h>
#include <QtGui/qtextdocument.h>
#include <QtGui/qtextcursor.h>
#include <QtGui/qpalette.h>

QT_BEGIN_NAMESPACE


class QAbstractTextDocumentLayoutPrivate;
class QTextBlock;
class QTextObjectInterface;
class QTextFrame;

class Q_GUI_EXPORT QAbstractTextDocumentLayout : public QObject
{
    Q_OBJECT
    Q_DECLARE_PRIVATE(QAbstractTextDocumentLayout)

public:
    explicit QAbstractTextDocumentLayout(QTextDocument *doc);
    ~QAbstractTextDocumentLayout();

    struct Selection
    {
        QTextCursor cursor;
        QTextCharFormat format;
    };

    struct PaintContext
    {
        PaintContext()
            : cursorPosition(-1)
            {}
        int cursorPosition;
        QPalette palette;
        QRectF clip;
        QVector<Selection> selections;
    };

    virtual void draw(QPainter *painter, const PaintContext &context) = 0;
    virtual int hitTest(const QPointF &point, Qt::HitTestAccuracy accuracy) const = 0;

    QString anchorAt(const QPointF& pos) const;
    QString imageAt(const QPointF &pos) const;
    QTextFormat formatAt(const QPointF &pos) const;
    QTextBlock blockWithMarkerAt(const QPointF &pos) const;

    virtual int pageCount() const = 0;
    virtual QSizeF documentSize() const = 0;

    virtual QRectF frameBoundingRect(QTextFrame *frame) const = 0;
    virtual QRectF blockBoundingRect(const QTextBlock &block) const = 0;

    void setPaintDevice(QPaintDevice *device);
    QPaintDevice *paintDevice() const;

    QTextDocument *document() const;

    void registerHandler(int objectType, QObject *component);
    void unregisterHandler(int objectType, QObject *component = nullptr);
    QTextObjectInterface *handlerForObject(int objectType) const;

Q_SIGNALS:
    void update(const QRectF & = QRectF(0., 0., 1000000000., 1000000000.));
    void updateBlock(const QTextBlock &block);
    void documentSizeChanged(const QSizeF &newSize);
    void pageCountChanged(int newPages);

protected:
    QAbstractTextDocumentLayout(QAbstractTextDocumentLayoutPrivate &, QTextDocument *);

    virtual void documentChanged(int from, int charsRemoved, int charsAdded) = 0;

    virtual void resizeInlineObject(QTextInlineObject item, int posInDocument, const QTextFormat &format);
    virtual void positionInlineObject(QTextInlineObject item, int posInDocument, const QTextFormat &format);
    virtual void drawInlineObject(QPainter *painter, const QRectF &rect, QTextInlineObject object, int posInDocument, const QTextFormat &format);

    int formatIndex(int pos);
    QTextCharFormat format(int pos);

private:
    friend class QWidgetTextControl;
    friend class QTextDocument;
    friend class QTextDocumentPrivate;
    friend class QTextEngine;
    friend class QTextLayout;
    friend class QTextLine;
    Q_PRIVATE_SLOT(d_func(), void _q_handlerDestroyed(QObject *obj))
    Q_PRIVATE_SLOT(d_func(), int _q_dynamicPageCountSlot())
    Q_PRIVATE_SLOT(d_func(), QSizeF _q_dynamicDocumentSizeSlot())
};
Q_DECLARE_TYPEINFO(QAbstractTextDocumentLayout::Selection,    Q_RELOCATABLE_TYPE);
Q_DECLARE_TYPEINFO(QAbstractTextDocumentLayout::PaintContext, Q_RELOCATABLE_TYPE);

class Q_GUI_EXPORT QTextObjectInterface
{
public:
    virtual ~QTextObjectInterface();
    virtual QSizeF intrinsicSize(QTextDocument *doc, int posInDocument, const QTextFormat &format) = 0;
    virtual void drawObject(QPainter *painter, const QRectF &rect, QTextDocument *doc, int posInDocument, const QTextFormat &format) = 0;
};

#ifndef Q_CLANG_QDOC
Q_DECLARE_INTERFACE(QTextObjectInterface, "org.qt-project.Qt.QTextObjectInterface")
#endif

QT_END_NAMESPACE

#endif // QABSTRACTTEXTDOCUMENTLAYOUT_H
