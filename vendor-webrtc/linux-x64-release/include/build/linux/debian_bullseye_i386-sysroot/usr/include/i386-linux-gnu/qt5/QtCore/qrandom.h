/****************************************************************************
**
** Copyright (C) 2017 Intel Corporation.
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

#ifndef QRANDOM_H
#define QRANDOM_H

#include <QtCore/qglobal.h>
#include <algorithm>    // for std::generate
#include <random>       // for std::mt19937

#ifdef min
#  undef min
#endif
#ifdef max
#  undef max
#endif

QT_BEGIN_NAMESPACE

class QRandomGenerator
{
    // restrict the template parameters to unsigned integers 32 bits wide or larger
    template <typename UInt> using IfValidUInt =
        typename std::enable_if<std::is_unsigned<UInt>::value && sizeof(UInt) >= sizeof(uint), bool>::type;
public:
    QRandomGenerator(quint32 seedValue = 1)
        : QRandomGenerator(&seedValue, 1)
    {}
    template <qsizetype N> QRandomGenerator(const quint32 (&seedBuffer)[N])
        : QRandomGenerator(seedBuffer, seedBuffer + N)
    {}
    QRandomGenerator(const quint32 *seedBuffer, qsizetype len)
        : QRandomGenerator(seedBuffer, seedBuffer + len)
    {}
    Q_CORE_EXPORT QRandomGenerator(std::seed_seq &sseq) noexcept;
    Q_CORE_EXPORT QRandomGenerator(const quint32 *begin, const quint32 *end);

    // copy constructor & assignment operator (move unnecessary)
    Q_CORE_EXPORT QRandomGenerator(const QRandomGenerator &other);
    Q_CORE_EXPORT QRandomGenerator &operator=(const QRandomGenerator &other);

    friend Q_CORE_EXPORT bool operator==(const QRandomGenerator &rng1, const QRandomGenerator &rng2);
    friend bool operator!=(const QRandomGenerator &rng1, const QRandomGenerator &rng2)
    {
        return !(rng1 == rng2);
    }

    quint32 generate()
    {
        quint32 ret;
        fillRange(&ret, 1);
        return ret;
    }

    quint64 generate64()
    {
        quint32 buf[2];
        fillRange(buf);
        return buf[0] | (quint64(buf[1]) << 32);
    }

    double generateDouble()
    {
        // IEEE 754 double precision has:
        //   1 bit      sign
        //  10 bits     exponent
        //  53 bits     mantissa
        // In order for our result to be normalized in the range [0, 1), we
        // need exactly 53 bits of random data. Use generate64() to get enough.
        quint64 x = generate64();
        quint64 limit = Q_UINT64_C(1) << std::numeric_limits<double>::digits;
        x >>= std::numeric_limits<quint64>::digits - std::numeric_limits<double>::digits;
        return double(x) / double(limit);
    }

    double bounded(double highest)
    {
        return generateDouble() * highest;
    }

    quint32 bounded(quint32 highest)
    {
        quint64 value = generate();
        value *= highest;
        value /= (max)() + quint64(1);
        return quint32(value);
    }

    quint32 bounded(quint32 lowest, quint32 highest)
    {
        Q_ASSERT(highest > lowest);
        return bounded(highest - lowest) + lowest;
    }

    int bounded(int highest)
    {
        Q_ASSERT(highest > 0);
        return int(bounded(0U, quint32(highest)));
    }

    int bounded(int lowest, int highest)
    {
        return bounded(highest - lowest) + lowest;
    }

    template <typename UInt, IfValidUInt<UInt> = true>
    void fillRange(UInt *buffer, qsizetype count)
    {
        _fillRange(buffer, buffer + count);
    }

    template <typename UInt, size_t N, IfValidUInt<UInt> = true>
    void fillRange(UInt (&buffer)[N])
    {
        _fillRange(buffer, buffer + N);
    }

    // API like std::seed_seq
    template <typename ForwardIterator>
    void generate(ForwardIterator begin, ForwardIterator end)
    {
        std::generate(begin, end, [this]() { return generate(); });
    }

    void generate(quint32 *begin, quint32 *end)
    {
        _fillRange(begin, end);
    }

    // API like std:: random engines
    typedef quint32 result_type;
    result_type operator()() { return generate(); }
    void seed(quint32 s = 1) { *this = { s }; }
    void seed(std::seed_seq &sseq) noexcept { *this = { sseq }; }
    Q_CORE_EXPORT void discard(unsigned long long z);
    static Q_DECL_CONSTEXPR result_type min() { return std::numeric_limits<result_type>::min(); }
    static Q_DECL_CONSTEXPR result_type max() { return std::numeric_limits<result_type>::max(); }

    static inline Q_DECL_CONST_FUNCTION QRandomGenerator *system();
    static inline Q_DECL_CONST_FUNCTION QRandomGenerator *global();
    static inline QRandomGenerator securelySeeded();

protected:
    enum System {};
    QRandomGenerator(System);

private:
    Q_CORE_EXPORT void _fillRange(void *buffer, void *bufferEnd);

    friend class QRandomGenerator64;
    struct SystemGenerator;
    struct SystemAndGlobalGenerators;
    using RandomEngine = std::mersenne_twister_engine<quint32,
        32,624,397,31,0x9908b0df,11,0xffffffff,7,0x9d2c5680,15,0xefc60000,18,1812433253>;

    union Storage {
        uint dummy;
#ifdef Q_COMPILER_UNRESTRICTED_UNIONS
        RandomEngine twister;
        RandomEngine &engine() { return twister; }
        const RandomEngine &engine() const { return twister; }
#else
        std::aligned_storage<sizeof(RandomEngine), Q_ALIGNOF(RandomEngine)>::type buffer;
        RandomEngine &engine() { return reinterpret_cast<RandomEngine &>(buffer); }
        const RandomEngine &engine() const { return reinterpret_cast<const RandomEngine &>(buffer); }
#endif

        Q_STATIC_ASSERT_X(std::is_trivially_destructible<RandomEngine>::value,
                          "std::mersenne_twister not trivially destructible as expected");
        Q_DECL_CONSTEXPR Storage();
    };
    uint type;
    Storage storage;
};

class QRandomGenerator64 : public QRandomGenerator
{
    QRandomGenerator64(System);
public:
    // unshadow generate() overloads, since we'll override.
    using QRandomGenerator::generate;
    quint64 generate() { return generate64(); }

    typedef quint64 result_type;
    result_type operator()() { return generate64(); }

#ifndef Q_QDOC
    QRandomGenerator64(quint32 seedValue = 1)
        : QRandomGenerator(seedValue)
    {}
    template <qsizetype N> QRandomGenerator64(const quint32 (&seedBuffer)[N])
        : QRandomGenerator(seedBuffer)
    {}
    QRandomGenerator64(const quint32 *seedBuffer, qsizetype len)
        : QRandomGenerator(seedBuffer, len)
    {}
    QRandomGenerator64(std::seed_seq &sseq) noexcept
        : QRandomGenerator(sseq)
    {}
    QRandomGenerator64(const quint32 *begin, const quint32 *end)
        : QRandomGenerator(begin, end)
    {}
    QRandomGenerator64(const QRandomGenerator &other) : QRandomGenerator(other) {}

    void discard(unsigned long long z)
    {
        Q_ASSERT_X(z * 2 > z, "QRandomGenerator64::discard",
                   "Overflow. Are you sure you want to skip over 9 quintillion samples?");
        QRandomGenerator::discard(z * 2);
    }

    static Q_DECL_CONSTEXPR result_type min() { return std::numeric_limits<result_type>::min(); }
    static Q_DECL_CONSTEXPR result_type max() { return std::numeric_limits<result_type>::max(); }
    static Q_DECL_CONST_FUNCTION Q_CORE_EXPORT QRandomGenerator64 *system();
    static Q_DECL_CONST_FUNCTION Q_CORE_EXPORT QRandomGenerator64 *global();
    static Q_CORE_EXPORT QRandomGenerator64 securelySeeded();
#endif // Q_QDOC
};

inline QRandomGenerator *QRandomGenerator::system()
{
    return QRandomGenerator64::system();
}

inline QRandomGenerator *QRandomGenerator::global()
{
    return QRandomGenerator64::global();
}

QRandomGenerator QRandomGenerator::securelySeeded()
{
    return QRandomGenerator64::securelySeeded();
}

QT_END_NAMESPACE

#endif // QRANDOM_H
