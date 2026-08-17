// Copyright (C) 2020 Intel Corporation.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QRANDOM_H
#define QRANDOM_H

#include <QtCore/qalgorithms.h>
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
        return quint32(_fillRange(nullptr, 1));
    }

    quint64 generate64()
    {
        return _fillRange(nullptr, sizeof(quint64) / sizeof(quint32));
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

    quint64 bounded(quint64 highest);

    quint64 bounded(quint64 lowest, quint64 highest)
    {
        Q_ASSERT(highest > lowest);
        return bounded(highest - lowest) + lowest;
    }

    qint64 bounded(qint64 highest)
    {
        Q_ASSERT(highest > 0);
        return qint64(bounded(quint64(0), quint64(highest)));
    }

    qint64 bounded(qint64 lowest, qint64 highest)
    {
        return bounded(highest - lowest) + lowest;
    }

    // these functions here only to help with ambiguous overloads
    qint64 bounded(int lowest, qint64 highest)
    {
        return bounded(qint64(lowest), qint64(highest));
    }
    qint64 bounded(qint64 lowest, int highest)
    {
        return bounded(qint64(lowest), qint64(highest));
    }

    quint64 bounded(unsigned lowest, quint64 highest)
    {
        return bounded(quint64(lowest), quint64(highest));
    }
    quint64 bounded(quint64 lowest, unsigned highest)
    {
        return bounded(quint64(lowest), quint64(highest));
    }

    template <typename UInt, IfValidUInt<UInt> = true>
    void fillRange(UInt *buffer, qsizetype count)
    {
        _fillRange(buffer, count * sizeof(UInt) / sizeof(quint32));
    }

    template <typename UInt, size_t N, IfValidUInt<UInt> = true>
    void fillRange(UInt (&buffer)[N])
    {
        _fillRange(buffer, N * sizeof(UInt) / sizeof(quint32));
    }

    // API like std::seed_seq
    template <typename ForwardIterator>
    void generate(ForwardIterator begin, ForwardIterator end)
    {
        std::generate(begin, end, [this]() { return generate(); });
    }

    void generate(quint32 *begin, quint32 *end)
    {
        _fillRange(begin, end - begin);
    }

    // API like std:: random engines
    typedef quint32 result_type;
    result_type operator()() { return generate(); }
    void seed(quint32 s = 1) { *this = { s }; }
    void seed(std::seed_seq &sseq) noexcept { *this = { sseq }; }
    Q_CORE_EXPORT void discard(unsigned long long z);
    static constexpr result_type min() { return (std::numeric_limits<result_type>::min)(); }
    static constexpr result_type max() { return (std::numeric_limits<result_type>::max)(); }

    static inline Q_DECL_CONST_FUNCTION QRandomGenerator *system();
    static inline Q_DECL_CONST_FUNCTION QRandomGenerator *global();
    static inline QRandomGenerator securelySeeded();

protected:
    enum System {};
    QRandomGenerator(System);

private:
    Q_CORE_EXPORT quint64 _fillRange(void *buffer, qptrdiff count);

    struct InitialRandomData {
        quintptr data[16 / sizeof(quintptr)];
    };
    friend InitialRandomData qt_initial_random_value() noexcept;
    friend class QRandomGenerator64;
    struct SystemGenerator;
    struct SystemAndGlobalGenerators;
    using RandomEngine = std::mersenne_twister_engine<quint32,
        32,624,397,31,0x9908b0df,11,0xffffffff,7,0x9d2c5680,15,0xefc60000,18,1812433253>;

    union Storage {
        uint dummy;
        RandomEngine twister;
        RandomEngine &engine() { return twister; }
        const RandomEngine &engine() const { return twister; }

        static_assert(std::is_trivially_destructible<RandomEngine>::value,
                          "std::mersenne_twister not trivially destructible as expected");
        constexpr Storage();
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

    static constexpr result_type min() { return (std::numeric_limits<result_type>::min)(); }
    static constexpr result_type max() { return (std::numeric_limits<result_type>::max)(); }
    static Q_DECL_CONST_FUNCTION Q_CORE_EXPORT QRandomGenerator64 *system();
    static Q_DECL_CONST_FUNCTION Q_CORE_EXPORT QRandomGenerator64 *global();
    static Q_CORE_EXPORT QRandomGenerator64 securelySeeded();
#endif // Q_QDOC
};

inline quint64 QRandomGenerator::bounded(quint64 highest)
{
    // Implement an algorithm similar to libc++'s uniform_int_distribution:
    // loop around getting a random number, mask off any bits that "highest"
    // will never need, then check if it's higher than "highest". The number of
    // times the loop will run is unbounded but the probability of terminating
    // is better than 1/2 on each iteration. Therefore, the average loop count
    // should be less than 2.

    const int width = qCountLeadingZeroBits(highest - 1);
    const quint64 mask = (quint64(1) << (std::numeric_limits<quint64>::digits - width)) - 1;
    quint64 v;
    do {
        v = generate64() & mask;
    } while (v >= highest);
    return v;
}

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
