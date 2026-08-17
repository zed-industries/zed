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

#ifndef QPRINTER_H
#define QPRINTER_H

#include <QtPrintSupport/qtprintsupportglobal.h>
#include <QtCore/qstring.h>
#include <QtCore/qscopedpointer.h>
#include <QtGui/qpagedpaintdevice.h>
#include <QtGui/qpagelayout.h>

QT_BEGIN_NAMESPACE


#ifndef QT_NO_PRINTER

#if defined(B0)
#undef B0 // Terminal hang-up.  We assume that you do not want that.
#endif

class QPrinterPrivate;
class QPaintEngine;
class QPrintEngine;
class QPrinterInfo;
class QPageSize;
class QPageMargins;

class Q_PRINTSUPPORT_EXPORT QPrinter : public QPagedPaintDevice
{
    Q_DECLARE_PRIVATE(QPrinter)
public:
    enum PrinterMode { ScreenResolution, PrinterResolution, HighResolution };

    explicit QPrinter(PrinterMode mode = ScreenResolution);
    explicit QPrinter(const QPrinterInfo& printer, PrinterMode mode = ScreenResolution);
    ~QPrinter();

    int devType() const override;

    enum Orientation { Portrait, Landscape };

    // ### Qt6 Remove in favor of QPage::PageSize
    typedef PageSize PaperSize;

    enum PageOrder   { FirstPageFirst,
                       LastPageFirst };

    enum ColorMode   { GrayScale,
                       Color };

    enum PaperSource { OnlyOne,
                       Lower,
                       Middle,
                       Manual,
                       Envelope,
                       EnvelopeManual,
                       Auto,
                       Tractor,
                       SmallFormat,
                       LargeFormat,
                       LargeCapacity,
                       Cassette,
                       FormSource,
                       MaxPageSource, // Deprecated
                       CustomSource,
                       LastPaperSource = CustomSource,
                       Upper = OnlyOne  // As defined in Windows
    };

    enum PrinterState { Idle,
                        Active,
                        Aborted,
                        Error };

    enum OutputFormat { NativeFormat, PdfFormat };

    // Keep in sync with QAbstractPrintDialog::PrintRange
    enum PrintRange { AllPages, Selection, PageRange, CurrentPage };

    enum Unit {
        Millimeter,
        Point,
        Inch,
        Pica,
        Didot,
        Cicero,
        DevicePixel
    };

    enum DuplexMode {
        DuplexNone = 0,
        DuplexAuto,
        DuplexLongSide,
        DuplexShortSide
    };

    void setOutputFormat(OutputFormat format);
    OutputFormat outputFormat() const;

    void setPdfVersion(PdfVersion version);
    PdfVersion pdfVersion() const;

    void setPrinterName(const QString &);
    QString printerName() const;

    bool isValid() const;

    void setOutputFileName(const QString &);
    QString outputFileName()const;

    void setPrintProgram(const QString &);
    QString printProgram() const;

    void setDocName(const QString &);
    QString docName() const;

    void setCreator(const QString &);
    QString creator() const;

#ifdef Q_CLANG_QDOC
    // ### Qt6 Remove when these are made virtual in QPagedPaintDevice
    bool setPageLayout(const QPageLayout &pageLayout);
    bool setPageSize(const QPageSize &pageSize);
    bool setPageOrientation(QPageLayout::Orientation orientation);
    bool setPageMargins(const QMarginsF &margins);
    bool setPageMargins(const QMarginsF &margins, QPageLayout::Unit units);
    QPageLayout pageLayout() const;
#else
    using QPagedPaintDevice::setPageSize;
    using QPagedPaintDevice::setPageMargins;
#endif

#if QT_DEPRECATED_SINCE(5,15)
    QT_DEPRECATED_VERSION_X_5_15("Use setPageOrientation() instead.")
    void setOrientation(Orientation);
    QT_DEPRECATED_VERSION_X_5_15("Use pageLayout().orientation() instead.")
    Orientation orientation() const;

    QT_DEPRECATED_VERSION_X_5_15("Use setPageSize(QPageSize) instead.")
    void setPageSize(PageSize) override;
    QT_DEPRECATED_VERSION_X_5_15("Use pageLayout().pageSize().id() instead.")
    PageSize pageSize() const;

    QT_DEPRECATED_VERSION_X_5_15("Use setPageSize(QPageSize) instead.")
    void setPageSizeMM(const QSizeF &size) override;

    QT_DEPRECATED_VERSION_X_5_15("Use setPageSize(QPageSize) instead.")
    void setPaperSize(PaperSize);
    QT_DEPRECATED_VERSION_X_5_15("pageLayout().pageSize().id()")
    PaperSize paperSize() const;

