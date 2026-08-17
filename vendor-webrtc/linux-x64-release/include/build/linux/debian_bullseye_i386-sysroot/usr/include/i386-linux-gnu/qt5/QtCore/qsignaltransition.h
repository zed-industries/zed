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

#ifndef QSIGNALTRANSITION_H
#define QSIGNALTRANSITION_H

#include <QtCore/qabstracttransition.h>
#include <QtCore/qmetaobject.h>

QT_REQUIRE_CONFIG(statemachine);

QT_BEGIN_NAMESPACE

class QSignalTransitionPrivate;
class Q_CORE_EXPORT QSignalTransition : public QAbstractTransition
{
    Q_OBJECT
    Q_PROPERTY(QObject* senderObject READ senderObject WRITE setSenderObject NOTIFY senderObjectChanged)
    Q_PROPERTY(QByteArray signal READ signal WRITE setSignal NOTIFY signalChanged)

public:
    QSignalTransition(QState *sourceState = nullptr);
    QSignalTransition(const QObject *sender, const char *signal,
                      QState *sourceState = nullptr);
#ifdef Q_QDOC
    template<typename PointerToMemberFunction>
    QSignalTransition(const QObject *object, PointerToMemberFunction signal,
                      QState *sourceState = nullptr);
#elif defined(Q_COMPILER_DELEGATING_CONSTRUCTORS)
    template <typename Func>
    QSignalTransition(const typename QtPrivate::FunctionPointer<Func>::Object *obj,
                      Func sig, QState *srcState = nullptr)
    : QSignalTransition(obj, QMetaMethod::fromSignal(sig).methodSignature().constData(), srcState)
    {
    }
#endif

    ~QSignalTransition();

    QObject *senderObject() const;
    void setSenderObject(const QObject *sender);

    QByteArray signal() const;
    void setSignal(const QByteArray &signal);

protected:
    bool eventTest(QEvent *event) override;
    void onTransition(QEvent *event) override;

    bool event(QEvent *e) override;

Q_SIGNALS:
    void senderObjectChanged(QPrivateSignal);
    void signalChanged(QPrivateSignal);

private:
    Q_DISABLE_COPY(QSignalTransition)
    Q_DECLARE_PRIVATE(QSignalTransition)
};

QT_END_NAMESPACE

#endif
