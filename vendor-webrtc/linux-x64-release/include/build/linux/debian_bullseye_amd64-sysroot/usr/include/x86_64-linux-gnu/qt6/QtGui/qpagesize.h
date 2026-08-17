// Copyright (C) 2014 John Layt <jlayt@kde.org>
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QPAGESIZE_H
#define QPAGESIZE_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/qsharedpointer.h>

QT_BEGIN_NAMESPACE

#if defined(B0)
#undef B0 // Terminal hang-up.  We assume that you do not want that.
#endif

class QPageSizePrivate;
class QString;
class QSize;
class QSizeF;

class Q_GUI_EXPORT QPageSize
{
public:

    enum PageSizeId {
        // Old Qt sizes
        Letter,
        Legal,
        Executive,
        A0,
        A1,
        A2,
        A3,
        A4,
        A5,
        A6,
        A7,
        A8,
        A9,
        A10,
        B0,
        B1,
        B2,
        B3,
        B4,
        B5,
        B6,
        B7,
        B8,
        B9,
        B10,
        C5E,
        Comm10E,
        DLE,
        Folio,
        Ledger,
        Tabloid,
        Custom,

        // New values derived from PPD standard
        A3Extra,
        A4Extra,
        A4Plus,
        A4Small,
        A5Extra,
        B5Extra,

        JisB0,
        JisB1,
        JisB2,
        JisB3,
        JisB4,
        JisB5,
        JisB6,
        JisB7,
        JisB8,
        JisB9,
        JisB10,

        // AnsiA = Letter,
        // AnsiB = Ledger,
        AnsiC,
        AnsiD,
        AnsiE,
        LegalExtra,
        LetterExtra,
        LetterPlus,
        LetterSmall,
        TabloidExtra,

        ArchA,
        ArchB,
        ArchC,
        ArchD,
        ArchE,

        Imperial7x9,
        Imperial8x10,
        Imperial9x11,
        Imperial9x12,
        Imperial10x11,
        Imperial10x13,
        Imperial10x14,
        Imperial12x11,
        Imperial15x11,

        ExecutiveStandard,
        Note,
        Quarto,
        Statement,
        SuperA,
        SuperB,
        Postcard,
        DoublePostcard,
        Prc16K,
        Prc32K,
        Prc32KBig,

        FanFoldUS,
        FanFoldGerman,
        FanFoldGermanLegal,

        EnvelopeB4,
        EnvelopeB5,
        EnvelopeB6,
        EnvelopeC0,
        EnvelopeC1,
        EnvelopeC2,
        EnvelopeC3,
        EnvelopeC4,
        // EnvelopeC5 = C5E,
        EnvelopeC6,
        EnvelopeC65,
        EnvelopeC7,
        // EnvelopeDL = DLE,

        Envelope9,
        // Envelope10 = Comm10E,
        Envelope11,
        Envelope12,
        Envelope14,
        EnvelopeMonarch,
        EnvelopePersonal,

        EnvelopeChou3,
        EnvelopeChou4,
        EnvelopeInvite,
        EnvelopeItalian,
        EnvelopeKaku2,
        EnvelopeKaku3,
        EnvelopePrc1,
        EnvelopePrc2,
        EnvelopePrc3,
        EnvelopePrc4,
        EnvelopePrc5,
        EnvelopePrc6,
        EnvelopePrc7,
        EnvelopePrc8,
        EnvelopePrc9,
        EnvelopePrc10,
        EnvelopeYou4,

        // Last item
        LastPageSize = EnvelopeYou4,

        // Convenience overloads for naming consistency
        AnsiA = Letter,
        AnsiB = Ledger,
        EnvelopeC5 = C5E,
        EnvelopeDL = DLE,
        Envelope10 = Comm10E
    };

    // NOTE: Must keep in sync with QPageLayout::Unit and QPrinter::Unit
    enum Unit {
        Millimeter,
        Point,
        Inch,
        Pica,
        Didot,
        Cicero
    };

    enum SizeMatchPolicy {
        FuzzyMatch,
        FuzzyOrientationMatch,
        ExactMatch
    };

    QPageSize();
    Q_IMPLICIT QPageSize(PageSizeId pageSizeId);
    explicit QPageSize(const QSize &pointSize,
                       const QString &name = QString(),
                       SizeMatchPolicy matchPolicy = FuzzyMatch);
    explicit QPageSize(const QSizeF &size, Unit units,
                       const QString &name = QString(),
                       SizeMatchPolicy matchPolicy = FuzzyMatch);
    QPageSize(const QPageSize &other);
    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_PURE_SWAP(QPageSize)
    QPageSize &operator=(const QPageSize &other);
    ~QPageSize();


    void swap(QPageSize &other) noexcept { d.swap(other.d); }

    friend Q_GUI_EXPORT bool operator==(const QPageSize &lhs, const QPageSize &rhs);
    bool isEquivalentTo(const QPageSize &other) const;

    bool isValid() const;

    QString key() const;
    QString name() const;

    PageSizeId id() const;

    int windowsId() const;

    QSizeF definitionSize() const;
    Unit definitionUnits() const;

    QSizeF size(Unit units) const;
    QSize sizePoints() const;
    QSize sizePixels(int resolution) const;

    QRectF rect(Unit units) const;
    QRect rectPoints() const;
    QRect rectPixels(int resolution) const;

    static QString key(PageSizeId pageSizeId);
    static QString name(PageSizeId pageSizeId);

    static PageSizeId id(const QSize &pointSize,
                         SizeMatchPolicy matchPolicy = FuzzyMatch);
    static PageSizeId id(const QSizeF &size, Unit units,
                         SizeMatchPolicy matchPolicy = FuzzyMatch);

    static PageSizeId id(int windowsId);
    static int windowsId(PageSizeId pageSizeId);

    static QSizeF definitionSize(PageSizeId pageSizeId);
    static Unit definitionUnits(PageSizeId pageSizeId);

    static QSizeF size(PageSizeId pageSizeId, Unit units);
    static QSize sizePoints(PageSizeId pageSizeId);
    static QSize sizePixels(PageSizeId pageSizeId, int resolution);

private:
    friend class QPageSizePrivate;
    friend class QPlatformPrintDevice;

    bool equals(const QPageSize &other) const;
    friend inline bool operator==(const QPageSize &lhs, const QPageSize &rhs)
    { return lhs.equals(rhs); }
    friend inline bool operator!=(const QPageSize &lhs, const QPageSize &rhs)
    { return !(lhs == rhs); }

    QPageSize(const QString &key, const QSize &pointSize, const QString &name);
    QPageSize(int windowsId, const QSize &pointSize, const QString &name);
    QPageSize(QPageSizePrivate &dd);
    QSharedDataPointer<QPageSizePrivate> d;
};

Q_DECLARE_SHARED(QPageSize)

#ifndef QT_NO_DEBUG_STREAM
Q_GUI_EXPORT QDebug operator<<(QDebug dbg, const QPageSize &pageSize);
#endif

QT_END_NAMESPACE

QT_DECL_METATYPE_EXTERN(QPageSize, Q_GUI_EXPORT)
QT_DECL_METATYPE_EXTERN_TAGGED(QPageSize::PageSizeId, QPageSize__PageSizeId, Q_GUI_EXPORT)
QT_DECL_METATYPE_EXTERN_TAGGED(QPageSize::Unit, QPageSize__Unit, Q_GUI_EXPORT)

#endif // QPAGESIZE_H
