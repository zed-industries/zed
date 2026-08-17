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

#ifndef QRGBA64_H
#define QRGBA64_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/qprocessordetection.h>

QT_BEGIN_NAMESPACE

class QRgba64 {
    quint64 rgba;

    // Make sure that the representation always has the order: red green blue alpha, independent
    // of byte order. This way, vector operations that assume 4 16-bit values see the correct ones.
    enum Shifts {
#if Q_BYTE_ORDER == Q_BIG_ENDIAN
        RedShift = 48,
        GreenShift = 32,
        BlueShift = 16,
        AlphaShift = 0
#else // little endian:
        RedShift = 0,
        GreenShift = 16,
        BlueShift = 32,
        AlphaShift = 48
#endif
    };

    explicit Q_ALWAYS_INLINE Q_DECL_CONSTEXPR QRgba64(quint64 c) : rgba(c) { }
public:
    QRgba64() = default;

    Q_DECL_CONSTEXPR static
    QRgba64 fromRgba64(quint64 c)
    {
        return QRgba64(c);
    }
    Q_DECL_CONSTEXPR static
    QRgba64 fromRgba64(quint16 red, quint16 green, quint16 blue, quint16 alpha)
    {
        return fromRgba64(quint64(red)   << RedShift
                        | quint64(green) << GreenShift
                        | quint64(blue)  << BlueShift
                        | quint64(alpha) << AlphaShift);
    }
    Q_DECL_RELAXED_CONSTEXPR static QRgba64 fromRgba(quint8 red, quint8 green, quint8 blue, quint8 alpha)
    {
        QRgba64 rgb64 = fromRgba64(red, green, blue, alpha);
        // Expand the range so that 0x00 maps to 0x0000 and 0xff maps to 0xffff.
        rgb64.rgba |= rgb64.rgba << 8;
        return rgb64;
    }
    Q_DECL_RELAXED_CONSTEXPR static
    QRgba64 fromArgb32(uint rgb)
    {
        return fromRgba(quint8(rgb >> 16), quint8(rgb >> 8), quint8(rgb), quint8(rgb >> 24));
    }

    Q_DECL_CONSTEXPR bool isOpaque() const
    {
        return (rgba & alphaMask()) == alphaMask();
    }
    Q_DECL_CONSTEXPR bool isTransparent() const
    {
        return (rgba & alphaMask()) == 0;
    }

    Q_DECL_CONSTEXPR quint16 red()   const { return quint16(rgba >> RedShift);   }
    Q_DECL_CONSTEXPR quint16 green() const { return quint16(rgba >> GreenShift); }
    Q_DECL_CONSTEXPR quint16 blue()  const { return quint16(rgba >> BlueShift);  }
    Q_DECL_CONSTEXPR quint16 alpha() const { return quint16(rgba >> AlphaShift); }
    void setRed(quint16 _red)     { rgba = (rgba & ~(Q_UINT64_C(0xffff) << RedShift))   | (quint64(_red) << RedShift); }
    void setGreen(quint16 _green) { rgba = (rgba & ~(Q_UINT64_C(0xffff) << GreenShift)) | (quint64(_green) << GreenShift); }
    void setBlue(quint16 _blue)   { rgba = (rgba & ~(Q_UINT64_C(0xffff) << BlueShift))  | (quint64(_blue) << BlueShift); }
    void setAlpha(quint16 _alpha) { rgba = (rgba & ~(Q_UINT64_C(0xffff) << AlphaShift)) | (quint64(_alpha) << AlphaShift); }

    Q_DECL_CONSTEXPR quint8 red8()   const { return div_257(red()); }
    Q_DECL_CONSTEXPR quint8 green8() const { return div_257(green()); }
    Q_DECL_CONSTEXPR quint8 blue8()  const { return div_257(blue()); }
    Q_DECL_CONSTEXPR quint8 alpha8() const { return div_257(alpha()); }
    Q_DECL_CONSTEXPR uint toArgb32() const
    {
#if defined(__cpp_constexpr) && __cpp_constexpr-0 >= 201304
        quint64 br = rgba & Q_UINT64_C(0xffff0000ffff);
        quint64 ag = (rgba >> 16) & Q_UINT64_C(0xffff0000ffff);
        br += Q_UINT64_C(0x8000000080);
        ag += Q_UINT64_C(0x8000000080);
        br = (br - ((br >> 8) & Q_UINT64_C(0xffff0000ffff))) >> 8;
        ag = (ag - ((ag >> 8) & Q_UINT64_C(0xffff0000ffff)));
#if Q_BYTE_ORDER == Q_BIG_ENDIAN
        return ((br << 24) & 0xff000000)
             | ((ag >> 24) & 0xff0000)
             | ((br >> 24) & 0xff00)
             | ((ag >> 8)  & 0xff);
#else
        return ((ag >> 16) & 0xff000000)
             | ((br << 16) & 0xff0000)
             | (ag         & 0xff00)
             | ((br >> 32) & 0xff);
#endif
#else
        return uint((alpha8() << 24) | (red8() << 16) | (green8() << 8) | blue8());
#endif
    }
    Q_DECL_CONSTEXPR ushort toRgb16() const
    {
        return ushort((red() & 0xf800) | ((green() >> 10) << 5) | (blue() >> 11));
    }

