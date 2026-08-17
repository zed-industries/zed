/****************************************************************************
**
** Copyright (C) 2018 The Qt Company Ltd.
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

#ifndef QCOLORSPACE_H
#define QCOLORSPACE_H

#include <QtGui/qtguiglobal.h>
#include <QtGui/qcolortransform.h>
#include <QtCore/qobjectdefs.h>
#include <QtCore/qshareddata.h>
#include <QtCore/qvariant.h>

QT_BEGIN_NAMESPACE

class QColorSpacePrivate;
class QPointF;

class Q_GUI_EXPORT QColorSpace
{
    Q_GADGET
public:
    enum NamedColorSpace {
        SRgb = 1,
        SRgbLinear,
        AdobeRgb,
        DisplayP3,
        ProPhotoRgb
    };
    Q_ENUM(NamedColorSpace)
    enum class Primaries {
        Custom = 0,
        SRgb,
        AdobeRgb,
        DciP3D65,
        ProPhotoRgb
    };
    Q_ENUM(Primaries)
    enum class TransferFunction {
        Custom = 0,
        Linear,
        Gamma,
        SRgb,
        ProPhotoRgb
    };
    Q_ENUM(TransferFunction)

    QColorSpace();
    QColorSpace(NamedColorSpace namedColorSpace);
    QColorSpace(Primaries primaries, TransferFunction fun, float gamma = 0.0f);
    QColorSpace(Primaries primaries, float gamma);
    QColorSpace(const QPointF &whitePoint, const QPointF &redPoint,
                const QPointF &greenPoint, const QPointF &bluePoint,
                TransferFunction fun, float gamma = 0.0f);
    ~QColorSpace();

    QColorSpace(const QColorSpace &colorSpace);
    QColorSpace &operator=(const QColorSpace &colorSpace);

    QColorSpace(QColorSpace &&colorSpace) noexcept
            : d_ptr(qExchange(colorSpace.d_ptr, nullptr))
    { }
    QColorSpace &operator=(QColorSpace &&colorSpace) noexcept
    {
        // Make the deallocation of this->d_ptr happen in ~QColorSpace()
        QColorSpace(std::move(colorSpace)).swap(*this);
        return *this;
    }

    void swap(QColorSpace &colorSpace) noexcept
    { qSwap(d_ptr, colorSpace.d_ptr); }

    Primaries primaries() const noexcept;
    TransferFunction transferFunction() const noexcept;
    float gamma() const noexcept;

    void setTransferFunction(TransferFunction transferFunction, float gamma = 0.0f);
    QColorSpace withTransferFunction(TransferFunction transferFunction, float gamma = 0.0f) const;

    void setPrimaries(Primaries primariesId);
    void setPrimaries(const QPointF &whitePoint, const QPointF &redPoint,
                      const QPointF &greenPoint, const QPointF &bluePoint);

    bool isValid() const noexcept;

    friend Q_GUI_EXPORT bool operator==(const QColorSpace &colorSpace1, const QColorSpace &colorSpace2);
    friend inline bool operator!=(const QColorSpace &colorSpace1, const QColorSpace &colorSpace2);

    static QColorSpace fromIccProfile(const QByteArray &iccProfile);
    QByteArray iccProfile() const;

    QColorTransform transformationToColorSpace(const QColorSpace &colorspace) const;

    operator QVariant() const;

private:
    Q_DECLARE_PRIVATE(QColorSpace)
    QColorSpacePrivate *d_ptr = nullptr;

#ifndef QT_NO_DEBUG_STREAM
    friend Q_GUI_EXPORT QDebug operator<<(QDebug dbg, const QColorSpace &colorSpace);
#endif
};

bool Q_GUI_EXPORT operator==(const QColorSpace &colorSpace1, const QColorSpace &colorSpace2);
inline bool operator!=(const QColorSpace &colorSpace1, const QColorSpace &colorSpace2)
{
    return !(colorSpace1 == colorSpace2);
}

Q_DECLARE_SHARED(QColorSpace)

// QColorSpace stream functions
#if !defined(QT_NO_DATASTREAM)
Q_GUI_EXPORT QDataStream &operator<<(QDataStream &, const QColorSpace &);
Q_GUI_EXPORT QDataStream &operator>>(QDataStream &, QColorSpace &);
#endif

#ifndef QT_NO_DEBUG_STREAM
Q_GUI_EXPORT QDebug operator<<(QDebug, const QColorSpace &);
#endif

QT_END_NAMESPACE

#endif // QCOLORSPACE_P_H