    QT_DEPRECATED_VERSION_X_5_15("Use setPageSize(QPageSize) instead.")
    void setPaperSize(const QSizeF &paperSize, Unit unit);
    QT_DEPRECATED_VERSION_X_5_15("Use pageLayout().pageSize().size() or pageLayout().fullPageSize() instead.")
    QSizeF paperSize(Unit unit) const;

    QT_DEPRECATED_VERSION_X_5_15("Use setPageSize(QPageSize) instead.")
    void setPaperName(const QString &paperName);
    QT_DEPRECATED_VERSION_X_5_15("Use pageLayout().pageSize().name() instead.")
    QString paperName() const;
#endif

    void setPageOrder(PageOrder);
    PageOrder pageOrder() const;

    void setResolution(int);
    int resolution() const;

    void setColorMode(ColorMode);
    ColorMode colorMode() const;

    void setCollateCopies(bool collate);
    bool collateCopies() const;

    void setFullPage(bool);
    bool fullPage() const;

#if QT_DEPRECATED_SINCE(5,15)
    QT_DEPRECATED_VERSION_X_5_15("Use setCopyCount() instead.")
    void setNumCopies(int);
    QT_DEPRECATED_VERSION_X_5_15("Use copyCount() instead.")
    int numCopies() const;
    QT_DEPRECATED_VERSION_X_5_15("Use copyCount() instead.")
    int actualNumCopies() const;
#endif

    void setCopyCount(int);
    int copyCount() const;
    bool supportsMultipleCopies() const;

    void setPaperSource(PaperSource);
    PaperSource paperSource() const;

    void setDuplex(DuplexMode duplex);
    DuplexMode duplex() const;

    QList<int> supportedResolutions() const;

#if defined(Q_OS_WIN) || defined(Q_CLANG_QDOC)
    QList<PaperSource> supportedPaperSources() const;
#endif

    void setFontEmbeddingEnabled(bool enable);
    bool fontEmbeddingEnabled() const;

#if QT_DEPRECATED_SINCE(5,15)
    QT_DEPRECATED_VERSION_X_5_15("Use setDuplex() instead.")
    void setDoubleSidedPrinting(bool enable);
    QT_DEPRECATED_VERSION_X_5_15("Use duplex() instead.")
    bool doubleSidedPrinting() const;
#endif

#if QT_DEPRECATED_SINCE(5,15)
    QT_DEPRECATED_VERSION_X_5_15("Use QPageSize::id(windowsId) and setPageLayout(QPageSize) instead.")
    void setWinPageSize(int winPageSize);
    QT_DEPRECATED_VERSION_X_5_15("Use pageLayout.pageSize().windowsId() instead.")
    int winPageSize() const;

    QT_DEPRECATED_VERSION_X_5_15("Use pageLayout().fullRectPixels(resolution()) instead.")
    QRect paperRect() const;
    QT_DEPRECATED_VERSION_X_5_15("Use pageLayout().paintRectPixels(resolution()) instead.")
    QRect pageRect() const;
#endif
    QRectF paperRect(Unit) const;
    QRectF pageRect(Unit) const;

    QString printerSelectionOption() const;
    void setPrinterSelectionOption(const QString &);

    bool newPage() override;
    bool abort();

    PrinterState printerState() const;

    QPaintEngine *paintEngine() const override;
    QPrintEngine *printEngine() const;

    void setFromTo(int fromPage, int toPage);
    int fromPage() const;
    int toPage() const;

    void setPrintRange(PrintRange range);
    PrintRange printRange() const;

#if QT_DEPRECATED_SINCE(5,15)
    QT_DEPRECATED_VERSION_X_5_15("Use setPageMargins(QMarginsF, QPageLayout::Unit) instead.")
    void setMargins(const Margins &m) override;

    QT_DEPRECATED_VERSION_X_5_15("Use setPageMargins(QMarginsF, QPageLayout::Unit) instead.")
    void setPageMargins(qreal left, qreal top, qreal right, qreal bottom, Unit unit);
    QT_DEPRECATED_VERSION_X_5_15("Use pageLayout().margins() instead.")
    void getPageMargins(qreal *left, qreal *top, qreal *right, qreal *bottom, Unit unit) const;
#endif

protected:
    int metric(PaintDeviceMetric) const override;
    void setEngines(QPrintEngine *printEngine, QPaintEngine *paintEngine);

private:
    Q_DISABLE_COPY(QPrinter)

    QScopedPointer<QPrinterPrivate> d_ptr;

    friend class QPrintDialogPrivate;
    friend class QAbstractPrintDialog;
    friend class QAbstractPrintDialogPrivate;
    friend class QPrintPreviewWidgetPrivate;
    friend class QTextDocument;
    friend class QPageSetupWidget;
};

#endif // QT_NO_PRINTER

QT_END_NAMESPACE

#endif // QPRINTER_H
