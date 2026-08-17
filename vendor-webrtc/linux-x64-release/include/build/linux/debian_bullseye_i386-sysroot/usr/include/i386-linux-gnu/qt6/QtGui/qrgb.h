// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QRGB_H
#define QRGB_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/qprocessordetection.h>

QT_BEGIN_NAMESPACE


typedef unsigned int QRgb;                        // RGB triplet

// non-namespaced Qt global variable
 inline constexpr QRgb RGB_MASK = 0x00ffffff;     // masks RGB values

inline constexpr int qRed(QRgb rgb)                     // get red part of RGB
{ return ((rgb >> 16) & 0xff); }

inline constexpr int qGreen(QRgb rgb)                   // get green part of RGB
{ return ((rgb >> 8) & 0xff); }

inline constexpr int qBlue(QRgb rgb)                    // get blue part of RGB
{ return (rgb & 0xff); }

inline constexpr int qAlpha(QRgb rgb)                   // get alpha part of RGBA
{ return rgb >> 24; }

inline constexpr QRgb qRgb(int r, int g, int b)         // set RGB value
{ return (0xffu << 24) | ((r & 0xffu) << 16) | ((g & 0xffu) << 8) | (b & 0xffu); }

inline constexpr QRgb qRgba(int r, int g, int b, int a) // set RGBA value
{ return ((a & 0xffu) << 24) | ((r & 0xffu) << 16) | ((g & 0xffu) << 8) | (b & 0xffu); }

inline constexpr int qGray(int r, int g, int b)         // convert R,G,B to gray 0..255
{ return (r*11+g*16+b*5)/32; }

inline constexpr int qGray(QRgb rgb)                    // convert RGB to gray 0..255
{ return qGray(qRed(rgb), qGreen(rgb), qBlue(rgb)); }

inline constexpr bool qIsGray(QRgb rgb)
{ return qRed(rgb) == qGreen(rgb) && qRed(rgb) == qBlue(rgb); }

inline constexpr QRgb qPremultiply(QRgb x)
{
    const uint a = qAlpha(x);
    uint t = (x & 0xff00ff) * a;
    t = (t + ((t >> 8) & 0xff00ff) + 0x800080) >> 8;
    t &= 0xff00ff;

    x = ((x >> 8) & 0xff) * a;
    x = (x + ((x >> 8) & 0xff) + 0x80);
    x &= 0xff00;
    return x | t | (a << 24);
}

Q_GUI_EXPORT extern const uint qt_inv_premul_factor[];

inline QRgb qUnpremultiply(QRgb p)
{
    const uint alpha = qAlpha(p);
    // Alpha 255 and 0 are the two most common values, which makes them beneficial to short-cut.
    if (alpha == 255)
        return p;
    if (alpha == 0)
        return 0;
    // (p*(0x00ff00ff/alpha)) >> 16 == (p*255)/alpha for all p and alpha <= 256.
    const uint invAlpha = qt_inv_premul_factor[alpha];
    // We add 0x8000 to get even rounding. The rounding also ensures that qPremultiply(qUnpremultiply(p)) == p for all p.
    return qRgba((qRed(p)*invAlpha + 0x8000)>>16, (qGreen(p)*invAlpha + 0x8000)>>16, (qBlue(p)*invAlpha + 0x8000)>>16, alpha);
}

QT_END_NAMESPACE

#endif // QRGB_H
