// Copyright (C) 2018 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

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

QT_DECLARE_QESDP_SPECIALIZATION_DTOR_WITH_EXPORT(QColorSpacePrivate, Q_GUI_EXPORT)

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

    QColorSpace() noexcept = default;
    QColorSpace(NamedColorSpace namedColorSpace);
    QColorSpace(Primaries primaries, TransferFunction transferFunction, float gamma = 0.0f);
    QColorSpace(Primaries primaries, float gamma);
    QColorSpace(Primaries primaries, const QList<uint16_t> &transferFunctionTable);
    QColorSpace(const QPointF &whitePoint, const QPointF &redPoint,
                const QPointF &greenPoint, const QPointF &bluePoint,
                TransferFunction transferFunction, float gamma = 0.0f);
    QColorSpace(const QPointF &whitePoint, const QPointF &redPoint,
                const QPointF &greenPoint, const QPointF &bluePoint,
                const QList<uint16_t> &transferFunctionTable);
    QColorSpace(const QPointF &whitePoint, const QPointF &redPoint,
                const QPointF &greenPoint, const QPointF &bluePoint,
                const QList<uint16_t> &redTransferFunctionTable,
                const QList<uint16_t> &greenTransferFunctionTable,
                const QList<uint16_t> &blueTransferFunctionTable);
    ~QColorSpace();

    QColorSpace(const QColorSpace &colorSpace) noexcept;
    QColorSpace &operator=(const QColorSpace &colorSpace) noexcept
    {
        QColorSpace copy(colorSpace);
        swap(copy);
        return *this;
    }

    QColorSpace(QColorSpace &&colorSpace) noexcept = default;
    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_MOVE_AND_SWAP(QColorSpace)

    void swap(QColorSpace &colorSpace) noexcept
    { d_ptr.swap(colorSpace.d_ptr); }

    Primaries primaries() const noexcept;
    TransferFunction transferFunction() const noexcept;
    float gamma() const noexcept;

    QString description() const noexcept;
    void setDescription(const QString &description);

    void setTransferFunction(TransferFunction transferFunction, float gamma = 0.0f);
    void setTransferFunction(const QList<uint16_t> &transferFunctionTable);
    void setTransferFunctions(const QList<uint16_t> &redTransferFunctionTable,
                              const QList<uint16_t> &greenTransferFunctionTable,
                              const QList<uint16_t> &blueTransferFunctionTable);
    QColorSpace withTransferFunction(TransferFunction transferFunction, float gamma = 0.0f) const;
    QColorSpace withTransferFunction(const QList<uint16_t> &transferFunctionTable) const;
    QColorSpace withTransferFunctions(const QList<uint16_t> &redTransferFunctionTable,
                                      const QList<uint16_t> &greenTransferFunctionTable,
                                      const QList<uint16_t> &blueTransferFunctionTable) const;

    void setPrimaries(Primaries primariesId);
    void setPrimaries(const QPointF &whitePoint, const QPointF &redPoint,
                      const QPointF &greenPoint, const QPointF &bluePoint);

    void detach();
    bool isValid() const noexcept;

    friend inline bool operator==(const QColorSpace &colorSpace1, const QColorSpace &colorSpace2)
    { return colorSpace1.equals(colorSpace2); }
    friend inline bool operator!=(const QColorSpace &colorSpace1, const QColorSpace &colorSpace2)
    { return !(colorSpace1 == colorSpace2); }

    static QColorSpace fromIccProfile(const QByteArray &iccProfile);
    QByteArray iccProfile() const;

    QColorTransform transformationToColorSpace(const QColorSpace &colorspace) const;

    operator QVariant() const;

private:
    friend class QColorSpacePrivate;
    bool equals(const QColorSpace &other) const;

    QExplicitlySharedDataPointer<QColorSpacePrivate> d_ptr;

#ifndef QT_NO_DEBUG_STREAM
    friend Q_GUI_EXPORT QDebug operator<<(QDebug dbg, const QColorSpace &colorSpace);
#endif
};

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
