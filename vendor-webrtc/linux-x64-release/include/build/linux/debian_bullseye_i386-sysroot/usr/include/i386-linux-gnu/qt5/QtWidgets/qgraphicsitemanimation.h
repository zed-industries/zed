/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtWidgets module of the Qt Toolkit.
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

#ifndef QGRAPHICSITEMANIMATION_H
#define QGRAPHICSITEMANIMATION_H

#include <QtWidgets/qtwidgetsglobal.h>
#include <QtCore/qobject.h>

QT_REQUIRE_CONFIG(graphicsview);

QT_BEGIN_NAMESPACE

class QGraphicsItem;
class QMatrix;
class QPointF;
class QTimeLine;
class QTransform;
template <class T1, class T2> struct QPair;

class QGraphicsItemAnimationPrivate;
class Q_WIDGETS_EXPORT QGraphicsItemAnimation : public QObject
{
    Q_OBJECT
public:
    QGraphicsItemAnimation(QObject *parent = nullptr);
    virtual ~QGraphicsItemAnimation();

    QGraphicsItem *item() const;
    void setItem(QGraphicsItem *item);

    QTimeLine *timeLine() const;
    void setTimeLine(QTimeLine *timeLine);

    QPointF posAt(qreal step) const;
    QList<QPair<qreal, QPointF> > posList() const;
    void setPosAt(qreal step, const QPointF &pos);

#if QT_DEPRECATED_SINCE(5, 14)
    QT_DEPRECATED_X("Use transformAt() instead")
    QMatrix matrixAt(qreal step) const;
#endif
    QTransform transformAt(qreal step) const;

    qreal rotationAt(qreal step) const;
    QList<QPair<qreal, qreal> > rotationList() const;
    void setRotationAt(qreal step, qreal angle);

    qreal xTranslationAt(qreal step) const;
    qreal yTranslationAt(qreal step) const;
    QList<QPair<qreal, QPointF> > translationList() const;
    void setTranslationAt(qreal step, qreal dx, qreal dy);

    qreal verticalScaleAt(qreal step) const;
    qreal horizontalScaleAt(qreal step) const;
    QList<QPair<qreal, QPointF> > scaleList() const;
    void setScaleAt(qreal step, qreal sx, qreal sy);

    qreal verticalShearAt(qreal step) const;
    qreal horizontalShearAt(qreal step) const;
    QList<QPair<qreal, QPointF> > shearList() const;
    void setShearAt(qreal step, qreal sh, qreal sv);

    void clear();

public Q_SLOTS:
    void setStep(qreal x);
#if QT_DEPRECATED_SINCE(5, 13)
    QT_DEPRECATED_X("Use setStep(0) instead")
    void reset();
#endif

protected:
    virtual void beforeAnimationStep(qreal step);
    virtual void afterAnimationStep(qreal step);

private:
    Q_DISABLE_COPY(QGraphicsItemAnimation)
    QGraphicsItemAnimationPrivate *d;
};

QT_END_NAMESPACE

#endif