    Q_DECL_RELAXED_CONSTEXPR QRgba64 premultiplied() const
    {
        if (isOpaque())
            return *this;
        if (isTransparent())
            return QRgba64::fromRgba64(0);
        const quint64 a = alpha();
        quint64 br = (rgba & Q_UINT64_C(0xffff0000ffff)) * a;
        quint64 ag = ((rgba >> 16) & Q_UINT64_C(0xffff0000ffff)) * a;
        br = (br + ((br >> 16) & Q_UINT64_C(0xffff0000ffff)) + Q_UINT64_C(0x800000008000));
        ag = (ag + ((ag >> 16) & Q_UINT64_C(0xffff0000ffff)) + Q_UINT64_C(0x800000008000));
#if Q_BYTE_ORDER == Q_BIG_ENDIAN
        ag = ag & Q_UINT64_C(0xffff0000ffff0000);
        br = (br >> 16) & Q_UINT64_C(0xffff00000000);
        return fromRgba64(a | br | ag);
#else
        br = (br >> 16) & Q_UINT64_C(0xffff0000ffff);
        ag = ag & Q_UINT64_C(0xffff0000);
        return fromRgba64((a << 48) | br | ag);
#endif
    }

    Q_DECL_RELAXED_CONSTEXPR QRgba64 unpremultiplied() const
    {
#if Q_PROCESSOR_WORDSIZE < 8
        return unpremultiplied_32bit();
#else
        return unpremultiplied_64bit();
#endif
    }

    Q_DECL_CONSTEXPR operator quint64() const
    {
        return rgba;
    }

    QRgba64 operator=(quint64 _rgba)
    {
        rgba = _rgba;
        return *this;
    }

private:
    static Q_DECL_CONSTEXPR Q_ALWAYS_INLINE quint64 alphaMask() { return Q_UINT64_C(0xffff) << AlphaShift; }

    static Q_DECL_CONSTEXPR Q_ALWAYS_INLINE quint8 div_257_floor(uint x) { return quint8((x - (x >> 8)) >> 8); }
    static Q_DECL_CONSTEXPR Q_ALWAYS_INLINE quint8 div_257(quint16 x) { return div_257_floor(x + 128U); }
    Q_DECL_RELAXED_CONSTEXPR Q_ALWAYS_INLINE QRgba64 unpremultiplied_32bit() const
    {
        if (isOpaque() || isTransparent())
            return *this;
        const quint32 a = alpha();
        const quint16 r = quint16((red()   * 0xffff + a/2) / a);
        const quint16 g = quint16((green() * 0xffff + a/2) / a);
        const quint16 b = quint16((blue()  * 0xffff + a/2) / a);
        return fromRgba64(r, g, b, quint16(a));
    }
    Q_DECL_RELAXED_CONSTEXPR Q_ALWAYS_INLINE QRgba64 unpremultiplied_64bit() const
    {
        if (isOpaque() || isTransparent())
            return *this;
        const quint64 a = alpha();
        const quint64 fa = (Q_UINT64_C(0xffff00008000) + a/2) / a;
        const quint16 r = quint16((red()   * fa + 0x80000000) >> 32);
        const quint16 g = quint16((green() * fa + 0x80000000) >> 32);
        const quint16 b = quint16((blue()  * fa + 0x80000000) >> 32);
        return fromRgba64(r, g, b, quint16(a));
    }
};

Q_DECLARE_TYPEINFO(QRgba64, Q_PRIMITIVE_TYPE);

Q_DECL_CONSTEXPR inline QRgba64 qRgba64(quint16 r, quint16 g, quint16 b, quint16 a)
{
    return QRgba64::fromRgba64(r, g, b, a);
}

Q_DECL_CONSTEXPR inline QRgba64 qRgba64(quint64 c)
{
    return QRgba64::fromRgba64(c);
}

Q_DECL_RELAXED_CONSTEXPR inline QRgba64 qPremultiply(QRgba64 c)
{
    return c.premultiplied();
}

Q_DECL_RELAXED_CONSTEXPR inline QRgba64 qUnpremultiply(QRgba64 c)
{
    return c.unpremultiplied();
}

inline Q_DECL_CONSTEXPR uint qRed(QRgba64 rgb)
{ return rgb.red8(); }

inline Q_DECL_CONSTEXPR uint qGreen(QRgba64 rgb)
{ return rgb.green8(); }

inline Q_DECL_CONSTEXPR uint qBlue(QRgba64 rgb)
{ return rgb.blue8(); }

inline Q_DECL_CONSTEXPR uint qAlpha(QRgba64 rgb)
{ return rgb.alpha8(); }

QT_END_NAMESPACE

#endif // QRGBA64_H
