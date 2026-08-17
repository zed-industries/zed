// Copyright (C) 2022 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QSTRINGCONVERTER_BASE_H
#define QSTRINGCONVERTER_BASE_H

#if 0
// QStringConverter(Base) class are handled in qstringconverter
#pragma qt_sync_stop_processing
#endif

#include <optional>

#include <QtCore/qglobal.h> // QT_{BEGIN,END}_NAMESPACE
#include <QtCore/qflags.h> // Q_DECLARE_FLAGS

#include <cstring>

QT_BEGIN_NAMESPACE

class QByteArrayView;
class QChar;
class QByteArrayView;
class QStringView;

class QStringConverterBase
{
public:
    enum class Flag {
        Default = 0,
        Stateless = 0x1,
        ConvertInvalidToNull = 0x2,
        WriteBom = 0x4,
        ConvertInitialBom = 0x8,
        UsesIcu = 0x10,
    };
    Q_DECLARE_FLAGS(Flags, Flag)

    struct State {
        constexpr State(Flags f = Flag::Default) noexcept
            : flags(f), state_data{0, 0, 0, 0} {}
        ~State() { clear(); }

        State(State &&other) noexcept
            : flags(other.flags),
              remainingChars(other.remainingChars),
              invalidChars(other.invalidChars),
              state_data{other.state_data[0], other.state_data[1],
                         other.state_data[2], other.state_data[3]},
              clearFn(other.clearFn)
        { other.clearFn = nullptr; }
        State &operator=(State &&other) noexcept
        {
            clear();
            flags = other.flags;
            remainingChars = other.remainingChars;
            invalidChars = other.invalidChars;
            std::memmove(state_data, other.state_data, sizeof state_data); // self-assignment-safe
            clearFn = other.clearFn;
            other.clearFn = nullptr;
            return *this;
        }
        Q_CORE_EXPORT void clear() noexcept;
        Q_CORE_EXPORT void reset() noexcept;

        Flags flags;
        int internalState = 0;
        qsizetype remainingChars = 0;
        qsizetype invalidChars = 0;

        union {
            uint state_data[4];
            void *d[2];
        };
        using ClearDataFn = void (*)(State *) noexcept;
        ClearDataFn clearFn = nullptr;
    private:
        Q_DISABLE_COPY(State)
    };
protected:
    ~QStringConverterBase() = default;
};
Q_DECLARE_OPERATORS_FOR_FLAGS(QStringConverterBase::Flags)

class QStringConverter : public QStringConverterBase
{
public:

    enum Encoding {
        Utf8,
        Utf16,
        Utf16LE,
        Utf16BE,
        Utf32,
        Utf32LE,
        Utf32BE,
        Latin1,
        System,
        LastEncoding = System
    };
#ifdef Q_QDOC
    // document the flags here
    enum class Flag {
        Default = 0,
        Stateless = 0x1,
        ConvertInvalidToNull = 0x2,
        WriteBom = 0x4,
        ConvertInitialBom = 0x8,
        UsesIcu = 0x10,
    };
    Q_DECLARE_FLAGS(Flags, Flag)
#endif

protected:

    struct Interface
    {
        using DecoderFn = QChar * (*)(QChar *out, QByteArrayView in, State *state);
        using LengthFn = qsizetype (*)(qsizetype inLength);
        using EncoderFn = char * (*)(char *out, QStringView in, State *state);
        const char *name = nullptr;
        DecoderFn toUtf16 = nullptr;
        LengthFn toUtf16Len = nullptr;
        EncoderFn fromUtf16 = nullptr;
        LengthFn fromUtf16Len = nullptr;
    };

    constexpr QStringConverter() noexcept
        : iface(nullptr)
    {}
    constexpr explicit QStringConverter(Encoding encoding, Flags f)
        : iface(&encodingInterfaces[qsizetype(encoding)]), state(f)
    {}
    constexpr explicit QStringConverter(const Interface *i) noexcept
        : iface(i)
    {}
    Q_CORE_EXPORT explicit QStringConverter(const char *name, Flags f);


    ~QStringConverter() = default;

public:
    QStringConverter(QStringConverter &&) = default;
    QStringConverter &operator=(QStringConverter &&) = default;

    bool isValid() const noexcept { return iface != nullptr; }

    void resetState() noexcept
    {
        state.reset();
    }
    bool hasError() const noexcept { return state.invalidChars != 0; }

    Q_CORE_EXPORT const char *name() const noexcept;

    Q_CORE_EXPORT static std::optional<Encoding> encodingForName(const char *name) noexcept;
    Q_CORE_EXPORT static const char *nameForEncoding(Encoding e);
    Q_CORE_EXPORT static std::optional<Encoding>
    encodingForData(QByteArrayView data, char16_t expectedFirstCharacter = 0) noexcept;
    Q_CORE_EXPORT static std::optional<Encoding> encodingForHtml(QByteArrayView data);

protected:
    const Interface *iface;
    State state;
private:
    Q_CORE_EXPORT static const Interface encodingInterfaces[Encoding::LastEncoding + 1];
};

QT_END_NAMESPACE

#endif
