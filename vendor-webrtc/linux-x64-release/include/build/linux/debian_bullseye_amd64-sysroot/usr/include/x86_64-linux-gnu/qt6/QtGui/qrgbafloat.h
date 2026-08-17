// Copyright (C) 2021 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QRGBAFLOAT_H
#define QRGBAFLOAT_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/qfloat16.h>

#include <algorithm>
#include <cmath>
#include <type_traits>

QT_BEGIN_NAMESPACE

template<typename F>
class alignas(sizeof(F) * 4) QRgbaFloat
{
    static_assert(std::is_same<F, qfloat16>::value || std::is_same<F, float>::value);
public:
    using Type = F;
    using FastType = float;
    F r;
    F g;
    F b;
    F a;

    static constexpr
    QRgbaFloat fromRgba64(quint16 red, quint16 green, quint16 blue, quint16 alpha)
    {
        return QRgbaFloat{
            red   * (1.0f / 65535.0f),
            green * (1.0f / 65535.0f),
            blue  * (1.0f / 65535.0f),
            alpha * (1.0f / 65535.0f) };
    }

    static constexpr
    QRgbaFloat fromRgba(quint8 red, quint8 green, quint8 blue, quint8 alpha)
    {
        return QRgbaFloat{
            red   * (1.0f / 255.0f),
            green * (1.0f / 255.0f),
            blue  * (1.0f / 255.0f),
            alpha * (1.0f / 255.0f) };
    }
    static constexpr
    QRgbaFloat fromArgb32(uint rgb)
    {
        return fromRgba(quint8(rgb >> 16), quint8(rgb >> 8), quint8(rgb), quint8(rgb >> 24));
    }

    constexpr bool isOpaque() const { return a >= 1.0f; }
    constexpr bool isTransparent() const { return a <= 0.0f; }

    constexpr FastType red()   const { return r; }
    constexpr FastType green() const { return g; }
    constexpr FastType blue()  const { return b; }
    constexpr FastType alpha() const { return a; }
    void setRed(FastType _red)     { r = _red; }
    void setGreen(FastType _green) { g = _green; }
    void setBlue(FastType _blue)   { b = _blue; }
    void setAlpha(FastType _alpha) { a = _alpha; }

    constexpr FastType redNormalized()   const { return std::clamp(static_cast<FastType>(r), 0.0f, 1.0f); }
    constexpr FastType greenNormalized() const { return std::clamp(static_cast<FastType>(g), 0.0f, 1.0f); }
    constexpr FastType blueNormalized()  const { return std::clamp(static_cast<FastType>(b), 0.0f, 1.0f); }
    constexpr FastType alphaNormalized() const { return std::clamp(static_cast<FastType>(a), 0.0f, 1.0f); }

    constexpr quint8 red8()   const { return std::lround(redNormalized()   * 255.0f); }
    constexpr quint8 green8() const { return std::lround(greenNormalized() * 255.0f); }
    constexpr quint8 blue8()  const { return std::lround(blueNormalized()  * 255.0f); }
    constexpr quint8 alpha8() const { return std::lround(alphaNormalized() * 255.0f); }
    constexpr uint toArgb32() const
    {
       return uint((alpha8() << 24) | (red8() << 16) | (green8() << 8) | blue8());
    }

    constexpr quint16 red16()   const { return std::lround(redNormalized()   * 65535.0f); }
    constexpr quint16 green16() const { return std::lround(greenNormalized() * 65535.0f); }
    constexpr quint16 blue16()  const { return std::lround(blueNormalized()  * 65535.0f); }
    constexpr quint16 alpha16() const { return std::lround(alphaNormalized() * 65535.0f); }

    constexpr Q_ALWAYS_INLINE QRgbaFloat premultiplied() const
    {
        return QRgbaFloat{r * a, g * a, b * a, a};
    }
    constexpr Q_ALWAYS_INLINE QRgbaFloat unpremultiplied() const
    {
        if (a <= 0.0f)
            return QRgbaFloat{0.0f, 0.0f, 0.0f, 0.0f};
        if (a >= 1.0f)
            return *this;
        const FastType ia = 1.0f / a;
        return QRgbaFloat{r * ia, g * ia, b * ia, a};
    }
    constexpr bool operator==(QRgbaFloat f) const
    {
        return r == f.r && g == f.g && b == f.b && a == f.a;
    }
    constexpr bool operator!=(QRgbaFloat f) const
    {
        return !(*this == f);
    }
};

typedef QRgbaFloat<qfloat16> QRgbaFloat16;
typedef QRgbaFloat<float> QRgbaFloat32;

QT_END_NAMESPACE

#endif // QRGBAFLOAT_H
