// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QSIZEPOLICY_H
#define QSIZEPOLICY_H

#include <QtWidgets/qtwidgetsglobal.h>
#include <QtCore/qobject.h>
#include <QtCore/qalgorithms.h>
#include <QtCore/qhashfunctions.h>

QT_BEGIN_NAMESPACE

class QVariant;
class QSizePolicy;

class Q_WIDGETS_EXPORT QSizePolicy
{
    Q_GADGET

public:
    enum PolicyFlag {
        GrowFlag = 1,
        ExpandFlag = 2,
        ShrinkFlag = 4,
        IgnoreFlag = 8
    };

    enum Policy {
        Fixed = 0,
        Minimum = GrowFlag,
        Maximum = ShrinkFlag,
        Preferred = GrowFlag | ShrinkFlag,
        MinimumExpanding = GrowFlag | ExpandFlag,
        Expanding = GrowFlag | ShrinkFlag | ExpandFlag,
        Ignored = ShrinkFlag | GrowFlag | IgnoreFlag
    };
    Q_ENUM(Policy)

    enum ControlType {
        DefaultType      = 0x00000001,
        ButtonBox        = 0x00000002,
        CheckBox         = 0x00000004,
        ComboBox         = 0x00000008,
        Frame            = 0x00000010,
        GroupBox         = 0x00000020,
        Label            = 0x00000040,
        Line             = 0x00000080,
        LineEdit         = 0x00000100,
        PushButton       = 0x00000200,
        RadioButton      = 0x00000400,
        Slider           = 0x00000800,
        SpinBox          = 0x00001000,
        TabWidget        = 0x00002000,
        ToolButton       = 0x00004000
    };
    Q_DECLARE_FLAGS(ControlTypes, ControlType)
    Q_FLAG(ControlTypes)

    constexpr QSizePolicy() noexcept : data(0) { }

    constexpr QSizePolicy(Policy horizontal, Policy vertical, ControlType type = DefaultType) noexcept
        : bits{0, 0, quint32(horizontal), quint32(vertical),
               type == DefaultType ? 0 : toControlTypeFieldValue(type), 0, 0, 0}
    {}
    constexpr Policy horizontalPolicy() const noexcept { return static_cast<Policy>(bits.horPolicy); }
    constexpr Policy verticalPolicy() const noexcept { return static_cast<Policy>(bits.verPolicy); }
    ControlType controlType() const noexcept;

    constexpr void setHorizontalPolicy(Policy d) noexcept { bits.horPolicy = d; }
    constexpr void setVerticalPolicy(Policy d) noexcept { bits.verPolicy = d; }
    void setControlType(ControlType type) noexcept;

    // ### Qt 7: consider making Policy a QFlags and removing these casts
    constexpr Qt::Orientations expandingDirections() const noexcept {
        return ( (verticalPolicy()   & static_cast<Policy>(ExpandFlag)) ? Qt::Vertical   : Qt::Orientations() )
             | ( (horizontalPolicy() & static_cast<Policy>(ExpandFlag)) ? Qt::Horizontal : Qt::Orientations() ) ;
    }

    constexpr void setHeightForWidth(bool b) noexcept { bits.hfw = b;  }
    constexpr bool hasHeightForWidth() const noexcept { return bits.hfw; }
    constexpr void setWidthForHeight(bool b) noexcept { bits.wfh = b;  }
    constexpr bool hasWidthForHeight() const noexcept { return bits.wfh; }

    constexpr bool operator==(const QSizePolicy& s) const noexcept { return data == s.data; }
    constexpr bool operator!=(const QSizePolicy& s) const noexcept { return data != s.data; }

    friend Q_DECL_CONST_FUNCTION size_t qHash(QSizePolicy key, size_t seed = 0) noexcept { return qHash(key.data, seed); }

    operator QVariant() const;

    constexpr int horizontalStretch() const noexcept { return static_cast<int>(bits.horStretch); }
    constexpr int verticalStretch() const noexcept { return static_cast<int>(bits.verStretch); }
    constexpr void setHorizontalStretch(int stretchFactor) { bits.horStretch = static_cast<quint32>(qBound(0, stretchFactor, 255)); }
    constexpr void setVerticalStretch(int stretchFactor) { bits.verStretch = static_cast<quint32>(qBound(0, stretchFactor, 255)); }

    constexpr bool retainSizeWhenHidden() const noexcept { return bits.retainSizeWhenHidden; }
    constexpr void setRetainSizeWhenHidden(bool retainSize) noexcept { bits.retainSizeWhenHidden = retainSize; }

    constexpr void transpose() noexcept { *this = transposed(); }
    [[nodiscard]] constexpr QSizePolicy transposed() const noexcept
    {
        return QSizePolicy(bits.transposed());
    }

private:
#ifndef QT_NO_DATASTREAM
    friend Q_WIDGETS_EXPORT QDataStream &operator<<(QDataStream &, const QSizePolicy &);
    friend Q_WIDGETS_EXPORT QDataStream &operator>>(QDataStream &, QSizePolicy &);
#endif
    constexpr QSizePolicy(int i) noexcept : data(i) { }
    struct Bits;
    constexpr explicit QSizePolicy(Bits b) noexcept : bits(b) { }

    static constexpr quint32 toControlTypeFieldValue(ControlType type) noexcept
    {
        /*
          The control type is a flag type, with values 0x1, 0x2, 0x4, 0x8, 0x10,
          etc. In memory, we pack it onto the available bits (CTSize) in
          setControlType(), and unpack it here.

          Example:

          0x00000001 maps to 0
          0x00000002 maps to 1
          0x00000004 maps to 2
          0x00000008 maps to 3
          etc.
        */

        return qCountTrailingZeroBits(static_cast<quint32>(type));
    }

    struct Bits {
        quint32 horStretch : 8;
        quint32 verStretch : 8;
        quint32 horPolicy : 4;
        quint32 verPolicy : 4;
        quint32 ctype : 5;
        quint32 hfw : 1;
        quint32 wfh : 1;
        quint32 retainSizeWhenHidden : 1;

        constexpr Bits transposed() const noexcept
        {
            return {verStretch, // \ swap
                    horStretch, // /
                    verPolicy, // \ swap
                    horPolicy, // /
                    ctype,
                    hfw, // \ don't swap (historic behavior)
                    wfh, // /
                    retainSizeWhenHidden};
        }
    };
    union {
        Bits bits;
        quint32 data;
    };
};

Q_DECLARE_TYPEINFO(QSizePolicy, Q_PRIMITIVE_TYPE);

Q_DECLARE_OPERATORS_FOR_FLAGS(QSizePolicy::ControlTypes)
Q_DECLARE_MIXED_ENUM_OPERATORS(int, QSizePolicy::Policy, QSizePolicy::PolicyFlag)

#ifndef QT_NO_DATASTREAM
Q_WIDGETS_EXPORT QDataStream &operator<<(QDataStream &, const QSizePolicy &);
Q_WIDGETS_EXPORT QDataStream &operator>>(QDataStream &, QSizePolicy &);
#endif

#ifndef QT_NO_DEBUG_STREAM
Q_WIDGETS_EXPORT QDebug operator<<(QDebug dbg, const QSizePolicy &);
#endif

QT_END_NAMESPACE

#endif // QSIZEPOLICY_H
