/****************************************************************************
**
** Copyright (C) 2019 The Qt Company Ltd.
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

#ifndef QNUMERIC_H
#define QNUMERIC_H

#include <QtCore/qglobal.h>

QT_BEGIN_NAMESPACE

Q_CORE_EXPORT Q_DECL_CONST_FUNCTION bool qIsInf(double d);
Q_CORE_EXPORT Q_DECL_CONST_FUNCTION bool qIsNaN(double d);
Q_CORE_EXPORT Q_DECL_CONST_FUNCTION bool qIsFinite(double d);
Q_CORE_EXPORT Q_DECL_CONST_FUNCTION int qFpClassify(double val);
Q_CORE_EXPORT Q_DECL_CONST_FUNCTION bool qIsInf(float f);
Q_CORE_EXPORT Q_DECL_CONST_FUNCTION bool qIsNaN(float f);
Q_CORE_EXPORT Q_DECL_CONST_FUNCTION bool qIsFinite(float f);
Q_CORE_EXPORT Q_DECL_CONST_FUNCTION int qFpClassify(float val);
#if QT_CONFIG(signaling_nan)
Q_CORE_EXPORT Q_DECL_CONST_FUNCTION double qSNaN();
#endif
Q_CORE_EXPORT Q_DECL_CONST_FUNCTION double qQNaN();
Q_CORE_EXPORT Q_DECL_CONST_FUNCTION double qInf();

Q_CORE_EXPORT quint32 qFloatDistance(float a, float b);
Q_CORE_EXPORT quint64 qFloatDistance(double a, double b);

#define Q_INFINITY (QT_PREPEND_NAMESPACE(qInf)())
#if QT_CONFIG(signaling_nan)
#  define Q_SNAN (QT_PREPEND_NAMESPACE(qSNaN)())
#endif
#define Q_QNAN (QT_PREPEND_NAMESPACE(qQNaN)())

QT_END_NAMESPACE

#endif // QNUMERIC_H
